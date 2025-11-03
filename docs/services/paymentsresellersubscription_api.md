# Paymentsresellersubscription_api Service



**Resources**: 5

---

## Overview

The paymentsresellersubscription_api service provides access to 5 resource types:

- [Subscription](#subscription) [CR]
- [Line_item](#line_item) [U]
- [Promotion](#promotion) [CR]
- [Product](#product) [R]
- [User_session](#user_session) [C]

---

## Resources


### Subscription

Used by partners to create a subscription for their customers. The created subscription is associated with the end user inferred from the end user credentials. This API must be authorized by the end user using OAuth.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. System generated timestamp when the subscription is created. UTC timezone. |
| `cycle_end_time` | String |  | Output only. The time at which the subscription is expected to be extended, in ISO 8061 format. UTC timezone. For example: "2019-08-31T17:28:54.564Z" |
| `state` | String |  | Output only. Describes the state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle). |
| `partner_user_token` | String |  | Required. Identifier of the end-user in partner’s system. The value is restricted to 63 ASCII characters at the maximum. |
| `products` | Vec<String> |  | Optional. Deprecated: consider using `line_items` as the input. Required. Resource name that identifies the purchased products. The format will be 'partners/{partner_id}/products/{product_id}'. |
| `promotions` | Vec<String> |  | Optional. Deprecated: consider using the top-level `promotion_specs` as the input. Optional. Resource name that identifies one or more promotions that can be applied on the product. A typical promotion for a subscription is Free trial. The format will be 'partners/{partner_id}/promotions/{promotion_id}'. |
| `redirect_uri` | String |  | Output only. The place where partners should redirect the end-user to after creation. This field might also be populated when creation failed. However, Partners should always prepare a default URL to redirect the user in case this field is empty. |
| `upgrade_downgrade_details` | String |  | Optional. Details about the previous subscription that this new subscription upgrades/downgrades from. Only populated if this subscription is an upgrade/downgrade from another subscription. |
| `line_items` | Vec<String> |  | Required. The line items of the subscription. |
| `end_user_entitled` | bool |  | Output only. Indicates if the subscription is entitled to the end user. |
| `cancellation_details` | String |  | Output only. Describes the details of a cancelled subscription. Only applicable to subscription of state `STATE_CANCELLED`. |
| `migration_details` | String |  | Output only. Describes the details of the migrated subscription. Only populated if this subscription is migrated from another system. |
| `service_location` | String |  | Required. The location that the service is provided as indicated by the partner. |
| `update_time` | String |  | Output only. System generated timestamp when the subscription is most recently updated. UTC timezone. |
| `promotion_specs` | Vec<String> |  | Optional. Subscription-level promotions. Only free trial is supported on this level. It determines the first renewal time of the subscription to be the end of the free trial period. Specify the promotion resource name only when used as input. |
| `free_trial_end_time` | String |  | Output only. End of the free trial period, in ISO 8061 format. For example, "2019-08-31T17:28:54.564Z". It will be set the same as createTime if no free trial promotion is specified. |
| `name` | String |  | Identifier. Resource name of the subscription. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}". This is available for authorizeAddon, but otherwise is response only. |
| `processing_state` | String |  | Output only. Describes the processing state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle). |
| `purchase_time` | String |  | Optional. The timestamp when the user transaction was made with the Partner. Specify for the case of "bundle with choice", and it must be before the provision_time (when the user makes a selection). |
| `renewal_time` | String |  | Output only. The time at which the subscription is expected to be renewed by Google - a new charge will be incurred and the service entitlement will be renewed. A non-immediate cancellation will take place at this time too, before which, the service entitlement for the end user will remain valid. UTC timezone in ISO 8061 format. For example: "2019-08-31T17:28:54.564Z" |
| `parent` | String | ✅ | Required. The parent resource name, which is the identifier of the partner. It will have the format of "partners/{partner_id}". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. System generated timestamp when the subscription is created. UTC timezone. |
| `cycle_end_time` | String | Output only. The time at which the subscription is expected to be extended, in ISO 8061 format. UTC timezone. For example: "2019-08-31T17:28:54.564Z" |
| `state` | String | Output only. Describes the state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle). |
| `partner_user_token` | String | Required. Identifier of the end-user in partner’s system. The value is restricted to 63 ASCII characters at the maximum. |
| `products` | Vec<String> | Optional. Deprecated: consider using `line_items` as the input. Required. Resource name that identifies the purchased products. The format will be 'partners/{partner_id}/products/{product_id}'. |
| `promotions` | Vec<String> | Optional. Deprecated: consider using the top-level `promotion_specs` as the input. Optional. Resource name that identifies one or more promotions that can be applied on the product. A typical promotion for a subscription is Free trial. The format will be 'partners/{partner_id}/promotions/{promotion_id}'. |
| `redirect_uri` | String | Output only. The place where partners should redirect the end-user to after creation. This field might also be populated when creation failed. However, Partners should always prepare a default URL to redirect the user in case this field is empty. |
| `upgrade_downgrade_details` | String | Optional. Details about the previous subscription that this new subscription upgrades/downgrades from. Only populated if this subscription is an upgrade/downgrade from another subscription. |
| `line_items` | Vec<String> | Required. The line items of the subscription. |
| `end_user_entitled` | bool | Output only. Indicates if the subscription is entitled to the end user. |
| `cancellation_details` | String | Output only. Describes the details of a cancelled subscription. Only applicable to subscription of state `STATE_CANCELLED`. |
| `migration_details` | String | Output only. Describes the details of the migrated subscription. Only populated if this subscription is migrated from another system. |
| `service_location` | String | Required. The location that the service is provided as indicated by the partner. |
| `update_time` | String | Output only. System generated timestamp when the subscription is most recently updated. UTC timezone. |
| `promotion_specs` | Vec<String> | Optional. Subscription-level promotions. Only free trial is supported on this level. It determines the first renewal time of the subscription to be the end of the free trial period. Specify the promotion resource name only when used as input. |
| `free_trial_end_time` | String | Output only. End of the free trial period, in ISO 8061 format. For example, "2019-08-31T17:28:54.564Z". It will be set the same as createTime if no free trial promotion is specified. |
| `name` | String | Identifier. Resource name of the subscription. It will have the format of "partners/{partner_id}/subscriptions/{subscription_id}". This is available for authorizeAddon, but otherwise is response only. |
| `processing_state` | String | Output only. Describes the processing state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle). |
| `purchase_time` | String | Optional. The timestamp when the user transaction was made with the Partner. Specify for the case of "bundle with choice", and it must be before the provision_time (when the user makes a selection). |
| `renewal_time` | String | Output only. The time at which the subscription is expected to be renewed by Google - a new charge will be incurred and the service entitlement will be renewed. A non-immediate cancellation will take place at this time too, before which, the service entitlement for the end user will remain valid. UTC timezone in ISO 8061 format. For example: "2019-08-31T17:28:54.564Z" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subscription
subscription = provider.paymentsresellersubscription_api.Subscription {
    parent = "value"  # Required. The parent resource name, which is the identifier of the partner. It will have the format of "partners/{partner_id}".
}

