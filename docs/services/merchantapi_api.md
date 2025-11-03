# Merchantapi_api Service



**Resources**: 82

---

## Overview

The merchantapi_api service provides access to 82 resource types:

- [File_upload](#file_upload) [R]
- [Data_source](#data_source) [CRUD]
- [Regional_inventorie](#regional_inventorie) [CRD]
- [Local_inventorie](#local_inventorie) [CRD]
- [Quota](#quota) [R]
- [Product](#product) [R]
- [Product_input](#product_input) [CUD]
- [Order_tracking_signal](#order_tracking_signal) [C]
- [File_upload](#file_upload) [R]
- [Data_source](#data_source) [CRUD]
- [Lfp_merchant_state](#lfp_merchant_state) [R]
- [Lfp_inventorie](#lfp_inventorie) [C]
- [Lfp_store](#lfp_store) [CRD]
- [Lfp_sale](#lfp_sale) [C]
- [Conversion_source](#conversion_source) [CRUD]
- [Aggregate_product_statuse](#aggregate_product_statuse) [R]
- [Issueresolution](#issueresolution) [C]
- [Order_tracking_signal](#order_tracking_signal) [C]
- [Promotion](#promotion) [CR]
- [Report](#report) [C]
- [Aggregate_product_statuse](#aggregate_product_statuse) [R]
- [Issueresolution](#issueresolution) [C]
- [Promotion](#promotion) [CR]
- [Terms_of_service_agreement_state](#terms_of_service_agreement_state) [R]
- [Email_preference](#email_preference) [RU]
- [Shipping_setting](#shipping_setting) [CR]
- [Developer_registration](#developer_registration) [CR]
- [Account](#account) [CRUD]
- [Relationship](#relationship) [RU]
- [Gbp_account](#gbp_account) [CR]
- [Business_info](#business_info) [RU]
- [Homepage](#homepage) [CRU]
- [Online_return_policie](#online_return_policie) [CRD]
- [Terms_of_service](#terms_of_service) [CR]
- [Omnichannel_setting](#omnichannel_setting) [CRU]
- [Automatic_improvement](#automatic_improvement) [RU]
- [Program](#program) [CR]
- [Service](#service) [CR]
- [Business_identity](#business_identity) [RU]
- [User](#user) [CRUD]
- [Autofeed_setting](#autofeed_setting) [RU]
- [Issue](#issue) [R]
- [Checkout_setting](#checkout_setting) [CRUD]
- [Lfp_provider](#lfp_provider) [CR]
- [Region](#region) [CRUD]
- [Product](#product) [R]
- [Product_input](#product_input) [CUD]
- [Notificationsubscription](#notificationsubscription) [CRUD]
- [Account](#account) [CRUD]
- [Terms_of_service](#terms_of_service) [CR]
- [Omnichannel_setting](#omnichannel_setting) [CRU]
- [Business_identity](#business_identity) [RU]
- [Autofeed_setting](#autofeed_setting) [RU]
- [Checkout_setting](#checkout_setting) [CRUD]
- [Automatic_improvement](#automatic_improvement) [RU]
- [Service](#service) [CR]
- [Program](#program) [CR]
- [Developer_registration](#developer_registration) [CR]
- [Region](#region) [CRUD]
- [Email_preference](#email_preference) [RU]
- [Gbp_account](#gbp_account) [CR]
- [Business_info](#business_info) [RU]
- [User](#user) [CRUD]
- [Homepage](#homepage) [CRU]
- [Online_return_policie](#online_return_policie) [CRUD]
- [Terms_of_service_agreement_state](#terms_of_service_agreement_state) [R]
- [Shipping_setting](#shipping_setting) [CR]
- [Relationship](#relationship) [RU]
- [Lfp_provider](#lfp_provider) [CR]
- [Issue](#issue) [R]
- [Merchant_review](#merchant_review) [CRD]
- [Product_review](#product_review) [CRD]
- [Quota](#quota) [R]
- [Conversion_source](#conversion_source) [CRUD]
- [Local_inventorie](#local_inventorie) [CRD]
- [Regional_inventorie](#regional_inventorie) [CRD]
- [Notificationsubscription](#notificationsubscription) [CRUD]
- [Lfp_merchant_state](#lfp_merchant_state) [R]
- [Lfp_sale](#lfp_sale) [C]
- [Lfp_store](#lfp_store) [CRD]
- [Lfp_inventorie](#lfp_inventorie) [C]
- [Report](#report) [C]

---

## Resources


### File_upload

Gets the latest data source file upload. Only the `latest` alias is accepted for a file upload.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_source_id` | String | Output only. The data source id. |
| `issues` | Vec<String> | Output only. The list of issues occurring in the data source. |
| `name` | String | Identifier. The name of the data source file upload. Format: `{datasource.name=accounts/{account}/dataSources/{datasource}/fileUploads/{fileupload}}` |
| `items_updated` | String | Output only. The number of items in the data source that were updated. |
| `processing_state` | String | Output only. The processing state of the data source. |
| `upload_time` | String | Output only. The date at which the file of the data source was uploaded. |
| `items_created` | String | Output only. The number of items in the data source that were created. |
| `items_total` | String | Output only. The number of items in the data source that were processed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access file_upload outputs
file_upload_id = file_upload.id
file_upload_data_source_id = file_upload.data_source_id
file_upload_issues = file_upload.issues
file_upload_name = file_upload.name
file_upload_items_updated = file_upload.items_updated
file_upload_processing_state = file_upload.processing_state
file_upload_upload_time = file_upload.upload_time
file_upload_items_created = file_upload.items_created
file_upload_items_total = file_upload.items_total
```

---


### Data_source

Creates the new data source configuration for the given account. This method always creates a new data source.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_input` | String |  | Optional. The field is used only when data is managed through a file. |
| `input` | String |  | Output only. Determines the type of input to the data source. Based on the input some settings might not work. Only generic data sources can be created through the API. |
| `local_inventory_data_source` | String |  | The [local inventory](https://support.google.com/merchants/answer/7023001) data source. |
| `primary_product_data_source` | String |  | The [primary data source](https://support.google.com/merchants/answer/7439058) for local and online products. |
| `merchant_review_data_source` | String |  | The [merchant review](https://support.google.com/merchants/answer/7045996) data source. |
| `product_review_data_source` | String |  | The [product review](https://support.google.com/merchants/answer/7045996) data source. |
| `promotion_data_source` | String |  | The [promotion](https://support.google.com/merchants/answer/2906014) data source. |
| `supplemental_product_data_source` | String |  | The [supplemental data source](https://support.google.com/merchants/answer/7439058) for local and online products. |
| `display_name` | String |  | Required. The displayed data source name in the Merchant Center UI. |
| `data_source_id` | String |  | Output only. The data source id. |
| `name` | String |  | Required. Identifier. The name of the data source. Format: `accounts/{account}/dataSources/{datasource}` |
| `regional_inventory_data_source` | String |  | The [regional inventory](https://support.google.com/merchants/answer/7439058) data source. |
| `parent` | String | ✅ | Required. The account where this data source will be created. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_input` | String | Optional. The field is used only when data is managed through a file. |
| `input` | String | Output only. Determines the type of input to the data source. Based on the input some settings might not work. Only generic data sources can be created through the API. |
| `local_inventory_data_source` | String | The [local inventory](https://support.google.com/merchants/answer/7023001) data source. |
| `primary_product_data_source` | String | The [primary data source](https://support.google.com/merchants/answer/7439058) for local and online products. |
| `merchant_review_data_source` | String | The [merchant review](https://support.google.com/merchants/answer/7045996) data source. |
| `product_review_data_source` | String | The [product review](https://support.google.com/merchants/answer/7045996) data source. |
| `promotion_data_source` | String | The [promotion](https://support.google.com/merchants/answer/2906014) data source. |
| `supplemental_product_data_source` | String | The [supplemental data source](https://support.google.com/merchants/answer/7439058) for local and online products. |
| `display_name` | String | Required. The displayed data source name in the Merchant Center UI. |
| `data_source_id` | String | Output only. The data source id. |
| `name` | String | Required. Identifier. The name of the data source. Format: `accounts/{account}/dataSources/{datasource}` |
| `regional_inventory_data_source` | String | The [regional inventory](https://support.google.com/merchants/answer/7439058) data source. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_source
data_source = provider.merchantapi_api.Data_source {
    parent = "value"  # Required. The account where this data source will be created. Format: `accounts/{account}`
}

# Access data_source outputs
data_source_id = data_source.id
data_source_file_input = data_source.file_input
data_source_input = data_source.input
data_source_local_inventory_data_source = data_source.local_inventory_data_source
data_source_primary_product_data_source = data_source.primary_product_data_source
data_source_merchant_review_data_source = data_source.merchant_review_data_source
data_source_product_review_data_source = data_source.product_review_data_source
data_source_promotion_data_source = data_source.promotion_data_source
data_source_supplemental_product_data_source = data_source.supplemental_product_data_source
data_source_display_name = data_source.display_name
data_source_data_source_id = data_source.data_source_id
data_source_name = data_source.name
data_source_regional_inventory_data_source = data_source.regional_inventory_data_source
```

---


### Regional_inventorie

Inserts a `RegionalInventory` to a given product in your merchant account. Replaces the full `RegionalInventory` resource if an entry with the same `region` already exists for the product. It might take up to 30 minutes for the new or updated `RegionalInventory` resource to appear in products.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account` | String |  | Output only. The account that owns the product. This field will be ignored if set by the client. |
| `custom_attributes` | Vec<String> |  | A list of custom (merchant-provided) attributes. You can also use `CustomAttribute` to submit any attribute of the data specification in its generic form. |
| `region` | String |  | Required. Immutable. ID of the region for this `RegionalInventory` resource. See the [Regional availability and pricing](https://support.google.com/merchants/answer/9698880) for more details. |
| `price` | String |  | Optional. Price of the product in this region. |
| `name` | String |  | Output only. The name of the `RegionalInventory` resource. Format: `{regional_inventory.name=accounts/{account}/products/{product}/regionalInventories/{region}` |
| `availability` | String |  | Availability of the product in this region. For accepted attribute values, see the [regional product inventory data specification](https://support.google.com/merchants/answer/14644124). |
| `sale_price_effective_date` | String |  | Optional. The `TimePeriod` of the sale price in this region. |
| `sale_price` | String |  | Optional. Sale price of the product in this region. Mandatory if `salePriceEffectiveDate` is defined. |
| `parent` | String | ✅ | Required. The account and product where this inventory will be inserted. Format: `accounts/{account}/products/{product}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `regional_inventories` | Vec<String> | The `RegionalInventory` resources for the given product from the specified account. |
| `next_page_token` | String | A token, which can be sent as `pageToken` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create regional_inventorie
regional_inventorie = provider.merchantapi_api.Regional_inventorie {
    parent = "value"  # Required. The account and product where this inventory will be inserted. Format: `accounts/{account}/products/{product}`
}

# Access regional_inventorie outputs
regional_inventorie_id = regional_inventorie.id
regional_inventorie_regional_inventories = regional_inventorie.regional_inventories
regional_inventorie_next_page_token = regional_inventorie.next_page_token
```

---


### Local_inventorie

Inserts a `LocalInventory` resource to a product in your merchant account. Replaces the full `LocalInventory` resource if an entry with the same `storeCode` already exists for the product. It might take up to 30 minutes for the new or updated `LocalInventory` resource to appear in products.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pickup_sla` | String |  | Relative time period from the order date for an order for this product, from this store, to be ready for pickup. Must be submitted with `pickupMethod`. For accepted attribute values, see the [local product inventory data specification](https://support.google.com/merchants/answer/3061342) |
| `account` | String |  | Output only. The account that owns the product. This field will be ignored if set by the client. |
| `custom_attributes` | Vec<String> |  | A list of custom (merchant-provided) attributes. You can also use `CustomAttribute` to submit any attribute of the data specification in its generic form. |
| `name` | String |  | Output only. The name of the `LocalInventory` resource. Format: `accounts/{account}/products/{product}/localInventories/{store_code}` |
| `price` | String |  | Optional. Price of the product at this store. |
| `quantity` | String |  | Quantity of the product available at this store. Must be greater than or equal to zero. |
| `availability` | String |  | Availability of the product at this store. For accepted attribute values, see the [local product inventory data specification](https://support.google.com/merchants/answer/3061342) |
| `sale_price` | String |  | Optional. Sale price of the product at this store. Mandatory if `salePriceEffectiveDate` is defined. |
| `sale_price_effective_date` | String |  | Optional. The `TimePeriod` of the sale at this store. |
| `instore_product_location` | String |  | Location of the product inside the store. Maximum length is 20 bytes. |
| `store_code` | String |  | Required. Immutable. Store code (the store ID from your Business Profile) of the physical store the product is sold in. See the [Local product inventory data specification](https://support.google.com/merchants/answer/3061342) for more information. |
| `pickup_method` | String |  | Supported pickup method for this product. Unless the value is `"not supported"`, this field must be submitted together with `pickupSla`. For accepted attribute values, see the [local product inventory data specification](https://support.google.com/merchants/answer/3061342) |
| `parent` | String | ✅ | Required. The account and product where this inventory will be inserted. Format: `accounts/{account}/products/{product}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `local_inventories` | Vec<String> | The `LocalInventory` resources for the given product from the specified account. |
| `next_page_token` | String | A token, which can be sent as `pageToken` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create local_inventorie
local_inventorie = provider.merchantapi_api.Local_inventorie {
    parent = "value"  # Required. The account and product where this inventory will be inserted. Format: `accounts/{account}/products/{product}`
}

# Access local_inventorie outputs
local_inventorie_id = local_inventorie.id
local_inventorie_local_inventories = local_inventorie.local_inventories
local_inventorie_next_page_token = local_inventorie.next_page_token
```

---


### Quota

Lists the daily call quota and usage per group for your Merchant Center account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `quota_groups` | Vec<String> | The methods, current quota usage and limits per each group. The quota is shared between all methods in the group. The groups are sorted in descending order based on quota_usage. |


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
quota_quota_groups = quota.quota_groups
```

---


### Product

Retrieves the processed product from your Merchant Center account. After inserting, updating, or deleting a product input, it may take several minutes before the updated final product can be retrieved.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the product. Format: `accounts/{account}/products/{product}` where the last section `product` consists of: `content_language~feed_label~offer_id` example for product name is `accounts/123/products/en~US~sku123`. A legacy local product name would be `accounts/123/products/local~en~US~sku123`. Note: For calls to the v1beta version, the `product` section consists of: `channel~content_language~feed_label~offer_id`, for example: `accounts/123/products/online~en~US~sku123`. |
| `data_source` | String | Output only. The primary data source of the product. |
| `product_attributes` | String | Output only. A list of strongly-typed product attributes. |
| `content_language` | String | Output only. The two-letter [ISO 639-1](http://en.wikipedia.org/wiki/ISO_639-1) language code for the product. |
| `feed_label` | String | Output only. The feed label lets you categorize and identify your products. The maximum allowed characters is 20 and the supported characters are`A-Z`, `0-9`, hyphen and underscore. The feed label must not include any spaces. For more information, see [Using feed labels](//support.google.com/merchants/answer/14994087) |
| `custom_attributes` | Vec<String> | Output only. A list of custom (merchant-provided) attributes. It can also be used to submit any attribute of the data specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google. |
| `offer_id` | String | Output only. Your unique identifier for the product. This is the same for the product input and processed product. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. See the [product data specification](https://support.google.com/merchants/answer/188494#id) for details. |
| `automated_discounts` | String | Output only. The automated discounts information for the product. |
| `product_status` | String | Output only. The status of a product, data validation issues, that is, information about a product computed asynchronously. |
| `version_number` | String | Output only. Represents the existing version (freshness) of the product, which can be used to preserve the right order when multiple updates are done at the same time. If set, the insertion is prevented when version number is lower than the current version number of the existing product. Re-insertion (for example, product refresh after 30 days) can be performed with the current `version_number`. Only supported for insertions into primary data sources. If the operation is prevented, the aborted exception will be thrown. |
| `legacy_local` | bool | Output only. Determines whether the product is **only** targeting local destinations and whether the product name should be distinguished with a `local~` prefix. For example, `accounts/123/products/local~en~US~sku123`. |


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
product_name = product.name
product_data_source = product.data_source
product_product_attributes = product.product_attributes
product_content_language = product.content_language
product_feed_label = product.feed_label
product_custom_attributes = product.custom_attributes
product_offer_id = product.offer_id
product_automated_discounts = product.automated_discounts
product_product_status = product.product_status
product_version_number = product.version_number
product_legacy_local = product.legacy_local
```

---


### Product_input

[Uploads a product input to your Merchant Center account](/merchant/api/guides/products/add-manage#add_a_product). You must have a products [data source](/merchant/api/guides/data-sources/api-sources#create-primary-data-source) to be able to insert a product. The unique identifier of the data source is passed as a query parameter in the request URL. If a product input with the same contentLanguage, offerId, and dataSource already exists, then the product input inserted by this method replaces that entry. After inserting, updating, or deleting a product input, it may take several minutes before the processed product can be retrieved.

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_attributes` | String |  | Optional. A list of strongly-typed product attributes. |
| `offer_id` | String |  | Required. Immutable. Your unique identifier for the product. This is the same for the product input and processed product. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. See the [products data specification](https://support.google.com/merchants/answer/188494#id) for details. |
| `name` | String |  | Identifier. The name of the product input. Format: `accounts/{account}/productInputs/{productinput}` where the last section `productinput` consists of: `content_language~feed_label~offer_id` example for product input name is `accounts/123/productInputs/en~US~sku123`. A legacy local product input name would be `accounts/123/productInputs/local~en~US~sku123`. Note: For calls to the v1beta version, the `productInput` section consists of: `channel~content_language~feed_label~offer_id`, for example: `accounts/123/productInputs/online~en~US~sku123`. |
| `legacy_local` | bool |  | Immutable. Determines whether the product is **only** targeting local destinations and whether the product name should be distinguished with a `local~` prefix. For example, `accounts/123/productInputs/local~en~US~sku123`. If a product that is not `legacy_local` is already targeting local destinations, creating a `legacy_local` product with an otherwise matching name will fail. |
| `content_language` | String |  | Required. Immutable. The two-letter [ISO 639-1](http://en.wikipedia.org/wiki/ISO_639-1) language code for the product. |
| `feed_label` | String |  | Required. Immutable. The feed label that lets you categorize and identify your products. The maximum allowed characters are 20, and the supported characters are `A-Z`, `0-9`, hyphen, and underscore. The feed label must not include any spaces. For more information, see [Using feed labels](//support.google.com/merchants/answer/14994087). |
| `version_number` | String |  | Optional. Immutable. Represents the existing version (freshness) of the product, which can be used to preserve the right order when multiple updates are done at the same time. If set, the insertion is prevented when version number is lower than the current version number of the existing product. Re-insertion (for example, product refresh after 30 days) can be performed with the current `version_number`. Only supported for insertions into primary data sources. Do not set this field for updates. Do not set this field for insertions into supplemental data sources. If the operation is prevented, the aborted exception will be thrown. |
| `custom_attributes` | Vec<String> |  | Optional. A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the data specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API. Maximum allowed number of characters for each custom attribute is 10240 (represents sum of characters for name and value). Maximum 2500 custom attributes can be set per product, with total size of 102.4kB. Underscores in custom attribute names are replaced by spaces upon insertion. |
| `product` | String |  | Output only. The name of the processed product. Format: `accounts/{account}/products/{product}` |
| `parent` | String | ✅ | Required. The account where this product will be inserted. Format: `accounts/{account}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product_input
product_input = provider.merchantapi_api.Product_input {
    parent = "value"  # Required. The account where this product will be inserted. Format: `accounts/{account}`
}

```

---


### Order_tracking_signal

Creates new order tracking signal.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `line_items` | Vec<String> |  | Required. Information about line items in the order. |
| `shipment_line_item_mapping` | Vec<String> |  | Optional. The mapping of the line items to the shipment information. |
| `shipping_info` | Vec<String> |  | Required. The shipping information for the order. |
| `order_created_time` | String |  | Required. The time when the order was created on the businesses side. Include the year and timezone string, if available. |
| `delivery_postal_code` | String |  | Optional. The delivery postal code, as a continuous string without spaces or dashes, for example "95016". This field will be anonymized in returned OrderTrackingSignal creation response. |
| `merchant_id` | String |  | Optional. The Google Merchant Center ID of this order tracking signal. This value is optional. If left unset, the caller's Merchant Center ID is used. You must request access in order to provide data on behalf of another business. For more information, see [Submitting Order Tracking Signals](/shopping-content/guides/order-tracking-signals). |
| `delivery_region_code` | String |  | Optional. The [CLDR territory code] (http://www.unicode.org/repos/cldr/tags/latest/common/main/en.xml) for the shipping destination. |
| `order_tracking_signal_id` | String |  | Output only. The ID that uniquely identifies this order tracking signal. |
| `customer_shipping_fee` | String |  | Optional. The shipping fee of the order; this value should be set to zero in the case of free shipping. |
| `order_id` | String |  | Required. The ID of the order on the businesses side. This field will be hashed in returned OrderTrackingSignal creation response. |
| `parent` | String | ✅ | Required. The account of the business for which the order signal is created. Format: accounts/{account} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create order_tracking_signal
order_tracking_signal = provider.merchantapi_api.Order_tracking_signal {
    parent = "value"  # Required. The account of the business for which the order signal is created. Format: accounts/{account}
}

```

---


### File_upload

Gets the latest data source file upload. Only the `latest` alias is accepted for a file upload.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `processing_state` | String | Output only. The processing state of the data source. |
| `items_updated` | String | Output only. The number of items in the data source that were updated. |
| `items_created` | String | Output only. The number of items in the data source that were created. |
| `upload_time` | String | Output only. The date at which the file of the data source was uploaded. |
| `items_total` | String | Output only. The number of items in the data source that were processed. |
| `data_source_id` | String | Output only. The data source id. |
| `issues` | Vec<String> | Output only. The list of issues occurring in the data source. |
| `name` | String | Identifier. The name of the data source file upload. Format: `{datasource.name=accounts/{account}/dataSources/{datasource}/fileUploads/{fileupload}}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access file_upload outputs
file_upload_id = file_upload.id
file_upload_processing_state = file_upload.processing_state
file_upload_items_updated = file_upload.items_updated
file_upload_items_created = file_upload.items_created
file_upload_upload_time = file_upload.upload_time
file_upload_items_total = file_upload.items_total
file_upload_data_source_id = file_upload.data_source_id
file_upload_issues = file_upload.issues
file_upload_name = file_upload.name
```

---


### Data_source

Creates the new data source configuration for the given account. This method always creates a new data source.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_input` | String |  | Optional. The field is used only when data is managed through a file. |
| `display_name` | String |  | Required. The displayed data source name in the Merchant Center UI. |
| `primary_product_data_source` | String |  | The [primary data source](https://support.google.com/merchants/answer/7439058) for local and online products. |
| `data_source_id` | String |  | Output only. The data source id. |
| `product_review_data_source` | String |  | The [product review](https://support.google.com/merchants/answer/7045996) data source. |
| `name` | String |  | Required. Identifier. The name of the data source. Format: `accounts/{account}/dataSources/{datasource}` |
| `promotion_data_source` | String |  | The [promotion](https://support.google.com/merchants/answer/2906014) data source. |
| `regional_inventory_data_source` | String |  | The [regional inventory](https://support.google.com/merchants/answer/7439058) data source. |
| `supplemental_product_data_source` | String |  | The [supplemental data source](https://support.google.com/merchants/answer/7439058) for local and online products. |
| `input` | String |  | Output only. Determines the type of input to the data source. Based on the input some settings might not work. Only generic data sources can be created through the API. |
| `merchant_review_data_source` | String |  | The [merchant review](https://support.google.com/merchants/answer/7045996) data source. |
| `local_inventory_data_source` | String |  | The [local inventory](https://support.google.com/merchants/answer/7023001) data source. |
| `parent` | String | ✅ | Required. The account where this data source will be created. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_input` | String | Optional. The field is used only when data is managed through a file. |
| `display_name` | String | Required. The displayed data source name in the Merchant Center UI. |
| `primary_product_data_source` | String | The [primary data source](https://support.google.com/merchants/answer/7439058) for local and online products. |
| `data_source_id` | String | Output only. The data source id. |
| `product_review_data_source` | String | The [product review](https://support.google.com/merchants/answer/7045996) data source. |
| `name` | String | Required. Identifier. The name of the data source. Format: `accounts/{account}/dataSources/{datasource}` |
| `promotion_data_source` | String | The [promotion](https://support.google.com/merchants/answer/2906014) data source. |
| `regional_inventory_data_source` | String | The [regional inventory](https://support.google.com/merchants/answer/7439058) data source. |
| `supplemental_product_data_source` | String | The [supplemental data source](https://support.google.com/merchants/answer/7439058) for local and online products. |
| `input` | String | Output only. Determines the type of input to the data source. Based on the input some settings might not work. Only generic data sources can be created through the API. |
| `merchant_review_data_source` | String | The [merchant review](https://support.google.com/merchants/answer/7045996) data source. |
| `local_inventory_data_source` | String | The [local inventory](https://support.google.com/merchants/answer/7023001) data source. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_source
data_source = provider.merchantapi_api.Data_source {
    parent = "value"  # Required. The account where this data source will be created. Format: `accounts/{account}`
}

# Access data_source outputs
data_source_id = data_source.id
data_source_file_input = data_source.file_input
data_source_display_name = data_source.display_name
data_source_primary_product_data_source = data_source.primary_product_data_source
data_source_data_source_id = data_source.data_source_id
data_source_product_review_data_source = data_source.product_review_data_source
data_source_name = data_source.name
data_source_promotion_data_source = data_source.promotion_data_source
data_source_regional_inventory_data_source = data_source.regional_inventory_data_source
data_source_supplemental_product_data_source = data_source.supplemental_product_data_source
data_source_input = data_source.input
data_source_merchant_review_data_source = data_source.merchant_review_data_source
data_source_local_inventory_data_source = data_source.local_inventory_data_source
```

---


### Lfp_merchant_state

Gets the LFP state of a merchant

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `linked_gbps` | String | Number of [GBPs](https://www.google.com/business/) this merchant has access to. |
| `country_settings` | Vec<String> | Country-specific settings for the merchant. |
| `store_states` | Vec<String> | Output only. The state per store from the specified merchant. The field will be absent if the merchant has no stores submitted through LFP. |
| `inventory_stats` | String | The inventory statistics for the merchant. The field will be absent if the merchant has no inventory submitted through LFP. |
| `name` | String | Identifier. The name of the `LfpMerchantState` resource. Format: `accounts/{account}/lfpMerchantStates/{target_merchant}`. For example, `accounts/123456/lfpMerchantStates/567890`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access lfp_merchant_state outputs
lfp_merchant_state_id = lfp_merchant_state.id
lfp_merchant_state_linked_gbps = lfp_merchant_state.linked_gbps
lfp_merchant_state_country_settings = lfp_merchant_state.country_settings
lfp_merchant_state_store_states = lfp_merchant_state.store_states
lfp_merchant_state_inventory_stats = lfp_merchant_state.inventory_stats
lfp_merchant_state_name = lfp_merchant_state.name
```

---


### Lfp_inventorie

Inserts a `LfpInventory` resource for the given target merchant account. If the resource already exists, it will be replaced. The inventory automatically expires after 30 days.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gtin` | String |  | Optional. The Global Trade Item Number of the product. |
| `feed_label` | String |  | Optional. The [feed label](https://developers.google.com/shopping-content/guides/products/feed-labels) for the product. If this is not set, it will default to `regionCode`. |
| `offer_id` | String |  | Required. Immutable. A unique identifier for the product. If both inventories and sales are submitted for a merchant, this id should match for the same product. **Note**: if the merchant sells the same product new and used, they should have different IDs. |
| `price` | String |  | Optional. The current price of the product. |
| `name` | String |  | Output only. Identifier. The name for the `LfpInventory` resource. Format: `accounts/{account}/lfpInventories/{target_merchant}~{store_code}~{offer}` |
| `region_code` | String |  | Required. The [CLDR territory code](https://github.com/unicode-org/cldr/blob/latest/common/main/en.xml) for the country where the product is sold. |
| `pickup_method` | String |  | Optional. Supported pickup method for this offer. Unless the value is "not supported", this field must be submitted together with `pickupSla`. For accepted attribute values, see the [local product inventory data specification](https://support.google.com/merchants/answer/3061342). |
| `pickup_sla` | String |  | Optional. Expected date that an order will be ready for pickup relative to the order date. Must be submitted together with `pickupMethod`. For accepted attribute values, see the [local product inventory data specification](https://support.google.com/merchants/answer/3061342). |
| `quantity` | String |  | Optional. Quantity of the product available at this store. Must be greater than or equal to zero. |
| `store_code` | String |  | Required. The identifier of the merchant's store. Either the store code inserted through `InsertLfpStore` or the store code in the Business Profile. |
| `content_language` | String |  | Required. The two-letter ISO 639-1 language code for the item. |
| `availability` | String |  | Required. Availability of the product at this store. For accepted attribute values, see the [local product inventory data specification](https://support.google.com/merchants/answer/3061342) |
| `target_account` | String |  | Required. The Merchant Center ID of the merchant to submit the inventory for. |
| `collection_time` | String |  | Optional. The time when the inventory is collected. If not set, it will be set to the time when the inventory is submitted. |
| `parent` | String | ✅ | Required. The LFP provider account. Format: `accounts/{account}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lfp_inventorie
lfp_inventorie = provider.merchantapi_api.Lfp_inventorie {
    parent = "value"  # Required. The LFP provider account. Format: `accounts/{account}`
}

```

---


### Lfp_store

Inserts a store for the target merchant. If the store with the same store code already exists, it will be replaced.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Identifier. The name of the `LfpStore` resource. Format: `accounts/{account}/lfpStores/{target_merchant}~{store_code}` |
| `matching_state` | String |  | Optional. Output only. The state of matching to a Google Business Profile. See matchingStateHint for further details if no match is found. |
| `store_code` | String |  | Required. Immutable. A store identifier that is unique for the target merchant. |
| `website_uri` | String |  | Optional. The website URL for the store or merchant. |
| `target_account` | String |  | Required. The Merchant Center id of the merchant to submit the store for. |
| `place_id` | String |  | Optional. The [Google Place Id](https://developers.google.com/maps/documentation/places/web-service/place-id#id-overview) of the store location. |
| `matching_state_hint` | String |  | Optional. Output only. The hint of why the matching has failed. This is only set when matchingState=`STORE_MATCHING_STATE_FAILED`. Possible values are: - "`linked-store-not-found`": There aren't any Google Business Profile stores available for matching. - "`store-match-not-found`": The provided `LfpStore` couldn't be matched to any of the connected Google Business Profile stores. Merchant Center account is connected correctly and stores are available on Google Business Profile, but the `LfpStore` location address does not match with Google Business Profile stores' addresses. Update the `LfpStore` address or Google Business Profile store address to match correctly. - "`store-match-unverified`": The provided `LfpStore` couldn't be matched to any of the connected Google Business Profile stores, as the matched Google Business Profile store is unverified. Go through the Google Business Profile verification process to match correctly. |
| `gcid_category` | Vec<String> |  | Optional. [Google My Business category id](https://support.google.com/business/answer/7249669). |
| `phone_number` | String |  | Optional. The store phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. Example: `+15556767888` |
| `store_name` | String |  | Optional. The merchant or store name. |
| `store_address` | String |  | Required. The street address of the store. Example: 1600 Amphitheatre Pkwy, Mountain View, CA 94043, USA. |
| `parent` | String | ✅ | Required. The LFP provider account Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Identifier. The name of the `LfpStore` resource. Format: `accounts/{account}/lfpStores/{target_merchant}~{store_code}` |
| `matching_state` | String | Optional. Output only. The state of matching to a Google Business Profile. See matchingStateHint for further details if no match is found. |
| `store_code` | String | Required. Immutable. A store identifier that is unique for the target merchant. |
| `website_uri` | String | Optional. The website URL for the store or merchant. |
| `target_account` | String | Required. The Merchant Center id of the merchant to submit the store for. |
| `place_id` | String | Optional. The [Google Place Id](https://developers.google.com/maps/documentation/places/web-service/place-id#id-overview) of the store location. |
| `matching_state_hint` | String | Optional. Output only. The hint of why the matching has failed. This is only set when matchingState=`STORE_MATCHING_STATE_FAILED`. Possible values are: - "`linked-store-not-found`": There aren't any Google Business Profile stores available for matching. - "`store-match-not-found`": The provided `LfpStore` couldn't be matched to any of the connected Google Business Profile stores. Merchant Center account is connected correctly and stores are available on Google Business Profile, but the `LfpStore` location address does not match with Google Business Profile stores' addresses. Update the `LfpStore` address or Google Business Profile store address to match correctly. - "`store-match-unverified`": The provided `LfpStore` couldn't be matched to any of the connected Google Business Profile stores, as the matched Google Business Profile store is unverified. Go through the Google Business Profile verification process to match correctly. |
| `gcid_category` | Vec<String> | Optional. [Google My Business category id](https://support.google.com/business/answer/7249669). |
| `phone_number` | String | Optional. The store phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. Example: `+15556767888` |
| `store_name` | String | Optional. The merchant or store name. |
| `store_address` | String | Required. The street address of the store. Example: 1600 Amphitheatre Pkwy, Mountain View, CA 94043, USA. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lfp_store
lfp_store = provider.merchantapi_api.Lfp_store {
    parent = "value"  # Required. The LFP provider account Format: `accounts/{account}`
}

# Access lfp_store outputs
lfp_store_id = lfp_store.id
lfp_store_name = lfp_store.name
lfp_store_matching_state = lfp_store.matching_state
lfp_store_store_code = lfp_store.store_code
lfp_store_website_uri = lfp_store.website_uri
lfp_store_target_account = lfp_store.target_account
lfp_store_place_id = lfp_store.place_id
lfp_store_matching_state_hint = lfp_store.matching_state_hint
lfp_store_gcid_category = lfp_store.gcid_category
lfp_store_phone_number = lfp_store.phone_number
lfp_store_store_name = lfp_store.store_name
lfp_store_store_address = lfp_store.store_address
```

---


### Lfp_sale

Inserts a `LfpSale` for the given merchant.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `price` | String |  | Required. The unit price of the product. |
| `sale_time` | String |  | Required. The timestamp for the sale. |
| `store_code` | String |  | Required. The identifier of the merchant's store. Either a `storeCode` inserted through the API or the code of the store in the Business Profile. |
| `region_code` | String |  | Required. The [CLDR territory code](https://github.com/unicode-org/cldr/blob/latest/common/main/en.xml) for the country where the product is sold. |
| `uid` | String |  | Output only. System generated globally unique ID for the `LfpSale`. |
| `quantity` | String |  | Required. The relative change of the available quantity. Negative for items returned. |
| `gtin` | String |  | Required. The Global Trade Item Number of the sold product. |
| `feed_label` | String |  | Optional. The [feed label](https://developers.google.com/shopping-content/guides/products/feed-labels) for the product. If this is not set, it will default to `regionCode`. |
| `content_language` | String |  | Required. The two-letter ISO 639-1 language code for the item. |
| `target_account` | String |  | Required. The Merchant Center ID of the merchant to submit the sale for. |
| `offer_id` | String |  | Required. A unique identifier for the product. If both inventories and sales are submitted for a merchant, this id should match for the same product. **Note**: if the merchant sells the same product new and used, they should have different IDs. |
| `name` | String |  | Output only. Identifier. The name of the `LfpSale` resource. Format: `accounts/{account}/lfpSales/{sale}` |
| `parent` | String | ✅ | Required. The LFP provider account. Format: `accounts/{lfp_partner}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lfp_sale
lfp_sale = provider.merchantapi_api.Lfp_sale {
    parent = "value"  # Required. The LFP provider account. Format: `accounts/{lfp_partner}`
}

```

---


### Conversion_source

Creates a new conversion source.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `merchant_center_destination` | String |  | Conversion Source of type "Merchant Center Tag Destination". |
| `name` | String |  | Output only. Identifier. Generated by the Content API upon creation of a new `ConversionSource`. Format: `[a-z]{4}:.+` The four characters before the colon represent the type of conversion source. Content after the colon represents the ID of the conversion source within that type. The ID of two different conversion sources might be the same across different types. The following type prefixes are supported: * `galk`: For GoogleAnalyticsLink sources. * `mcdn`: For MerchantCenterDestination sources. |
| `state` | String |  | Output only. Current state of this conversion source. Can't be edited through the API. |
| `controller` | String |  | Output only. Controller of the conversion source. |
| `expire_time` | String |  | Output only. The time when an archived conversion source becomes permanently deleted and is no longer available to undelete. |
| `google_analytics_link` | String |  | Immutable. Conversion Source of type "Link to Google Analytics Property". |
| `parent` | String | ✅ | Required. The merchant account that will own the new conversion source. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `merchant_center_destination` | String | Conversion Source of type "Merchant Center Tag Destination". |
| `name` | String | Output only. Identifier. Generated by the Content API upon creation of a new `ConversionSource`. Format: `[a-z]{4}:.+` The four characters before the colon represent the type of conversion source. Content after the colon represents the ID of the conversion source within that type. The ID of two different conversion sources might be the same across different types. The following type prefixes are supported: * `galk`: For GoogleAnalyticsLink sources. * `mcdn`: For MerchantCenterDestination sources. |
| `state` | String | Output only. Current state of this conversion source. Can't be edited through the API. |
| `controller` | String | Output only. Controller of the conversion source. |
| `expire_time` | String | Output only. The time when an archived conversion source becomes permanently deleted and is no longer available to undelete. |
| `google_analytics_link` | String | Immutable. Conversion Source of type "Link to Google Analytics Property". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversion_source
conversion_source = provider.merchantapi_api.Conversion_source {
    parent = "value"  # Required. The merchant account that will own the new conversion source. Format: `accounts/{account}`
}

# Access conversion_source outputs
conversion_source_id = conversion_source.id
conversion_source_merchant_center_destination = conversion_source.merchant_center_destination
conversion_source_name = conversion_source.name
conversion_source_state = conversion_source.state
conversion_source_controller = conversion_source.controller
conversion_source_expire_time = conversion_source.expire_time
conversion_source_google_analytics_link = conversion_source.google_analytics_link
```

---


### Aggregate_product_statuse

Lists the `AggregateProductStatuses` resources for your merchant account. The response might contain fewer items than specified by `pageSize`. If `pageToken` was returned in previous request, it can be used to obtain additional results.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `pageToken` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `aggregate_product_statuses` | Vec<String> | The `AggregateProductStatuses` resources for the given account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access aggregate_product_statuse outputs
aggregate_product_statuse_id = aggregate_product_statuse.id
aggregate_product_statuse_next_page_token = aggregate_product_statuse.next_page_token
aggregate_product_statuse_aggregate_product_statuses = aggregate_product_statuse.aggregate_product_statuses
```

---


### Issueresolution

Provide a list of issues for business's product with an issue resolution content and available actions. This content and actions are meant to be rendered and shown in third-party applications.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content_option` | String |  | Optional. How the detailed content should be returned. Default option is to return the content as a pre-rendered HTML text. |
| `user_input_action_option` | String |  | Optional. How actions with user input form should be handled. If not provided, actions will be returned as links that points the business to Merchant Center where they can request the action. |
| `name` | String | ✅ | Required. The name of the product to fetch issues for. Format: `accounts/{account}/products/{product}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create issueresolution
issueresolution = provider.merchantapi_api.Issueresolution {
    name = "value"  # Required. The name of the product to fetch issues for. Format: `accounts/{account}/products/{product}`
}

```

---


### Order_tracking_signal

Creates new order tracking signal.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `order_tracking_signal_id` | String |  | Output only. The ID that uniquely identifies this order tracking signal. |
| `shipping_info` | Vec<String> |  | Required. The shipping information for the order. |
| `order_id` | String |  | Required. The ID of the order on the businesses side. This field will be hashed in returned OrderTrackingSignal creation response. |
| `customer_shipping_fee` | String |  | Optional. The shipping fee of the order; this value should be set to zero in the case of free shipping. |
| `merchant_id` | String |  | Optional. The Google Merchant Center ID of this order tracking signal. This value is optional. If left unset, the caller's Merchant Center ID is used. You must request access in order to provide data on behalf of another business. For more information, see [Submitting Order Tracking Signals](/shopping-content/guides/order-tracking-signals). |
| `delivery_postal_code` | String |  | Optional. The delivery postal code, as a continuous string without spaces or dashes, for example "95016". This field will be anonymized in returned OrderTrackingSignal creation response. |
| `delivery_region_code` | String |  | Optional. The [CLDR territory code] (http://www.unicode.org/repos/cldr/tags/latest/common/main/en.xml) for the shipping destination. |
| `order_created_time` | String |  | Required. The time when the order was created on the businesses side. Include the year and timezone string, if available. |
| `line_items` | Vec<String> |  | Required. Information about line items in the order. |
| `shipment_line_item_mapping` | Vec<String> |  | Optional. The mapping of the line items to the shipment information. |
| `parent` | String | ✅ | Required. The account of the business for which the order signal is created. Format: accounts/{account} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create order_tracking_signal
order_tracking_signal = provider.merchantapi_api.Order_tracking_signal {
    parent = "value"  # Required. The account of the business for which the order signal is created. Format: accounts/{account}
}

```

---


### Promotion

Inserts a promotion for your Merchant Center account. If the promotion already exists, then it updates the promotion instead.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source` | String |  | Required. The data source of the [promotion](https://support.google.com/merchants/answer/6396268?sjid=5155774230887277618-NC) Format: `accounts/{account}/dataSources/{datasource}`. |
| `promotion` | String |  | Required. The promotion to insert. |
| `parent` | String | ✅ | Required. The account where the promotion will be inserted. Format: accounts/{account} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `target_country` | String | Required. The target country used as part of the unique identifier. Represented as a [CLDR territory code](https://github.com/unicode-org/cldr/blob/latest/common/main/en.xml). Promotions are only available in selected countries, [Free Listings and Shopping ads](https://support.google.com/merchants/answer/4588460) [Local Inventory ads](https://support.google.com/merchants/answer/10146326) |
| `version_number` | String | Optional. Represents the existing version (freshness) of the promotion, which can be used to preserve the right order when multiple updates are done at the same time. If set, the insertion is prevented when version number is lower than the current version number of the existing promotion. Re-insertion (for example, promotion refresh after 30 days) can be performed with the current `version_number`. If the operation is prevented, the aborted exception will be thrown. |
| `data_source` | String | Output only. The primary data source of the promotion. |
| `name` | String | Identifier. The name of the promotion. Format: `accounts/{account}/promotions/{promotion}` |
| `custom_attributes` | Vec<String> | Optional. A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the data specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API. |
| `promotion_status` | String | Output only. The [status of a promotion](https://support.google.com/merchants/answer/3398326?ref_topic=7322924&sjid=5155774230887277618-NC), data validation issues, that is, information about a promotion computed asynchronously. |
| `attributes` | String | Optional. A list of promotion attributes. |
| `redemption_channel` | Vec<String> | Required. [Redemption channel](https://support.google.com/merchants/answer/13837674?ref_topic=13773355&sjid=17642868584668136159-NC) for the promotion. At least one channel is required. |
| `content_language` | String | Required. The two-letter [ISO 639-1](http://en.wikipedia.org/wiki/ISO_639-1) language code for the promotion. Promotions is only for [selected languages](https://support.google.com/merchants/answer/4588281?ref_topic=6396150&sjid=18314938579342094533-NC#option3&zippy=). |
| `promotion_id` | String | Required. The user provided promotion ID to uniquely identify the promotion. Follow [minimum requirements](https://support.google.com/merchants/answer/7050148?ref_topic=7322920&sjid=871860036916537104-NC#minimum_requirements) to prevent promotion disapprovals. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create promotion
promotion = provider.merchantapi_api.Promotion {
    parent = "value"  # Required. The account where the promotion will be inserted. Format: accounts/{account}
}

# Access promotion outputs
promotion_id = promotion.id
promotion_target_country = promotion.target_country
promotion_version_number = promotion.version_number
promotion_data_source = promotion.data_source
promotion_name = promotion.name
promotion_custom_attributes = promotion.custom_attributes
promotion_promotion_status = promotion.promotion_status
promotion_attributes = promotion.attributes
promotion_redemption_channel = promotion.redemption_channel
promotion_content_language = promotion.content_language
promotion_promotion_id = promotion.promotion_id
```

---


### Report

Retrieves a report defined by a search query. The response might contain fewer rows than specified by `page_size`. Rely on `next_page_token` to determine if there are more rows to be requested.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_token` | String |  | Optional. Token of the page to retrieve. If not specified, the first page of results is returned. In order to request the next page of results, the value obtained from `next_page_token` in the previous response should be used. |
| `query` | String |  | Required. Query that defines a report to be retrieved. For details on how to construct your query, see the [Query Language guide](/merchant/api/guides/reports/query-language). For the full list of available tables and fields, see the [Available fields](/merchant/api/reference/rest/reports_{api_version}/accounts.reports). |
| `page_size` | i64 |  | Optional. Number of `ReportRows` to retrieve in a single page. Defaults to 1000. Values above 5000 are coerced to 5000. |
| `parent` | String | ✅ | Required. Id of the account making the call. Must be a standalone account or an MCA subaccount. Format: accounts/{account} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report
report = provider.merchantapi_api.Report {
    parent = "value"  # Required. Id of the account making the call. Must be a standalone account or an MCA subaccount. Format: accounts/{account}
}

```

---


### Aggregate_product_statuse

Lists the `AggregateProductStatuses` resources for your merchant account. The response might contain fewer items than specified by `pageSize`. If `pageToken` was returned in previous request, it can be used to obtain additional results.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `pageToken` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `aggregate_product_statuses` | Vec<String> | The `AggregateProductStatuses` resources for the given account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access aggregate_product_statuse outputs
aggregate_product_statuse_id = aggregate_product_statuse.id
aggregate_product_statuse_next_page_token = aggregate_product_statuse.next_page_token
aggregate_product_statuse_aggregate_product_statuses = aggregate_product_statuse.aggregate_product_statuses
```

---


### Issueresolution

Start an action. The action can be requested by a business in third-party application. Before the business can request the action, the third-party application needs to show them action specific content and display a user input form. The action can be successfully started only once all `required` inputs are provided. If any `required` input is missing, or invalid value was provided, the service will return 400 error. Validation errors will contain Ids for all problematic field together with translated, human readable error messages that can be shown to the user.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action_input` | String |  | Required. Input provided by the business. |
| `action_context` | String |  | Required. The context from the selected action. The value is obtained from rendered issues and needs to be sent back to identify the action that is being triggered. |
| `name` | String | ✅ | Required. The business's account that is triggering the action. Format: `accounts/{account}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create issueresolution
issueresolution = provider.merchantapi_api.Issueresolution {
    name = "value"  # Required. The business's account that is triggering the action. Format: `accounts/{account}`
}

```

---


### Promotion

Inserts a promotion for your Merchant Center account. If the promotion already exists, then it updates the promotion instead.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source` | String |  | Required. The data source of the [promotion](https://support.google.com/merchants/answer/6396268?sjid=5155774230887277618-NC) Format: `accounts/{account}/dataSources/{datasource}`. |
| `promotion` | String |  | Required. The promotion to insert. |
| `parent` | String | ✅ | Required. The account where the promotion will be inserted. Format: accounts/{account} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_language` | String | Required. The two-letter [ISO 639-1](http://en.wikipedia.org/wiki/ISO_639-1) language code for the promotion. Promotions is only for [selected languages](https://support.google.com/merchants/answer/4588281?ref_topic=6396150&sjid=18314938579342094533-NC#option3&zippy=). |
| `name` | String | Identifier. The name of the promotion. Format: `accounts/{account}/promotions/{promotion}` |
| `target_country` | String | Required. The target country used as part of the unique identifier. Represented as a [CLDR territory code](https://github.com/unicode-org/cldr/blob/latest/common/main/en.xml). Promotions are only available in selected countries, [Free Listings and Shopping ads](https://support.google.com/merchants/answer/4588460) [Local Inventory ads](https://support.google.com/merchants/answer/10146326) |
| `attributes` | String | Optional. A list of promotion attributes. |
| `custom_attributes` | Vec<String> | Optional. A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the data specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API. |
| `data_source` | String | Output only. The primary data source of the promotion. |
| `promotion_id` | String | Required. The user provided promotion ID to uniquely identify the promotion. Follow [minimum requirements](https://support.google.com/merchants/answer/7050148?ref_topic=7322920&sjid=871860036916537104-NC#minimum_requirements) to prevent promotion disapprovals. |
| `redemption_channel` | Vec<String> | Required. [Redemption channel](https://support.google.com/merchants/answer/13837674?ref_topic=13773355&sjid=17642868584668136159-NC) for the promotion. At least one channel is required. |
| `promotion_status` | String | Output only. The [status of a promotion](https://support.google.com/merchants/answer/3398326?ref_topic=7322924&sjid=5155774230887277618-NC), data validation issues, that is, information about a promotion computed asynchronously. |
| `version_number` | String | Optional. Represents the existing version (freshness) of the promotion, which can be used to preserve the right order when multiple updates are done at the same time. If set, the insertion is prevented when version number is lower than the current version number of the existing promotion. Re-insertion (for example, promotion refresh after 30 days) can be performed with the current `version_number`. If the operation is prevented, the aborted exception will be thrown. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create promotion
promotion = provider.merchantapi_api.Promotion {
    parent = "value"  # Required. The account where the promotion will be inserted. Format: accounts/{account}
}

# Access promotion outputs
promotion_id = promotion.id
promotion_content_language = promotion.content_language
promotion_name = promotion.name
promotion_target_country = promotion.target_country
promotion_attributes = promotion.attributes
promotion_custom_attributes = promotion.custom_attributes
promotion_data_source = promotion.data_source
promotion_promotion_id = promotion.promotion_id
promotion_redemption_channel = promotion.redemption_channel
promotion_promotion_status = promotion.promotion_status
promotion_version_number = promotion.version_number
```

---


### Terms_of_service_agreement_state

Returns the state of a terms of service agreement.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `accepted` | String | Optional. The accepted terms of service of this kind and for the associated region_code |
| `required` | String | Optional. The required terms of service |
| `terms_of_service_kind` | String | Required. Terms of Service kind associated with the particular version. |
| `region_code` | String | Required. Region code as defined by https://cldr.unicode.org/. This is the country the current state applies to. |
| `name` | String | Identifier. The resource name of the terms of service version. Format: `accounts/{account}/termsOfServiceAgreementState/{identifier}` The identifier format is: `{TermsOfServiceKind}-{country}` For example, an identifier could be: `MERCHANT_CENTER-EU` or `MERCHANT_CENTER-US`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access terms_of_service_agreement_state outputs
terms_of_service_agreement_state_id = terms_of_service_agreement_state.id
terms_of_service_agreement_state_accepted = terms_of_service_agreement_state.accepted
terms_of_service_agreement_state_required = terms_of_service_agreement_state.required
terms_of_service_agreement_state_terms_of_service_kind = terms_of_service_agreement_state.terms_of_service_kind
terms_of_service_agreement_state_region_code = terms_of_service_agreement_state.region_code
terms_of_service_agreement_state_name = terms_of_service_agreement_state.name
```

---


### Email_preference

Returns the email preferences for a Merchant Center account user. This service only permits retrieving and updating email preferences for the authenticated user. Use the name=accounts/*/users/me/emailPreferences alias to get preferences for the authenticated user.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The name of the EmailPreferences. The endpoint is only supported for the authenticated user. |
| `news_and_tips` | String |  | Optional. Updates on new features, tips and best practices. |
| `name` | String | ✅ | Identifier. The name of the EmailPreferences. The endpoint is only supported for the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The name of the EmailPreferences. The endpoint is only supported for the authenticated user. |
| `news_and_tips` | String | Optional. Updates on new features, tips and best practices. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access email_preference outputs
email_preference_id = email_preference.id
email_preference_name = email_preference.name
email_preference_news_and_tips = email_preference.news_and_tips
```

---


### Shipping_setting

Replace the shipping setting of a business with the request shipping setting. Executing this method requires admin access.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Required. This field helps avoid async issues. It ensures that the shipping setting data doesn't change between the `get` call and the `insert` call. The user should follow these steps: 1. Set the etag field as an empty string for the initial shipping setting creation. 2. After the initial creation, call the `get` method to obtain an etag and the current shipping setting data before calling `insert`. 3. Modify the shipping setting information. 4. Call the `insert` method with the shipping setting information and the etag obtained in step 2. 5. If the shipping setting data changes between step 2 and step 4, the insert request will fail because the etag changes every time the shipping setting data changes. In this case, the user should repeat steps 2-4 with the new etag. |
| `warehouses` | Vec<String> |  | Optional. A list of warehouses which can be referred to in `services`. |
| `services` | Vec<String> |  | Optional. The target account's list of services. |
| `name` | String |  | Identifier. The resource name of the shipping settings. Format: `accounts/{account}/shippingSettings`. For example, `accounts/123456/shippingSettings`. |
| `parent` | String | ✅ | Required. The account for which this shipping setting will be inserted. If you are using an advanced account, you must specify the unique identifier of the sub-account for which you want to insert the shipping setting. Format: `accounts/{ACCOUNT_ID}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Required. This field helps avoid async issues. It ensures that the shipping setting data doesn't change between the `get` call and the `insert` call. The user should follow these steps: 1. Set the etag field as an empty string for the initial shipping setting creation. 2. After the initial creation, call the `get` method to obtain an etag and the current shipping setting data before calling `insert`. 3. Modify the shipping setting information. 4. Call the `insert` method with the shipping setting information and the etag obtained in step 2. 5. If the shipping setting data changes between step 2 and step 4, the insert request will fail because the etag changes every time the shipping setting data changes. In this case, the user should repeat steps 2-4 with the new etag. |
| `warehouses` | Vec<String> | Optional. A list of warehouses which can be referred to in `services`. |
| `services` | Vec<String> | Optional. The target account's list of services. |
| `name` | String | Identifier. The resource name of the shipping settings. Format: `accounts/{account}/shippingSettings`. For example, `accounts/123456/shippingSettings`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create shipping_setting
shipping_setting = provider.merchantapi_api.Shipping_setting {
    parent = "value"  # Required. The account for which this shipping setting will be inserted. If you are using an advanced account, you must specify the unique identifier of the sub-account for which you want to insert the shipping setting. Format: `accounts/{ACCOUNT_ID}`
}

# Access shipping_setting outputs
shipping_setting_id = shipping_setting.id
shipping_setting_etag = shipping_setting.etag
shipping_setting_warehouses = shipping_setting.warehouses
shipping_setting_services = shipping_setting.services
shipping_setting_name = shipping_setting.name
```

---


### Developer_registration

Registers the GCP used for the API call to the shopping account passed in the request. Will create a user with an "API developer" and add the "developer_email" as a contact with "API notifications" email preference on.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `developer_email` | String |  | Immutable. If the developer email provided is associated with a user in the merchant account provided, the user will be updated to have "API developer" access type and the email preference corresponding to that user will be updated to have the new "API notifications" preference. If the developer email provided is not associated with any user we will just add it as a contact. The email preference corresponding to that contact will have the new "API notifications" preference. Make sure the email used is associated with a Google Account (Google Workspace account or Gmail account) and is not a service account as service accounts can't receive emails. |
| `name` | String | ✅ | Required. The name of the developer registration to be created for the merchant account that the GCP will be registered with. Format: `accounts/{account}/developerRegistration` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the merchant account id that the GCP is registered with. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create developer_registration
developer_registration = provider.merchantapi_api.Developer_registration {
    name = "value"  # Required. The name of the developer registration to be created for the merchant account that the GCP will be registered with. Format: `accounts/{account}/developerRegistration`
}

# Access developer_registration outputs
developer_registration_id = developer_registration.id
developer_registration_name = developer_registration.name
```

---


### Account

Creates a Merchant Center account with additional configuration. Adds the user that makes the request as an admin for the new account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user` | Vec<String> |  | Optional. Users to be added to the account. |
| `account` | String |  | Required. The account to be created. |
| `service` | Vec<String> |  | Required. An account service between the account to be created and the provider account is initialized as part of the creation. At least one such service needs to be provided. Currently exactly one of these needs to be `account_aggregation` and `accounts.createAndConfigure` method can be used to create a sub-account under an existing advanced account through this method. Additional `account_management` or `product_management` services may be provided. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `time_zone` | String | Required. The time zone of the account. On writes, `time_zone` sets both the `reporting_time_zone` and the `display_time_zone`. For reads, `time_zone` always returns the `display_time_zone`. If `display_time_zone` doesn't exist for your account, `time_zone` is empty. The `version` field is not supported, won't be set in responses and will be silently ignored if specified in requests. |
| `account_id` | String | Output only. The ID of the account. |
| `name` | String | Identifier. The resource name of the account. Format: `accounts/{account}` |
| `adult_content` | bool | Optional. Whether this account contains adult content. |
| `language_code` | String | Required. The account's [BCP-47 language code](https://tools.ietf.org/html/bcp47), such as `en-US` or `sr-Latn`. |
| `test_account` | bool | Output only. Whether this is a test account. |
| `account_name` | String | Required. A human-readable name of the account. See [store name](https://support.google.com/merchants/answer/160556) and [business name](https://support.google.com/merchants/answer/12159159) for more information. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.merchantapi_api.Account {
}

# Access account outputs
account_id = account.id
account_time_zone = account.time_zone
account_account_id = account.account_id
account_name = account.name
account_adult_content = account.adult_content
account_language_code = account.language_code
account_test_account = account.test_account
account_account_name = account.account_name
```

---


### Relationship

Retrieve an account relationship.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `provider_display_name` | String |  | Output only. The human-readable display name of the provider account. |
| `name` | String |  | Identifier. The resource name of the account relationship. Format: `accounts/{account}/relationships/{relationship}`. For example, `accounts/123456/relationships/567890`. |
| `account_id_alias` | String |  | Optional. An optional alias you can assign to this account relationship. This alias acts as a convenient identifier for your own reference and management. It must be unique among all your account relationships with the same provider. For example, you might use `account_id_alias` to assign a friendly name to this relationship for easier identification in your systems. |
| `provider` | String |  | Immutable. The provider of the service. Either the reference to an account such as `providers/123` or a well-known service provider (one of `providers/GOOGLE_ADS` or `providers/GOOGLE_BUSINESS_PROFILE`). |
| `name` | String | ✅ | Identifier. The resource name of the account relationship. Format: `accounts/{account}/relationships/{relationship}`. For example, `accounts/123456/relationships/567890`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `provider_display_name` | String | Output only. The human-readable display name of the provider account. |
| `name` | String | Identifier. The resource name of the account relationship. Format: `accounts/{account}/relationships/{relationship}`. For example, `accounts/123456/relationships/567890`. |
| `account_id_alias` | String | Optional. An optional alias you can assign to this account relationship. This alias acts as a convenient identifier for your own reference and management. It must be unique among all your account relationships with the same provider. For example, you might use `account_id_alias` to assign a friendly name to this relationship for easier identification in your systems. |
| `provider` | String | Immutable. The provider of the service. Either the reference to an account such as `providers/123` or a well-known service provider (one of `providers/GOOGLE_ADS` or `providers/GOOGLE_BUSINESS_PROFILE`). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access relationship outputs
relationship_id = relationship.id
relationship_provider_display_name = relationship.provider_display_name
relationship_name = relationship.name
relationship_account_id_alias = relationship.account_id_alias
relationship_provider = relationship.provider
```

---


### Gbp_account

Link the specified merchant to a GBP account for all countries. To run this method, you must have admin access to the Merchant Center account. If you don't have admin access, the request fails with the error message `User is not an administrator of account {ACCOUNT_ID}`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gbp_email` | String |  | Required. The email address of the Business Profile account. |
| `parent` | String | ✅ | Required. The name of the parent resource to which the GBP account is linked. Format: `accounts/{account}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gbp_accounts` | Vec<String> | The GBP accounts from the specified merchant in the specified country. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gbp_account
gbp_account = provider.merchantapi_api.Gbp_account {
    parent = "value"  # Required. The name of the parent resource to which the GBP account is linked. Format: `accounts/{account}`.
}

# Access gbp_account outputs
gbp_account_id = gbp_account.id
gbp_account_gbp_accounts = gbp_account.gbp_accounts
gbp_account_next_page_token = gbp_account.next_page_token
```

---


### Business_info

Retrieves the business info of an account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `address` | String |  | Optional. The address of the business. Only `region_code`, `address_lines`, `postal_code`, `administrative_area` and `locality` fields are supported. All other fields are ignored. |
| `korean_business_registration_number` | String |  | Optional. The 10-digit [Korean business registration number](https://support.google.com/merchants/answer/9037766) separated with dashes in the format: XXX-XX-XXXXX. |
| `phone_verification_state` | String |  | Output only. The phone verification state of the business. |
| `name` | String |  | Identifier. The resource name of the business info. Format: `accounts/{account}/businessInfo` |
| `phone` | String |  | Output only. The phone number of the business. |
| `customer_service` | String |  | Optional. The customer service of the business. |
| `name` | String | ✅ | Identifier. The resource name of the business info. Format: `accounts/{account}/businessInfo` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `address` | String | Optional. The address of the business. Only `region_code`, `address_lines`, `postal_code`, `administrative_area` and `locality` fields are supported. All other fields are ignored. |
| `korean_business_registration_number` | String | Optional. The 10-digit [Korean business registration number](https://support.google.com/merchants/answer/9037766) separated with dashes in the format: XXX-XX-XXXXX. |
| `phone_verification_state` | String | Output only. The phone verification state of the business. |
| `name` | String | Identifier. The resource name of the business info. Format: `accounts/{account}/businessInfo` |
| `phone` | String | Output only. The phone number of the business. |
| `customer_service` | String | Optional. The customer service of the business. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access business_info outputs
business_info_id = business_info.id
business_info_address = business_info.address
business_info_korean_business_registration_number = business_info.korean_business_registration_number
business_info_phone_verification_state = business_info.phone_verification_state
business_info_name = business_info.name
business_info_phone = business_info.phone
business_info_customer_service = business_info.customer_service
```

---


### Homepage

Unclaims a store's homepage. Executing this method requires admin access.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the homepage to unclaim. Format: `accounts/{account}/homepage` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the store's homepage. Format: `accounts/{account}/homepage` |
| `uri` | String | Required. The URI (typically a URL) of the store's homepage. |
| `claimed` | bool | Output only. Whether the homepage is claimed. See https://support.google.com/merchants/answer/176793. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create homepage
homepage = provider.merchantapi_api.Homepage {
    name = "value"  # Required. The name of the homepage to unclaim. Format: `accounts/{account}/homepage`
}

# Access homepage outputs
homepage_id = homepage.id
homepage_name = homepage.name
homepage_uri = homepage.uri
homepage_claimed = homepage.claimed
```

---


### Online_return_policie

Creates a new return policy for a given business.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `return_policy_uri` | String |  | Required. The return policy uri. This can used by Google to do a sanity check for the policy. It must be a valid URL. |
| `seasonal_overrides` | Vec<String> |  | Optional. Overrides to the general policy for orders placed during a specific set of time intervals. |
| `name` | String |  | Identifier. The name of the `OnlineReturnPolicy` resource. Format: `accounts/{account}/onlineReturnPolicies/{return_policy}` |
| `return_methods` | Vec<String> |  | Optional. The return methods of how customers can return an item. This value is required to not be empty unless the type of return policy is noReturns. |
| `return_shipping_fee` | String |  | Optional. The return shipping fee. Should be set only when customer need to download and print the return label. |
| `accept_exchange` | bool |  | Optional. This field specifies if business allows customers to exchange products. |
| `accept_defective_only` | bool |  | Optional. This field specifies if business only accepts defective products for returns. |
| `return_policy_id` | String |  | Output only. Return policy ID generated by Google. |
| `process_refund_days` | i64 |  | Optional. The field specifies the number of days it takes for business to process refunds. |
| `label` | String |  | Optional. Immutable. This field represents the unique user-defined label of the return policy for the given country. It is important to note that the same label cannot be used in different return policies for the same country. If not given, policies will be automatically treated as the 'default' for the country. When using label, you are creating an exception policy in that country to assign a custom return policy to certain product groups, follow the instructions provided in the [Return policy label] (https://support.google.com/merchants/answer/9445425). The label can contain up to 50 characters. |
| `return_label_source` | String |  | Optional. The field specifies the return label source. |
| `countries` | Vec<String> |  | Required. Immutable. The countries of sale where the return policy applies. The values must be a valid 2 letter ISO 3166 code. |
| `policy` | String |  | Optional. The return policy. |
| `restocking_fee` | String |  | Optional. The restocking fee that applies to all return reason categories. This would be treated as a free restocking fee if the value is not set. |
| `item_conditions` | Vec<String> |  | Optional. The item conditions accepted for returns must not be empty unless the type of return policy is 'noReturns'. |
| `parent` | String | ✅ | Required. The Merchant Center account for which the return policy will be created. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `return_policy_uri` | String | Required. The return policy uri. This can used by Google to do a sanity check for the policy. It must be a valid URL. |
| `seasonal_overrides` | Vec<String> | Optional. Overrides to the general policy for orders placed during a specific set of time intervals. |
| `name` | String | Identifier. The name of the `OnlineReturnPolicy` resource. Format: `accounts/{account}/onlineReturnPolicies/{return_policy}` |
| `return_methods` | Vec<String> | Optional. The return methods of how customers can return an item. This value is required to not be empty unless the type of return policy is noReturns. |
| `return_shipping_fee` | String | Optional. The return shipping fee. Should be set only when customer need to download and print the return label. |
| `accept_exchange` | bool | Optional. This field specifies if business allows customers to exchange products. |
| `accept_defective_only` | bool | Optional. This field specifies if business only accepts defective products for returns. |
| `return_policy_id` | String | Output only. Return policy ID generated by Google. |
| `process_refund_days` | i64 | Optional. The field specifies the number of days it takes for business to process refunds. |
| `label` | String | Optional. Immutable. This field represents the unique user-defined label of the return policy for the given country. It is important to note that the same label cannot be used in different return policies for the same country. If not given, policies will be automatically treated as the 'default' for the country. When using label, you are creating an exception policy in that country to assign a custom return policy to certain product groups, follow the instructions provided in the [Return policy label] (https://support.google.com/merchants/answer/9445425). The label can contain up to 50 characters. |
| `return_label_source` | String | Optional. The field specifies the return label source. |
| `countries` | Vec<String> | Required. Immutable. The countries of sale where the return policy applies. The values must be a valid 2 letter ISO 3166 code. |
| `policy` | String | Optional. The return policy. |
| `restocking_fee` | String | Optional. The restocking fee that applies to all return reason categories. This would be treated as a free restocking fee if the value is not set. |
| `item_conditions` | Vec<String> | Optional. The item conditions accepted for returns must not be empty unless the type of return policy is 'noReturns'. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create online_return_policie
online_return_policie = provider.merchantapi_api.Online_return_policie {
    parent = "value"  # Required. The Merchant Center account for which the return policy will be created. Format: `accounts/{account}`
}

# Access online_return_policie outputs
online_return_policie_id = online_return_policie.id
online_return_policie_return_policy_uri = online_return_policie.return_policy_uri
online_return_policie_seasonal_overrides = online_return_policie.seasonal_overrides
online_return_policie_name = online_return_policie.name
online_return_policie_return_methods = online_return_policie.return_methods
online_return_policie_return_shipping_fee = online_return_policie.return_shipping_fee
online_return_policie_accept_exchange = online_return_policie.accept_exchange
online_return_policie_accept_defective_only = online_return_policie.accept_defective_only
online_return_policie_return_policy_id = online_return_policie.return_policy_id
online_return_policie_process_refund_days = online_return_policie.process_refund_days
online_return_policie_label = online_return_policie.label
online_return_policie_return_label_source = online_return_policie.return_label_source
online_return_policie_countries = online_return_policie.countries
online_return_policie_policy = online_return_policie.policy
online_return_policie_restocking_fee = online_return_policie.restocking_fee
online_return_policie_item_conditions = online_return_policie.item_conditions
```

---


### Terms_of_service

Accepts a `TermsOfService`. Executing this method requires admin access.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The resource name of the terms of service version. Format: `termsOfService/{version}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the terms of service version. Format: `termsOfService/{version}` |
| `file_uri` | String | URI for terms of service file that needs to be displayed to signing users. |
| `kind` | String | The Kind this terms of service version applies to. |
| `external` | bool | Whether this terms of service version is external. External terms of service versions can only be agreed through external processes and not directly by the merchant through UI or API. |
| `region_code` | String | Region code as defined by [CLDR](https://cldr.unicode.org/). This is either a country where the ToS applies specifically to that country or `001` when the same `TermsOfService` can be signed in any country. However note that when signing a ToS that applies globally we still expect that a specific country is provided (this should be merchant business country or program country of participation). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create terms_of_service
terms_of_service = provider.merchantapi_api.Terms_of_service {
    name = "value"  # Required. The resource name of the terms of service version. Format: `termsOfService/{version}`
}

# Access terms_of_service outputs
terms_of_service_id = terms_of_service.id
terms_of_service_name = terms_of_service.name
terms_of_service_file_uri = terms_of_service.file_uri
terms_of_service_kind = terms_of_service.kind
terms_of_service_external = terms_of_service.external
terms_of_service_region_code = terms_of_service.region_code
```

---


### Omnichannel_setting

Create the omnichannel settings for a given merchant.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inventory_verification` | String |  | Optional. The inventory verification contact and state for this country. |
| `name` | String |  | Identifier. The resource name of the omnichannel setting. Format: `accounts/{account}/omnichannelSettings/{omnichannel_setting}` |
| `region_code` | String |  | Required. Immutable. Region code defined by [CLDR](https://cldr.unicode.org/). Must be provided in the Create method, and is immutable. |
| `lsf_type` | String |  | Required. The Local Store Front type for this country. |
| `lfp_link` | String |  | Output only. The established link to a LFP provider. |
| `in_stock` | String |  | Optional. The InStock URI and state for this country. |
| `odo` | String |  | Optional. The On Display to Order (ODO) policy URI and state for this country. |
| `about` | String |  | Optional. The about page URI and state for this country. |
| `pickup` | String |  | Optional. The Pickup URI and state for this country. |
| `parent` | String | ✅ | Required. The parent resource where this omnichannel setting will be created. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `inventory_verification` | String | Optional. The inventory verification contact and state for this country. |
| `name` | String | Identifier. The resource name of the omnichannel setting. Format: `accounts/{account}/omnichannelSettings/{omnichannel_setting}` |
| `region_code` | String | Required. Immutable. Region code defined by [CLDR](https://cldr.unicode.org/). Must be provided in the Create method, and is immutable. |
| `lsf_type` | String | Required. The Local Store Front type for this country. |
| `lfp_link` | String | Output only. The established link to a LFP provider. |
| `in_stock` | String | Optional. The InStock URI and state for this country. |
| `odo` | String | Optional. The On Display to Order (ODO) policy URI and state for this country. |
| `about` | String | Optional. The about page URI and state for this country. |
| `pickup` | String | Optional. The Pickup URI and state for this country. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create omnichannel_setting
omnichannel_setting = provider.merchantapi_api.Omnichannel_setting {
    parent = "value"  # Required. The parent resource where this omnichannel setting will be created. Format: `accounts/{account}`
}

# Access omnichannel_setting outputs
omnichannel_setting_id = omnichannel_setting.id
omnichannel_setting_inventory_verification = omnichannel_setting.inventory_verification
omnichannel_setting_name = omnichannel_setting.name
omnichannel_setting_region_code = omnichannel_setting.region_code
omnichannel_setting_lsf_type = omnichannel_setting.lsf_type
omnichannel_setting_lfp_link = omnichannel_setting.lfp_link
omnichannel_setting_in_stock = omnichannel_setting.in_stock
omnichannel_setting_odo = omnichannel_setting.odo
omnichannel_setting_about = omnichannel_setting.about
omnichannel_setting_pickup = omnichannel_setting.pickup
```

---


### Automatic_improvement

Retrieves the automatic improvements of an account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `shipping_improvements` | String |  | Not available for [advanced accounts](https://support.google.com/merchants/answer/188487). By turning on [automatic shipping improvements](https://support.google.com/merchants/answer/10027038), you are allowing Google to improve the accuracy of your delivery times shown to shoppers using Google. More accurate delivery times, especially when faster, typically lead to better conversion rates. Google will improve your estimated delivery times based on various factors: * Delivery address of an order * Current handling time and shipping time settings * Estimated weekdays or business days * Parcel tracking data This field is only updated (cleared) if provided in the update mask. |
| `image_improvements` | String |  | This improvement will attempt to automatically correct submitted images if they don't meet the [image requirements](https://support.google.com/merchants/answer/6324350), for example, removing overlays. If successful, the image will be replaced and approved. This improvement is only applied to images of disapproved offers. For more information see: [Automatic image improvements](https://support.google.com/merchants/answer/9242973) This field is only updated (cleared) if provided in the update mask. |
| `item_updates` | String |  | Turning on [item updates](https://support.google.com/merchants/answer/3246284) allows Google to automatically update items for you. When item updates are on, Google uses the structured data markup on the website and advanced data extractors to update the price and availability of the items. When the item updates are off, items with mismatched data aren't shown. This field is only updated (cleared) if provided in the update mask. |
| `name` | String |  | Identifier. The resource name of the automatic improvements. Format: `accounts/{account}/automaticImprovements`. |
| `name` | String | ✅ | Identifier. The resource name of the automatic improvements. Format: `accounts/{account}/automaticImprovements`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `shipping_improvements` | String | Not available for [advanced accounts](https://support.google.com/merchants/answer/188487). By turning on [automatic shipping improvements](https://support.google.com/merchants/answer/10027038), you are allowing Google to improve the accuracy of your delivery times shown to shoppers using Google. More accurate delivery times, especially when faster, typically lead to better conversion rates. Google will improve your estimated delivery times based on various factors: * Delivery address of an order * Current handling time and shipping time settings * Estimated weekdays or business days * Parcel tracking data This field is only updated (cleared) if provided in the update mask. |
| `image_improvements` | String | This improvement will attempt to automatically correct submitted images if they don't meet the [image requirements](https://support.google.com/merchants/answer/6324350), for example, removing overlays. If successful, the image will be replaced and approved. This improvement is only applied to images of disapproved offers. For more information see: [Automatic image improvements](https://support.google.com/merchants/answer/9242973) This field is only updated (cleared) if provided in the update mask. |
| `item_updates` | String | Turning on [item updates](https://support.google.com/merchants/answer/3246284) allows Google to automatically update items for you. When item updates are on, Google uses the structured data markup on the website and advanced data extractors to update the price and availability of the items. When the item updates are off, items with mismatched data aren't shown. This field is only updated (cleared) if provided in the update mask. |
| `name` | String | Identifier. The resource name of the automatic improvements. Format: `accounts/{account}/automaticImprovements`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access automatic_improvement outputs
automatic_improvement_id = automatic_improvement.id
automatic_improvement_shipping_improvements = automatic_improvement.shipping_improvements
automatic_improvement_image_improvements = automatic_improvement.image_improvements
automatic_improvement_item_updates = automatic_improvement.item_updates
automatic_improvement_name = automatic_improvement.name
```

---


### Program

Enable participation in the specified program for the account.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the program for which to enable participation for the given account. Format: `accounts/{account}/programs/{program}`. For example, `accounts/123456/programs/free-listings`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `unmet_requirements` | Vec<String> | Output only. The requirements that the account has not yet satisfied that are affecting participation in the program. |
| `state` | String | Output only. The participation state of the account in the program. |
| `active_region_codes` | Vec<String> | Output only. The regions in which the account is actively participating in the program. Active regions are defined as those where all program requirements affecting the regions have been met. Region codes are defined by [CLDR](https://cldr.unicode.org/). This is either a country where the program applies specifically to that country or `001` when the program applies globally. |
| `name` | String | Identifier. The resource name of the program. Format: `accounts/{account}/programs/{program}` |
| `documentation_uri` | String | Output only. The URL of a Merchant Center help page describing the program. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create program
program = provider.merchantapi_api.Program {
    name = "value"  # Required. The name of the program for which to enable participation for the given account. Format: `accounts/{account}/programs/{program}`. For example, `accounts/123456/programs/free-listings`.
}

# Access program outputs
program_id = program.id
program_unmet_requirements = program.unmet_requirements
program_state = program.state
program_active_region_codes = program.active_region_codes
program_name = program.name
program_documentation_uri = program.documentation_uri
```

---


### Service

Propose an account service.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `provider` | String |  | Required. The provider of the service. Either the reference to an account such as `providers/123` or a well-known service provider (one of `providers/GOOGLE_ADS` or `providers/GOOGLE_BUSINESS_PROFILE`). |
| `account_service` | String |  | Required. The account service to propose. |
| `parent` | String | ✅ | Required. The resource name of the parent account for the service. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_aggregation` | String | Service type for account aggregation. This enables the provider, which is an advanced account, to manage multiple sub-accounts (client accounts). Through this service, the advanced account provider can perform administrative and operational tasks across all linked sub-accounts. This is useful for agencies, aggregators, or large retailers that need centralized control over many Merchant Center accounts. |
| `name` | String | Identifier. The resource name of the account service. Format: `accounts/{account}/services/{service}` |
| `campaigns_management` | String | Service type for managing advertising campaigns. Grants the provider access to create and manage the business's ad campaigns, including setting up campaigns, adjusting bids, and optimizing performance. |
| `handshake` | String | Output only. Information about the state of the service in terms of establishing it (e.g. is it pending approval or approved). |
| `local_listing_management` | String | Service type for local listings management. The business group associated with the external account id will be used to provide local inventory to this Merchant Center account. |
| `provider_display_name` | String | Output only. The human-readable display name of the provider account. |
| `products_management` | String | Service type for managing products. This allows the provider to handle product data on behalf of the business, including reading and writing product listings. It's commonly used when the provider offers inventory management or catalog synchronization services to keep the business's product information up-to-date across platforms. |
| `account_management` | String | Service type for account management. Enables the provider to perform administrative actions on the business's account, such as configuring account settings, managing users, or updating business information. |
| `provider` | String | Output only. The provider of the service. Either the reference to an account such as `providers/123` or a well-known service provider (one of `providers/GOOGLE_ADS` or `providers/GOOGLE_BUSINESS_PROFILE`). |
| `mutability` | String | Output only. Whether the service is mutable (e.g. through Approve / Reject RPCs). A service that was created through another system or API might be immutable. |
| `external_account_id` | String | Immutable. An optional, immutable identifier that Google uses to refer to this account when communicating with the provider. This should be the unique account ID within the provider's system (for example, your shop ID in Shopify). If you have multiple accounts with the same provider - for instance, different accounts for various regions — the `external_account_id` differentiates between them, ensuring accurate linking and integration between Google and the provider. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.merchantapi_api.Service {
    parent = "value"  # Required. The resource name of the parent account for the service. Format: `accounts/{account}`
}

# Access service outputs
service_id = service.id
service_account_aggregation = service.account_aggregation
service_name = service.name
service_campaigns_management = service.campaigns_management
service_handshake = service.handshake
service_local_listing_management = service.local_listing_management
service_provider_display_name = service.provider_display_name
service_products_management = service.products_management
service_account_management = service.account_management
service_provider = service.provider
service_mutability = service.mutability
service_external_account_id = service.external_account_id
```

---


### Business_identity

Retrieves the business identity of an account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `women_owned` | String |  | Optional. Specifies whether the business identifies itself as being women-owned. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `veteran_owned` | String |  | Optional. Specifies whether the business identifies itself as being veteran-owned. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `small_business` | String |  | Optional. Specifies whether the business identifies itself as a small business. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces. |
| `promotions_consent` | String |  | Required. Whether the identity attributes may be used for promotions. |
| `latino_owned` | String |  | Optional. Specifies whether the business identifies itself as being latino-owned. This optional field will only be available for businesses with the business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `black_owned` | String |  | Optional. Specifies whether the business identifies itself as being black-owned. This optional field will only be available for businesses with the business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `name` | String |  | Identifier. The resource name of the business identity. Format: `accounts/{account}/businessIdentity` |
| `name` | String | ✅ | Identifier. The resource name of the business identity. Format: `accounts/{account}/businessIdentity` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `women_owned` | String | Optional. Specifies whether the business identifies itself as being women-owned. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `veteran_owned` | String | Optional. Specifies whether the business identifies itself as being veteran-owned. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `small_business` | String | Optional. Specifies whether the business identifies itself as a small business. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces. |
| `promotions_consent` | String | Required. Whether the identity attributes may be used for promotions. |
| `latino_owned` | String | Optional. Specifies whether the business identifies itself as being latino-owned. This optional field will only be available for businesses with the business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `black_owned` | String | Optional. Specifies whether the business identifies itself as being black-owned. This optional field will only be available for businesses with the business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `name` | String | Identifier. The resource name of the business identity. Format: `accounts/{account}/businessIdentity` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access business_identity outputs
business_identity_id = business_identity.id
business_identity_women_owned = business_identity.women_owned
business_identity_veteran_owned = business_identity.veteran_owned
business_identity_small_business = business_identity.small_business
business_identity_promotions_consent = business_identity.promotions_consent
business_identity_latino_owned = business_identity.latino_owned
business_identity_black_owned = business_identity.black_owned
business_identity_name = business_identity.name
```

---


### User

Creates a Merchant Center account user. Executing this method requires admin access.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the user. Format: `accounts/{account}/user/{email}` Use `me` to refer to your own email address, for example `accounts/{account}/users/me`. |
| `state` | String |  | Output only. The state of the user. |
| `access_rights` | Vec<String> |  | Required. The [access rights](https://support.google.com/merchants/answer/12160472?sjid=6789834943175119429-EU#accesstypes) the user has. |
| `parent` | String | ✅ | Required. The resource name of the account for which a user will be created. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the user. Format: `accounts/{account}/user/{email}` Use `me` to refer to your own email address, for example `accounts/{account}/users/me`. |
| `state` | String | Output only. The state of the user. |
| `access_rights` | Vec<String> | Required. The [access rights](https://support.google.com/merchants/answer/12160472?sjid=6789834943175119429-EU#accesstypes) the user has. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.merchantapi_api.User {
    parent = "value"  # Required. The resource name of the account for which a user will be created. Format: `accounts/{account}`
}

# Access user outputs
user_id = user.id
user_name = user.name
user_state = user.state
user_access_rights = user.access_rights
```

---


### Autofeed_setting

Retrieves the autofeed settings of an account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `eligible` | bool |  | Output only. Determines whether the business is eligible for being enrolled into an autofeed. |
| `enable_products` | bool |  | Required. Enables or disables product crawling through the autofeed for the given account. Autofeed accounts must meet [certain conditions](https://support.google.com/merchants/answer/7538732#Configure_automated_feeds_Standard_Experience), which can be checked through the `eligible` field. The account must **not** be a marketplace. When the autofeed is enabled for the first time, the products usually appear instantly. When re-enabling, it might take up to 24 hours for products to appear. |
| `name` | String |  | Identifier. The resource name of the autofeed settings. Format: `accounts/{account}/autofeedSettings`. |
| `name` | String | ✅ | Identifier. The resource name of the autofeed settings. Format: `accounts/{account}/autofeedSettings`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `eligible` | bool | Output only. Determines whether the business is eligible for being enrolled into an autofeed. |
| `enable_products` | bool | Required. Enables or disables product crawling through the autofeed for the given account. Autofeed accounts must meet [certain conditions](https://support.google.com/merchants/answer/7538732#Configure_automated_feeds_Standard_Experience), which can be checked through the `eligible` field. The account must **not** be a marketplace. When the autofeed is enabled for the first time, the products usually appear instantly. When re-enabling, it might take up to 24 hours for products to appear. |
| `name` | String | Identifier. The resource name of the autofeed settings. Format: `accounts/{account}/autofeedSettings`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access autofeed_setting outputs
autofeed_setting_id = autofeed_setting.id
autofeed_setting_eligible = autofeed_setting.eligible
autofeed_setting_enable_products = autofeed_setting.enable_products
autofeed_setting_name = autofeed_setting.name
```

---


### Issue

Lists all account issues of a Merchant Center account. When called on a multi-client account, this method only returns issues belonging to that account, not its sub-accounts. To retrieve issues for sub-accounts, you must first call the accounts.listSubaccounts method to obtain a list of sub-accounts, and then call `accounts.issues.list` for each sub-account individually.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `account_issues` | Vec<String> | The issues from the specified account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access issue outputs
issue_id = issue.id
issue_next_page_token = issue.next_page_token
issue_account_issues = issue.account_issues
```

---


### Checkout_setting

Creates `CheckoutSettings` for the given merchant.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `effective_review_state` | String |  | Output only. The effective value of `review_state` for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |
| `effective_enrollment_state` | String |  | Output only. The effective value of enrollment_state for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |
| `enrollment_state` | String |  | Output only. Reflects the merchant enrollment state in `Checkout` program. |
| `name` | String |  | Identifier. The resource name of the program configuration settings. Format: `accounts/{account}/programs/{program}/checkoutSettings` |
| `review_state` | String |  | Output only. Reflects the merchant review state in `Checkout` program. This is set based on the data quality reviews of the URL provided by the merchant. A merchant with enrollment state as `ENROLLED` can be in the following review states: `IN_REVIEW`, `APPROVED` or `DISAPPROVED`. A merchant must be in an `enrollment_state` of `ENROLLED` before a review can begin for the merchant.For more details, check the help center doc. |
| `effective_uri_settings` | String |  | Output only. The effective value of `uri_settings` for a given merchant. If account level settings are present then this value will be a copy of url settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |
| `uri_settings` | String |  | URI settings for cart or checkout URL. |
| `eligible_destinations` | Vec<String> |  | Optional. The destinations (also known as [Marketing methods](https://support.google.com/merchants/answer/15130232)) to which the checkout program applies, valid destination values are `SHOPPING_ADS`, `FREE_LISTINGS` |
| `parent` | String | ✅ | Required. The merchant account for which the `CheckoutSettings` will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `effective_review_state` | String | Output only. The effective value of `review_state` for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |
| `effective_enrollment_state` | String | Output only. The effective value of enrollment_state for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |
| `enrollment_state` | String | Output only. Reflects the merchant enrollment state in `Checkout` program. |
| `name` | String | Identifier. The resource name of the program configuration settings. Format: `accounts/{account}/programs/{program}/checkoutSettings` |
| `review_state` | String | Output only. Reflects the merchant review state in `Checkout` program. This is set based on the data quality reviews of the URL provided by the merchant. A merchant with enrollment state as `ENROLLED` can be in the following review states: `IN_REVIEW`, `APPROVED` or `DISAPPROVED`. A merchant must be in an `enrollment_state` of `ENROLLED` before a review can begin for the merchant.For more details, check the help center doc. |
| `effective_uri_settings` | String | Output only. The effective value of `uri_settings` for a given merchant. If account level settings are present then this value will be a copy of url settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |
| `uri_settings` | String | URI settings for cart or checkout URL. |
| `eligible_destinations` | Vec<String> | Optional. The destinations (also known as [Marketing methods](https://support.google.com/merchants/answer/15130232)) to which the checkout program applies, valid destination values are `SHOPPING_ADS`, `FREE_LISTINGS` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create checkout_setting
checkout_setting = provider.merchantapi_api.Checkout_setting {
    parent = "value"  # Required. The merchant account for which the `CheckoutSettings` will be created.
}

# Access checkout_setting outputs
checkout_setting_id = checkout_setting.id
checkout_setting_effective_review_state = checkout_setting.effective_review_state
checkout_setting_effective_enrollment_state = checkout_setting.effective_enrollment_state
checkout_setting_enrollment_state = checkout_setting.enrollment_state
checkout_setting_name = checkout_setting.name
checkout_setting_review_state = checkout_setting.review_state
checkout_setting_effective_uri_settings = checkout_setting.effective_uri_settings
checkout_setting_uri_settings = checkout_setting.uri_settings
checkout_setting_eligible_destinations = checkout_setting.eligible_destinations
```

---


### Lfp_provider

Link the specified merchant to a LFP provider for the specified country.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `external_account_id` | String |  | Required. The external account ID by which this merchant is known to the LFP provider. |
| `name` | String | ✅ | Required. The name of the LFP provider resource to link. Format: `accounts/{account}/omnichannelSettings/{omnichannel_setting}/lfpProviders/{lfp_provider}`. The `lfp_provider` is the LFP provider ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lfp_providers` | Vec<String> | The LFP providers from the specified merchant in the specified country. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lfp_provider
lfp_provider = provider.merchantapi_api.Lfp_provider {
    name = "value"  # Required. The name of the LFP provider resource to link. Format: `accounts/{account}/omnichannelSettings/{omnichannel_setting}/lfpProviders/{lfp_provider}`. The `lfp_provider` is the LFP provider ID.
}

# Access lfp_provider outputs
lfp_provider_id = lfp_provider.id
lfp_provider_lfp_providers = lfp_provider.lfp_providers
lfp_provider_next_page_token = lfp_provider.next_page_token
```

---


### Region

Creates a region definition in your Merchant Center account. Executing this method requires admin access.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `shipping_eligible` | bool |  | Output only. Indicates if the region is eligible for use in the Shipping Services configuration. |
| `geotarget_area` | String |  | Optional. A list of geotargets that defines the region area. |
| `name` | String |  | Identifier. The resource name of the region. Format: `accounts/{account}/regions/{region}` |
| `display_name` | String |  | Optional. The display name of the region. |
| `postal_code_area` | String |  | Optional. A list of postal codes that defines the region area. |
| `regional_inventory_eligible` | bool |  | Output only. Indicates if the region is eligible for use in the Regional Inventory configuration. |
| `parent` | String | ✅ | Required. The account to create a region for. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `shipping_eligible` | bool | Output only. Indicates if the region is eligible for use in the Shipping Services configuration. |
| `geotarget_area` | String | Optional. A list of geotargets that defines the region area. |
| `name` | String | Identifier. The resource name of the region. Format: `accounts/{account}/regions/{region}` |
| `display_name` | String | Optional. The display name of the region. |
| `postal_code_area` | String | Optional. A list of postal codes that defines the region area. |
| `regional_inventory_eligible` | bool | Output only. Indicates if the region is eligible for use in the Regional Inventory configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create region
region = provider.merchantapi_api.Region {
    parent = "value"  # Required. The account to create a region for. Format: `accounts/{account}`
}

# Access region outputs
region_id = region.id
region_shipping_eligible = region.shipping_eligible
region_geotarget_area = region.geotarget_area
region_name = region.name
region_display_name = region.display_name
region_postal_code_area = region.postal_code_area
region_regional_inventory_eligible = region.regional_inventory_eligible
```

---


### Product

Retrieves the processed product from your Merchant Center account. After inserting, updating, or deleting a product input, it may take several minutes before the updated final product can be retrieved.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the product. Format: `accounts/{account}/products/{product}` where the last section `product` consists of: `content_language~feed_label~offer_id` example for product name is `accounts/123/products/en~US~sku123`. A legacy local product name would be `accounts/123/products/local~en~US~sku123`. Note: For calls to the v1beta version, the `product` section consists of: `channel~content_language~feed_label~offer_id`, for example: `accounts/123/products/online~en~US~sku123`. |
| `offer_id` | String | Output only. Your unique identifier for the product. This is the same for the product input and processed product. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. See the [product data specification](https://support.google.com/merchants/answer/188494#id) for details. |
| `channel` | String | Output only. The [channel](https://support.google.com/merchants/answer/7361332) of the product. |
| `attributes` | String | Output only. A list of product attributes. |
| `custom_attributes` | Vec<String> | Output only. A list of custom (merchant-provided) attributes. It can also be used to submit any attribute of the data specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google. |
| `feed_label` | String | Output only. The feed label lets you categorize and identify your products. The maximum allowed characters is 20 and the supported characters are`A-Z`, `0-9`, hyphen and underscore. The feed label must not include any spaces. For more information, see [Using feed labels](//support.google.com/merchants/answer/14994087) |
| `content_language` | String | Output only. The two-letter [ISO 639-1](http://en.wikipedia.org/wiki/ISO_639-1) language code for the product. |
| `data_source` | String | Output only. The primary data source of the product. |
| `version_number` | String | Output only. Represents the existing version (freshness) of the product, which can be used to preserve the right order when multiple updates are done at the same time. If set, the insertion is prevented when version number is lower than the current version number of the existing product. Re-insertion (for example, product refresh after 30 days) can be performed with the current `version_number`. Only supported for insertions into primary data sources. If the operation is prevented, the aborted exception will be thrown. |
| `automated_discounts` | String | Output only. The automated discounts information for the product. |
| `product_status` | String | Output only. The status of a product, data validation issues, that is, information about a product computed asynchronously. |


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
product_name = product.name
product_offer_id = product.offer_id
product_channel = product.channel
product_attributes = product.attributes
product_custom_attributes = product.custom_attributes
product_feed_label = product.feed_label
product_content_language = product.content_language
product_data_source = product.data_source
product_version_number = product.version_number
product_automated_discounts = product.automated_discounts
product_product_status = product.product_status
```

---


### Product_input

[Uploads a product input to your Merchant Center account](/merchant/api/guides/products/add-manage#add_a_product). You must have a products [data source](/merchant/api/guides/data-sources/api-sources#create-primary-data-source) to be able to insert a product. The unique identifier of the data source is passed as a query parameter in the request URL. If a product input with the same contentLanguage, offerId, and dataSource already exists, then the product input inserted by this method replaces that entry. After inserting, updating, or deleting a product input, it may take several minutes before the processed product can be retrieved.

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `channel` | String |  | Immutable. The [channel](https://support.google.com/merchants/answer/7361332) of the product. |
| `version_number` | String |  | Optional. Immutable. Represents the existing version (freshness) of the product, which can be used to preserve the right order when multiple updates are done at the same time. If set, the insertion is prevented when version number is lower than the current version number of the existing product. Re-insertion (for example, product refresh after 30 days) can be performed with the current `version_number`. Only supported for insertions into primary data sources. Do not set this field for updates. Do not set this field for insertions into supplemental data sources. If the operation is prevented, the aborted exception will be thrown. |
| `feed_label` | String |  | Required. Immutable. The feed label that lets you categorize and identify your products. The maximum allowed characters are 20, and the supported characters are `A-Z`, `0-9`, hyphen, and underscore. The feed label must not include any spaces. For more information, see [Using feed labels](//support.google.com/merchants/answer/14994087). |
| `custom_attributes` | Vec<String> |  | Optional. A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the data specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API. Maximum allowed number of characters for each custom attribute is 10240 (represents sum of characters for name and value). Maximum 2500 custom attributes can be set per product, with total size of 102.4kB. Underscores in custom attribute names are replaced by spaces upon insertion. |
| `content_language` | String |  | Required. Immutable. The two-letter [ISO 639-1](http://en.wikipedia.org/wiki/ISO_639-1) language code for the product. |
| `name` | String |  | Identifier. The name of the product input. Format: `accounts/{account}/productInputs/{productinput}` where the last section `productinput` consists of: `content_language~feed_label~offer_id` example for product input name is `accounts/123/productInputs/en~US~sku123`. A legacy local product input name would be `accounts/123/productInputs/local~en~US~sku123`. Note: For calls to the v1beta version, the `productInput` section consists of: `channel~content_language~feed_label~offer_id`, for example: `accounts/123/productInputs/online~en~US~sku123`. |
| `offer_id` | String |  | Required. Immutable. Your unique identifier for the product. This is the same for the product input and processed product. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. See the [products data specification](https://support.google.com/merchants/answer/188494#id) for details. |
| `product` | String |  | Output only. The name of the processed product. Format: `accounts/{account}/products/{product}` |
| `attributes` | String |  | Optional. A list of product attributes. |
| `parent` | String | ✅ | Required. The account where this product will be inserted. Format: `accounts/{account}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product_input
product_input = provider.merchantapi_api.Product_input {
    parent = "value"  # Required. The account where this product will be inserted. Format: `accounts/{account}`
}

```

---


### Notificationsubscription

Creates a notification subscription for a business. For standalone or subaccounts accounts, the business can create a subscription for self. For MCAs, the business can create a subscription for all managed accounts or for a specific subaccount. We will allow the following types of notification subscriptions to exist together (per business as a subscriber per event type): 1. Subscription for all managed accounts + subscription for self. 2. Multiple "partial" subscriptions for managed accounts + subscription for self. we will not allow (per business as a subscriber per event type): 1. Multiple self subscriptions. 2. Multiple "all managed accounts" subscriptions. 3. "All managed accounts" subscription and partial subscriptions at the same time. 4. Multiple partial subscriptions for the same target account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `all_managed_accounts` | bool |  | If this value is true, the requesting account is notified of the specified event for all managed accounts (can be subaccounts or other linked accounts) including newly added accounts on a daily basis. |
| `registered_event` | String |  | The event that the merchant wants to be notified about. |
| `name` | String |  | Output only. The `name` of the notification configuration. Generated by the Content API upon creation of a new `NotificationSubscription`. The `account` represents the merchant ID of the merchant that owns the configuration. Format: `accounts/{account}/notificationsubscriptions/{notification_subscription}` |
| `call_back_uri` | String |  | URL to be used to push the notification to the merchant. |
| `target_account` | String |  | The `name` of the account you want to receive notifications for. Format: `accounts/{account}` |
| `parent` | String | ✅ | Required. The merchant account that owns the new notification subscription. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `all_managed_accounts` | bool | If this value is true, the requesting account is notified of the specified event for all managed accounts (can be subaccounts or other linked accounts) including newly added accounts on a daily basis. |
| `registered_event` | String | The event that the merchant wants to be notified about. |
| `name` | String | Output only. The `name` of the notification configuration. Generated by the Content API upon creation of a new `NotificationSubscription`. The `account` represents the merchant ID of the merchant that owns the configuration. Format: `accounts/{account}/notificationsubscriptions/{notification_subscription}` |
| `call_back_uri` | String | URL to be used to push the notification to the merchant. |
| `target_account` | String | The `name` of the account you want to receive notifications for. Format: `accounts/{account}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notificationsubscription
notificationsubscription = provider.merchantapi_api.Notificationsubscription {
    parent = "value"  # Required. The merchant account that owns the new notification subscription. Format: `accounts/{account}`
}

# Access notificationsubscription outputs
notificationsubscription_id = notificationsubscription.id
notificationsubscription_all_managed_accounts = notificationsubscription.all_managed_accounts
notificationsubscription_registered_event = notificationsubscription.registered_event
notificationsubscription_name = notificationsubscription.name
notificationsubscription_call_back_uri = notificationsubscription.call_back_uri
notificationsubscription_target_account = notificationsubscription.target_account
```

---


### Account

Creates a Merchant Center account with additional configuration. Adds the user that makes the request as an admin for the new account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service` | Vec<String> |  | Required. An account service between the account to be created and the provider account is initialized as part of the creation. At least one such service needs to be provided. Currently exactly one of these needs to be `account_aggregation` and `accounts.createAndConfigure` method can be used to create a sub-account under an existing advanced account through this method. Additional `account_management` or `product_management` services may be provided. |
| `users` | Vec<String> |  | Optional. Users to be added to the account. This field is deprecated and will not exist after the API evolves out of beta. Use the `user` field instead. |
| `account` | String |  | Required. The account to be created. |
| `user` | Vec<String> |  | Optional. Users to be added to the account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `language_code` | String | Required. The account's [BCP-47 language code](https://tools.ietf.org/html/bcp47), such as `en-US` or `sr-Latn`. |
| `name` | String | Identifier. The resource name of the account. Format: `accounts/{account}` |
| `time_zone` | String | Required. The time zone of the account. On writes, `time_zone` sets both the `reporting_time_zone` and the `display_time_zone`. For reads, `time_zone` always returns the `display_time_zone`. If `display_time_zone` doesn't exist for your account, `time_zone` is empty. The `version` field is not supported, won't be set in responses and will be silently ignored if specified in requests. |
| `account_name` | String | Required. A human-readable name of the account. See [store name](https://support.google.com/merchants/answer/160556) and [business name](https://support.google.com/merchants/answer/12159159) for more information. |
| `adult_content` | bool | Optional. Whether this account contains adult content. |
| `account_id` | String | Output only. The ID of the account. |
| `test_account` | bool | Output only. Whether this is a test account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.merchantapi_api.Account {
}

# Access account outputs
account_id = account.id
account_language_code = account.language_code
account_name = account.name
account_time_zone = account.time_zone
account_account_name = account.account_name
account_adult_content = account.adult_content
account_account_id = account.account_id
account_test_account = account.test_account
```

---


### Terms_of_service

Accepts a `TermsOfService`. Executing this method requires admin access.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The resource name of the terms of service version. Format: `termsOfService/{version}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `external` | bool | Whether this terms of service version is external. External terms of service versions can only be agreed through external processes and not directly by the merchant through UI or API. |
| `name` | String | Identifier. The resource name of the terms of service version. Format: `termsOfService/{version}` |
| `file_uri` | String | URI for terms of service file that needs to be displayed to signing users. |
| `region_code` | String | Region code as defined by [CLDR](https://cldr.unicode.org/). This is either a country where the ToS applies specifically to that country or `001` when the same `TermsOfService` can be signed in any country. However note that when signing a ToS that applies globally we still expect that a specific country is provided (this should be merchant business country or program country of participation). |
| `kind` | String | The Kind this terms of service version applies to. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create terms_of_service
terms_of_service = provider.merchantapi_api.Terms_of_service {
    name = "value"  # Required. The resource name of the terms of service version. Format: `termsOfService/{version}`
}

# Access terms_of_service outputs
terms_of_service_id = terms_of_service.id
terms_of_service_external = terms_of_service.external
terms_of_service_name = terms_of_service.name
terms_of_service_file_uri = terms_of_service.file_uri
terms_of_service_region_code = terms_of_service.region_code
terms_of_service_kind = terms_of_service.kind
```

---


### Omnichannel_setting

Create the omnichannel settings for a given merchant.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the omnichannel setting. Format: `accounts/{account}/omnichannelSettings/{omnichannel_setting}` |
| `about` | String |  | Optional. The about page URI and state for this country. |
| `pickup` | String |  | Optional. The Pickup URI and state for this country. |
| `region_code` | String |  | Required. Immutable. Region code defined by [CLDR](https://cldr.unicode.org/). Must be provided in the Create method, and is immutable. |
| `lfp_link` | String |  | Output only. The established link to a LFP provider. |
| `inventory_verification` | String |  | Optional. The inventory verification contact and state for this country. |
| `in_stock` | String |  | Optional. The InStock URI and state for this country. |
| `odo` | String |  | Optional. The On Display to Order (ODO) policy URI and state for this country. |
| `lsf_type` | String |  | Required. The Local Store Front type for this country. |
| `parent` | String | ✅ | Required. The parent resource where this omnichannel setting will be created. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the omnichannel setting. Format: `accounts/{account}/omnichannelSettings/{omnichannel_setting}` |
| `about` | String | Optional. The about page URI and state for this country. |
| `pickup` | String | Optional. The Pickup URI and state for this country. |
| `region_code` | String | Required. Immutable. Region code defined by [CLDR](https://cldr.unicode.org/). Must be provided in the Create method, and is immutable. |
| `lfp_link` | String | Output only. The established link to a LFP provider. |
| `inventory_verification` | String | Optional. The inventory verification contact and state for this country. |
| `in_stock` | String | Optional. The InStock URI and state for this country. |
| `odo` | String | Optional. The On Display to Order (ODO) policy URI and state for this country. |
| `lsf_type` | String | Required. The Local Store Front type for this country. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create omnichannel_setting
omnichannel_setting = provider.merchantapi_api.Omnichannel_setting {
    parent = "value"  # Required. The parent resource where this omnichannel setting will be created. Format: `accounts/{account}`
}

# Access omnichannel_setting outputs
omnichannel_setting_id = omnichannel_setting.id
omnichannel_setting_name = omnichannel_setting.name
omnichannel_setting_about = omnichannel_setting.about
omnichannel_setting_pickup = omnichannel_setting.pickup
omnichannel_setting_region_code = omnichannel_setting.region_code
omnichannel_setting_lfp_link = omnichannel_setting.lfp_link
omnichannel_setting_inventory_verification = omnichannel_setting.inventory_verification
omnichannel_setting_in_stock = omnichannel_setting.in_stock
omnichannel_setting_odo = omnichannel_setting.odo
omnichannel_setting_lsf_type = omnichannel_setting.lsf_type
```

---


### Business_identity

Retrieves the business identity of an account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `black_owned` | String |  | Optional. Specifies whether the business identifies itself as being black-owned. This optional field will only be available for businesses with the business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `small_business` | String |  | Optional. Specifies whether the business identifies itself as a small business. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces. |
| `veteran_owned` | String |  | Optional. Specifies whether the business identifies itself as being veteran-owned. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `women_owned` | String |  | Optional. Specifies whether the business identifies itself as being women-owned. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `name` | String |  | Identifier. The resource name of the business identity. Format: `accounts/{account}/businessIdentity` |
| `promotions_consent` | String |  | Required. Whether the identity attributes may be used for promotions. |
| `latino_owned` | String |  | Optional. Specifies whether the business identifies itself as being latino-owned. This optional field will only be available for businesses with the business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `name` | String | ✅ | Identifier. The resource name of the business identity. Format: `accounts/{account}/businessIdentity` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `black_owned` | String | Optional. Specifies whether the business identifies itself as being black-owned. This optional field will only be available for businesses with the business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `small_business` | String | Optional. Specifies whether the business identifies itself as a small business. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces. |
| `veteran_owned` | String | Optional. Specifies whether the business identifies itself as being veteran-owned. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `women_owned` | String | Optional. Specifies whether the business identifies itself as being women-owned. This optional field will only be available for businesses with a business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |
| `name` | String | Identifier. The resource name of the business identity. Format: `accounts/{account}/businessIdentity` |
| `promotions_consent` | String | Required. Whether the identity attributes may be used for promotions. |
| `latino_owned` | String | Optional. Specifies whether the business identifies itself as being latino-owned. This optional field will only be available for businesses with the business country set to `US`. It is also not applicable for marketplaces or marketplace sellers. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access business_identity outputs
business_identity_id = business_identity.id
business_identity_black_owned = business_identity.black_owned
business_identity_small_business = business_identity.small_business
business_identity_veteran_owned = business_identity.veteran_owned
business_identity_women_owned = business_identity.women_owned
business_identity_name = business_identity.name
business_identity_promotions_consent = business_identity.promotions_consent
business_identity_latino_owned = business_identity.latino_owned
```

---


### Autofeed_setting

Retrieves the autofeed settings of an account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_products` | bool |  | Required. Enables or disables product crawling through the autofeed for the given account. Autofeed accounts must meet [certain conditions](https://support.google.com/merchants/answer/7538732#Configure_automated_feeds_Standard_Experience), which can be checked through the `eligible` field. The account must **not** be a marketplace. When the autofeed is enabled for the first time, the products usually appear instantly. When re-enabling, it might take up to 24 hours for products to appear. |
| `name` | String |  | Identifier. The resource name of the autofeed settings. Format: `accounts/{account}/autofeedSettings`. |
| `eligible` | bool |  | Output only. Determines whether the business is eligible for being enrolled into an autofeed. |
| `name` | String | ✅ | Identifier. The resource name of the autofeed settings. Format: `accounts/{account}/autofeedSettings`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_products` | bool | Required. Enables or disables product crawling through the autofeed for the given account. Autofeed accounts must meet [certain conditions](https://support.google.com/merchants/answer/7538732#Configure_automated_feeds_Standard_Experience), which can be checked through the `eligible` field. The account must **not** be a marketplace. When the autofeed is enabled for the first time, the products usually appear instantly. When re-enabling, it might take up to 24 hours for products to appear. |
| `name` | String | Identifier. The resource name of the autofeed settings. Format: `accounts/{account}/autofeedSettings`. |
| `eligible` | bool | Output only. Determines whether the business is eligible for being enrolled into an autofeed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access autofeed_setting outputs
autofeed_setting_id = autofeed_setting.id
autofeed_setting_enable_products = autofeed_setting.enable_products
autofeed_setting_name = autofeed_setting.name
autofeed_setting_eligible = autofeed_setting.eligible
```

---


### Checkout_setting

Creates `CheckoutSettings` for the given merchant.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `effective_review_state` | String |  | Output only. The effective value of `review_state` for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |
| `uri_settings` | String |  | URI settings for cart or checkout URL. |
| `effective_uri_settings` | String |  | Output only. The effective value of `uri_settings` for a given merchant. If account level settings are present then this value will be a copy of url settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |
| `eligible_destinations` | Vec<String> |  | Optional. The destinations (also known as [Marketing methods](https://support.google.com/merchants/answer/15130232)) to which the checkout program applies, valid destination values are `SHOPPING_ADS`, `FREE_LISTINGS` |
| `enrollment_state` | String |  | Output only. Reflects the merchant enrollment state in `Checkout` program. |
| `name` | String |  | Identifier. The resource name of the program configuration settings. Format: `accounts/{account}/programs/{program}/checkoutSettings` |
| `review_state` | String |  | Output only. Reflects the merchant review state in `Checkout` program. This is set based on the data quality reviews of the URL provided by the merchant. A merchant with enrollment state as `ENROLLED` can be in the following review states: `IN_REVIEW`, `APPROVED` or `DISAPPROVED`. A merchant must be in an `enrollment_state` of `ENROLLED` before a review can begin for the merchant.For more details, check the help center doc. |
| `effective_enrollment_state` | String |  | Output only. The effective value of enrollment_state for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |
| `parent` | String | ✅ | Required. The merchant account for which the `CheckoutSettings` will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `effective_review_state` | String | Output only. The effective value of `review_state` for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |
| `uri_settings` | String | URI settings for cart or checkout URL. |
| `effective_uri_settings` | String | Output only. The effective value of `uri_settings` for a given merchant. If account level settings are present then this value will be a copy of url settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |
| `eligible_destinations` | Vec<String> | Optional. The destinations (also known as [Marketing methods](https://support.google.com/merchants/answer/15130232)) to which the checkout program applies, valid destination values are `SHOPPING_ADS`, `FREE_LISTINGS` |
| `enrollment_state` | String | Output only. Reflects the merchant enrollment state in `Checkout` program. |
| `name` | String | Identifier. The resource name of the program configuration settings. Format: `accounts/{account}/programs/{program}/checkoutSettings` |
| `review_state` | String | Output only. Reflects the merchant review state in `Checkout` program. This is set based on the data quality reviews of the URL provided by the merchant. A merchant with enrollment state as `ENROLLED` can be in the following review states: `IN_REVIEW`, `APPROVED` or `DISAPPROVED`. A merchant must be in an `enrollment_state` of `ENROLLED` before a review can begin for the merchant.For more details, check the help center doc. |
| `effective_enrollment_state` | String | Output only. The effective value of enrollment_state for a given merchant ID. If account level settings are present then this value will be a copy of the account level settings. Otherwise, it will have the value of the parent account (for only marketplace sellers). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create checkout_setting
checkout_setting = provider.merchantapi_api.Checkout_setting {
    parent = "value"  # Required. The merchant account for which the `CheckoutSettings` will be created.
}

# Access checkout_setting outputs
checkout_setting_id = checkout_setting.id
checkout_setting_effective_review_state = checkout_setting.effective_review_state
checkout_setting_uri_settings = checkout_setting.uri_settings
checkout_setting_effective_uri_settings = checkout_setting.effective_uri_settings
checkout_setting_eligible_destinations = checkout_setting.eligible_destinations
checkout_setting_enrollment_state = checkout_setting.enrollment_state
checkout_setting_name = checkout_setting.name
checkout_setting_review_state = checkout_setting.review_state
checkout_setting_effective_enrollment_state = checkout_setting.effective_enrollment_state
```

---


### Automatic_improvement

Retrieves the automatic improvements of an account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `shipping_improvements` | String |  | Not available for [advanced accounts](https://support.google.com/merchants/answer/188487). By turning on [automatic shipping improvements](https://support.google.com/merchants/answer/10027038), you are allowing Google to improve the accuracy of your delivery times shown to shoppers using Google. More accurate delivery times, especially when faster, typically lead to better conversion rates. Google will improve your estimated delivery times based on various factors: * Delivery address of an order * Current handling time and shipping time settings * Estimated weekdays or business days * Parcel tracking data This field is only updated (cleared) if provided in the update mask. |
| `image_improvements` | String |  | This improvement will attempt to automatically correct submitted images if they don't meet the [image requirements](https://support.google.com/merchants/answer/6324350), for example, removing overlays. If successful, the image will be replaced and approved. This improvement is only applied to images of disapproved offers. For more information see: [Automatic image improvements](https://support.google.com/merchants/answer/9242973) This field is only updated (cleared) if provided in the update mask. |
| `name` | String |  | Identifier. The resource name of the automatic improvements. Format: `accounts/{account}/automaticImprovements`. |
| `item_updates` | String |  | Turning on [item updates](https://support.google.com/merchants/answer/3246284) allows Google to automatically update items for you. When item updates are on, Google uses the structured data markup on the website and advanced data extractors to update the price and availability of the items. When the item updates are off, items with mismatched data aren't shown. This field is only updated (cleared) if provided in the update mask. |
| `name` | String | ✅ | Identifier. The resource name of the automatic improvements. Format: `accounts/{account}/automaticImprovements`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `shipping_improvements` | String | Not available for [advanced accounts](https://support.google.com/merchants/answer/188487). By turning on [automatic shipping improvements](https://support.google.com/merchants/answer/10027038), you are allowing Google to improve the accuracy of your delivery times shown to shoppers using Google. More accurate delivery times, especially when faster, typically lead to better conversion rates. Google will improve your estimated delivery times based on various factors: * Delivery address of an order * Current handling time and shipping time settings * Estimated weekdays or business days * Parcel tracking data This field is only updated (cleared) if provided in the update mask. |
| `image_improvements` | String | This improvement will attempt to automatically correct submitted images if they don't meet the [image requirements](https://support.google.com/merchants/answer/6324350), for example, removing overlays. If successful, the image will be replaced and approved. This improvement is only applied to images of disapproved offers. For more information see: [Automatic image improvements](https://support.google.com/merchants/answer/9242973) This field is only updated (cleared) if provided in the update mask. |
| `name` | String | Identifier. The resource name of the automatic improvements. Format: `accounts/{account}/automaticImprovements`. |
| `item_updates` | String | Turning on [item updates](https://support.google.com/merchants/answer/3246284) allows Google to automatically update items for you. When item updates are on, Google uses the structured data markup on the website and advanced data extractors to update the price and availability of the items. When the item updates are off, items with mismatched data aren't shown. This field is only updated (cleared) if provided in the update mask. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access automatic_improvement outputs
automatic_improvement_id = automatic_improvement.id
automatic_improvement_shipping_improvements = automatic_improvement.shipping_improvements
automatic_improvement_image_improvements = automatic_improvement.image_improvements
automatic_improvement_name = automatic_improvement.name
automatic_improvement_item_updates = automatic_improvement.item_updates
```

---


### Service

Propose an account service.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_service` | String |  | Required. The account service to propose. |
| `provider` | String |  | Required. The provider of the service. Either the reference to an account such as `providers/123` or a well-known service provider (one of `providers/GOOGLE_ADS` or `providers/GOOGLE_BUSINESS_PROFILE`). |
| `parent` | String | ✅ | Required. The resource name of the parent account for the service. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `external_account_id` | String | Immutable. An optional, immutable identifier that Google uses to refer to this account when communicating with the provider. This should be the unique account ID within the provider's system (for example, your shop ID in Shopify). If you have multiple accounts with the same provider - for instance, different accounts for various regions — the `external_account_id` differentiates between them, ensuring accurate linking and integration between Google and the provider. |
| `handshake` | String | Output only. Information about the state of the service in terms of establishing it (e.g. is it pending approval or approved). |
| `provider_display_name` | String | Output only. The human-readable display name of the provider account. |
| `account_aggregation` | String | Service type for account aggregation. This enables the provider, which is an advanced account, to manage multiple sub-accounts (client accounts). Through this service, the advanced account provider can perform administrative and operational tasks across all linked sub-accounts. This is useful for agencies, aggregators, or large retailers that need centralized control over many Merchant Center accounts. |
| `local_listing_management` | String | Service type for local listings management. The business group associated with the external account id will be used to provide local inventory to this Merchant Center account. |
| `account_management` | String | Service type for account management. Enables the provider to perform administrative actions on the business's account, such as configuring account settings, managing users, or updating business information. |
| `campaigns_management` | String | Service type for managing advertising campaigns. Grants the provider access to create and manage the business's ad campaigns, including setting up campaigns, adjusting bids, and optimizing performance. |
| `name` | String | Identifier. The resource name of the account service. Format: `accounts/{account}/services/{service}` |
| `mutability` | String | Output only. Whether the service is mutable (e.g. through Approve / Reject RPCs). A service that was created through another system or API might be immutable. |
| `products_management` | String | Service type for managing products. This allows the provider to handle product data on behalf of the business, including reading and writing product listings. It's commonly used when the provider offers inventory management or catalog synchronization services to keep the business's product information up-to-date across platforms. |
| `provider` | String | Output only. The provider of the service. Either the reference to an account such as `providers/123` or a well-known service provider (one of `providers/GOOGLE_ADS` or `providers/GOOGLE_BUSINESS_PROFILE`). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.merchantapi_api.Service {
    parent = "value"  # Required. The resource name of the parent account for the service. Format: `accounts/{account}`
}

# Access service outputs
service_id = service.id
service_external_account_id = service.external_account_id
service_handshake = service.handshake
service_provider_display_name = service.provider_display_name
service_account_aggregation = service.account_aggregation
service_local_listing_management = service.local_listing_management
service_account_management = service.account_management
service_campaigns_management = service.campaigns_management
service_name = service.name
service_mutability = service.mutability
service_products_management = service.products_management
service_provider = service.provider
```

---


### Program

Disable participation in the specified program for the account.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the program for which to disable participation for the given account. Format: `accounts/{account}/programs/{program}`. For example, `accounts/123456/programs/free-listings`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `documentation_uri` | String | Output only. The URL of a Merchant Center help page describing the program. |
| `unmet_requirements` | Vec<String> | Output only. The requirements that the account has not yet satisfied that are affecting participation in the program. |
| `active_region_codes` | Vec<String> | Output only. The regions in which the account is actively participating in the program. Active regions are defined as those where all program requirements affecting the regions have been met. Region codes are defined by [CLDR](https://cldr.unicode.org/). This is either a country where the program applies specifically to that country or `001` when the program applies globally. |
| `name` | String | Identifier. The resource name of the program. Format: `accounts/{account}/programs/{program}` |
| `state` | String | Output only. The participation state of the account in the program. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create program
program = provider.merchantapi_api.Program {
    name = "value"  # Required. The name of the program for which to disable participation for the given account. Format: `accounts/{account}/programs/{program}`. For example, `accounts/123456/programs/free-listings`.
}

# Access program outputs
program_id = program.id
program_documentation_uri = program.documentation_uri
program_unmet_requirements = program.unmet_requirements
program_active_region_codes = program.active_region_codes
program_name = program.name
program_state = program.state
```

---


### Developer_registration

Registers the GCP used for the API call to the shopping account passed in the request. Will create a user with an "API developer" and add the "developer_email" as a contact with "API notifications" email preference on.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `developer_email` | String |  | Immutable. If the developer email provided is associated with a user in the merchant account provided, the user will be updated to have "API developer" access type and the email preference corresponding to that user will be updated to have the new "API notifications" preference. If the developer email provided is not associated with any user we will just add it as a contact. The email preference corresponding to that contact will have the new "API notifications" preference. Make sure the email used is associated with a Google Account (Google Workspace account or Gmail account) and is not a service account as service accounts can't receive emails. |
| `name` | String | ✅ | Required. The name of the developer registration to be created for the merchant account that the GCP will be registered with. Format: `accounts/{account}/developerRegistration` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the merchant account id that the GCP is registered with. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create developer_registration
developer_registration = provider.merchantapi_api.Developer_registration {
    name = "value"  # Required. The name of the developer registration to be created for the merchant account that the GCP will be registered with. Format: `accounts/{account}/developerRegistration`
}

# Access developer_registration outputs
developer_registration_id = developer_registration.id
developer_registration_name = developer_registration.name
```

---


### Region

Creates a region definition in your Merchant Center account. Executing this method requires admin access.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. The display name of the region. |
| `postal_code_area` | String |  | Optional. A list of postal codes that defines the region area. |
| `name` | String |  | Identifier. The resource name of the region. Format: `accounts/{account}/regions/{region}` |
| `geotarget_area` | String |  | Optional. A list of geotargets that defines the region area. |
| `regional_inventory_eligible` | bool |  | Output only. Indicates if the region is eligible for use in the Regional Inventory configuration. |
| `shipping_eligible` | bool |  | Output only. Indicates if the region is eligible for use in the Shipping Services configuration. |
| `parent` | String | ✅ | Required. The account to create a region for. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. The display name of the region. |
| `postal_code_area` | String | Optional. A list of postal codes that defines the region area. |
| `name` | String | Identifier. The resource name of the region. Format: `accounts/{account}/regions/{region}` |
| `geotarget_area` | String | Optional. A list of geotargets that defines the region area. |
| `regional_inventory_eligible` | bool | Output only. Indicates if the region is eligible for use in the Regional Inventory configuration. |
| `shipping_eligible` | bool | Output only. Indicates if the region is eligible for use in the Shipping Services configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create region
region = provider.merchantapi_api.Region {
    parent = "value"  # Required. The account to create a region for. Format: `accounts/{account}`
}

# Access region outputs
region_id = region.id
region_display_name = region.display_name
region_postal_code_area = region.postal_code_area
region_name = region.name
region_geotarget_area = region.geotarget_area
region_regional_inventory_eligible = region.regional_inventory_eligible
region_shipping_eligible = region.shipping_eligible
```

---


### Email_preference

Returns the email preferences for a Merchant Center account user. This service only permits retrieving and updating email preferences for the authenticated user. Use the name=accounts/*/users/me/emailPreferences alias to get preferences for the authenticated user.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `news_and_tips` | String |  | Optional. Updates on new features, tips and best practices. |
| `name` | String |  | Identifier. The name of the EmailPreferences. The endpoint is only supported for the authenticated user. |
| `name` | String | ✅ | Identifier. The name of the EmailPreferences. The endpoint is only supported for the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `news_and_tips` | String | Optional. Updates on new features, tips and best practices. |
| `name` | String | Identifier. The name of the EmailPreferences. The endpoint is only supported for the authenticated user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access email_preference outputs
email_preference_id = email_preference.id
email_preference_news_and_tips = email_preference.news_and_tips
email_preference_name = email_preference.name
```

---


### Gbp_account

Link the specified merchant to a GBP account for all countries. To run this method, you must have admin access to the Merchant Center account. If you don't have admin access, the request fails with the error message `User is not an administrator of account {ACCOUNT_ID}`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gbp_email` | String |  | Required. The email address of the Business Profile account. |
| `parent` | String | ✅ | Required. The name of the parent resource to which the GBP account is linked. Format: `accounts/{account}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gbp_accounts` | Vec<String> | The GBP accounts from the specified merchant in the specified country. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gbp_account
gbp_account = provider.merchantapi_api.Gbp_account {
    parent = "value"  # Required. The name of the parent resource to which the GBP account is linked. Format: `accounts/{account}`.
}

# Access gbp_account outputs
gbp_account_id = gbp_account.id
gbp_account_gbp_accounts = gbp_account.gbp_accounts
gbp_account_next_page_token = gbp_account.next_page_token
```

---


### Business_info

Retrieves the business info of an account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer_service` | String |  | Optional. The customer service of the business. |
| `address` | String |  | Optional. The address of the business. Only `region_code`, `address_lines`, `postal_code`, `administrative_area` and `locality` fields are supported. All other fields are ignored. |
| `name` | String |  | Identifier. The resource name of the business info. Format: `accounts/{account}/businessInfo` |
| `korean_business_registration_number` | String |  | Optional. The 10-digit [Korean business registration number](https://support.google.com/merchants/answer/9037766) separated with dashes in the format: XXX-XX-XXXXX. |
| `phone_verification_state` | String |  | Output only. The phone verification state of the business. |
| `phone` | String |  | Output only. The phone number of the business. |
| `name` | String | ✅ | Identifier. The resource name of the business info. Format: `accounts/{account}/businessInfo` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer_service` | String | Optional. The customer service of the business. |
| `address` | String | Optional. The address of the business. Only `region_code`, `address_lines`, `postal_code`, `administrative_area` and `locality` fields are supported. All other fields are ignored. |
| `name` | String | Identifier. The resource name of the business info. Format: `accounts/{account}/businessInfo` |
| `korean_business_registration_number` | String | Optional. The 10-digit [Korean business registration number](https://support.google.com/merchants/answer/9037766) separated with dashes in the format: XXX-XX-XXXXX. |
| `phone_verification_state` | String | Output only. The phone verification state of the business. |
| `phone` | String | Output only. The phone number of the business. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access business_info outputs
business_info_id = business_info.id
business_info_customer_service = business_info.customer_service
business_info_address = business_info.address
business_info_name = business_info.name
business_info_korean_business_registration_number = business_info.korean_business_registration_number
business_info_phone_verification_state = business_info.phone_verification_state
business_info_phone = business_info.phone
```

---


### User

Creates a Merchant Center account user. Executing this method requires admin access.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The state of the user. |
| `access_rights` | Vec<String> |  | Required. The [access rights](https://support.google.com/merchants/answer/12160472?sjid=6789834943175119429-EU#accesstypes) the user has. |
| `name` | String |  | Identifier. The resource name of the user. Format: `accounts/{account}/user/{email}` Use `me` to refer to your own email address, for example `accounts/{account}/users/me`. |
| `parent` | String | ✅ | Required. The resource name of the account for which a user will be created. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The state of the user. |
| `access_rights` | Vec<String> | Required. The [access rights](https://support.google.com/merchants/answer/12160472?sjid=6789834943175119429-EU#accesstypes) the user has. |
| `name` | String | Identifier. The resource name of the user. Format: `accounts/{account}/user/{email}` Use `me` to refer to your own email address, for example `accounts/{account}/users/me`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.merchantapi_api.User {
    parent = "value"  # Required. The resource name of the account for which a user will be created. Format: `accounts/{account}`
}

# Access user outputs
user_id = user.id
user_state = user.state
user_access_rights = user.access_rights
user_name = user.name
```

---


### Homepage

Unclaims a store's homepage. Executing this method requires admin access.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the homepage to unclaim. Format: `accounts/{account}/homepage` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uri` | String | Required. The URI (typically a URL) of the store's homepage. |
| `claimed` | bool | Output only. Whether the homepage is claimed. See https://support.google.com/merchants/answer/176793. |
| `name` | String | Identifier. The resource name of the store's homepage. Format: `accounts/{account}/homepage` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create homepage
homepage = provider.merchantapi_api.Homepage {
    name = "value"  # Required. The name of the homepage to unclaim. Format: `accounts/{account}/homepage`
}

# Access homepage outputs
homepage_id = homepage.id
homepage_uri = homepage.uri
homepage_claimed = homepage.claimed
homepage_name = homepage.name
```

---


### Online_return_policie

Creates a new return policy for a given business.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `process_refund_days` | i64 |  | Optional. The field specifies the number of days it takes for business to process refunds. |
| `return_policy_id` | String |  | Output only. Return policy ID generated by Google. |
| `return_methods` | Vec<String> |  | Optional. The return methods of how customers can return an item. This value is required to not be empty unless the type of return policy is noReturns. |
| `accept_defective_only` | bool |  | Optional. This field specifies if business only accepts defective products for returns. |
| `countries` | Vec<String> |  | Required. Immutable. The countries of sale where the return policy applies. The values must be a valid 2 letter ISO 3166 code. |
| `label` | String |  | Optional. Immutable. This field represents the unique user-defined label of the return policy for the given country. It is important to note that the same label cannot be used in different return policies for the same country. If not given, policies will be automatically treated as the 'default' for the country. When using label, you are creating an exception policy in that country to assign a custom return policy to certain product groups, follow the instructions provided in the [Return policy label] (https://support.google.com/merchants/answer/9445425). The label can contain up to 50 characters. |
| `return_policy_uri` | String |  | Required. The return policy uri. This can used by Google to do a sanity check for the policy. It must be a valid URL. |
| `item_conditions` | Vec<String> |  | Optional. The item conditions accepted for returns must not be empty unless the type of return policy is 'noReturns'. |
| `accept_exchange` | bool |  | Optional. This field specifies if business allows customers to exchange products. |
| `return_label_source` | String |  | Optional. The field specifies the return label source. |
| `seasonal_overrides` | Vec<String> |  | Optional. Overrides to the general policy for orders placed during a specific set of time intervals. |
| `policy` | String |  | Optional. The return policy. |
| `return_shipping_fee` | String |  | Optional. The return shipping fee. Should be set only when customer need to download and print the return label. |
| `name` | String |  | Identifier. The name of the `OnlineReturnPolicy` resource. Format: `accounts/{account}/onlineReturnPolicies/{return_policy}` |
| `restocking_fee` | String |  | Optional. The restocking fee that applies to all return reason categories. This would be treated as a free restocking fee if the value is not set. |
| `parent` | String | ✅ | Required. The Merchant Center account for which the return policy will be created. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `process_refund_days` | i64 | Optional. The field specifies the number of days it takes for business to process refunds. |
| `return_policy_id` | String | Output only. Return policy ID generated by Google. |
| `return_methods` | Vec<String> | Optional. The return methods of how customers can return an item. This value is required to not be empty unless the type of return policy is noReturns. |
| `accept_defective_only` | bool | Optional. This field specifies if business only accepts defective products for returns. |
| `countries` | Vec<String> | Required. Immutable. The countries of sale where the return policy applies. The values must be a valid 2 letter ISO 3166 code. |
| `label` | String | Optional. Immutable. This field represents the unique user-defined label of the return policy for the given country. It is important to note that the same label cannot be used in different return policies for the same country. If not given, policies will be automatically treated as the 'default' for the country. When using label, you are creating an exception policy in that country to assign a custom return policy to certain product groups, follow the instructions provided in the [Return policy label] (https://support.google.com/merchants/answer/9445425). The label can contain up to 50 characters. |
| `return_policy_uri` | String | Required. The return policy uri. This can used by Google to do a sanity check for the policy. It must be a valid URL. |
| `item_conditions` | Vec<String> | Optional. The item conditions accepted for returns must not be empty unless the type of return policy is 'noReturns'. |
| `accept_exchange` | bool | Optional. This field specifies if business allows customers to exchange products. |
| `return_label_source` | String | Optional. The field specifies the return label source. |
| `seasonal_overrides` | Vec<String> | Optional. Overrides to the general policy for orders placed during a specific set of time intervals. |
| `policy` | String | Optional. The return policy. |
| `return_shipping_fee` | String | Optional. The return shipping fee. Should be set only when customer need to download and print the return label. |
| `name` | String | Identifier. The name of the `OnlineReturnPolicy` resource. Format: `accounts/{account}/onlineReturnPolicies/{return_policy}` |
| `restocking_fee` | String | Optional. The restocking fee that applies to all return reason categories. This would be treated as a free restocking fee if the value is not set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create online_return_policie
online_return_policie = provider.merchantapi_api.Online_return_policie {
    parent = "value"  # Required. The Merchant Center account for which the return policy will be created. Format: `accounts/{account}`
}

# Access online_return_policie outputs
online_return_policie_id = online_return_policie.id
online_return_policie_process_refund_days = online_return_policie.process_refund_days
online_return_policie_return_policy_id = online_return_policie.return_policy_id
online_return_policie_return_methods = online_return_policie.return_methods
online_return_policie_accept_defective_only = online_return_policie.accept_defective_only
online_return_policie_countries = online_return_policie.countries
online_return_policie_label = online_return_policie.label
online_return_policie_return_policy_uri = online_return_policie.return_policy_uri
online_return_policie_item_conditions = online_return_policie.item_conditions
online_return_policie_accept_exchange = online_return_policie.accept_exchange
online_return_policie_return_label_source = online_return_policie.return_label_source
online_return_policie_seasonal_overrides = online_return_policie.seasonal_overrides
online_return_policie_policy = online_return_policie.policy
online_return_policie_return_shipping_fee = online_return_policie.return_shipping_fee
online_return_policie_name = online_return_policie.name
online_return_policie_restocking_fee = online_return_policie.restocking_fee
```

---


### Terms_of_service_agreement_state

Returns the state of a terms of service agreement.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `required` | String | Optional. The required terms of service |
| `region_code` | String | Required. Region code as defined by https://cldr.unicode.org/. This is the country the current state applies to. |
| `terms_of_service_kind` | String | Required. Terms of Service kind associated with the particular version. |
| `name` | String | Identifier. The resource name of the terms of service version. Format: `accounts/{account}/termsOfServiceAgreementState/{identifier}` The identifier format is: `{TermsOfServiceKind}-{country}` For example, an identifier could be: `MERCHANT_CENTER-EU` or `MERCHANT_CENTER-US`. |
| `accepted` | String | Optional. The accepted terms of service of this kind and for the associated region_code |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access terms_of_service_agreement_state outputs
terms_of_service_agreement_state_id = terms_of_service_agreement_state.id
terms_of_service_agreement_state_required = terms_of_service_agreement_state.required
terms_of_service_agreement_state_region_code = terms_of_service_agreement_state.region_code
terms_of_service_agreement_state_terms_of_service_kind = terms_of_service_agreement_state.terms_of_service_kind
terms_of_service_agreement_state_name = terms_of_service_agreement_state.name
terms_of_service_agreement_state_accepted = terms_of_service_agreement_state.accepted
```

---


### Shipping_setting

Replace the shipping setting of a business with the request shipping setting. Executing this method requires admin access.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `warehouses` | Vec<String> |  | Optional. A list of warehouses which can be referred to in `services`. |
| `name` | String |  | Identifier. The resource name of the shipping settings. Format: `accounts/{account}/shippingSettings`. For example, `accounts/123456/shippingSettings`. |
| `etag` | String |  | Required. This field helps avoid async issues. It ensures that the shipping setting data doesn't change between the `get` call and the `insert` call. The user should follow these steps: 1. Set the etag field as an empty string for the initial shipping setting creation. 2. After the initial creation, call the `get` method to obtain an etag and the current shipping setting data before calling `insert`. 3. Modify the shipping setting information. 4. Call the `insert` method with the shipping setting information and the etag obtained in step 2. 5. If the shipping setting data changes between step 2 and step 4, the insert request will fail because the etag changes every time the shipping setting data changes. In this case, the user should repeat steps 2-4 with the new etag. |
| `services` | Vec<String> |  | Optional. The target account's list of services. |
| `parent` | String | ✅ | Required. The account for which this shipping setting will be inserted. If you are using an advanced account, you must specify the unique identifier of the sub-account for which you want to insert the shipping setting. Format: `accounts/{ACCOUNT_ID}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `warehouses` | Vec<String> | Optional. A list of warehouses which can be referred to in `services`. |
| `name` | String | Identifier. The resource name of the shipping settings. Format: `accounts/{account}/shippingSettings`. For example, `accounts/123456/shippingSettings`. |
| `etag` | String | Required. This field helps avoid async issues. It ensures that the shipping setting data doesn't change between the `get` call and the `insert` call. The user should follow these steps: 1. Set the etag field as an empty string for the initial shipping setting creation. 2. After the initial creation, call the `get` method to obtain an etag and the current shipping setting data before calling `insert`. 3. Modify the shipping setting information. 4. Call the `insert` method with the shipping setting information and the etag obtained in step 2. 5. If the shipping setting data changes between step 2 and step 4, the insert request will fail because the etag changes every time the shipping setting data changes. In this case, the user should repeat steps 2-4 with the new etag. |
| `services` | Vec<String> | Optional. The target account's list of services. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create shipping_setting
shipping_setting = provider.merchantapi_api.Shipping_setting {
    parent = "value"  # Required. The account for which this shipping setting will be inserted. If you are using an advanced account, you must specify the unique identifier of the sub-account for which you want to insert the shipping setting. Format: `accounts/{ACCOUNT_ID}`
}

# Access shipping_setting outputs
shipping_setting_id = shipping_setting.id
shipping_setting_warehouses = shipping_setting.warehouses
shipping_setting_name = shipping_setting.name
shipping_setting_etag = shipping_setting.etag
shipping_setting_services = shipping_setting.services
```

---


### Relationship

Retrieve an account relationship.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `provider_display_name` | String |  | Output only. The human-readable display name of the provider account. |
| `provider` | String |  | Immutable. The provider of the service. Either the reference to an account such as `providers/123` or a well-known service provider (one of `providers/GOOGLE_ADS` or `providers/GOOGLE_BUSINESS_PROFILE`). |
| `account_id_alias` | String |  | Optional. An optional alias you can assign to this account relationship. This alias acts as a convenient identifier for your own reference and management. It must be unique among all your account relationships with the same provider. For example, you might use `account_id_alias` to assign a friendly name to this relationship for easier identification in your systems. |
| `name` | String |  | Identifier. The resource name of the account relationship. Format: `accounts/{account}/relationships/{relationship}`. For example, `accounts/123456/relationships/567890`. |
| `name` | String | ✅ | Identifier. The resource name of the account relationship. Format: `accounts/{account}/relationships/{relationship}`. For example, `accounts/123456/relationships/567890`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `provider_display_name` | String | Output only. The human-readable display name of the provider account. |
| `provider` | String | Immutable. The provider of the service. Either the reference to an account such as `providers/123` or a well-known service provider (one of `providers/GOOGLE_ADS` or `providers/GOOGLE_BUSINESS_PROFILE`). |
| `account_id_alias` | String | Optional. An optional alias you can assign to this account relationship. This alias acts as a convenient identifier for your own reference and management. It must be unique among all your account relationships with the same provider. For example, you might use `account_id_alias` to assign a friendly name to this relationship for easier identification in your systems. |
| `name` | String | Identifier. The resource name of the account relationship. Format: `accounts/{account}/relationships/{relationship}`. For example, `accounts/123456/relationships/567890`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access relationship outputs
relationship_id = relationship.id
relationship_provider_display_name = relationship.provider_display_name
relationship_provider = relationship.provider
relationship_account_id_alias = relationship.account_id_alias
relationship_name = relationship.name
```

---


### Lfp_provider

Link the specified merchant to a LFP provider for the specified country.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `external_account_id` | String |  | Required. The external account ID by which this merchant is known to the LFP provider. |
| `name` | String | ✅ | Required. The name of the LFP provider resource to link. Format: `accounts/{account}/omnichannelSettings/{omnichannel_setting}/lfpProviders/{lfp_provider}`. The `lfp_provider` is the LFP provider ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lfp_providers` | Vec<String> | The LFP providers from the specified merchant in the specified country. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lfp_provider
lfp_provider = provider.merchantapi_api.Lfp_provider {
    name = "value"  # Required. The name of the LFP provider resource to link. Format: `accounts/{account}/omnichannelSettings/{omnichannel_setting}/lfpProviders/{lfp_provider}`. The `lfp_provider` is the LFP provider ID.
}

# Access lfp_provider outputs
lfp_provider_id = lfp_provider.id
lfp_provider_lfp_providers = lfp_provider.lfp_providers
lfp_provider_next_page_token = lfp_provider.next_page_token
```

---


### Issue

Lists all account issues of a Merchant Center account. When called on a multi-client account, this method only returns issues belonging to that account, not its sub-accounts. To retrieve issues for sub-accounts, you must first call the accounts.listSubaccounts method to obtain a list of sub-accounts, and then call `accounts.issues.list` for each sub-account individually.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `account_issues` | Vec<String> | The issues from the specified account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access issue outputs
issue_id = issue.id
issue_next_page_token = issue.next_page_token
issue_account_issues = issue.account_issues
```

---


### Merchant_review

Inserts a review for your Merchant Center account. If the review already exists, then the review is replaced with the new instance.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_attributes` | Vec<String> |  | Optional. A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the data specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as experimental attributes. Maximum allowed number of characters for each custom attribute is 10240 (represents sum of characters for name and value). Maximum 2500 custom attributes can be set per product, with total size of 102.4kB. Underscores in custom attribute names are replaced by spaces upon insertion. |
| `merchant_review_attributes` | String |  | Optional. A list of merchant review attributes. |
| `merchant_review_id` | String |  | Required. The user provided merchant review ID to uniquely identify the merchant review. |
| `data_source` | String |  | Output only. The primary data source of the merchant review. |
| `name` | String |  | Identifier. The name of the merchant review. Format: `"{merchantreview.name=accounts/{account}/merchantReviews/{merchantReview}}"` |
| `merchant_review_status` | String |  | Output only. The status of a merchant review, data validation issues, that is, information about a merchant review computed asynchronously. |
| `parent` | String | ✅ | Required. The account where the merchant review will be inserted. Format: accounts/{account} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_attributes` | Vec<String> | Optional. A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the data specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as experimental attributes. Maximum allowed number of characters for each custom attribute is 10240 (represents sum of characters for name and value). Maximum 2500 custom attributes can be set per product, with total size of 102.4kB. Underscores in custom attribute names are replaced by spaces upon insertion. |
| `merchant_review_attributes` | String | Optional. A list of merchant review attributes. |
| `merchant_review_id` | String | Required. The user provided merchant review ID to uniquely identify the merchant review. |
| `data_source` | String | Output only. The primary data source of the merchant review. |
| `name` | String | Identifier. The name of the merchant review. Format: `"{merchantreview.name=accounts/{account}/merchantReviews/{merchantReview}}"` |
| `merchant_review_status` | String | Output only. The status of a merchant review, data validation issues, that is, information about a merchant review computed asynchronously. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create merchant_review
merchant_review = provider.merchantapi_api.Merchant_review {
    parent = "value"  # Required. The account where the merchant review will be inserted. Format: accounts/{account}
}

# Access merchant_review outputs
merchant_review_id = merchant_review.id
merchant_review_custom_attributes = merchant_review.custom_attributes
merchant_review_merchant_review_attributes = merchant_review.merchant_review_attributes
merchant_review_merchant_review_id = merchant_review.merchant_review_id
merchant_review_data_source = merchant_review.data_source
merchant_review_name = merchant_review.name
merchant_review_merchant_review_status = merchant_review.merchant_review_status
```

---


### Product_review

Inserts a product review.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_review_attributes` | String |  | Optional. A list of product review attributes. |
| `product_review_status` | String |  | Output only. The status of a product review, data validation issues, that is, information about a product review computed asynchronously. |
| `custom_attributes` | Vec<String> |  | Optional. A list of custom (merchant-provided) attributes. |
| `data_source` | String |  | Output only. The primary data source of the product review. |
| `product_review_id` | String |  | Required. The permanent, unique identifier for the product review in the publisher’s system. |
| `name` | String |  | Identifier. The name of the product review. Format: `"{productreview.name=accounts/{account}/productReviews/{productReview}}"` |
| `parent` | String | ✅ | Required. The account where the product review will be inserted. Format: accounts/{account} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `product_review_attributes` | String | Optional. A list of product review attributes. |
| `product_review_status` | String | Output only. The status of a product review, data validation issues, that is, information about a product review computed asynchronously. |
| `custom_attributes` | Vec<String> | Optional. A list of custom (merchant-provided) attributes. |
| `data_source` | String | Output only. The primary data source of the product review. |
| `product_review_id` | String | Required. The permanent, unique identifier for the product review in the publisher’s system. |
| `name` | String | Identifier. The name of the product review. Format: `"{productreview.name=accounts/{account}/productReviews/{productReview}}"` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product_review
product_review = provider.merchantapi_api.Product_review {
    parent = "value"  # Required. The account where the product review will be inserted. Format: accounts/{account}
}

# Access product_review outputs
product_review_id = product_review.id
product_review_product_review_attributes = product_review.product_review_attributes
product_review_product_review_status = product_review.product_review_status
product_review_custom_attributes = product_review.custom_attributes
product_review_data_source = product_review.data_source
product_review_product_review_id = product_review.product_review_id
product_review_name = product_review.name
```

---


### Quota

Lists the daily call quota and usage per group for your Merchant Center account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `quota_groups` | Vec<String> | The methods, current quota usage and limits per each group. The quota is shared between all methods in the group. The groups are sorted in descending order based on quota_usage. |
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
quota_quota_groups = quota.quota_groups
quota_next_page_token = quota.next_page_token
```

---


### Conversion_source

Creates a new conversion source.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `merchant_center_destination` | String |  | Conversion Source of type "Merchant Center Tag Destination". |
| `google_analytics_link` | String |  | Immutable. Conversion Source of type "Link to Google Analytics Property". |
| `name` | String |  | Output only. Identifier. Generated by the Content API upon creation of a new `ConversionSource`. Format: `[a-z]{4}:.+` The four characters before the colon represent the type of conversion source. Content after the colon represents the ID of the conversion source within that type. The ID of two different conversion sources might be the same across different types. The following type prefixes are supported: * `galk`: For GoogleAnalyticsLink sources. * `mcdn`: For MerchantCenterDestination sources. |
| `controller` | String |  | Output only. Controller of the conversion source. |
| `expire_time` | String |  | Output only. The time when an archived conversion source becomes permanently deleted and is no longer available to undelete. |
| `state` | String |  | Output only. Current state of this conversion source. Can't be edited through the API. |
| `parent` | String | ✅ | Required. The merchant account that will own the new conversion source. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `merchant_center_destination` | String | Conversion Source of type "Merchant Center Tag Destination". |
| `google_analytics_link` | String | Immutable. Conversion Source of type "Link to Google Analytics Property". |
| `name` | String | Output only. Identifier. Generated by the Content API upon creation of a new `ConversionSource`. Format: `[a-z]{4}:.+` The four characters before the colon represent the type of conversion source. Content after the colon represents the ID of the conversion source within that type. The ID of two different conversion sources might be the same across different types. The following type prefixes are supported: * `galk`: For GoogleAnalyticsLink sources. * `mcdn`: For MerchantCenterDestination sources. |
| `controller` | String | Output only. Controller of the conversion source. |
| `expire_time` | String | Output only. The time when an archived conversion source becomes permanently deleted and is no longer available to undelete. |
| `state` | String | Output only. Current state of this conversion source. Can't be edited through the API. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversion_source
conversion_source = provider.merchantapi_api.Conversion_source {
    parent = "value"  # Required. The merchant account that will own the new conversion source. Format: `accounts/{account}`
}

# Access conversion_source outputs
conversion_source_id = conversion_source.id
conversion_source_merchant_center_destination = conversion_source.merchant_center_destination
conversion_source_google_analytics_link = conversion_source.google_analytics_link
conversion_source_name = conversion_source.name
conversion_source_controller = conversion_source.controller
conversion_source_expire_time = conversion_source.expire_time
conversion_source_state = conversion_source.state
```

---


### Local_inventorie

Inserts a `LocalInventory` resource to a product in your merchant account. Replaces the full `LocalInventory` resource if an entry with the same `storeCode` already exists for the product. It might take up to 30 minutes for the new or updated `LocalInventory` resource to appear in products.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account` | String |  | Output only. The account that owns the product. This field will be ignored if set by the client. |
| `name` | String |  | Output only. The name of the `LocalInventory` resource. Format: `accounts/{account}/products/{product}/localInventories/{store_code}` |
| `local_inventory_attributes` | String |  | Optional. A list of local inventory attributes. |
| `store_code` | String |  | Required. Immutable. Store code (the store ID from your Business Profile) of the physical store the product is sold in. See the [Local product inventory data specification](https://support.google.com/merchants/answer/3061342) for more information. |
| `parent` | String | ✅ | Required. The account and product where this inventory will be inserted. Format: `accounts/{account}/products/{product}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `local_inventories` | Vec<String> | The `LocalInventory` resources for the given product from the specified account. |
| `next_page_token` | String | A token, which can be sent as `pageToken` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create local_inventorie
local_inventorie = provider.merchantapi_api.Local_inventorie {
    parent = "value"  # Required. The account and product where this inventory will be inserted. Format: `accounts/{account}/products/{product}`
}

# Access local_inventorie outputs
local_inventorie_id = local_inventorie.id
local_inventorie_local_inventories = local_inventorie.local_inventories
local_inventorie_next_page_token = local_inventorie.next_page_token
```

---


### Regional_inventorie

Inserts a `RegionalInventory` to a given product in your merchant account. Replaces the full `RegionalInventory` resource if an entry with the same `region` already exists for the product. It might take up to 30 minutes for the new or updated `RegionalInventory` resource to appear in products.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account` | String |  | Output only. The account that owns the product. This field will be ignored if set by the client. |
| `name` | String |  | Output only. The name of the `RegionalInventory` resource. Format: `{regional_inventory.name=accounts/{account}/products/{product}/regionalInventories/{region}` |
| `region` | String |  | Required. Immutable. ID of the region for this `RegionalInventory` resource. See the [Regional availability and pricing](https://support.google.com/merchants/answer/9698880) for more details. |
| `regional_inventory_attributes` | String |  | Optional. A list of regional inventory attributes. |
| `parent` | String | ✅ | Required. The account and product where this inventory will be inserted. Format: `accounts/{account}/products/{product}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `pageToken` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `regional_inventories` | Vec<String> | The `RegionalInventory` resources for the given product from the specified account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create regional_inventorie
regional_inventorie = provider.merchantapi_api.Regional_inventorie {
    parent = "value"  # Required. The account and product where this inventory will be inserted. Format: `accounts/{account}/products/{product}`
}

# Access regional_inventorie outputs
regional_inventorie_id = regional_inventorie.id
regional_inventorie_next_page_token = regional_inventorie.next_page_token
regional_inventorie_regional_inventories = regional_inventorie.regional_inventories
```

---


### Notificationsubscription

Creates a notification subscription for a business. For standalone or subaccounts accounts, the business can create a subscription for self. For MCAs, the business can create a subscription for all managed accounts or for a specific subaccount. We will allow the following types of notification subscriptions to exist together (per business as a subscriber per event type): 1. Subscription for all managed accounts + subscription for self. 2. Multiple "partial" subscriptions for managed accounts + subscription for self. we will not allow (per business as a subscriber per event type): 1. Multiple self subscriptions. 2. Multiple "all managed accounts" subscriptions. 3. "All managed accounts" subscription and partial subscriptions at the same time. 4. Multiple partial subscriptions for the same target account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `all_managed_accounts` | bool |  | If this value is true, the requesting account is notified of the specified event for all managed accounts (can be subaccounts or other linked accounts) including newly added accounts on a daily basis. |
| `name` | String |  | Output only. The `name` of the notification configuration. Generated by the Content API upon creation of a new `NotificationSubscription`. The `account` represents the merchant ID of the merchant that owns the configuration. Format: `accounts/{account}/notificationsubscriptions/{notification_subscription}` |
| `call_back_uri` | String |  | URL to be used to push the notification to the merchant. |
| `registered_event` | String |  | The event that the merchant wants to be notified about. |
| `target_account` | String |  | The `name` of the account you want to receive notifications for. Format: `accounts/{account}` |
| `parent` | String | ✅ | Required. The merchant account that owns the new notification subscription. Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `all_managed_accounts` | bool | If this value is true, the requesting account is notified of the specified event for all managed accounts (can be subaccounts or other linked accounts) including newly added accounts on a daily basis. |
| `name` | String | Output only. The `name` of the notification configuration. Generated by the Content API upon creation of a new `NotificationSubscription`. The `account` represents the merchant ID of the merchant that owns the configuration. Format: `accounts/{account}/notificationsubscriptions/{notification_subscription}` |
| `call_back_uri` | String | URL to be used to push the notification to the merchant. |
| `registered_event` | String | The event that the merchant wants to be notified about. |
| `target_account` | String | The `name` of the account you want to receive notifications for. Format: `accounts/{account}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notificationsubscription
notificationsubscription = provider.merchantapi_api.Notificationsubscription {
    parent = "value"  # Required. The merchant account that owns the new notification subscription. Format: `accounts/{account}`
}

# Access notificationsubscription outputs
notificationsubscription_id = notificationsubscription.id
notificationsubscription_all_managed_accounts = notificationsubscription.all_managed_accounts
notificationsubscription_name = notificationsubscription.name
notificationsubscription_call_back_uri = notificationsubscription.call_back_uri
notificationsubscription_registered_event = notificationsubscription.registered_event
notificationsubscription_target_account = notificationsubscription.target_account
```

---


### Lfp_merchant_state

Gets the LFP state of a merchant

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `linked_gbps` | String | Number of [GBPs](https://www.google.com/business/) this merchant has access to. |
| `inventory_stats` | String | The inventory statistics for the merchant. The field will be absent if the merchant has no inventory submitted through LFP. |
| `name` | String | Identifier. The name of the `LfpMerchantState` resource. Format: `accounts/{account}/lfpMerchantStates/{target_merchant}`. For example, `accounts/123456/lfpMerchantStates/567890`. |
| `country_settings` | Vec<String> | Country-specific settings for the merchant. |
| `store_states` | Vec<String> | Output only. The state per store from the specified merchant. The field will be absent if the merchant has no stores submitted through LFP. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access lfp_merchant_state outputs
lfp_merchant_state_id = lfp_merchant_state.id
lfp_merchant_state_linked_gbps = lfp_merchant_state.linked_gbps
lfp_merchant_state_inventory_stats = lfp_merchant_state.inventory_stats
lfp_merchant_state_name = lfp_merchant_state.name
lfp_merchant_state_country_settings = lfp_merchant_state.country_settings
lfp_merchant_state_store_states = lfp_merchant_state.store_states
```

---


### Lfp_sale

Inserts a `LfpSale` for the given merchant.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `quantity` | String |  | Required. The relative change of the available quantity. Negative for items returned. |
| `price` | String |  | Required. The unit price of the product. |
| `feed_label` | String |  | Optional. The [feed label](https://developers.google.com/shopping-content/guides/products/feed-labels) for the product. If this is not set, it will default to `regionCode`. |
| `name` | String |  | Output only. Identifier. The name of the `LfpSale` resource. Format: `accounts/{account}/lfpSales/{sale}` |
| `sale_time` | String |  | Required. The timestamp for the sale. |
| `gtin` | String |  | Required. The Global Trade Item Number of the sold product. |
| `store_code` | String |  | Required. The identifier of the merchant's store. Either a `storeCode` inserted through the API or the code of the store in the Business Profile. |
| `content_language` | String |  | Required. The two-letter ISO 639-1 language code for the item. |
| `target_account` | String |  | Required. The Merchant Center ID of the merchant to submit the sale for. |
| `uid` | String |  | Output only. System generated globally unique ID for the `LfpSale`. |
| `region_code` | String |  | Required. The [CLDR territory code](https://github.com/unicode-org/cldr/blob/latest/common/main/en.xml) for the country where the product is sold. |
| `offer_id` | String |  | Required. A unique identifier for the product. If both inventories and sales are submitted for a merchant, this id should match for the same product. **Note**: if the merchant sells the same product new and used, they should have different IDs. |
| `parent` | String | ✅ | Required. The LFP provider account. Format: `accounts/{lfp_partner}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lfp_sale
lfp_sale = provider.merchantapi_api.Lfp_sale {
    parent = "value"  # Required. The LFP provider account. Format: `accounts/{lfp_partner}`
}

```

---


### Lfp_store

Inserts a store for the target merchant. If the store with the same store code already exists, it will be replaced.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `matching_state` | String |  | Optional. Output only. The state of matching to a Google Business Profile. See matchingStateHint for further details if no match is found. |
| `matching_state_hint` | String |  | Optional. Output only. The hint of why the matching has failed. This is only set when matchingState=`STORE_MATCHING_STATE_FAILED`. Possible values are: - "`linked-store-not-found`": There aren't any Google Business Profile stores available for matching. - "`store-match-not-found`": The provided `LfpStore` couldn't be matched to any of the connected Google Business Profile stores. Merchant Center account is connected correctly and stores are available on Google Business Profile, but the `LfpStore` location address does not match with Google Business Profile stores' addresses. Update the `LfpStore` address or Google Business Profile store address to match correctly. - "`store-match-unverified`": The provided `LfpStore` couldn't be matched to any of the connected Google Business Profile stores, as the matched Google Business Profile store is unverified. Go through the Google Business Profile verification process to match correctly. |
| `place_id` | String |  | Optional. The [Google Place Id](https://developers.google.com/maps/documentation/places/web-service/place-id#id-overview) of the store location. |
| `store_address` | String |  | Required. The street address of the store. Example: 1600 Amphitheatre Pkwy, Mountain View, CA 94043, USA. |
| `store_code` | String |  | Required. Immutable. A store identifier that is unique for the target merchant. |
| `store_name` | String |  | Optional. The merchant or store name. |
| `name` | String |  | Output only. Identifier. The name of the `LfpStore` resource. Format: `accounts/{account}/lfpStores/{target_merchant}~{store_code}` |
| `gcid_category` | Vec<String> |  | Optional. [Google My Business category id](https://support.google.com/business/answer/7249669). |
| `target_account` | String |  | Required. The Merchant Center id of the merchant to submit the store for. |
| `phone_number` | String |  | Optional. The store phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. Example: `+15556767888` |
| `website_uri` | String |  | Optional. The website URL for the store or merchant. |
| `parent` | String | ✅ | Required. The LFP provider account Format: `accounts/{account}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `matching_state` | String | Optional. Output only. The state of matching to a Google Business Profile. See matchingStateHint for further details if no match is found. |
| `matching_state_hint` | String | Optional. Output only. The hint of why the matching has failed. This is only set when matchingState=`STORE_MATCHING_STATE_FAILED`. Possible values are: - "`linked-store-not-found`": There aren't any Google Business Profile stores available for matching. - "`store-match-not-found`": The provided `LfpStore` couldn't be matched to any of the connected Google Business Profile stores. Merchant Center account is connected correctly and stores are available on Google Business Profile, but the `LfpStore` location address does not match with Google Business Profile stores' addresses. Update the `LfpStore` address or Google Business Profile store address to match correctly. - "`store-match-unverified`": The provided `LfpStore` couldn't be matched to any of the connected Google Business Profile stores, as the matched Google Business Profile store is unverified. Go through the Google Business Profile verification process to match correctly. |
| `place_id` | String | Optional. The [Google Place Id](https://developers.google.com/maps/documentation/places/web-service/place-id#id-overview) of the store location. |
| `store_address` | String | Required. The street address of the store. Example: 1600 Amphitheatre Pkwy, Mountain View, CA 94043, USA. |
| `store_code` | String | Required. Immutable. A store identifier that is unique for the target merchant. |
| `store_name` | String | Optional. The merchant or store name. |
| `name` | String | Output only. Identifier. The name of the `LfpStore` resource. Format: `accounts/{account}/lfpStores/{target_merchant}~{store_code}` |
| `gcid_category` | Vec<String> | Optional. [Google My Business category id](https://support.google.com/business/answer/7249669). |
| `target_account` | String | Required. The Merchant Center id of the merchant to submit the store for. |
| `phone_number` | String | Optional. The store phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. Example: `+15556767888` |
| `website_uri` | String | Optional. The website URL for the store or merchant. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lfp_store
lfp_store = provider.merchantapi_api.Lfp_store {
    parent = "value"  # Required. The LFP provider account Format: `accounts/{account}`
}

# Access lfp_store outputs
lfp_store_id = lfp_store.id
lfp_store_matching_state = lfp_store.matching_state
lfp_store_matching_state_hint = lfp_store.matching_state_hint
lfp_store_place_id = lfp_store.place_id
lfp_store_store_address = lfp_store.store_address
lfp_store_store_code = lfp_store.store_code
lfp_store_store_name = lfp_store.store_name
lfp_store_name = lfp_store.name
lfp_store_gcid_category = lfp_store.gcid_category
lfp_store_target_account = lfp_store.target_account
lfp_store_phone_number = lfp_store.phone_number
lfp_store_website_uri = lfp_store.website_uri
```

---


### Lfp_inventorie

Inserts a `LfpInventory` resource for the given target merchant account. If the resource already exists, it will be replaced. The inventory automatically expires after 30 days.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `feed_label` | String |  | Optional. The [feed label](https://developers.google.com/shopping-content/guides/products/feed-labels) for the product. If this is not set, it will default to `regionCode`. |
| `offer_id` | String |  | Required. Immutable. A unique identifier for the product. If both inventories and sales are submitted for a merchant, this id should match for the same product. **Note**: if the merchant sells the same product new and used, they should have different IDs. |
| `target_account` | String |  | Required. The Merchant Center ID of the merchant to submit the inventory for. |
| `collection_time` | String |  | Optional. The time when the inventory is collected. If not set, it will be set to the time when the inventory is submitted. |
| `pickup_method` | String |  | Optional. Supported pickup method for this offer. Unless the value is "not supported", this field must be submitted together with `pickupSla`. For accepted attribute values, see the [local product inventory data specification](https://support.google.com/merchants/answer/3061342). |
| `gtin` | String |  | Optional. The Global Trade Item Number of the product. |
| `pickup_sla` | String |  | Optional. Expected date that an order will be ready for pickup relative to the order date. Must be submitted together with `pickupMethod`. For accepted attribute values, see the [local product inventory data specification](https://support.google.com/merchants/answer/3061342). |
| `quantity` | String |  | Optional. Quantity of the product available at this store. Must be greater than or equal to zero. |
| `availability` | String |  | Required. Availability of the product at this store. For accepted attribute values, see the [local product inventory data specification](https://support.google.com/merchants/answer/3061342) |
| `price` | String |  | Optional. The current price of the product. |
| `content_language` | String |  | Required. The two-letter ISO 639-1 language code for the item. |
| `name` | String |  | Output only. Identifier. The name for the `LfpInventory` resource. Format: `accounts/{account}/lfpInventories/{target_merchant}~{store_code}~{offer}` |
| `region_code` | String |  | Required. The [CLDR territory code](https://github.com/unicode-org/cldr/blob/latest/common/main/en.xml) for the country where the product is sold. |
| `store_code` | String |  | Required. The identifier of the merchant's store. Either the store code inserted through `InsertLfpStore` or the store code in the Business Profile. |
| `parent` | String | ✅ | Required. The LFP provider account. Format: `accounts/{account}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lfp_inventorie
lfp_inventorie = provider.merchantapi_api.Lfp_inventorie {
    parent = "value"  # Required. The LFP provider account. Format: `accounts/{account}`
}

```

---


### Report

Retrieves a report defined by a search query. The response might contain fewer rows than specified by `page_size`. Rely on `next_page_token` to determine if there are more rows to be requested.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_size` | i64 |  | Optional. Number of `ReportRows` to retrieve in a single page. Defaults to 1000. Values above 5000 are coerced to 5000. |
| `query` | String |  | Required. Query that defines a report to be retrieved. For details on how to construct your query, see the [Query Language guide](/merchant/api/guides/reports/query-language). For the full list of available tables and fields, see the [Available fields](/merchant/api/reference/rest/reports_{api_version}/accounts.reports). |
| `page_token` | String |  | Optional. Token of the page to retrieve. If not specified, the first page of results is returned. In order to request the next page of results, the value obtained from `next_page_token` in the previous response should be used. |
| `parent` | String | ✅ | Required. Id of the account making the call. Must be a standalone account or an MCA subaccount. Format: accounts/{account} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report
report = provider.merchantapi_api.Report {
    parent = "value"  # Required. Id of the account making the call. Must be a standalone account or an MCA subaccount. Format: accounts/{account}
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

# Create multiple file_upload resources
file_upload_0 = provider.merchantapi_api.File_upload {
}
file_upload_1 = provider.merchantapi_api.File_upload {
}
file_upload_2 = provider.merchantapi_api.File_upload {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    file_upload = provider.merchantapi_api.File_upload {
    }
```

---

## Related Documentation

- [GCP Merchantapi_api Documentation](https://cloud.google.com/merchantapi_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
