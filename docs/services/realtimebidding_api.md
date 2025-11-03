# Realtimebidding_api Service



**Resources**: 8

---

## Overview

The realtimebidding_api service provides access to 8 resource types:

- [Pretargeting_config](#pretargeting_config) [CRUD]
- [User_list](#user_list) [CRU]
- [Publisher_connection](#publisher_connection) [CR]
- [Creative](#creative) [CRU]
- [Bidder](#bidder) [R]
- [Buyer](#buyer) [R]
- [Endpoint](#endpoint) [RU]
- [Bidding_function](#bidding_function) [CR]

---

## Resources


### Pretargeting_config

Creates a pretargeting configuration. A pretargeting configuration's state (PretargetingConfig.state) is active upon creation, and it will start to affect traffic shortly after. A bidder may create a maximum of 10 pretargeting configurations. Attempts to exceed this maximum results in a 400 bad request error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vertical_targeting` | String |  | The verticals included or excluded in this configuration as defined in https://developers.google.com/authorized-buyers/rtb/downloads/publisher-verticals |
| `included_languages` | Vec<String> |  | The languages included in this configuration, represented by their language code. See https://developers.google.com/adwords/api/docs/appendix/languagecodes. |
| `interstitial_targeting` | String |  | The interstitial targeting specified for this configuration. The unset value will allow bid requests to be sent regardless of whether they are for interstitials or not. |
| `included_user_id_types` | Vec<String> |  | User identifier types included in this configuration. At least one of the user identifier types specified in this list must be available for the bid request to be sent. |
| `user_list_targeting` | String |  | The remarketing lists included or excluded in this configuration as defined in UserList. |
| `billing_id` | String |  | Output only. The identifier that corresponds to this pretargeting configuration that helps buyers track and attribute their spend across their own arbitrary divisions. If a bid request matches more than one configuration, the buyer chooses which billing_id to attribute each of their bids. |
| `display_name` | String |  | The diplay name associated with this configuration. This name must be unique among all the pretargeting configurations a bidder has. |
| `web_targeting` | String |  | Targeting on a subset of site inventory. If WEB is listed in included_environments, the specified targeting is applied. A maximum of 50,000 site URLs can be targeted. An unset value for targeting allows all web-based bid requests to be sent. Sites can either be targeting positively (bid requests will be sent only if the destination site is listed in the targeting dimension) or negatively (bid requests will be sent only if the destination site is not listed in the pretargeting configuration). |
| `included_mobile_operating_system_ids` | Vec<String> |  | The mobile operating systems included in this configuration as defined in https://storage.googleapis.com/adx-rtb-dictionaries/mobile-os.csv |
| `included_environments` | Vec<String> |  | Environments that are being included. Bid requests will not be sent for a given environment if it is not included. Further restrictions can be applied to included environments to target only a subset of its inventory. An unset value includes all environments. |
| `excluded_content_label_ids` | Vec<String> |  | The sensitive content category label IDs excluded in this configuration. Bid requests for inventory with any of the specified content label IDs will not be sent. Refer to this file https://storage.googleapis.com/adx-rtb-dictionaries/content-labels.txt for category IDs. |
| `geo_targeting` | String |  | The geos included or excluded in this configuration defined in https://storage.googleapis.com/adx-rtb-dictionaries/geo-table.csv |
| `maximum_qps` | String |  | The maximum QPS threshold for this configuration. The bidder should receive no more than this number of bid requests matching this configuration per second across all their bidding endpoints among all trading locations. Further information available at https://developers.google.com/authorized-buyers/rtb/peer-guide |
| `state` | String |  | Output only. The state of this pretargeting configuration. |
| `publisher_targeting` | String |  | Targeting on a subset of publisher inventory. Publishers can either be targeted positively (bid requests will be sent only if the publisher is listed in the targeting dimension) or negatively (bid requests will be sent only if the publisher is not listed in the targeting dimension). A maximum of 10,000 publisher IDs can be targeted. Publisher IDs are found in [ads.txt](https://iabtechlab.com/ads-txt/) / [app-ads.txt](https://iabtechlab.com/app-ads-txt/) and in bid requests in the `BidRequest.publisher_id` field on the [Google RTB protocol](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) or the `BidRequest.site.publisher.id` / `BidRequest.app.publisher.id` field on the [OpenRTB protocol](https://developers.google.com/authorized-buyers/rtb/downloads/openrtb-adx-proto). Publisher IDs will be returned in the order that they were entered. |
| `app_targeting` | String |  | Targeting on a subset of app inventory. If APP is listed in targeted_environments, the specified targeting is applied. A maximum of 30,000 app IDs can be targeted. An unset value for targeting allows all app-based bid requests to be sent. Apps can either be targeting positively (bid requests will be sent only if the destination app is listed in the targeting dimension) or negatively (bid requests will be sent only if the destination app is not listed in the targeting dimension). |
| `included_formats` | Vec<String> |  | Creative formats included by this configuration. Only bid requests eligible for at least one of the specified creative formats will be sent. An unset value will allow all bid requests to be sent, regardless of format. |
| `name` | String |  | Output only. Name of the pretargeting configuration that must follow the pattern `bidders/{bidder_account_id}/pretargetingConfigs/{config_id}` |
| `included_creative_dimensions` | Vec<String> |  | Creative dimensions included by this configuration. Only bid requests eligible for at least one of the specified creative dimensions will be sent. An unset value allows all bid requests to be sent, regardless of creative dimension. |
| `included_platforms` | Vec<String> |  | The platforms included by this configration. Bid requests for devices with the specified platform types will be sent. An unset value allows all bid requests to be sent, regardless of platform. |
| `minimum_viewability_decile` | i64 |  | The targeted minimum viewability decile, ranging in values [0, 10]. A value of 5 means that the configuration will only match adslots for which we predict at least 50% viewability. Values > 10 will be rounded down to 10. An unset value or a value of 0 indicates that bid requests will be sent regardless of viewability. |
| `allowed_user_targeting_modes` | Vec<String> |  | Targeting modes included by this configuration. A bid request must allow all the specified targeting modes. An unset value allows all bid requests to be sent, regardless of which targeting modes they allow. |
| `invalid_geo_ids` | Vec<String> |  | Output only. Existing included or excluded geos that are invalid. Previously targeted geos may become invalid due to privacy restrictions. |
| `parent` | String | ✅ | Required. Name of the bidder to create the pretargeting configuration for. Format: bidders/{bidderAccountId} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vertical_targeting` | String | The verticals included or excluded in this configuration as defined in https://developers.google.com/authorized-buyers/rtb/downloads/publisher-verticals |
| `included_languages` | Vec<String> | The languages included in this configuration, represented by their language code. See https://developers.google.com/adwords/api/docs/appendix/languagecodes. |
| `interstitial_targeting` | String | The interstitial targeting specified for this configuration. The unset value will allow bid requests to be sent regardless of whether they are for interstitials or not. |
| `included_user_id_types` | Vec<String> | User identifier types included in this configuration. At least one of the user identifier types specified in this list must be available for the bid request to be sent. |
| `user_list_targeting` | String | The remarketing lists included or excluded in this configuration as defined in UserList. |
| `billing_id` | String | Output only. The identifier that corresponds to this pretargeting configuration that helps buyers track and attribute their spend across their own arbitrary divisions. If a bid request matches more than one configuration, the buyer chooses which billing_id to attribute each of their bids. |
| `display_name` | String | The diplay name associated with this configuration. This name must be unique among all the pretargeting configurations a bidder has. |
| `web_targeting` | String | Targeting on a subset of site inventory. If WEB is listed in included_environments, the specified targeting is applied. A maximum of 50,000 site URLs can be targeted. An unset value for targeting allows all web-based bid requests to be sent. Sites can either be targeting positively (bid requests will be sent only if the destination site is listed in the targeting dimension) or negatively (bid requests will be sent only if the destination site is not listed in the pretargeting configuration). |
| `included_mobile_operating_system_ids` | Vec<String> | The mobile operating systems included in this configuration as defined in https://storage.googleapis.com/adx-rtb-dictionaries/mobile-os.csv |
| `included_environments` | Vec<String> | Environments that are being included. Bid requests will not be sent for a given environment if it is not included. Further restrictions can be applied to included environments to target only a subset of its inventory. An unset value includes all environments. |
| `excluded_content_label_ids` | Vec<String> | The sensitive content category label IDs excluded in this configuration. Bid requests for inventory with any of the specified content label IDs will not be sent. Refer to this file https://storage.googleapis.com/adx-rtb-dictionaries/content-labels.txt for category IDs. |
| `geo_targeting` | String | The geos included or excluded in this configuration defined in https://storage.googleapis.com/adx-rtb-dictionaries/geo-table.csv |
| `maximum_qps` | String | The maximum QPS threshold for this configuration. The bidder should receive no more than this number of bid requests matching this configuration per second across all their bidding endpoints among all trading locations. Further information available at https://developers.google.com/authorized-buyers/rtb/peer-guide |
| `state` | String | Output only. The state of this pretargeting configuration. |
| `publisher_targeting` | String | Targeting on a subset of publisher inventory. Publishers can either be targeted positively (bid requests will be sent only if the publisher is listed in the targeting dimension) or negatively (bid requests will be sent only if the publisher is not listed in the targeting dimension). A maximum of 10,000 publisher IDs can be targeted. Publisher IDs are found in [ads.txt](https://iabtechlab.com/ads-txt/) / [app-ads.txt](https://iabtechlab.com/app-ads-txt/) and in bid requests in the `BidRequest.publisher_id` field on the [Google RTB protocol](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto) or the `BidRequest.site.publisher.id` / `BidRequest.app.publisher.id` field on the [OpenRTB protocol](https://developers.google.com/authorized-buyers/rtb/downloads/openrtb-adx-proto). Publisher IDs will be returned in the order that they were entered. |
| `app_targeting` | String | Targeting on a subset of app inventory. If APP is listed in targeted_environments, the specified targeting is applied. A maximum of 30,000 app IDs can be targeted. An unset value for targeting allows all app-based bid requests to be sent. Apps can either be targeting positively (bid requests will be sent only if the destination app is listed in the targeting dimension) or negatively (bid requests will be sent only if the destination app is not listed in the targeting dimension). |
| `included_formats` | Vec<String> | Creative formats included by this configuration. Only bid requests eligible for at least one of the specified creative formats will be sent. An unset value will allow all bid requests to be sent, regardless of format. |
| `name` | String | Output only. Name of the pretargeting configuration that must follow the pattern `bidders/{bidder_account_id}/pretargetingConfigs/{config_id}` |
| `included_creative_dimensions` | Vec<String> | Creative dimensions included by this configuration. Only bid requests eligible for at least one of the specified creative dimensions will be sent. An unset value allows all bid requests to be sent, regardless of creative dimension. |
| `included_platforms` | Vec<String> | The platforms included by this configration. Bid requests for devices with the specified platform types will be sent. An unset value allows all bid requests to be sent, regardless of platform. |
| `minimum_viewability_decile` | i64 | The targeted minimum viewability decile, ranging in values [0, 10]. A value of 5 means that the configuration will only match adslots for which we predict at least 50% viewability. Values > 10 will be rounded down to 10. An unset value or a value of 0 indicates that bid requests will be sent regardless of viewability. |
| `allowed_user_targeting_modes` | Vec<String> | Targeting modes included by this configuration. A bid request must allow all the specified targeting modes. An unset value allows all bid requests to be sent, regardless of which targeting modes they allow. |
| `invalid_geo_ids` | Vec<String> | Output only. Existing included or excluded geos that are invalid. Previously targeted geos may become invalid due to privacy restrictions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create pretargeting_config
pretargeting_config = provider.realtimebidding_api.Pretargeting_config {
    parent = "value"  # Required. Name of the bidder to create the pretargeting configuration for. Format: bidders/{bidderAccountId}
}

# Access pretargeting_config outputs
pretargeting_config_id = pretargeting_config.id
pretargeting_config_vertical_targeting = pretargeting_config.vertical_targeting
pretargeting_config_included_languages = pretargeting_config.included_languages
pretargeting_config_interstitial_targeting = pretargeting_config.interstitial_targeting
pretargeting_config_included_user_id_types = pretargeting_config.included_user_id_types
pretargeting_config_user_list_targeting = pretargeting_config.user_list_targeting
pretargeting_config_billing_id = pretargeting_config.billing_id
pretargeting_config_display_name = pretargeting_config.display_name
pretargeting_config_web_targeting = pretargeting_config.web_targeting
pretargeting_config_included_mobile_operating_system_ids = pretargeting_config.included_mobile_operating_system_ids
pretargeting_config_included_environments = pretargeting_config.included_environments
pretargeting_config_excluded_content_label_ids = pretargeting_config.excluded_content_label_ids
pretargeting_config_geo_targeting = pretargeting_config.geo_targeting
pretargeting_config_maximum_qps = pretargeting_config.maximum_qps
pretargeting_config_state = pretargeting_config.state
pretargeting_config_publisher_targeting = pretargeting_config.publisher_targeting
pretargeting_config_app_targeting = pretargeting_config.app_targeting
pretargeting_config_included_formats = pretargeting_config.included_formats
pretargeting_config_name = pretargeting_config.name
pretargeting_config_included_creative_dimensions = pretargeting_config.included_creative_dimensions
pretargeting_config_included_platforms = pretargeting_config.included_platforms
pretargeting_config_minimum_viewability_decile = pretargeting_config.minimum_viewability_decile
pretargeting_config_allowed_user_targeting_modes = pretargeting_config.allowed_user_targeting_modes
pretargeting_config_invalid_geo_ids = pretargeting_config.invalid_geo_ids
```

---


### User_list

Creates a new user list.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. Display name of the user list. This must be unique across all user lists for a given account. |
| `url_restriction` | String |  | Required. Deprecated. This will be removed in October 2023. For more information, see the release notes: https://developers.google.com/authorized-buyers/apis/relnotes#real-time-bidding-api The URL restriction for the user list. |
| `description` | String |  | The description for the user list. |
| `membership_duration_days` | String |  | Required. The number of days a user's cookie stays on the user list. The field must be between 0 and 540 inclusive. |
| `name` | String |  | Output only. Name of the user list that must follow the pattern `buyers/{buyer}/userLists/{user_list}`, where `{buyer}` represents the account ID of the buyer who owns the user list. For a bidder accessing user lists on behalf of a child seat buyer, `{buyer}` represents the account ID of the child seat buyer. `{user_list}` is an int64 identifier assigned by Google to uniquely identify a user list. |
| `status` | String |  | Output only. The status of the user list. A new user list starts out as open. |
| `parent` | String | ✅ | Required. The name of the parent buyer of the user list to be retrieved, which must follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}` represents the account ID of the buyer who owns the user list. For a bidder accessing user lists on behalf of a child seat buyer, `{buyerAccountId}` should represent the account ID of the child seat buyer. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. Display name of the user list. This must be unique across all user lists for a given account. |
| `url_restriction` | String | Required. Deprecated. This will be removed in October 2023. For more information, see the release notes: https://developers.google.com/authorized-buyers/apis/relnotes#real-time-bidding-api The URL restriction for the user list. |
| `description` | String | The description for the user list. |
| `membership_duration_days` | String | Required. The number of days a user's cookie stays on the user list. The field must be between 0 and 540 inclusive. |
| `name` | String | Output only. Name of the user list that must follow the pattern `buyers/{buyer}/userLists/{user_list}`, where `{buyer}` represents the account ID of the buyer who owns the user list. For a bidder accessing user lists on behalf of a child seat buyer, `{buyer}` represents the account ID of the child seat buyer. `{user_list}` is an int64 identifier assigned by Google to uniquely identify a user list. |
| `status` | String | Output only. The status of the user list. A new user list starts out as open. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_list
user_list = provider.realtimebidding_api.User_list {
    parent = "value"  # Required. The name of the parent buyer of the user list to be retrieved, which must follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}` represents the account ID of the buyer who owns the user list. For a bidder accessing user lists on behalf of a child seat buyer, `{buyerAccountId}` should represent the account ID of the child seat buyer.
}

# Access user_list outputs
user_list_id = user_list.id
user_list_display_name = user_list.display_name
user_list_url_restriction = user_list.url_restriction
user_list_description = user_list.description
user_list_membership_duration_days = user_list.membership_duration_days
user_list_name = user_list.name
user_list_status = user_list.status
```

---


### Publisher_connection

Batch approves multiple publisher connections.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `names` | Vec<String> |  | Required. The names of the publishers with which connections will be approved. In the pattern `bidders/{bidder}/publisherConnections/{publisher}` where `{bidder}` is the account ID of the bidder, and `{publisher}` is the ads.txt/app-ads.txt publisher ID. |
| `parent` | String | ✅ | Required. The bidder for whom publisher connections will be approved. Format: `bidders/{bidder}` where `{bidder}` is the account ID of the bidder. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time at which the publisher initiated a connection with the bidder (irrespective of if or when the bidder approves it). This is subsequently updated if the publisher revokes and re-initiates the connection. |
| `publisher_platform` | String | Output only. Whether the publisher is an Ad Manager or AdMob publisher. |
| `name` | String | Output only. Name of the publisher connection. This follows the pattern `bidders/{bidder}/publisherConnections/{publisher}`, where `{bidder}` represents the account ID of the bidder, and `{publisher}` is the ads.txt/app-ads.txt publisher ID. |
| `bidding_state` | String | Whether the publisher has been approved by the bidder. |
| `display_name` | String | Output only. Publisher display name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create publisher_connection
publisher_connection = provider.realtimebidding_api.Publisher_connection {
    parent = "value"  # Required. The bidder for whom publisher connections will be approved. Format: `bidders/{bidder}` where `{bidder}` is the account ID of the bidder.
}

# Access publisher_connection outputs
publisher_connection_id = publisher_connection.id
publisher_connection_create_time = publisher_connection.create_time
publisher_connection_publisher_platform = publisher_connection.publisher_platform
publisher_connection_name = publisher_connection.name
publisher_connection_bidding_state = publisher_connection.bidding_state
publisher_connection_display_name = publisher_connection.display_name
```

---


### Creative

Creates a creative.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `native` | String |  | A native creative. |
| `declared_attributes` | Vec<String> |  | All declared attributes for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method. If the `excluded_attribute` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto") contains one of the attributes that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction. |
| `video` | String |  | A video creative. |
| `deal_ids` | Vec<String> |  | Output only. IDs of all of the deals with which this creative has been used in bidding. Can be used to filter the response of the creatives.list method. |
| `agency_id` | String |  | The agency ID for this creative. |
| `html` | String |  | An HTML creative. |
| `impression_tracking_urls` | Vec<String> |  | The set of URLs to be called to record an impression. |
| `advertiser_name` | String |  | The name of the company being advertised in the creative. Can be used to filter the response of the creatives.list method. |
| `declared_restricted_categories` | Vec<String> |  | All declared restricted categories for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method. |
| `declared_click_through_urls` | Vec<String> |  | The set of declared destination URLs for the creative. Can be used to filter the response of the creatives.list method. |
| `api_update_time` | String |  | Output only. The last update timestamp of the creative through the API. |
| `creative_id` | String |  | Buyer-specific creative ID that references this creative in bid responses. This field is Ignored in update operations. Can be used to filter the response of the creatives.list method. The maximum length of the creative ID is 128 bytes. |
| `restricted_categories` | Vec<String> |  | All restricted categories for the ads that may be shown from this creative. |
| `render_url` | String |  | Experimental field that can be used during the [FLEDGE Origin Trial](/authorized-buyers/rtb/fledge-origin-trial). The URL to fetch an interest group ad used in [TURTLEDOVE on-device auction](https://github.com/WICG/turtledove/blob/main/FLEDGE.md#1-browsers-record-interest-groups"). This should be unique among all creatives for a given `accountId`. This URL should be the same as the URL returned by [generateBid()](https://github.com/WICG/turtledove/blob/main/FLEDGE.md#32-on-device-bidding). |
| `creative_serving_decision` | String |  | Output only. Top level status and detected attributes of a creative (for example domain, language, advertiser, product category, etc.) that affect whether (status) and where (context) a creative will be allowed to serve. |
| `declared_vendor_ids` | Vec<i64> |  | IDs for the declared ad technology vendors that may be used by this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values. Can be used to filter the response of the creatives.list method. |
| `creative_format` | String |  | Output only. The format of this creative. Can be used to filter the response of the creatives.list method. |
| `name` | String |  | Output only. Name of the creative. Follows the pattern `buyers/{buyer}/creatives/{creative}`, where `{buyer}` represents the account ID of the buyer who owns the creative, and `{creative}` is the buyer-specific creative ID that references this creative in the bid response. |
| `account_id` | String |  | Output only. ID of the buyer account that this creative is owned by. Can be used to filter the response of the creatives.list method with equality and inequality check. |
| `ad_choices_destination_url` | String |  | The link to AdChoices destination page. This is only supported for native ads. |
| `version` | i64 |  | Output only. The version of the creative. Version for a new creative is 1 and it increments during subsequent creative updates. |
| `parent` | String | ✅ | Required. The name of the parent buyer that the new creative belongs to that must follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}` represents the account ID of the buyer who owns a creative. For a bidder accessing creatives on behalf of a child seat buyer, `{buyerAccountId}` should represent the account ID of the child seat buyer. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `native` | String | A native creative. |
| `declared_attributes` | Vec<String> | All declared attributes for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method. If the `excluded_attribute` field of a [bid request](https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto") contains one of the attributes that were declared or detected for a given creative, and a bid is submitted with that creative, the bid will be filtered before the auction. |
| `video` | String | A video creative. |
| `deal_ids` | Vec<String> | Output only. IDs of all of the deals with which this creative has been used in bidding. Can be used to filter the response of the creatives.list method. |
| `agency_id` | String | The agency ID for this creative. |
| `html` | String | An HTML creative. |
| `impression_tracking_urls` | Vec<String> | The set of URLs to be called to record an impression. |
| `advertiser_name` | String | The name of the company being advertised in the creative. Can be used to filter the response of the creatives.list method. |
| `declared_restricted_categories` | Vec<String> | All declared restricted categories for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method. |
| `declared_click_through_urls` | Vec<String> | The set of declared destination URLs for the creative. Can be used to filter the response of the creatives.list method. |
| `api_update_time` | String | Output only. The last update timestamp of the creative through the API. |
| `creative_id` | String | Buyer-specific creative ID that references this creative in bid responses. This field is Ignored in update operations. Can be used to filter the response of the creatives.list method. The maximum length of the creative ID is 128 bytes. |
| `restricted_categories` | Vec<String> | All restricted categories for the ads that may be shown from this creative. |
| `render_url` | String | Experimental field that can be used during the [FLEDGE Origin Trial](/authorized-buyers/rtb/fledge-origin-trial). The URL to fetch an interest group ad used in [TURTLEDOVE on-device auction](https://github.com/WICG/turtledove/blob/main/FLEDGE.md#1-browsers-record-interest-groups"). This should be unique among all creatives for a given `accountId`. This URL should be the same as the URL returned by [generateBid()](https://github.com/WICG/turtledove/blob/main/FLEDGE.md#32-on-device-bidding). |
| `creative_serving_decision` | String | Output only. Top level status and detected attributes of a creative (for example domain, language, advertiser, product category, etc.) that affect whether (status) and where (context) a creative will be allowed to serve. |
| `declared_vendor_ids` | Vec<i64> | IDs for the declared ad technology vendors that may be used by this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values. Can be used to filter the response of the creatives.list method. |
| `creative_format` | String | Output only. The format of this creative. Can be used to filter the response of the creatives.list method. |
| `name` | String | Output only. Name of the creative. Follows the pattern `buyers/{buyer}/creatives/{creative}`, where `{buyer}` represents the account ID of the buyer who owns the creative, and `{creative}` is the buyer-specific creative ID that references this creative in the bid response. |
| `account_id` | String | Output only. ID of the buyer account that this creative is owned by. Can be used to filter the response of the creatives.list method with equality and inequality check. |
| `ad_choices_destination_url` | String | The link to AdChoices destination page. This is only supported for native ads. |
| `version` | i64 | Output only. The version of the creative. Version for a new creative is 1 and it increments during subsequent creative updates. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create creative
creative = provider.realtimebidding_api.Creative {
    parent = "value"  # Required. The name of the parent buyer that the new creative belongs to that must follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}` represents the account ID of the buyer who owns a creative. For a bidder accessing creatives on behalf of a child seat buyer, `{buyerAccountId}` should represent the account ID of the child seat buyer.
}

# Access creative outputs
creative_id = creative.id
creative_native = creative.native
creative_declared_attributes = creative.declared_attributes
creative_video = creative.video
creative_deal_ids = creative.deal_ids
creative_agency_id = creative.agency_id
creative_html = creative.html
creative_impression_tracking_urls = creative.impression_tracking_urls
creative_advertiser_name = creative.advertiser_name
creative_declared_restricted_categories = creative.declared_restricted_categories
creative_declared_click_through_urls = creative.declared_click_through_urls
creative_api_update_time = creative.api_update_time
creative_creative_id = creative.creative_id
creative_restricted_categories = creative.restricted_categories
creative_render_url = creative.render_url
creative_creative_serving_decision = creative.creative_serving_decision
creative_declared_vendor_ids = creative.declared_vendor_ids
creative_creative_format = creative.creative_format
creative_name = creative.name
creative_account_id = creative.account_id
creative_ad_choices_destination_url = creative.ad_choices_destination_url
creative_version = creative.version
```

---


### Bidder

Gets a bidder account by its name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cookie_matching_network_id` | String | Output only. The buyer's network ID used for cookie matching. This ID corresponds to the `google_nid` parameter in the URL used in cookie match requests. Refer to https://developers.google.com/authorized-buyers/rtb/cookie-guide for further information. |
| `name` | String | Output only. Name of the bidder resource that must follow the pattern `bidders/{bidderAccountId}`, where `{bidderAccountId}` is the account ID of the bidder whose information is to be received. One can get their account ID on the Authorized Buyers or Open Bidding UI, or by contacting their Google account manager. |
| `cookie_matching_url` | String | Output only. The base URL used in cookie match requests. Refer to https://developers.google.com/authorized-buyers/rtb/cookie-guide for further information. |
| `deals_billing_id` | String | Output only. The billing ID for the deals pretargeting config. This billing ID is sent on the bid request for guaranteed and nonguaranteed deals matched in pretargeting. |
| `bypass_nonguaranteed_deals_pretargeting` | bool | Output only. An option to bypass pretargeting for private auctions and preferred deals. When true, bid requests from these nonguaranteed deals will always be sent. When false, bid requests will be subject to regular pretargeting configurations. Programmatic Guaranteed deals will always be sent to the bidder, regardless of the value for this option. Auction packages are not impacted by this value and are subject to the regular pretargeting configurations. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access bidder outputs
bidder_id = bidder.id
bidder_cookie_matching_network_id = bidder.cookie_matching_network_id
bidder_name = bidder.name
bidder_cookie_matching_url = bidder.cookie_matching_url
bidder_deals_billing_id = bidder.deals_billing_id
bidder_bypass_nonguaranteed_deals_pretargeting = bidder.bypass_nonguaranteed_deals_pretargeting
```

---


### Buyer

Gets a buyer account by its name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Output only. The diplay name associated with this buyer account, as visible to sellers. |
| `maximum_active_creative_count` | String | Output only. The maximum number of active creatives that this buyer can have. |
| `name` | String | Output only. Name of the buyer resource that must follow the pattern `buyers/{buyerAccountId}`, where `{buyerAccountId}` is the account ID of the buyer account whose information is to be received. One can get their account ID on the Authorized Buyers or Open Bidding UI, or by contacting their Google account manager. |
| `bidder` | String | Output only. The name of the bidder resource that is responsible for receiving bidding traffic for this account. The bidder name must follow the pattern `bidders/{bidderAccountId}`, where `{bidderAccountId}` is the account ID of the bidder receiving traffic for this buyer. |
| `active_creative_count` | String | Output only. The number of creatives that this buyer submitted through the API or bid with in the last 30 days. This is counted against the maximum number of active creatives. |
| `billing_ids` | Vec<String> | Output only. A list of billing IDs associated with this account. These IDs appear on: 1. A bid request, to signal which buyers are eligible to bid on a given opportunity, and which pretargeting configurations were matched for each eligible buyer. 2. The bid response, to attribute a winning impression to a specific account for billing, reporting, policy and publisher block enforcement. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access buyer outputs
buyer_id = buyer.id
buyer_display_name = buyer.display_name
buyer_maximum_active_creative_count = buyer.maximum_active_creative_count
buyer_name = buyer.name
buyer_bidder = buyer.bidder
buyer_active_creative_count = buyer.active_creative_count
buyer_billing_ids = buyer.billing_ids
```

---


### Endpoint

Gets a bidder endpoint by its name.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | String |  | Output only. The URL that bid requests should be sent to. |
| `name` | String |  | Output only. Name of the endpoint resource that must follow the pattern `bidders/{bidderAccountId}/endpoints/{endpointId}`, where {bidderAccountId} is the account ID of the bidder who operates this endpoint, and {endpointId} is a unique ID assigned by the server. |
| `maximum_qps` | String |  | The maximum number of queries per second allowed to be sent to this server. |
| `bid_protocol` | String |  | The protocol that the bidder endpoint is using. |
| `trading_location` | String |  | The trading location that bid requests should be sent from. See https://developers.google.com/authorized-buyers/rtb/peer-guide#trading-locations for further information. |
| `name` | String | ✅ | Output only. Name of the endpoint resource that must follow the pattern `bidders/{bidderAccountId}/endpoints/{endpointId}`, where {bidderAccountId} is the account ID of the bidder who operates this endpoint, and {endpointId} is a unique ID assigned by the server. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `url` | String | Output only. The URL that bid requests should be sent to. |
| `name` | String | Output only. Name of the endpoint resource that must follow the pattern `bidders/{bidderAccountId}/endpoints/{endpointId}`, where {bidderAccountId} is the account ID of the bidder who operates this endpoint, and {endpointId} is a unique ID assigned by the server. |
| `maximum_qps` | String | The maximum number of queries per second allowed to be sent to this server. |
| `bid_protocol` | String | The protocol that the bidder endpoint is using. |
| `trading_location` | String | The trading location that bid requests should be sent from. See https://developers.google.com/authorized-buyers/rtb/peer-guide#trading-locations for further information. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_url = endpoint.url
endpoint_name = endpoint.name
endpoint_maximum_qps = endpoint.maximum_qps
endpoint_bid_protocol = endpoint.bid_protocol
endpoint_trading_location = endpoint.trading_location
```

---


### Bidding_function

Creates a new bidding function.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bidding_function` | String |  | The raw Javascript source code of the bidding function. |
| `name` | String |  | The name of the bidding function that must follow the pattern: `bidders/{bidder_account_id}/biddingFunctions/{bidding_function_name}`. |
| `state` | String |  | Output only. The state of the bidding function. |
| `type` | String |  | The type of the bidding function to be created. |
| `parent` | String | ✅ | Required. The name of the bidder for which to create the bidding function. Format: `bidders/{bidderAccountId}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token which can be passed to a subsequent call to the `ListBiddingFunctions` method to retrieve the next page of results in ListBiddingFunctionsRequest.pageToken. |
| `bidding_functions` | Vec<String> | A list of a bidder's bidding functions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bidding_function
bidding_function = provider.realtimebidding_api.Bidding_function {
    parent = "value"  # Required. The name of the bidder for which to create the bidding function. Format: `bidders/{bidderAccountId}`
}

# Access bidding_function outputs
bidding_function_id = bidding_function.id
bidding_function_next_page_token = bidding_function.next_page_token
bidding_function_bidding_functions = bidding_function.bidding_functions
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple pretargeting_config resources
pretargeting_config_0 = provider.realtimebidding_api.Pretargeting_config {
    parent = "value-0"
}
pretargeting_config_1 = provider.realtimebidding_api.Pretargeting_config {
    parent = "value-1"
}
pretargeting_config_2 = provider.realtimebidding_api.Pretargeting_config {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    pretargeting_config = provider.realtimebidding_api.Pretargeting_config {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Realtimebidding_api Documentation](https://cloud.google.com/realtimebidding_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