# Access subscription outputs
subscription_id = subscription.id
subscription_create_time = subscription.create_time
subscription_cycle_end_time = subscription.cycle_end_time
subscription_state = subscription.state
subscription_partner_user_token = subscription.partner_user_token
subscription_products = subscription.products
subscription_promotions = subscription.promotions
subscription_redirect_uri = subscription.redirect_uri
subscription_upgrade_downgrade_details = subscription.upgrade_downgrade_details
subscription_line_items = subscription.line_items
subscription_end_user_entitled = subscription.end_user_entitled
subscription_cancellation_details = subscription.cancellation_details
subscription_migration_details = subscription.migration_details
subscription_service_location = subscription.service_location
subscription_update_time = subscription.update_time
subscription_promotion_specs = subscription.promotion_specs
subscription_free_trial_end_time = subscription.free_trial_end_time
subscription_name = subscription.name
subscription_processing_state = subscription.processing_state
subscription_purchase_time = subscription.purchase_time
subscription_renewal_time = subscription.renewal_time
```

---


### Line_item



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `line_item_index` | i64 |  | Output only. A unique index of the subscription line item. |
| `line_item_promotion_specs` | Vec<String> |  | Optional. The promotions applied on the line item. It can be: - an introductory pricing promotion. - a free trial promotion. This feature is not enabled. If used, the request will be rejected. When used as input in Create or Provision API, specify its resource name only. |
| `one_time_recurrence_details` | String |  | Output only. Details only set for a ONE_TIME recurrence line item. |
| `recurrence_type` | String |  | Output only. The recurrence type of the line item. |
| `description` | String |  | Output only. Description of this line item. |
| `amount` | String |  | Output only. The price of the product/service in this line item. The amount could be the wholesale price, or it can include a cost of sale based on the contract. |
| `line_item_free_trial_end_time` | String |  | Output only. The free trial end time will be populated after the line item is successfully processed. End time of the line item free trial period, in ISO 8061 format. For example, "2019-08-31T17:28:54.564Z". It will be set the same as createTime if no free trial promotion is specified. |
| `product` | String |  | Required. Product resource name that identifies one the line item The format is 'partners/{partner_id}/products/{product_id}'. |
| `product_payload` | String |  | Optional. Product specific payload for this line item. |
| `bundle_details` | String |  | Output only. The bundle details for the line item. Only populated if the line item corresponds to a hard bundle. |
| `finite_billing_cycle_details` | String |  | Optional. Details for a subscription line item with finite billing cycles. If unset, the line item will be charged indefinitely. Used only with LINE_ITEM_RECURRENCE_TYPE_PERIODIC. |
| `name` | String |  | Identifier. Resource name of the line item. Format: partners/{partner}/subscriptions/{subscription}/lineItems/{lineItem} |
| `state` | String |  | Output only. The state of the line item. |
| `name` | String | ✅ | Identifier. Resource name of the line item. Format: partners/{partner}/subscriptions/{subscription}/lineItems/{lineItem} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

```

