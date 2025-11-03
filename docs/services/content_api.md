# Content_api Service



**Resources**: 52

---

## Overview

The content_api service provides access to 52 resource types:

- [Promotion](#promotion) [CR]
- [Shoppingadsprogram](#shoppingadsprogram) [CR]
- [Collectionstatuse](#collectionstatuse) [R]
- [Recommendation](#recommendation) [CR]
- [Pubsubnotificationsetting](#pubsubnotificationsetting) [RU]
- [Po](#po) [CRD]
- [Collection](#collection) [CRD]
- [Ordertrackingsignal](#ordertrackingsignal) [C]
- [Regionalinventory](#regionalinventory) [C]
- [Returnaddres](#returnaddres) [CRD]
- [Product](#product) [CRUD]
- [Datafeed](#datafeed) [CRUD]
- [Conversionsource](#conversionsource) [CRUD]
- [Returncarrier](#returncarrier) [CRUD]
- [Accountstatuse](#accountstatuse) [CR]
- [Csse](#csse) [CR]
- [Region](#region) [CRUD]
- [Productstatuse](#productstatuse) [CR]
- [Label](#label) [CRUD]
- [Returnpolicy](#returnpolicy) [CRD]
- [Datafeedstatuse](#datafeedstatuse) [CR]
- [Report](#report) [C]
- [Quota](#quota) [R]
- [Account](#account) [CRUD]
- [Liasetting](#liasetting) [CRU]
- [Shippingsetting](#shippingsetting) [CRU]
- [Productdeliverytime](#productdeliverytime) [CRD]
- [Merchantsupport](#merchantsupport) [C]
- [Credential](#credential) [C]
- [Returnpolicyonline](#returnpolicyonline) [CRUD]
- [Localinventory](#localinventory) [C]
- [Freelistingsprogram](#freelistingsprogram) [CR]
- [Checkoutsetting](#checkoutsetting) [CRD]
- [Accounttax](#accounttax) [CRU]
- [Orderpayment](#orderpayment) [C]
- [Order](#order) [CR]
- [Orderreturn](#orderreturn) [R]
- [Orderinvoice](#orderinvoice) [C]
- [Shippingsetting](#shippingsetting) [CRU]
- [Orderreport](#orderreport) [R]
- [Liasetting](#liasetting) [CRU]
- [Orderinvoice](#orderinvoice) [C]
- [Datafeedstatuse](#datafeedstatuse) [CR]
- [Accountstatuse](#accountstatuse) [CR]
- [Datafeed](#datafeed) [CRUD]
- [Accounttax](#accounttax) [CRU]
- [Po](#po) [CRD]
- [Order](#order) [CR]
- [Orderreturn](#orderreturn) [R]
- [Account](#account) [CRUD]
- [Productstatuse](#productstatuse) [CR]
- [Product](#product) [CRD]

---

## Resources


### Promotion

Inserts a promotion for your Merchant Center account. If the promotion already exists, then it updates the promotion instead. To [end or delete] (https://developers.google.com/shopping-content/guides/promotions#end_a_promotion) a promotion update the time period of the promotion to a time that has already passed.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `promotion_id` | String |  | Required. The user provided promotion ID to uniquely identify the promotion. |
| `percent_off` | i64 |  | The percentage discount offered in the promotion. |
| `item_group_id_exclusion` | Vec<String> |  | Product filter by item group ID exclusion for the promotion. |
| `free_gift_value` | String |  | Free gift value for the promotion. |
| `brand` | Vec<String> |  | Product filter by brand for the promotion. |
| `free_gift_description` | String |  | Free gift description for the promotion. |
| `product_type` | Vec<String> |  | Product filter by product type for the promotion. |
| `store_applicability` | String |  | Whether the promotion applies to all stores, or only specified stores. Local Inventory ads promotions throw an error if no store applicability is included. An INVALID_ARGUMENT error is thrown if store_applicability is set to ALL_STORES and store_code or score_code_exclusion is set to a value. |
| `generic_redemption_code` | String |  | Generic redemption code for the promotion. To be used with the `offerType` field. |
| `promotion_effective_dates` | String |  | String representation of the promotion effective dates. Deprecated. Use `promotion_effective_time_period` instead. |
| `item_id` | Vec<String> |  | Product filter by item ID for the promotion. |
| `promotion_display_dates` | String |  | String representation of the promotion display dates. Deprecated. Use `promotion_display_time_period` instead. |
| `minimum_purchase_quantity` | i64 |  | Minimum purchase quantity for the promotion. |
| `store_code` | Vec<String> |  | Store codes to include for the promotion. |
| `promotion_display_time_period` | String |  | `TimePeriod` representation of the promotion's display dates. |
| `limit_value` | String |  | Maximum purchase value for the promotion. |
| `max_discount_amount` | String |  | The maximum monetary discount a customer can receive for the promotion. This field is only supported with the `Percent off` coupon value type. |
| `store_code_exclusion` | Vec<String> |  | Store codes to exclude for the promotion. |
| `id` | String |  | Output only. The REST promotion ID to uniquely identify the promotion. Content API methods that operate on promotions take this as their `promotionId` parameter. The REST ID for a promotion is of the form channel:contentLanguage:targetCountry:promotionId The `channel` field has a value of `"online"`, `"in_store"`, or `"online_in_store"`. |
| `content_language` | String |  | Required. The content language used as part of the unique identifier. `en` content language is available for all target countries. `fr` content language is available for `CA` and `FR` target countries. `de` content language is available for `DE` target country. `nl` content language is available for `NL` target country. `it` content language is available for `IT` target country. `pt` content language is available for `BR` target country. `ja` content language is available for `JP` target country. `ko` content language is available for `KR` target country. |
| `item_id_exclusion` | Vec<String> |  | Product filter by item ID exclusion for the promotion. |
| `long_title` | String |  | Required. Long title for the promotion. |
| `limit_quantity` | i64 |  | Maximum purchase quantity for the promotion. |
| `free_gift_item_id` | String |  | Free gift item ID for the promotion. |
| `item_group_id` | Vec<String> |  | Product filter by item group ID for the promotion. |
| `offer_type` | String |  | Required. Type of the promotion. |
| `product_type_exclusion` | Vec<String> |  | Product filter by product type exclusion for the promotion. |
| `custom_redemption_restriction` | String |  | The custom redemption restriction for the promotion. If the `redemption_restriction` field is set to `CUSTOM`, this field must be set. |
| `minimum_purchase_amount` | String |  | Minimum purchase amount for the promotion. |
| `shipping_service_names` | Vec<String> |  | Shipping service names for the promotion. |
| `promotion_destination_ids` | Vec<String> |  | Destination ID for the promotion. |
| `money_off_amount` | String |  | The money off amount offered in the promotion. |
| `promotion_url` | String |  | URL to the page on the merchant's site where the promotion shows. Local Inventory ads promotions throw an error if no promo url is included. URL is used to confirm that the promotion is valid and can be redeemed. |
| `product_applicability` | String |  | Required. Applicability of the promotion to either all products or only specific products. |
| `promotion_status` | String |  | Output only. The current status of the promotion. |
| `target_country` | String |  | Required. The target country used as part of the unique identifier. Can be `AU`, `CA`, `DE`, `FR`, `GB`, `IN`, `US`, `BR`, `ES`, `NL`, `JP`, `IT` or `KR`. |
| `order_limit` | i64 |  | Order limit for the promotion. |
| `promotion_effective_time_period` | String |  | Required. `TimePeriod` representation of the promotion's effective dates. |
| `redemption_channel` | Vec<String> |  | Required. Redemption channel for the promotion. At least one channel is required. |
| `brand_exclusion` | Vec<String> |  | Product filter by brand exclusion for the promotion. |
| `money_budget` | String |  | Cost cap for the promotion. |
| `get_this_quantity_discounted` | i64 |  | The number of items discounted in the promotion. |
| `coupon_value_type` | String |  | Required. Coupon value type for the promotion. |
| `redemption_restriction` | String |  | The redemption restriction for the promotion. |
| `merchant_id` | String | ✅ | Required. The ID of the account that contains the collection. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `promotion_id` | String | Required. The user provided promotion ID to uniquely identify the promotion. |
| `percent_off` | i64 | The percentage discount offered in the promotion. |
| `item_group_id_exclusion` | Vec<String> | Product filter by item group ID exclusion for the promotion. |
| `free_gift_value` | String | Free gift value for the promotion. |
| `brand` | Vec<String> | Product filter by brand for the promotion. |
| `free_gift_description` | String | Free gift description for the promotion. |
| `product_type` | Vec<String> | Product filter by product type for the promotion. |
| `store_applicability` | String | Whether the promotion applies to all stores, or only specified stores. Local Inventory ads promotions throw an error if no store applicability is included. An INVALID_ARGUMENT error is thrown if store_applicability is set to ALL_STORES and store_code or score_code_exclusion is set to a value. |
| `generic_redemption_code` | String | Generic redemption code for the promotion. To be used with the `offerType` field. |
| `promotion_effective_dates` | String | String representation of the promotion effective dates. Deprecated. Use `promotion_effective_time_period` instead. |
| `item_id` | Vec<String> | Product filter by item ID for the promotion. |
| `promotion_display_dates` | String | String representation of the promotion display dates. Deprecated. Use `promotion_display_time_period` instead. |
| `minimum_purchase_quantity` | i64 | Minimum purchase quantity for the promotion. |
| `store_code` | Vec<String> | Store codes to include for the promotion. |
| `promotion_display_time_period` | String | `TimePeriod` representation of the promotion's display dates. |
| `limit_value` | String | Maximum purchase value for the promotion. |
| `max_discount_amount` | String | The maximum monetary discount a customer can receive for the promotion. This field is only supported with the `Percent off` coupon value type. |
| `store_code_exclusion` | Vec<String> | Store codes to exclude for the promotion. |
| `id` | String | Output only. The REST promotion ID to uniquely identify the promotion. Content API methods that operate on promotions take this as their `promotionId` parameter. The REST ID for a promotion is of the form channel:contentLanguage:targetCountry:promotionId The `channel` field has a value of `"online"`, `"in_store"`, or `"online_in_store"`. |
| `content_language` | String | Required. The content language used as part of the unique identifier. `en` content language is available for all target countries. `fr` content language is available for `CA` and `FR` target countries. `de` content language is available for `DE` target country. `nl` content language is available for `NL` target country. `it` content language is available for `IT` target country. `pt` content language is available for `BR` target country. `ja` content language is available for `JP` target country. `ko` content language is available for `KR` target country. |
| `item_id_exclusion` | Vec<String> | Product filter by item ID exclusion for the promotion. |
| `long_title` | String | Required. Long title for the promotion. |
| `limit_quantity` | i64 | Maximum purchase quantity for the promotion. |
| `free_gift_item_id` | String | Free gift item ID for the promotion. |
| `item_group_id` | Vec<String> | Product filter by item group ID for the promotion. |
| `offer_type` | String | Required. Type of the promotion. |
| `product_type_exclusion` | Vec<String> | Product filter by product type exclusion for the promotion. |
| `custom_redemption_restriction` | String | The custom redemption restriction for the promotion. If the `redemption_restriction` field is set to `CUSTOM`, this field must be set. |
| `minimum_purchase_amount` | String | Minimum purchase amount for the promotion. |
| `shipping_service_names` | Vec<String> | Shipping service names for the promotion. |
| `promotion_destination_ids` | Vec<String> | Destination ID for the promotion. |
| `money_off_amount` | String | The money off amount offered in the promotion. |
| `promotion_url` | String | URL to the page on the merchant's site where the promotion shows. Local Inventory ads promotions throw an error if no promo url is included. URL is used to confirm that the promotion is valid and can be redeemed. |
| `product_applicability` | String | Required. Applicability of the promotion to either all products or only specific products. |
| `promotion_status` | String | Output only. The current status of the promotion. |
| `target_country` | String | Required. The target country used as part of the unique identifier. Can be `AU`, `CA`, `DE`, `FR`, `GB`, `IN`, `US`, `BR`, `ES`, `NL`, `JP`, `IT` or `KR`. |
| `order_limit` | i64 | Order limit for the promotion. |
| `promotion_effective_time_period` | String | Required. `TimePeriod` representation of the promotion's effective dates. |
| `redemption_channel` | Vec<String> | Required. Redemption channel for the promotion. At least one channel is required. |
| `brand_exclusion` | Vec<String> | Product filter by brand exclusion for the promotion. |
| `money_budget` | String | Cost cap for the promotion. |
| `get_this_quantity_discounted` | i64 | The number of items discounted in the promotion. |
| `coupon_value_type` | String | Required. Coupon value type for the promotion. |
| `redemption_restriction` | String | The redemption restriction for the promotion. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create promotion
promotion = provider.content_api.Promotion {
    merchant_id = "value"  # Required. The ID of the account that contains the collection.
}

# Access promotion outputs
promotion_id = promotion.id
promotion_promotion_id = promotion.promotion_id
promotion_percent_off = promotion.percent_off
promotion_item_group_id_exclusion = promotion.item_group_id_exclusion
promotion_free_gift_value = promotion.free_gift_value
promotion_brand = promotion.brand
promotion_free_gift_description = promotion.free_gift_description
promotion_product_type = promotion.product_type
promotion_store_applicability = promotion.store_applicability
promotion_generic_redemption_code = promotion.generic_redemption_code
promotion_promotion_effective_dates = promotion.promotion_effective_dates
promotion_item_id = promotion.item_id
promotion_promotion_display_dates = promotion.promotion_display_dates
promotion_minimum_purchase_quantity = promotion.minimum_purchase_quantity
promotion_store_code = promotion.store_code
promotion_promotion_display_time_period = promotion.promotion_display_time_period
promotion_limit_value = promotion.limit_value
promotion_max_discount_amount = promotion.max_discount_amount
promotion_store_code_exclusion = promotion.store_code_exclusion
promotion_id = promotion.id
promotion_content_language = promotion.content_language
promotion_item_id_exclusion = promotion.item_id_exclusion
promotion_long_title = promotion.long_title
promotion_limit_quantity = promotion.limit_quantity
promotion_free_gift_item_id = promotion.free_gift_item_id
promotion_item_group_id = promotion.item_group_id
promotion_offer_type = promotion.offer_type
promotion_product_type_exclusion = promotion.product_type_exclusion
promotion_custom_redemption_restriction = promotion.custom_redemption_restriction
promotion_minimum_purchase_amount = promotion.minimum_purchase_amount
promotion_shipping_service_names = promotion.shipping_service_names
promotion_promotion_destination_ids = promotion.promotion_destination_ids
promotion_money_off_amount = promotion.money_off_amount
promotion_promotion_url = promotion.promotion_url
promotion_product_applicability = promotion.product_applicability
promotion_promotion_status = promotion.promotion_status
promotion_target_country = promotion.target_country
promotion_order_limit = promotion.order_limit
promotion_promotion_effective_time_period = promotion.promotion_effective_time_period
promotion_redemption_channel = promotion.redemption_channel
promotion_brand_exclusion = promotion.brand_exclusion
promotion_money_budget = promotion.money_budget
promotion_get_this_quantity_discounted = promotion.get_this_quantity_discounted
promotion_coupon_value_type = promotion.coupon_value_type
promotion_redemption_restriction = promotion.redemption_restriction
```

---


### Shoppingadsprogram

Requests a review of Shopping ads in a specific region. This method deprecated. Use the `MerchantSupportService` to view product and account issues and request a review.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `region_code` | String |  | The code [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) of the country for which review is to be requested. |
| `merchant_id` | String | ✅ | Required. The ID of the account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `global_state` | String | State of the program. `ENABLED` if there are offers for at least one region. |
| `region_statuses` | Vec<String> | Status of the program in each region. Regions with the same status and review eligibility are grouped together in `regionCodes`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create shoppingadsprogram
shoppingadsprogram = provider.content_api.Shoppingadsprogram {
    merchant_id = "value"  # Required. The ID of the account.
}

# Access shoppingadsprogram outputs
shoppingadsprogram_id = shoppingadsprogram.id
shoppingadsprogram_global_state = shoppingadsprogram.global_state
shoppingadsprogram_region_statuses = shoppingadsprogram.region_statuses
```

---


### Collectionstatuse

Gets the status of a collection from your Merchant Center account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | Date on which the collection has been created in [ISO 8601](http://en.wikipedia.org/wiki/ISO_8601) format: Date, time, and offset, for example "2020-01-02T09:00:00+01:00" or "2020-01-02T09:00:00Z" |
| `id` | String | Required. The ID of the collection for which status is reported. |
| `last_update_date` | String | Date on which the collection has been last updated in [ISO 8601](http://en.wikipedia.org/wiki/ISO_8601) format: Date, time, and offset, for example "2020-01-02T09:00:00+01:00" or "2020-01-02T09:00:00Z" |
| `destination_statuses` | Vec<String> | The intended destinations for the collection. |
| `collection_level_issuses` | Vec<String> | A list of all issues associated with the collection. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access collectionstatuse outputs
collectionstatuse_id = collectionstatuse.id
collectionstatuse_creation_date = collectionstatuse.creation_date
collectionstatuse_id = collectionstatuse.id
collectionstatuse_last_update_date = collectionstatuse.last_update_date
collectionstatuse_destination_statuses = collectionstatuse.destination_statuses
collectionstatuse_collection_level_issuses = collectionstatuse.collection_level_issuses
```

---


### Recommendation

Reports an interaction on a recommendation for a merchant.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subtype` | String |  | Optional. Subtype of the recommendations this interaction happened on. This field must be set only to the value that is returned by {@link `RecommendationsService.GenerateRecommendations`} call. |
| `type` | String |  | Required. Type of the recommendations on which this interaction happened. This field must be set only to the value that is returned by {@link `GenerateRecommendationsResponse`} call. |
| `interaction_type` | String |  | Required. Type of the interaction that is reported, for example INTERACTION_CLICK. |
| `response_token` | String |  | Required. Token of the response when recommendation was returned. |
| `merchant_id` | String | ✅ | Required. The ID of the account that wants to report an interaction. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommendations` | Vec<String> | Recommendations generated for a request. |
| `response_token` | String | Output only. Response token is a string created for each `GenerateRecommendationsResponse`. This token doesn't expire, and is globally unique. This token must be used when reporting interactions for recommendations. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create recommendation
recommendation = provider.content_api.Recommendation {
    merchant_id = "value"  # Required. The ID of the account that wants to report an interaction.
}

# Access recommendation outputs
recommendation_id = recommendation.id
recommendation_recommendations = recommendation.recommendations
recommendation_response_token = recommendation.response_token
```

---


### Pubsubnotificationsetting

Retrieves a Merchant Center account's pubsub notification settings.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cloud_topic_name` | String |  | Cloud pub/sub topic to which notifications are sent (read-only). |
| `registered_events` | Vec<String> |  | List of event types. Acceptable values are: - "`orderPendingShipment`"  |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#pubsubNotificationSettings`" |
| `merchant_id` | String | ✅ | The ID of the account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_topic_name` | String | Cloud pub/sub topic to which notifications are sent (read-only). |
| `registered_events` | Vec<String> | List of event types. Acceptable values are: - "`orderPendingShipment`"  |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#pubsubNotificationSettings`" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access pubsubnotificationsetting outputs
pubsubnotificationsetting_id = pubsubnotificationsetting.id
pubsubnotificationsetting_cloud_topic_name = pubsubnotificationsetting.cloud_topic_name
pubsubnotificationsetting_registered_events = pubsubnotificationsetting.registered_events
pubsubnotificationsetting_kind = pubsubnotificationsetting.kind
```

---


### Po

Creates a store for the given merchant.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `store_name` | String |  | The merchant or store name. |
| `store_address` | String |  | Required. The street address of the store. |
| `matching_status` | String |  | Output only. The matching status of POS store and Google Business Profile store. Possible values are: - "`matched`": The POS store is successfully matched with the Google Business Profile store. - "`failed`": The POS store is not matched with the Google Business Profile store. See matching_status_hint for further details. Note that there is up to 48 hours propagation delay for changes in Merchant Center (e.g. creation of new account, accounts linking) and Google Business Profile (e.g. store address update) which may affect the matching status. In such cases, after a delay call [pos.list](https://developers.google.com/shopping-content/reference/rest/v2.1/pos/list) to retrieve the updated matching status.  |
| `website_url` | String |  | The website url for the store or merchant. |
| `phone_number` | String |  | The store phone number. |
| `matching_status_hint` | String |  | Output only. The hint of why the matching has failed. This is only set when matching_status=failed. Possible values are: - "`linked-store-not-found`": There aren't any Google Business Profile stores available for matching. Connect your Merchant Center account with the Google Business Profile account. Or add a new Google Business Profile store corresponding to the POS store. - "`store-match-not-found`": The provided POS store couldn't be matched to any of the connected Google Business Profile stores. Merchant Center account is connected correctly and stores are available on Google Business Profile, but POS store location address does not match with Google Business Profile stores' addresses. Update POS store address or Google Business Profile store address to match correctly. - "`store-match-unverified`": The provided POS store couldn't be matched to any of the connected Google Business Profile stores, as the matched Google Business Profile store is unverified. Go through the Google Business Profile verification process to match correctly.  |
| `place_id` | String |  | The Google Place Id of the store location. |
| `gcid_category` | Vec<String> |  | The business type of the store. |
| `store_code` | String |  | Required. A store identifier that is unique for the given merchant. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#posStore`" |
| `merchant_id` | String | ✅ | The ID of the POS or inventory data provider. |
| `target_merchant_id` | String | ✅ | The ID of the target merchant. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `store_name` | String | The merchant or store name. |
| `store_address` | String | Required. The street address of the store. |
| `matching_status` | String | Output only. The matching status of POS store and Google Business Profile store. Possible values are: - "`matched`": The POS store is successfully matched with the Google Business Profile store. - "`failed`": The POS store is not matched with the Google Business Profile store. See matching_status_hint for further details. Note that there is up to 48 hours propagation delay for changes in Merchant Center (e.g. creation of new account, accounts linking) and Google Business Profile (e.g. store address update) which may affect the matching status. In such cases, after a delay call [pos.list](https://developers.google.com/shopping-content/reference/rest/v2.1/pos/list) to retrieve the updated matching status.  |
| `website_url` | String | The website url for the store or merchant. |
| `phone_number` | String | The store phone number. |
| `matching_status_hint` | String | Output only. The hint of why the matching has failed. This is only set when matching_status=failed. Possible values are: - "`linked-store-not-found`": There aren't any Google Business Profile stores available for matching. Connect your Merchant Center account with the Google Business Profile account. Or add a new Google Business Profile store corresponding to the POS store. - "`store-match-not-found`": The provided POS store couldn't be matched to any of the connected Google Business Profile stores. Merchant Center account is connected correctly and stores are available on Google Business Profile, but POS store location address does not match with Google Business Profile stores' addresses. Update POS store address or Google Business Profile store address to match correctly. - "`store-match-unverified`": The provided POS store couldn't be matched to any of the connected Google Business Profile stores, as the matched Google Business Profile store is unverified. Go through the Google Business Profile verification process to match correctly.  |
| `place_id` | String | The Google Place Id of the store location. |
| `gcid_category` | Vec<String> | The business type of the store. |
| `store_code` | String | Required. A store identifier that is unique for the given merchant. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#posStore`" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create po
po = provider.content_api.Po {
    merchant_id = "value"  # The ID of the POS or inventory data provider.
    target_merchant_id = "value"  # The ID of the target merchant.
}

# Access po outputs
po_id = po.id
po_store_name = po.store_name
po_store_address = po.store_address
po_matching_status = po.matching_status
po_website_url = po.website_url
po_phone_number = po.phone_number
po_matching_status_hint = po.matching_status_hint
po_place_id = po.place_id
po_gcid_category = po.gcid_category
po_store_code = po.store_code
po_kind = po.kind
```

---


### Collection

Uploads a collection to your Merchant Center account. If a collection with the same collectionId already exists, this method updates that entry. In each update, the collection is completely replaced by the fields in the body of the update request.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_label2` | String |  | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `id` | String |  | Required. The REST ID of the collection. Content API methods that operate on collections take this as their collectionId parameter. The REST ID for a collection is of the form collectionId. [id attribute](https://support.google.com/merchants/answer/9649290) |
| `featured_product` | Vec<String> |  | This identifies one or more products associated with the collection. Used as a lookup to the corresponding product ID in your product feeds. Provide a maximum of 100 featuredProduct (for collections). Provide up to 10 featuredProduct (for Shoppable Images only) with ID and X and Y coordinates. [featured_product attribute](https://support.google.com/merchants/answer/9703736) |
| `image_link` | Vec<String> |  | The URL of a collection’s image. [image_link attribute](https://support.google.com/merchants/answer/9703236) |
| `custom_label3` | String |  | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `link` | String |  | A collection’s landing page. URL directly linking to your collection's page on your website. [link attribute](https://support.google.com/merchants/answer/9673983) |
| `language` | String |  | The language of a collection and the language of any featured products linked to the collection. [language attribute](https://support.google.com/merchants/answer/9673781) |
| `product_country` | String |  | [product_country attribute](https://support.google.com/merchants/answer/9674155) |
| `custom_label1` | String |  | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `custom_label0` | String |  | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. [Custom label](https://support.google.com/merchants/answer/9674217) |
| `custom_label4` | String |  | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `mobile_link` | String |  | A collection’s mobile-optimized landing page when you have a different URL for mobile and desktop traffic. [mobile_link attribute](https://support.google.com/merchants/answer/9646123) |
| `headline` | Vec<String> |  | Your collection's name. [headline attribute](https://support.google.com/merchants/answer/9673580) |
| `merchant_id` | String | ✅ | Required. The ID of the account that contains the collection. This account cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_label2` | String | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `id` | String | Required. The REST ID of the collection. Content API methods that operate on collections take this as their collectionId parameter. The REST ID for a collection is of the form collectionId. [id attribute](https://support.google.com/merchants/answer/9649290) |
| `featured_product` | Vec<String> | This identifies one or more products associated with the collection. Used as a lookup to the corresponding product ID in your product feeds. Provide a maximum of 100 featuredProduct (for collections). Provide up to 10 featuredProduct (for Shoppable Images only) with ID and X and Y coordinates. [featured_product attribute](https://support.google.com/merchants/answer/9703736) |
| `image_link` | Vec<String> | The URL of a collection’s image. [image_link attribute](https://support.google.com/merchants/answer/9703236) |
| `custom_label3` | String | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `link` | String | A collection’s landing page. URL directly linking to your collection's page on your website. [link attribute](https://support.google.com/merchants/answer/9673983) |
| `language` | String | The language of a collection and the language of any featured products linked to the collection. [language attribute](https://support.google.com/merchants/answer/9673781) |
| `product_country` | String | [product_country attribute](https://support.google.com/merchants/answer/9674155) |
| `custom_label1` | String | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `custom_label0` | String | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. [Custom label](https://support.google.com/merchants/answer/9674217) |
| `custom_label4` | String | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `mobile_link` | String | A collection’s mobile-optimized landing page when you have a different URL for mobile and desktop traffic. [mobile_link attribute](https://support.google.com/merchants/answer/9646123) |
| `headline` | Vec<String> | Your collection's name. [headline attribute](https://support.google.com/merchants/answer/9673580) |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create collection
collection = provider.content_api.Collection {
    merchant_id = "value"  # Required. The ID of the account that contains the collection. This account cannot be a multi-client account.
}

# Access collection outputs
collection_id = collection.id
collection_custom_label2 = collection.custom_label2
collection_id = collection.id
collection_featured_product = collection.featured_product
collection_image_link = collection.image_link
collection_custom_label3 = collection.custom_label3
collection_link = collection.link
collection_language = collection.language
collection_product_country = collection.product_country
collection_custom_label1 = collection.custom_label1
collection_custom_label0 = collection.custom_label0
collection_custom_label4 = collection.custom_label4
collection_mobile_link = collection.mobile_link
collection_headline = collection.headline
```

---


### Ordertrackingsignal

Creates new order tracking signal.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delivery_region_code` | String |  | Required. The [CLDR territory code] (http://www.unicode.org/repos/cldr/tags/latest/common/main/en.xml) for the shipping destination. |
| `order_created_time` | String |  | Required. The time when the order was created on the merchant side. Include the year and timezone string, if available. |
| `delivery_postal_code` | String |  | Required. The delivery postal code, as a continuous string without spaces or dashes, e.g. "95016". This field will be anonymized in returned OrderTrackingSignal creation response. |
| `line_items` | Vec<String> |  | Information about line items in the order. |
| `customer_shipping_fee` | String |  | The shipping fee of the order; this value should be set to zero in the case of free shipping. |
| `merchant_id` | String |  | The Google merchant ID of this order tracking signal. This value is optional. If left unset, the caller's merchant ID is used. You must request access in order to provide data on behalf of another merchant. For more information, see [Submitting Order Tracking Signals](/shopping-content/guides/order-tracking-signals). |
| `shipping_info` | Vec<String> |  | The shipping information for the order. |
| `shipment_line_item_mapping` | Vec<String> |  | The mapping of the line items to the shipment information. |
| `order_id` | String |  | Required. The ID of the order on the merchant side. This field will be hashed in returned OrderTrackingSignal creation response. |
| `order_tracking_signal_id` | String |  | Output only. The ID that uniquely identifies this order tracking signal. |
| `merchant_id` | String | ✅ | The ID of the merchant for which the order signal is created. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ordertrackingsignal
ordertrackingsignal = provider.content_api.Ordertrackingsignal {
    merchant_id = "value"  # The ID of the merchant for which the order signal is created.
}

```

---


### Regionalinventory

Updates the regional inventory of a product in your Merchant Center account. If a regional inventory with the same region ID already exists, this method updates that entry.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `price` | String |  | The price of the product. |
| `custom_attributes` | Vec<String> |  | A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form. |
| `region_id` | String |  | The ID uniquely identifying each region. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#regionalInventory`". |
| `availability` | String |  | The availability of the product. |
| `sale_price` | String |  | The sale price of the product. Mandatory if `sale_price_effective_date` is defined. |
| `sale_price_effective_date` | String |  | A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates might be specified as 'null' if undecided. |
| `merchant_id` | String | ✅ | The ID of the account that contains the product. This account cannot be a multi-client account. |
| `product_id` | String | ✅ | The REST ID of the product for which to update the regional inventory. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create regionalinventory
regionalinventory = provider.content_api.Regionalinventory {
    merchant_id = "value"  # The ID of the account that contains the product. This account cannot be a multi-client account.
    product_id = "value"  # The REST ID of the product for which to update the regional inventory.
}

```

---


### Returnaddres

Inserts a return address for the Merchant Center account.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `address` | String |  | Required. The address. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#returnAddress`" |
| `country` | String |  | Required. The country of sale where the return address is applicable. |
| `phone_number` | String |  | Required. The merchant's contact phone number regarding the return. |
| `return_address_id` | String |  | Return address ID generated by Google. |
| `label` | String |  | Required. The user-defined label of the return address. For the default address, use the label "default". |
| `merchant_id` | String | ✅ | The Merchant Center account to insert a return address for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `address` | String | Required. The address. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#returnAddress`" |
| `country` | String | Required. The country of sale where the return address is applicable. |
| `phone_number` | String | Required. The merchant's contact phone number regarding the return. |
| `return_address_id` | String | Return address ID generated by Google. |
| `label` | String | Required. The user-defined label of the return address. For the default address, use the label "default". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create returnaddres
returnaddres = provider.content_api.Returnaddres {
    merchant_id = "value"  # The Merchant Center account to insert a return address for.
}

# Access returnaddres outputs
returnaddres_id = returnaddres.id
returnaddres_address = returnaddres.address
returnaddres_kind = returnaddres.kind
returnaddres_country = returnaddres.country
returnaddres_phone_number = returnaddres.phone_number
returnaddres_return_address_id = returnaddres.return_address_id
returnaddres_label = returnaddres.label
```

---


### Product

Uploads a product to your Merchant Center account. If an item with the same channel, contentLanguage, offerId, and targetCountry already exists, this method updates that entry.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `availability_date` | String |  | The day a pre-ordered product becomes available for delivery, in ISO 8601 format. |
| `display_ads_link` | String |  | URL directly to your item's landing page for dynamic remarketing campaigns. |
| `pause` | String |  | Publication of this item should be temporarily paused. Acceptable values are: - "`ads`"  |
| `price` | String |  | Price of the item. |
| `disclosure_date` | String |  | The date time when an offer becomes visible in search results across Google’s YouTube surfaces, in [ISO 8601](http://en.wikipedia.org/wiki/ISO_8601) format. See [Disclosure date](https://support.google.com/merchants/answer/13034208) for more information. |
| `min_handling_time` | String |  | Minimal product handling time (in business days). |
| `promotion_ids` | Vec<String> |  | The unique ID of a promotion. |
| `pickup_sla` | String |  | Item store pickup timeline. Acceptable values are: - "`same day`" - "`next day`" - "`2-day`" - "`3-day`" - "`4-day`" - "`5-day`" - "`6-day`" - "`7-day`" - "`multi-week`"  |
| `shipping_height` | String |  | Height of the item for shipping. |
| `product_length` | String |  | The length of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `subscription_cost` | String |  | Number of periods (months or years) and amount of payment per period for an item with an associated subscription contract. |
| `source` | String |  | Output only. The source of the offer, that is, how the offer was created. Acceptable values are: - "`api`" - "`crawl`" - "`feed`"  |
| `max_energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `google_product_category` | String |  | Google's category of the item (see [Google product taxonomy](https://support.google.com/merchants/answer/1705911)). When querying products, this field will contain the user provided value. There is currently no way to get back the auto assigned google product categories through the API. |
| `custom_label2` | String |  | Custom label 2 for custom grouping of items in a Shopping campaign. |
| `shopping_ads_excluded_countries` | Vec<String> |  | List of country codes (ISO 3166-1 alpha-2) to exclude the offer from Shopping Ads destination. Countries from this list are removed from countries configured in MC feed settings. |
| `sizes` | Vec<String> |  | Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition). |
| `shipping` | Vec<String> |  | Shipping rules. |
| `description` | String |  | Description of the item. |
| `shipping_width` | String |  | Width of the item for shipping. |
| `target_country` | String |  | Required. The CLDR territory code for the item's country of sale. |
| `additional_image_links` | Vec<String> |  | Additional URLs of images of the item. |
| `transit_time_label` | String |  | The transit time label of the product, used to group product in account-level transit time tables. |
| `ads_labels` | Vec<String> |  | Similar to ads_grouping, but only works on CPC. |
| `custom_label4` | String |  | Custom label 4 for custom grouping of items in a Shopping campaign. |
| `cloud_export_additional_properties` | Vec<String> |  | Extra fields to export to the Cloud Retail program. |
| `loyalty_programs` | Vec<String> |  | Optional. A list of loyalty program information that is used to surface loyalty benefits (for example, better pricing, points, etc) to the user of this item. |
| `color` | String |  | Color of the item. |
| `certifications` | Vec<String> |  | Product [certification](https://support.google.com/merchants/answer/13528839), introduced for EU energy efficiency labeling compliance using the [EU EPREL](https://eprel.ec.europa.eu/screen/home) database. |
| `free_shipping_threshold` | Vec<String> |  | Optional. Conditions to be met for a product to have free shipping. |
| `ads_grouping` | String |  | Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise. |
| `auto_pricing_min_price` | String |  | A safeguard in the [Automated Discounts](//support.google.com/merchants/answer/10295759) and [Dynamic Promotions](//support.google.com/merchants/answer/13949249) projects, ensuring that discounts on merchants' offers do not fall below this value, thereby preserving the offer's value and profitability. |
| `excluded_destinations` | Vec<String> |  | The list of [destinations to exclude](//support.google.com/merchants/answer/6324486) for this target (corresponds to cleared check boxes in Merchant Center). Products that are excluded from all destinations for more than 7 days are automatically deleted. |
| `id` | String |  | The REST ID of the product. Content API methods that operate on products take this as their `productId` parameter. The REST ID for a product has one of the 2 forms channel:contentLanguage: targetCountry: offerId or channel:contentLanguage:feedLabel: offerId. |
| `link` | String |  | URL directly linking to your item's page on your website. |
| `product_weight` | String |  | The weight of the product in the units provided. The value must be between 0 (exclusive) and 2000 (inclusive). |
| `product_details` | Vec<String> |  | Technical specification or additional product details. |
| `unit_pricing_base_measure` | String |  | The preference of the denominator of the unit price. |
| `unit_pricing_measure` | String |  | The measure and dimension of an item. |
| `content_language` | String |  | Required. The two-letter ISO 639-1 language code for the item. |
| `display_ads_similar_ids` | Vec<String> |  | Advertiser-specified recommendations. |
| `ads_redirect` | String |  | Allows advertisers to override the item URL when the product is shown within the context of Product Ads. |
| `maximum_retail_price` | String |  | Maximum retail price (MRP) of the item. Applicable to India only. |
| `structured_description` | String |  | Structured description, for algorithmically (AI)-generated descriptions. |
| `title` | String |  | Title of the item. |
| `external_seller_id` | String |  | Required for multi-seller accounts. Use this attribute if you're a marketplace uploading products for various sellers to your multi-seller account. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#product`" |
| `lifestyle_image_links` | Vec<String> |  | Additional URLs of lifestyle images of the item. Used to explicitly identify images that showcase your item in a real-world context. See the Help Center article for more information. |
| `included_destinations` | Vec<String> |  | The list of [destinations to include](//support.google.com/merchants/answer/7501026) for this target (corresponds to checked check boxes in Merchant Center). Default destinations are always included unless provided in `excludedDestinations`. |
| `multipack` | String |  | The number of identical products in a merchant-defined multipack. |
| `size_type` | String |  | The cut of the item. Recommended for apparel items. |
| `identifier_exists` | bool |  | False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada. |
| `max_handling_time` | String |  | Maximal product handling time (in business days). |
| `offer_id` | String |  | Required. A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details. *Note:* Content API methods that operate on products take the REST ID of the product, *not* this identifier. |
| `shipping_length` | String |  | Length of the item for shipping. |
| `pickup_method` | String |  | The pick up option for the item. Acceptable values are: - "`buy`" - "`reserve`" - "`ship to store`" - "`not supported`"  |
| `additional_size_type` | String |  | Additional cut of the item. Used together with size_type to represent combined size types for apparel items. |
| `custom_label0` | String |  | Custom label 0 for custom grouping of items in a Shopping campaign. |
| `display_ads_title` | String |  | Title of an item for dynamic remarketing campaigns. |
| `display_ads_value` | f64 |  | Offer margin for dynamic remarketing campaigns. |
| `product_height` | String |  | The height of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `taxes` | Vec<String> |  | Tax information. |
| `sale_price` | String |  | Advertised sale price of the item. |
| `energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `canonical_link` | String |  | URL for the canonical version of your item's landing page. |
| `link_template` | String |  | URL template for merchant hosted local storefront. |
| `condition` | String |  | Condition or state of the item. |
| `installment` | String |  | Number and amount of installments to pay for an item. |
| `structured_title` | String |  | Structured title, for algorithmically (AI)-generated titles. |
| `virtual_model_link` | String |  | URL of the 3D model of the item to provide more visuals. |
| `age_group` | String |  | Target age group of the item. |
| `cost_of_goods_sold` | String |  | Cost of goods sold. Used for gross profit reporting. |
| `custom_label1` | String |  | Custom label 1 for custom grouping of items in a Shopping campaign. |
| `gender` | String |  | Target gender of the item. |
| `custom_label3` | String |  | Custom label 3 for custom grouping of items in a Shopping campaign. |
| `image_link` | String |  | URL of an image of the item. |
| `adult` | bool |  | Should be set to true if the item is targeted towards adults. |
| `sale_price_effective_date` | String |  | Date range during which the item is on sale (see product data specification ). |
| `availability` | String |  | Availability status of the item. |
| `sell_on_google_quantity` | String |  | The quantity of the product that is available for selling on Google. Supported only for online products. |
| `mobile_link_template` | String |  | URL template for merchant hosted local storefront optimized for mobile devices. |
| `feed_label` | String |  | Feed label for the item. Either `targetCountry` or `feedLabel` is required. Must be less than or equal to 20 uppercase letters (A-Z), numbers (0-9), and dashes (-). |
| `sustainability_incentives` | Vec<String> |  | Optional. The list of sustainability incentive programs. |
| `gtin` | String |  | Global Trade Item Number (GTIN) of the item. |
| `tax_category` | String |  | The tax category of the product, used to configure detailed tax nexus in account-level tax settings. |
| `product_types` | Vec<String> |  | Categories of the item (formatted as in product data specification). |
| `mpn` | String |  | Manufacturer Part Number (MPN) of the item. |
| `item_group_id` | String |  | Shared identifier for all variants of the same product. |
| `shipping_weight` | String |  | Weight of the item for shipping. |
| `is_bundle` | bool |  | Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price. |
| `size_system` | String |  | System in which the size is specified. Recommended for apparel items. |
| `material` | String |  | The material of which the item is made. |
| `mobile_link` | String |  | URL for the mobile-optimized version of your item's landing page. |
| `display_ads_id` | String |  | An identifier for an item for dynamic remarketing campaigns. |
| `custom_attributes` | Vec<String> |  | A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google (formerly known as Shopping Actions). |
| `brand` | String |  | Brand of the item. |
| `channel` | String |  | Required. The item's channel (online or local). Acceptable values are: - "`local`" - "`online`"  |
| `min_energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `expiration_date` | String |  | Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future. |
| `loyalty_program` | String |  | Loyalty program information that is used to surface loyalty benefits ( for example, better pricing, points, etc) to the user of this item. This signular field points to the latest uploaded loyalty program info. This field will be deprecated in the coming weeks and should not be used in favor of the plural 'LoyaltyProgram' field below. |
| `pattern` | String |  | The item's pattern (for example, polka dots). |
| `product_width` | String |  | The width of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `product_highlights` | Vec<String> |  | Bullet points describing the most relevant highlights of a product. |
| `shipping_label` | String |  | The shipping label of the product, used to group product in account-level shipping rules. |
| `merchant_id` | String | ✅ | The ID of the account that contains the product. This account cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `availability_date` | String | The day a pre-ordered product becomes available for delivery, in ISO 8601 format. |
| `display_ads_link` | String | URL directly to your item's landing page for dynamic remarketing campaigns. |
| `pause` | String | Publication of this item should be temporarily paused. Acceptable values are: - "`ads`"  |
| `price` | String | Price of the item. |
| `disclosure_date` | String | The date time when an offer becomes visible in search results across Google’s YouTube surfaces, in [ISO 8601](http://en.wikipedia.org/wiki/ISO_8601) format. See [Disclosure date](https://support.google.com/merchants/answer/13034208) for more information. |
| `min_handling_time` | String | Minimal product handling time (in business days). |
| `promotion_ids` | Vec<String> | The unique ID of a promotion. |
| `pickup_sla` | String | Item store pickup timeline. Acceptable values are: - "`same day`" - "`next day`" - "`2-day`" - "`3-day`" - "`4-day`" - "`5-day`" - "`6-day`" - "`7-day`" - "`multi-week`"  |
| `shipping_height` | String | Height of the item for shipping. |
| `product_length` | String | The length of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `subscription_cost` | String | Number of periods (months or years) and amount of payment per period for an item with an associated subscription contract. |
| `source` | String | Output only. The source of the offer, that is, how the offer was created. Acceptable values are: - "`api`" - "`crawl`" - "`feed`"  |
| `max_energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `google_product_category` | String | Google's category of the item (see [Google product taxonomy](https://support.google.com/merchants/answer/1705911)). When querying products, this field will contain the user provided value. There is currently no way to get back the auto assigned google product categories through the API. |
| `custom_label2` | String | Custom label 2 for custom grouping of items in a Shopping campaign. |
| `shopping_ads_excluded_countries` | Vec<String> | List of country codes (ISO 3166-1 alpha-2) to exclude the offer from Shopping Ads destination. Countries from this list are removed from countries configured in MC feed settings. |
| `sizes` | Vec<String> | Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition). |
| `shipping` | Vec<String> | Shipping rules. |
| `description` | String | Description of the item. |
| `shipping_width` | String | Width of the item for shipping. |
| `target_country` | String | Required. The CLDR territory code for the item's country of sale. |
| `additional_image_links` | Vec<String> | Additional URLs of images of the item. |
| `transit_time_label` | String | The transit time label of the product, used to group product in account-level transit time tables. |
| `ads_labels` | Vec<String> | Similar to ads_grouping, but only works on CPC. |
| `custom_label4` | String | Custom label 4 for custom grouping of items in a Shopping campaign. |
| `cloud_export_additional_properties` | Vec<String> | Extra fields to export to the Cloud Retail program. |
| `loyalty_programs` | Vec<String> | Optional. A list of loyalty program information that is used to surface loyalty benefits (for example, better pricing, points, etc) to the user of this item. |
| `color` | String | Color of the item. |
| `certifications` | Vec<String> | Product [certification](https://support.google.com/merchants/answer/13528839), introduced for EU energy efficiency labeling compliance using the [EU EPREL](https://eprel.ec.europa.eu/screen/home) database. |
| `free_shipping_threshold` | Vec<String> | Optional. Conditions to be met for a product to have free shipping. |
| `ads_grouping` | String | Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise. |
| `auto_pricing_min_price` | String | A safeguard in the [Automated Discounts](//support.google.com/merchants/answer/10295759) and [Dynamic Promotions](//support.google.com/merchants/answer/13949249) projects, ensuring that discounts on merchants' offers do not fall below this value, thereby preserving the offer's value and profitability. |
| `excluded_destinations` | Vec<String> | The list of [destinations to exclude](//support.google.com/merchants/answer/6324486) for this target (corresponds to cleared check boxes in Merchant Center). Products that are excluded from all destinations for more than 7 days are automatically deleted. |
| `id` | String | The REST ID of the product. Content API methods that operate on products take this as their `productId` parameter. The REST ID for a product has one of the 2 forms channel:contentLanguage: targetCountry: offerId or channel:contentLanguage:feedLabel: offerId. |
| `link` | String | URL directly linking to your item's page on your website. |
| `product_weight` | String | The weight of the product in the units provided. The value must be between 0 (exclusive) and 2000 (inclusive). |
| `product_details` | Vec<String> | Technical specification or additional product details. |
| `unit_pricing_base_measure` | String | The preference of the denominator of the unit price. |
| `unit_pricing_measure` | String | The measure and dimension of an item. |
| `content_language` | String | Required. The two-letter ISO 639-1 language code for the item. |
| `display_ads_similar_ids` | Vec<String> | Advertiser-specified recommendations. |
| `ads_redirect` | String | Allows advertisers to override the item URL when the product is shown within the context of Product Ads. |
| `maximum_retail_price` | String | Maximum retail price (MRP) of the item. Applicable to India only. |
| `structured_description` | String | Structured description, for algorithmically (AI)-generated descriptions. |
| `title` | String | Title of the item. |
| `external_seller_id` | String | Required for multi-seller accounts. Use this attribute if you're a marketplace uploading products for various sellers to your multi-seller account. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#product`" |
| `lifestyle_image_links` | Vec<String> | Additional URLs of lifestyle images of the item. Used to explicitly identify images that showcase your item in a real-world context. See the Help Center article for more information. |
| `included_destinations` | Vec<String> | The list of [destinations to include](//support.google.com/merchants/answer/7501026) for this target (corresponds to checked check boxes in Merchant Center). Default destinations are always included unless provided in `excludedDestinations`. |
| `multipack` | String | The number of identical products in a merchant-defined multipack. |
| `size_type` | String | The cut of the item. Recommended for apparel items. |
| `identifier_exists` | bool | False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada. |
| `max_handling_time` | String | Maximal product handling time (in business days). |
| `offer_id` | String | Required. A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details. *Note:* Content API methods that operate on products take the REST ID of the product, *not* this identifier. |
| `shipping_length` | String | Length of the item for shipping. |
| `pickup_method` | String | The pick up option for the item. Acceptable values are: - "`buy`" - "`reserve`" - "`ship to store`" - "`not supported`"  |
| `additional_size_type` | String | Additional cut of the item. Used together with size_type to represent combined size types for apparel items. |
| `custom_label0` | String | Custom label 0 for custom grouping of items in a Shopping campaign. |
| `display_ads_title` | String | Title of an item for dynamic remarketing campaigns. |
| `display_ads_value` | f64 | Offer margin for dynamic remarketing campaigns. |
| `product_height` | String | The height of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `taxes` | Vec<String> | Tax information. |
| `sale_price` | String | Advertised sale price of the item. |
| `energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `canonical_link` | String | URL for the canonical version of your item's landing page. |
| `link_template` | String | URL template for merchant hosted local storefront. |
| `condition` | String | Condition or state of the item. |
| `installment` | String | Number and amount of installments to pay for an item. |
| `structured_title` | String | Structured title, for algorithmically (AI)-generated titles. |
| `virtual_model_link` | String | URL of the 3D model of the item to provide more visuals. |
| `age_group` | String | Target age group of the item. |
| `cost_of_goods_sold` | String | Cost of goods sold. Used for gross profit reporting. |
| `custom_label1` | String | Custom label 1 for custom grouping of items in a Shopping campaign. |
| `gender` | String | Target gender of the item. |
| `custom_label3` | String | Custom label 3 for custom grouping of items in a Shopping campaign. |
| `image_link` | String | URL of an image of the item. |
| `adult` | bool | Should be set to true if the item is targeted towards adults. |
| `sale_price_effective_date` | String | Date range during which the item is on sale (see product data specification ). |
| `availability` | String | Availability status of the item. |
| `sell_on_google_quantity` | String | The quantity of the product that is available for selling on Google. Supported only for online products. |
| `mobile_link_template` | String | URL template for merchant hosted local storefront optimized for mobile devices. |
| `feed_label` | String | Feed label for the item. Either `targetCountry` or `feedLabel` is required. Must be less than or equal to 20 uppercase letters (A-Z), numbers (0-9), and dashes (-). |
| `sustainability_incentives` | Vec<String> | Optional. The list of sustainability incentive programs. |
| `gtin` | String | Global Trade Item Number (GTIN) of the item. |
| `tax_category` | String | The tax category of the product, used to configure detailed tax nexus in account-level tax settings. |
| `product_types` | Vec<String> | Categories of the item (formatted as in product data specification). |
| `mpn` | String | Manufacturer Part Number (MPN) of the item. |
| `item_group_id` | String | Shared identifier for all variants of the same product. |
| `shipping_weight` | String | Weight of the item for shipping. |
| `is_bundle` | bool | Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price. |
| `size_system` | String | System in which the size is specified. Recommended for apparel items. |
| `material` | String | The material of which the item is made. |
| `mobile_link` | String | URL for the mobile-optimized version of your item's landing page. |
| `display_ads_id` | String | An identifier for an item for dynamic remarketing campaigns. |
| `custom_attributes` | Vec<String> | A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google (formerly known as Shopping Actions). |
| `brand` | String | Brand of the item. |
| `channel` | String | Required. The item's channel (online or local). Acceptable values are: - "`local`" - "`online`"  |
| `min_energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `expiration_date` | String | Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future. |
| `loyalty_program` | String | Loyalty program information that is used to surface loyalty benefits ( for example, better pricing, points, etc) to the user of this item. This signular field points to the latest uploaded loyalty program info. This field will be deprecated in the coming weeks and should not be used in favor of the plural 'LoyaltyProgram' field below. |
| `pattern` | String | The item's pattern (for example, polka dots). |
| `product_width` | String | The width of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `product_highlights` | Vec<String> | Bullet points describing the most relevant highlights of a product. |
| `shipping_label` | String | The shipping label of the product, used to group product in account-level shipping rules. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product
product = provider.content_api.Product {
    merchant_id = "value"  # The ID of the account that contains the product. This account cannot be a multi-client account.
}

# Access product outputs
product_id = product.id
product_availability_date = product.availability_date
product_display_ads_link = product.display_ads_link
product_pause = product.pause
product_price = product.price
product_disclosure_date = product.disclosure_date
product_min_handling_time = product.min_handling_time
product_promotion_ids = product.promotion_ids
product_pickup_sla = product.pickup_sla
product_shipping_height = product.shipping_height
product_product_length = product.product_length
product_subscription_cost = product.subscription_cost
product_source = product.source
product_max_energy_efficiency_class = product.max_energy_efficiency_class
product_google_product_category = product.google_product_category
product_custom_label2 = product.custom_label2
product_shopping_ads_excluded_countries = product.shopping_ads_excluded_countries
product_sizes = product.sizes
product_shipping = product.shipping
product_description = product.description
product_shipping_width = product.shipping_width
product_target_country = product.target_country
product_additional_image_links = product.additional_image_links
product_transit_time_label = product.transit_time_label
product_ads_labels = product.ads_labels
product_custom_label4 = product.custom_label4
product_cloud_export_additional_properties = product.cloud_export_additional_properties
product_loyalty_programs = product.loyalty_programs
product_color = product.color
product_certifications = product.certifications
product_free_shipping_threshold = product.free_shipping_threshold
product_ads_grouping = product.ads_grouping
product_auto_pricing_min_price = product.auto_pricing_min_price
product_excluded_destinations = product.excluded_destinations
product_id = product.id
product_link = product.link
product_product_weight = product.product_weight
product_product_details = product.product_details
product_unit_pricing_base_measure = product.unit_pricing_base_measure
product_unit_pricing_measure = product.unit_pricing_measure
product_content_language = product.content_language
product_display_ads_similar_ids = product.display_ads_similar_ids
product_ads_redirect = product.ads_redirect
product_maximum_retail_price = product.maximum_retail_price
product_structured_description = product.structured_description
product_title = product.title
product_external_seller_id = product.external_seller_id
product_kind = product.kind
product_lifestyle_image_links = product.lifestyle_image_links
product_included_destinations = product.included_destinations
product_multipack = product.multipack
product_size_type = product.size_type
product_identifier_exists = product.identifier_exists
product_max_handling_time = product.max_handling_time
product_offer_id = product.offer_id
product_shipping_length = product.shipping_length
product_pickup_method = product.pickup_method
product_additional_size_type = product.additional_size_type
product_custom_label0 = product.custom_label0
product_display_ads_title = product.display_ads_title
product_display_ads_value = product.display_ads_value
product_product_height = product.product_height
product_taxes = product.taxes
product_sale_price = product.sale_price
product_energy_efficiency_class = product.energy_efficiency_class
product_canonical_link = product.canonical_link
product_link_template = product.link_template
product_condition = product.condition
product_installment = product.installment
product_structured_title = product.structured_title
product_virtual_model_link = product.virtual_model_link
product_age_group = product.age_group
product_cost_of_goods_sold = product.cost_of_goods_sold
product_custom_label1 = product.custom_label1
product_gender = product.gender
product_custom_label3 = product.custom_label3
product_image_link = product.image_link
product_adult = product.adult
product_sale_price_effective_date = product.sale_price_effective_date
product_availability = product.availability
product_sell_on_google_quantity = product.sell_on_google_quantity
product_mobile_link_template = product.mobile_link_template
product_feed_label = product.feed_label
product_sustainability_incentives = product.sustainability_incentives
product_gtin = product.gtin
product_tax_category = product.tax_category
product_product_types = product.product_types
product_mpn = product.mpn
product_item_group_id = product.item_group_id
product_shipping_weight = product.shipping_weight
product_is_bundle = product.is_bundle
product_size_system = product.size_system
product_material = product.material
product_mobile_link = product.mobile_link
product_display_ads_id = product.display_ads_id
product_custom_attributes = product.custom_attributes
product_brand = product.brand
product_channel = product.channel
product_min_energy_efficiency_class = product.min_energy_efficiency_class
product_expiration_date = product.expiration_date
product_loyalty_program = product.loyalty_program
product_pattern = product.pattern
product_product_width = product.product_width
product_product_highlights = product.product_highlights
product_shipping_label = product.shipping_label
```

---


### Datafeed

Registers a datafeed configuration with your Merchant Center account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Required for update. The ID of the data feed. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#datafeed`" |
| `format` | String |  | Format of the feed file. |
| `targets` | Vec<String> |  | The targets this feed should apply to (country, language, destinations). |
| `fetch_schedule` | String |  | Fetch schedule for the feed file. |
| `attribute_language` | String |  | The two-letter ISO 639-1 language in which the attributes are defined in the data feed. |
| `content_type` | String |  | Required. The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported. Acceptable values are: - "`local products`" - "`product inventory`" - "`products`"  |
| `file_name` | String |  | Required. The filename of the feed. All feeds must have a unique file name. |
| `name` | String |  | Required for insert. A descriptive name of the data feed. |
| `merchant_id` | String | ✅ | The ID of the account that manages the datafeed. This account cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Required for update. The ID of the data feed. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#datafeed`" |
| `format` | String | Format of the feed file. |
| `targets` | Vec<String> | The targets this feed should apply to (country, language, destinations). |
| `fetch_schedule` | String | Fetch schedule for the feed file. |
| `attribute_language` | String | The two-letter ISO 639-1 language in which the attributes are defined in the data feed. |
| `content_type` | String | Required. The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported. Acceptable values are: - "`local products`" - "`product inventory`" - "`products`"  |
| `file_name` | String | Required. The filename of the feed. All feeds must have a unique file name. |
| `name` | String | Required for insert. A descriptive name of the data feed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create datafeed
datafeed = provider.content_api.Datafeed {
    merchant_id = "value"  # The ID of the account that manages the datafeed. This account cannot be a multi-client account.
}

# Access datafeed outputs
datafeed_id = datafeed.id
datafeed_id = datafeed.id
datafeed_kind = datafeed.kind
datafeed_format = datafeed.format
datafeed_targets = datafeed.targets
datafeed_fetch_schedule = datafeed.fetch_schedule
datafeed_attribute_language = datafeed.attribute_language
datafeed_content_type = datafeed.content_type
datafeed_file_name = datafeed.file_name
datafeed_name = datafeed.name
```

---


### Conversionsource

Creates a new conversion source.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. Current state of this conversion source. Can't be edited through the API. |
| `google_analytics_link` | String |  | Immutable. Conversion Source of type "Link to Google Analytics Property". |
| `conversion_source_id` | String |  | Output only. Generated by the Content API upon creation of a new `ConversionSource`. Format: [a-z]{4}:.+ The four characters before the colon represent the type of conversio source. Content after the colon represents the ID of the conversion source within that type. The ID of two different conversion sources might be the same across different types. The following type prefixes are supported: - galk: For GoogleAnalyticsLink sources. - mcdn: For MerchantCenterDestination sources. |
| `expire_time` | String |  | Output only. The time when an archived conversion source becomes permanently deleted and is no longer available to undelete. |
| `merchant_center_destination` | String |  | Conversion Source of type "Merchant Center Tag Destination". |
| `merchant_id` | String | ✅ | Required. The ID of the account that owns the new conversion source. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. Current state of this conversion source. Can't be edited through the API. |
| `google_analytics_link` | String | Immutable. Conversion Source of type "Link to Google Analytics Property". |
| `conversion_source_id` | String | Output only. Generated by the Content API upon creation of a new `ConversionSource`. Format: [a-z]{4}:.+ The four characters before the colon represent the type of conversio source. Content after the colon represents the ID of the conversion source within that type. The ID of two different conversion sources might be the same across different types. The following type prefixes are supported: - galk: For GoogleAnalyticsLink sources. - mcdn: For MerchantCenterDestination sources. |
| `expire_time` | String | Output only. The time when an archived conversion source becomes permanently deleted and is no longer available to undelete. |
| `merchant_center_destination` | String | Conversion Source of type "Merchant Center Tag Destination". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversionsource
conversionsource = provider.content_api.Conversionsource {
    merchant_id = "value"  # Required. The ID of the account that owns the new conversion source.
}

# Access conversionsource outputs
conversionsource_id = conversionsource.id
conversionsource_state = conversionsource.state
conversionsource_google_analytics_link = conversionsource.google_analytics_link
conversionsource_conversion_source_id = conversionsource.conversion_source_id
conversionsource_expire_time = conversionsource.expire_time
conversionsource_merchant_center_destination = conversionsource.merchant_center_destination
```

---


### Returncarrier

Links return carrier to a merchant account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `carrier_account_id` | String |  | Output only. Immutable. The Google-provided unique carrier ID, used to update the resource. |
| `carrier_account_name` | String |  | Name of the carrier account. |
| `carrier_code` | String |  | The carrier code enum. Accepts the values FEDEX or UPS. |
| `carrier_account_number` | String |  | Number of the carrier account. |
| `account_id` | String | ✅ | Required. The Merchant Center Account Id under which the Return Carrier is to be linked. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_return_carriers` | Vec<String> | List of all available account return carriers for the merchant. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create returncarrier
returncarrier = provider.content_api.Returncarrier {
    account_id = "value"  # Required. The Merchant Center Account Id under which the Return Carrier is to be linked.
}

# Access returncarrier outputs
returncarrier_id = returncarrier.id
returncarrier_account_return_carriers = returncarrier.account_return_carriers
```

---


### Accountstatuse

Retrieves multiple Merchant Center account statuses in a single request.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> |  | The request entries to be processed in the batch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_management` | String | How the account is managed. Acceptable values are: - "`manual`" - "`automatic`"  |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#accountStatus`" |
| `products` | Vec<String> | List of product-related data by channel, destination, and country. Data in this field may be delayed by up to 30 minutes. |
| `account_id` | String | The ID of the account for which the status is reported. |
| `website_claimed` | bool | Whether the account's website is claimed or not. |
| `account_level_issues` | Vec<String> | A list of account level issues. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create accountstatuse
accountstatuse = provider.content_api.Accountstatuse {
}

# Access accountstatuse outputs
accountstatuse_id = accountstatuse.id
accountstatuse_account_management = accountstatuse.account_management
accountstatuse_kind = accountstatuse.kind
accountstatuse_products = accountstatuse.products
accountstatuse_account_id = accountstatuse.account_id
accountstatuse_website_claimed = accountstatuse.website_claimed
accountstatuse_account_level_issues = accountstatuse.account_level_issues
```

---


### Csse

Updates labels that are assigned to a CSS domain by its CSS group.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `label_ids` | Vec<String> |  | The list of label IDs. |
| `css_domain_id` | String | ✅ | Required. The ID of the updated CSS domain. |
| `css_group_id` | String | ✅ | Required. The CSS group ID of the updated CSS domain. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `homepage_uri` | String | Output only. Immutable. The CSS domain's homepage. |
| `full_name` | String | Output only. Immutable. The CSS domain's full name. |
| `display_name` | String | Output only. Immutable. The CSS domain's display name, used when space is constrained. |
| `label_ids` | Vec<String> | A list of label IDs that are assigned to this CSS domain by its CSS group. Only populated for CSS group users. |
| `css_group_id` | String | Output only. Immutable. The ID of the CSS group this CSS domain is affiliated with. Only populated for CSS group users. |
| `css_domain_id` | String | Output only. Immutable. The CSS domain ID. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create csse
csse = provider.content_api.Csse {
    css_domain_id = "value"  # Required. The ID of the updated CSS domain.
    css_group_id = "value"  # Required. The CSS group ID of the updated CSS domain.
}

# Access csse outputs
csse_id = csse.id
csse_homepage_uri = csse.homepage_uri
csse_full_name = csse.full_name
csse_display_name = csse.display_name
csse_label_ids = csse.label_ids
csse_css_group_id = csse.css_group_id
csse_css_domain_id = csse.css_domain_id
```

---


### Region

Creates a region definition in your Merchant Center account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The display name of the region. |
| `regional_inventory_eligible` | bool |  | Output only. Indicates if the region is eligible to use in the Regional Inventory configuration. |
| `shipping_eligible` | bool |  | Output only. Indicates if the region is eligible to use in the Shipping Services configuration. |
| `merchant_id` | String |  | Output only. Immutable. Merchant that owns the region. |
| `geotarget_area` | String |  | A list of geotargets that defines the region area. |
| `postal_code_area` | String |  | A list of postal codes that defines the region area. |
| `region_id` | String |  | Output only. Immutable. The ID uniquely identifying each region. |
| `merchant_id` | String | ✅ | Required. The id of the merchant for which to create region definition. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The display name of the region. |
| `regional_inventory_eligible` | bool | Output only. Indicates if the region is eligible to use in the Regional Inventory configuration. |
| `shipping_eligible` | bool | Output only. Indicates if the region is eligible to use in the Shipping Services configuration. |
| `merchant_id` | String | Output only. Immutable. Merchant that owns the region. |
| `geotarget_area` | String | A list of geotargets that defines the region area. |
| `postal_code_area` | String | A list of postal codes that defines the region area. |
| `region_id` | String | Output only. Immutable. The ID uniquely identifying each region. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create region
region = provider.content_api.Region {
    merchant_id = "value"  # Required. The id of the merchant for which to create region definition.
}

# Access region outputs
region_id = region.id
region_display_name = region.display_name
region_regional_inventory_eligible = region.regional_inventory_eligible
region_shipping_eligible = region.shipping_eligible
region_merchant_id = region.merchant_id
region_geotarget_area = region.geotarget_area
region_postal_code_area = region.postal_code_area
region_region_id = region.region_id
```

---


### Productstatuse

Gets the statuses of multiple products in a single request.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> |  | The request entries to be processed in the batch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `google_expiration_date` | String | Date on which the item expires in Google Shopping, in ISO 8601 format. |
| `last_update_date` | String | Date on which the item has been last updated, in ISO 8601 format. |
| `product_id` | String | The ID of the product for which status is reported. |
| `title` | String | The title of the product. |
| `link` | String | The link to the product. |
| `item_level_issues` | Vec<String> | A list of all issues associated with the product. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#productStatus`" |
| `creation_date` | String | Date on which the item has been created, in ISO 8601 format. |
| `destination_statuses` | Vec<String> | The intended destinations for the product. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create productstatuse
productstatuse = provider.content_api.Productstatuse {
}

# Access productstatuse outputs
productstatuse_id = productstatuse.id
productstatuse_google_expiration_date = productstatuse.google_expiration_date
productstatuse_last_update_date = productstatuse.last_update_date
productstatuse_product_id = productstatuse.product_id
productstatuse_title = productstatuse.title
productstatuse_link = productstatuse.link
productstatuse_item_level_issues = productstatuse.item_level_issues
productstatuse_kind = productstatuse.kind
productstatuse_creation_date = productstatuse.creation_date
productstatuse_destination_statuses = productstatuse.destination_statuses
```

---


### Label

Creates a new label, not assigned to any account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description of this label. |
| `label_type` | String |  | Output only. The type of this label. |
| `account_id` | String |  | Immutable. The ID of account this label belongs to. |
| `label_id` | String |  | Output only. The ID of the label. |
| `name` | String |  | The display name of this label. |
| `account_id` | String | ✅ | Required. The id of the account this label belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_labels` | Vec<String> | The labels from the specified account. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create label
label = provider.content_api.Label {
    account_id = "value"  # Required. The id of the account this label belongs to.
}

# Access label outputs
label_id = label.id
label_account_labels = label.account_labels
label_next_page_token = label.next_page_token
```

---


### Returnpolicy

Inserts a return policy for the Merchant Center account.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | Required. The policy. |
| `non_free_return_reasons` | Vec<String> |  | Return reasons that will incur return fees. |
| `country` | String |  | Required. The country of sale where the return policy is applicable. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#returnPolicy`" |
| `label` | String |  | Required. The user-defined label of the return policy. For the default policy, use the label "default". |
| `name` | String |  | Required. The name of the policy as shown in Merchant Center. |
| `return_policy_id` | String |  | Return policy ID generated by Google. |
| `return_shipping_fee` | String |  | The return shipping fee that will apply to non free return reasons. |
| `seasonal_overrides` | Vec<String> |  | An optional list of seasonal overrides. |
| `merchant_id` | String | ✅ | The Merchant Center account to insert a return policy for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy` | String | Required. The policy. |
| `non_free_return_reasons` | Vec<String> | Return reasons that will incur return fees. |
| `country` | String | Required. The country of sale where the return policy is applicable. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#returnPolicy`" |
| `label` | String | Required. The user-defined label of the return policy. For the default policy, use the label "default". |
| `name` | String | Required. The name of the policy as shown in Merchant Center. |
| `return_policy_id` | String | Return policy ID generated by Google. |
| `return_shipping_fee` | String | The return shipping fee that will apply to non free return reasons. |
| `seasonal_overrides` | Vec<String> | An optional list of seasonal overrides. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create returnpolicy
returnpolicy = provider.content_api.Returnpolicy {
    merchant_id = "value"  # The Merchant Center account to insert a return policy for.
}

# Access returnpolicy outputs
returnpolicy_id = returnpolicy.id
returnpolicy_policy = returnpolicy.policy
returnpolicy_non_free_return_reasons = returnpolicy.non_free_return_reasons
returnpolicy_country = returnpolicy.country
returnpolicy_kind = returnpolicy.kind
returnpolicy_label = returnpolicy.label
returnpolicy_name = returnpolicy.name
returnpolicy_return_policy_id = returnpolicy.return_policy_id
returnpolicy_return_shipping_fee = returnpolicy.return_shipping_fee
returnpolicy_seasonal_overrides = returnpolicy.seasonal_overrides
```

---


### Datafeedstatuse

Gets multiple Merchant Center datafeed statuses in a single request.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> |  | The request entries to be processed in the batch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `datafeed_id` | String | The ID of the feed for which the status is reported. |
| `errors` | Vec<String> | The list of errors occurring in the feed. |
| `items_total` | String | The number of items in the feed that were processed. |
| `country` | String | The country for which the status is reported, represented as a CLDR territory code. |
| `processing_status` | String | The processing status of the feed. Acceptable values are: - "`"`failure`": The feed could not be processed or all items had errors.`" - "`in progress`": The feed is being processed. - "`none`": The feed has not yet been processed. For example, a feed that has never been uploaded will have this processing status. - "`success`": The feed was processed successfully, though some items might have had errors.  |
| `language` | String | The two-letter ISO 639-1 language for which the status is reported. |
| `last_upload_date` | String | The last date at which the feed was uploaded. |
| `warnings` | Vec<String> | The list of errors occurring in the feed. |
| `feed_label` | String | The feed label status is reported for. |
| `items_valid` | String | The number of items in the feed that were valid. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#datafeedStatus`" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create datafeedstatuse
datafeedstatuse = provider.content_api.Datafeedstatuse {
}

# Access datafeedstatuse outputs
datafeedstatuse_id = datafeedstatuse.id
datafeedstatuse_datafeed_id = datafeedstatuse.datafeed_id
datafeedstatuse_errors = datafeedstatuse.errors
datafeedstatuse_items_total = datafeedstatuse.items_total
datafeedstatuse_country = datafeedstatuse.country
datafeedstatuse_processing_status = datafeedstatuse.processing_status
datafeedstatuse_language = datafeedstatuse.language
datafeedstatuse_last_upload_date = datafeedstatuse.last_upload_date
datafeedstatuse_warnings = datafeedstatuse.warnings
datafeedstatuse_feed_label = datafeedstatuse.feed_label
datafeedstatuse_items_valid = datafeedstatuse.items_valid
datafeedstatuse_kind = datafeedstatuse.kind
```

---


### Report

Retrieves merchant performance metrics matching the search query and optionally segmented by selected dimensions.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query` | String |  | Required. Query that defines performance metrics to retrieve and dimensions according to which the metrics are to be segmented. For details on how to construct your query, see the [Query Language guide](https://developers.google.com/shopping-content/guides/reports/query-language/overview). |
| `page_token` | String |  | Token of the page to retrieve. If not specified, the first page of results is returned. In order to request the next page of results, the value obtained from `next_page_token` in the previous response should be used. |
| `page_size` | i64 |  | Number of ReportRows to retrieve in a single page. Defaults to 1000. Values above 5000 are coerced to 5000. |
| `merchant_id` | String | ✅ | Required. Id of the merchant making the call. Must be a standalone account or an MCA subaccount. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report
report = provider.content_api.Report {
    merchant_id = "value"  # Required. Id of the merchant making the call. Must be a standalone account or an MCA subaccount.
}

```

---


### Quota

Lists the daily call quota and usage per method for your Merchant Center account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `method_quotas` | Vec<String> | The current quota usage and limits per each method. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access quota outputs
quota_id = quota.id
quota_method_quotas = quota.method_quotas
quota_next_page_token = quota.next_page_token
```

---


### Account

Creates a Merchant Center sub-account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `business_identity` | String |  | The business identity attributes can be used to self-declare attributes that let customers know more about your business. |
| `business_information` | String |  | The business information of the account. |
| `google_my_business_link` | String |  | The Business Profile which is linked or in the process of being linked with the Merchant Center account. |
| `id` | String |  | Required. 64-bit Merchant Center account ID. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#account`". |
| `account_management` | String |  | Output only. How the account is managed. Acceptable values are: - "`manual`" - "`automatic`"  |
| `conversion_settings` | String |  | Settings for conversion tracking. |
| `seller_id` | String |  | Client-specific, locally-unique, internal ID for the child account. |
| `name` | String |  | Required. Display name for the account. |
| `css_id` | String |  | ID of CSS the account belongs to. |
| `adult_content` | bool |  | Indicates whether the merchant sells adult content. |
| `youtube_channel_links` | Vec<String> |  | Linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list. |
| `label_ids` | Vec<String> |  | Manually created label IDs that are assigned to the account by CSS. |
| `automatic_label_ids` | Vec<String> |  | Automatically created label IDs that are assigned to the account by CSS Center. |
| `users` | Vec<String> |  | Users with access to the account. Every account (except for subaccounts) must have at least one admin user. |
| `website_url` | String |  | The merchant's website. |
| `automatic_improvements` | String |  | The automatic improvements of the account can be used to automatically update items, improve images and shipping. Each section inside AutomaticImprovements is updated separately. |
| `ads_links` | Vec<String> |  | Linked Ads accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the Ads interface or through the Google Ads API. To delete an active link, or to cancel a link request, remove it from the list. |
| `merchant_id` | String | ✅ | The ID of the managing account. This must be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `business_identity` | String | The business identity attributes can be used to self-declare attributes that let customers know more about your business. |
| `business_information` | String | The business information of the account. |
| `google_my_business_link` | String | The Business Profile which is linked or in the process of being linked with the Merchant Center account. |
| `id` | String | Required. 64-bit Merchant Center account ID. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#account`". |
| `account_management` | String | Output only. How the account is managed. Acceptable values are: - "`manual`" - "`automatic`"  |
| `conversion_settings` | String | Settings for conversion tracking. |
| `seller_id` | String | Client-specific, locally-unique, internal ID for the child account. |
| `name` | String | Required. Display name for the account. |
| `css_id` | String | ID of CSS the account belongs to. |
| `adult_content` | bool | Indicates whether the merchant sells adult content. |
| `youtube_channel_links` | Vec<String> | Linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list. |
| `label_ids` | Vec<String> | Manually created label IDs that are assigned to the account by CSS. |
| `automatic_label_ids` | Vec<String> | Automatically created label IDs that are assigned to the account by CSS Center. |
| `users` | Vec<String> | Users with access to the account. Every account (except for subaccounts) must have at least one admin user. |
| `website_url` | String | The merchant's website. |
| `automatic_improvements` | String | The automatic improvements of the account can be used to automatically update items, improve images and shipping. Each section inside AutomaticImprovements is updated separately. |
| `ads_links` | Vec<String> | Linked Ads accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the Ads interface or through the Google Ads API. To delete an active link, or to cancel a link request, remove it from the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.content_api.Account {
    merchant_id = "value"  # The ID of the managing account. This must be a multi-client account.
}

# Access account outputs
account_id = account.id
account_business_identity = account.business_identity
account_business_information = account.business_information
account_google_my_business_link = account.google_my_business_link
account_id = account.id
account_kind = account.kind
account_account_management = account.account_management
account_conversion_settings = account.conversion_settings
account_seller_id = account.seller_id
account_name = account.name
account_css_id = account.css_id
account_adult_content = account.adult_content
account_youtube_channel_links = account.youtube_channel_links
account_label_ids = account.label_ids
account_automatic_label_ids = account.automatic_label_ids
account_users = account.users
account_website_url = account.website_url
account_automatic_improvements = account.automatic_improvements
account_ads_links = account.ads_links
```

---


### Liasetting

Sets the omnichannel experience for the specified country. Only supported for merchants whose POS data provider is trusted to enable the corresponding experience. For more context, see these help articles [about LFP](https://support.google.com/merchants/answer/7676652) and [how to get started](https://support.google.com/merchants/answer/7676578) with it.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | The ID of the account for which to retrieve accessible Business Profiles. |
| `merchant_id` | String | ✅ | The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | The ID of the account to which these LIA settings belong. Ignored upon update, always present in get request responses. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#liaSettings`" |
| `country_settings` | Vec<String> | The LIA settings for each country. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create liasetting
liasetting = provider.content_api.Liasetting {
    account_id = "value"  # The ID of the account for which to retrieve accessible Business Profiles.
    merchant_id = "value"  # The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
}

# Access liasetting outputs
liasetting_id = liasetting.id
liasetting_account_id = liasetting.account_id
liasetting_kind = liasetting.kind
liasetting_country_settings = liasetting.country_settings
```

---


### Shippingsetting

Retrieves and updates the shipping settings of multiple accounts in a single request.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> |  | The request entries to be processed in the batch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `postal_code_groups` | Vec<String> | A list of postal code groups that can be referred to in `services`. Optional. |
| `services` | Vec<String> | The target account's list of services. Optional. |
| `account_id` | String | The ID of the account to which these account shipping settings belong. Ignored upon update, always present in get request responses. |
| `warehouses` | Vec<String> | Optional. A list of warehouses which can be referred to in `services`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create shippingsetting
shippingsetting = provider.content_api.Shippingsetting {
}

# Access shippingsetting outputs
shippingsetting_id = shippingsetting.id
shippingsetting_postal_code_groups = shippingsetting.postal_code_groups
shippingsetting_services = shippingsetting.services
shippingsetting_account_id = shippingsetting.account_id
shippingsetting_warehouses = shippingsetting.warehouses
```

---


### Productdeliverytime

Creates or updates the delivery time of a product.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `area_delivery_times` | Vec<String> |  | Required. A set of associations between `DeliveryArea` and `DeliveryTime` entries. The total number of `areaDeliveryTimes` can be at most 100. |
| `product_id` | String |  | Required. The `id` of the product. |
| `merchant_id` | String | ✅ | The Google merchant ID of the account that contains the product. This account cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `area_delivery_times` | Vec<String> | Required. A set of associations between `DeliveryArea` and `DeliveryTime` entries. The total number of `areaDeliveryTimes` can be at most 100. |
| `product_id` | String | Required. The `id` of the product. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create productdeliverytime
productdeliverytime = provider.content_api.Productdeliverytime {
    merchant_id = "value"  # The Google merchant ID of the account that contains the product. This account cannot be a multi-client account.
}

# Access productdeliverytime outputs
productdeliverytime_id = productdeliverytime.id
productdeliverytime_area_delivery_times = productdeliverytime.area_delivery_times
productdeliverytime_product_id = productdeliverytime.product_id
```

---


### Merchantsupport

Start an action. The action can be requested by merchants in third-party application. Before merchants can request the action, the third-party application needs to show them action specific content and display a user input form. The action can be successfully started only once all `required` inputs are provided. If any `required` input is missing, or invalid value was provided, the service will return 400 error. Validation errors will contain Ids for all problematic field together with translated, human readable error messages that can be shown to the user.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action_context` | String |  | Required. The context from the selected action. The value is obtained from rendered issues and needs to be sent back to identify the action that is being triggered. |
| `action_input` | String |  | Required. Input provided by the merchant. |
| `merchant_id` | String | ✅ | Required. The ID of the merchant's account. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create merchantsupport
merchantsupport = provider.content_api.Merchantsupport {
    merchant_id = "value"  # Required. The ID of the merchant's account.
}

```

---


### Credential

Uploads credentials for the Merchant Center account. If credentials already exist for this Merchant Center account and purpose, this method updates them.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `purpose` | String |  | Indicates to Google how Google should use these OAuth tokens. |
| `expires_in` | String |  | The amount of time, in seconds, after which the access token is no longer valid. |
| `access_token` | String |  | An OAuth access token. |
| `account_id` | String | ✅ | Required. The merchant id of the account these credentials belong to. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create credential
credential = provider.content_api.Credential {
    account_id = "value"  # Required. The merchant id of the account these credentials belong to.
}

```

---


### Returnpolicyonline

Creates a new return policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the policy as shown in Merchant Center. |
| `countries` | Vec<String> |  | The countries of sale where the return policy is applicable. The values must be a valid 2 letter ISO 3166 code, e.g. "US". |
| `item_conditions` | Vec<String> |  | The item conditions that are accepted for returns. This is required to not be empty unless the type of return policy is noReturns. |
| `return_reason_category_info` | Vec<String> |  | The return reason category information. This required to not be empty unless the type of return policy is noReturns. |
| `label` | String |  | The unique user-defined label of the return policy. The same label cannot be used in different return policies for the same country. Policies with the label 'default' will apply to all products, unless a product specifies a return_policy_label attribute. |
| `restocking_fee` | String |  | The restocking fee that applies to all return reason categories. This would be treated as a free restocking fee if the value is not set. |
| `return_policy_id` | String |  | Output only. Return policy ID generated by Google. |
| `return_methods` | Vec<String> |  | The return methods of how customers can return an item. This value is required to not be empty unless the type of return policy is noReturns. |
| `policy` | String |  | The return policy. |
| `return_policy_uri` | String |  | The return policy uri. This can used by Google to do a sanity check for the policy. |
| `merchant_id` | String | ✅ | Required. The id of the merchant for which to retrieve the return policy online object. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the policy as shown in Merchant Center. |
| `countries` | Vec<String> | The countries of sale where the return policy is applicable. The values must be a valid 2 letter ISO 3166 code, e.g. "US". |
| `item_conditions` | Vec<String> | The item conditions that are accepted for returns. This is required to not be empty unless the type of return policy is noReturns. |
| `return_reason_category_info` | Vec<String> | The return reason category information. This required to not be empty unless the type of return policy is noReturns. |
| `label` | String | The unique user-defined label of the return policy. The same label cannot be used in different return policies for the same country. Policies with the label 'default' will apply to all products, unless a product specifies a return_policy_label attribute. |
| `restocking_fee` | String | The restocking fee that applies to all return reason categories. This would be treated as a free restocking fee if the value is not set. |
| `return_policy_id` | String | Output only. Return policy ID generated by Google. |
| `return_methods` | Vec<String> | The return methods of how customers can return an item. This value is required to not be empty unless the type of return policy is noReturns. |
| `policy` | String | The return policy. |
| `return_policy_uri` | String | The return policy uri. This can used by Google to do a sanity check for the policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create returnpolicyonline
returnpolicyonline = provider.content_api.Returnpolicyonline {
    merchant_id = "value"  # Required. The id of the merchant for which to retrieve the return policy online object.
}

# Access returnpolicyonline outputs
returnpolicyonline_id = returnpolicyonline.id
returnpolicyonline_name = returnpolicyonline.name
returnpolicyonline_countries = returnpolicyonline.countries
returnpolicyonline_item_conditions = returnpolicyonline.item_conditions
returnpolicyonline_return_reason_category_info = returnpolicyonline.return_reason_category_info
returnpolicyonline_label = returnpolicyonline.label
returnpolicyonline_restocking_fee = returnpolicyonline.restocking_fee
returnpolicyonline_return_policy_id = returnpolicyonline.return_policy_id
returnpolicyonline_return_methods = returnpolicyonline.return_methods
returnpolicyonline_policy = returnpolicyonline.policy
returnpolicyonline_return_policy_uri = returnpolicyonline.return_policy_uri
```

---


### Localinventory

Updates the local inventory of a product in your Merchant Center account.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#localInventory`" |
| `instore_product_location` | String |  | The in-store product location. |
| `availability` | String |  | The availability of the product. For accepted attribute values, see the local product inventory feed specification. |
| `sale_price_effective_date` | String |  | A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates may be specified as 'null' if undecided. |
| `price` | String |  | The price of the product. |
| `pickup_sla` | String |  | The expected date that an order will be ready for pickup relative to the order date. Must be submitted together with `pickupMethod`. For accepted attribute values, see the local product inventory feed specification. |
| `store_code` | String |  | Required. The store code of this local inventory resource. |
| `custom_attributes` | Vec<String> |  | A list of custom (merchant-provided) attributes. Can also be used to submit any attribute of the feed specification in its generic form, for example, `{ "name": "size type", "value": "regular" }`. |
| `quantity` | i64 |  | The quantity of the product. Must be nonnegative. |
| `sale_price` | String |  | The sale price of the product. Mandatory if `sale_price_effective_date` is defined. |
| `pickup_method` | String |  | The supported pickup method for this offer. Unless the value is "not supported", this field must be submitted together with `pickupSla`. For accepted attribute values, see the local product inventory feed specification. |
| `product_id` | String | ✅ | The REST ID of the product for which to update local inventory. |
| `merchant_id` | String | ✅ | The ID of the account that contains the product. This account cannot be a multi-client account. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create localinventory
localinventory = provider.content_api.Localinventory {
    product_id = "value"  # The REST ID of the product for which to update local inventory.
    merchant_id = "value"  # The ID of the account that contains the product. This account cannot be a multi-client account.
}

```

---


### Freelistingsprogram

Requests a review of free listings in a specific region. This method deprecated. Use the `MerchantSupportService` to view product and account issues and request a review.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `region_code` | String |  | The code [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) of the country for which review is to be requested. |
| `merchant_id` | String | ✅ | Required. The ID of the account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `region_statuses` | Vec<String> | Status of the program in each region. Regions with the same status and review eligibility are grouped together in `regionCodes`. |
| `global_state` | String | State of the program. `ENABLED` if there are offers for at least one region. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create freelistingsprogram
freelistingsprogram = provider.content_api.Freelistingsprogram {
    merchant_id = "value"  # Required. The ID of the account.
}

# Access freelistingsprogram outputs
freelistingsprogram_id = freelistingsprogram.id
freelistingsprogram_region_statuses = freelistingsprogram.region_statuses
freelistingsprogram_global_state = freelistingsprogram.global_state
```

---


### Checkoutsetting

Enrolls merchant in `Checkout` program.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uri_settings` | String |  | Required. The `UrlSettings` for the request. The presence of URL settings indicates `Checkout` enrollment. |
| `merchant_id` | String | ✅ | Required. The ID of the account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `effective_uri_settings` | String | The effective value of `url_settings` for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account. |
| `effective_review_state` | String | Output only. The effective value of review state for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account. |
| `enrollment_state` | String | Output only. Reflects the merchant enrollment state in `Checkout` feature. |
| `effective_enrollment_state` | String | Output only. The effective value of enrollment state for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account. |
| `merchant_id` | String | Required. The ID of the account. |
| `review_state` | String | Output only. Reflects the merchant review state in `Checkout` feature. This is set based on the data quality reviews of the URL provided by the merchant. A merchant with enrollment state as `ENROLLED` can be in the following review states: `IN_REVIEW`, `APPROVED` or `DISAPPROVED`. A merchant must be in an enrollment_state of `ENROLLED` before a review can begin for the merchant. |
| `uri_settings` | String | URL settings for cart or checkout URL. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create checkoutsetting
checkoutsetting = provider.content_api.Checkoutsetting {
    merchant_id = "value"  # Required. The ID of the account.
}

# Access checkoutsetting outputs
checkoutsetting_id = checkoutsetting.id
checkoutsetting_effective_uri_settings = checkoutsetting.effective_uri_settings
checkoutsetting_effective_review_state = checkoutsetting.effective_review_state
checkoutsetting_enrollment_state = checkoutsetting.enrollment_state
checkoutsetting_effective_enrollment_state = checkoutsetting.effective_enrollment_state
checkoutsetting_merchant_id = checkoutsetting.merchant_id
checkoutsetting_review_state = checkoutsetting.review_state
checkoutsetting_uri_settings = checkoutsetting.uri_settings
```

---


### Accounttax

Retrieves and updates tax settings of multiple accounts in a single request.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> |  | The request entries to be processed in the batch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#accountTax`". |
| `rules` | Vec<String> | Tax rules. Updating the tax rules will enable "US" taxes (not reversible). Defining no rules is equivalent to not charging tax at all. |
| `account_id` | String | Required. The ID of the account to which these account tax settings belong. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create accounttax
accounttax = provider.content_api.Accounttax {
}

# Access accounttax outputs
accounttax_id = accounttax.id
accounttax_kind = accounttax.kind
accounttax_rules = accounttax.rules
accounttax_account_id = accounttax.account_id
```

---


### Orderpayment

Notify about refund on user's selected payments method.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `invoice_id` | String |  | Deprecated. Please use invoiceIds instead. |
| `invoice_ids` | Vec<String> |  | Invoice IDs from the orderinvoices service that correspond to the refund. |
| `refund_state` | String |  | Whether refund was successful. |
| `order_id` | String | ✅ | The ID of the order for which charge is happening. |
| `merchant_id` | String | ✅ | The ID of the account that manages the order. This cannot be a multi-client account. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create orderpayment
orderpayment = provider.content_api.Orderpayment {
    order_id = "value"  # The ID of the order for which charge is happening.
    merchant_id = "value"  # The ID of the account that manages the order. This cannot be a multi-client account.
}

```

---


### Order

Marks line item(s) as shipped.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `carrier` | String |  | Deprecated. Please use shipmentInfo instead. The carrier handling the shipment. See shipments[].carrier in the  Orders resource representation for a list of acceptable values. |
| `operation_id` | String |  | The ID of the operation. Unique across all operations for a given order. |
| `shipment_infos` | Vec<String> |  | Shipment information. This field is repeated because a single line item can be shipped in several packages (and have several tracking IDs). |
| `tracking_id` | String |  | Deprecated. Please use shipmentInfo instead. The tracking id for the shipment. |
| `shipment_group_id` | String |  | ID of the shipment group. Required for orders that use the orderinvoices service. |
| `shipment_id` | String |  | Deprecated. Please use shipmentInfo instead. The ID of the shipment. |
| `line_items` | Vec<String> |  | Line items to ship. |
| `order_id` | String | ✅ | The ID of the order. |
| `merchant_id` | String | ✅ | The ID of the account that manages the order. This cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The REST id of the order. Globally unique. |
| `payment_status` | String | The status of the payment. |
| `promotions` | Vec<String> | Deprecated. The details of the merchant provided promotions applied to the order. More details about the program are here. |
| `placed_date` | String | The date when the order was placed, in ISO 8601 format. |
| `shipments` | Vec<String> | Shipments of the order. |
| `shipping_cost` | String | The total cost of shipping for all items. |
| `merchant_order_id` | String | Merchant-provided id of the order. |
| `payment_method` | String | The details of the payment method. |
| `line_items` | Vec<String> | Line items that are ordered. |
| `net_amount` | String | The net amount for the order. For example, if an order was originally for a grand total of $100 and a refund was issued for $20, the net amount will be $80. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "content#order". |
| `refunds` | Vec<String> | Refunds for the order. |
| `shipping_cost_tax` | String | The tax for the total shipping cost. |
| `shipping_option` | String | The requested shipping option. |
| `delivery_details` | String | The details for the delivery. |
| `status` | String | The status of the order. |
| `customer` | String | The details of the customer who placed the order. |
| `merchant_id` | String |  |
| `acknowledged` | bool | Whether the order was acknowledged. |
| `channel_type` | String | The channel type of the order: "purchaseOnGoogle" or "googleExpress". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create order
order = provider.content_api.Order {
    order_id = "value"  # The ID of the order.
    merchant_id = "value"  # The ID of the account that manages the order. This cannot be a multi-client account.
}

# Access order outputs
order_id = order.id
order_id = order.id
order_payment_status = order.payment_status
order_promotions = order.promotions
order_placed_date = order.placed_date
order_shipments = order.shipments
order_shipping_cost = order.shipping_cost
order_merchant_order_id = order.merchant_order_id
order_payment_method = order.payment_method
order_line_items = order.line_items
order_net_amount = order.net_amount
order_kind = order.kind
order_refunds = order.refunds
order_shipping_cost_tax = order.shipping_cost_tax
order_shipping_option = order.shipping_option
order_delivery_details = order.delivery_details
order_status = order.status
order_customer = order.customer
order_merchant_id = order.merchant_id
order_acknowledged = order.acknowledged
order_channel_type = order.channel_type
```

---


### Orderreturn

Retrieves an order return from your Merchant Center account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `merchant_order_id` | String |  |
| `return_shipments` | Vec<String> |  |
| `creation_date` | String |  |
| `order_return_id` | String |  |
| `return_items` | Vec<String> |  |
| `order_id` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access orderreturn outputs
orderreturn_id = orderreturn.id
orderreturn_merchant_order_id = orderreturn.merchant_order_id
orderreturn_return_shipments = orderreturn.return_shipments
orderreturn_creation_date = orderreturn.creation_date
orderreturn_order_return_id = orderreturn.order_return_id
orderreturn_return_items = orderreturn.return_items
orderreturn_order_id = orderreturn.order_id
```

---


### Orderinvoice

Creates a refund invoice for one or more shipment groups, and triggers a refund for non-facilitated payment orders. This can only be used for line items that have previously been charged using createChargeInvoice. All amounts (except for the summary) are incremental with respect to the previous invoice.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `refund_only_option` | String |  | Option to create a refund-only invoice. Exactly one of refundOnlyOption or returnOption must be provided. |
| `operation_id` | String |  | [required] The ID of the operation, unique across all operations for a given order. |
| `return_option` | String |  | Option to create an invoice for a refund and mark all items within the invoice as returned. Exactly one of refundOnlyOption or returnOption must be provided. |
| `shipment_invoices` | Vec<String> |  | Invoice details for different shipment groups. |
| `invoice_id` | String |  | [required] The ID of the invoice. |
| `merchant_id` | String | ✅ | The ID of the account that manages the order. This cannot be a multi-client account. |
| `order_id` | String | ✅ | The ID of the order. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create orderinvoice
orderinvoice = provider.content_api.Orderinvoice {
    merchant_id = "value"  # The ID of the account that manages the order. This cannot be a multi-client account.
    order_id = "value"  # The ID of the order.
}

```

---


### Shippingsetting

Retrieves and updates the shipping settings of multiple accounts in a single request.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> |  | The request entries to be processed in the batch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | The ID of the account to which these account shipping settings belong. Ignored upon update, always present in get request responses. |
| `postal_code_groups` | Vec<String> | A list of postal code groups that can be referred to in `services`. Optional. |
| `warehouses` | Vec<String> | Optional. A list of warehouses which can be referred to in `services`. |
| `services` | Vec<String> | The target account's list of services. Optional. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create shippingsetting
shippingsetting = provider.content_api.Shippingsetting {
}

# Access shippingsetting outputs
shippingsetting_id = shippingsetting.id
shippingsetting_account_id = shippingsetting.account_id
shippingsetting_postal_code_groups = shippingsetting.postal_code_groups
shippingsetting_warehouses = shippingsetting.warehouses
shippingsetting_services = shippingsetting.services
```

---


### Orderreport

Retrieves a list of transactions for a disbursement from your Merchant Center account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "content#orderreportsListTransactionsResponse". |
| `transactions` | Vec<String> | The list of transactions. |
| `next_page_token` | String | The token for the retrieval of the next page of transactions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access orderreport outputs
orderreport_id = orderreport.id
orderreport_kind = orderreport.kind
orderreport_transactions = orderreport.transactions
orderreport_next_page_token = orderreport.next_page_token
```

---


### Liasetting

Sets the inventory verification contract for the specified country.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | The ID of the account that manages the order. This cannot be a multi-client account. |
| `merchant_id` | String | ✅ | The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#liaSettings`" |
| `country_settings` | Vec<String> | The LIA settings for each country. |
| `account_id` | String | The ID of the account to which these LIA settings belong. Ignored upon update, always present in get request responses. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create liasetting
liasetting = provider.content_api.Liasetting {
    account_id = "value"  # The ID of the account that manages the order. This cannot be a multi-client account.
    merchant_id = "value"  # The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
}

# Access liasetting outputs
liasetting_id = liasetting.id
liasetting_kind = liasetting.kind
liasetting_country_settings = liasetting.country_settings
liasetting_account_id = liasetting.account_id
```

---


### Orderinvoice

Creates a refund invoice for one or more shipment groups, and triggers a refund for orderinvoice enabled orders. This can only be used for line items that have previously been charged using `createChargeInvoice`. All amounts (except for the summary) are incremental with respect to the previous invoice.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `return_option` | String |  | Option to create an invoice for a refund and mark all items within the invoice as returned. Exactly one of `refundOnlyOption` or `returnOption` must be provided. |
| `invoice_id` | String |  | [required] The ID of the invoice. |
| `refund_only_option` | String |  | Option to create a refund-only invoice. Exactly one of `refundOnlyOption` or `returnOption` must be provided. |
| `operation_id` | String |  | [required] The ID of the operation, unique across all operations for a given order. |
| `shipment_invoices` | Vec<String> |  | Invoice details for different shipment groups. |
| `order_id` | String | ✅ | The ID of the order. |
| `merchant_id` | String | ✅ | The ID of the account that manages the order. This cannot be a multi-client account. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create orderinvoice
orderinvoice = provider.content_api.Orderinvoice {
    order_id = "value"  # The ID of the order.
    merchant_id = "value"  # The ID of the account that manages the order. This cannot be a multi-client account.
}

```

---


### Datafeedstatuse

Gets multiple Merchant Center datafeed statuses in a single request.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> |  | The request entries to be processed in the batch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items_valid` | String | The number of items in the feed that were valid. |
| `language` | String | The two-letter ISO 639-1 language for which the status is reported. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#datafeedStatus`" |
| `errors` | Vec<String> | The list of errors occurring in the feed. |
| `items_total` | String | The number of items in the feed that were processed. |
| `datafeed_id` | String | The ID of the feed for which the status is reported. |
| `last_upload_date` | String | The last date at which the feed was uploaded. |
| `warnings` | Vec<String> | The list of errors occurring in the feed. |
| `country` | String | The country for which the status is reported, represented as a CLDR territory code. |
| `processing_status` | String | The processing status of the feed. Acceptable values are: - "`"`failure`": The feed could not be processed or all items had errors.`" - "`in progress`": The feed is being processed. - "`none`": The feed has not yet been processed. For example, a feed that has never been uploaded will have this processing status. - "`success`": The feed was processed successfully, though some items might have had errors.  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create datafeedstatuse
datafeedstatuse = provider.content_api.Datafeedstatuse {
}

# Access datafeedstatuse outputs
datafeedstatuse_id = datafeedstatuse.id
datafeedstatuse_items_valid = datafeedstatuse.items_valid
datafeedstatuse_language = datafeedstatuse.language
datafeedstatuse_kind = datafeedstatuse.kind
datafeedstatuse_errors = datafeedstatuse.errors
datafeedstatuse_items_total = datafeedstatuse.items_total
datafeedstatuse_datafeed_id = datafeedstatuse.datafeed_id
datafeedstatuse_last_upload_date = datafeedstatuse.last_upload_date
datafeedstatuse_warnings = datafeedstatuse.warnings
datafeedstatuse_country = datafeedstatuse.country
datafeedstatuse_processing_status = datafeedstatuse.processing_status
```

---


### Accountstatuse

Retrieves multiple Merchant Center account statuses in a single request.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> |  | The request entries to be processed in the batch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_quality_issues` | Vec<String> | DEPRECATED - never populated. |
| `account_id` | String | The ID of the account for which the status is reported. |
| `account_level_issues` | Vec<String> | A list of account level issues. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#accountStatus`" |
| `products` | Vec<String> | List of product-related data by channel, destination, and country. Data in this field may be delayed by up to 30 minutes. |
| `website_claimed` | bool | Whether the account's website is claimed or not. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create accountstatuse
accountstatuse = provider.content_api.Accountstatuse {
}

# Access accountstatuse outputs
accountstatuse_id = accountstatuse.id
accountstatuse_data_quality_issues = accountstatuse.data_quality_issues
accountstatuse_account_id = accountstatuse.account_id
accountstatuse_account_level_issues = accountstatuse.account_level_issues
accountstatuse_kind = accountstatuse.kind
accountstatuse_products = accountstatuse.products
accountstatuse_website_claimed = accountstatuse.website_claimed
```

---


### Datafeed

Registers a datafeed configuration with your Merchant Center account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `intended_destinations` | Vec<String> |  | [DEPRECATED] Please use targets[].includedDestinations instead. The list of intended destinations (corresponds to checked check boxes in Merchant Center). |
| `content_language` | String |  | [DEPRECATED] Please use targets[].language instead. The two-letter ISO 639-1 language of the items in the feed. Must be a valid language for `targetCountry`. |
| `target_country` | String |  | [DEPRECATED] Please use targets[].country instead. The country where the items in the feed will be included in the search index, represented as a CLDR territory code. |
| `format` | String |  | Format of the feed file. |
| `fetch_schedule` | String |  | Fetch schedule for the feed file. |
| `name` | String |  | Required for insert. A descriptive name of the data feed. |
| `content_type` | String |  | Required. The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported. Acceptable values are: - "`local products`" - "`product inventory`" - "`products`"  |
| `attribute_language` | String |  | The two-letter ISO 639-1 language in which the attributes are defined in the data feed. |
| `file_name` | String |  | Required. The filename of the feed. All feeds must have a unique file name. |
| `id` | String |  | Required for update. The ID of the data feed. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#datafeed`" |
| `targets` | Vec<String> |  | The targets this feed should apply to (country, language, destinations). |
| `merchant_id` | String | ✅ | The ID of the account that manages the datafeed. This account cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `intended_destinations` | Vec<String> | [DEPRECATED] Please use targets[].includedDestinations instead. The list of intended destinations (corresponds to checked check boxes in Merchant Center). |
| `content_language` | String | [DEPRECATED] Please use targets[].language instead. The two-letter ISO 639-1 language of the items in the feed. Must be a valid language for `targetCountry`. |
| `target_country` | String | [DEPRECATED] Please use targets[].country instead. The country where the items in the feed will be included in the search index, represented as a CLDR territory code. |
| `format` | String | Format of the feed file. |
| `fetch_schedule` | String | Fetch schedule for the feed file. |
| `name` | String | Required for insert. A descriptive name of the data feed. |
| `content_type` | String | Required. The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported. Acceptable values are: - "`local products`" - "`product inventory`" - "`products`"  |
| `attribute_language` | String | The two-letter ISO 639-1 language in which the attributes are defined in the data feed. |
| `file_name` | String | Required. The filename of the feed. All feeds must have a unique file name. |
| `id` | String | Required for update. The ID of the data feed. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#datafeed`" |
| `targets` | Vec<String> | The targets this feed should apply to (country, language, destinations). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create datafeed
datafeed = provider.content_api.Datafeed {
    merchant_id = "value"  # The ID of the account that manages the datafeed. This account cannot be a multi-client account.
}

# Access datafeed outputs
datafeed_id = datafeed.id
datafeed_intended_destinations = datafeed.intended_destinations
datafeed_content_language = datafeed.content_language
datafeed_target_country = datafeed.target_country
datafeed_format = datafeed.format
datafeed_fetch_schedule = datafeed.fetch_schedule
datafeed_name = datafeed.name
datafeed_content_type = datafeed.content_type
datafeed_attribute_language = datafeed.attribute_language
datafeed_file_name = datafeed.file_name
datafeed_id = datafeed.id
datafeed_kind = datafeed.kind
datafeed_targets = datafeed.targets
```

---


### Accounttax

Retrieves and updates tax settings of multiple accounts in a single request.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> |  | The request entries to be processed in the batch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | Required. The ID of the account to which these account tax settings belong. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "content#accountTax". |
| `rules` | Vec<String> | Tax rules. Updating the tax rules will enable US taxes (not reversible). Defining no rules is equivalent to not charging tax at all. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create accounttax
accounttax = provider.content_api.Accounttax {
}

# Access accounttax outputs
accounttax_id = accounttax.id
accounttax_account_id = accounttax.account_id
accounttax_kind = accounttax.kind
accounttax_rules = accounttax.rules
```

---


### Po

Creates a store for the given merchant.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#posStore`" |
| `store_address` | String |  | Required. The street address of the store. |
| `gcid_category` | Vec<String> |  | The business type of the store. |
| `store_code` | String |  | Required. A store identifier that is unique for the given merchant. |
| `website_url` | String |  | The website url for the store or merchant. |
| `phone_number` | String |  | The store phone number. |
| `store_name` | String |  | The merchant or store name. |
| `place_id` | String |  | The Google Place Id of the store location. |
| `target_merchant_id` | String | ✅ | The ID of the target merchant. |
| `merchant_id` | String | ✅ | The ID of the POS or inventory data provider. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#posStore`" |
| `store_address` | String | Required. The street address of the store. |
| `gcid_category` | Vec<String> | The business type of the store. |
| `store_code` | String | Required. A store identifier that is unique for the given merchant. |
| `website_url` | String | The website url for the store or merchant. |
| `phone_number` | String | The store phone number. |
| `store_name` | String | The merchant or store name. |
| `place_id` | String | The Google Place Id of the store location. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create po
po = provider.content_api.Po {
    target_merchant_id = "value"  # The ID of the target merchant.
    merchant_id = "value"  # The ID of the POS or inventory data provider.
}

# Access po outputs
po_id = po.id
po_kind = po.kind
po_store_address = po.store_address
po_gcid_category = po.gcid_category
po_store_code = po.store_code
po_website_url = po.website_url
po_phone_number = po.phone_number
po_store_name = po.store_name
po_place_id = po.place_id
```

---


### Order

Updates a shipment's status, carrier, and/or tracking ID.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `shipment_id` | String |  | The ID of the shipment. |
| `carrier` | String |  | The carrier handling the shipment. Not updated if missing. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values. |
| `status` | String |  | New status for the shipment. Not updated if missing. Acceptable values are: - "`delivered`" - "`undeliverable`" - "`readyForPickup`"  |
| `delivery_date` | String |  | Date on which the shipment has been delivered, in ISO 8601 format. Optional and can be provided only if `status` is `delivered`. |
| `tracking_id` | String |  | The tracking ID for the shipment. Not updated if missing. |
| `operation_id` | String |  | The ID of the operation. Unique across all operations for a given order. |
| `merchant_id` | String | ✅ | The ID of the account that manages the order. This cannot be a multi-client account. |
| `order_id` | String | ✅ | The ID of the order. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `acknowledged` | bool | Whether the order was acknowledged. |
| `refunds` | Vec<String> | Refunds for the order. |
| `shipping_cost` | String | The total cost of shipping for all items. |
| `delivery_details` | String | Delivery details for shipments of type `delivery`. |
| `merchant_order_id` | String | Merchant-provided ID of the order. |
| `promotions` | Vec<String> | The details of the merchant provided promotions applied to the order. To determine which promotions apply to which products, check the `Promotions[].Benefits[].OfferIds` field against the `LineItems[].Product.OfferId` field for each promotion. If a promotion is applied to more than 1 `offerId`, divide the discount value by the number of affected offers to determine how much discount to apply to each `offerId`. Examples: 1. To calculate the line item level discount for a single specific item: For each promotion, subtract the `Promotions[].Benefits[].Discount.value` amount from the `LineItems[].Price.value`. 2. To calculate the line item level discount for multiple quantity of a specific item: For each promotion, divide the `Promotions[].Benefits[].Discount.value` by the quantity of products and substract it from `LineItems[].Product.Price.value` for each quantity item. Only 1 promotion can be applied to an offerId in a given order. To refund an item which had a promotion applied to it, make sure to refund the amount after first subtracting the promotion discount from the item price. More details about the program are here. |
| `merchant_id` | String |  |
| `net_amount` | String | The net amount for the order. For example, if an order was originally for a grand total of $100 and a refund was issued for $20, the net amount will be $80. |
| `payment_method` | String | The details of the payment method. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#order`" |
| `line_items` | Vec<String> | Line items that are ordered. |
| `customer` | String | The details of the customer who placed the order. |
| `pickup_details` | String | Pickup details for shipments of type `pickup`. |
| `shipments` | Vec<String> | Shipments of the order. |
| `id` | String | The REST ID of the order. Globally unique. |
| `channel_type` | String | Deprecated. Acceptable values are: - "`googleExpress`" - "`purchasesOnGoogle`"  |
| `shipping_option` | String | Deprecated. Shipping details are provided with line items instead. Acceptable values are: - "`economy`" - "`expedited`" - "`oneDay`" - "`sameDay`" - "`standard`" - "`twoDay`"  |
| `tax_collector` | String | The party responsible for collecting and remitting taxes. Acceptable values are: - "`marketplaceFacilitator`" - "`merchant`"  |
| `payment_status` | String | The status of the payment. Acceptable values are: - "`paymentCaptured`" - "`paymentRejected`" - "`paymentSecured`" - "`pendingAuthorization`"  |
| `shipping_cost_tax` | String | The tax for the total shipping cost. |
| `placed_date` | String | The date when the order was placed, in ISO 8601 format. |
| `status` | String | The status of the order. Acceptable values are: - "`canceled`" - "`delivered`" - "`inProgress`" - "`partiallyDelivered`" - "`partiallyReturned`" - "`partiallyShipped`" - "`pendingShipment`" - "`returned`" - "`shipped`"  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create order
order = provider.content_api.Order {
    merchant_id = "value"  # The ID of the account that manages the order. This cannot be a multi-client account.
    order_id = "value"  # The ID of the order.
}

# Access order outputs
order_id = order.id
order_acknowledged = order.acknowledged
order_refunds = order.refunds
order_shipping_cost = order.shipping_cost
order_delivery_details = order.delivery_details
order_merchant_order_id = order.merchant_order_id
order_promotions = order.promotions
order_merchant_id = order.merchant_id
order_net_amount = order.net_amount
order_payment_method = order.payment_method
order_kind = order.kind
order_line_items = order.line_items
order_customer = order.customer
order_pickup_details = order.pickup_details
order_shipments = order.shipments
order_id = order.id
order_channel_type = order.channel_type
order_shipping_option = order.shipping_option
order_tax_collector = order.tax_collector
order_payment_status = order.payment_status
order_shipping_cost_tax = order.shipping_cost_tax
order_placed_date = order.placed_date
order_status = order.status
```

---


### Orderreturn

Retrieves an order return from your Merchant Center account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_date` | String | The date of creation of the return, in ISO 8601 format. |
| `order_id` | String | Google order ID. |
| `return_items` | Vec<String> | Items of the return. |
| `merchant_order_id` | String | Merchant defined order ID. |
| `return_shipments` | Vec<String> | Shipments of the return. |
| `order_return_id` | String | Order return ID generated by Google. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access orderreturn outputs
orderreturn_id = orderreturn.id
orderreturn_creation_date = orderreturn.creation_date
orderreturn_order_id = orderreturn.order_id
orderreturn_return_items = orderreturn.return_items
orderreturn_merchant_order_id = orderreturn.merchant_order_id
orderreturn_return_shipments = orderreturn.return_shipments
orderreturn_order_return_id = orderreturn.order_return_id
```

---


### Account

Creates a Merchant Center sub-account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reviews_url` | String |  | [DEPRECATED] This field is never returned and will be ignored if provided. |
| `id` | String |  | Required for update. Merchant Center account ID. |
| `google_my_business_link` | String |  | The GMB account which is linked or in the process of being linked with the Merchant Center account. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#account`" |
| `name` | String |  | Required. Display name for the account. |
| `seller_id` | String |  | Client-specific, locally-unique, internal ID for the child account. |
| `business_information` | String |  | The business information of the account. |
| `users` | Vec<String> |  | Users with access to the account. Every account (except for subaccounts) must have at least one admin user. |
| `adult_content` | bool |  | Indicates whether the merchant sells adult content. |
| `adwords_links` | Vec<String> |  | List of linked AdWords accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the AdWords interface or through the AdWords API. To delete an active link, or to cancel a link request, remove it from the list. |
| `website_url` | String |  | The merchant's website. |
| `youtube_channel_links` | Vec<String> |  | List of linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list. |
| `merchant_id` | String | ✅ | The ID of the managing account. This must be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reviews_url` | String | [DEPRECATED] This field is never returned and will be ignored if provided. |
| `id` | String | Required for update. Merchant Center account ID. |
| `google_my_business_link` | String | The GMB account which is linked or in the process of being linked with the Merchant Center account. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#account`" |
| `name` | String | Required. Display name for the account. |
| `seller_id` | String | Client-specific, locally-unique, internal ID for the child account. |
| `business_information` | String | The business information of the account. |
| `users` | Vec<String> | Users with access to the account. Every account (except for subaccounts) must have at least one admin user. |
| `adult_content` | bool | Indicates whether the merchant sells adult content. |
| `adwords_links` | Vec<String> | List of linked AdWords accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the AdWords interface or through the AdWords API. To delete an active link, or to cancel a link request, remove it from the list. |
| `website_url` | String | The merchant's website. |
| `youtube_channel_links` | Vec<String> | List of linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.content_api.Account {
    merchant_id = "value"  # The ID of the managing account. This must be a multi-client account.
}

# Access account outputs
account_id = account.id
account_reviews_url = account.reviews_url
account_id = account.id
account_google_my_business_link = account.google_my_business_link
account_kind = account.kind
account_name = account.name
account_seller_id = account.seller_id
account_business_information = account.business_information
account_users = account.users
account_adult_content = account.adult_content
account_adwords_links = account.adwords_links
account_website_url = account.website_url
account_youtube_channel_links = account.youtube_channel_links
```

---


### Productstatuse

Gets the statuses of multiple products in a single request.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entries` | Vec<String> |  | The request entries to be processed in the batch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_quality_issues` | Vec<String> | DEPRECATED - never populated |
| `item_level_issues` | Vec<String> | A list of all issues associated with the product. |
| `title` | String | The title of the product. |
| `google_expiration_date` | String | Date on which the item expires in Google Shopping, in ISO 8601 format. |
| `last_update_date` | String | Date on which the item has been last updated, in ISO 8601 format. |
| `creation_date` | String | Date on which the item has been created, in ISO 8601 format. |
| `link` | String | The link to the product. |
| `product` | String | Product data after applying all the join inputs. |
| `product_id` | String | The ID of the product for which status is reported. |
| `destination_statuses` | Vec<String> | The intended destinations for the product. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#productStatus`" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create productstatuse
productstatuse = provider.content_api.Productstatuse {
}

# Access productstatuse outputs
productstatuse_id = productstatuse.id
productstatuse_data_quality_issues = productstatuse.data_quality_issues
productstatuse_item_level_issues = productstatuse.item_level_issues
productstatuse_title = productstatuse.title
productstatuse_google_expiration_date = productstatuse.google_expiration_date
productstatuse_last_update_date = productstatuse.last_update_date
productstatuse_creation_date = productstatuse.creation_date
productstatuse_link = productstatuse.link
productstatuse_product = productstatuse.product
productstatuse_product_id = productstatuse.product_id
productstatuse_destination_statuses = productstatuse.destination_statuses
productstatuse_kind = productstatuse.kind
```

---


### Product

Uploads a product to your Merchant Center account. If an item with the same channel, contentLanguage, offerId, and targetCountry already exists, this method updates that entry.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `online_only` | bool |  | Deprecated. |
| `custom_attributes` | Vec<String> |  | A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (e.g., `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google (formerly known as Shopping Actions). |
| `shipping_width` | String |  | Width of the item for shipping. |
| `unit_pricing_base_measure` | String |  | The preference of the denominator of the unit price. |
| `aspects` | Vec<String> |  | Deprecated. Do not use. |
| `sizes` | Vec<String> |  | Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition). |
| `canonical_link` | String |  | URL for the canonical version of your item's landing page. |
| `loyalty_points` | String |  | Loyalty points that users receive after purchasing the item. Japan only. |
| `size_system` | String |  | System in which the size is specified. Recommended for apparel items. Acceptable values are: - "`AU`" - "`BR`" - "`CN`" - "`DE`" - "`EU`" - "`FR`" - "`IT`" - "`JP`" - "`MEX`" - "`UK`" - "`US`"  |
| `max_energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `custom_label4` | String |  | Custom label 4 for custom grouping of items in a Shopping campaign. |
| `size_type` | String |  | The cut of the item. Recommended for apparel items. Acceptable values are: - "`big and tall`" - "`maternity`" - "`oversize`" - "`petite`" - "`plus`" - "`regular`"  |
| `adult` | bool |  | Should be set to true if the item is targeted towards adults. |
| `availability` | String |  | Availability status of the item. Acceptable values are: - "`in stock`" - "`out of stock`" - "`preorder`"  |
| `shipping_height` | String |  | Height of the item for shipping. |
| `custom_groups` | Vec<String> |  | A list of custom (merchant-provided) custom attribute groups. |
| `sale_price_effective_date` | String |  | Date range during which the item is on sale (see products data specification ). |
| `shipping_weight` | String |  | Weight of the item for shipping. |
| `content_language` | String |  | Required. The two-letter ISO 639-1 language code for the item. |
| `custom_label0` | String |  | Custom label 0 for custom grouping of items in a Shopping campaign. |
| `gender` | String |  | Target gender of the item. Acceptable values are: - "`female`" - "`male`" - "`unisex`"  |
| `id` | String |  | The REST ID of the product. Content API methods that operate on products take this as their `productId` parameter. The REST ID for a product is of the form channel:contentLanguage: targetCountry: offerId. |
| `adwords_grouping` | String |  | Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise. |
| `condition` | String |  | Condition or state of the item. Acceptable values are: - "`new`" - "`refurbished`" - "`used`"  |
| `custom_label3` | String |  | Custom label 3 for custom grouping of items in a Shopping campaign. |
| `display_ads_title` | String |  | Title of an item for dynamic remarketing campaigns. |
| `is_bundle` | bool |  | Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price. |
| `display_ads_similar_ids` | Vec<String> |  | Advertiser-specified recommendations. |
| `unit_pricing_measure` | String |  | The measure and dimension of an item. |
| `cost_of_goods_sold` | String |  | Cost of goods sold. Used for gross profit reporting. |
| `display_ads_value` | f64 |  | Offer margin for dynamic remarketing campaigns. |
| `product_type` | String |  | Your category of the item (formatted as in products data specification). |
| `min_energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#product`" |
| `brand` | String |  | Brand of the item. |
| `installment` | String |  | Number and amount of installments to pay for an item. |
| `availability_date` | String |  | The day a pre-ordered product becomes available for delivery, in ISO 8601 format. |
| `google_product_category` | String |  | Google's category of the item (see [Google product taxonomy](https://support.google.com/merchants/answer/1705911)). When querying products, this field will contain the user provided value. There is currently no way to get back the auto assigned google product categories through the API. |
| `mpn` | String |  | Manufacturer Part Number (MPN) of the item. |
| `gtin` | String |  | Global Trade Item Number (GTIN) of the item. |
| `channel` | String |  | Required. The item's channel (online or local). Acceptable values are: - "`local`" - "`online`"  |
| `shipping_length` | String |  | Length of the item for shipping. |
| `multipack` | String |  | The number of identical products in a merchant-defined multipack. |
| `additional_image_links` | Vec<String> |  | Additional URLs of images of the item. |
| `sell_on_google_quantity` | String |  | The quantity of the product that is available for selling on Google. Supported only for online products. |
| `taxes` | Vec<String> |  | Tax information. |
| `adwords_labels` | Vec<String> |  | Similar to adwords_grouping, but only works on CPC. |
| `promotion_ids` | Vec<String> |  | The unique ID of a promotion. |
| `destinations` | Vec<String> |  | Specifies the intended destinations for the product. |
| `title` | String |  | Title of the item. |
| `shipping` | Vec<String> |  | Shipping rules. |
| `warnings` | Vec<String> |  | Read-only warnings. |
| `custom_label2` | String |  | Custom label 2 for custom grouping of items in a Shopping campaign. |
| `display_ads_id` | String |  | An identifier for an item for dynamic remarketing campaigns. |
| `image_link` | String |  | URL of an image of the item. |
| `additional_product_types` | Vec<String> |  | Additional categories of the item (formatted as in products data specification). |
| `min_handling_time` | String |  | Minimal product handling time (in business days). |
| `offer_id` | String |  | Required. A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details. *Note:* Content API methods that operate on products take the REST ID of the product, *not* this identifier. |
| `sale_price` | String |  | Advertised sale price of the item. |
| `shipping_label` | String |  | The shipping label of the product, used to group product in account-level shipping rules. |
| `target_country` | String |  | Required. The CLDR territory code for the item. |
| `expiration_date` | String |  | Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future. |
| `color` | String |  | Color of the item. |
| `age_group` | String |  | Target age group of the item. Acceptable values are: - "`adult`" - "`infant`" - "`kids`" - "`newborn`" - "`toddler`" - "`youngAdult`"  |
| `description` | String |  | Description of the item. |
| `material` | String |  | The material of which the item is made. |
| `adwords_redirect` | String |  | Allows advertisers to override the item URL when the product is shown within the context of Product Ads. |
| `price` | String |  | Price of the item. |
| `mobile_link` | String |  | URL for the mobile-optimized version of your item's landing page. |
| `display_ads_link` | String |  | URL directly to your item's landing page for dynamic remarketing campaigns. |
| `pattern` | String |  | The item's pattern (e.g. polka dots). |
| `energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `item_group_id` | String |  | Shared identifier for all variants of the same product. |
| `identifier_exists` | bool |  | False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada. |
| `source` | String |  | The source of the offer, i.e., how the offer was created. Acceptable values are: - "`api`" - "`crawl`" - "`feed`"  |
| `max_handling_time` | String |  | Maximal product handling time (in business days). |
| `custom_label1` | String |  | Custom label 1 for custom grouping of items in a Shopping campaign. |
| `validated_destinations` | Vec<String> |  | Deprecated. The read-only list of intended destinations which passed validation. |
| `link` | String |  | URL directly linking to your item's page on your website. |
| `merchant_id` | String | ✅ | The ID of the account that contains the product. This account cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `online_only` | bool | Deprecated. |
| `custom_attributes` | Vec<String> | A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (e.g., `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google (formerly known as Shopping Actions). |
| `shipping_width` | String | Width of the item for shipping. |
| `unit_pricing_base_measure` | String | The preference of the denominator of the unit price. |
| `aspects` | Vec<String> | Deprecated. Do not use. |
| `sizes` | Vec<String> | Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition). |
| `canonical_link` | String | URL for the canonical version of your item's landing page. |
| `loyalty_points` | String | Loyalty points that users receive after purchasing the item. Japan only. |
| `size_system` | String | System in which the size is specified. Recommended for apparel items. Acceptable values are: - "`AU`" - "`BR`" - "`CN`" - "`DE`" - "`EU`" - "`FR`" - "`IT`" - "`JP`" - "`MEX`" - "`UK`" - "`US`"  |
| `max_energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `custom_label4` | String | Custom label 4 for custom grouping of items in a Shopping campaign. |
| `size_type` | String | The cut of the item. Recommended for apparel items. Acceptable values are: - "`big and tall`" - "`maternity`" - "`oversize`" - "`petite`" - "`plus`" - "`regular`"  |
| `adult` | bool | Should be set to true if the item is targeted towards adults. |
| `availability` | String | Availability status of the item. Acceptable values are: - "`in stock`" - "`out of stock`" - "`preorder`"  |
| `shipping_height` | String | Height of the item for shipping. |
| `custom_groups` | Vec<String> | A list of custom (merchant-provided) custom attribute groups. |
| `sale_price_effective_date` | String | Date range during which the item is on sale (see products data specification ). |
| `shipping_weight` | String | Weight of the item for shipping. |
| `content_language` | String | Required. The two-letter ISO 639-1 language code for the item. |
| `custom_label0` | String | Custom label 0 for custom grouping of items in a Shopping campaign. |
| `gender` | String | Target gender of the item. Acceptable values are: - "`female`" - "`male`" - "`unisex`"  |
| `id` | String | The REST ID of the product. Content API methods that operate on products take this as their `productId` parameter. The REST ID for a product is of the form channel:contentLanguage: targetCountry: offerId. |
| `adwords_grouping` | String | Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise. |
| `condition` | String | Condition or state of the item. Acceptable values are: - "`new`" - "`refurbished`" - "`used`"  |
| `custom_label3` | String | Custom label 3 for custom grouping of items in a Shopping campaign. |
| `display_ads_title` | String | Title of an item for dynamic remarketing campaigns. |
| `is_bundle` | bool | Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price. |
| `display_ads_similar_ids` | Vec<String> | Advertiser-specified recommendations. |
| `unit_pricing_measure` | String | The measure and dimension of an item. |
| `cost_of_goods_sold` | String | Cost of goods sold. Used for gross profit reporting. |
| `display_ads_value` | f64 | Offer margin for dynamic remarketing campaigns. |
| `product_type` | String | Your category of the item (formatted as in products data specification). |
| `min_energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#product`" |
| `brand` | String | Brand of the item. |
| `installment` | String | Number and amount of installments to pay for an item. |
| `availability_date` | String | The day a pre-ordered product becomes available for delivery, in ISO 8601 format. |
| `google_product_category` | String | Google's category of the item (see [Google product taxonomy](https://support.google.com/merchants/answer/1705911)). When querying products, this field will contain the user provided value. There is currently no way to get back the auto assigned google product categories through the API. |
| `mpn` | String | Manufacturer Part Number (MPN) of the item. |
| `gtin` | String | Global Trade Item Number (GTIN) of the item. |
| `channel` | String | Required. The item's channel (online or local). Acceptable values are: - "`local`" - "`online`"  |
| `shipping_length` | String | Length of the item for shipping. |
| `multipack` | String | The number of identical products in a merchant-defined multipack. |
| `additional_image_links` | Vec<String> | Additional URLs of images of the item. |
| `sell_on_google_quantity` | String | The quantity of the product that is available for selling on Google. Supported only for online products. |
| `taxes` | Vec<String> | Tax information. |
| `adwords_labels` | Vec<String> | Similar to adwords_grouping, but only works on CPC. |
| `promotion_ids` | Vec<String> | The unique ID of a promotion. |
| `destinations` | Vec<String> | Specifies the intended destinations for the product. |
| `title` | String | Title of the item. |
| `shipping` | Vec<String> | Shipping rules. |
| `warnings` | Vec<String> | Read-only warnings. |
| `custom_label2` | String | Custom label 2 for custom grouping of items in a Shopping campaign. |
| `display_ads_id` | String | An identifier for an item for dynamic remarketing campaigns. |
| `image_link` | String | URL of an image of the item. |
| `additional_product_types` | Vec<String> | Additional categories of the item (formatted as in products data specification). |
| `min_handling_time` | String | Minimal product handling time (in business days). |
| `offer_id` | String | Required. A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details. *Note:* Content API methods that operate on products take the REST ID of the product, *not* this identifier. |
| `sale_price` | String | Advertised sale price of the item. |
| `shipping_label` | String | The shipping label of the product, used to group product in account-level shipping rules. |
| `target_country` | String | Required. The CLDR territory code for the item. |
| `expiration_date` | String | Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future. |
| `color` | String | Color of the item. |
| `age_group` | String | Target age group of the item. Acceptable values are: - "`adult`" - "`infant`" - "`kids`" - "`newborn`" - "`toddler`" - "`youngAdult`"  |
| `description` | String | Description of the item. |
| `material` | String | The material of which the item is made. |
| `adwords_redirect` | String | Allows advertisers to override the item URL when the product is shown within the context of Product Ads. |
| `price` | String | Price of the item. |
| `mobile_link` | String | URL for the mobile-optimized version of your item's landing page. |
| `display_ads_link` | String | URL directly to your item's landing page for dynamic remarketing campaigns. |
| `pattern` | String | The item's pattern (e.g. polka dots). |
| `energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `item_group_id` | String | Shared identifier for all variants of the same product. |
| `identifier_exists` | bool | False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada. |
| `source` | String | The source of the offer, i.e., how the offer was created. Acceptable values are: - "`api`" - "`crawl`" - "`feed`"  |
| `max_handling_time` | String | Maximal product handling time (in business days). |
| `custom_label1` | String | Custom label 1 for custom grouping of items in a Shopping campaign. |
| `validated_destinations` | Vec<String> | Deprecated. The read-only list of intended destinations which passed validation. |
| `link` | String | URL directly linking to your item's page on your website. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product
product = provider.content_api.Product {
    merchant_id = "value"  # The ID of the account that contains the product. This account cannot be a multi-client account.
}

# Access product outputs
product_id = product.id
product_online_only = product.online_only
product_custom_attributes = product.custom_attributes
product_shipping_width = product.shipping_width
product_unit_pricing_base_measure = product.unit_pricing_base_measure
product_aspects = product.aspects
product_sizes = product.sizes
product_canonical_link = product.canonical_link
product_loyalty_points = product.loyalty_points
product_size_system = product.size_system
product_max_energy_efficiency_class = product.max_energy_efficiency_class
product_custom_label4 = product.custom_label4
product_size_type = product.size_type
product_adult = product.adult
product_availability = product.availability
product_shipping_height = product.shipping_height
product_custom_groups = product.custom_groups
product_sale_price_effective_date = product.sale_price_effective_date
product_shipping_weight = product.shipping_weight
product_content_language = product.content_language
product_custom_label0 = product.custom_label0
product_gender = product.gender
product_id = product.id
product_adwords_grouping = product.adwords_grouping
product_condition = product.condition
product_custom_label3 = product.custom_label3
product_display_ads_title = product.display_ads_title
product_is_bundle = product.is_bundle
product_display_ads_similar_ids = product.display_ads_similar_ids
product_unit_pricing_measure = product.unit_pricing_measure
product_cost_of_goods_sold = product.cost_of_goods_sold
product_display_ads_value = product.display_ads_value
product_product_type = product.product_type
product_min_energy_efficiency_class = product.min_energy_efficiency_class
product_kind = product.kind
product_brand = product.brand
product_installment = product.installment
product_availability_date = product.availability_date
product_google_product_category = product.google_product_category
product_mpn = product.mpn
product_gtin = product.gtin
product_channel = product.channel
product_shipping_length = product.shipping_length
product_multipack = product.multipack
product_additional_image_links = product.additional_image_links
product_sell_on_google_quantity = product.sell_on_google_quantity
product_taxes = product.taxes
product_adwords_labels = product.adwords_labels
product_promotion_ids = product.promotion_ids
product_destinations = product.destinations
product_title = product.title
product_shipping = product.shipping
product_warnings = product.warnings
product_custom_label2 = product.custom_label2
product_display_ads_id = product.display_ads_id
product_image_link = product.image_link
product_additional_product_types = product.additional_product_types
product_min_handling_time = product.min_handling_time
product_offer_id = product.offer_id
product_sale_price = product.sale_price
product_shipping_label = product.shipping_label
product_target_country = product.target_country
product_expiration_date = product.expiration_date
product_color = product.color
product_age_group = product.age_group
product_description = product.description
product_material = product.material
product_adwords_redirect = product.adwords_redirect
product_price = product.price
product_mobile_link = product.mobile_link
product_display_ads_link = product.display_ads_link
product_pattern = product.pattern
product_energy_efficiency_class = product.energy_efficiency_class
product_item_group_id = product.item_group_id
product_identifier_exists = product.identifier_exists
product_source = product.source
product_max_handling_time = product.max_handling_time
product_custom_label1 = product.custom_label1
product_validated_destinations = product.validated_destinations
product_link = product.link
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple promotion resources
promotion_0 = provider.content_api.Promotion {
    merchant_id = "value-0"
}
promotion_1 = provider.content_api.Promotion {
    merchant_id = "value-1"
}
promotion_2 = provider.content_api.Promotion {
    merchant_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    promotion = provider.content_api.Promotion {
        merchant_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Content_api Documentation](https://cloud.google.com/content_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
