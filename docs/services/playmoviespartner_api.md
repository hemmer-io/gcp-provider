# Playmoviespartner_api Service



**Resources**: 4

---

## Overview

The playmoviespartner_api service provides access to 4 resource types:

- [Country](#country) [R]
- [Store_info](#store_info) [R]
- [Avail](#avail) [R]
- [Order](#order) [R]

---

## Resources


### Country

Get a StoreInfo given its video id and country.

See _Authentication and Authorization rules_ and
_Get methods rules_ for more information about this method.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subtitles` | Vec<String> | Subtitles available for this Edit. |
| `country` | String | Country where Edit is available in ISO 3166-1 alpha-2 country
code.
Example: "US". |
| `trailer_id` | String | Google-generated ID identifying the trailer linked to the Edit.
Example: 'bhd_4e_cx' |
| `has_hd_offer` | bool | Whether the Edit has a HD offer. |
| `pph_names` | Vec<String> | Name of the post-production houses that manage the Edit. |
| `studio_name` | String | Name of the studio that owns the Edit ordered. |
| `live_time` | String | Timestamp when the Edit went live on the Store. |
| `title_level_eidr` | String | Title-level EIDR ID.
Example: "10.5240/1489-49A2-3956-4B2D-FE16-5". |
| `name` | String | Default Edit name, usually in the language of the country of
origin.
Example: "Googlers, The". |
| `video_id` | String | Google-generated ID identifying the video linked to the Edit.
Example: 'gtry456_xc' |
| `show_name` | String | Default Show name, usually in the language of the country of
origin.
Only available for TV Edits
Example: "Googlers, The". |
| `season_name` | String | Default Season name, usually in the language of the country of
origin.
Only available for TV Edits
Example: "Googlers, The - A Brave New World". |
| `season_number` | String | The number assigned to the season within a show.
Only available on TV Edits.
Example: "1". |
| `show_id` | String | Google-generated ID identifying the show linked to the Edit.
Only available for TV Edits.
Example: 'et2hsue_x' |
| `has_audio51` | bool | Whether the Edit has a 5.1 channel audio track. |
| `mid` | String | Knowledge Graph ID associated to this Edit, if available.
This ID links the Edit to its knowledge entity, externally accessible
at http://freebase.com.
In the absense of Title EIDR or Edit EIDR, this ID helps link together
multiple Edits across countries.
Example: '/m/0ffx29' |
| `has_info_cards` | bool | Whether the Edit has info cards. |
| `edit_level_eidr` | String | Edit-level EIDR ID.
Example: "10.5240/1489-49A2-3956-4B2D-FE16-6". |
| `has_sd_offer` | bool | Whether the Edit has a SD offer. |
| `episode_number` | String | The number assigned to the episode within a season.
Only available on TV Edits.
Example: "1". |
| `season_id` | String | Google-generated ID identifying the season linked to the Edit.
Only available for TV Edits.
Example: 'ster23ex' |
| `has_est_offer` | bool | Whether the Edit has a EST offer. |
| `has_vod_offer` | bool | Whether the Edit has a VOD offer. |
| `audio_tracks` | Vec<String> | Audio tracks available for this Edit. |
| `type` | String | Edit type, like Movie, Episode or Season. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access country outputs
country_id = country.id
country_subtitles = country.subtitles
country_country = country.country
country_trailer_id = country.trailer_id
country_has_hd_offer = country.has_hd_offer
country_pph_names = country.pph_names
country_studio_name = country.studio_name
country_live_time = country.live_time
country_title_level_eidr = country.title_level_eidr
country_name = country.name
country_video_id = country.video_id
country_show_name = country.show_name
country_season_name = country.season_name
country_season_number = country.season_number
country_show_id = country.show_id
country_has_audio51 = country.has_audio51
country_mid = country.mid
country_has_info_cards = country.has_info_cards
country_edit_level_eidr = country.edit_level_eidr
country_has_sd_offer = country.has_sd_offer
country_episode_number = country.episode_number
country_season_id = country.season_id
country_has_est_offer = country.has_est_offer
country_has_vod_offer = country.has_vod_offer
country_audio_tracks = country.audio_tracks
country_type = country.type
```

---


### Store_info

List StoreInfos owned or managed by the partner.

See _Authentication and Authorization rules_ and
_List methods rules_ for more information about this method.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | See 'List methods rules' for info about this field. |
| `store_infos` | Vec<String> | List of StoreInfos that match the request criteria. |
| `total_size` | i64 | See _List methods rules_ for more information about this field. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access store_info outputs
store_info_id = store_info.id
store_info_next_page_token = store_info.next_page_token
store_info_store_infos = store_info.store_infos
store_info_total_size = store_info.total_size
```

---


### Avail

Get an Avail given its avail group id and avail id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rating_system` | String | Rating system applied to the version of title within territory
of Avail.
Rating systems should be formatted as per
[EMA ratings spec](http://www.movielabs.com/md/ratings/)
Example: "MPAA" |
| `avail_id` | String | ID internally generated by Google to uniquely identify an Avail.
Not part of EMA Specs. |
| `release_date` | String | Release date of the Title in earliest released territory.
Typically it is just the year, but it is free-form as per EMA spec.
Examples: "1979", "Oct 2014" |
| `end` | String | End of term in YYYY-MM-DD format in the timezone of the country
of the Avail.
"Open" if no end date is available.
Example: "2019-02-17" |
| `start` | String | Start of term in YYYY-MM-DD format in the timezone of the
country of the Avail.
Example: "2013-05-14". |
| `suppression_lift_date` | String | First date an Edit could be publically announced as becoming
available at a specific future date in territory of Avail.
*Not* the Avail start date or pre-order start date.
Format is YYYY-MM-DD.
Only available for pre-orders.
Example: "2012-12-10" |
| `display_name` | String | The name of the studio that owns the Edit referred in the Avail.
This is the equivalent of `studio_name` in other resources, but it follows
the EMA nomenclature.
Example: "Google Films". |
| `content_id` | String | Title Identifier. This should be the Title Level EIDR.
Example: "10.5240/1489-49A2-3956-4B2D-FE16-5". |
| `video_id` | String | Google-generated ID identifying the video linked to this Avail, once
delivered.
Not part of EMA Specs.
Example: 'gtry456_xc' |
| `season_title_internal_alias` | String | Title used by involved parties to refer to this season.
Only available on TV Avails.
Example: "Googlers, The". |
| `work_type` | String | Work type as enumerated in EMA. |
| `price_value` | String | Value to be applied to the pricing type.
Example: "4" or "2.99" |
| `product_id` | String | Edit Identifier. This should be the Edit Level EIDR.
Example: "10.2340/1489-49A2-3956-4B2D-FE16-6" |
| `pph_names` | Vec<String> | Name of the post-production houses that manage the Avail.
Not part of EMA Specs. |
| `episode_alt_id` | String | Other identifier referring to the episode, as defined by partner.
Only available on TV avails.
Example: "rs_googlers_s1_3". |
| `store_language` | String | Spoken language of the intended audience.
Language shall be encoded in accordance with RFC 5646.
Example: "fr". |
| `price_type` | String | Type of pricing that should be applied to this Avail
based on how the partner classify them.
Example: "Tier", "WSP", "SRP", or "Category". |
| `series_title_internal_alias` | String | Title used by involved parties to refer to this series.
Only available on TV Avails.
Example: "Googlers, The". |
| `caption_included` | bool | Communicating if caption file will be delivered. |
| `alt_id` | String | Other identifier referring to the Edit, as defined by partner.
Example: "GOOGLER_2006" |
| `season_number` | String | The number assigned to the season within a series.
Only available on TV Avails.
Example: "1". |
| `series_alt_id` | String | Other identifier referring to the series, as defined by partner.
Only available on TV avails.
Example: "rs_googlers". |
| `title_internal_alias` | String | Title used by involved parties to refer to this content.
Example: "Googlers, The".
Only available on Movie Avails. |
| `license_type` | String | Type of transaction. |
| `format_profile` | String | Indicates the format profile covered by the transaction. |
| `rating_value` | String | Value representing the rating.
Ratings should be formatted as per http://www.movielabs.com/md/ratings/
Example: "PG" |
| `episode_title_internal_alias` | String | OPTIONAL.TV Only. Title used by involved parties to refer to this episode.
Only available on TV Avails.
Example: "Coding at Google". |
| `encode_id` | String | Manifestation Identifier. This should be the Manifestation
Level EIDR.
Example: "10.2340/1489-49A2-3956-4B2D-FE16-7" |
| `caption_exemption` | String | Communicating an exempt category as defined by FCC regulations.
It is not required for non-US Avails.
Example: "1" |
| `season_alt_id` | String | Other identifier referring to the season, as defined by partner.
Only available on TV avails.
Example: "rs_googlers_s1". |
| `episode_number` | String | The number assigned to the episode within a season.
Only available on TV Avails.
Example: "3". |
| `territory` | String | ISO 3166-1 alpha-2 country code for the country or territory
of this Avail.
For Avails, we use Territory in lieu of Country to comply with
EMA specifications.
But please note that Territory and Country identify the same thing.
Example: "US". |
| `rating_reason` | String | Value representing the rating reason.
Rating reasons should be formatted as per
[EMA ratings spec](http://www.movielabs.com/md/ratings/)
and comma-separated for inclusion of multiple reasons.
Example: "L, S, V" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access avail outputs
avail_id = avail.id
avail_rating_system = avail.rating_system
avail_avail_id = avail.avail_id
avail_release_date = avail.release_date
avail_end = avail.end
avail_start = avail.start
avail_suppression_lift_date = avail.suppression_lift_date
avail_display_name = avail.display_name
avail_content_id = avail.content_id
avail_video_id = avail.video_id
avail_season_title_internal_alias = avail.season_title_internal_alias
avail_work_type = avail.work_type
avail_price_value = avail.price_value
avail_product_id = avail.product_id
avail_pph_names = avail.pph_names
avail_episode_alt_id = avail.episode_alt_id
avail_store_language = avail.store_language
avail_price_type = avail.price_type
avail_series_title_internal_alias = avail.series_title_internal_alias
avail_caption_included = avail.caption_included
avail_alt_id = avail.alt_id
avail_season_number = avail.season_number
avail_series_alt_id = avail.series_alt_id
avail_title_internal_alias = avail.title_internal_alias
avail_license_type = avail.license_type
avail_format_profile = avail.format_profile
avail_rating_value = avail.rating_value
avail_episode_title_internal_alias = avail.episode_title_internal_alias
avail_encode_id = avail.encode_id
avail_caption_exemption = avail.caption_exemption
avail_season_alt_id = avail.season_alt_id
avail_episode_number = avail.episode_number
avail_territory = avail.territory
avail_rating_reason = avail.rating_reason
```

---


### Order

Get an Order given its id.

See _Authentication and Authorization rules_ and
_Get methods rules_ for more information about this method.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `studio_name` | String | Name of the studio that owns the Edit ordered. |
| `status` | String | High-level status of the order. |
| `custom_id` | String | ID that can be used to externally identify an Order.
This ID is provided by partners when submitting the Avails.
Example: 'GOOGLER_2006' |
| `approved_time` | String | Timestamp when the Order was approved. |
| `legacy_priority` | String | Legacy Order priority, as defined by Google.
Example: 'P0' |
| `pph_name` | String | Name of the post-production house that manages the Edit ordered. |
| `show_name` | String | Default Show name,
usually in the language of the country of origin.
Only available for TV Edits
Example: "Googlers, The". |
| `order_id` | String | ID internally generated by Google to uniquely identify an Order.
Example: 'abcde12_x' |
| `earliest_avail_start_time` | String | Timestamp of the earliest start date of the Avails
linked to this Order. |
| `status_detail` | String | Detailed status of the order |
| `name` | String | Default Edit name,
usually in the language of the country of origin.
Example: "Googlers, The". |
| `channel_name` | String | YouTube Channel Name that should be used to fulfill the Order.
Example: "Google_channel". |
| `priority` | f64 | Order priority, as defined by Google.
The higher the value, the higher the priority.
Example: 90 |
| `channel_id` | String | YouTube Channel ID that should be used to fulfill the Order.
Example: "UCRG64darCZhb". |
| `rejection_note` | String | Field explaining why an Order has been rejected.
Example: "Trailer audio is 2ch mono, please re-deliver in stereo". |
| `countries` | Vec<String> | Countries where the Order is available,
using the "ISO 3166-1 alpha-2" format (example: "US"). |
| `episode_name` | String | Default Episode name,
usually in the language of the country of origin.
Only available for TV Edits
Example: "Googlers, The - Pilot". |
| `season_name` | String | Default Season name,
usually in the language of the country of origin.
Only available for TV Edits
Example: "Googlers, The - A Brave New World". |
| `received_time` | String | Timestamp when the Order was fulfilled. |
| `ordered_time` | String | Timestamp when the Order was created. |
| `normalized_priority` | String | A simpler representation of the priority. |
| `video_id` | String | Google-generated ID identifying the video linked to this Order, once
delivered.
Example: 'gtry456_xc'. |
| `type` | String | Type of the Edit linked to the Order. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access order outputs
order_id = order.id
order_studio_name = order.studio_name
order_status = order.status
order_custom_id = order.custom_id
order_approved_time = order.approved_time
order_legacy_priority = order.legacy_priority
order_pph_name = order.pph_name
order_show_name = order.show_name
order_order_id = order.order_id
order_earliest_avail_start_time = order.earliest_avail_start_time
order_status_detail = order.status_detail
order_name = order.name
order_channel_name = order.channel_name
order_priority = order.priority
order_channel_id = order.channel_id
order_rejection_note = order.rejection_note
order_countries = order.countries
order_episode_name = order.episode_name
order_season_name = order.season_name
order_received_time = order.received_time
order_ordered_time = order.ordered_time
order_normalized_priority = order.normalized_priority
order_video_id = order.video_id
order_type = order.type
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple country resources
country_0 = provider.playmoviespartner_api.Country {
}
country_1 = provider.playmoviespartner_api.Country {
}
country_2 = provider.playmoviespartner_api.Country {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    country = provider.playmoviespartner_api.Country {
    }
```

---

## Related Documentation

- [GCP Playmoviespartner_api Documentation](https://cloud.google.com/playmoviespartner_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