---


### Promotion

Currently, it is only enabeld for **YouTube**. Finds eligible promotions for the current user. The API requires user authorization via OAuth. The bare minimum oauth scope `openid` is sufficient, which will skip the consent screen.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_token` | String |  | Optional. A page token, received from a previous `ListPromotions` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListPromotions` must match the call that provided the page token. |
| `filter` | String |  | Optional. Specifies the filters for the promotion results. The syntax is defined in https://google.aip.dev/160 with the following caveats: 1. Only the following features are supported: - Logical operator `AND` - Comparison operator `=` (no wildcards `*`) - Traversal operator `.` - Has operator `:` (no wildcards `*`) 2. Only the following fields are supported: - `applicableProducts` - `regionCodes` - `youtubePayload.partnerEligibilityId` - `youtubePayload.postalCode` 3. Unless explicitly mentioned above, other features are not supported. Example: `applicableProducts:partners/partner1/products/product1 AND regionCodes:US AND youtubePayload.postalCode=94043 AND youtubePayload.partnerEligibilityId=eligibility-id` |
| `page_size` | i64 |  | Optional. The maximum number of promotions to return. The service may return fewer than this value. If unspecified, at most 50 products will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000. |
| `parent` | String | ✅ | Required. The parent, the partner that can resell. Format: partners/{partner} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages. |
| `promotions` | Vec<String> | The promotions for the specified partner. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create promotion
promotion = provider.paymentsresellersubscription_api.Promotion {
    parent = "value"  # Required. The parent, the partner that can resell. Format: partners/{partner}
}

# Access promotion outputs
promotion_id = promotion.id
promotion_next_page_token = promotion.next_page_token
promotion_promotions = promotion.promotions
```

---


### Product

Currently, it doesn't support **YouTube** products. Retrieves the products that can be resold by the partner. It should be autenticated with a service account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `products` | Vec<String> | The products for the specified partner. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages. |


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
product_products = product.products
product_next_page_token = product.next_page_token
```

---


### User_session

This API replaces user authorized OAuth consent based APIs (Create, Entitle). Issues a timed session token for the given user intent. You can use the session token to redirect the user to Google to finish the signup flow. You can re-generate new session token repeatedly for the same request if necessary, regardless of the previous tokens being expired or not. By default, the session token is valid for 1 hour.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `intent_payload` | String |  | The user intent to generate the user session. |
| `parent` | String | ✅ | Required. The parent, the partner that can resell. Format: partners/{partner} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_session
user_session = provider.paymentsresellersubscription_api.User_session {
    parent = "value"  # Required. The parent, the partner that can resell. Format: partners/{partner}
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

# Create multiple subscription resources
subscription_0 = provider.paymentsresellersubscription_api.Subscription {
    parent = "value-0"
}
subscription_1 = provider.paymentsresellersubscription_api.Subscription {
    parent = "value-1"
}
subscription_2 = provider.paymentsresellersubscription_api.Subscription {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    subscription = provider.paymentsresellersubscription_api.Subscription {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Paymentsresellersubscription_api Documentation](https://cloud.google.com/paymentsresellersubscription_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
