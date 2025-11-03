# Adexchangebuyer2_api Service



**Resources**: 19

---

## Overview

The adexchangebuyer2_api service provides access to 19 resource types:

- [User](#user) [RU]
- [Filtered_bid](#filtered_bid) [R]
- [Filtered_bid_request](#filtered_bid_request) [R]
- [Proposal](#proposal) [CRU]
- [Creative](#creative) [CRU]
- [Invitation](#invitation) [CR]
- [Bid_metric](#bid_metric) [R]
- [Client](#client) [CRU]
- [Finalized_proposal](#finalized_proposal) [CR]
- [Product](#product) [R]
- [Non_billable_winning_bid](#non_billable_winning_bid) [R]
- [Bid_responses_without_bid](#bid_responses_without_bid) [R]
- [Deal_association](#deal_association) [CR]
- [Filter_set](#filter_set) [CRD]
- [Bid_response_error](#bid_response_error) [R]
- [Detail](#detail) [R]
- [Losing_bid](#losing_bid) [R]
- [Impression_metric](#impression_metric) [R]
- [Publisher_profile](#publisher_profile) [R]

---

## Resources


### User

Retrieves an existing client user.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String |  | The status of the client user. |
| `email` | String |  | User's email address. The value of this field is ignored in an update operation. |
| `client_account_id` | String |  | Numerical account ID of the client buyer with which the user is associated; the buyer must be a client of the current sponsor buyer. The value of this field is ignored in an update operation. |
| `user_id` | String |  | The unique numerical ID of the client user that has accepted an invitation. The value of this field is ignored in an update operation. |
| `client_account_id` | String | ✅ | Numerical account ID of the client buyer that the user to be retrieved is associated with. (required) |
| `account_id` | String | ✅ | Numerical account ID of the client's sponsor buyer. (required) |
| `user_id` | String | ✅ | Numerical identifier of the user to retrieve. (required) |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | The status of the client user. |
| `email` | String | User's email address. The value of this field is ignored in an update operation. |
| `client_account_id` | String | Numerical account ID of the client buyer with which the user is associated; the buyer must be a client of the current sponsor buyer. The value of this field is ignored in an update operation. |
| `user_id` | String | The unique numerical ID of the client user that has accepted an invitation. The value of this field is ignored in an update operation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user outputs
user_id = user.id
user_status = user.status
user_email = user.email
user_client_account_id = user.client_account_id
user_user_id = user.user_id
```

---


### Filtered_bid

List all reasons for which bids were filtered, with the number of bids filtered for each reason.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creative_status_rows` | Vec<String> | List of rows, with counts of filtered bids aggregated by filtering reason (for example, creative status). |
| `next_page_token` | String | A token to retrieve the next page of results. Pass this value in the ListFilteredBidsRequest.pageToken field in the subsequent call to the filteredBids.list method to retrieve the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access filtered_bid outputs
filtered_bid_id = filtered_bid.id
filtered_bid_creative_status_rows = filtered_bid.creative_status_rows
filtered_bid_next_page_token = filtered_bid.next_page_token
```

---


### Filtered_bid_request

List all reasons that caused a bid request not to be sent for an impression, with the number of bid requests not sent for each reason.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token to retrieve the next page of results. Pass this value in the ListFilteredBidRequestsRequest.pageToken field in the subsequent call to the filteredBidRequests.list method to retrieve the next page of results. |
| `callout_status_rows` | Vec<String> | List of rows, with counts of filtered bid requests aggregated by callout status. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access filtered_bid_request outputs
filtered_bid_request_id = filtered_bid_request.id
filtered_bid_request_next_page_token = filtered_bid_request.next_page_token
filtered_bid_request_callout_status_rows = filtered_bid_request.callout_status_rows
```

---


### Proposal

Create the given proposal. Each created proposal and any deals it contains are assigned a unique ID by the server.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The name for the proposal. |
| `private_auction_id` | String |  | Output only. Private auction ID if this proposal is a private auction proposal. |
| `proposal_id` | String |  | Output only. The unique ID of the proposal. |
| `proposal_state` | String |  | Output only. The current state of the proposal. |
| `terms_and_conditions` | String |  | Output only. The terms and conditions set by the publisher for this proposal. |
| `update_time` | String |  | Output only. The time when the proposal was last revised. |
| `originator_role` | String |  | Output only. Indicates whether the buyer/seller created the proposal. |
| `buyer_contacts` | Vec<String> |  | Contact information for the buyer. |
| `proposal_revision` | String |  | Output only. The revision number for the proposal. Each update to the proposal or the deal causes the proposal revision number to auto-increment. The buyer keeps track of the last revision number they know of and pass it in when making an update. If the head revision number on the server has since incremented, then an ABORTED error is returned during the update operation to let the buyer know that a subsequent update was made. |
| `is_setup_complete` | bool |  | Output only. True, if the buyside inventory setup is complete for this proposal. |
| `notes` | Vec<String> |  | Output only. The notes associated with this proposal. |
| `buyer` | String |  | Reference to the buyer on the proposal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error. |
| `last_updater_or_commentor_role` | String |  | Output only. The role of the last user that either updated the proposal or left a comment. |
| `deals` | Vec<String> |  | The deals associated with this proposal. For Private Auction proposals (whose deals have NonGuaranteedAuctionTerms), there will only be one deal. |
| `seller_contacts` | Vec<String> |  | Output only. Contact information for the seller. |
| `billed_buyer` | String |  | Output only. Reference to the buyer that will get billed for this proposal. |
| `is_renegotiating` | bool |  | Output only. True if the proposal is being renegotiated. |
| `seller` | String |  | Reference to the seller on the proposal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error. |
| `buyer_private_data` | String |  | Private data for buyer. (hidden from seller). |
| `account_id` | String | ✅ | Account ID of the buyer. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The name for the proposal. |
| `private_auction_id` | String | Output only. Private auction ID if this proposal is a private auction proposal. |
| `proposal_id` | String | Output only. The unique ID of the proposal. |
| `proposal_state` | String | Output only. The current state of the proposal. |
| `terms_and_conditions` | String | Output only. The terms and conditions set by the publisher for this proposal. |
| `update_time` | String | Output only. The time when the proposal was last revised. |
| `originator_role` | String | Output only. Indicates whether the buyer/seller created the proposal. |
| `buyer_contacts` | Vec<String> | Contact information for the buyer. |
| `proposal_revision` | String | Output only. The revision number for the proposal. Each update to the proposal or the deal causes the proposal revision number to auto-increment. The buyer keeps track of the last revision number they know of and pass it in when making an update. If the head revision number on the server has since incremented, then an ABORTED error is returned during the update operation to let the buyer know that a subsequent update was made. |
| `is_setup_complete` | bool | Output only. True, if the buyside inventory setup is complete for this proposal. |
| `notes` | Vec<String> | Output only. The notes associated with this proposal. |
| `buyer` | String | Reference to the buyer on the proposal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error. |
| `last_updater_or_commentor_role` | String | Output only. The role of the last user that either updated the proposal or left a comment. |
| `deals` | Vec<String> | The deals associated with this proposal. For Private Auction proposals (whose deals have NonGuaranteedAuctionTerms), there will only be one deal. |
| `seller_contacts` | Vec<String> | Output only. Contact information for the seller. |
| `billed_buyer` | String | Output only. Reference to the buyer that will get billed for this proposal. |
| `is_renegotiating` | bool | Output only. True if the proposal is being renegotiated. |
| `seller` | String | Reference to the seller on the proposal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error. |
| `buyer_private_data` | String | Private data for buyer. (hidden from seller). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create proposal
proposal = provider.adexchangebuyer2_api.Proposal {
    account_id = "value"  # Account ID of the buyer.
}

# Access proposal outputs
proposal_id = proposal.id
proposal_display_name = proposal.display_name
proposal_private_auction_id = proposal.private_auction_id
proposal_proposal_id = proposal.proposal_id
proposal_proposal_state = proposal.proposal_state
proposal_terms_and_conditions = proposal.terms_and_conditions
proposal_update_time = proposal.update_time
proposal_originator_role = proposal.originator_role
proposal_buyer_contacts = proposal.buyer_contacts
proposal_proposal_revision = proposal.proposal_revision
proposal_is_setup_complete = proposal.is_setup_complete
proposal_notes = proposal.notes
proposal_buyer = proposal.buyer
proposal_last_updater_or_commentor_role = proposal.last_updater_or_commentor_role
proposal_deals = proposal.deals
proposal_seller_contacts = proposal.seller_contacts
proposal_billed_buyer = proposal.billed_buyer
proposal_is_renegotiating = proposal.is_renegotiating
proposal_seller = proposal.seller
proposal_buyer_private_data = proposal.buyer_private_data
```

---


### Creative

Creates a creative.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ad_technology_providers` | String |  | Output only. The detected ad technology providers. |
| `corrections` | Vec<String> |  | Output only. Shows any corrections that were applied to this creative. |
| `creative_id` | String |  | The buyer-defined creative ID of this creative. Can be used to filter the response of the creatives.list method. |
| `click_through_urls` | Vec<String> |  | The set of destination URLs for the creative. |
| `native` | String |  | A native creative. |
| `deals_status` | String |  | Output only. The top-level deals status of this creative. If disapproved, an entry for 'auctionType=DIRECT_DEALS' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method. |
| `account_id` | String |  | The account that this creative belongs to. Can be used to filter the response of the creatives.list method. |
| `detected_sensitive_categories` | Vec<i64> |  | Output only. Detected sensitive categories, if any. See the ad-sensitive-categories.txt file in the technical documentation for a list of IDs. You should use these IDs along with the excluded-sensitive-category field in the bid request to filter your bids. |
| `restricted_categories` | Vec<String> |  | All restricted categories for the ads that may be shown from this creative. |
| `ad_choices_destination_url` | String |  | The link to AdChoices destination page. |
| `detected_advertiser_ids` | Vec<String> |  | Output only. Detected advertiser IDs, if any. |
| `declared_click_through_urls` | Vec<String> |  | The set of declared destination URLs for the creative. |
| `impression_tracking_urls` | Vec<String> |  | The set of URLs to be called to record an impression. |
| `agency_id` | String |  | The agency ID for this creative. |
| `api_update_time` | String |  | Output only. The last update timestamp of the creative through the API. |
| `attributes` | Vec<String> |  | All attributes for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method. |
| `detected_languages` | Vec<String> |  | Output only. The detected languages for this creative. The order is arbitrary. The codes are 2 or 5 characters and are documented at https://developers.google.com/adwords/api/docs/appendix/languagecodes. |
| `vendor_ids` | Vec<i64> |  | All vendor IDs for the ads that may be shown from this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values. |
| `video` | String |  | A video creative. |
| `open_auction_status` | String |  | Output only. The top-level open auction status of this creative. If disapproved, an entry for 'auctionType = OPEN_AUCTION' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method. |
| `serving_restrictions` | Vec<String> |  | Output only. The granular status of this ad in specific contexts. A context here relates to where something ultimately serves (for example, a physical location, a platform, an HTTPS versus HTTP request, or the type of auction). |
| `html` | String |  | An HTML creative. |
| `detected_product_categories` | Vec<i64> |  | Output only. Detected product categories, if any. See the ad-product-categories.txt file in the technical documentation for a list of IDs. |
| `detected_domains` | Vec<String> |  | Output only. The detected domains for this creative. |
| `version` | i64 |  | Output only. The version of this creative. |
| `advertiser_name` | String |  | The name of the company being advertised in the creative. |
| `account_id` | String | ✅ | The account that this creative belongs to. Can be used to filter the response of the creatives.list method. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ad_technology_providers` | String | Output only. The detected ad technology providers. |
| `corrections` | Vec<String> | Output only. Shows any corrections that were applied to this creative. |
| `creative_id` | String | The buyer-defined creative ID of this creative. Can be used to filter the response of the creatives.list method. |
| `click_through_urls` | Vec<String> | The set of destination URLs for the creative. |
| `native` | String | A native creative. |
| `deals_status` | String | Output only. The top-level deals status of this creative. If disapproved, an entry for 'auctionType=DIRECT_DEALS' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method. |
| `account_id` | String | The account that this creative belongs to. Can be used to filter the response of the creatives.list method. |
| `detected_sensitive_categories` | Vec<i64> | Output only. Detected sensitive categories, if any. See the ad-sensitive-categories.txt file in the technical documentation for a list of IDs. You should use these IDs along with the excluded-sensitive-category field in the bid request to filter your bids. |
| `restricted_categories` | Vec<String> | All restricted categories for the ads that may be shown from this creative. |
| `ad_choices_destination_url` | String | The link to AdChoices destination page. |
| `detected_advertiser_ids` | Vec<String> | Output only. Detected advertiser IDs, if any. |
| `declared_click_through_urls` | Vec<String> | The set of declared destination URLs for the creative. |
| `impression_tracking_urls` | Vec<String> | The set of URLs to be called to record an impression. |
| `agency_id` | String | The agency ID for this creative. |
| `api_update_time` | String | Output only. The last update timestamp of the creative through the API. |
| `attributes` | Vec<String> | All attributes for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method. |
| `detected_languages` | Vec<String> | Output only. The detected languages for this creative. The order is arbitrary. The codes are 2 or 5 characters and are documented at https://developers.google.com/adwords/api/docs/appendix/languagecodes. |
| `vendor_ids` | Vec<i64> | All vendor IDs for the ads that may be shown from this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values. |
| `video` | String | A video creative. |
| `open_auction_status` | String | Output only. The top-level open auction status of this creative. If disapproved, an entry for 'auctionType = OPEN_AUCTION' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method. |
| `serving_restrictions` | Vec<String> | Output only. The granular status of this ad in specific contexts. A context here relates to where something ultimately serves (for example, a physical location, a platform, an HTTPS versus HTTP request, or the type of auction). |
| `html` | String | An HTML creative. |
| `detected_product_categories` | Vec<i64> | Output only. Detected product categories, if any. See the ad-product-categories.txt file in the technical documentation for a list of IDs. |
| `detected_domains` | Vec<String> | Output only. The detected domains for this creative. |
| `version` | i64 | Output only. The version of this creative. |
| `advertiser_name` | String | The name of the company being advertised in the creative. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create creative
creative = provider.adexchangebuyer2_api.Creative {
    account_id = "value"  # The account that this creative belongs to. Can be used to filter the response of the creatives.list method.
}

# Access creative outputs
creative_id = creative.id
creative_ad_technology_providers = creative.ad_technology_providers
creative_corrections = creative.corrections
creative_creative_id = creative.creative_id
creative_click_through_urls = creative.click_through_urls
creative_native = creative.native
creative_deals_status = creative.deals_status
creative_account_id = creative.account_id
creative_detected_sensitive_categories = creative.detected_sensitive_categories
creative_restricted_categories = creative.restricted_categories
creative_ad_choices_destination_url = creative.ad_choices_destination_url
creative_detected_advertiser_ids = creative.detected_advertiser_ids
creative_declared_click_through_urls = creative.declared_click_through_urls
creative_impression_tracking_urls = creative.impression_tracking_urls
creative_agency_id = creative.agency_id
creative_api_update_time = creative.api_update_time
creative_attributes = creative.attributes
creative_detected_languages = creative.detected_languages
creative_vendor_ids = creative.vendor_ids
creative_video = creative.video
creative_open_auction_status = creative.open_auction_status
creative_serving_restrictions = creative.serving_restrictions
creative_html = creative.html
creative_detected_product_categories = creative.detected_product_categories
creative_detected_domains = creative.detected_domains
creative_version = creative.version
creative_advertiser_name = creative.advertiser_name
```

---


### Invitation

Creates and sends out an email invitation to access an Ad Exchange client buyer account.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `invitation_id` | String |  | The unique numerical ID of the invitation that is sent to the user. The value of this field is ignored in create operations. |
| `client_account_id` | String |  | Numerical account ID of the client buyer that the invited user is associated with. The value of this field is ignored in create operations. |
| `email` | String |  | The email address to which the invitation is sent. Email addresses should be unique among all client users under each sponsor buyer. |
| `account_id` | String | ✅ | Numerical account ID of the client's sponsor buyer. (required) |
| `client_account_id` | String | ✅ | Numerical account ID of the client buyer that the user should be associated with. (required) |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invitation_id` | String | The unique numerical ID of the invitation that is sent to the user. The value of this field is ignored in create operations. |
| `client_account_id` | String | Numerical account ID of the client buyer that the invited user is associated with. The value of this field is ignored in create operations. |
| `email` | String | The email address to which the invitation is sent. Email addresses should be unique among all client users under each sponsor buyer. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create invitation
invitation = provider.adexchangebuyer2_api.Invitation {
    account_id = "value"  # Numerical account ID of the client's sponsor buyer. (required)
    client_account_id = "value"  # Numerical account ID of the client buyer that the user should be associated with. (required)
}

# Access invitation outputs
invitation_id = invitation.id
invitation_invitation_id = invitation.invitation_id
invitation_client_account_id = invitation.client_account_id
invitation_email = invitation.email
```

---


### Bid_metric

Lists all metrics that are measured in terms of number of bids.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bid_metrics_rows` | Vec<String> | List of rows, each containing a set of bid metrics. |
| `next_page_token` | String | A token to retrieve the next page of results. Pass this value in the ListBidMetricsRequest.pageToken field in the subsequent call to the bidMetrics.list method to retrieve the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access bid_metric outputs
bid_metric_id = bid_metric.id
bid_metric_bid_metrics_rows = bid_metric.bid_metrics_rows
bid_metric_next_page_token = bid_metric.next_page_token
```

---


### Client

Creates a new client buyer.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `visible_to_seller` | bool |  | Whether the client buyer will be visible to sellers. |
| `status` | String |  | The status of the client buyer. |
| `entity_name` | String |  | The name of the entity. This field is automatically fetched based on the type and ID. The value of this field is ignored in create and update operations. |
| `entity_id` | String |  | Numerical identifier of the client entity. The entity can be an advertiser, a brand, or an agency. This identifier is unique among all the entities with the same type. The value of this field is ignored if the entity type is not provided. A list of all known advertisers with their identifiers is available in the [advertisers.txt](https://storage.googleapis.com/adx-rtb-dictionaries/advertisers.txt) file. A list of all known brands with their identifiers is available in the [brands.txt](https://storage.googleapis.com/adx-rtb-dictionaries/brands.txt) file. A list of all known agencies with their identifiers is available in the [agencies.txt](https://storage.googleapis.com/adx-rtb-dictionaries/agencies.txt) file. |
| `entity_type` | String |  | An optional field for specifying the type of the client entity: `ADVERTISER`, `BRAND`, or `AGENCY`. |
| `partner_client_id` | String |  | Optional arbitrary unique identifier of this client buyer from the standpoint of its Ad Exchange sponsor buyer. This field can be used to associate a client buyer with the identifier in the namespace of its sponsor buyer, lookup client buyers by that identifier and verify whether an Ad Exchange counterpart of a given client buyer already exists. If present, must be unique among all the client buyers for its Ad Exchange sponsor buyer. |
| `client_name` | String |  | Name used to represent this client to publishers. You may have multiple clients that map to the same entity, but for each client the combination of `clientName` and entity must be unique. You can specify this field as empty. Maximum length of 255 characters is allowed. |
| `client_account_id` | String |  | The globally-unique numerical ID of the client. The value of this field is ignored in create and update operations. |
| `role` | String |  | The role which is assigned to the client buyer. Each role implies a set of permissions granted to the client. Must be one of `CLIENT_DEAL_VIEWER`, `CLIENT_DEAL_NEGOTIATOR` or `CLIENT_DEAL_APPROVER`. |
| `account_id` | String | ✅ | Unique numerical account ID for the buyer of which the client buyer is a customer; the sponsor buyer to create a client for. (required) |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `visible_to_seller` | bool | Whether the client buyer will be visible to sellers. |
| `status` | String | The status of the client buyer. |
| `entity_name` | String | The name of the entity. This field is automatically fetched based on the type and ID. The value of this field is ignored in create and update operations. |
| `entity_id` | String | Numerical identifier of the client entity. The entity can be an advertiser, a brand, or an agency. This identifier is unique among all the entities with the same type. The value of this field is ignored if the entity type is not provided. A list of all known advertisers with their identifiers is available in the [advertisers.txt](https://storage.googleapis.com/adx-rtb-dictionaries/advertisers.txt) file. A list of all known brands with their identifiers is available in the [brands.txt](https://storage.googleapis.com/adx-rtb-dictionaries/brands.txt) file. A list of all known agencies with their identifiers is available in the [agencies.txt](https://storage.googleapis.com/adx-rtb-dictionaries/agencies.txt) file. |
| `entity_type` | String | An optional field for specifying the type of the client entity: `ADVERTISER`, `BRAND`, or `AGENCY`. |
| `partner_client_id` | String | Optional arbitrary unique identifier of this client buyer from the standpoint of its Ad Exchange sponsor buyer. This field can be used to associate a client buyer with the identifier in the namespace of its sponsor buyer, lookup client buyers by that identifier and verify whether an Ad Exchange counterpart of a given client buyer already exists. If present, must be unique among all the client buyers for its Ad Exchange sponsor buyer. |
| `client_name` | String | Name used to represent this client to publishers. You may have multiple clients that map to the same entity, but for each client the combination of `clientName` and entity must be unique. You can specify this field as empty. Maximum length of 255 characters is allowed. |
| `client_account_id` | String | The globally-unique numerical ID of the client. The value of this field is ignored in create and update operations. |
| `role` | String | The role which is assigned to the client buyer. Each role implies a set of permissions granted to the client. Must be one of `CLIENT_DEAL_VIEWER`, `CLIENT_DEAL_NEGOTIATOR` or `CLIENT_DEAL_APPROVER`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create client
client = provider.adexchangebuyer2_api.Client {
    account_id = "value"  # Unique numerical account ID for the buyer of which the client buyer is a customer; the sponsor buyer to create a client for. (required)
}

# Access client outputs
client_id = client.id
client_visible_to_seller = client.visible_to_seller
client_status = client.status
client_entity_name = client.entity_name
client_entity_id = client.entity_id
client_entity_type = client.entity_type
client_partner_client_id = client.partner_client_id
client_client_name = client.client_name
client_client_account_id = client.client_account_id
client_role = client.role
```

---


### Finalized_proposal

Update given deals to resume serving. This method will set the `DealServingMetadata.DealPauseStatus.has_buyer_paused` bit to false for all listed deals in the request. Currently, this method only applies to PG and PD deals. For PA deals, call accounts.proposals.resume endpoint. It is a no-op to resume running deals or deals paused by the other party. It is an error to call ResumeProposalDeals for deals which are not part of the proposal of proposal_id or which are not finalized or renegotiating.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `external_deal_ids` | Vec<String> |  | The external_deal_id's of the deals to resume. If empty, all the deals in the proposal will be resumed. |
| `account_id` | String | ✅ | Account ID of the buyer. |
| `proposal_id` | String | ✅ | The proposal_id of the proposal containing the deals. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `proposals` | Vec<String> | The list of proposals. |
| `next_page_token` | String | Continuation token for fetching the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create finalized_proposal
finalized_proposal = provider.adexchangebuyer2_api.Finalized_proposal {
    account_id = "value"  # Account ID of the buyer.
    proposal_id = "value"  # The proposal_id of the proposal containing the deals.
}

# Access finalized_proposal outputs
finalized_proposal_id = finalized_proposal.id
finalized_proposal_proposals = finalized_proposal.proposals
finalized_proposal_next_page_token = finalized_proposal.next_page_token
```

---


### Product

Gets the requested product by ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creator_contacts` | Vec<String> | Optional contact information for the creator of this product. |
| `terms` | String | The negotiable terms of the deal. |
| `product_id` | String | The unique ID for the product. |
| `create_time` | String | Creation time. |
| `publisher_profile_id` | String | An ID which can be used by the Publisher Profile API to get more information about the seller that created this product. |
| `display_name` | String | The display name for this product as set by the seller. |
| `has_creator_signed_off` | bool | If the creator has already signed off on the product, then the buyer can finalize the deal by accepting the product as is. When copying to a proposal, if any of the terms are changed, then auto_finalize is automatically set to false. |
| `web_property_code` | String | The web-property code for the seller. This needs to be copied as is when adding a new deal to a proposal. |
| `update_time` | String | Time of last update. |
| `targeting_criterion` | Vec<String> | Targeting that is shared between the buyer and the seller. Each targeting criterion has a specified key and for each key there is a list of inclusion value or exclusion values. |
| `product_revision` | String | The revision number of the product (auto-assigned by Marketplace). |
| `seller` | String | Information about the seller that created this product. |
| `syndication_product` | String | The syndication product associated with the deal. |
| `available_start_time` | String | Inventory availability dates. The start time will be truncated to seconds during serving. Thus, a field specified as 3:23:34.456 (HH:mm:ss.SSS) will be truncated to 3:23:34 when serving. |
| `available_end_time` | String | The proposed end time for the deal. The field will be truncated to the order of seconds during serving. |


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
product_creator_contacts = product.creator_contacts
product_terms = product.terms
product_product_id = product.product_id
product_create_time = product.create_time
product_publisher_profile_id = product.publisher_profile_id
product_display_name = product.display_name
product_has_creator_signed_off = product.has_creator_signed_off
product_web_property_code = product.web_property_code
product_update_time = product.update_time
product_targeting_criterion = product.targeting_criterion
product_product_revision = product.product_revision
product_seller = product.seller
product_syndication_product = product.syndication_product
product_available_start_time = product.available_start_time
product_available_end_time = product.available_end_time
```

---


### Non_billable_winning_bid

List all reasons for which winning bids were not billable, with the number of bids not billed for each reason.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token to retrieve the next page of results. Pass this value in the ListNonBillableWinningBidsRequest.pageToken field in the subsequent call to the nonBillableWinningBids.list method to retrieve the next page of results. |
| `non_billable_winning_bid_status_rows` | Vec<String> | List of rows, with counts of bids not billed aggregated by reason. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access non_billable_winning_bid outputs
non_billable_winning_bid_id = non_billable_winning_bid.id
non_billable_winning_bid_next_page_token = non_billable_winning_bid.next_page_token
non_billable_winning_bid_non_billable_winning_bid_status_rows = non_billable_winning_bid.non_billable_winning_bid_status_rows
```

---


### Bid_responses_without_bid

List all reasons for which bid responses were considered to have no applicable bids, with the number of bid responses affected for each reason.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token to retrieve the next page of results. Pass this value in the ListBidResponsesWithoutBidsRequest.pageToken field in the subsequent call to the bidResponsesWithoutBids.list method to retrieve the next page of results. |
| `bid_response_without_bids_status_rows` | Vec<String> | List of rows, with counts of bid responses without bids aggregated by status. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access bid_responses_without_bid outputs
bid_responses_without_bid_id = bid_responses_without_bid.id
bid_responses_without_bid_next_page_token = bid_responses_without_bid.next_page_token
bid_responses_without_bid_bid_response_without_bids_status_rows = bid_responses_without_bid.bid_response_without_bids_status_rows
```

---


### Deal_association

Associate an existing deal with a creative.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `association` | String |  | The association between a creative and a deal that should be added. |
| `account_id` | String | ✅ | The account the creative belongs to. |
| `creative_id` | String | ✅ | The ID of the creative associated with the deal. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associations` | Vec<String> | The list of associations. |
| `next_page_token` | String | A token to retrieve the next page of results. Pass this value in the ListDealAssociationsRequest.page_token field in the subsequent call to 'ListDealAssociation' method to retrieve the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deal_association
deal_association = provider.adexchangebuyer2_api.Deal_association {
    account_id = "value"  # The account the creative belongs to.
    creative_id = "value"  # The ID of the creative associated with the deal.
}

# Access deal_association outputs
deal_association_id = deal_association.id
deal_association_associations = deal_association.associations
deal_association_next_page_token = deal_association.next_page_token
```

---


### Filter_set

Creates the specified filter set for the account with the given account ID.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `environment` | String |  | The environment on which to filter; optional. |
| `breakdown_dimensions` | Vec<String> |  | The set of dimensions along which to break down the response; may be empty. If multiple dimensions are requested, the breakdown is along the Cartesian product of the requested dimensions. |
| `formats` | Vec<String> |  | Creative formats bidded on or allowed to bid on, can be empty. Although this field is a list, it can only be populated with a single item. A HTTP 400 bad request error will be returned in the response if you specify multiple items. |
| `name` | String |  | A user-defined name of the filter set. Filter set names must be unique globally and match one of the patterns: - `bidders/*/filterSets/*` (for accessing bidder-level troubleshooting data) - `bidders/*/accounts/*/filterSets/*` (for accessing account-level troubleshooting data) This field is required in create operations. |
| `deal_id` | String |  | The ID of the deal on which to filter; optional. This field may be set only for a filter set that accesses account-level troubleshooting data, for example, one whose name matches the `bidders/*/accounts/*/filterSets/*` pattern. |
| `creative_id` | String |  | The ID of the creative on which to filter; optional. This field may be set only for a filter set that accesses account-level troubleshooting data, for example, one whose name matches the `bidders/*/accounts/*/filterSets/*` pattern. |
| `platforms` | Vec<String> |  | The list of platforms on which to filter; may be empty. The filters represented by multiple platforms are ORed together (for example, if non-empty, results must match any one of the platforms). |
| `publisher_identifiers` | Vec<String> |  | For Open Bidding partners only. The list of publisher identifiers on which to filter; may be empty. The filters represented by multiple publisher identifiers are ORed together. |
| `seller_network_ids` | Vec<i64> |  | For Authorized Buyers only. The list of IDs of the seller (publisher) networks on which to filter; may be empty. The filters represented by multiple seller network IDs are ORed together (for example, if non-empty, results must match any one of the publisher networks). See [seller-network-ids](https://developers.google.com/authorized-buyers/rtb/downloads/seller-network-ids) file for the set of existing seller network IDs. |
| `time_series_granularity` | String |  | The granularity of time intervals if a time series breakdown is preferred; optional. |
| `realtime_time_range` | String |  | An open-ended realtime time range, defined by the aggregation start timestamp. |
| `format` | String |  | Creative format bidded on or allowed to bid on, can be empty. |
| `relative_date_range` | String |  | A relative date range, defined by an offset from today and a duration. Interpreted relative to Pacific time zone. |
| `absolute_date_range` | String |  | An absolute date range, defined by a start date and an end date. Interpreted relative to Pacific time zone. |
| `owner_name` | String | ✅ | Name of the owner (bidder or account) of the filter set to be created. For example: - For a bidder-level filter set for bidder 123: `bidders/123` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `environment` | String | The environment on which to filter; optional. |
| `breakdown_dimensions` | Vec<String> | The set of dimensions along which to break down the response; may be empty. If multiple dimensions are requested, the breakdown is along the Cartesian product of the requested dimensions. |
| `formats` | Vec<String> | Creative formats bidded on or allowed to bid on, can be empty. Although this field is a list, it can only be populated with a single item. A HTTP 400 bad request error will be returned in the response if you specify multiple items. |
| `name` | String | A user-defined name of the filter set. Filter set names must be unique globally and match one of the patterns: - `bidders/*/filterSets/*` (for accessing bidder-level troubleshooting data) - `bidders/*/accounts/*/filterSets/*` (for accessing account-level troubleshooting data) This field is required in create operations. |
| `deal_id` | String | The ID of the deal on which to filter; optional. This field may be set only for a filter set that accesses account-level troubleshooting data, for example, one whose name matches the `bidders/*/accounts/*/filterSets/*` pattern. |
| `creative_id` | String | The ID of the creative on which to filter; optional. This field may be set only for a filter set that accesses account-level troubleshooting data, for example, one whose name matches the `bidders/*/accounts/*/filterSets/*` pattern. |
| `platforms` | Vec<String> | The list of platforms on which to filter; may be empty. The filters represented by multiple platforms are ORed together (for example, if non-empty, results must match any one of the platforms). |
| `publisher_identifiers` | Vec<String> | For Open Bidding partners only. The list of publisher identifiers on which to filter; may be empty. The filters represented by multiple publisher identifiers are ORed together. |
| `seller_network_ids` | Vec<i64> | For Authorized Buyers only. The list of IDs of the seller (publisher) networks on which to filter; may be empty. The filters represented by multiple seller network IDs are ORed together (for example, if non-empty, results must match any one of the publisher networks). See [seller-network-ids](https://developers.google.com/authorized-buyers/rtb/downloads/seller-network-ids) file for the set of existing seller network IDs. |
| `time_series_granularity` | String | The granularity of time intervals if a time series breakdown is preferred; optional. |
| `realtime_time_range` | String | An open-ended realtime time range, defined by the aggregation start timestamp. |
| `format` | String | Creative format bidded on or allowed to bid on, can be empty. |
| `relative_date_range` | String | A relative date range, defined by an offset from today and a duration. Interpreted relative to Pacific time zone. |
| `absolute_date_range` | String | An absolute date range, defined by a start date and an end date. Interpreted relative to Pacific time zone. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create filter_set
filter_set = provider.adexchangebuyer2_api.Filter_set {
    owner_name = "value"  # Name of the owner (bidder or account) of the filter set to be created. For example: - For a bidder-level filter set for bidder 123: `bidders/123` - For an account-level filter set for the buyer account representing bidder 123: `bidders/123/accounts/123` - For an account-level filter set for the child seat buyer account 456 whose bidder is 123: `bidders/123/accounts/456`
}

# Access filter_set outputs
filter_set_id = filter_set.id
filter_set_environment = filter_set.environment
filter_set_breakdown_dimensions = filter_set.breakdown_dimensions
filter_set_formats = filter_set.formats
filter_set_name = filter_set.name
filter_set_deal_id = filter_set.deal_id
filter_set_creative_id = filter_set.creative_id
filter_set_platforms = filter_set.platforms
filter_set_publisher_identifiers = filter_set.publisher_identifiers
filter_set_seller_network_ids = filter_set.seller_network_ids
filter_set_time_series_granularity = filter_set.time_series_granularity
filter_set_realtime_time_range = filter_set.realtime_time_range
filter_set_format = filter_set.format
filter_set_relative_date_range = filter_set.relative_date_range
filter_set_absolute_date_range = filter_set.absolute_date_range
```

---


### Bid_response_error

List all errors that occurred in bid responses, with the number of bid responses affected for each reason.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token to retrieve the next page of results. Pass this value in the ListBidResponseErrorsRequest.pageToken field in the subsequent call to the bidResponseErrors.list method to retrieve the next page of results. |
| `callout_status_rows` | Vec<String> | List of rows, with counts of bid responses aggregated by callout status. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access bid_response_error outputs
bid_response_error_id = bid_response_error.id
bid_response_error_next_page_token = bid_response_error.next_page_token
bid_response_error_callout_status_rows = bid_response_error.callout_status_rows
```

---


### Detail

List all details associated with a specific reason for which bids were filtered, with the number of bids filtered for each detail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `filtered_bid_detail_rows` | Vec<String> | List of rows, with counts of bids with a given creative status aggregated by detail. |
| `next_page_token` | String | A token to retrieve the next page of results. Pass this value in the ListCreativeStatusBreakdownByDetailRequest.pageToken field in the subsequent call to the filteredBids.details.list method to retrieve the next page of results. |
| `detail_type` | String | The type of detail that the detail IDs represent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access detail outputs
detail_id = detail.id
detail_filtered_bid_detail_rows = detail.filtered_bid_detail_rows
detail_next_page_token = detail.next_page_token
detail_detail_type = detail.detail_type
```

---


### Losing_bid

List all reasons for which bids lost in the auction, with the number of bids that lost for each reason.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token to retrieve the next page of results. Pass this value in the ListLosingBidsRequest.pageToken field in the subsequent call to the losingBids.list method to retrieve the next page of results. |
| `creative_status_rows` | Vec<String> | List of rows, with counts of losing bids aggregated by loss reason (for example, creative status). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access losing_bid outputs
losing_bid_id = losing_bid.id
losing_bid_next_page_token = losing_bid.next_page_token
losing_bid_creative_status_rows = losing_bid.creative_status_rows
```

---


### Impression_metric

Lists all metrics that are measured in terms of number of impressions.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `impression_metrics_rows` | Vec<String> | List of rows, each containing a set of impression metrics. |
| `next_page_token` | String | A token to retrieve the next page of results. Pass this value in the ListImpressionMetricsRequest.pageToken field in the subsequent call to the impressionMetrics.list method to retrieve the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access impression_metric outputs
impression_metric_id = impression_metric.id
impression_metric_impression_metrics_rows = impression_metric.impression_metrics_rows
impression_metric_next_page_token = impression_metric.next_page_token
```

---


### Publisher_profile

Gets the requested publisher profile by id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audience_description` | String | Description on the publisher's audience. |
| `direct_deals_contact` | String | Contact information for direct reservation deals. This is free text entered by the publisher and may include information like names, phone numbers and email addresses. |
| `mobile_apps` | Vec<String> | The list of apps represented in this publisher profile. Empty if this is a parent profile. |
| `top_headlines` | Vec<String> | Up to three key metrics and rankings. Max 100 characters each. For example "#1 Mobile News Site for 20 Straight Months". |
| `google_plus_url` | String | URL to publisher's Google+ page. |
| `rate_card_info_url` | String | URL to a publisher rate card. |
| `media_kit_url` | String | URL to additional marketing and sales materials. |
| `domains` | Vec<String> | The list of domains represented in this publisher profile. Empty if this is a parent profile. These are top private domains, meaning that these will not contain a string like "photos.google.co.uk/123", but will instead contain "google.co.uk". |
| `overview` | String | Overview of the publisher. |
| `publisher_profile_id` | String | Unique ID for publisher profile. |
| `sample_page_url` | String | URL to a sample content page. |
| `display_name` | String | Name of the publisher profile. |
| `is_parent` | bool | Indicates if this profile is the parent profile of the seller. A parent profile represents all the inventory from the seller, as opposed to child profile that is created to brand a portion of inventory. One seller should have only one parent publisher profile, and can have multiple child profiles. Publisher profiles for the same seller will have same value of field google.ads.adexchange.buyer.v2beta1.PublisherProfile.seller. See https://support.google.com/admanager/answer/6035806 for details. |
| `buyer_pitch_statement` | String | Statement explaining what's unique about publisher's business, and why buyers should partner with the publisher. |
| `programmatic_deals_contact` | String | Contact information for programmatic deals. This is free text entered by the publisher and may include information like names, phone numbers and email addresses. |
| `seller` | String | Seller of the publisher profile. |
| `logo_url` | String | A Google public URL to the logo for this publisher profile. The logo is stored as a PNG, JPG, or GIF image. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access publisher_profile outputs
publisher_profile_id = publisher_profile.id
publisher_profile_audience_description = publisher_profile.audience_description
publisher_profile_direct_deals_contact = publisher_profile.direct_deals_contact
publisher_profile_mobile_apps = publisher_profile.mobile_apps
publisher_profile_top_headlines = publisher_profile.top_headlines
publisher_profile_google_plus_url = publisher_profile.google_plus_url
publisher_profile_rate_card_info_url = publisher_profile.rate_card_info_url
publisher_profile_media_kit_url = publisher_profile.media_kit_url
publisher_profile_domains = publisher_profile.domains
publisher_profile_overview = publisher_profile.overview
publisher_profile_publisher_profile_id = publisher_profile.publisher_profile_id
publisher_profile_sample_page_url = publisher_profile.sample_page_url
publisher_profile_display_name = publisher_profile.display_name
publisher_profile_is_parent = publisher_profile.is_parent
publisher_profile_buyer_pitch_statement = publisher_profile.buyer_pitch_statement
publisher_profile_programmatic_deals_contact = publisher_profile.programmatic_deals_contact
publisher_profile_seller = publisher_profile.seller
publisher_profile_logo_url = publisher_profile.logo_url
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple user resources
user_0 = provider.adexchangebuyer2_api.User {
    client_account_id = "value-0"
    account_id = "value-0"
    user_id = "value-0"
}
user_1 = provider.adexchangebuyer2_api.User {
    client_account_id = "value-1"
    account_id = "value-1"
    user_id = "value-1"
}
user_2 = provider.adexchangebuyer2_api.User {
    client_account_id = "value-2"
    account_id = "value-2"
    user_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    user = provider.adexchangebuyer2_api.User {
        client_account_id = "production-value"
        account_id = "production-value"
        user_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Adexchangebuyer2_api Documentation](https://cloud.google.com/adexchangebuyer2_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
