# Content_api Service



**Resources**: 52

---

## Overview

The content_api service provides access to 52 resource types:

- [Returncarrier](#returncarrier) [CRUD]
- [Ordertrackingsignal](#ordertrackingsignal) [C]
- [Collectionstatuse](#collectionstatuse) [R]
- [Freelistingsprogram](#freelistingsprogram) [CR]
- [Promotion](#promotion) [CR]
- [Credential](#credential) [C]
- [Merchantsupport](#merchantsupport) [C]
- [Label](#label) [CRUD]
- [Accountstatuse](#accountstatuse) [CR]
- [Accounttax](#accounttax) [CRU]
- [Datafeed](#datafeed) [CRUD]
- [Returnaddres](#returnaddres) [CRD]
- [Csse](#csse) [CR]
- [Shippingsetting](#shippingsetting) [CRU]
- [Returnpolicyonline](#returnpolicyonline) [CRUD]
- [Pubsubnotificationsetting](#pubsubnotificationsetting) [RU]
- [Liasetting](#liasetting) [CRU]
- [Product](#product) [CRUD]
- [Productdeliverytime](#productdeliverytime) [CRD]
- [Po](#po) [CRD]
- [Recommendation](#recommendation) [CR]
- [Returnpolicy](#returnpolicy) [CRD]
- [Checkoutsetting](#checkoutsetting) [CRD]
- [Productstatuse](#productstatuse) [CR]
- [Region](#region) [CRUD]
- [Collection](#collection) [CRD]
- [Datafeedstatuse](#datafeedstatuse) [CR]
- [Conversionsource](#conversionsource) [CRUD]
- [Quota](#quota) [R]
- [Regionalinventory](#regionalinventory) [C]
- [Account](#account) [CRUD]
- [Report](#report) [C]
- [Localinventory](#localinventory) [C]
- [Shoppingadsprogram](#shoppingadsprogram) [CR]
- [Orderpayment](#orderpayment) [C]
- [Order](#order) [CR]
- [Orderreturn](#orderreturn) [R]
- [Orderinvoice](#orderinvoice) [C]
- [Order](#order) [CR]
- [Datafeed](#datafeed) [CRUD]
- [Orderreport](#orderreport) [R]
- [Productstatuse](#productstatuse) [CR]
- [Datafeedstatuse](#datafeedstatuse) [CR]
- [Accountstatuse](#accountstatuse) [CR]
- [Product](#product) [CRD]
- [Accounttax](#accounttax) [CRU]
- [Liasetting](#liasetting) [CRU]
- [Account](#account) [CRUD]
- [Orderreturn](#orderreturn) [R]
- [Shippingsetting](#shippingsetting) [CRU]
- [Po](#po) [CRD]
- [Orderinvoice](#orderinvoice) [C]

---

## Resources


### Returncarrier

Links return carrier to a merchant account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `carrier_account_number` | String |  | Number of the carrier account. |
| `carrier_code` | String |  | The carrier code enum. Accepts the values FEDEX or UPS. |
| `carrier_account_name` | String |  | Name of the carrier account. |
| `carrier_account_id` | String |  | Output only. Immutable. The Google-provided unique carrier ID, used to update the resource. |
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


### Ordertrackingsignal

Creates new order tracking signal.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `line_items` | Vec<String> |  | Information about line items in the order. |
| `delivery_region_code` | String |  | Required. The [CLDR territory code] (http://www.unicode.org/repos/cldr/tags/latest/common/main/en.xml) for the shipping destination. |
| `customer_shipping_fee` | String |  | The shipping fee of the order; this value should be set to zero in the case of free shipping. |
| `shipment_line_item_mapping` | Vec<String> |  | The mapping of the line items to the shipment information. |
| `delivery_postal_code` | String |  | Required. The delivery postal code, as a continuous string without spaces or dashes, e.g. "95016". This field will be anonymized in returned OrderTrackingSignal creation response. |
| `order_created_time` | String |  | Required. The time when the order was created on the merchant side. Include the year and timezone string, if available. |
| `order_id` | String |  | Required. The ID of the order on the merchant side. This field will be hashed in returned OrderTrackingSignal creation response. |
| `shipping_info` | Vec<String> |  | The shipping information for the order. |
| `order_tracking_signal_id` | String |  | Output only. The ID that uniquely identifies this order tracking signal. |
| `merchant_id` | String |  | The Google merchant ID of this order tracking signal. This value is optional. If left unset, the caller's merchant ID is used. You must request access in order to provide data on behalf of another merchant. For more information, see [Submitting Order Tracking Signals](/shopping-content/guides/order-tracking-signals). |
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
| `destination_statuses` | Vec<String> | The intended destinations for the collection. |
| `last_update_date` | String | Date on which the collection has been last updated in [ISO 8601](http://en.wikipedia.org/wiki/ISO_8601) format: Date, time, and offset, for example "2020-01-02T09:00:00+01:00" or "2020-01-02T09:00:00Z" |
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
collectionstatuse_destination_statuses = collectionstatuse.destination_statuses
collectionstatuse_last_update_date = collectionstatuse.last_update_date
collectionstatuse_collection_level_issuses = collectionstatuse.collection_level_issuses
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


### Promotion

Inserts a promotion for your Merchant Center account. If the promotion already exists, then it updates the promotion instead. To [end or delete] (https://developers.google.com/shopping-content/guides/promotions#end_a_promotion) a promotion update the time period of the promotion to a time that has already passed.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `promotion_effective_time_period` | String |  | Required. `TimePeriod` representation of the promotion's effective dates. |
| `brand` | Vec<String> |  | Product filter by brand for the promotion. |
| `product_type` | Vec<String> |  | Product filter by product type for the promotion. |
| `store_code_exclusion` | Vec<String> |  | Store codes to exclude for the promotion. |
| `get_this_quantity_discounted` | i64 |  | The number of items discounted in the promotion. |
| `redemption_channel` | Vec<String> |  | Required. Redemption channel for the promotion. At least one channel is required. |
| `free_gift_value` | String |  | Free gift value for the promotion. |
| `redemption_restriction` | String |  | The redemption restriction for the promotion. |
| `product_type_exclusion` | Vec<String> |  | Product filter by product type exclusion for the promotion. |
| `limit_quantity` | i64 |  | Maximum purchase quantity for the promotion. |
| `store_code` | Vec<String> |  | Store codes to include for the promotion. |
| `id` | String |  | Output only. The REST promotion ID to uniquely identify the promotion. Content API methods that operate on promotions take this as their `promotionId` parameter. The REST ID for a promotion is of the form channel:contentLanguage:targetCountry:promotionId The `channel` field has a value of `"online"`, `"in_store"`, or `"online_in_store"`. |
| `promotion_destination_ids` | Vec<String> |  | Destination ID for the promotion. |
| `long_title` | String |  | Required. Long title for the promotion. |
| `content_language` | String |  | Required. The content language used as part of the unique identifier. `en` content language is available for all target countries. `fr` content language is available for `CA` and `FR` target countries. `de` content language is available for `DE` target country. `nl` content language is available for `NL` target country. `it` content language is available for `IT` target country. `pt` content language is available for `BR` target country. `ja` content language is available for `JP` target country. `ko` content language is available for `KR` target country. |
| `coupon_value_type` | String |  | Required. Coupon value type for the promotion. |
| `promotion_display_dates` | String |  | String representation of the promotion display dates. Deprecated. Use `promotion_display_time_period` instead. |
| `target_country` | String |  | Required. The target country used as part of the unique identifier. Can be `AU`, `CA`, `DE`, `FR`, `GB`, `IN`, `US`, `BR`, `ES`, `NL`, `JP`, `IT` or `KR`. |
| `order_limit` | i64 |  | Order limit for the promotion. |
| `money_off_amount` | String |  | The money off amount offered in the promotion. |
| `shipping_service_names` | Vec<String> |  | Shipping service names for the promotion. |
| `promotion_url` | String |  | URL to the page on the merchant's site where the promotion shows. Local Inventory ads promotions throw an error if no promo url is included. URL is used to confirm that the promotion is valid and can be redeemed. |
| `generic_redemption_code` | String |  | Generic redemption code for the promotion. To be used with the `offerType` field. |
| `free_gift_item_id` | String |  | Free gift item ID for the promotion. |
| `item_id_exclusion` | Vec<String> |  | Product filter by item ID exclusion for the promotion. |
| `max_discount_amount` | String |  | The maximum monetary discount a customer can receive for the promotion. This field is only supported with the `Percent off` coupon value type. |
| `item_group_id_exclusion` | Vec<String> |  | Product filter by item group ID exclusion for the promotion. |
| `minimum_purchase_amount` | String |  | Minimum purchase amount for the promotion. |
| `item_id` | Vec<String> |  | Product filter by item ID for the promotion. |
| `minimum_purchase_quantity` | i64 |  | Minimum purchase quantity for the promotion. |
| `offer_type` | String |  | Required. Type of the promotion. |
| `item_group_id` | Vec<String> |  | Product filter by item group ID for the promotion. |
| `percent_off` | i64 |  | The percentage discount offered in the promotion. |
| `promotion_display_time_period` | String |  | `TimePeriod` representation of the promotion's display dates. |
| `free_gift_description` | String |  | Free gift description for the promotion. |
| `product_applicability` | String |  | Required. Applicability of the promotion to either all products or only specific products. |
| `money_budget` | String |  | Cost cap for the promotion. |
| `promotion_status` | String |  | Output only. The current status of the promotion. |
| `brand_exclusion` | Vec<String> |  | Product filter by brand exclusion for the promotion. |
| `limit_value` | String |  | Maximum purchase value for the promotion. |
| `promotion_id` | String |  | Required. The user provided promotion ID to uniquely identify the promotion. |
| `custom_redemption_restriction` | String |  | The custom redemption restriction for the promotion. If the `redemption_restriction` field is set to `CUSTOM`, this field must be set. |
| `store_applicability` | String |  | Whether the promotion applies to all stores, or only specified stores. Local Inventory ads promotions throw an error if no store applicability is included. An INVALID_ARGUMENT error is thrown if store_applicability is set to ALL_STORES and store_code or score_code_exclusion is set to a value. |
| `promotion_effective_dates` | String |  | String representation of the promotion effective dates. Deprecated. Use `promotion_effective_time_period` instead. |
| `merchant_id` | String | ✅ | Required. The ID of the account that contains the collection. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `promotion_effective_time_period` | String | Required. `TimePeriod` representation of the promotion's effective dates. |
| `brand` | Vec<String> | Product filter by brand for the promotion. |
| `product_type` | Vec<String> | Product filter by product type for the promotion. |
| `store_code_exclusion` | Vec<String> | Store codes to exclude for the promotion. |
| `get_this_quantity_discounted` | i64 | The number of items discounted in the promotion. |
| `redemption_channel` | Vec<String> | Required. Redemption channel for the promotion. At least one channel is required. |
| `free_gift_value` | String | Free gift value for the promotion. |
| `redemption_restriction` | String | The redemption restriction for the promotion. |
| `product_type_exclusion` | Vec<String> | Product filter by product type exclusion for the promotion. |
| `limit_quantity` | i64 | Maximum purchase quantity for the promotion. |
| `store_code` | Vec<String> | Store codes to include for the promotion. |
| `id` | String | Output only. The REST promotion ID to uniquely identify the promotion. Content API methods that operate on promotions take this as their `promotionId` parameter. The REST ID for a promotion is of the form channel:contentLanguage:targetCountry:promotionId The `channel` field has a value of `"online"`, `"in_store"`, or `"online_in_store"`. |
| `promotion_destination_ids` | Vec<String> | Destination ID for the promotion. |
| `long_title` | String | Required. Long title for the promotion. |
| `content_language` | String | Required. The content language used as part of the unique identifier. `en` content language is available for all target countries. `fr` content language is available for `CA` and `FR` target countries. `de` content language is available for `DE` target country. `nl` content language is available for `NL` target country. `it` content language is available for `IT` target country. `pt` content language is available for `BR` target country. `ja` content language is available for `JP` target country. `ko` content language is available for `KR` target country. |
| `coupon_value_type` | String | Required. Coupon value type for the promotion. |
| `promotion_display_dates` | String | String representation of the promotion display dates. Deprecated. Use `promotion_display_time_period` instead. |
| `target_country` | String | Required. The target country used as part of the unique identifier. Can be `AU`, `CA`, `DE`, `FR`, `GB`, `IN`, `US`, `BR`, `ES`, `NL`, `JP`, `IT` or `KR`. |
| `order_limit` | i64 | Order limit for the promotion. |
| `money_off_amount` | String | The money off amount offered in the promotion. |
| `shipping_service_names` | Vec<String> | Shipping service names for the promotion. |
| `promotion_url` | String | URL to the page on the merchant's site where the promotion shows. Local Inventory ads promotions throw an error if no promo url is included. URL is used to confirm that the promotion is valid and can be redeemed. |
| `generic_redemption_code` | String | Generic redemption code for the promotion. To be used with the `offerType` field. |
| `free_gift_item_id` | String | Free gift item ID for the promotion. |
| `item_id_exclusion` | Vec<String> | Product filter by item ID exclusion for the promotion. |
| `max_discount_amount` | String | The maximum monetary discount a customer can receive for the promotion. This field is only supported with the `Percent off` coupon value type. |
| `item_group_id_exclusion` | Vec<String> | Product filter by item group ID exclusion for the promotion. |
| `minimum_purchase_amount` | String | Minimum purchase amount for the promotion. |
| `item_id` | Vec<String> | Product filter by item ID for the promotion. |
| `minimum_purchase_quantity` | i64 | Minimum purchase quantity for the promotion. |
| `offer_type` | String | Required. Type of the promotion. |
| `item_group_id` | Vec<String> | Product filter by item group ID for the promotion. |
| `percent_off` | i64 | The percentage discount offered in the promotion. |
| `promotion_display_time_period` | String | `TimePeriod` representation of the promotion's display dates. |
| `free_gift_description` | String | Free gift description for the promotion. |
| `product_applicability` | String | Required. Applicability of the promotion to either all products or only specific products. |
| `money_budget` | String | Cost cap for the promotion. |
| `promotion_status` | String | Output only. The current status of the promotion. |
| `brand_exclusion` | Vec<String> | Product filter by brand exclusion for the promotion. |
| `limit_value` | String | Maximum purchase value for the promotion. |
| `promotion_id` | String | Required. The user provided promotion ID to uniquely identify the promotion. |
| `custom_redemption_restriction` | String | The custom redemption restriction for the promotion. If the `redemption_restriction` field is set to `CUSTOM`, this field must be set. |
| `store_applicability` | String | Whether the promotion applies to all stores, or only specified stores. Local Inventory ads promotions throw an error if no store applicability is included. An INVALID_ARGUMENT error is thrown if store_applicability is set to ALL_STORES and store_code or score_code_exclusion is set to a value. |
| `promotion_effective_dates` | String | String representation of the promotion effective dates. Deprecated. Use `promotion_effective_time_period` instead. |


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
promotion_promotion_effective_time_period = promotion.promotion_effective_time_period
promotion_brand = promotion.brand
promotion_product_type = promotion.product_type
promotion_store_code_exclusion = promotion.store_code_exclusion
promotion_get_this_quantity_discounted = promotion.get_this_quantity_discounted
promotion_redemption_channel = promotion.redemption_channel
promotion_free_gift_value = promotion.free_gift_value
promotion_redemption_restriction = promotion.redemption_restriction
promotion_product_type_exclusion = promotion.product_type_exclusion
promotion_limit_quantity = promotion.limit_quantity
promotion_store_code = promotion.store_code
promotion_id = promotion.id
promotion_promotion_destination_ids = promotion.promotion_destination_ids
promotion_long_title = promotion.long_title
promotion_content_language = promotion.content_language
promotion_coupon_value_type = promotion.coupon_value_type
promotion_promotion_display_dates = promotion.promotion_display_dates
promotion_target_country = promotion.target_country
promotion_order_limit = promotion.order_limit
promotion_money_off_amount = promotion.money_off_amount
promotion_shipping_service_names = promotion.shipping_service_names
promotion_promotion_url = promotion.promotion_url
promotion_generic_redemption_code = promotion.generic_redemption_code
promotion_free_gift_item_id = promotion.free_gift_item_id
promotion_item_id_exclusion = promotion.item_id_exclusion
promotion_max_discount_amount = promotion.max_discount_amount
promotion_item_group_id_exclusion = promotion.item_group_id_exclusion
promotion_minimum_purchase_amount = promotion.minimum_purchase_amount
promotion_item_id = promotion.item_id
promotion_minimum_purchase_quantity = promotion.minimum_purchase_quantity
promotion_offer_type = promotion.offer_type
promotion_item_group_id = promotion.item_group_id
promotion_percent_off = promotion.percent_off
promotion_promotion_display_time_period = promotion.promotion_display_time_period
promotion_free_gift_description = promotion.free_gift_description
promotion_product_applicability = promotion.product_applicability
promotion_money_budget = promotion.money_budget
promotion_promotion_status = promotion.promotion_status
promotion_brand_exclusion = promotion.brand_exclusion
promotion_limit_value = promotion.limit_value
promotion_promotion_id = promotion.promotion_id
promotion_custom_redemption_restriction = promotion.custom_redemption_restriction
promotion_store_applicability = promotion.store_applicability
promotion_promotion_effective_dates = promotion.promotion_effective_dates
```

---


### Credential

Uploads credentials for the Merchant Center account. If credentials already exist for this Merchant Center account and purpose, this method updates them.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `access_token` | String |  | An OAuth access token. |
| `purpose` | String |  | Indicates to Google how Google should use these OAuth tokens. |
| `expires_in` | String |  | The amount of time, in seconds, after which the access token is no longer valid. |
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


### Merchantsupport

Provide a list of merchant's issues with a support content and available actions. This content and actions are meant to be rendered and shown in third-party applications.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content_option` | String |  | Optional. How the detailed content should be returned. Default option is to return the content as a pre-rendered HTML text. |
| `user_input_action_option` | String |  | Optional. How actions with user input form should be handled. If not provided, actions will be returned as links that points merchant to Merchant Center where they can request the action. |
| `merchant_id` | String | ✅ | Required. The ID of the account to fetch issues for. |



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
    merchant_id = "value"  # Required. The ID of the account to fetch issues for.
}

```

---


### Label

Creates a new label, not assigned to any account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The display name of this label. |
| `label_id` | String |  | Output only. The ID of the label. |
| `description` | String |  | The description of this label. |
| `label_type` | String |  | Output only. The type of this label. |
| `account_id` | String |  | Immutable. The ID of account this label belongs to. |
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
| `website_claimed` | bool | Whether the account's website is claimed or not. |
| `products` | Vec<String> | List of product-related data by channel, destination, and country. Data in this field may be delayed by up to 30 minutes. |
| `account_id` | String | The ID of the account for which the status is reported. |
| `account_management` | String | How the account is managed. Acceptable values are: - "`manual`" - "`automatic`"  |
| `account_level_issues` | Vec<String> | A list of account level issues. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#accountStatus`" |


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
accountstatuse_website_claimed = accountstatuse.website_claimed
accountstatuse_products = accountstatuse.products
accountstatuse_account_id = accountstatuse.account_id
accountstatuse_account_management = accountstatuse.account_management
accountstatuse_account_level_issues = accountstatuse.account_level_issues
accountstatuse_kind = accountstatuse.kind
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
| `account_id` | String | Required. The ID of the account to which these account tax settings belong. |
| `rules` | Vec<String> | Tax rules. Updating the tax rules will enable "US" taxes (not reversible). Defining no rules is equivalent to not charging tax at all. |


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
accounttax_account_id = accounttax.account_id
accounttax_rules = accounttax.rules
```

---


### Datafeed

Registers a datafeed configuration with your Merchant Center account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `format` | String |  | Format of the feed file. |
| `id` | String |  | Required for update. The ID of the data feed. |
| `attribute_language` | String |  | The two-letter ISO 639-1 language in which the attributes are defined in the data feed. |
| `fetch_schedule` | String |  | Fetch schedule for the feed file. |
| `file_name` | String |  | Required. The filename of the feed. All feeds must have a unique file name. |
| `content_type` | String |  | Required. The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported. Acceptable values are: - "`local products`" - "`product inventory`" - "`products`"  |
| `targets` | Vec<String> |  | The targets this feed should apply to (country, language, destinations). |
| `name` | String |  | Required for insert. A descriptive name of the data feed. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#datafeed`" |
| `merchant_id` | String | ✅ | The ID of the account that manages the datafeed. This account cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `format` | String | Format of the feed file. |
| `id` | String | Required for update. The ID of the data feed. |
| `attribute_language` | String | The two-letter ISO 639-1 language in which the attributes are defined in the data feed. |
| `fetch_schedule` | String | Fetch schedule for the feed file. |
| `file_name` | String | Required. The filename of the feed. All feeds must have a unique file name. |
| `content_type` | String | Required. The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported. Acceptable values are: - "`local products`" - "`product inventory`" - "`products`"  |
| `targets` | Vec<String> | The targets this feed should apply to (country, language, destinations). |
| `name` | String | Required for insert. A descriptive name of the data feed. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#datafeed`" |


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
datafeed_format = datafeed.format
datafeed_id = datafeed.id
datafeed_attribute_language = datafeed.attribute_language
datafeed_fetch_schedule = datafeed.fetch_schedule
datafeed_file_name = datafeed.file_name
datafeed_content_type = datafeed.content_type
datafeed_targets = datafeed.targets
datafeed_name = datafeed.name
datafeed_kind = datafeed.kind
```

---


### Returnaddres

Inserts a return address for the Merchant Center account.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `country` | String |  | Required. The country of sale where the return address is applicable. |
| `label` | String |  | Required. The user-defined label of the return address. For the default address, use the label "default". |
| `phone_number` | String |  | Required. The merchant's contact phone number regarding the return. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#returnAddress`" |
| `return_address_id` | String |  | Return address ID generated by Google. |
| `address` | String |  | Required. The address. |
| `merchant_id` | String | ✅ | The Merchant Center account to insert a return address for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `country` | String | Required. The country of sale where the return address is applicable. |
| `label` | String | Required. The user-defined label of the return address. For the default address, use the label "default". |
| `phone_number` | String | Required. The merchant's contact phone number regarding the return. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#returnAddress`" |
| `return_address_id` | String | Return address ID generated by Google. |
| `address` | String | Required. The address. |


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
returnaddres_country = returnaddres.country
returnaddres_label = returnaddres.label
returnaddres_phone_number = returnaddres.phone_number
returnaddres_kind = returnaddres.kind
returnaddres_return_address_id = returnaddres.return_address_id
returnaddres_address = returnaddres.address
```

---


### Csse

Updates labels that are assigned to a CSS domain by its CSS group.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `label_ids` | Vec<String> |  | The list of label IDs. |
| `css_group_id` | String | ✅ | Required. The CSS group ID of the updated CSS domain. |
| `css_domain_id` | String | ✅ | Required. The ID of the updated CSS domain. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `full_name` | String | Output only. Immutable. The CSS domain's full name. |
| `homepage_uri` | String | Output only. Immutable. The CSS domain's homepage. |
| `css_group_id` | String | Output only. Immutable. The ID of the CSS group this CSS domain is affiliated with. Only populated for CSS group users. |
| `display_name` | String | Output only. Immutable. The CSS domain's display name, used when space is constrained. |
| `label_ids` | Vec<String> | A list of label IDs that are assigned to this CSS domain by its CSS group. Only populated for CSS group users. |
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
    css_group_id = "value"  # Required. The CSS group ID of the updated CSS domain.
    css_domain_id = "value"  # Required. The ID of the updated CSS domain.
}

# Access csse outputs
csse_id = csse.id
csse_full_name = csse.full_name
csse_homepage_uri = csse.homepage_uri
csse_css_group_id = csse.css_group_id
csse_display_name = csse.display_name
csse_label_ids = csse.label_ids
csse_css_domain_id = csse.css_domain_id
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
| `warehouses` | Vec<String> | Optional. A list of warehouses which can be referred to in `services`. |
| `postal_code_groups` | Vec<String> | A list of postal code groups that can be referred to in `services`. Optional. |
| `services` | Vec<String> | The target account's list of services. Optional. |
| `account_id` | String | The ID of the account to which these account shipping settings belong. Ignored upon update, always present in get request responses. |


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
shippingsetting_warehouses = shippingsetting.warehouses
shippingsetting_postal_code_groups = shippingsetting.postal_code_groups
shippingsetting_services = shippingsetting.services
shippingsetting_account_id = shippingsetting.account_id
```

---


### Returnpolicyonline

Creates a new return policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `countries` | Vec<String> |  | The countries of sale where the return policy is applicable. The values must be a valid 2 letter ISO 3166 code, e.g. "US". |
| `return_reason_category_info` | Vec<String> |  | The return reason category information. This required to not be empty unless the type of return policy is noReturns. |
| `name` | String |  | The name of the policy as shown in Merchant Center. |
| `return_policy_uri` | String |  | The return policy uri. This can used by Google to do a sanity check for the policy. |
| `restocking_fee` | String |  | The restocking fee that applies to all return reason categories. This would be treated as a free restocking fee if the value is not set. |
| `label` | String |  | The unique user-defined label of the return policy. The same label cannot be used in different return policies for the same country. Policies with the label 'default' will apply to all products, unless a product specifies a return_policy_label attribute. |
| `item_conditions` | Vec<String> |  | The item conditions that are accepted for returns. This is required to not be empty unless the type of return policy is noReturns. |
| `policy` | String |  | The return policy. |
| `return_policy_id` | String |  | Output only. Return policy ID generated by Google. |
| `return_methods` | Vec<String> |  | The return methods of how customers can return an item. This value is required to not be empty unless the type of return policy is noReturns. |
| `merchant_id` | String | ✅ | Required. The id of the merchant for which to retrieve the return policy online object. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `countries` | Vec<String> | The countries of sale where the return policy is applicable. The values must be a valid 2 letter ISO 3166 code, e.g. "US". |
| `return_reason_category_info` | Vec<String> | The return reason category information. This required to not be empty unless the type of return policy is noReturns. |
| `name` | String | The name of the policy as shown in Merchant Center. |
| `return_policy_uri` | String | The return policy uri. This can used by Google to do a sanity check for the policy. |
| `restocking_fee` | String | The restocking fee that applies to all return reason categories. This would be treated as a free restocking fee if the value is not set. |
| `label` | String | The unique user-defined label of the return policy. The same label cannot be used in different return policies for the same country. Policies with the label 'default' will apply to all products, unless a product specifies a return_policy_label attribute. |
| `item_conditions` | Vec<String> | The item conditions that are accepted for returns. This is required to not be empty unless the type of return policy is noReturns. |
| `policy` | String | The return policy. |
| `return_policy_id` | String | Output only. Return policy ID generated by Google. |
| `return_methods` | Vec<String> | The return methods of how customers can return an item. This value is required to not be empty unless the type of return policy is noReturns. |


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
returnpolicyonline_countries = returnpolicyonline.countries
returnpolicyonline_return_reason_category_info = returnpolicyonline.return_reason_category_info
returnpolicyonline_name = returnpolicyonline.name
returnpolicyonline_return_policy_uri = returnpolicyonline.return_policy_uri
returnpolicyonline_restocking_fee = returnpolicyonline.restocking_fee
returnpolicyonline_label = returnpolicyonline.label
returnpolicyonline_item_conditions = returnpolicyonline.item_conditions
returnpolicyonline_policy = returnpolicyonline.policy
returnpolicyonline_return_policy_id = returnpolicyonline.return_policy_id
returnpolicyonline_return_methods = returnpolicyonline.return_methods
```

---


### Pubsubnotificationsetting

Retrieves a Merchant Center account's pubsub notification settings.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cloud_topic_name` | String |  | Cloud pub/sub topic to which notifications are sent (read-only). |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#pubsubNotificationSettings`" |
| `registered_events` | Vec<String> |  | List of event types. Acceptable values are: - "`orderPendingShipment`"  |
| `merchant_id` | String | ✅ | The ID of the account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_topic_name` | String | Cloud pub/sub topic to which notifications are sent (read-only). |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#pubsubNotificationSettings`" |
| `registered_events` | Vec<String> | List of event types. Acceptable values are: - "`orderPendingShipment`"  |


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
pubsubnotificationsetting_kind = pubsubnotificationsetting.kind
pubsubnotificationsetting_registered_events = pubsubnotificationsetting.registered_events
```

---


### Liasetting

Sets the POS data provider for the specified country.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `merchant_id` | String | ✅ | The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account. |
| `account_id` | String | ✅ | The ID of the account for which to retrieve accessible Business Profiles. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `country_settings` | Vec<String> | The LIA settings for each country. |
| `account_id` | String | The ID of the account to which these LIA settings belong. Ignored upon update, always present in get request responses. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#liaSettings`" |


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
    merchant_id = "value"  # The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
    account_id = "value"  # The ID of the account for which to retrieve accessible Business Profiles.
}

# Access liasetting outputs
liasetting_id = liasetting.id
liasetting_country_settings = liasetting.country_settings
liasetting_account_id = liasetting.account_id
liasetting_kind = liasetting.kind
```

---


### Product

Uploads a product to your Merchant Center account. If an item with the same channel, contentLanguage, offerId, and targetCountry already exists, this method updates that entry.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String |  | Title of the item. |
| `transit_time_label` | String |  | The transit time label of the product, used to group product in account-level transit time tables. |
| `installment` | String |  | Number and amount of installments to pay for an item. |
| `loyalty_programs` | Vec<String> |  | Optional. A list of loyalty program information that is used to surface loyalty benefits (for example, better pricing, points, etc) to the user of this item. |
| `pattern` | String |  | The item's pattern (for example, polka dots). |
| `subscription_cost` | String |  | Number of periods (months or years) and amount of payment per period for an item with an associated subscription contract. |
| `max_energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `certifications` | Vec<String> |  | Product [certification](https://support.google.com/merchants/answer/13528839), introduced for EU energy efficiency labeling compliance using the [EU EPREL](https://eprel.ec.europa.eu/screen/home) database. |
| `canonical_link` | String |  | URL for the canonical version of your item's landing page. |
| `ads_grouping` | String |  | Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise. |
| `mobile_link_template` | String |  | URL template for merchant hosted local storefront optimized for mobile devices. |
| `excluded_destinations` | Vec<String> |  | The list of [destinations to exclude](//support.google.com/merchants/answer/6324486) for this target (corresponds to cleared check boxes in Merchant Center). Products that are excluded from all destinations for more than 7 days are automatically deleted. |
| `link` | String |  | URL directly linking to your item's page on your website. |
| `sell_on_google_quantity` | String |  | The quantity of the product that is available for selling on Google. Supported only for online products. |
| `item_group_id` | String |  | Shared identifier for all variants of the same product. |
| `content_language` | String |  | Required. The two-letter ISO 639-1 language code for the item. |
| `free_shipping_threshold` | Vec<String> |  | Optional. Conditions to be met for a product to have free shipping. |
| `image_link` | String |  | URL of an image of the item. |
| `product_height` | String |  | The height of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `availability` | String |  | Availability status of the item. |
| `max_handling_time` | String |  | Maximal product handling time (in business days). |
| `min_handling_time` | String |  | Minimal product handling time (in business days). |
| `unit_pricing_measure` | String |  | The measure and dimension of an item. |
| `virtual_model_link` | String |  | URL of the 3D model of the item to provide more visuals. |
| `ads_labels` | Vec<String> |  | Similar to ads_grouping, but only works on CPC. |
| `sizes` | Vec<String> |  | Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition). |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#product`" |
| `brand` | String |  | Brand of the item. |
| `maximum_retail_price` | String |  | Maximum retail price (MRP) of the item. Applicable to India only. |
| `custom_attributes` | Vec<String> |  | A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google (formerly known as Shopping Actions). |
| `size_type` | String |  | The cut of the item. Recommended for apparel items. |
| `additional_size_type` | String |  | Additional cut of the item. Used together with size_type to represent combined size types for apparel items. |
| `sale_price_effective_date` | String |  | Date range during which the item is on sale (see product data specification ). |
| `auto_pricing_min_price` | String |  | A safeguard in the [Automated Discounts](//support.google.com/merchants/answer/10295759) and [Dynamic Promotions](//support.google.com/merchants/answer/13949249) projects, ensuring that discounts on merchants' offers do not fall below this value, thereby preserving the offer's value and profitability. |
| `tax_category` | String |  | The tax category of the product, used to configure detailed tax nexus in account-level tax settings. |
| `size_system` | String |  | System in which the size is specified. Recommended for apparel items. |
| `pickup_method` | String |  | The pick up option for the item. Acceptable values are: - "`buy`" - "`reserve`" - "`ship to store`" - "`not supported`"  |
| `custom_label2` | String |  | Custom label 2 for custom grouping of items in a Shopping campaign. |
| `shipping_width` | String |  | Width of the item for shipping. |
| `product_length` | String |  | The length of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `link_template` | String |  | URL template for merchant hosted local storefront. |
| `custom_label1` | String |  | Custom label 1 for custom grouping of items in a Shopping campaign. |
| `pickup_sla` | String |  | Item store pickup timeline. Acceptable values are: - "`same day`" - "`next day`" - "`2-day`" - "`3-day`" - "`4-day`" - "`5-day`" - "`6-day`" - "`7-day`" - "`multi-week`"  |
| `shipping_height` | String |  | Height of the item for shipping. |
| `source` | String |  | Output only. The source of the offer, that is, how the offer was created. Acceptable values are: - "`api`" - "`crawl`" - "`feed`"  |
| `display_ads_similar_ids` | Vec<String> |  | Advertiser-specified recommendations. |
| `additional_image_links` | Vec<String> |  | Additional URLs of images of the item. |
| `product_details` | Vec<String> |  | Technical specification or additional product details. |
| `shipping_label` | String |  | The shipping label of the product, used to group product in account-level shipping rules. |
| `shopping_ads_excluded_countries` | Vec<String> |  | List of country codes (ISO 3166-1 alpha-2) to exclude the offer from Shopping Ads destination. Countries from this list are removed from countries configured in MC feed settings. |
| `unit_pricing_base_measure` | String |  | The preference of the denominator of the unit price. |
| `cost_of_goods_sold` | String |  | Cost of goods sold. Used for gross profit reporting. |
| `description` | String |  | Description of the item. |
| `display_ads_value` | f64 |  | Offer margin for dynamic remarketing campaigns. |
| `feed_label` | String |  | Feed label for the item. Either `targetCountry` or `feedLabel` is required. Must be less than or equal to 20 uppercase letters (A-Z), numbers (0-9), and dashes (-). |
| `google_product_category` | String |  | Google's category of the item (see [Google product taxonomy](https://support.google.com/merchants/answer/1705911)). When querying products, this field will contain the user provided value. There is currently no way to get back the auto assigned google product categories through the API. |
| `identifier_exists` | bool |  | False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada. |
| `display_ads_id` | String |  | An identifier for an item for dynamic remarketing campaigns. |
| `material` | String |  | The material of which the item is made. |
| `gtin` | String |  | Global Trade Item Number (GTIN) of the item. |
| `multipack` | String |  | The number of identical products in a merchant-defined multipack. |
| `custom_label4` | String |  | Custom label 4 for custom grouping of items in a Shopping campaign. |
| `product_types` | Vec<String> |  | Categories of the item (formatted as in product data specification). |
| `promotion_ids` | Vec<String> |  | The unique ID of a promotion. |
| `shipping_weight` | String |  | Weight of the item for shipping. |
| `structured_description` | String |  | Structured description, for algorithmically (AI)-generated descriptions. |
| `min_energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `disclosure_date` | String |  | The date time when an offer becomes visible in search results across Google’s YouTube surfaces, in [ISO 8601](http://en.wikipedia.org/wiki/ISO_8601) format. See [Disclosure date](https://support.google.com/merchants/answer/13034208) for more information. |
| `pause` | String |  | Publication of this item should be temporarily paused. Acceptable values are: - "`ads`"  |
| `gender` | String |  | Target gender of the item. |
| `cloud_export_additional_properties` | Vec<String> |  | Extra fields to export to the Cloud Retail program. |
| `id` | String |  | The REST ID of the product. Content API methods that operate on products take this as their `productId` parameter. The REST ID for a product has one of the 2 forms channel:contentLanguage: targetCountry: offerId or channel:contentLanguage:feedLabel: offerId. |
| `external_seller_id` | String |  | Required for multi-seller accounts. Use this attribute if you're a marketplace uploading products for various sellers to your multi-seller account. |
| `age_group` | String |  | Target age group of the item. |
| `offer_id` | String |  | Required. A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details. *Note:* Content API methods that operate on products take the REST ID of the product, *not* this identifier. |
| `mobile_link` | String |  | URL for the mobile-optimized version of your item's landing page. |
| `sale_price` | String |  | Advertised sale price of the item. |
| `sustainability_incentives` | Vec<String> |  | Optional. The list of sustainability incentive programs. |
| `shipping_length` | String |  | Length of the item for shipping. |
| `product_width` | String |  | The width of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `custom_label0` | String |  | Custom label 0 for custom grouping of items in a Shopping campaign. |
| `is_bundle` | bool |  | Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price. |
| `target_country` | String |  | Required. The CLDR territory code for the item's country of sale. |
| `channel` | String |  | Required. The item's channel (online or local). Acceptable values are: - "`local`" - "`online`"  |
| `condition` | String |  | Condition or state of the item. |
| `availability_date` | String |  | The day a pre-ordered product becomes available for delivery, in ISO 8601 format. |
| `display_ads_title` | String |  | Title of an item for dynamic remarketing campaigns. |
| `display_ads_link` | String |  | URL directly to your item's landing page for dynamic remarketing campaigns. |
| `lifestyle_image_links` | Vec<String> |  | Additional URLs of lifestyle images of the item. Used to explicitly identify images that showcase your item in a real-world context. See the Help Center article for more information. |
| `expiration_date` | String |  | Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future. |
| `mpn` | String |  | Manufacturer Part Number (MPN) of the item. |
| `price` | String |  | Price of the item. |
| `product_highlights` | Vec<String> |  | Bullet points describing the most relevant highlights of a product. |
| `loyalty_program` | String |  | Loyalty program information that is used to surface loyalty benefits ( for example, better pricing, points, etc) to the user of this item. This signular field points to the latest uploaded loyalty program info. This field will be deprecated in the coming weeks and should not be used in favor of the plural 'LoyaltyProgram' field below. |
| `product_weight` | String |  | The weight of the product in the units provided. The value must be between 0 (exclusive) and 2000 (inclusive). |
| `color` | String |  | Color of the item. |
| `ads_redirect` | String |  | Allows advertisers to override the item URL when the product is shown within the context of Product Ads. |
| `shipping` | Vec<String> |  | Shipping rules. |
| `structured_title` | String |  | Structured title, for algorithmically (AI)-generated titles. |
| `taxes` | Vec<String> |  | Tax information. |
| `adult` | bool |  | Should be set to true if the item is targeted towards adults. |
| `custom_label3` | String |  | Custom label 3 for custom grouping of items in a Shopping campaign. |
| `included_destinations` | Vec<String> |  | The list of [destinations to include](//support.google.com/merchants/answer/7501026) for this target (corresponds to checked check boxes in Merchant Center). Default destinations are always included unless provided in `excludedDestinations`. |
| `merchant_id` | String | ✅ | The ID of the account that contains the product. This account cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `title` | String | Title of the item. |
| `transit_time_label` | String | The transit time label of the product, used to group product in account-level transit time tables. |
| `installment` | String | Number and amount of installments to pay for an item. |
| `loyalty_programs` | Vec<String> | Optional. A list of loyalty program information that is used to surface loyalty benefits (for example, better pricing, points, etc) to the user of this item. |
| `pattern` | String | The item's pattern (for example, polka dots). |
| `subscription_cost` | String | Number of periods (months or years) and amount of payment per period for an item with an associated subscription contract. |
| `max_energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `certifications` | Vec<String> | Product [certification](https://support.google.com/merchants/answer/13528839), introduced for EU energy efficiency labeling compliance using the [EU EPREL](https://eprel.ec.europa.eu/screen/home) database. |
| `canonical_link` | String | URL for the canonical version of your item's landing page. |
| `ads_grouping` | String | Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise. |
| `mobile_link_template` | String | URL template for merchant hosted local storefront optimized for mobile devices. |
| `excluded_destinations` | Vec<String> | The list of [destinations to exclude](//support.google.com/merchants/answer/6324486) for this target (corresponds to cleared check boxes in Merchant Center). Products that are excluded from all destinations for more than 7 days are automatically deleted. |
| `link` | String | URL directly linking to your item's page on your website. |
| `sell_on_google_quantity` | String | The quantity of the product that is available for selling on Google. Supported only for online products. |
| `item_group_id` | String | Shared identifier for all variants of the same product. |
| `content_language` | String | Required. The two-letter ISO 639-1 language code for the item. |
| `free_shipping_threshold` | Vec<String> | Optional. Conditions to be met for a product to have free shipping. |
| `image_link` | String | URL of an image of the item. |
| `product_height` | String | The height of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `availability` | String | Availability status of the item. |
| `max_handling_time` | String | Maximal product handling time (in business days). |
| `min_handling_time` | String | Minimal product handling time (in business days). |
| `unit_pricing_measure` | String | The measure and dimension of an item. |
| `virtual_model_link` | String | URL of the 3D model of the item to provide more visuals. |
| `ads_labels` | Vec<String> | Similar to ads_grouping, but only works on CPC. |
| `sizes` | Vec<String> | Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition). |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#product`" |
| `brand` | String | Brand of the item. |
| `maximum_retail_price` | String | Maximum retail price (MRP) of the item. Applicable to India only. |
| `custom_attributes` | Vec<String> | A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google (formerly known as Shopping Actions). |
| `size_type` | String | The cut of the item. Recommended for apparel items. |
| `additional_size_type` | String | Additional cut of the item. Used together with size_type to represent combined size types for apparel items. |
| `sale_price_effective_date` | String | Date range during which the item is on sale (see product data specification ). |
| `auto_pricing_min_price` | String | A safeguard in the [Automated Discounts](//support.google.com/merchants/answer/10295759) and [Dynamic Promotions](//support.google.com/merchants/answer/13949249) projects, ensuring that discounts on merchants' offers do not fall below this value, thereby preserving the offer's value and profitability. |
| `tax_category` | String | The tax category of the product, used to configure detailed tax nexus in account-level tax settings. |
| `size_system` | String | System in which the size is specified. Recommended for apparel items. |
| `pickup_method` | String | The pick up option for the item. Acceptable values are: - "`buy`" - "`reserve`" - "`ship to store`" - "`not supported`"  |
| `custom_label2` | String | Custom label 2 for custom grouping of items in a Shopping campaign. |
| `shipping_width` | String | Width of the item for shipping. |
| `product_length` | String | The length of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `link_template` | String | URL template for merchant hosted local storefront. |
| `custom_label1` | String | Custom label 1 for custom grouping of items in a Shopping campaign. |
| `pickup_sla` | String | Item store pickup timeline. Acceptable values are: - "`same day`" - "`next day`" - "`2-day`" - "`3-day`" - "`4-day`" - "`5-day`" - "`6-day`" - "`7-day`" - "`multi-week`"  |
| `shipping_height` | String | Height of the item for shipping. |
| `source` | String | Output only. The source of the offer, that is, how the offer was created. Acceptable values are: - "`api`" - "`crawl`" - "`feed`"  |
| `display_ads_similar_ids` | Vec<String> | Advertiser-specified recommendations. |
| `additional_image_links` | Vec<String> | Additional URLs of images of the item. |
| `product_details` | Vec<String> | Technical specification or additional product details. |
| `shipping_label` | String | The shipping label of the product, used to group product in account-level shipping rules. |
| `shopping_ads_excluded_countries` | Vec<String> | List of country codes (ISO 3166-1 alpha-2) to exclude the offer from Shopping Ads destination. Countries from this list are removed from countries configured in MC feed settings. |
| `unit_pricing_base_measure` | String | The preference of the denominator of the unit price. |
| `cost_of_goods_sold` | String | Cost of goods sold. Used for gross profit reporting. |
| `description` | String | Description of the item. |
| `display_ads_value` | f64 | Offer margin for dynamic remarketing campaigns. |
| `feed_label` | String | Feed label for the item. Either `targetCountry` or `feedLabel` is required. Must be less than or equal to 20 uppercase letters (A-Z), numbers (0-9), and dashes (-). |
| `google_product_category` | String | Google's category of the item (see [Google product taxonomy](https://support.google.com/merchants/answer/1705911)). When querying products, this field will contain the user provided value. There is currently no way to get back the auto assigned google product categories through the API. |
| `identifier_exists` | bool | False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada. |
| `display_ads_id` | String | An identifier for an item for dynamic remarketing campaigns. |
| `material` | String | The material of which the item is made. |
| `gtin` | String | Global Trade Item Number (GTIN) of the item. |
| `multipack` | String | The number of identical products in a merchant-defined multipack. |
| `custom_label4` | String | Custom label 4 for custom grouping of items in a Shopping campaign. |
| `product_types` | Vec<String> | Categories of the item (formatted as in product data specification). |
| `promotion_ids` | Vec<String> | The unique ID of a promotion. |
| `shipping_weight` | String | Weight of the item for shipping. |
| `structured_description` | String | Structured description, for algorithmically (AI)-generated descriptions. |
| `min_energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `disclosure_date` | String | The date time when an offer becomes visible in search results across Google’s YouTube surfaces, in [ISO 8601](http://en.wikipedia.org/wiki/ISO_8601) format. See [Disclosure date](https://support.google.com/merchants/answer/13034208) for more information. |
| `pause` | String | Publication of this item should be temporarily paused. Acceptable values are: - "`ads`"  |
| `gender` | String | Target gender of the item. |
| `cloud_export_additional_properties` | Vec<String> | Extra fields to export to the Cloud Retail program. |
| `id` | String | The REST ID of the product. Content API methods that operate on products take this as their `productId` parameter. The REST ID for a product has one of the 2 forms channel:contentLanguage: targetCountry: offerId or channel:contentLanguage:feedLabel: offerId. |
| `external_seller_id` | String | Required for multi-seller accounts. Use this attribute if you're a marketplace uploading products for various sellers to your multi-seller account. |
| `age_group` | String | Target age group of the item. |
| `offer_id` | String | Required. A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details. *Note:* Content API methods that operate on products take the REST ID of the product, *not* this identifier. |
| `mobile_link` | String | URL for the mobile-optimized version of your item's landing page. |
| `sale_price` | String | Advertised sale price of the item. |
| `sustainability_incentives` | Vec<String> | Optional. The list of sustainability incentive programs. |
| `shipping_length` | String | Length of the item for shipping. |
| `product_width` | String | The width of the product in the units provided. The value must be between 0 (exclusive) and 3000 (inclusive). |
| `energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. |
| `custom_label0` | String | Custom label 0 for custom grouping of items in a Shopping campaign. |
| `is_bundle` | bool | Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price. |
| `target_country` | String | Required. The CLDR territory code for the item's country of sale. |
| `channel` | String | Required. The item's channel (online or local). Acceptable values are: - "`local`" - "`online`"  |
| `condition` | String | Condition or state of the item. |
| `availability_date` | String | The day a pre-ordered product becomes available for delivery, in ISO 8601 format. |
| `display_ads_title` | String | Title of an item for dynamic remarketing campaigns. |
| `display_ads_link` | String | URL directly to your item's landing page for dynamic remarketing campaigns. |
| `lifestyle_image_links` | Vec<String> | Additional URLs of lifestyle images of the item. Used to explicitly identify images that showcase your item in a real-world context. See the Help Center article for more information. |
| `expiration_date` | String | Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future. |
| `mpn` | String | Manufacturer Part Number (MPN) of the item. |
| `price` | String | Price of the item. |
| `product_highlights` | Vec<String> | Bullet points describing the most relevant highlights of a product. |
| `loyalty_program` | String | Loyalty program information that is used to surface loyalty benefits ( for example, better pricing, points, etc) to the user of this item. This signular field points to the latest uploaded loyalty program info. This field will be deprecated in the coming weeks and should not be used in favor of the plural 'LoyaltyProgram' field below. |
| `product_weight` | String | The weight of the product in the units provided. The value must be between 0 (exclusive) and 2000 (inclusive). |
| `color` | String | Color of the item. |
| `ads_redirect` | String | Allows advertisers to override the item URL when the product is shown within the context of Product Ads. |
| `shipping` | Vec<String> | Shipping rules. |
| `structured_title` | String | Structured title, for algorithmically (AI)-generated titles. |
| `taxes` | Vec<String> | Tax information. |
| `adult` | bool | Should be set to true if the item is targeted towards adults. |
| `custom_label3` | String | Custom label 3 for custom grouping of items in a Shopping campaign. |
| `included_destinations` | Vec<String> | The list of [destinations to include](//support.google.com/merchants/answer/7501026) for this target (corresponds to checked check boxes in Merchant Center). Default destinations are always included unless provided in `excludedDestinations`. |


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
product_title = product.title
product_transit_time_label = product.transit_time_label
product_installment = product.installment
product_loyalty_programs = product.loyalty_programs
product_pattern = product.pattern
product_subscription_cost = product.subscription_cost
product_max_energy_efficiency_class = product.max_energy_efficiency_class
product_certifications = product.certifications
product_canonical_link = product.canonical_link
product_ads_grouping = product.ads_grouping
product_mobile_link_template = product.mobile_link_template
product_excluded_destinations = product.excluded_destinations
product_link = product.link
product_sell_on_google_quantity = product.sell_on_google_quantity
product_item_group_id = product.item_group_id
product_content_language = product.content_language
product_free_shipping_threshold = product.free_shipping_threshold
product_image_link = product.image_link
product_product_height = product.product_height
product_availability = product.availability
product_max_handling_time = product.max_handling_time
product_min_handling_time = product.min_handling_time
product_unit_pricing_measure = product.unit_pricing_measure
product_virtual_model_link = product.virtual_model_link
product_ads_labels = product.ads_labels
product_sizes = product.sizes
product_kind = product.kind
product_brand = product.brand
product_maximum_retail_price = product.maximum_retail_price
product_custom_attributes = product.custom_attributes
product_size_type = product.size_type
product_additional_size_type = product.additional_size_type
product_sale_price_effective_date = product.sale_price_effective_date
product_auto_pricing_min_price = product.auto_pricing_min_price
product_tax_category = product.tax_category
product_size_system = product.size_system
product_pickup_method = product.pickup_method
product_custom_label2 = product.custom_label2
product_shipping_width = product.shipping_width
product_product_length = product.product_length
product_link_template = product.link_template
product_custom_label1 = product.custom_label1
product_pickup_sla = product.pickup_sla
product_shipping_height = product.shipping_height
product_source = product.source
product_display_ads_similar_ids = product.display_ads_similar_ids
product_additional_image_links = product.additional_image_links
product_product_details = product.product_details
product_shipping_label = product.shipping_label
product_shopping_ads_excluded_countries = product.shopping_ads_excluded_countries
product_unit_pricing_base_measure = product.unit_pricing_base_measure
product_cost_of_goods_sold = product.cost_of_goods_sold
product_description = product.description
product_display_ads_value = product.display_ads_value
product_feed_label = product.feed_label
product_google_product_category = product.google_product_category
product_identifier_exists = product.identifier_exists
product_display_ads_id = product.display_ads_id
product_material = product.material
product_gtin = product.gtin
product_multipack = product.multipack
product_custom_label4 = product.custom_label4
product_product_types = product.product_types
product_promotion_ids = product.promotion_ids
product_shipping_weight = product.shipping_weight
product_structured_description = product.structured_description
product_min_energy_efficiency_class = product.min_energy_efficiency_class
product_disclosure_date = product.disclosure_date
product_pause = product.pause
product_gender = product.gender
product_cloud_export_additional_properties = product.cloud_export_additional_properties
product_id = product.id
product_external_seller_id = product.external_seller_id
product_age_group = product.age_group
product_offer_id = product.offer_id
product_mobile_link = product.mobile_link
product_sale_price = product.sale_price
product_sustainability_incentives = product.sustainability_incentives
product_shipping_length = product.shipping_length
product_product_width = product.product_width
product_energy_efficiency_class = product.energy_efficiency_class
product_custom_label0 = product.custom_label0
product_is_bundle = product.is_bundle
product_target_country = product.target_country
product_channel = product.channel
product_condition = product.condition
product_availability_date = product.availability_date
product_display_ads_title = product.display_ads_title
product_display_ads_link = product.display_ads_link
product_lifestyle_image_links = product.lifestyle_image_links
product_expiration_date = product.expiration_date
product_mpn = product.mpn
product_price = product.price
product_product_highlights = product.product_highlights
product_loyalty_program = product.loyalty_program
product_product_weight = product.product_weight
product_color = product.color
product_ads_redirect = product.ads_redirect
product_shipping = product.shipping
product_structured_title = product.structured_title
product_taxes = product.taxes
product_adult = product.adult
product_custom_label3 = product.custom_label3
product_included_destinations = product.included_destinations
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


### Po

Creates a store for the given merchant.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `website_url` | String |  | The website url for the store or merchant. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#posStore`" |
| `store_address` | String |  | Required. The street address of the store. |
| `matching_status` | String |  | Output only. The matching status of POS store and Google Business Profile store. Possible values are: - "`matched`": The POS store is successfully matched with the Google Business Profile store. - "`failed`": The POS store is not matched with the Google Business Profile store. See matching_status_hint for further details. Note that there is up to 48 hours propagation delay for changes in Merchant Center (e.g. creation of new account, accounts linking) and Google Business Profile (e.g. store address update) which may affect the matching status. In such cases, after a delay call [pos.list](https://developers.google.com/shopping-content/reference/rest/v2.1/pos/list) to retrieve the updated matching status.  |
| `store_code` | String |  | Required. A store identifier that is unique for the given merchant. |
| `store_name` | String |  | The merchant or store name. |
| `gcid_category` | Vec<String> |  | The business type of the store. |
| `matching_status_hint` | String |  | Output only. The hint of why the matching has failed. This is only set when matching_status=failed. Possible values are: - "`linked-store-not-found`": There aren't any Google Business Profile stores available for matching. Connect your Merchant Center account with the Google Business Profile account. Or add a new Google Business Profile store corresponding to the POS store. - "`store-match-not-found`": The provided POS store couldn't be matched to any of the connected Google Business Profile stores. Merchant Center account is connected correctly and stores are available on Google Business Profile, but POS store location address does not match with Google Business Profile stores' addresses. Update POS store address or Google Business Profile store address to match correctly. - "`store-match-unverified`": The provided POS store couldn't be matched to any of the connected Google Business Profile stores, as the matched Google Business Profile store is unverified. Go through the Google Business Profile verification process to match correctly.  |
| `phone_number` | String |  | The store phone number. |
| `place_id` | String |  | The Google Place Id of the store location. |
| `target_merchant_id` | String | ✅ | The ID of the target merchant. |
| `merchant_id` | String | ✅ | The ID of the POS or inventory data provider. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `website_url` | String | The website url for the store or merchant. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#posStore`" |
| `store_address` | String | Required. The street address of the store. |
| `matching_status` | String | Output only. The matching status of POS store and Google Business Profile store. Possible values are: - "`matched`": The POS store is successfully matched with the Google Business Profile store. - "`failed`": The POS store is not matched with the Google Business Profile store. See matching_status_hint for further details. Note that there is up to 48 hours propagation delay for changes in Merchant Center (e.g. creation of new account, accounts linking) and Google Business Profile (e.g. store address update) which may affect the matching status. In such cases, after a delay call [pos.list](https://developers.google.com/shopping-content/reference/rest/v2.1/pos/list) to retrieve the updated matching status.  |
| `store_code` | String | Required. A store identifier that is unique for the given merchant. |
| `store_name` | String | The merchant or store name. |
| `gcid_category` | Vec<String> | The business type of the store. |
| `matching_status_hint` | String | Output only. The hint of why the matching has failed. This is only set when matching_status=failed. Possible values are: - "`linked-store-not-found`": There aren't any Google Business Profile stores available for matching. Connect your Merchant Center account with the Google Business Profile account. Or add a new Google Business Profile store corresponding to the POS store. - "`store-match-not-found`": The provided POS store couldn't be matched to any of the connected Google Business Profile stores. Merchant Center account is connected correctly and stores are available on Google Business Profile, but POS store location address does not match with Google Business Profile stores' addresses. Update POS store address or Google Business Profile store address to match correctly. - "`store-match-unverified`": The provided POS store couldn't be matched to any of the connected Google Business Profile stores, as the matched Google Business Profile store is unverified. Go through the Google Business Profile verification process to match correctly.  |
| `phone_number` | String | The store phone number. |
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
po_website_url = po.website_url
po_kind = po.kind
po_store_address = po.store_address
po_matching_status = po.matching_status
po_store_code = po.store_code
po_store_name = po.store_name
po_gcid_category = po.gcid_category
po_matching_status_hint = po.matching_status_hint
po_phone_number = po.phone_number
po_place_id = po.place_id
```

---


### Recommendation

Reports an interaction on a recommendation for a merchant.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subtype` | String |  | Optional. Subtype of the recommendations this interaction happened on. This field must be set only to the value that is returned by {@link `RecommendationsService.GenerateRecommendations`} call. |
| `response_token` | String |  | Required. Token of the response when recommendation was returned. |
| `type` | String |  | Required. Type of the recommendations on which this interaction happened. This field must be set only to the value that is returned by {@link `GenerateRecommendationsResponse`} call. |
| `interaction_type` | String |  | Required. Type of the interaction that is reported, for example INTERACTION_CLICK. |
| `merchant_id` | String | ✅ | Required. The ID of the account that wants to report an interaction. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response_token` | String | Output only. Response token is a string created for each `GenerateRecommendationsResponse`. This token doesn't expire, and is globally unique. This token must be used when reporting interactions for recommendations. |
| `recommendations` | Vec<String> | Recommendations generated for a request. |


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
recommendation_response_token = recommendation.response_token
recommendation_recommendations = recommendation.recommendations
```

---


### Returnpolicy

Inserts a return policy for the Merchant Center account.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `seasonal_overrides` | Vec<String> |  | An optional list of seasonal overrides. |
| `country` | String |  | Required. The country of sale where the return policy is applicable. |
| `policy` | String |  | Required. The policy. |
| `name` | String |  | Required. The name of the policy as shown in Merchant Center. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#returnPolicy`" |
| `label` | String |  | Required. The user-defined label of the return policy. For the default policy, use the label "default". |
| `return_policy_id` | String |  | Return policy ID generated by Google. |
| `non_free_return_reasons` | Vec<String> |  | Return reasons that will incur return fees. |
| `return_shipping_fee` | String |  | The return shipping fee that will apply to non free return reasons. |
| `merchant_id` | String | ✅ | The Merchant Center account to insert a return policy for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `seasonal_overrides` | Vec<String> | An optional list of seasonal overrides. |
| `country` | String | Required. The country of sale where the return policy is applicable. |
| `policy` | String | Required. The policy. |
| `name` | String | Required. The name of the policy as shown in Merchant Center. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#returnPolicy`" |
| `label` | String | Required. The user-defined label of the return policy. For the default policy, use the label "default". |
| `return_policy_id` | String | Return policy ID generated by Google. |
| `non_free_return_reasons` | Vec<String> | Return reasons that will incur return fees. |
| `return_shipping_fee` | String | The return shipping fee that will apply to non free return reasons. |


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
returnpolicy_seasonal_overrides = returnpolicy.seasonal_overrides
returnpolicy_country = returnpolicy.country
returnpolicy_policy = returnpolicy.policy
returnpolicy_name = returnpolicy.name
returnpolicy_kind = returnpolicy.kind
returnpolicy_label = returnpolicy.label
returnpolicy_return_policy_id = returnpolicy.return_policy_id
returnpolicy_non_free_return_reasons = returnpolicy.non_free_return_reasons
returnpolicy_return_shipping_fee = returnpolicy.return_shipping_fee
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
| `merchant_id` | String | Required. The ID of the account. |
| `effective_review_state` | String | Output only. The effective value of review state for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account. |
| `uri_settings` | String | URL settings for cart or checkout URL. |
| `effective_enrollment_state` | String | Output only. The effective value of enrollment state for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account. |
| `effective_uri_settings` | String | The effective value of `url_settings` for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account. |
| `enrollment_state` | String | Output only. Reflects the merchant enrollment state in `Checkout` feature. |
| `review_state` | String | Output only. Reflects the merchant review state in `Checkout` feature. This is set based on the data quality reviews of the URL provided by the merchant. A merchant with enrollment state as `ENROLLED` can be in the following review states: `IN_REVIEW`, `APPROVED` or `DISAPPROVED`. A merchant must be in an enrollment_state of `ENROLLED` before a review can begin for the merchant. |


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
checkoutsetting_merchant_id = checkoutsetting.merchant_id
checkoutsetting_effective_review_state = checkoutsetting.effective_review_state
checkoutsetting_uri_settings = checkoutsetting.uri_settings
checkoutsetting_effective_enrollment_state = checkoutsetting.effective_enrollment_state
checkoutsetting_effective_uri_settings = checkoutsetting.effective_uri_settings
checkoutsetting_enrollment_state = checkoutsetting.enrollment_state
checkoutsetting_review_state = checkoutsetting.review_state
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
| `product_id` | String | The ID of the product for which status is reported. |
| `creation_date` | String | Date on which the item has been created, in ISO 8601 format. |
| `google_expiration_date` | String | Date on which the item expires in Google Shopping, in ISO 8601 format. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#productStatus`" |
| `destination_statuses` | Vec<String> | The intended destinations for the product. |
| `item_level_issues` | Vec<String> | A list of all issues associated with the product. |
| `last_update_date` | String | Date on which the item has been last updated, in ISO 8601 format. |
| `title` | String | The title of the product. |
| `link` | String | The link to the product. |


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
productstatuse_product_id = productstatuse.product_id
productstatuse_creation_date = productstatuse.creation_date
productstatuse_google_expiration_date = productstatuse.google_expiration_date
productstatuse_kind = productstatuse.kind
productstatuse_destination_statuses = productstatuse.destination_statuses
productstatuse_item_level_issues = productstatuse.item_level_issues
productstatuse_last_update_date = productstatuse.last_update_date
productstatuse_title = productstatuse.title
productstatuse_link = productstatuse.link
```

---


### Region

Creates a region definition in your Merchant Center account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `postal_code_area` | String |  | A list of postal codes that defines the region area. |
| `region_id` | String |  | Output only. Immutable. The ID uniquely identifying each region. |
| `regional_inventory_eligible` | bool |  | Output only. Indicates if the region is eligible to use in the Regional Inventory configuration. |
| `geotarget_area` | String |  | A list of geotargets that defines the region area. |
| `merchant_id` | String |  | Output only. Immutable. Merchant that owns the region. |
| `shipping_eligible` | bool |  | Output only. Indicates if the region is eligible to use in the Shipping Services configuration. |
| `display_name` | String |  | The display name of the region. |
| `merchant_id` | String | ✅ | Required. The id of the merchant for which to create region definition. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `postal_code_area` | String | A list of postal codes that defines the region area. |
| `region_id` | String | Output only. Immutable. The ID uniquely identifying each region. |
| `regional_inventory_eligible` | bool | Output only. Indicates if the region is eligible to use in the Regional Inventory configuration. |
| `geotarget_area` | String | A list of geotargets that defines the region area. |
| `merchant_id` | String | Output only. Immutable. Merchant that owns the region. |
| `shipping_eligible` | bool | Output only. Indicates if the region is eligible to use in the Shipping Services configuration. |
| `display_name` | String | The display name of the region. |


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
region_postal_code_area = region.postal_code_area
region_region_id = region.region_id
region_regional_inventory_eligible = region.regional_inventory_eligible
region_geotarget_area = region.geotarget_area
region_merchant_id = region.merchant_id
region_shipping_eligible = region.shipping_eligible
region_display_name = region.display_name
```

---


### Collection

Uploads a collection to your Merchant Center account. If a collection with the same collectionId already exists, this method updates that entry. In each update, the collection is completely replaced by the fields in the body of the update request.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `link` | String |  | A collection’s landing page. URL directly linking to your collection's page on your website. [link attribute](https://support.google.com/merchants/answer/9673983) |
| `custom_label0` | String |  | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. [Custom label](https://support.google.com/merchants/answer/9674217) |
| `headline` | Vec<String> |  | Your collection's name. [headline attribute](https://support.google.com/merchants/answer/9673580) |
| `custom_label3` | String |  | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `featured_product` | Vec<String> |  | This identifies one or more products associated with the collection. Used as a lookup to the corresponding product ID in your product feeds. Provide a maximum of 100 featuredProduct (for collections). Provide up to 10 featuredProduct (for Shoppable Images only) with ID and X and Y coordinates. [featured_product attribute](https://support.google.com/merchants/answer/9703736) |
| `custom_label4` | String |  | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `id` | String |  | Required. The REST ID of the collection. Content API methods that operate on collections take this as their collectionId parameter. The REST ID for a collection is of the form collectionId. [id attribute](https://support.google.com/merchants/answer/9649290) |
| `image_link` | Vec<String> |  | The URL of a collection’s image. [image_link attribute](https://support.google.com/merchants/answer/9703236) |
| `custom_label2` | String |  | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `custom_label1` | String |  | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `language` | String |  | The language of a collection and the language of any featured products linked to the collection. [language attribute](https://support.google.com/merchants/answer/9673781) |
| `mobile_link` | String |  | A collection’s mobile-optimized landing page when you have a different URL for mobile and desktop traffic. [mobile_link attribute](https://support.google.com/merchants/answer/9646123) |
| `product_country` | String |  | [product_country attribute](https://support.google.com/merchants/answer/9674155) |
| `merchant_id` | String | ✅ | Required. The ID of the account that contains the collection. This account cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `link` | String | A collection’s landing page. URL directly linking to your collection's page on your website. [link attribute](https://support.google.com/merchants/answer/9673983) |
| `custom_label0` | String | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. [Custom label](https://support.google.com/merchants/answer/9674217) |
| `headline` | Vec<String> | Your collection's name. [headline attribute](https://support.google.com/merchants/answer/9673580) |
| `custom_label3` | String | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `featured_product` | Vec<String> | This identifies one or more products associated with the collection. Used as a lookup to the corresponding product ID in your product feeds. Provide a maximum of 100 featuredProduct (for collections). Provide up to 10 featuredProduct (for Shoppable Images only) with ID and X and Y coordinates. [featured_product attribute](https://support.google.com/merchants/answer/9703736) |
| `custom_label4` | String | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `id` | String | Required. The REST ID of the collection. Content API methods that operate on collections take this as their collectionId parameter. The REST ID for a collection is of the form collectionId. [id attribute](https://support.google.com/merchants/answer/9649290) |
| `image_link` | Vec<String> | The URL of a collection’s image. [image_link attribute](https://support.google.com/merchants/answer/9703236) |
| `custom_label2` | String | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `custom_label1` | String | Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. |
| `language` | String | The language of a collection and the language of any featured products linked to the collection. [language attribute](https://support.google.com/merchants/answer/9673781) |
| `mobile_link` | String | A collection’s mobile-optimized landing page when you have a different URL for mobile and desktop traffic. [mobile_link attribute](https://support.google.com/merchants/answer/9646123) |
| `product_country` | String | [product_country attribute](https://support.google.com/merchants/answer/9674155) |


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
collection_link = collection.link
collection_custom_label0 = collection.custom_label0
collection_headline = collection.headline
collection_custom_label3 = collection.custom_label3
collection_featured_product = collection.featured_product
collection_custom_label4 = collection.custom_label4
collection_id = collection.id
collection_image_link = collection.image_link
collection_custom_label2 = collection.custom_label2
collection_custom_label1 = collection.custom_label1
collection_language = collection.language
collection_mobile_link = collection.mobile_link
collection_product_country = collection.product_country
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
| `language` | String | The two-letter ISO 639-1 language for which the status is reported. |
| `feed_label` | String | The feed label status is reported for. |
| `items_total` | String | The number of items in the feed that were processed. |
| `errors` | Vec<String> | The list of errors occurring in the feed. |
| `processing_status` | String | The processing status of the feed. Acceptable values are: - "`"`failure`": The feed could not be processed or all items had errors.`" - "`in progress`": The feed is being processed. - "`none`": The feed has not yet been processed. For example, a feed that has never been uploaded will have this processing status. - "`success`": The feed was processed successfully, though some items might have had errors.  |
| `country` | String | The country for which the status is reported, represented as a CLDR territory code. |
| `warnings` | Vec<String> | The list of errors occurring in the feed. |
| `items_valid` | String | The number of items in the feed that were valid. |
| `datafeed_id` | String | The ID of the feed for which the status is reported. |
| `last_upload_date` | String | The last date at which the feed was uploaded. |
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
datafeedstatuse_language = datafeedstatuse.language
datafeedstatuse_feed_label = datafeedstatuse.feed_label
datafeedstatuse_items_total = datafeedstatuse.items_total
datafeedstatuse_errors = datafeedstatuse.errors
datafeedstatuse_processing_status = datafeedstatuse.processing_status
datafeedstatuse_country = datafeedstatuse.country
datafeedstatuse_warnings = datafeedstatuse.warnings
datafeedstatuse_items_valid = datafeedstatuse.items_valid
datafeedstatuse_datafeed_id = datafeedstatuse.datafeed_id
datafeedstatuse_last_upload_date = datafeedstatuse.last_upload_date
datafeedstatuse_kind = datafeedstatuse.kind
```

---


### Conversionsource

Creates a new conversion source.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expire_time` | String |  | Output only. The time when an archived conversion source becomes permanently deleted and is no longer available to undelete. |
| `conversion_source_id` | String |  | Output only. Generated by the Content API upon creation of a new `ConversionSource`. Format: [a-z]{4}:.+ The four characters before the colon represent the type of conversio source. Content after the colon represents the ID of the conversion source within that type. The ID of two different conversion sources might be the same across different types. The following type prefixes are supported: - galk: For GoogleAnalyticsLink sources. - mcdn: For MerchantCenterDestination sources. |
| `state` | String |  | Output only. Current state of this conversion source. Can't be edited through the API. |
| `google_analytics_link` | String |  | Immutable. Conversion Source of type "Link to Google Analytics Property". |
| `merchant_center_destination` | String |  | Conversion Source of type "Merchant Center Tag Destination". |
| `merchant_id` | String | ✅ | Required. The ID of the account that owns the new conversion source. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expire_time` | String | Output only. The time when an archived conversion source becomes permanently deleted and is no longer available to undelete. |
| `conversion_source_id` | String | Output only. Generated by the Content API upon creation of a new `ConversionSource`. Format: [a-z]{4}:.+ The four characters before the colon represent the type of conversio source. Content after the colon represents the ID of the conversion source within that type. The ID of two different conversion sources might be the same across different types. The following type prefixes are supported: - galk: For GoogleAnalyticsLink sources. - mcdn: For MerchantCenterDestination sources. |
| `state` | String | Output only. Current state of this conversion source. Can't be edited through the API. |
| `google_analytics_link` | String | Immutable. Conversion Source of type "Link to Google Analytics Property". |
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
conversionsource_expire_time = conversionsource.expire_time
conversionsource_conversion_source_id = conversionsource.conversion_source_id
conversionsource_state = conversionsource.state
conversionsource_google_analytics_link = conversionsource.google_analytics_link
conversionsource_merchant_center_destination = conversionsource.merchant_center_destination
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
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `method_quotas` | Vec<String> | The current quota usage and limits per each method. |


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
quota_next_page_token = quota.next_page_token
quota_method_quotas = quota.method_quotas
```

---


### Regionalinventory

Updates the regional inventory of a product in your Merchant Center account. If a regional inventory with the same region ID already exists, this method updates that entry.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sale_price` | String |  | The sale price of the product. Mandatory if `sale_price_effective_date` is defined. |
| `custom_attributes` | Vec<String> |  | A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form. |
| `sale_price_effective_date` | String |  | A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates might be specified as 'null' if undecided. |
| `availability` | String |  | The availability of the product. |
| `price` | String |  | The price of the product. |
| `region_id` | String |  | The ID uniquely identifying each region. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#regionalInventory`". |
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


### Account

Creates a Merchant Center sub-account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `css_id` | String |  | ID of CSS the account belongs to. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#account`". |
| `ads_links` | Vec<String> |  | Linked Ads accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the Ads interface or through the Google Ads API. To delete an active link, or to cancel a link request, remove it from the list. |
| `conversion_settings` | String |  | Settings for conversion tracking. |
| `id` | String |  | Required. 64-bit Merchant Center account ID. |
| `google_my_business_link` | String |  | The Business Profile which is linked or in the process of being linked with the Merchant Center account. |
| `label_ids` | Vec<String> |  | Manually created label IDs that are assigned to the account by CSS. |
| `seller_id` | String |  | Client-specific, locally-unique, internal ID for the child account. |
| `business_identity` | String |  | The business identity attributes can be used to self-declare attributes that let customers know more about your business. |
| `account_management` | String |  | Output only. How the account is managed. Acceptable values are: - "`manual`" - "`automatic`"  |
| `users` | Vec<String> |  | Users with access to the account. Every account (except for subaccounts) must have at least one admin user. |
| `youtube_channel_links` | Vec<String> |  | Linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list. |
| `name` | String |  | Required. Display name for the account. |
| `automatic_improvements` | String |  | The automatic improvements of the account can be used to automatically update items, improve images and shipping. Each section inside AutomaticImprovements is updated separately. |
| `automatic_label_ids` | Vec<String> |  | Automatically created label IDs that are assigned to the account by CSS Center. |
| `website_url` | String |  | The merchant's website. |
| `adult_content` | bool |  | Indicates whether the merchant sells adult content. |
| `business_information` | String |  | The business information of the account. |
| `merchant_id` | String | ✅ | The ID of the managing account. This must be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `css_id` | String | ID of CSS the account belongs to. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#account`". |
| `ads_links` | Vec<String> | Linked Ads accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the Ads interface or through the Google Ads API. To delete an active link, or to cancel a link request, remove it from the list. |
| `conversion_settings` | String | Settings for conversion tracking. |
| `id` | String | Required. 64-bit Merchant Center account ID. |
| `google_my_business_link` | String | The Business Profile which is linked or in the process of being linked with the Merchant Center account. |
| `label_ids` | Vec<String> | Manually created label IDs that are assigned to the account by CSS. |
| `seller_id` | String | Client-specific, locally-unique, internal ID for the child account. |
| `business_identity` | String | The business identity attributes can be used to self-declare attributes that let customers know more about your business. |
| `account_management` | String | Output only. How the account is managed. Acceptable values are: - "`manual`" - "`automatic`"  |
| `users` | Vec<String> | Users with access to the account. Every account (except for subaccounts) must have at least one admin user. |
| `youtube_channel_links` | Vec<String> | Linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list. |
| `name` | String | Required. Display name for the account. |
| `automatic_improvements` | String | The automatic improvements of the account can be used to automatically update items, improve images and shipping. Each section inside AutomaticImprovements is updated separately. |
| `automatic_label_ids` | Vec<String> | Automatically created label IDs that are assigned to the account by CSS Center. |
| `website_url` | String | The merchant's website. |
| `adult_content` | bool | Indicates whether the merchant sells adult content. |
| `business_information` | String | The business information of the account. |


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
account_css_id = account.css_id
account_kind = account.kind
account_ads_links = account.ads_links
account_conversion_settings = account.conversion_settings
account_id = account.id
account_google_my_business_link = account.google_my_business_link
account_label_ids = account.label_ids
account_seller_id = account.seller_id
account_business_identity = account.business_identity
account_account_management = account.account_management
account_users = account.users
account_youtube_channel_links = account.youtube_channel_links
account_name = account.name
account_automatic_improvements = account.automatic_improvements
account_automatic_label_ids = account.automatic_label_ids
account_website_url = account.website_url
account_adult_content = account.adult_content
account_business_information = account.business_information
```

---


### Report

Retrieves merchant performance metrics matching the search query and optionally segmented by selected dimensions.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_size` | i64 |  | Number of ReportRows to retrieve in a single page. Defaults to 1000. Values above 5000 are coerced to 5000. |
| `query` | String |  | Required. Query that defines performance metrics to retrieve and dimensions according to which the metrics are to be segmented. For details on how to construct your query, see the [Query Language guide](https://developers.google.com/shopping-content/guides/reports/query-language/overview). |
| `page_token` | String |  | Token of the page to retrieve. If not specified, the first page of results is returned. In order to request the next page of results, the value obtained from `next_page_token` in the previous response should be used. |
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


### Localinventory

Updates the local inventory of a product in your Merchant Center account.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `store_code` | String |  | Required. The store code of this local inventory resource. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#localInventory`" |
| `pickup_method` | String |  | The supported pickup method for this offer. Unless the value is "not supported", this field must be submitted together with `pickupSla`. For accepted attribute values, see the local product inventory feed specification. |
| `pickup_sla` | String |  | The expected date that an order will be ready for pickup relative to the order date. Must be submitted together with `pickupMethod`. For accepted attribute values, see the local product inventory feed specification. |
| `price` | String |  | The price of the product. |
| `sale_price` | String |  | The sale price of the product. Mandatory if `sale_price_effective_date` is defined. |
| `instore_product_location` | String |  | The in-store product location. |
| `quantity` | i64 |  | The quantity of the product. Must be nonnegative. |
| `sale_price_effective_date` | String |  | A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates may be specified as 'null' if undecided. |
| `availability` | String |  | The availability of the product. For accepted attribute values, see the local product inventory feed specification. |
| `custom_attributes` | Vec<String> |  | A list of custom (merchant-provided) attributes. Can also be used to submit any attribute of the feed specification in its generic form, for example, `{ "name": "size type", "value": "regular" }`. |
| `merchant_id` | String | ✅ | The ID of the account that contains the product. This account cannot be a multi-client account. |
| `product_id` | String | ✅ | The REST ID of the product for which to update local inventory. |



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
    merchant_id = "value"  # The ID of the account that contains the product. This account cannot be a multi-client account.
    product_id = "value"  # The REST ID of the product for which to update local inventory.
}

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


### Orderpayment

Notify about refund on user's selected payments method.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `refund_state` | String |  | Whether refund was successful. |
| `invoice_id` | String |  | Deprecated. Please use invoiceIds instead. |
| `invoice_ids` | Vec<String> |  | Invoice IDs from the orderinvoices service that correspond to the refund. |
| `merchant_id` | String | ✅ | The ID of the account that manages the order. This cannot be a multi-client account. |
| `order_id` | String | ✅ | The ID of the order for which charge is happening. |



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
    merchant_id = "value"  # The ID of the account that manages the order. This cannot be a multi-client account.
    order_id = "value"  # The ID of the order for which charge is happening.
}

```

---


### Order

Cancels a line item, making a full refund.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `quantity` | i64 |  | The quantity to cancel. |
| `operation_id` | String |  | The ID of the operation. Unique across all operations for a given order. |
| `amount_pretax` | String |  | Amount to refund for the cancelation. Optional. If not set, Google will calculate the default based on the price and tax of the items involved. The amount must not be larger than the net amount left on the order. |
| `product_id` | String |  | The ID of the product to cancel. This is the REST ID used in the products service. Either lineItemId or productId is required. |
| `amount_tax` | String |  | Tax amount that correspond to cancellation amount in amountPretax. |
| `line_item_id` | String |  | The ID of the line item to cancel. Either lineItemId or productId is required. |
| `reason` | String |  | The reason for the cancellation. |
| `amount` | String |  | Deprecated. Please use amountPretax and amountTax instead. |
| `reason_text` | String |  | The explanation of the reason. |
| `merchant_id` | String | ✅ | The ID of the account that manages the order. This cannot be a multi-client account. |
| `order_id` | String | ✅ | The ID of the order. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channel_type` | String | The channel type of the order: "purchaseOnGoogle" or "googleExpress". |
| `payment_status` | String | The status of the payment. |
| `promotions` | Vec<String> | Deprecated. The details of the merchant provided promotions applied to the order. More details about the program are here. |
| `acknowledged` | bool | Whether the order was acknowledged. |
| `refunds` | Vec<String> | Refunds for the order. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "content#order". |
| `net_amount` | String | The net amount for the order. For example, if an order was originally for a grand total of $100 and a refund was issued for $20, the net amount will be $80. |
| `payment_method` | String | The details of the payment method. |
| `id` | String | The REST id of the order. Globally unique. |
| `merchant_order_id` | String | Merchant-provided id of the order. |
| `shipping_cost_tax` | String | The tax for the total shipping cost. |
| `placed_date` | String | The date when the order was placed, in ISO 8601 format. |
| `shipping_cost` | String | The total cost of shipping for all items. |
| `status` | String | The status of the order. |
| `customer` | String | The details of the customer who placed the order. |
| `merchant_id` | String |  |
| `delivery_details` | String | The details for the delivery. |
| `shipments` | Vec<String> | Shipments of the order. |
| `line_items` | Vec<String> | Line items that are ordered. |
| `shipping_option` | String | The requested shipping option. |


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
order_channel_type = order.channel_type
order_payment_status = order.payment_status
order_promotions = order.promotions
order_acknowledged = order.acknowledged
order_refunds = order.refunds
order_kind = order.kind
order_net_amount = order.net_amount
order_payment_method = order.payment_method
order_id = order.id
order_merchant_order_id = order.merchant_order_id
order_shipping_cost_tax = order.shipping_cost_tax
order_placed_date = order.placed_date
order_shipping_cost = order.shipping_cost
order_status = order.status
order_customer = order.customer
order_merchant_id = order.merchant_id
order_delivery_details = order.delivery_details
order_shipments = order.shipments
order_line_items = order.line_items
order_shipping_option = order.shipping_option
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
| `order_return_id` | String |  |
| `creation_date` | String |  |
| `return_items` | Vec<String> |  |
| `merchant_order_id` | String |  |
| `return_shipments` | Vec<String> |  |
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
orderreturn_order_return_id = orderreturn.order_return_id
orderreturn_creation_date = orderreturn.creation_date
orderreturn_return_items = orderreturn.return_items
orderreturn_merchant_order_id = orderreturn.merchant_order_id
orderreturn_return_shipments = orderreturn.return_shipments
orderreturn_order_id = orderreturn.order_id
```

---


### Orderinvoice

Creates a charge invoice for a shipment group, and triggers a charge capture for non-facilitated payment orders.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `line_item_invoices` | Vec<String> |  | [required] Invoice details per line item. |
| `invoice_id` | String |  | [required] The ID of the invoice. |
| `operation_id` | String |  | [required] The ID of the operation, unique across all operations for a given order. |
| `invoice_summary` | String |  | [required] Invoice summary. |
| `shipment_group_id` | String |  | [required] ID of the shipment group. |
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


### Order

Deprecated, please use returnRefundLineItem instead.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reason_text` | String |  | The explanation of the reason. |
| `amount_tax` | String |  | Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, amountPretax must be set. Calculated automatically if not provided. |
| `reason` | String |  | The reason for the refund. Acceptable values are: - "`adjustment`" - "`courtesyAdjustment`" - "`customerCanceled`" - "`customerDiscretionaryReturn`" - "`deliveredLateByCarrier`" - "`feeAdjustment`" - "`lateShipmentCredit`" - "`noInventory`" - "`other`" - "`priceError`" - "`productArrivedDamaged`" - "`productNotAsDescribed`" - "`shippingCostAdjustment`" - "`taxAdjustment`" - "`undeliverableShippingAddress`" - "`wrongProductShipped`"  |
| `operation_id` | String |  | The ID of the operation. Unique across all operations for a given order. |
| `amount` | String |  | Deprecated. Please use amountPretax and amountTax instead. |
| `amount_pretax` | String |  | The amount that is refunded. Either amount or amountPretax should be filled. |
| `order_id` | String | ✅ | The ID of the order to refund. |
| `merchant_id` | String | ✅ | The ID of the account that manages the order. This cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `line_items` | Vec<String> | Line items that are ordered. |
| `payment_method` | String | The details of the payment method. |
| `tax_collector` | String | The party responsible for collecting and remitting taxes. Acceptable values are: - "`marketplaceFacilitator`" - "`merchant`"  |
| `acknowledged` | bool | Whether the order was acknowledged. |
| `customer` | String | The details of the customer who placed the order. |
| `channel_type` | String | Deprecated. Acceptable values are: - "`googleExpress`" - "`purchasesOnGoogle`"  |
| `shipping_cost_tax` | String | The tax for the total shipping cost. |
| `delivery_details` | String | Delivery details for shipments of type `delivery`. |
| `shipping_option` | String | Deprecated. Shipping details are provided with line items instead. Acceptable values are: - "`economy`" - "`expedited`" - "`oneDay`" - "`sameDay`" - "`standard`" - "`twoDay`"  |
| `net_amount` | String | The net amount for the order. For example, if an order was originally for a grand total of $100 and a refund was issued for $20, the net amount will be $80. |
| `payment_status` | String | The status of the payment. Acceptable values are: - "`paymentCaptured`" - "`paymentRejected`" - "`paymentSecured`" - "`pendingAuthorization`"  |
| `placed_date` | String | The date when the order was placed, in ISO 8601 format. |
| `promotions` | Vec<String> | The details of the merchant provided promotions applied to the order. To determine which promotions apply to which products, check the `Promotions[].Benefits[].OfferIds` field against the `LineItems[].Product.OfferId` field for each promotion. If a promotion is applied to more than 1 `offerId`, divide the discount value by the number of affected offers to determine how much discount to apply to each `offerId`. Examples: 1. To calculate the line item level discount for a single specific item: For each promotion, subtract the `Promotions[].Benefits[].Discount.value` amount from the `LineItems[].Price.value`. 2. To calculate the line item level discount for multiple quantity of a specific item: For each promotion, divide the `Promotions[].Benefits[].Discount.value` by the quantity of products and substract it from `LineItems[].Product.Price.value` for each quantity item. Only 1 promotion can be applied to an offerId in a given order. To refund an item which had a promotion applied to it, make sure to refund the amount after first subtracting the promotion discount from the item price. More details about the program are here. |
| `merchant_order_id` | String | Merchant-provided ID of the order. |
| `id` | String | The REST ID of the order. Globally unique. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#order`" |
| `shipping_cost` | String | The total cost of shipping for all items. |
| `pickup_details` | String | Pickup details for shipments of type `pickup`. |
| `refunds` | Vec<String> | Refunds for the order. |
| `shipments` | Vec<String> | Shipments of the order. |
| `status` | String | The status of the order. Acceptable values are: - "`canceled`" - "`delivered`" - "`inProgress`" - "`partiallyDelivered`" - "`partiallyReturned`" - "`partiallyShipped`" - "`pendingShipment`" - "`returned`" - "`shipped`"  |
| `merchant_id` | String |  |


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
    order_id = "value"  # The ID of the order to refund.
    merchant_id = "value"  # The ID of the account that manages the order. This cannot be a multi-client account.
}

# Access order outputs
order_id = order.id
order_line_items = order.line_items
order_payment_method = order.payment_method
order_tax_collector = order.tax_collector
order_acknowledged = order.acknowledged
order_customer = order.customer
order_channel_type = order.channel_type
order_shipping_cost_tax = order.shipping_cost_tax
order_delivery_details = order.delivery_details
order_shipping_option = order.shipping_option
order_net_amount = order.net_amount
order_payment_status = order.payment_status
order_placed_date = order.placed_date
order_promotions = order.promotions
order_merchant_order_id = order.merchant_order_id
order_id = order.id
order_kind = order.kind
order_shipping_cost = order.shipping_cost
order_pickup_details = order.pickup_details
order_refunds = order.refunds
order_shipments = order.shipments
order_status = order.status
order_merchant_id = order.merchant_id
```

---


### Datafeed

Registers a datafeed configuration with your Merchant Center account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_name` | String |  | Required. The filename of the feed. All feeds must have a unique file name. |
| `intended_destinations` | Vec<String> |  | [DEPRECATED] Please use targets[].includedDestinations instead. The list of intended destinations (corresponds to checked check boxes in Merchant Center). |
| `content_type` | String |  | Required. The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported. Acceptable values are: - "`local products`" - "`product inventory`" - "`products`"  |
| `targets` | Vec<String> |  | The targets this feed should apply to (country, language, destinations). |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#datafeed`" |
| `content_language` | String |  | [DEPRECATED] Please use targets[].language instead. The two-letter ISO 639-1 language of the items in the feed. Must be a valid language for `targetCountry`. |
| `name` | String |  | Required for insert. A descriptive name of the data feed. |
| `format` | String |  | Format of the feed file. |
| `id` | String |  | Required for update. The ID of the data feed. |
| `target_country` | String |  | [DEPRECATED] Please use targets[].country instead. The country where the items in the feed will be included in the search index, represented as a CLDR territory code. |
| `attribute_language` | String |  | The two-letter ISO 639-1 language in which the attributes are defined in the data feed. |
| `fetch_schedule` | String |  | Fetch schedule for the feed file. |
| `merchant_id` | String | ✅ | The ID of the account that manages the datafeed. This account cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_name` | String | Required. The filename of the feed. All feeds must have a unique file name. |
| `intended_destinations` | Vec<String> | [DEPRECATED] Please use targets[].includedDestinations instead. The list of intended destinations (corresponds to checked check boxes in Merchant Center). |
| `content_type` | String | Required. The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported. Acceptable values are: - "`local products`" - "`product inventory`" - "`products`"  |
| `targets` | Vec<String> | The targets this feed should apply to (country, language, destinations). |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#datafeed`" |
| `content_language` | String | [DEPRECATED] Please use targets[].language instead. The two-letter ISO 639-1 language of the items in the feed. Must be a valid language for `targetCountry`. |
| `name` | String | Required for insert. A descriptive name of the data feed. |
| `format` | String | Format of the feed file. |
| `id` | String | Required for update. The ID of the data feed. |
| `target_country` | String | [DEPRECATED] Please use targets[].country instead. The country where the items in the feed will be included in the search index, represented as a CLDR territory code. |
| `attribute_language` | String | The two-letter ISO 639-1 language in which the attributes are defined in the data feed. |
| `fetch_schedule` | String | Fetch schedule for the feed file. |


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
datafeed_file_name = datafeed.file_name
datafeed_intended_destinations = datafeed.intended_destinations
datafeed_content_type = datafeed.content_type
datafeed_targets = datafeed.targets
datafeed_kind = datafeed.kind
datafeed_content_language = datafeed.content_language
datafeed_name = datafeed.name
datafeed_format = datafeed.format
datafeed_id = datafeed.id
datafeed_target_country = datafeed.target_country
datafeed_attribute_language = datafeed.attribute_language
datafeed_fetch_schedule = datafeed.fetch_schedule
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
| `next_page_token` | String | The token for the retrieval of the next page of transactions. |
| `transactions` | Vec<String> | The list of transactions. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "content#orderreportsListTransactionsResponse". |


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
orderreport_next_page_token = orderreport.next_page_token
orderreport_transactions = orderreport.transactions
orderreport_kind = orderreport.kind
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
| `item_level_issues` | Vec<String> | A list of all issues associated with the product. |
| `title` | String | The title of the product. |
| `destination_statuses` | Vec<String> | The intended destinations for the product. |
| `google_expiration_date` | String | Date on which the item expires in Google Shopping, in ISO 8601 format. |
| `last_update_date` | String | Date on which the item has been last updated, in ISO 8601 format. |
| `link` | String | The link to the product. |
| `creation_date` | String | Date on which the item has been created, in ISO 8601 format. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#productStatus`" |
| `product` | String | Product data after applying all the join inputs. |
| `data_quality_issues` | Vec<String> | DEPRECATED - never populated |
| `product_id` | String | The ID of the product for which status is reported. |


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
productstatuse_item_level_issues = productstatuse.item_level_issues
productstatuse_title = productstatuse.title
productstatuse_destination_statuses = productstatuse.destination_statuses
productstatuse_google_expiration_date = productstatuse.google_expiration_date
productstatuse_last_update_date = productstatuse.last_update_date
productstatuse_link = productstatuse.link
productstatuse_creation_date = productstatuse.creation_date
productstatuse_kind = productstatuse.kind
productstatuse_product = productstatuse.product
productstatuse_data_quality_issues = productstatuse.data_quality_issues
productstatuse_product_id = productstatuse.product_id
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
| `errors` | Vec<String> | The list of errors occurring in the feed. |
| `items_total` | String | The number of items in the feed that were processed. |
| `language` | String | The two-letter ISO 639-1 language for which the status is reported. |
| `items_valid` | String | The number of items in the feed that were valid. |
| `country` | String | The country for which the status is reported, represented as a CLDR territory code. |
| `last_upload_date` | String | The last date at which the feed was uploaded. |
| `warnings` | Vec<String> | The list of errors occurring in the feed. |
| `datafeed_id` | String | The ID of the feed for which the status is reported. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#datafeedStatus`" |
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
datafeedstatuse_errors = datafeedstatuse.errors
datafeedstatuse_items_total = datafeedstatuse.items_total
datafeedstatuse_language = datafeedstatuse.language
datafeedstatuse_items_valid = datafeedstatuse.items_valid
datafeedstatuse_country = datafeedstatuse.country
datafeedstatuse_last_upload_date = datafeedstatuse.last_upload_date
datafeedstatuse_warnings = datafeedstatuse.warnings
datafeedstatuse_datafeed_id = datafeedstatuse.datafeed_id
datafeedstatuse_kind = datafeedstatuse.kind
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
| `website_claimed` | bool | Whether the account's website is claimed or not. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#accountStatus`" |
| `account_level_issues` | Vec<String> | A list of account level issues. |
| `data_quality_issues` | Vec<String> | DEPRECATED - never populated. |
| `products` | Vec<String> | List of product-related data by channel, destination, and country. Data in this field may be delayed by up to 30 minutes. |
| `account_id` | String | The ID of the account for which the status is reported. |


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
accountstatuse_website_claimed = accountstatuse.website_claimed
accountstatuse_kind = accountstatuse.kind
accountstatuse_account_level_issues = accountstatuse.account_level_issues
accountstatuse_data_quality_issues = accountstatuse.data_quality_issues
accountstatuse_products = accountstatuse.products
accountstatuse_account_id = accountstatuse.account_id
```

---


### Product

Uploads a product to your Merchant Center account. If an item with the same channel, contentLanguage, offerId, and targetCountry already exists, this method updates that entry.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `offer_id` | String |  | Required. A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details. *Note:* Content API methods that operate on products take the REST ID of the product, *not* this identifier. |
| `age_group` | String |  | Target age group of the item. Acceptable values are: - "`adult`" - "`infant`" - "`kids`" - "`newborn`" - "`toddler`" - "`youngAdult`"  |
| `max_energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `pattern` | String |  | The item's pattern (e.g. polka dots). |
| `product_type` | String |  | Your category of the item (formatted as in products data specification). |
| `availability_date` | String |  | The day a pre-ordered product becomes available for delivery, in ISO 8601 format. |
| `adult` | bool |  | Should be set to true if the item is targeted towards adults. |
| `is_bundle` | bool |  | Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price. |
| `shipping` | Vec<String> |  | Shipping rules. |
| `adwords_redirect` | String |  | Allows advertisers to override the item URL when the product is shown within the context of Product Ads. |
| `shipping_length` | String |  | Length of the item for shipping. |
| `target_country` | String |  | Required. The CLDR territory code for the item. |
| `unit_pricing_base_measure` | String |  | The preference of the denominator of the unit price. |
| `brand` | String |  | Brand of the item. |
| `loyalty_points` | String |  | Loyalty points that users receive after purchasing the item. Japan only. |
| `display_ads_id` | String |  | An identifier for an item for dynamic remarketing campaigns. |
| `min_handling_time` | String |  | Minimal product handling time (in business days). |
| `aspects` | Vec<String> |  | Deprecated. Do not use. |
| `shipping_height` | String |  | Height of the item for shipping. |
| `custom_label0` | String |  | Custom label 0 for custom grouping of items in a Shopping campaign. |
| `mpn` | String |  | Manufacturer Part Number (MPN) of the item. |
| `expiration_date` | String |  | Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future. |
| `custom_label1` | String |  | Custom label 1 for custom grouping of items in a Shopping campaign. |
| `sizes` | Vec<String> |  | Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition). |
| `custom_groups` | Vec<String> |  | A list of custom (merchant-provided) custom attribute groups. |
| `display_ads_similar_ids` | Vec<String> |  | Advertiser-specified recommendations. |
| `id` | String |  | The REST ID of the product. Content API methods that operate on products take this as their `productId` parameter. The REST ID for a product is of the form channel:contentLanguage: targetCountry: offerId. |
| `condition` | String |  | Condition or state of the item. Acceptable values are: - "`new`" - "`refurbished`" - "`used`"  |
| `additional_image_links` | Vec<String> |  | Additional URLs of images of the item. |
| `min_energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `validated_destinations` | Vec<String> |  | Deprecated. The read-only list of intended destinations which passed validation. |
| `gender` | String |  | Target gender of the item. Acceptable values are: - "`female`" - "`male`" - "`unisex`"  |
| `online_only` | bool |  | Deprecated. |
| `description` | String |  | Description of the item. |
| `availability` | String |  | Availability status of the item. Acceptable values are: - "`in stock`" - "`out of stock`" - "`preorder`"  |
| `google_product_category` | String |  | Google's category of the item (see [Google product taxonomy](https://support.google.com/merchants/answer/1705911)). When querying products, this field will contain the user provided value. There is currently no way to get back the auto assigned google product categories through the API. |
| `warnings` | Vec<String> |  | Read-only warnings. |
| `promotion_ids` | Vec<String> |  | The unique ID of a promotion. |
| `link` | String |  | URL directly linking to your item's page on your website. |
| `shipping_width` | String |  | Width of the item for shipping. |
| `destinations` | Vec<String> |  | Specifies the intended destinations for the product. |
| `custom_attributes` | Vec<String> |  | A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (e.g., `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google (formerly known as Shopping Actions). |
| `size_system` | String |  | System in which the size is specified. Recommended for apparel items. Acceptable values are: - "`AU`" - "`BR`" - "`CN`" - "`DE`" - "`EU`" - "`FR`" - "`IT`" - "`JP`" - "`MEX`" - "`UK`" - "`US`"  |
| `source` | String |  | The source of the offer, i.e., how the offer was created. Acceptable values are: - "`api`" - "`crawl`" - "`feed`"  |
| `adwords_grouping` | String |  | Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise. |
| `sell_on_google_quantity` | String |  | The quantity of the product that is available for selling on Google. Supported only for online products. |
| `custom_label3` | String |  | Custom label 3 for custom grouping of items in a Shopping campaign. |
| `mobile_link` | String |  | URL for the mobile-optimized version of your item's landing page. |
| `adwords_labels` | Vec<String> |  | Similar to adwords_grouping, but only works on CPC. |
| `display_ads_link` | String |  | URL directly to your item's landing page for dynamic remarketing campaigns. |
| `identifier_exists` | bool |  | False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada. |
| `shipping_label` | String |  | The shipping label of the product, used to group product in account-level shipping rules. |
| `size_type` | String |  | The cut of the item. Recommended for apparel items. Acceptable values are: - "`big and tall`" - "`maternity`" - "`oversize`" - "`petite`" - "`plus`" - "`regular`"  |
| `max_handling_time` | String |  | Maximal product handling time (in business days). |
| `unit_pricing_measure` | String |  | The measure and dimension of an item. |
| `cost_of_goods_sold` | String |  | Cost of goods sold. Used for gross profit reporting. |
| `additional_product_types` | Vec<String> |  | Additional categories of the item (formatted as in products data specification). |
| `energy_efficiency_class` | String |  | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `taxes` | Vec<String> |  | Tax information. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#product`" |
| `content_language` | String |  | Required. The two-letter ISO 639-1 language code for the item. |
| `color` | String |  | Color of the item. |
| `material` | String |  | The material of which the item is made. |
| `item_group_id` | String |  | Shared identifier for all variants of the same product. |
| `multipack` | String |  | The number of identical products in a merchant-defined multipack. |
| `image_link` | String |  | URL of an image of the item. |
| `installment` | String |  | Number and amount of installments to pay for an item. |
| `custom_label4` | String |  | Custom label 4 for custom grouping of items in a Shopping campaign. |
| `channel` | String |  | Required. The item's channel (online or local). Acceptable values are: - "`local`" - "`online`"  |
| `title` | String |  | Title of the item. |
| `canonical_link` | String |  | URL for the canonical version of your item's landing page. |
| `gtin` | String |  | Global Trade Item Number (GTIN) of the item. |
| `price` | String |  | Price of the item. |
| `display_ads_title` | String |  | Title of an item for dynamic remarketing campaigns. |
| `display_ads_value` | f64 |  | Offer margin for dynamic remarketing campaigns. |
| `custom_label2` | String |  | Custom label 2 for custom grouping of items in a Shopping campaign. |
| `sale_price_effective_date` | String |  | Date range during which the item is on sale (see products data specification ). |
| `shipping_weight` | String |  | Weight of the item for shipping. |
| `sale_price` | String |  | Advertised sale price of the item. |
| `merchant_id` | String | ✅ | The ID of the account that contains the product. This account cannot be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `offer_id` | String | Required. A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details. *Note:* Content API methods that operate on products take the REST ID of the product, *not* this identifier. |
| `age_group` | String | Target age group of the item. Acceptable values are: - "`adult`" - "`infant`" - "`kids`" - "`newborn`" - "`toddler`" - "`youngAdult`"  |
| `max_energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `pattern` | String | The item's pattern (e.g. polka dots). |
| `product_type` | String | Your category of the item (formatted as in products data specification). |
| `availability_date` | String | The day a pre-ordered product becomes available for delivery, in ISO 8601 format. |
| `adult` | bool | Should be set to true if the item is targeted towards adults. |
| `is_bundle` | bool | Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price. |
| `shipping` | Vec<String> | Shipping rules. |
| `adwords_redirect` | String | Allows advertisers to override the item URL when the product is shown within the context of Product Ads. |
| `shipping_length` | String | Length of the item for shipping. |
| `target_country` | String | Required. The CLDR territory code for the item. |
| `unit_pricing_base_measure` | String | The preference of the denominator of the unit price. |
| `brand` | String | Brand of the item. |
| `loyalty_points` | String | Loyalty points that users receive after purchasing the item. Japan only. |
| `display_ads_id` | String | An identifier for an item for dynamic remarketing campaigns. |
| `min_handling_time` | String | Minimal product handling time (in business days). |
| `aspects` | Vec<String> | Deprecated. Do not use. |
| `shipping_height` | String | Height of the item for shipping. |
| `custom_label0` | String | Custom label 0 for custom grouping of items in a Shopping campaign. |
| `mpn` | String | Manufacturer Part Number (MPN) of the item. |
| `expiration_date` | String | Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future. |
| `custom_label1` | String | Custom label 1 for custom grouping of items in a Shopping campaign. |
| `sizes` | Vec<String> | Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition). |
| `custom_groups` | Vec<String> | A list of custom (merchant-provided) custom attribute groups. |
| `display_ads_similar_ids` | Vec<String> | Advertiser-specified recommendations. |
| `id` | String | The REST ID of the product. Content API methods that operate on products take this as their `productId` parameter. The REST ID for a product is of the form channel:contentLanguage: targetCountry: offerId. |
| `condition` | String | Condition or state of the item. Acceptable values are: - "`new`" - "`refurbished`" - "`used`"  |
| `additional_image_links` | Vec<String> | Additional URLs of images of the item. |
| `min_energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `validated_destinations` | Vec<String> | Deprecated. The read-only list of intended destinations which passed validation. |
| `gender` | String | Target gender of the item. Acceptable values are: - "`female`" - "`male`" - "`unisex`"  |
| `online_only` | bool | Deprecated. |
| `description` | String | Description of the item. |
| `availability` | String | Availability status of the item. Acceptable values are: - "`in stock`" - "`out of stock`" - "`preorder`"  |
| `google_product_category` | String | Google's category of the item (see [Google product taxonomy](https://support.google.com/merchants/answer/1705911)). When querying products, this field will contain the user provided value. There is currently no way to get back the auto assigned google product categories through the API. |
| `warnings` | Vec<String> | Read-only warnings. |
| `promotion_ids` | Vec<String> | The unique ID of a promotion. |
| `link` | String | URL directly linking to your item's page on your website. |
| `shipping_width` | String | Width of the item for shipping. |
| `destinations` | Vec<String> | Specifies the intended destinations for the product. |
| `custom_attributes` | Vec<String> | A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (e.g., `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google (formerly known as Shopping Actions). |
| `size_system` | String | System in which the size is specified. Recommended for apparel items. Acceptable values are: - "`AU`" - "`BR`" - "`CN`" - "`DE`" - "`EU`" - "`FR`" - "`IT`" - "`JP`" - "`MEX`" - "`UK`" - "`US`"  |
| `source` | String | The source of the offer, i.e., how the offer was created. Acceptable values are: - "`api`" - "`crawl`" - "`feed`"  |
| `adwords_grouping` | String | Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise. |
| `sell_on_google_quantity` | String | The quantity of the product that is available for selling on Google. Supported only for online products. |
| `custom_label3` | String | Custom label 3 for custom grouping of items in a Shopping campaign. |
| `mobile_link` | String | URL for the mobile-optimized version of your item's landing page. |
| `adwords_labels` | Vec<String> | Similar to adwords_grouping, but only works on CPC. |
| `display_ads_link` | String | URL directly to your item's landing page for dynamic remarketing campaigns. |
| `identifier_exists` | bool | False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada. |
| `shipping_label` | String | The shipping label of the product, used to group product in account-level shipping rules. |
| `size_type` | String | The cut of the item. Recommended for apparel items. Acceptable values are: - "`big and tall`" - "`maternity`" - "`oversize`" - "`petite`" - "`plus`" - "`regular`"  |
| `max_handling_time` | String | Maximal product handling time (in business days). |
| `unit_pricing_measure` | String | The measure and dimension of an item. |
| `cost_of_goods_sold` | String | Cost of goods sold. Used for gross profit reporting. |
| `additional_product_types` | Vec<String> | Additional categories of the item (formatted as in products data specification). |
| `energy_efficiency_class` | String | The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - "`A`" - "`A+`" - "`A++`" - "`A+++`" - "`B`" - "`C`" - "`D`" - "`E`" - "`F`" - "`G`"  |
| `taxes` | Vec<String> | Tax information. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#product`" |
| `content_language` | String | Required. The two-letter ISO 639-1 language code for the item. |
| `color` | String | Color of the item. |
| `material` | String | The material of which the item is made. |
| `item_group_id` | String | Shared identifier for all variants of the same product. |
| `multipack` | String | The number of identical products in a merchant-defined multipack. |
| `image_link` | String | URL of an image of the item. |
| `installment` | String | Number and amount of installments to pay for an item. |
| `custom_label4` | String | Custom label 4 for custom grouping of items in a Shopping campaign. |
| `channel` | String | Required. The item's channel (online or local). Acceptable values are: - "`local`" - "`online`"  |
| `title` | String | Title of the item. |
| `canonical_link` | String | URL for the canonical version of your item's landing page. |
| `gtin` | String | Global Trade Item Number (GTIN) of the item. |
| `price` | String | Price of the item. |
| `display_ads_title` | String | Title of an item for dynamic remarketing campaigns. |
| `display_ads_value` | f64 | Offer margin for dynamic remarketing campaigns. |
| `custom_label2` | String | Custom label 2 for custom grouping of items in a Shopping campaign. |
| `sale_price_effective_date` | String | Date range during which the item is on sale (see products data specification ). |
| `shipping_weight` | String | Weight of the item for shipping. |
| `sale_price` | String | Advertised sale price of the item. |


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
product_offer_id = product.offer_id
product_age_group = product.age_group
product_max_energy_efficiency_class = product.max_energy_efficiency_class
product_pattern = product.pattern
product_product_type = product.product_type
product_availability_date = product.availability_date
product_adult = product.adult
product_is_bundle = product.is_bundle
product_shipping = product.shipping
product_adwords_redirect = product.adwords_redirect
product_shipping_length = product.shipping_length
product_target_country = product.target_country
product_unit_pricing_base_measure = product.unit_pricing_base_measure
product_brand = product.brand
product_loyalty_points = product.loyalty_points
product_display_ads_id = product.display_ads_id
product_min_handling_time = product.min_handling_time
product_aspects = product.aspects
product_shipping_height = product.shipping_height
product_custom_label0 = product.custom_label0
product_mpn = product.mpn
product_expiration_date = product.expiration_date
product_custom_label1 = product.custom_label1
product_sizes = product.sizes
product_custom_groups = product.custom_groups
product_display_ads_similar_ids = product.display_ads_similar_ids
product_id = product.id
product_condition = product.condition
product_additional_image_links = product.additional_image_links
product_min_energy_efficiency_class = product.min_energy_efficiency_class
product_validated_destinations = product.validated_destinations
product_gender = product.gender
product_online_only = product.online_only
product_description = product.description
product_availability = product.availability
product_google_product_category = product.google_product_category
product_warnings = product.warnings
product_promotion_ids = product.promotion_ids
product_link = product.link
product_shipping_width = product.shipping_width
product_destinations = product.destinations
product_custom_attributes = product.custom_attributes
product_size_system = product.size_system
product_source = product.source
product_adwords_grouping = product.adwords_grouping
product_sell_on_google_quantity = product.sell_on_google_quantity
product_custom_label3 = product.custom_label3
product_mobile_link = product.mobile_link
product_adwords_labels = product.adwords_labels
product_display_ads_link = product.display_ads_link
product_identifier_exists = product.identifier_exists
product_shipping_label = product.shipping_label
product_size_type = product.size_type
product_max_handling_time = product.max_handling_time
product_unit_pricing_measure = product.unit_pricing_measure
product_cost_of_goods_sold = product.cost_of_goods_sold
product_additional_product_types = product.additional_product_types
product_energy_efficiency_class = product.energy_efficiency_class
product_taxes = product.taxes
product_kind = product.kind
product_content_language = product.content_language
product_color = product.color
product_material = product.material
product_item_group_id = product.item_group_id
product_multipack = product.multipack
product_image_link = product.image_link
product_installment = product.installment
product_custom_label4 = product.custom_label4
product_channel = product.channel
product_title = product.title
product_canonical_link = product.canonical_link
product_gtin = product.gtin
product_price = product.price
product_display_ads_title = product.display_ads_title
product_display_ads_value = product.display_ads_value
product_custom_label2 = product.custom_label2
product_sale_price_effective_date = product.sale_price_effective_date
product_shipping_weight = product.shipping_weight
product_sale_price = product.sale_price
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
| `rules` | Vec<String> | Tax rules. Updating the tax rules will enable US taxes (not reversible). Defining no rules is equivalent to not charging tax at all. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "content#accountTax". |
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
accounttax_rules = accounttax.rules
accounttax_kind = accounttax.kind
accounttax_account_id = accounttax.account_id
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
| `account_id` | String | The ID of the account to which these LIA settings belong. Ignored upon update, always present in get request responses. |
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
    account_id = "value"  # The ID of the account that manages the order. This cannot be a multi-client account.
    merchant_id = "value"  # The ID of the managing account. If this parameter is not the same as accountId, then this account must be a multi-client account and `accountId` must be the ID of a sub-account of this account.
}

# Access liasetting outputs
liasetting_id = liasetting.id
liasetting_kind = liasetting.kind
liasetting_account_id = liasetting.account_id
liasetting_country_settings = liasetting.country_settings
```

---


### Account

Creates a Merchant Center sub-account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reviews_url` | String |  | [DEPRECATED] This field is never returned and will be ignored if provided. |
| `youtube_channel_links` | Vec<String> |  | List of linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list. |
| `business_information` | String |  | The business information of the account. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#account`" |
| `name` | String |  | Required. Display name for the account. |
| `adult_content` | bool |  | Indicates whether the merchant sells adult content. |
| `adwords_links` | Vec<String> |  | List of linked AdWords accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the AdWords interface or through the AdWords API. To delete an active link, or to cancel a link request, remove it from the list. |
| `seller_id` | String |  | Client-specific, locally-unique, internal ID for the child account. |
| `google_my_business_link` | String |  | The GMB account which is linked or in the process of being linked with the Merchant Center account. |
| `id` | String |  | Required for update. Merchant Center account ID. |
| `users` | Vec<String> |  | Users with access to the account. Every account (except for subaccounts) must have at least one admin user. |
| `website_url` | String |  | The merchant's website. |
| `merchant_id` | String | ✅ | The ID of the managing account. This must be a multi-client account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reviews_url` | String | [DEPRECATED] This field is never returned and will be ignored if provided. |
| `youtube_channel_links` | Vec<String> | List of linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list. |
| `business_information` | String | The business information of the account. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#account`" |
| `name` | String | Required. Display name for the account. |
| `adult_content` | bool | Indicates whether the merchant sells adult content. |
| `adwords_links` | Vec<String> | List of linked AdWords accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the AdWords interface or through the AdWords API. To delete an active link, or to cancel a link request, remove it from the list. |
| `seller_id` | String | Client-specific, locally-unique, internal ID for the child account. |
| `google_my_business_link` | String | The GMB account which is linked or in the process of being linked with the Merchant Center account. |
| `id` | String | Required for update. Merchant Center account ID. |
| `users` | Vec<String> | Users with access to the account. Every account (except for subaccounts) must have at least one admin user. |
| `website_url` | String | The merchant's website. |


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
account_youtube_channel_links = account.youtube_channel_links
account_business_information = account.business_information
account_kind = account.kind
account_name = account.name
account_adult_content = account.adult_content
account_adwords_links = account.adwords_links
account_seller_id = account.seller_id
account_google_my_business_link = account.google_my_business_link
account_id = account.id
account_users = account.users
account_website_url = account.website_url
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
| `merchant_order_id` | String | Merchant defined order ID. |
| `order_id` | String | Google order ID. |
| `creation_date` | String | The date of creation of the return, in ISO 8601 format. |
| `order_return_id` | String | Order return ID generated by Google. |
| `return_shipments` | Vec<String> | Shipments of the return. |
| `return_items` | Vec<String> | Items of the return. |


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
orderreturn_order_id = orderreturn.order_id
orderreturn_creation_date = orderreturn.creation_date
orderreturn_order_return_id = orderreturn.order_return_id
orderreturn_return_shipments = orderreturn.return_shipments
orderreturn_return_items = orderreturn.return_items
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
| `services` | Vec<String> | The target account's list of services. Optional. |
| `postal_code_groups` | Vec<String> | A list of postal code groups that can be referred to in `services`. Optional. |
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
shippingsetting_account_id = shippingsetting.account_id
shippingsetting_services = shippingsetting.services
shippingsetting_postal_code_groups = shippingsetting.postal_code_groups
shippingsetting_warehouses = shippingsetting.warehouses
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
| `gcid_category` | Vec<String> |  | The business type of the store. |
| `phone_number` | String |  | The store phone number. |
| `website_url` | String |  | The website url for the store or merchant. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "`content#posStore`" |
| `store_code` | String |  | Required. A store identifier that is unique for the given merchant. |
| `place_id` | String |  | The Google Place Id of the store location. |
| `merchant_id` | String | ✅ | The ID of the POS or inventory data provider. |
| `target_merchant_id` | String | ✅ | The ID of the target merchant. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `store_name` | String | The merchant or store name. |
| `store_address` | String | Required. The street address of the store. |
| `gcid_category` | Vec<String> | The business type of the store. |
| `phone_number` | String | The store phone number. |
| `website_url` | String | The website url for the store or merchant. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "`content#posStore`" |
| `store_code` | String | Required. A store identifier that is unique for the given merchant. |
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
    merchant_id = "value"  # The ID of the POS or inventory data provider.
    target_merchant_id = "value"  # The ID of the target merchant.
}

# Access po outputs
po_id = po.id
po_store_name = po.store_name
po_store_address = po.store_address
po_gcid_category = po.gcid_category
po_phone_number = po.phone_number
po_website_url = po.website_url
po_kind = po.kind
po_store_code = po.store_code
po_place_id = po.place_id
```

---


### Orderinvoice

Creates a charge invoice for a shipment group, and triggers a charge capture for orderinvoice enabled orders.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `shipment_group_id` | String |  | [required] ID of the shipment group. It is assigned by the merchant in the `shipLineItems` method and is used to group multiple line items that have the same kind of shipping charges. |
| `invoice_id` | String |  | [required] The ID of the invoice. |
| `invoice_summary` | String |  | [required] Invoice summary. |
| `line_item_invoices` | Vec<String> |  | [required] Invoice details per line item. |
| `operation_id` | String |  | [required] The ID of the operation, unique across all operations for a given order. |
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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple returncarrier resources
returncarrier_0 = provider.content_api.Returncarrier {
    account_id = "value-0"
}
returncarrier_1 = provider.content_api.Returncarrier {
    account_id = "value-1"
}
returncarrier_2 = provider.content_api.Returncarrier {
    account_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    returncarrier = provider.content_api.Returncarrier {
        account_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Content_api Documentation](https://cloud.google.com/content_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
