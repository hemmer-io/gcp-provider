# Recommendationengine_api Service



**Resources**: 6

---

## Overview

The recommendationengine_api service provides access to 6 resource types:

- [Placement](#placement) [C]
- [Prediction_api_key_registration](#prediction_api_key_registration) [CRD]
- [Catalog](#catalog) [RU]
- [Operation](#operation) [R]
- [Catalog_item](#catalog_item) [CRUD]
- [User_event](#user_event) [CR]

---

## Resources


### Placement

Makes a recommendation prediction. If using API Key based authentication, the API Key must be registered using the PredictionApiKeyRegistry service. [Learn more](https://cloud.google.com/recommendations-ai/docs/setting-up#register-key).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_event` | String |  | Required. Context about the user, what they are looking at and what action they took to trigger the predict request. Note that this user event detail won't be ingested to userEvent logs. Thus, a separate userEvent write request is required for event logging. Don't set UserInfo.visitor_id or UserInfo.user_id to the same fixed ID for different users. If you are trying to receive non-personalized recommendations (not recommended; this can negatively impact model performance), instead set UserInfo.visitor_id to a random unique ID and leave UserInfo.user_id unset. |
| `labels` | HashMap<String, String> |  | Optional. The labels for the predict request. * Label keys can contain lowercase letters, digits and hyphens, must start with a letter, and must end with a letter or digit. * Non-zero label values can contain lowercase letters, digits and hyphens, must start with a letter, and must end with a letter or digit. * No more than 64 labels can be associated with a given request. See https://goo.gl/xmQnxf for more information on and examples of labels. |
| `filter` | String |  | Optional. Filter for restricting prediction results. Accepts values for tags and the `filterOutOfStockItems` flag. * Tag expressions. Restricts predictions to items that match all of the specified tags. Boolean operators `OR` and `NOT` are supported if the expression is enclosed in parentheses, and must be separated from the tag values by a space. `-"tagA"` is also supported and is equivalent to `NOT "tagA"`. Tag values must be double quoted UTF-8 encoded strings with a size limit of 1 KiB. * filterOutOfStockItems. Restricts predictions to items that do not have a stockState value of OUT_OF_STOCK. Examples: * tag=("Red" OR "Blue") tag="New-Arrival" tag=(NOT "promotional") * filterOutOfStockItems tag=(-"promotional") * filterOutOfStockItems If your filter blocks all prediction results, nothing will be returned. If you want generic (unfiltered) popular items to be returned instead, set `strictFiltering` to false in `PredictRequest.params`. |
| `page_size` | i64 |  | Optional. Maximum number of results to return per page. Set this property to the number of prediction results required. If zero, the service will choose a reasonable default. |
| `params` | HashMap<String, String> |  | Optional. Additional domain specific parameters for the predictions. Allowed values: * `returnCatalogItem`: Boolean. If set to true, the associated catalogItem object will be returned in the `PredictResponse.PredictionResult.itemMetadata` object in the method response. * `returnItemScore`: Boolean. If set to true, the prediction 'score' corresponding to each returned item will be set in the `metadata` field in the prediction response. The given 'score' indicates the probability of an item being clicked/purchased given the user's context and history. * `strictFiltering`: Boolean. True by default. If set to false, the service will return generic (unfiltered) popular items instead of empty if your filter blocks all prediction results. * `priceRerankLevel`: String. Default empty. If set to be non-empty, then it needs to be one of {'no-price-reranking', 'low-price-reranking', 'medium-price-reranking', 'high-price-reranking'}. This gives request level control and adjust prediction results based on product price. * `diversityLevel`: String. Default empty. If set to be non-empty, then it needs to be one of {'no-diversity', 'low-diversity', 'medium-diversity', 'high-diversity', 'auto-diversity'}. This gives request level control and adjust prediction results based on product category. |
| `dry_run` | bool |  | Optional. Use dryRun mode for this prediction query. If set to true, a fake model will be used that returns arbitrary catalog items. Note that the dryRun mode should only be used for testing the API, or if the model is not ready. |
| `page_token` | String |  | Optional. The previous PredictResponse.next_page_token. |
| `name` | String | ✅ |  |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create placement
placement = provider.recommendationengine_api.Placement {
    name = "value"  # Required field
}

```

---


### Prediction_api_key_registration

Register an API key for use with predict method.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prediction_api_key_registration` | String |  | Required. The prediction API key registration. |
| `parent` | String | ✅ | Required. The parent resource path. `projects/*/locations/global/catalogs/default_catalog/eventStores/default_event_store`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If empty, the list is complete. If nonempty, pass the token to the next request's `ListPredictionApiKeysRegistrationsRequest.pageToken`. |
| `prediction_api_key_registrations` | Vec<String> | The list of registered API keys. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create prediction_api_key_registration
prediction_api_key_registration = provider.recommendationengine_api.Prediction_api_key_registration {
    parent = "value"  # Required. The parent resource path. `projects/*/locations/global/catalogs/default_catalog/eventStores/default_event_store`.
}

# Access prediction_api_key_registration outputs
prediction_api_key_registration_id = prediction_api_key_registration.id
prediction_api_key_registration_next_page_token = prediction_api_key_registration.next_page_token
prediction_api_key_registration_prediction_api_key_registrations = prediction_api_key_registration.prediction_api_key_registrations
```

---


### Catalog

Lists all the catalog configurations associated with the project.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The fully qualified resource name of the catalog. |
| `display_name` | String |  | Required. The catalog display name. |
| `catalog_item_level_config` | String |  | Required. The catalog item level configuration. |
| `default_event_store_id` | String |  | Required. The ID of the default event store. |
| `name` | String | ✅ | The fully qualified resource name of the catalog. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `catalogs` | Vec<String> | Output only. All the customer's catalogs. |
| `next_page_token` | String | Pagination token, if not returned indicates the last page. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access catalog outputs
catalog_id = catalog.id
catalog_catalogs = catalog.catalogs
catalog_next_page_token = catalog.next_page_token
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Catalog_item

Creates a catalog item.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `item_group_id` | String |  | Optional. Variant group identifier for prediction results. UTF-8 encoded string with a length limit of 128 bytes. This field must be enabled before it can be used. [Learn more](/recommendations-ai/docs/catalog#item-group-id). |
| `product_metadata` | String |  | Optional. Metadata specific to retail products. |
| `title` | String |  | Required. Catalog item title. UTF-8 encoded string with a length limit of 1 KiB. |
| `item_attributes` | String |  | Optional. Highly encouraged. Extra catalog item attributes to be included in the recommendation model. For example, for retail products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the item attributes here. |
| `category_hierarchies` | Vec<String> |  | Required. Catalog item categories. This field is repeated for supporting one catalog item belonging to several parallel category hierarchies. For example, if a shoes product belongs to both ["Shoes & Accessories" -> "Shoes"] and ["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"], it could be represented as: "categoryHierarchies": [ { "categories": ["Shoes & Accessories", "Shoes"]}, { "categories": ["Sports & Fitness", "Athletic Clothing", "Shoes"] } ] |
| `description` | String |  | Optional. Catalog item description. UTF-8 encoded string with a length limit of 5 KiB. |
| `language_code` | String |  | Optional. Deprecated. The model automatically detects the text language. Your catalog can include text in different languages, but duplicating catalog items to provide text in multiple languages can result in degraded model performance. |
| `id` | String |  | Required. Catalog item identifier. UTF-8 encoded string with a length limit of 128 bytes. This id must be unique among all catalog items within the same catalog. It should also be used when logging user events in order for the user events to be joined with the Catalog. |
| `tags` | Vec<String> |  | Optional. Filtering tags associated with the catalog item. Each tag should be a UTF-8 encoded string with a length limit of 1 KiB. This tag can be used for filtering recommendation results by passing the tag as part of the predict request filter. |
| `parent` | String | ✅ | Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `item_group_id` | String | Optional. Variant group identifier for prediction results. UTF-8 encoded string with a length limit of 128 bytes. This field must be enabled before it can be used. [Learn more](/recommendations-ai/docs/catalog#item-group-id). |
| `product_metadata` | String | Optional. Metadata specific to retail products. |
| `title` | String | Required. Catalog item title. UTF-8 encoded string with a length limit of 1 KiB. |
| `item_attributes` | String | Optional. Highly encouraged. Extra catalog item attributes to be included in the recommendation model. For example, for retail products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the item attributes here. |
| `category_hierarchies` | Vec<String> | Required. Catalog item categories. This field is repeated for supporting one catalog item belonging to several parallel category hierarchies. For example, if a shoes product belongs to both ["Shoes & Accessories" -> "Shoes"] and ["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"], it could be represented as: "categoryHierarchies": [ { "categories": ["Shoes & Accessories", "Shoes"]}, { "categories": ["Sports & Fitness", "Athletic Clothing", "Shoes"] } ] |
| `description` | String | Optional. Catalog item description. UTF-8 encoded string with a length limit of 5 KiB. |
| `language_code` | String | Optional. Deprecated. The model automatically detects the text language. Your catalog can include text in different languages, but duplicating catalog items to provide text in multiple languages can result in degraded model performance. |
| `id` | String | Required. Catalog item identifier. UTF-8 encoded string with a length limit of 128 bytes. This id must be unique among all catalog items within the same catalog. It should also be used when logging user events in order for the user events to be joined with the Catalog. |
| `tags` | Vec<String> | Optional. Filtering tags associated with the catalog item. Each tag should be a UTF-8 encoded string with a length limit of 1 KiB. This tag can be used for filtering recommendation results by passing the tag as part of the predict request filter. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create catalog_item
catalog_item = provider.recommendationengine_api.Catalog_item {
    parent = "value"  # Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog`.
}

# Access catalog_item outputs
catalog_item_id = catalog_item.id
catalog_item_item_group_id = catalog_item.item_group_id
catalog_item_product_metadata = catalog_item.product_metadata
catalog_item_title = catalog_item.title
catalog_item_item_attributes = catalog_item.item_attributes
catalog_item_category_hierarchies = catalog_item.category_hierarchies
catalog_item_description = catalog_item.description
catalog_item_language_code = catalog_item.language_code
catalog_item_id = catalog_item.id
catalog_item_tags = catalog_item.tags
```

---


### User_event

Writes a single user event.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_info` | String |  | Required. User information. |
| `event_source` | String |  | Optional. This field should *not* be set when using JavaScript pixel or the Recommendations AI Tag. Defaults to `EVENT_SOURCE_UNSPECIFIED`. |
| `event_type` | String |  | Required. User event type. Allowed values are: * `add-to-cart` Products being added to cart. * `add-to-list` Items being added to a list (shopping list, favorites etc). * `category-page-view` Special pages such as sale or promotion pages viewed. * `checkout-start` User starting a checkout process. * `detail-page-view` Products detail page viewed. * `home-page-view` Homepage viewed. * `page-visit` Generic page visits not included in the event types above. * `purchase-complete` User finishing a purchase. * `refund` Purchased items being refunded or returned. * `remove-from-cart` Products being removed from cart. * `remove-from-list` Items being removed from a list. * `search` Product search. * `shopping-cart-page-view` User viewing a shopping cart. * `impression` List of items displayed. Used by Google Tag Manager. |
| `event_time` | String |  | Optional. Only required for ImportUserEvents method. Timestamp of user event created. |
| `product_event_detail` | String |  | Optional. Retail product specific user event metadata. This field is required for the following event types: * `add-to-cart` * `add-to-list` * `category-page-view` * `checkout-start` * `detail-page-view` * `purchase-complete` * `refund` * `remove-from-cart` * `remove-from-list` * `search` This field is optional for the following event types: * `page-visit` * `shopping-cart-page-view` - note that 'product_event_detail' should be set for this unless the shopping cart is empty. This field is not allowed for the following event types: * `home-page-view` |
| `event_detail` | String |  | Optional. User event detailed information common across different recommendation types. |
| `parent` | String | ✅ | Required. The parent eventStore resource name, such as "projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_events` | Vec<String> | The user events. |
| `next_page_token` | String | If empty, the list is complete. If nonempty, the token to pass to the next request's ListUserEvents.page_token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_event
user_event = provider.recommendationengine_api.User_event {
    parent = "value"  # Required. The parent eventStore resource name, such as "projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store".
}

# Access user_event outputs
user_event_id = user_event.id
user_event_user_events = user_event.user_events
user_event_next_page_token = user_event.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple placement resources
placement_0 = provider.recommendationengine_api.Placement {
    name = "value-0"
}
placement_1 = provider.recommendationengine_api.Placement {
    name = "value-1"
}
placement_2 = provider.recommendationengine_api.Placement {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    placement = provider.recommendationengine_api.Placement {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Recommendationengine_api Documentation](https://cloud.google.com/recommendationengine_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
