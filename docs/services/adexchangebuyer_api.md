# Adexchangebuyer_api Service



**Resources**: 21

---

## Overview

The adexchangebuyer_api service provides access to 21 resource types:

- [Proposal](#proposal) [CRU]
- [Product](#product) [R]
- [Marketplacenote](#marketplacenote) [CR]
- [Pubprofile](#pubprofile) [R]
- [Pretargeting_config](#pretargeting_config) [CRUD]
- [Billing_info](#billing_info) [R]
- [Budget](#budget) [RU]
- [Creative](#creative) [CR]
- [Marketplacedeal](#marketplacedeal) [CRUD]
- [Marketplaceprivateauction](#marketplaceprivateauction) [C]
- [Performance_report](#performance_report) [R]
- [Account](#account) [RU]
- [Performance_report](#performance_report) [R]
- [Budget](#budget) [RU]
- [Direct_deal](#direct_deal) [R]
- [Pretargeting_config](#pretargeting_config) [CRUD]
- [Account](#account) [RU]
- [Billing_info](#billing_info) [R]
- [Creative](#creative) [CR]
- [Creative](#creative) [CR]
- [Account](#account) [RU]

---

## Resources


### Proposal

Create the given list of proposals

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `proposals` | Vec<String> |  | The list of proposals to create. |
| `web_property_code` | String |  | Web property id of the seller creating these orders |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `negotiation_id` | String | Optional negotiation id if this proposal is a preferred deal proposal. |
| `originator_role` | String | Indicates whether the buyer/seller created the proposal.(readonly) |
| `seller` | String | Reference to the seller on the proposal. (readonly, except on create) |
| `dbm_advertiser_ids` | Vec<String> | IDs of DBM advertisers permission to this proposal. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "adexchangebuyer#proposal". |
| `private_auction_id` | String | Optional private auction id if this proposal is a private auction proposal. |
| `labels` | Vec<String> | List of labels associated with the proposal. (readonly) |
| `name` | String | The name for the proposal (updatable) |
| `seller_contacts` | Vec<String> | Optional contact information of the seller (buyer-readonly). |
| `has_seller_signed_off` | bool | When an proposal is in an accepted state, indicates whether the buyer has signed off Once both sides have signed off on a deal, the proposal can be finalized by the seller. (buyer-readonly) |
| `buyer_private_data` | String | Private data for buyer. (hidden from seller). |
| `revision_time_ms` | String | The time (ms since epoch) when the proposal was last revised (readonly). |
| `buyer` | String | Reference to the buyer on the proposal. (readonly, except on create) |
| `buyer_contacts` | Vec<String> | Optional contact information of the buyer. (seller-readonly) |
| `last_updater_or_commentor_role` | String | The role of the last user that either updated the proposal or left a comment. (readonly) |
| `proposal_id` | String | The unique id of the proposal. (readonly). |
| `billed_buyer` | String | Reference to the buyer that will get billed for this proposal. (readonly) |
| `proposal_state` | String | The current state of the proposal. (readonly) |
| `revision_number` | String | The revision number for the proposal (readonly). |
| `is_renegotiating` | bool | True if the proposal is being renegotiated (readonly). |
| `is_setup_complete` | bool | True, if the buyside inventory setup is complete for this proposal. (readonly, except via OrderSetupCompleted action) Deprecated in favor of deal level setup complete flag. |
| `inventory_source` | String | What exchange will provide this inventory (readonly, except on create). |
| `has_buyer_signed_off` | bool | When an proposal is in an accepted state, indicates whether the buyer has signed off. Once both sides have signed off on a deal, the proposal can be finalized by the seller. (seller-readonly) |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create proposal
proposal = provider.adexchangebuyer_api.Proposal {
}

# Access proposal outputs
proposal_id = proposal.id
proposal_negotiation_id = proposal.negotiation_id
proposal_originator_role = proposal.originator_role
proposal_seller = proposal.seller
proposal_dbm_advertiser_ids = proposal.dbm_advertiser_ids
proposal_kind = proposal.kind
proposal_private_auction_id = proposal.private_auction_id
proposal_labels = proposal.labels
proposal_name = proposal.name
proposal_seller_contacts = proposal.seller_contacts
proposal_has_seller_signed_off = proposal.has_seller_signed_off
proposal_buyer_private_data = proposal.buyer_private_data
proposal_revision_time_ms = proposal.revision_time_ms
proposal_buyer = proposal.buyer
proposal_buyer_contacts = proposal.buyer_contacts
proposal_last_updater_or_commentor_role = proposal.last_updater_or_commentor_role
proposal_proposal_id = proposal.proposal_id
proposal_billed_buyer = proposal.billed_buyer
proposal_proposal_state = proposal.proposal_state
proposal_revision_number = proposal.revision_number
proposal_is_renegotiating = proposal.is_renegotiating
proposal_is_setup_complete = proposal.is_setup_complete
proposal_inventory_source = proposal.inventory_source
proposal_has_buyer_signed_off = proposal.has_buyer_signed_off
```

---


### Product

Gets the requested product by id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time_ms` | String | Creation time in ms. since epoch (readonly) |
| `buyer` | String | The buyer that created the offer if this is a buyer initiated offer (readonly, except on create) |
| `legacy_offer_id` | String | Optional legacy offer id if this offer is a preferred deal offer. |
| `billed_buyer` | String | The billed buyer corresponding to the buyer that created the offer. (readonly, except on create) |
| `inventory_source` | String | What exchange will provide this inventory (readonly, except on create). |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "adexchangebuyer#product". |
| `terms` | String | The negotiable terms of the deal (buyer-readonly) |
| `web_property_code` | String | The web property code for the seller. This field is meant to be copied over as is when creating deals. |
| `publisher_profile_id` | String | Id of the publisher profile for a given seller. A (seller.account_id, publisher_profile_id) pair uniquely identifies a publisher profile. Buyers can call the PublisherProfiles::List endpoint to get a list of publisher profiles for a given seller. |
| `last_update_time_ms` | String | Time of last update in ms. since epoch (readonly) |
| `flight_end_time_ms` | String | The proposed end time for the deal (ms since epoch) (buyer-readonly) |
| `revision_number` | String | The revision number of the product. (readonly) |
| `seller` | String | Information about the seller that created this product (readonly, except on create) |
| `shared_targetings` | Vec<String> | Targeting that is shared between the buyer and the seller. Each targeting criteria has a specified key and for each key there is a list of inclusion value or exclusion values. (buyer-readonly) |
| `syndication_product` | String | The syndication product associated with the deal. (readonly, except on create) |
| `flight_start_time_ms` | String | Inventory availability dates. (times are in ms since epoch) The granularity is generally in the order of seconds. (buyer-readonly) |
| `labels` | Vec<String> | Optional List of labels for the product (optional, buyer-readonly). |
| `creator_contacts` | Vec<String> | Optional contact information for the creator of this product. (buyer-readonly) |
| `private_auction_id` | String | Optional private auction id if this offer is a private auction offer. |
| `has_creator_signed_off` | bool | If the creator has already signed off on the product, then the buyer can finalize the deal by accepting the product as is. When copying to a proposal, if any of the terms are changed, then auto_finalize is automatically set to false. |
| `creator_role` | String | The role that created the offer. Set to BUYER for buyer initiated offers. |
| `marketplace_publisher_profile_id` | String | Marketplace publisher profile Id. This Id differs from the regular publisher_profile_id in that 1. This is a new id, the old Id will be deprecated in 2017. 2. This id uniquely identifies a publisher profile by itself. |
| `name` | String | The name for this product as set by the seller. (buyer-readonly) |
| `publisher_provided_forecast` | String | Publisher self-provided forecast information. |
| `delivery_control` | String | The set of fields around delivery control that are interesting for a buyer to see but are non-negotiable. These are set by the publisher. This message is assigned an id of 100 since some day we would want to model this as a protobuf extension. |
| `state` | String | The state of the product. (buyer-readonly) |
| `product_id` | String | The unique id for the product (readonly) |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access product outputs
product_id = product.id
product_creation_time_ms = product.creation_time_ms
product_buyer = product.buyer
product_legacy_offer_id = product.legacy_offer_id
product_billed_buyer = product.billed_buyer
product_inventory_source = product.inventory_source
product_kind = product.kind
product_terms = product.terms
product_web_property_code = product.web_property_code
product_publisher_profile_id = product.publisher_profile_id
product_last_update_time_ms = product.last_update_time_ms
product_flight_end_time_ms = product.flight_end_time_ms
product_revision_number = product.revision_number
product_seller = product.seller
product_shared_targetings = product.shared_targetings
product_syndication_product = product.syndication_product
product_flight_start_time_ms = product.flight_start_time_ms
product_labels = product.labels
product_creator_contacts = product.creator_contacts
product_private_auction_id = product.private_auction_id
product_has_creator_signed_off = product.has_creator_signed_off
product_creator_role = product.creator_role
product_marketplace_publisher_profile_id = product.marketplace_publisher_profile_id
product_name = product.name
product_publisher_provided_forecast = product.publisher_provided_forecast
product_delivery_control = product.delivery_control
product_state = product.state
product_product_id = product.product_id
```

---


### Marketplacenote

Add notes to the proposal

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notes` | Vec<String> |  | The list of notes to add. |
| `proposal_id` | String | ✅ | The proposalId to add notes for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notes` | Vec<String> | The list of matching notes. The notes for a proposal are ordered from oldest to newest. If the notes span multiple proposals, they will be grouped by proposal, with the notes for the most recently modified proposal appearing first. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create marketplacenote
marketplacenote = provider.adexchangebuyer_api.Marketplacenote {
    proposal_id = "value"  # The proposalId to add notes for.
}

# Access marketplacenote outputs
marketplacenote_id = marketplacenote.id
marketplacenote_notes = marketplacenote.notes
```

---


### Pubprofile

Gets the requested publisher profile(s) by publisher accountId.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `profiles` | Vec<String> | Profiles for the requested publisher |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access pubprofile outputs
pubprofile_id = pubprofile.id
pubprofile_profiles = pubprofile.profiles
```

---


### Pretargeting_config

Inserts a new pretargeting configuration.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `excluded_geo_criteria_ids` | Vec<String> |  | Requests containing any of these geo criteria ids will not match. |
| `excluded_placements` | Vec<String> |  | Requests containing any of these placements will not match. |
| `user_identifier_data_required` | Vec<String> |  | Requests containing the specified type of user data will match. Possible values are HOSTED_MATCH_DATA, which means the request is cookie-targetable and has a match in the buyer's hosted match table, and COOKIE_OR_IDFA, which means the request has either a targetable cookie or an iOS IDFA. |
| `maximum_qps` | String |  | The maximum QPS allocated to this pretargeting configuration, used for pretargeting-level QPS limits. By default, this is not set, which indicates that there is no QPS limit at the configuration level (a global or account-level limit may still be imposed). |
| `user_lists` | Vec<String> |  | Requests containing any of these user list ids will match. |
| `excluded_content_labels` | Vec<String> |  | Requests with any of these content labels will not match. Values are from content-labels.txt in the downloadable files section. |
| `platforms` | Vec<String> |  | Requests matching any of these platforms will match. Possible values are PRETARGETING_PLATFORM_MOBILE, PRETARGETING_PLATFORM_DESKTOP, and PRETARGETING_PLATFORM_TABLET. |
| `vendor_types` | Vec<String> |  | Requests that allow any of these vendor ids will match. Values are from vendors.txt in the downloadable files section. |
| `mobile_carriers` | Vec<String> |  | Requests containing any of these mobile carrier ids will match. Values are from mobile-carriers.csv in the downloadable files section. |
| `video_player_sizes` | Vec<String> |  | Video requests satisfying any of these player size constraints will match. |
| `geo_criteria_ids` | Vec<String> |  | Requests containing any of these geo criteria ids will match. |
| `config_id` | String |  | The config id; generated automatically. Leave this field blank for insert requests. |
| `creative_type` | Vec<String> |  | List must contain exactly one of PRETARGETING_CREATIVE_TYPE_HTML or PRETARGETING_CREATIVE_TYPE_VIDEO. |
| `billing_id` | String |  | The id for billing purposes, provided for reference. Leave this field blank for insert requests; the id will be generated automatically. |
| `excluded_user_lists` | Vec<String> |  | Requests containing any of these users list ids will not match. |
| `languages` | Vec<String> |  | Request containing any of these language codes will match. |
| `minimum_viewability_decile` | i64 |  | Requests where the predicted viewability is below the specified decile will not match. E.g. if the buyer sets this value to 5, requests from slots where the predicted viewability is below 50% will not match. If the predicted viewability is unknown this field will be ignored. |
| `mobile_devices` | Vec<String> |  | Requests containing any of these mobile device ids will match. Values are from mobile-devices.csv in the downloadable files section. |
| `dimensions` | Vec<String> |  | Requests which allow one of these (width, height) pairs will match. All pairs must be supported ad dimensions. |
| `placements` | Vec<String> |  | Requests containing any of these placements will match. |
| `verticals` | Vec<String> |  | Requests containing any of these vertical ids will match. |
| `supported_creative_attributes` | Vec<String> |  | Creative attributes should be declared here if all creatives corresponding to this pretargeting configuration have that creative attribute. Values are from pretargetable-creative-attributes.txt in the downloadable files section. |
| `excluded_verticals` | Vec<String> |  | Requests containing any of these vertical ids will not match. Values are from the publisher-verticals.txt file in the downloadable files section. |
| `kind` | String |  | The kind of the resource, i.e. "adexchangebuyer#pretargetingConfig". |
| `config_name` | String |  | The name of the config. Must be unique. Required for all requests. |
| `mobile_operating_system_versions` | Vec<String> |  | Requests containing any of these mobile operating system version ids will match. Values are from mobile-os.csv in the downloadable files section. |
| `is_active` | bool |  | Whether this config is active. Required for all requests. |
| `account_id` | String | ✅ | The account id to insert the pretargeting config for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `excluded_geo_criteria_ids` | Vec<String> | Requests containing any of these geo criteria ids will not match. |
| `excluded_placements` | Vec<String> | Requests containing any of these placements will not match. |
| `user_identifier_data_required` | Vec<String> | Requests containing the specified type of user data will match. Possible values are HOSTED_MATCH_DATA, which means the request is cookie-targetable and has a match in the buyer's hosted match table, and COOKIE_OR_IDFA, which means the request has either a targetable cookie or an iOS IDFA. |
| `maximum_qps` | String | The maximum QPS allocated to this pretargeting configuration, used for pretargeting-level QPS limits. By default, this is not set, which indicates that there is no QPS limit at the configuration level (a global or account-level limit may still be imposed). |
| `user_lists` | Vec<String> | Requests containing any of these user list ids will match. |
| `excluded_content_labels` | Vec<String> | Requests with any of these content labels will not match. Values are from content-labels.txt in the downloadable files section. |
| `platforms` | Vec<String> | Requests matching any of these platforms will match. Possible values are PRETARGETING_PLATFORM_MOBILE, PRETARGETING_PLATFORM_DESKTOP, and PRETARGETING_PLATFORM_TABLET. |
| `vendor_types` | Vec<String> | Requests that allow any of these vendor ids will match. Values are from vendors.txt in the downloadable files section. |
| `mobile_carriers` | Vec<String> | Requests containing any of these mobile carrier ids will match. Values are from mobile-carriers.csv in the downloadable files section. |
| `video_player_sizes` | Vec<String> | Video requests satisfying any of these player size constraints will match. |
| `geo_criteria_ids` | Vec<String> | Requests containing any of these geo criteria ids will match. |
| `config_id` | String | The config id; generated automatically. Leave this field blank for insert requests. |
| `creative_type` | Vec<String> | List must contain exactly one of PRETARGETING_CREATIVE_TYPE_HTML or PRETARGETING_CREATIVE_TYPE_VIDEO. |
| `billing_id` | String | The id for billing purposes, provided for reference. Leave this field blank for insert requests; the id will be generated automatically. |
| `excluded_user_lists` | Vec<String> | Requests containing any of these users list ids will not match. |
| `languages` | Vec<String> | Request containing any of these language codes will match. |
| `minimum_viewability_decile` | i64 | Requests where the predicted viewability is below the specified decile will not match. E.g. if the buyer sets this value to 5, requests from slots where the predicted viewability is below 50% will not match. If the predicted viewability is unknown this field will be ignored. |
| `mobile_devices` | Vec<String> | Requests containing any of these mobile device ids will match. Values are from mobile-devices.csv in the downloadable files section. |
| `dimensions` | Vec<String> | Requests which allow one of these (width, height) pairs will match. All pairs must be supported ad dimensions. |
| `placements` | Vec<String> | Requests containing any of these placements will match. |
| `verticals` | Vec<String> | Requests containing any of these vertical ids will match. |
| `supported_creative_attributes` | Vec<String> | Creative attributes should be declared here if all creatives corresponding to this pretargeting configuration have that creative attribute. Values are from pretargetable-creative-attributes.txt in the downloadable files section. |
| `excluded_verticals` | Vec<String> | Requests containing any of these vertical ids will not match. Values are from the publisher-verticals.txt file in the downloadable files section. |
| `kind` | String | The kind of the resource, i.e. "adexchangebuyer#pretargetingConfig". |
| `config_name` | String | The name of the config. Must be unique. Required for all requests. |
| `mobile_operating_system_versions` | Vec<String> | Requests containing any of these mobile operating system version ids will match. Values are from mobile-os.csv in the downloadable files section. |
| `is_active` | bool | Whether this config is active. Required for all requests. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create pretargeting_config
pretargeting_config = provider.adexchangebuyer_api.Pretargeting_config {
    account_id = "value"  # The account id to insert the pretargeting config for.
}

# Access pretargeting_config outputs
pretargeting_config_id = pretargeting_config.id
pretargeting_config_excluded_geo_criteria_ids = pretargeting_config.excluded_geo_criteria_ids
pretargeting_config_excluded_placements = pretargeting_config.excluded_placements
pretargeting_config_user_identifier_data_required = pretargeting_config.user_identifier_data_required
pretargeting_config_maximum_qps = pretargeting_config.maximum_qps
pretargeting_config_user_lists = pretargeting_config.user_lists
pretargeting_config_excluded_content_labels = pretargeting_config.excluded_content_labels
pretargeting_config_platforms = pretargeting_config.platforms
pretargeting_config_vendor_types = pretargeting_config.vendor_types
pretargeting_config_mobile_carriers = pretargeting_config.mobile_carriers
pretargeting_config_video_player_sizes = pretargeting_config.video_player_sizes
pretargeting_config_geo_criteria_ids = pretargeting_config.geo_criteria_ids
pretargeting_config_config_id = pretargeting_config.config_id
pretargeting_config_creative_type = pretargeting_config.creative_type
pretargeting_config_billing_id = pretargeting_config.billing_id
pretargeting_config_excluded_user_lists = pretargeting_config.excluded_user_lists
pretargeting_config_languages = pretargeting_config.languages
pretargeting_config_minimum_viewability_decile = pretargeting_config.minimum_viewability_decile
pretargeting_config_mobile_devices = pretargeting_config.mobile_devices
pretargeting_config_dimensions = pretargeting_config.dimensions
pretargeting_config_placements = pretargeting_config.placements
pretargeting_config_verticals = pretargeting_config.verticals
pretargeting_config_supported_creative_attributes = pretargeting_config.supported_creative_attributes
pretargeting_config_excluded_verticals = pretargeting_config.excluded_verticals
pretargeting_config_kind = pretargeting_config.kind
pretargeting_config_config_name = pretargeting_config.config_name
pretargeting_config_mobile_operating_system_versions = pretargeting_config.mobile_operating_system_versions
pretargeting_config_is_active = pretargeting_config.is_active
```

---


### Billing_info

Returns the billing information for one account specified by account ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | i64 | Account id. |
| `account_name` | String | Account name. |
| `billing_id` | Vec<String> | A list of adgroup IDs associated with this particular account. These IDs may show up as part of a realtime bidding BidRequest, which indicates a bid request for this account. |
| `kind` | String | Resource type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access billing_info outputs
billing_info_id = billing_info.id
billing_info_account_id = billing_info.account_id
billing_info_account_name = billing_info.account_name
billing_info_billing_id = billing_info.billing_id
billing_info_kind = billing_info.kind
```

---


### Budget

Returns the budget information for the adgroup specified by the accountId and billingId.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `budget_amount` | String |  | The daily budget amount in unit amount of the account currency to apply for the billingId provided. This is required for update requests. |
| `account_id` | String |  | The id of the account. This is required for get and update requests. |
| `billing_id` | String |  | The billing id to determine which adgroup to provide budget information for. This is required for get and update requests. |
| `currency_code` | String |  | The currency code for the buyer. This cannot be altered here. |
| `id` | String |  | The unique id that describes this item. |
| `kind` | String |  | The kind of the resource, i.e. "adexchangebuyer#budget". |
| `account_id` | String | ✅ | The account id associated with the budget being updated. |
| `billing_id` | String | ✅ | The billing id associated with the budget being updated. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `budget_amount` | String | The daily budget amount in unit amount of the account currency to apply for the billingId provided. This is required for update requests. |
| `account_id` | String | The id of the account. This is required for get and update requests. |
| `billing_id` | String | The billing id to determine which adgroup to provide budget information for. This is required for get and update requests. |
| `currency_code` | String | The currency code for the buyer. This cannot be altered here. |
| `id` | String | The unique id that describes this item. |
| `kind` | String | The kind of the resource, i.e. "adexchangebuyer#budget". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access budget outputs
budget_id = budget.id
budget_budget_amount = budget.budget_amount
budget_account_id = budget.account_id
budget_billing_id = budget.billing_id
budget_currency_code = budget.currency_code
budget_id = budget.id
budget_kind = budget.kind
```

---


### Creative

Submit a new creative.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `width` | i64 |  | Ad width. |
| `deals_status` | String |  | Top-level deals status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=DIRECT_DEALS (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from servingRestrictions directly. |
| `vendor_type` | Vec<i64> |  | List of vendor types for the ads that may be shown from this snippet. Each vendor type is represented by an integer as defined in vendors.txt. |
| `creative_status_identity_type` | String |  | Creative status identity type that the creative item applies to. Ad Exchange real-time bidding is migrating to the sizeless creative verification. Originally, Ad Exchange assigned creative verification status to a unique combination of a buyer creative ID and creative dimensions. Post-migration, a single verification status will be assigned at the buyer creative ID level. This field allows to distinguish whether a given creative status applies to a unique combination of a buyer creative ID and creative dimensions, or to a buyer creative ID as a whole. |
| `sensitive_categories` | Vec<i64> |  | Detected sensitive categories, if any. Each category is represented by an integer as defined in  ad-sensitive-categories.txt. Read-only. This field should not be set in requests. |
| `languages` | Vec<String> |  | Detected languages for this creative. Read-only. This field should not be set in requests. |
| `ad_choices_destination_url` | String |  | The link to the Ad Preferences page. This is only supported for native ads. |
| `video_vast_xml` | String |  | The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard. If set, HTMLSnippet, videoURL, and nativeAd and should not be set. |
| `corrections` | Vec<String> |  | Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests. |
| `height` | i64 |  | Ad height. |
| `advertiser_name` | String |  | The name of the company being advertised in the creative. A list of advertisers is provided in the advertisers.txt file. |
| `account_id` | i64 |  | Account id. |
| `open_auction_status` | String |  | Top-level open auction status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=OPEN_AUCTION (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from ServingRestrictions directly. |
| `attribute` | Vec<i64> |  | List of buyer selectable attributes for the ads that may be shown from this snippet. Each attribute is represented by an integer as defined in  buyer-declarable-creative-attributes.txt. |
| `kind` | String |  | Resource type. |
| `html_snippet` | String |  | The HTML snippet that displays the ad when inserted in the web page. If set, videoURL, videoVastXML, and nativeAd should not be set. |
| `buyer_creative_id` | String |  | A buyer-specific id identifying the creative in this ad. |
| `click_through_url` | Vec<String> |  | The set of destination urls for the snippet. |
| `impression_tracking_url` | Vec<String> |  | The set of urls to be called to record an impression. |
| `agency_id` | String |  | The agency id for this creative. |
| `restricted_categories` | Vec<i64> |  | All restricted categories for the ads that may be shown from this snippet. Each category is represented by an integer as defined in the  ad-restricted-categories.txt. |
| `filtering_reasons` | String |  | The filtering reasons for the creative. Read-only. This field should not be set in requests. |
| `native_ad` | String |  | If nativeAd is set, HTMLSnippet, videoVastXML, and the videoURL outside of nativeAd should not be set. (The videoURL inside nativeAd can be set.) |
| `advertiser_id` | Vec<String> |  | Detected advertiser id, if any. Read-only. This field should not be set in requests. |
| `product_categories` | Vec<i64> |  | Detected product categories, if any. Each category is represented by an integer as defined in  ad-product-categories.txt. Read-only. This field should not be set in requests. |
| `version` | i64 |  | The version for this creative. Read-only. This field should not be set in requests. |
| `video_url` | String |  | The URL to fetch a video ad. If set, HTMLSnippet, videoVastXML, and nativeAd should not be set. Note, this is different from resource.native_ad.video_url above. |
| `serving_restrictions` | Vec<String> |  | The granular status of this ad in specific contexts. A context here relates to where something ultimately serves (for example, a physical location, a platform, an HTTPS vs HTTP request, or the type of auction). Read-only. This field should not be set in requests. See the examples in the Creatives guide for more details. |
| `detected_domains` | Vec<String> |  | Detected domains for this creative. Read-only. This field should not be set in requests. |
| `api_upload_timestamp` | String |  | The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp). |
| `ad_technology_providers` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `width` | i64 | Ad width. |
| `deals_status` | String | Top-level deals status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=DIRECT_DEALS (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from servingRestrictions directly. |
| `vendor_type` | Vec<i64> | List of vendor types for the ads that may be shown from this snippet. Each vendor type is represented by an integer as defined in vendors.txt. |
| `creative_status_identity_type` | String | Creative status identity type that the creative item applies to. Ad Exchange real-time bidding is migrating to the sizeless creative verification. Originally, Ad Exchange assigned creative verification status to a unique combination of a buyer creative ID and creative dimensions. Post-migration, a single verification status will be assigned at the buyer creative ID level. This field allows to distinguish whether a given creative status applies to a unique combination of a buyer creative ID and creative dimensions, or to a buyer creative ID as a whole. |
| `sensitive_categories` | Vec<i64> | Detected sensitive categories, if any. Each category is represented by an integer as defined in  ad-sensitive-categories.txt. Read-only. This field should not be set in requests. |
| `languages` | Vec<String> | Detected languages for this creative. Read-only. This field should not be set in requests. |
| `ad_choices_destination_url` | String | The link to the Ad Preferences page. This is only supported for native ads. |
| `video_vast_xml` | String | The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard. If set, HTMLSnippet, videoURL, and nativeAd and should not be set. |
| `corrections` | Vec<String> | Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests. |
| `height` | i64 | Ad height. |
| `advertiser_name` | String | The name of the company being advertised in the creative. A list of advertisers is provided in the advertisers.txt file. |
| `account_id` | i64 | Account id. |
| `open_auction_status` | String | Top-level open auction status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=OPEN_AUCTION (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from ServingRestrictions directly. |
| `attribute` | Vec<i64> | List of buyer selectable attributes for the ads that may be shown from this snippet. Each attribute is represented by an integer as defined in  buyer-declarable-creative-attributes.txt. |
| `kind` | String | Resource type. |
| `html_snippet` | String | The HTML snippet that displays the ad when inserted in the web page. If set, videoURL, videoVastXML, and nativeAd should not be set. |
| `buyer_creative_id` | String | A buyer-specific id identifying the creative in this ad. |
| `click_through_url` | Vec<String> | The set of destination urls for the snippet. |
| `impression_tracking_url` | Vec<String> | The set of urls to be called to record an impression. |
| `agency_id` | String | The agency id for this creative. |
| `restricted_categories` | Vec<i64> | All restricted categories for the ads that may be shown from this snippet. Each category is represented by an integer as defined in the  ad-restricted-categories.txt. |
| `filtering_reasons` | String | The filtering reasons for the creative. Read-only. This field should not be set in requests. |
| `native_ad` | String | If nativeAd is set, HTMLSnippet, videoVastXML, and the videoURL outside of nativeAd should not be set. (The videoURL inside nativeAd can be set.) |
| `advertiser_id` | Vec<String> | Detected advertiser id, if any. Read-only. This field should not be set in requests. |
| `product_categories` | Vec<i64> | Detected product categories, if any. Each category is represented by an integer as defined in  ad-product-categories.txt. Read-only. This field should not be set in requests. |
| `version` | i64 | The version for this creative. Read-only. This field should not be set in requests. |
| `video_url` | String | The URL to fetch a video ad. If set, HTMLSnippet, videoVastXML, and nativeAd should not be set. Note, this is different from resource.native_ad.video_url above. |
| `serving_restrictions` | Vec<String> | The granular status of this ad in specific contexts. A context here relates to where something ultimately serves (for example, a physical location, a platform, an HTTPS vs HTTP request, or the type of auction). Read-only. This field should not be set in requests. See the examples in the Creatives guide for more details. |
| `detected_domains` | Vec<String> | Detected domains for this creative. Read-only. This field should not be set in requests. |
| `api_upload_timestamp` | String | The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp). |
| `ad_technology_providers` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create creative
creative = provider.adexchangebuyer_api.Creative {
}

# Access creative outputs
creative_id = creative.id
creative_width = creative.width
creative_deals_status = creative.deals_status
creative_vendor_type = creative.vendor_type
creative_creative_status_identity_type = creative.creative_status_identity_type
creative_sensitive_categories = creative.sensitive_categories
creative_languages = creative.languages
creative_ad_choices_destination_url = creative.ad_choices_destination_url
creative_video_vast_xml = creative.video_vast_xml
creative_corrections = creative.corrections
creative_height = creative.height
creative_advertiser_name = creative.advertiser_name
creative_account_id = creative.account_id
creative_open_auction_status = creative.open_auction_status
creative_attribute = creative.attribute
creative_kind = creative.kind
creative_html_snippet = creative.html_snippet
creative_buyer_creative_id = creative.buyer_creative_id
creative_click_through_url = creative.click_through_url
creative_impression_tracking_url = creative.impression_tracking_url
creative_agency_id = creative.agency_id
creative_restricted_categories = creative.restricted_categories
creative_filtering_reasons = creative.filtering_reasons
creative_native_ad = creative.native_ad
creative_advertiser_id = creative.advertiser_id
creative_product_categories = creative.product_categories
creative_version = creative.version
creative_video_url = creative.video_url
creative_serving_restrictions = creative.serving_restrictions
creative_detected_domains = creative.detected_domains
creative_api_upload_timestamp = creative.api_upload_timestamp
creative_ad_technology_providers = creative.ad_technology_providers
```

---


### Marketplacedeal

Add new deals for the specified proposal

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_action` | String |  | Indicates an optional action to take on the proposal |
| `proposal_revision_number` | String |  | The last known proposal revision number. |
| `deals` | Vec<String> |  | The list of deals to add |
| `proposal_id` | String | ✅ | proposalId for which deals need to be added. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deals` | Vec<String> | List of deals for the proposal |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create marketplacedeal
marketplacedeal = provider.adexchangebuyer_api.Marketplacedeal {
    proposal_id = "value"  # proposalId for which deals need to be added.
}

# Access marketplacedeal outputs
marketplacedeal_id = marketplacedeal.id
marketplacedeal_deals = marketplacedeal.deals
```

---


### Marketplaceprivateauction

Update a given private auction proposal

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `note` | String |  | Optional note to be added. |
| `update_action` | String |  | The proposed action on the private auction proposal. |
| `proposal_revision_number` | String |  | The current revision number of the proposal to be updated. |
| `external_deal_id` | String |  | The externalDealId of the deal to be updated. |
| `private_auction_id` | String | ✅ | The private auction id to be updated. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create marketplaceprivateauction
marketplaceprivateauction = provider.adexchangebuyer_api.Marketplaceprivateauction {
    private_auction_id = "value"  # The private auction id to be updated.
}

```

---


### Performance_report

Retrieves the authenticated user's list of performance metrics.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `performance_report` | Vec<String> | A list of performance reports relevant for the account. |
| `kind` | String | Resource type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access performance_report outputs
performance_report_id = performance_report.id
performance_report_performance_report = performance_report.performance_report
performance_report_kind = performance_report.kind
```

---


### Account

Gets one account by ID.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cookie_matching_nid` | String |  | The nid parameter value used in cookie match requests. Please contact your technical account manager if you need to change this. |
| `number_active_creatives` | i64 |  | The number of creatives that this account inserted or bid with in the last 30 days. |
| `bidder_location` | Vec<String> |  | Your bidder locations that have distinct URLs. |
| `maximum_active_creatives` | i64 |  | The maximum number of active creatives that an account can have, where a creative is active if it was inserted or bid with in the last 30 days. Please contact your technical account manager if you need to change this. |
| `kind` | String |  | Resource type. |
| `cookie_matching_url` | String |  | The base URL used in cookie match requests. |
| `id` | i64 |  | Account id. |
| `maximum_total_qps` | i64 |  | The sum of all bidderLocation.maximumQps values cannot exceed this. Please contact your technical account manager if you need to change this. |
| `apply_pretargeting_to_non_guaranteed_deals` | bool |  | When this is false, bid requests that include a deal ID for a private auction or preferred deal are always sent to your bidder. When true, all active pretargeting configs will be applied to private auctions and preferred deals. Programmatic Guaranteed deals (when enabled) are always sent to your bidder. |
| `id` | i64 | ✅ | The account id |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cookie_matching_nid` | String | The nid parameter value used in cookie match requests. Please contact your technical account manager if you need to change this. |
| `number_active_creatives` | i64 | The number of creatives that this account inserted or bid with in the last 30 days. |
| `bidder_location` | Vec<String> | Your bidder locations that have distinct URLs. |
| `maximum_active_creatives` | i64 | The maximum number of active creatives that an account can have, where a creative is active if it was inserted or bid with in the last 30 days. Please contact your technical account manager if you need to change this. |
| `kind` | String | Resource type. |
| `cookie_matching_url` | String | The base URL used in cookie match requests. |
| `id` | i64 | Account id. |
| `maximum_total_qps` | i64 | The sum of all bidderLocation.maximumQps values cannot exceed this. Please contact your technical account manager if you need to change this. |
| `apply_pretargeting_to_non_guaranteed_deals` | bool | When this is false, bid requests that include a deal ID for a private auction or preferred deal are always sent to your bidder. When true, all active pretargeting configs will be applied to private auctions and preferred deals. Programmatic Guaranteed deals (when enabled) are always sent to your bidder. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account outputs
account_id = account.id
account_cookie_matching_nid = account.cookie_matching_nid
account_number_active_creatives = account.number_active_creatives
account_bidder_location = account.bidder_location
account_maximum_active_creatives = account.maximum_active_creatives
account_kind = account.kind
account_cookie_matching_url = account.cookie_matching_url
account_id = account.id
account_maximum_total_qps = account.maximum_total_qps
account_apply_pretargeting_to_non_guaranteed_deals = account.apply_pretargeting_to_non_guaranteed_deals
```

---


### Performance_report

Retrieves the authenticated user's list of performance metrics.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `performance_report` | Vec<String> | A list of performance reports relevant for the account. |
| `kind` | String | Resource type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access performance_report outputs
performance_report_id = performance_report.id
performance_report_performance_report = performance_report.performance_report
performance_report_kind = performance_report.kind
```

---


### Budget

Returns the budget information for the adgroup specified by the accountId and billingId.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The unique id that describes this item. |
| `billing_id` | String |  | The billing id to determine which adgroup to provide budget information for. This is required for get and update requests. |
| `account_id` | String |  | The id of the account. This is required for get and update requests. |
| `currency_code` | String |  | The currency code for the buyer. This cannot be altered here. |
| `kind` | String |  | The kind of the resource, i.e. "adexchangebuyer#budget". |
| `budget_amount` | String |  | The daily budget amount in unit amount of the account currency to apply for the billingId provided. This is required for update requests. |
| `billing_id` | String | ✅ | The billing id associated with the budget being updated. |
| `account_id` | String | ✅ | The account id associated with the budget being updated. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The unique id that describes this item. |
| `billing_id` | String | The billing id to determine which adgroup to provide budget information for. This is required for get and update requests. |
| `account_id` | String | The id of the account. This is required for get and update requests. |
| `currency_code` | String | The currency code for the buyer. This cannot be altered here. |
| `kind` | String | The kind of the resource, i.e. "adexchangebuyer#budget". |
| `budget_amount` | String | The daily budget amount in unit amount of the account currency to apply for the billingId provided. This is required for update requests. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access budget outputs
budget_id = budget.id
budget_id = budget.id
budget_billing_id = budget.billing_id
budget_account_id = budget.account_id
budget_currency_code = budget.currency_code
budget_kind = budget.kind
budget_budget_amount = budget.budget_amount
```

---


### Direct_deal

Gets one direct deal by ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Resource type. |
| `end_time` | String | End time for when this deal stops being active. If not set then this deal is valid until manually disabled by the publisher. In seconds since the epoch. |
| `start_time` | String | Start time for when this deal becomes active. If not set then this deal is active immediately upon creation. In seconds since the epoch. |
| `allows_alcohol` | bool | Whether the publisher for this deal is eligible for alcohol ads. |
| `fixed_cpm` | String | The fixed price for this direct deal. In cpm micros of currency according to currency_code. If set, then this deal is eligible for the fixed price tier of buying (highest priority, pay exactly the configured fixed price). |
| `account_id` | i64 | The account id of the buyer this deal is for. |
| `buyer_account_id` | String | The account id that this deal was negotiated for. It is either the buyer or the client that this deal was negotiated on behalf of. |
| `seller_network` | String | The name of the publisher offering this direct deal. |
| `id` | String | Deal id. |
| `advertiser` | String | The name of the advertiser this deal is for. |
| `currency_code` | String | The currency code that applies to the fixed_cpm value. If not set then assumed to be USD. |
| `private_exchange_min_cpm` | String | The minimum price for this direct deal. In cpm micros of currency according to currency_code. If set, then this deal is eligible for the private exchange tier of buying (below fixed price priority, run as a second price auction). |
| `deal_tier` | String | The deal type such as programmatic reservation or fixed price and so on. |
| `publisher_blocks_overriden` | bool | If true, the publisher has opted to have their blocks ignored when a creative is bid with for this deal. |
| `name` | String | Deal name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access direct_deal outputs
direct_deal_id = direct_deal.id
direct_deal_kind = direct_deal.kind
direct_deal_end_time = direct_deal.end_time
direct_deal_start_time = direct_deal.start_time
direct_deal_allows_alcohol = direct_deal.allows_alcohol
direct_deal_fixed_cpm = direct_deal.fixed_cpm
direct_deal_account_id = direct_deal.account_id
direct_deal_buyer_account_id = direct_deal.buyer_account_id
direct_deal_seller_network = direct_deal.seller_network
direct_deal_id = direct_deal.id
direct_deal_advertiser = direct_deal.advertiser
direct_deal_currency_code = direct_deal.currency_code
direct_deal_private_exchange_min_cpm = direct_deal.private_exchange_min_cpm
direct_deal_deal_tier = direct_deal.deal_tier
direct_deal_publisher_blocks_overriden = direct_deal.publisher_blocks_overriden
direct_deal_name = direct_deal.name
```

---


### Pretargeting_config

Inserts a new pretargeting configuration.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `config_name` | String |  | The name of the config. Must be unique. Required for all requests. |
| `mobile_operating_system_versions` | Vec<String> |  | Requests containing any of these mobile operating system version ids will match. Values are from mobile-os.csv in the downloadable files section. |
| `excluded_content_labels` | Vec<String> |  | Requests with any of these content labels will not match. Values are from content-labels.txt in the downloadable files section. |
| `is_active` | bool |  | Whether this config is active. Required for all requests. |
| `kind` | String |  | The kind of the resource, i.e. "adexchangebuyer#pretargetingConfig". |
| `dimensions` | Vec<String> |  | Requests which allow one of these (width, height) pairs will match. All pairs must be supported ad dimensions. |
| `excluded_verticals` | Vec<String> |  | Requests containing any of these vertical ids will not match. Values are from the publisher-verticals.txt file in the downloadable files section. |
| `config_id` | String |  | The config id; generated automatically. Leave this field blank for insert requests. |
| `languages` | Vec<String> |  | Request containing any of these language codes will match. |
| `placements` | Vec<String> |  | Requests containing any of these placements will match. |
| `creative_type` | Vec<String> |  | List must contain exactly one of PRETARGETING_CREATIVE_TYPE_HTML or PRETARGETING_CREATIVE_TYPE_VIDEO. |
| `platforms` | Vec<String> |  | Requests matching any of these platforms will match. Possible values are PRETARGETING_PLATFORM_MOBILE, PRETARGETING_PLATFORM_DESKTOP, and PRETARGETING_PLATFORM_TABLET. |
| `supported_creative_attributes` | Vec<String> |  | Creative attributes should be declared here if all creatives corresponding to this pretargeting configuration have that creative attribute. Values are from pretargetable-creative-attributes.txt in the downloadable files section. |
| `excluded_user_lists` | Vec<String> |  | Requests containing any of these users list ids will not match. |
| `vendor_types` | Vec<String> |  | Requests that allow any of these vendor ids will match. Values are from vendors.txt in the downloadable files section. |
| `mobile_devices` | Vec<String> |  | Requests containing any of these mobile device ids will match. Values are from mobile-devices.csv in the downloadable files section. |
| `excluded_geo_criteria_ids` | Vec<String> |  | Requests containing any of these geo criteria ids will not match. |
| `user_lists` | Vec<String> |  | Requests containing any of these user list ids will match. |
| `excluded_placements` | Vec<String> |  | Requests containing any of these placements will not match. |
| `mobile_carriers` | Vec<String> |  | Requests containing any of these mobile carrier ids will match. Values are from mobile-carriers.csv in the downloadable files section. |
| `maximum_qps` | String |  | The maximum QPS allocated to this pretargeting configuration, used for pretargeting-level QPS limits. By default, this is not set, which indicates that there is no QPS limit at the configuration level (a global or account-level limit may still be imposed). |
| `verticals` | Vec<String> |  | Requests containing any of these vertical ids will match. |
| `geo_criteria_ids` | Vec<String> |  | Requests containing any of these geo criteria ids will match. |
| `billing_id` | String |  | The id for billing purposes, provided for reference. Leave this field blank for insert requests; the id will be generated automatically. |
| `account_id` | String | ✅ | The account id to insert the pretargeting config for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `config_name` | String | The name of the config. Must be unique. Required for all requests. |
| `mobile_operating_system_versions` | Vec<String> | Requests containing any of these mobile operating system version ids will match. Values are from mobile-os.csv in the downloadable files section. |
| `excluded_content_labels` | Vec<String> | Requests with any of these content labels will not match. Values are from content-labels.txt in the downloadable files section. |
| `is_active` | bool | Whether this config is active. Required for all requests. |
| `kind` | String | The kind of the resource, i.e. "adexchangebuyer#pretargetingConfig". |
| `dimensions` | Vec<String> | Requests which allow one of these (width, height) pairs will match. All pairs must be supported ad dimensions. |
| `excluded_verticals` | Vec<String> | Requests containing any of these vertical ids will not match. Values are from the publisher-verticals.txt file in the downloadable files section. |
| `config_id` | String | The config id; generated automatically. Leave this field blank for insert requests. |
| `languages` | Vec<String> | Request containing any of these language codes will match. |
| `placements` | Vec<String> | Requests containing any of these placements will match. |
| `creative_type` | Vec<String> | List must contain exactly one of PRETARGETING_CREATIVE_TYPE_HTML or PRETARGETING_CREATIVE_TYPE_VIDEO. |
| `platforms` | Vec<String> | Requests matching any of these platforms will match. Possible values are PRETARGETING_PLATFORM_MOBILE, PRETARGETING_PLATFORM_DESKTOP, and PRETARGETING_PLATFORM_TABLET. |
| `supported_creative_attributes` | Vec<String> | Creative attributes should be declared here if all creatives corresponding to this pretargeting configuration have that creative attribute. Values are from pretargetable-creative-attributes.txt in the downloadable files section. |
| `excluded_user_lists` | Vec<String> | Requests containing any of these users list ids will not match. |
| `vendor_types` | Vec<String> | Requests that allow any of these vendor ids will match. Values are from vendors.txt in the downloadable files section. |
| `mobile_devices` | Vec<String> | Requests containing any of these mobile device ids will match. Values are from mobile-devices.csv in the downloadable files section. |
| `excluded_geo_criteria_ids` | Vec<String> | Requests containing any of these geo criteria ids will not match. |
| `user_lists` | Vec<String> | Requests containing any of these user list ids will match. |
| `excluded_placements` | Vec<String> | Requests containing any of these placements will not match. |
| `mobile_carriers` | Vec<String> | Requests containing any of these mobile carrier ids will match. Values are from mobile-carriers.csv in the downloadable files section. |
| `maximum_qps` | String | The maximum QPS allocated to this pretargeting configuration, used for pretargeting-level QPS limits. By default, this is not set, which indicates that there is no QPS limit at the configuration level (a global or account-level limit may still be imposed). |
| `verticals` | Vec<String> | Requests containing any of these vertical ids will match. |
| `geo_criteria_ids` | Vec<String> | Requests containing any of these geo criteria ids will match. |
| `billing_id` | String | The id for billing purposes, provided for reference. Leave this field blank for insert requests; the id will be generated automatically. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create pretargeting_config
pretargeting_config = provider.adexchangebuyer_api.Pretargeting_config {
    account_id = "value"  # The account id to insert the pretargeting config for.
}

# Access pretargeting_config outputs
pretargeting_config_id = pretargeting_config.id
pretargeting_config_config_name = pretargeting_config.config_name
pretargeting_config_mobile_operating_system_versions = pretargeting_config.mobile_operating_system_versions
pretargeting_config_excluded_content_labels = pretargeting_config.excluded_content_labels
pretargeting_config_is_active = pretargeting_config.is_active
pretargeting_config_kind = pretargeting_config.kind
pretargeting_config_dimensions = pretargeting_config.dimensions
pretargeting_config_excluded_verticals = pretargeting_config.excluded_verticals
pretargeting_config_config_id = pretargeting_config.config_id
pretargeting_config_languages = pretargeting_config.languages
pretargeting_config_placements = pretargeting_config.placements
pretargeting_config_creative_type = pretargeting_config.creative_type
pretargeting_config_platforms = pretargeting_config.platforms
pretargeting_config_supported_creative_attributes = pretargeting_config.supported_creative_attributes
pretargeting_config_excluded_user_lists = pretargeting_config.excluded_user_lists
pretargeting_config_vendor_types = pretargeting_config.vendor_types
pretargeting_config_mobile_devices = pretargeting_config.mobile_devices
pretargeting_config_excluded_geo_criteria_ids = pretargeting_config.excluded_geo_criteria_ids
pretargeting_config_user_lists = pretargeting_config.user_lists
pretargeting_config_excluded_placements = pretargeting_config.excluded_placements
pretargeting_config_mobile_carriers = pretargeting_config.mobile_carriers
pretargeting_config_maximum_qps = pretargeting_config.maximum_qps
pretargeting_config_verticals = pretargeting_config.verticals
pretargeting_config_geo_criteria_ids = pretargeting_config.geo_criteria_ids
pretargeting_config_billing_id = pretargeting_config.billing_id
```

---


### Account

Gets one account by ID.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cookie_matching_nid` | String |  | The nid parameter value used in cookie match requests. Please contact your technical account manager if you need to change this. |
| `maximum_total_qps` | i64 |  | The sum of all bidderLocation.maximumQps values cannot exceed this. Please contact your technical account manager if you need to change this. |
| `id` | i64 |  | Account id. |
| `number_active_creatives` | i64 |  | The number of creatives that this account inserted or bid with in the last 30 days. |
| `bidder_location` | Vec<String> |  | Your bidder locations that have distinct URLs. |
| `cookie_matching_url` | String |  | The base URL used in cookie match requests. |
| `kind` | String |  | Resource type. |
| `maximum_active_creatives` | i64 |  | The maximum number of active creatives that an account can have, where a creative is active if it was inserted or bid with in the last 30 days. Please contact your technical account manager if you need to change this. |
| `id` | i64 | ✅ | The account id |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cookie_matching_nid` | String | The nid parameter value used in cookie match requests. Please contact your technical account manager if you need to change this. |
| `maximum_total_qps` | i64 | The sum of all bidderLocation.maximumQps values cannot exceed this. Please contact your technical account manager if you need to change this. |
| `id` | i64 | Account id. |
| `number_active_creatives` | i64 | The number of creatives that this account inserted or bid with in the last 30 days. |
| `bidder_location` | Vec<String> | Your bidder locations that have distinct URLs. |
| `cookie_matching_url` | String | The base URL used in cookie match requests. |
| `kind` | String | Resource type. |
| `maximum_active_creatives` | i64 | The maximum number of active creatives that an account can have, where a creative is active if it was inserted or bid with in the last 30 days. Please contact your technical account manager if you need to change this. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account outputs
account_id = account.id
account_cookie_matching_nid = account.cookie_matching_nid
account_maximum_total_qps = account.maximum_total_qps
account_id = account.id
account_number_active_creatives = account.number_active_creatives
account_bidder_location = account.bidder_location
account_cookie_matching_url = account.cookie_matching_url
account_kind = account.kind
account_maximum_active_creatives = account.maximum_active_creatives
```

---


### Billing_info

Returns the billing information for one account specified by account ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | i64 | Account id. |
| `kind` | String | Resource type. |
| `account_name` | String | Account name. |
| `billing_id` | Vec<String> | A list of adgroup IDs associated with this particular account. These IDs may show up as part of a realtime bidding BidRequest, which indicates a bid request for this account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access billing_info outputs
billing_info_id = billing_info.id
billing_info_account_id = billing_info.account_id
billing_info_kind = billing_info.kind
billing_info_account_name = billing_info.account_name
billing_info_billing_id = billing_info.billing_id
```

---


### Creative

Submit a new creative.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vendor_type` | Vec<i64> |  | All vendor types for the ads that may be shown from this snippet. |
| `product_categories` | Vec<i64> |  | Detected product categories, if any. Read-only. This field should not be set in requests. |
| `html_snippet` | String |  | The HTML snippet that displays the ad when inserted in the web page. If set, videoURL should not be set. |
| `api_upload_timestamp` | String |  | The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp). |
| `video_url` | String |  | The URL to fetch a video ad. If set, HTMLSnippet and the nativeAd should not be set. |
| `width` | i64 |  | Ad width. |
| `height` | i64 |  | Ad height. |
| `corrections` | Vec<String> |  | Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests. |
| `buyer_creative_id` | String |  | A buyer-specific id identifying the creative in this ad. |
| `disapproval_reasons` | Vec<String> |  | The reasons for disapproval, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue. Read-only. This field should not be set in requests. |
| `advertiser_name` | String |  | The name of the company being advertised in the creative. |
| `kind` | String |  | Resource type. |
| `native_ad` | String |  | If nativeAd is set, HTMLSnippet and videoURL should not be set. |
| `sensitive_categories` | Vec<i64> |  | Detected sensitive categories, if any. Read-only. This field should not be set in requests. |
| `advertiser_id` | Vec<String> |  | Detected advertiser id, if any. Read-only. This field should not be set in requests. |
| `account_id` | i64 |  | Account id. |
| `filtering_reasons` | String |  | The filtering reasons for the creative. Read-only. This field should not be set in requests. |
| `restricted_categories` | Vec<i64> |  | All restricted categories for the ads that may be shown from this snippet. |
| `ad_technology_providers` | String |  |  |
| `attribute` | Vec<i64> |  | All attributes for the ads that may be shown from this snippet. |
| `click_through_url` | Vec<String> |  | The set of destination urls for the snippet. |
| `impression_tracking_url` | Vec<String> |  | The set of urls to be called to record an impression. |
| `agency_id` | String |  | The agency id for this creative. |
| `status` | String |  | Creative serving status. Read-only. This field should not be set in requests. |
| `version` | i64 |  | The version for this creative. Read-only. This field should not be set in requests. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vendor_type` | Vec<i64> | All vendor types for the ads that may be shown from this snippet. |
| `product_categories` | Vec<i64> | Detected product categories, if any. Read-only. This field should not be set in requests. |
| `html_snippet` | String | The HTML snippet that displays the ad when inserted in the web page. If set, videoURL should not be set. |
| `api_upload_timestamp` | String | The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp). |
| `video_url` | String | The URL to fetch a video ad. If set, HTMLSnippet and the nativeAd should not be set. |
| `width` | i64 | Ad width. |
| `height` | i64 | Ad height. |
| `corrections` | Vec<String> | Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests. |
| `buyer_creative_id` | String | A buyer-specific id identifying the creative in this ad. |
| `disapproval_reasons` | Vec<String> | The reasons for disapproval, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue. Read-only. This field should not be set in requests. |
| `advertiser_name` | String | The name of the company being advertised in the creative. |
| `kind` | String | Resource type. |
| `native_ad` | String | If nativeAd is set, HTMLSnippet and videoURL should not be set. |
| `sensitive_categories` | Vec<i64> | Detected sensitive categories, if any. Read-only. This field should not be set in requests. |
| `advertiser_id` | Vec<String> | Detected advertiser id, if any. Read-only. This field should not be set in requests. |
| `account_id` | i64 | Account id. |
| `filtering_reasons` | String | The filtering reasons for the creative. Read-only. This field should not be set in requests. |
| `restricted_categories` | Vec<i64> | All restricted categories for the ads that may be shown from this snippet. |
| `ad_technology_providers` | String |  |
| `attribute` | Vec<i64> | All attributes for the ads that may be shown from this snippet. |
| `click_through_url` | Vec<String> | The set of destination urls for the snippet. |
| `impression_tracking_url` | Vec<String> | The set of urls to be called to record an impression. |
| `agency_id` | String | The agency id for this creative. |
| `status` | String | Creative serving status. Read-only. This field should not be set in requests. |
| `version` | i64 | The version for this creative. Read-only. This field should not be set in requests. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create creative
creative = provider.adexchangebuyer_api.Creative {
}

# Access creative outputs
creative_id = creative.id
creative_vendor_type = creative.vendor_type
creative_product_categories = creative.product_categories
creative_html_snippet = creative.html_snippet
creative_api_upload_timestamp = creative.api_upload_timestamp
creative_video_url = creative.video_url
creative_width = creative.width
creative_height = creative.height
creative_corrections = creative.corrections
creative_buyer_creative_id = creative.buyer_creative_id
creative_disapproval_reasons = creative.disapproval_reasons
creative_advertiser_name = creative.advertiser_name
creative_kind = creative.kind
creative_native_ad = creative.native_ad
creative_sensitive_categories = creative.sensitive_categories
creative_advertiser_id = creative.advertiser_id
creative_account_id = creative.account_id
creative_filtering_reasons = creative.filtering_reasons
creative_restricted_categories = creative.restricted_categories
creative_ad_technology_providers = creative.ad_technology_providers
creative_attribute = creative.attribute
creative_click_through_url = creative.click_through_url
creative_impression_tracking_url = creative.impression_tracking_url
creative_agency_id = creative.agency_id
creative_status = creative.status
creative_version = creative.version
```

---


### Creative

Submit a new creative.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `vendor_type` | Vec<i64> |  | All vendor types for the ads that may be shown from this snippet. |
| `video_url` | String |  | The url to fetch a video ad. If set, HTMLSnippet should not be set. |
| `version` | i64 |  | The version for this creative. Read-only. This field should not be set in requests. |
| `width` | i64 |  | Ad width. |
| `advertiser_id` | Vec<String> |  | Detected advertiser id, if any. Read-only. This field should not be set in requests. |
| `filtering_reasons` | String |  | The filtering reasons for the creative. Read-only. This field should not be set in requests. |
| `disapproval_reasons` | Vec<String> |  | The reasons for disapproval, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue. Read-only. This field should not be set in requests. |
| `html_snippet` | String |  | The HTML snippet that displays the ad when inserted in the web page. If set, videoURL should not be set. |
| `corrections` | Vec<String> |  | Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests. |
| `height` | i64 |  | Ad height. |
| `api_upload_timestamp` | String |  | The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp). |
| `account_id` | i64 |  | Account id. |
| `agency_id` | String |  | The agency id for this creative. |
| `attribute` | Vec<i64> |  | All attributes for the ads that may be shown from this snippet. |
| `buyer_creative_id` | String |  | A buyer-specific id identifying the creative in this ad. |
| `impression_tracking_url` | Vec<String> |  | The set of urls to be called to record an impression. |
| `kind` | String |  | Resource type. |
| `restricted_categories` | Vec<i64> |  | All restricted categories for the ads that may be shown from this snippet. |
| `status` | String |  | Creative serving status. Read-only. This field should not be set in requests. |
| `advertiser_name` | String |  | The name of the company being advertised in the creative. |
| `click_through_url` | Vec<String> |  | The set of destination urls for the snippet. |
| `product_categories` | Vec<i64> |  | Detected product categories, if any. Read-only. This field should not be set in requests. |
| `sensitive_categories` | Vec<i64> |  | Detected sensitive categories, if any. Read-only. This field should not be set in requests. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vendor_type` | Vec<i64> | All vendor types for the ads that may be shown from this snippet. |
| `video_url` | String | The url to fetch a video ad. If set, HTMLSnippet should not be set. |
| `version` | i64 | The version for this creative. Read-only. This field should not be set in requests. |
| `width` | i64 | Ad width. |
| `advertiser_id` | Vec<String> | Detected advertiser id, if any. Read-only. This field should not be set in requests. |
| `filtering_reasons` | String | The filtering reasons for the creative. Read-only. This field should not be set in requests. |
| `disapproval_reasons` | Vec<String> | The reasons for disapproval, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue. Read-only. This field should not be set in requests. |
| `html_snippet` | String | The HTML snippet that displays the ad when inserted in the web page. If set, videoURL should not be set. |
| `corrections` | Vec<String> | Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests. |
| `height` | i64 | Ad height. |
| `api_upload_timestamp` | String | The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp). |
| `account_id` | i64 | Account id. |
| `agency_id` | String | The agency id for this creative. |
| `attribute` | Vec<i64> | All attributes for the ads that may be shown from this snippet. |
| `buyer_creative_id` | String | A buyer-specific id identifying the creative in this ad. |
| `impression_tracking_url` | Vec<String> | The set of urls to be called to record an impression. |
| `kind` | String | Resource type. |
| `restricted_categories` | Vec<i64> | All restricted categories for the ads that may be shown from this snippet. |
| `status` | String | Creative serving status. Read-only. This field should not be set in requests. |
| `advertiser_name` | String | The name of the company being advertised in the creative. |
| `click_through_url` | Vec<String> | The set of destination urls for the snippet. |
| `product_categories` | Vec<i64> | Detected product categories, if any. Read-only. This field should not be set in requests. |
| `sensitive_categories` | Vec<i64> | Detected sensitive categories, if any. Read-only. This field should not be set in requests. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create creative
creative = provider.adexchangebuyer_api.Creative {
}

# Access creative outputs
creative_id = creative.id
creative_vendor_type = creative.vendor_type
creative_video_url = creative.video_url
creative_version = creative.version
creative_width = creative.width
creative_advertiser_id = creative.advertiser_id
creative_filtering_reasons = creative.filtering_reasons
creative_disapproval_reasons = creative.disapproval_reasons
creative_html_snippet = creative.html_snippet
creative_corrections = creative.corrections
creative_height = creative.height
creative_api_upload_timestamp = creative.api_upload_timestamp
creative_account_id = creative.account_id
creative_agency_id = creative.agency_id
creative_attribute = creative.attribute
creative_buyer_creative_id = creative.buyer_creative_id
creative_impression_tracking_url = creative.impression_tracking_url
creative_kind = creative.kind
creative_restricted_categories = creative.restricted_categories
creative_status = creative.status
creative_advertiser_name = creative.advertiser_name
creative_click_through_url = creative.click_through_url
creative_product_categories = creative.product_categories
creative_sensitive_categories = creative.sensitive_categories
```

---


### Account

Gets one account by ID.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | i64 |  | Account id. |
| `kind` | String |  | Resource type. |
| `cookie_matching_url` | String |  | The base URL used in cookie match requests. |
| `maximum_total_qps` | i64 |  | The sum of all bidderLocation.maximumQps values cannot exceed this. Please contact your technical account manager if you need to change this. |
| `bidder_location` | Vec<String> |  | Your bidder locations that have distinct URLs. |
| `number_active_creatives` | i64 |  | The number of creatives that this account inserted or bid with in the last 30 days. |
| `cookie_matching_nid` | String |  | The nid parameter value used in cookie match requests. Please contact your technical account manager if you need to change this. |
| `maximum_active_creatives` | i64 |  | The maximum number of active creatives that an account can have, where a creative is active if it was inserted or bid with in the last 30 days. Please contact your technical account manager if you need to change this. |
| `id` | i64 | ✅ | The account id |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | i64 | Account id. |
| `kind` | String | Resource type. |
| `cookie_matching_url` | String | The base URL used in cookie match requests. |
| `maximum_total_qps` | i64 | The sum of all bidderLocation.maximumQps values cannot exceed this. Please contact your technical account manager if you need to change this. |
| `bidder_location` | Vec<String> | Your bidder locations that have distinct URLs. |
| `number_active_creatives` | i64 | The number of creatives that this account inserted or bid with in the last 30 days. |
| `cookie_matching_nid` | String | The nid parameter value used in cookie match requests. Please contact your technical account manager if you need to change this. |
| `maximum_active_creatives` | i64 | The maximum number of active creatives that an account can have, where a creative is active if it was inserted or bid with in the last 30 days. Please contact your technical account manager if you need to change this. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account outputs
account_id = account.id
account_id = account.id
account_kind = account.kind
account_cookie_matching_url = account.cookie_matching_url
account_maximum_total_qps = account.maximum_total_qps
account_bidder_location = account.bidder_location
account_number_active_creatives = account.number_active_creatives
account_cookie_matching_nid = account.cookie_matching_nid
account_maximum_active_creatives = account.maximum_active_creatives
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple proposal resources
proposal_0 = provider.adexchangebuyer_api.Proposal {
}
proposal_1 = provider.adexchangebuyer_api.Proposal {
}
proposal_2 = provider.adexchangebuyer_api.Proposal {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    proposal = provider.adexchangebuyer_api.Proposal {
    }
```

---

## Related Documentation

- [GCP Adexchangebuyer_api Documentation](https://cloud.google.com/adexchangebuyer_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
