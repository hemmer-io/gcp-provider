# Androidpublisher_api Service



**Resources**: 36

---

## Overview

The androidpublisher_api service provides access to 36 resource types:

- [Inapppurchase](#inapppurchase) [R]
- [Product](#product) [R]
- [Voidedpurchase](#voidedpurchase) [R]
- [Grant](#grant) [CUD]
- [Track](#track) [CRU]
- [Application](#application) [C]
- [Variant](#variant) [CR]
- [Internalappsharingartifact](#internalappsharingartifact) [C]
- [Product](#product) [CR]
- [Bundle](#bundle) [CR]
- [Expansionfile](#expansionfile) [CRU]
- [Subscriptionsv2](#subscriptionsv2) [CR]
- [Inappproduct](#inappproduct) [CRUD]
- [Detail](#detail) [RU]
- [User](#user) [CRUD]
- [Listing](#listing) [RUD]
- [Externaltransaction](#externaltransaction) [CR]
- [Monetization](#monetization) [C]
- [Base_plan](#base_plan) [CD]
- [Voidedpurchase](#voidedpurchase) [R]
- [Device_tier_config](#device_tier_config) [CR]
- [Productsv2](#productsv2) [R]
- [Countryavailability](#countryavailability) [R]
- [Subscription](#subscription) [CRUD]
- [Order](#order) [CR]
- [Image](#image) [CRD]
- [Review](#review) [CR]
- [Tester](#tester) [RU]
- [Offer](#offer) [CRUD]
- [Generatedapk](#generatedapk) [R]
- [Apprecovery](#apprecovery) [CR]
- [Purchase_option](#purchase_option) [C]
- [Deobfuscationfile](#deobfuscationfile) [C]
- [Onetimeproduct](#onetimeproduct) [CRUD]
- [Apk](#apk) [CR]
- [Edit](#edit) [CRD]

---

## Resources


### Inapppurchase

Checks the purchase and consumption status of an inapp item.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `purchase_time` | String | The time the product was purchased, in milliseconds since the epoch (Jan 1, 1970). |
| `purchase_type` | i64 | The type of purchase of the inapp product. This field is only set if this purchase was not made using the standard in-app billing flow. Possible values are:  
- Test (i.e. purchased from a license testing account) 
- Promo (i.e. purchased using a promo code) 
- Rewarded (i.e. from watching a video ad instead of paying) |
| `kind` | String | This kind represents an inappPurchase object in the androidpublisher service. |
| `consumption_state` | i64 | The consumption state of the inapp product. Possible values are:  
- Yet to be consumed 
- Consumed |
| `developer_payload` | String | A developer-specified string that contains supplemental information about an order. |
| `order_id` | String | The order id associated with the purchase of the inapp product. |
| `purchase_state` | i64 | The purchase state of the order. Possible values are:  
- Purchased 
- Canceled 
- Pending |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access inapppurchase outputs
inapppurchase_id = inapppurchase.id
inapppurchase_purchase_time = inapppurchase.purchase_time
inapppurchase_purchase_type = inapppurchase.purchase_type
inapppurchase_kind = inapppurchase.kind
inapppurchase_consumption_state = inapppurchase.consumption_state
inapppurchase_developer_payload = inapppurchase.developer_payload
inapppurchase_order_id = inapppurchase.order_id
inapppurchase_purchase_state = inapppurchase.purchase_state
```

---


### Product

Checks the purchase and consumption status of an inapp item.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `order_id` | String | The order id associated with the purchase of the inapp product. |
| `consumption_state` | i64 | The consumption state of the inapp product. Possible values are:  
- Yet to be consumed 
- Consumed |
| `purchase_state` | i64 | The purchase state of the order. Possible values are:  
- Purchased 
- Canceled 
- Pending |
| `purchase_time_millis` | String | The time the product was purchased, in milliseconds since the epoch (Jan 1, 1970). |
| `purchase_type` | i64 | The type of purchase of the inapp product. This field is only set if this purchase was not made using the standard in-app billing flow. Possible values are:  
- Test (i.e. purchased from a license testing account) 
- Promo (i.e. purchased using a promo code) 
- Rewarded (i.e. from watching a video ad instead of paying) |
| `kind` | String | This kind represents an inappPurchase object in the androidpublisher service. |
| `developer_payload` | String | A developer-specified string that contains supplemental information about an order. |


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
product_order_id = product.order_id
product_consumption_state = product.consumption_state
product_purchase_state = product.purchase_state
product_purchase_time_millis = product.purchase_time_millis
product_purchase_type = product.purchase_type
product_kind = product.kind
product_developer_payload = product.developer_payload
```

---


### Voidedpurchase

Lists the purchases that were canceled, refunded or charged-back.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `voided_purchases` | Vec<String> |  |
| `page_info` | String |  |
| `token_pagination` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access voidedpurchase outputs
voidedpurchase_id = voidedpurchase.id
voidedpurchase_voided_purchases = voidedpurchase.voided_purchases
voidedpurchase_page_info = voidedpurchase.page_info
voidedpurchase_token_pagination = voidedpurchase.token_pagination
```

---


### Grant

Grant access for a user to the given package.

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_level_permissions` | Vec<String> |  | The permissions granted to the user for this app. |
| `package_name` | String |  | Immutable. The package name of the app. This will be empty for draft apps. |
| `name` | String |  | Required. Resource name for this grant, following the pattern "developers/{developer}/users/{email}/grants/{package_name}". If this grant is for a draft app, the app ID will be used in this resource name instead of the package name. |
| `parent` | String | ✅ | Required. The user which needs permission. Format: developers/{developer}/users/{user} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create grant
grant = provider.androidpublisher_api.Grant {
    parent = "value"  # Required. The user which needs permission. Format: developers/{developer}/users/{user}
}

```

---


### Track

Creates a new track.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `track` | String |  | Required. Identifier of the new track. For default tracks, this field consists of the track alias only. Form factor tracks have a special prefix as an identifier, for example `wear:production`, `automotive:production`. This prefix must match the value of the `form_factor` field, if it is not a default track. [More on track name](https://developers.google.com/android-publisher/tracks#ff-track-name) |
| `type` | String |  | Required. Type of the new track. Currently, the only supported value is closedTesting. |
| `form_factor` | String |  | Required. Form factor of the new track. Defaults to the default track. |
| `edit_id` | String | ✅ | Required. Identifier of the edit. |
| `package_name` | String | ✅ | Required. Package name of the app. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `releases` | Vec<String> | In a read request, represents all active releases in the track. In an update request, represents desired changes. |
| `track` | String | Identifier of the track. Form factor tracks have a special prefix as an identifier, for example `wear:production`, `automotive:production`. [More on track name](https://developers.google.com/android-publisher/tracks#ff-track-name) |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create track
track = provider.androidpublisher_api.Track {
    edit_id = "value"  # Required. Identifier of the edit.
    package_name = "value"  # Required. Package name of the app.
}

# Access track outputs
track_id = track.id
track_releases = track.releases
track_track = track.track
```

---


### Application

Writes the Safety Labels declaration of an app.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `safety_labels` | String |  | Required. Contents of the CSV file containing Data Safety responses. For the format of this file, see the Help Center documentation at https://support.google.com/googleplay/android-developer/answer/10787469?#zippy=%2Cunderstand-the-csv-format To download an up to date template, follow the steps at https://support.google.com/googleplay/android-developer/answer/10787469?#zippy=%2Cexport-to-a-csv-file |
| `package_name` | String | ✅ | Required. Package name of the app. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create application
application = provider.androidpublisher_api.Application {
    package_name = "value"  # Required. Package name of the app.
}

```

---


### Variant

Creates an APK which is suitable for inclusion in a system image from an already uploaded Android App Bundle.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `options` | String |  | Optional. Options applied to the generated APK. |
| `variant_id` | i64 |  | Output only. The ID of a previously created system APK variant. |
| `device_spec` | String |  | The device spec used to generate the APK. |
| `version_code` | String | ✅ | The version code of the App Bundle. |
| `package_name` | String | ✅ | Package name of the app. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `options` | String | Optional. Options applied to the generated APK. |
| `variant_id` | i64 | Output only. The ID of a previously created system APK variant. |
| `device_spec` | String | The device spec used to generate the APK. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create variant
variant = provider.androidpublisher_api.Variant {
    version_code = "value"  # The version code of the App Bundle.
    package_name = "value"  # Package name of the app.
}

# Access variant outputs
variant_id = variant.id
variant_options = variant.options
variant_variant_id = variant.variant_id
variant_device_spec = variant.device_spec
```

---


### Internalappsharingartifact

Uploads an APK to internal app sharing. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See [Timeouts and Errors](https://developers.google.com/api-client-library/java/google-api-java-client/errors) for an example in java.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `package_name` | String | ✅ | Package name of the app. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create internalappsharingartifact
internalappsharingartifact = provider.androidpublisher_api.Internalappsharingartifact {
    package_name = "value"  # Package name of the app.
}

```

---


### Product

Consumes a purchase for an inapp item.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `token` | String | ✅ | The token provided to the user's device when the inapp product was purchased. |
| `package_name` | String | ✅ | The package name of the application the inapp product was sold in (for example, 'com.some.thing'). |
| `product_id` | String | ✅ | The inapp product SKU (for example, 'com.some.thing.inapp1'). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `consumption_state` | i64 | The consumption state of the inapp product. Possible values are: 0. Yet to be consumed 1. Consumed |
| `quantity` | i64 | The quantity associated with the purchase of the inapp product. If not present, the quantity is 1. |
| `region_code` | String | ISO 3166-1 alpha-2 billing region code of the user at the time the product was granted. |
| `purchase_time_millis` | String | The time the product was purchased, in milliseconds since the epoch (Jan 1, 1970). |
| `refundable_quantity` | i64 | The quantity eligible for refund, i.e. quantity that hasn't been refunded. The value reflects quantity-based partial refunds and full refunds. |
| `product_id` | String | The inapp product SKU. May not be present. |
| `purchase_type` | i64 | The type of purchase of the inapp product. This field is only set if this purchase was not made using the standard in-app billing flow. Possible values are: 0. Test (i.e. purchased from a license testing account) 1. Promo (i.e. purchased using a promo code). Does not include Play Points purchases. 2. Rewarded (i.e. from watching a video ad instead of paying) |
| `obfuscated_external_account_id` | String | An obfuscated version of the id that is uniquely associated with the user's account in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedaccountid when the purchase was made. |
| `acknowledgement_state` | i64 | The acknowledgement state of the inapp product. Possible values are: 0. Yet to be acknowledged 1. Acknowledged |
| `developer_payload` | String | A developer-specified string that contains supplemental information about an order. |
| `purchase_token` | String | The purchase token generated to identify this purchase. May not be present. |
| `obfuscated_external_profile_id` | String | An obfuscated version of the id that is uniquely associated with the user's profile in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedprofileid when the purchase was made. |
| `order_id` | String | The order id associated with the purchase of the inapp product. |
| `purchase_state` | i64 | The purchase state of the order. Possible values are: 0. Purchased 1. Canceled 2. Pending |
| `kind` | String | This kind represents an inappPurchase object in the androidpublisher service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product
product = provider.androidpublisher_api.Product {
    token = "value"  # The token provided to the user's device when the inapp product was purchased.
    package_name = "value"  # The package name of the application the inapp product was sold in (for example, 'com.some.thing').
    product_id = "value"  # The inapp product SKU (for example, 'com.some.thing.inapp1').
}

# Access product outputs
product_id = product.id
product_consumption_state = product.consumption_state
product_quantity = product.quantity
product_region_code = product.region_code
product_purchase_time_millis = product.purchase_time_millis
product_refundable_quantity = product.refundable_quantity
product_product_id = product.product_id
product_purchase_type = product.purchase_type
product_obfuscated_external_account_id = product.obfuscated_external_account_id
product_acknowledgement_state = product.acknowledgement_state
product_developer_payload = product.developer_payload
product_purchase_token = product.purchase_token
product_obfuscated_external_profile_id = product.obfuscated_external_profile_id
product_order_id = product.order_id
product_purchase_state = product.purchase_state
product_kind = product.kind
```

---


### Bundle

Uploads a new Android App Bundle to this edit. If you are using the Google API client libraries, please increase the timeout of the http request before calling this endpoint (a timeout of 2 minutes is recommended). See [Timeouts and Errors](https://developers.google.com/api-client-library/java/google-api-java-client/errors) for an example in java.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `edit_id` | String | ✅ | Identifier of the edit. |
| `package_name` | String | ✅ | Package name of the app. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | The kind of this response ("androidpublisher#bundlesListResponse"). |
| `bundles` | Vec<String> | All app bundles. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bundle
bundle = provider.androidpublisher_api.Bundle {
    edit_id = "value"  # Identifier of the edit.
    package_name = "value"  # Package name of the app.
}

# Access bundle outputs
bundle_id = bundle.id
bundle_kind = bundle.kind
bundle_bundles = bundle.bundles
```

---


### Expansionfile

Uploads a new expansion file and attaches to the specified APK.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `edit_id` | String | ✅ | Identifier of the edit. |
| `expansion_file_type` | String | ✅ | The file type of the expansion file configuration which is being updated. |
| `apk_version_code` | i64 | ✅ | The version code of the APK whose expansion file configuration is being read or modified. |
| `package_name` | String | ✅ | Package name of the app. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `file_size` | String | If set, this field indicates that this APK has an expansion file uploaded to it: this APK does not reference another APK's expansion file. The field's value is the size of the uploaded expansion file in bytes. |
| `references_version` | i64 | If set, this APK's expansion file references another APK's expansion file. The file_size field will not be set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create expansionfile
expansionfile = provider.androidpublisher_api.Expansionfile {
    edit_id = "value"  # Identifier of the edit.
    expansion_file_type = "value"  # The file type of the expansion file configuration which is being updated.
    apk_version_code = "value"  # The version code of the APK whose expansion file configuration is being read or modified.
    package_name = "value"  # Package name of the app.
}

# Access expansionfile outputs
expansionfile_id = expansionfile.id
expansionfile_file_size = expansionfile.file_size
expansionfile_references_version = expansionfile.references_version
```

---


### Subscriptionsv2

Cancel a subscription purchase for the user.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cancellation_context` | String |  | Required. Additional details around the subscription revocation. |
| `token` | String | ✅ | Required. The token provided to the user's device when the subscription was purchased. |
| `package_name` | String | ✅ | Required. The package of the application for which this subscription was purchased (for example, 'com.some.thing'). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `canceled_state_context` | String | Additional context around canceled subscriptions. Only present if the subscription currently has subscription_state SUBSCRIPTION_STATE_CANCELED or SUBSCRIPTION_STATE_EXPIRED. |
| `paused_state_context` | String | Additional context around paused subscriptions. Only present if the subscription currently has subscription_state SUBSCRIPTION_STATE_PAUSED. |
| `acknowledgement_state` | String | The acknowledgement state of the subscription. |
| `start_time` | String | Time at which the subscription was granted. Not set for pending subscriptions (subscription was created but awaiting payment during signup). |
| `subscription_state` | String | The current state of the subscription. |
| `test_purchase` | String | Only present if this subscription purchase is a test purchase. |
| `external_account_identifiers` | String | User account identifier in the third-party service. |
| `line_items` | Vec<String> | Item-level info for a subscription purchase. The items in the same purchase should be either all with AutoRenewingPlan or all with PrepaidPlan. |
| `kind` | String | This kind represents a SubscriptionPurchaseV2 object in the androidpublisher service. |
| `region_code` | String | ISO 3166-1 alpha-2 billing country/region code of the user at the time the subscription was granted. |
| `subscribe_with_google_info` | String | User profile associated with purchases made with 'Subscribe with Google'. |
| `latest_order_id` | String | Deprecated: Use line_items.latest_successful_order_id instead. The order id of the latest order associated with the purchase of the subscription. For autoRenewing subscription, this is the order id of signup order if it is not renewed yet, or the last recurring order id (success, pending, or declined order). For prepaid subscription, this is the order id associated with the queried purchase token. |
| `linked_purchase_token` | String | The purchase token of the old subscription if this subscription is one of the following: * Re-signup of a canceled but non-lapsed subscription * Upgrade/downgrade from a previous subscription. * Convert from prepaid to auto renewing subscription. * Convert from an auto renewing subscription to prepaid. * Topup a prepaid subscription. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subscriptionsv2
subscriptionsv2 = provider.androidpublisher_api.Subscriptionsv2 {
    token = "value"  # Required. The token provided to the user's device when the subscription was purchased.
    package_name = "value"  # Required. The package of the application for which this subscription was purchased (for example, 'com.some.thing').
}

# Access subscriptionsv2 outputs
subscriptionsv2_id = subscriptionsv2.id
subscriptionsv2_canceled_state_context = subscriptionsv2.canceled_state_context
subscriptionsv2_paused_state_context = subscriptionsv2.paused_state_context
subscriptionsv2_acknowledgement_state = subscriptionsv2.acknowledgement_state
subscriptionsv2_start_time = subscriptionsv2.start_time
subscriptionsv2_subscription_state = subscriptionsv2.subscription_state
subscriptionsv2_test_purchase = subscriptionsv2.test_purchase
subscriptionsv2_external_account_identifiers = subscriptionsv2.external_account_identifiers
subscriptionsv2_line_items = subscriptionsv2.line_items
subscriptionsv2_kind = subscriptionsv2.kind
subscriptionsv2_region_code = subscriptionsv2.region_code
subscriptionsv2_subscribe_with_google_info = subscriptionsv2.subscribe_with_google_info
subscriptionsv2_latest_order_id = subscriptionsv2.latest_order_id
subscriptionsv2_linked_purchase_token = subscriptionsv2.linked_purchase_token
```

---


### Inappproduct

Creates an in-app product (a managed product or a subscription). This method should no longer be used to create subscriptions. See [this article](https://android-developers.googleblog.com/2023/06/changes-to-google-play-developer-api-june-2023.html) for more information.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prices` | HashMap<String, String> |  | Prices per buyer region. None of these can be zero, as in-app products are never free. Map key is region code, as defined by ISO 3166-2. |
| `subscription_taxes_and_compliance_settings` | String |  | Details about taxes and legal compliance. Only applicable to subscription products. |
| `purchase_type` | String |  | The type of the product, e.g. a recurring subscription. |
| `subscription_period` | String |  | Subscription period, specified in ISO 8601 format. Acceptable values are P1W (one week), P1M (one month), P3M (three months), P6M (six months), and P1Y (one year). |
| `sku` | String |  | Stock-keeping-unit (SKU) of the product, unique within an app. |
| `package_name` | String |  | Package name of the parent app. |
| `listings` | HashMap<String, String> |  | List of localized title and description data. Map key is the language of the localized data, as defined by BCP-47, e.g. "en-US". |
| `default_language` | String |  | Default language of the localized data, as defined by BCP-47. e.g. "en-US". |
| `trial_period` | String |  | Trial period, specified in ISO 8601 format. Acceptable values are anything between P7D (seven days) and P999D (999 days). |
| `status` | String |  | The status of the product, e.g. whether it's active. |
| `managed_product_taxes_and_compliance_settings` | String |  | Details about taxes and legal compliance. Only applicable to managed products. |
| `default_price` | String |  | Default price. Cannot be zero, as in-app products are never free. Always in the developer's Checkout merchant currency. |
| `grace_period` | String |  | Grace period of the subscription, specified in ISO 8601 format. Allows developers to give their subscribers a grace period when the payment for the new recurrence period is declined. Acceptable values are P0D (zero days), P3D (three days), P7D (seven days), P14D (14 days), and P30D (30 days). |
| `package_name` | String | ✅ | Package name of the app. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `prices` | HashMap<String, String> | Prices per buyer region. None of these can be zero, as in-app products are never free. Map key is region code, as defined by ISO 3166-2. |
| `subscription_taxes_and_compliance_settings` | String | Details about taxes and legal compliance. Only applicable to subscription products. |
| `purchase_type` | String | The type of the product, e.g. a recurring subscription. |
| `subscription_period` | String | Subscription period, specified in ISO 8601 format. Acceptable values are P1W (one week), P1M (one month), P3M (three months), P6M (six months), and P1Y (one year). |
| `sku` | String | Stock-keeping-unit (SKU) of the product, unique within an app. |
| `package_name` | String | Package name of the parent app. |
| `listings` | HashMap<String, String> | List of localized title and description data. Map key is the language of the localized data, as defined by BCP-47, e.g. "en-US". |
| `default_language` | String | Default language of the localized data, as defined by BCP-47. e.g. "en-US". |
| `trial_period` | String | Trial period, specified in ISO 8601 format. Acceptable values are anything between P7D (seven days) and P999D (999 days). |
| `status` | String | The status of the product, e.g. whether it's active. |
| `managed_product_taxes_and_compliance_settings` | String | Details about taxes and legal compliance. Only applicable to managed products. |
| `default_price` | String | Default price. Cannot be zero, as in-app products are never free. Always in the developer's Checkout merchant currency. |
| `grace_period` | String | Grace period of the subscription, specified in ISO 8601 format. Allows developers to give their subscribers a grace period when the payment for the new recurrence period is declined. Acceptable values are P0D (zero days), P3D (three days), P7D (seven days), P14D (14 days), and P30D (30 days). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create inappproduct
inappproduct = provider.androidpublisher_api.Inappproduct {
    package_name = "value"  # Package name of the app.
}

# Access inappproduct outputs
inappproduct_id = inappproduct.id
inappproduct_prices = inappproduct.prices
inappproduct_subscription_taxes_and_compliance_settings = inappproduct.subscription_taxes_and_compliance_settings
inappproduct_purchase_type = inappproduct.purchase_type
inappproduct_subscription_period = inappproduct.subscription_period
inappproduct_sku = inappproduct.sku
inappproduct_package_name = inappproduct.package_name
inappproduct_listings = inappproduct.listings
inappproduct_default_language = inappproduct.default_language
inappproduct_trial_period = inappproduct.trial_period
inappproduct_status = inappproduct.status
inappproduct_managed_product_taxes_and_compliance_settings = inappproduct.managed_product_taxes_and_compliance_settings
inappproduct_default_price = inappproduct.default_price
inappproduct_grace_period = inappproduct.grace_period
```

---


### Detail

Gets details of an app.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `contact_website` | String |  | The user-visible website for this app. |
| `contact_phone` | String |  | The user-visible support telephone number for this app. |
| `default_language` | String |  | Default language code, in BCP 47 format (eg "en-US"). |
| `contact_email` | String |  | The user-visible support email for this app. |
| `package_name` | String | ✅ | Package name of the app. |
| `edit_id` | String | ✅ | Identifier of the edit. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `contact_website` | String | The user-visible website for this app. |
| `contact_phone` | String | The user-visible support telephone number for this app. |
| `default_language` | String | Default language code, in BCP 47 format (eg "en-US"). |
| `contact_email` | String | The user-visible support email for this app. |


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
detail_contact_website = detail.contact_website
detail_contact_phone = detail.contact_phone
detail_default_language = detail.default_language
detail_contact_email = detail.contact_email
```

---


### User

Grant access for a user to the given developer account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expiration_time` | String |  | The time at which the user's access expires, if set. When setting this value, it must always be in the future. |
| `name` | String |  | Required. Resource name for this user, following the pattern "developers/{developer}/users/{email}". |
| `email` | String |  | Immutable. The user's email address. |
| `access_state` | String |  | Output only. The state of the user's access to the Play Console. |
| `developer_account_permissions` | Vec<String> |  | Permissions for the user which apply across the developer account. |
| `grants` | Vec<String> |  | Output only. Per-app permissions for the user. |
| `partial` | bool |  | Output only. Whether there are more permissions for the user that are not represented here. This can happen if the caller does not have permission to manage all apps in the account. This is also `true` if this user is the account owner. If this field is `true`, it should be taken as a signal that this user cannot be fully managed via the API. That is, the API caller is not be able to manage all of the permissions this user holds, either because it doesn't know about them or because the user is the account owner. |
| `parent` | String | ✅ | Required. The developer account to add the user to. Format: developers/{developer} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `users` | Vec<String> | The resulting users. |
| `next_page_token` | String | A token to pass to subsequent calls in order to retrieve subsequent results. This will not be set if there are no more results to return. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.androidpublisher_api.User {
    parent = "value"  # Required. The developer account to add the user to. Format: developers/{developer}
}

# Access user outputs
user_id = user.id
user_users = user.users
user_next_page_token = user.next_page_token
```

---


### Listing

Gets a localized store listing.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String |  | Localized title of the app. |
| `full_description` | String |  | Full description of the app. |
| `language` | String |  | Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German). |
| `short_description` | String |  | Short description of the app. |
| `video` | String |  | URL of a promotional YouTube video for the app. |
| `edit_id` | String | ✅ | Identifier of the edit. |
| `language` | String | ✅ | Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German). |
| `package_name` | String | ✅ | Package name of the app. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `title` | String | Localized title of the app. |
| `full_description` | String | Full description of the app. |
| `language` | String | Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German). |
| `short_description` | String | Short description of the app. |
| `video` | String | URL of a promotional YouTube video for the app. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access listing outputs
listing_id = listing.id
listing_title = listing.title
listing_full_description = listing.full_description
listing_language = listing.language
listing_short_description = listing.short_description
listing_video = listing.video
```

---


### Externaltransaction

Creates a new external transaction.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `test_purchase` | String |  | Output only. If set, this transaction was a test purchase. Google will not charge for a test transaction. |
| `current_pre_tax_amount` | String |  | Output only. The current transaction amount before tax. This represents the current pre-tax amount including any refunds that may have been applied to this transaction. |
| `transaction_program_code` | i64 |  | Optional. The transaction program code, used to help determine service fee for eligible apps participating in partner programs. Developers participating in the Play Media Experience Program (https://play.google.com/console/about/programs/mediaprogram/) must provide the program code when reporting alternative billing transactions. If you are an eligible developer, please contact your BDM for more information on how to set this field. Note: this field can not be used for external offers transactions. |
| `transaction_state` | String |  | Output only. The current state of the transaction. |
| `user_tax_address` | String |  | Required. User address for tax computation. |
| `transaction_time` | String |  | Required. The time when the transaction was completed. |
| `original_pre_tax_amount` | String |  | Required. The original transaction amount before taxes. This represents the pre-tax amount originally notified to Google before any refunds were applied. |
| `one_time_transaction` | String |  | This is a one-time transaction and not part of a subscription. |
| `current_tax_amount` | String |  | Output only. The current tax amount. This represents the current tax amount including any refunds that may have been applied to this transaction. |
| `external_transaction_id` | String |  | Output only. The id of this transaction. All transaction ids under the same package name must be unique. Set when creating the external transaction. |
| `original_tax_amount` | String |  | Required. The original tax amount. This represents the tax amount originally notified to Google before any refunds were applied. |
| `package_name` | String |  | Output only. The resource name of the external transaction. The package name of the application the inapp products were sold (for example, 'com.some.app'). |
| `recurring_transaction` | String |  | This transaction is part of a recurring series of transactions. |
| `create_time` | String |  | Output only. The time when this transaction was created. This is the time when Google was notified of the transaction. |
| `parent` | String | ✅ | Required. The parent resource where this external transaction will be created. Format: applications/{package_name} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `test_purchase` | String | Output only. If set, this transaction was a test purchase. Google will not charge for a test transaction. |
| `current_pre_tax_amount` | String | Output only. The current transaction amount before tax. This represents the current pre-tax amount including any refunds that may have been applied to this transaction. |
| `transaction_program_code` | i64 | Optional. The transaction program code, used to help determine service fee for eligible apps participating in partner programs. Developers participating in the Play Media Experience Program (https://play.google.com/console/about/programs/mediaprogram/) must provide the program code when reporting alternative billing transactions. If you are an eligible developer, please contact your BDM for more information on how to set this field. Note: this field can not be used for external offers transactions. |
| `transaction_state` | String | Output only. The current state of the transaction. |
| `user_tax_address` | String | Required. User address for tax computation. |
| `transaction_time` | String | Required. The time when the transaction was completed. |
| `original_pre_tax_amount` | String | Required. The original transaction amount before taxes. This represents the pre-tax amount originally notified to Google before any refunds were applied. |
| `one_time_transaction` | String | This is a one-time transaction and not part of a subscription. |
| `current_tax_amount` | String | Output only. The current tax amount. This represents the current tax amount including any refunds that may have been applied to this transaction. |
| `external_transaction_id` | String | Output only. The id of this transaction. All transaction ids under the same package name must be unique. Set when creating the external transaction. |
| `original_tax_amount` | String | Required. The original tax amount. This represents the tax amount originally notified to Google before any refunds were applied. |
| `package_name` | String | Output only. The resource name of the external transaction. The package name of the application the inapp products were sold (for example, 'com.some.app'). |
| `recurring_transaction` | String | This transaction is part of a recurring series of transactions. |
| `create_time` | String | Output only. The time when this transaction was created. This is the time when Google was notified of the transaction. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create externaltransaction
externaltransaction = provider.androidpublisher_api.Externaltransaction {
    parent = "value"  # Required. The parent resource where this external transaction will be created. Format: applications/{package_name}
}

# Access externaltransaction outputs
externaltransaction_id = externaltransaction.id
externaltransaction_test_purchase = externaltransaction.test_purchase
externaltransaction_current_pre_tax_amount = externaltransaction.current_pre_tax_amount
externaltransaction_transaction_program_code = externaltransaction.transaction_program_code
externaltransaction_transaction_state = externaltransaction.transaction_state
externaltransaction_user_tax_address = externaltransaction.user_tax_address
externaltransaction_transaction_time = externaltransaction.transaction_time
externaltransaction_original_pre_tax_amount = externaltransaction.original_pre_tax_amount
externaltransaction_one_time_transaction = externaltransaction.one_time_transaction
externaltransaction_current_tax_amount = externaltransaction.current_tax_amount
externaltransaction_external_transaction_id = externaltransaction.external_transaction_id
externaltransaction_original_tax_amount = externaltransaction.original_tax_amount
externaltransaction_package_name = externaltransaction.package_name
externaltransaction_recurring_transaction = externaltransaction.recurring_transaction
externaltransaction_create_time = externaltransaction.create_time
```

---


### Monetization

Calculates the region prices, using today's exchange rate and country-specific pricing patterns, based on the price in the request for a set of regions.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `price` | String |  | The intital price to convert other regions from. Tax exclusive. |
| `package_name` | String | ✅ | Required. The app package name. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create monetization
monetization = provider.androidpublisher_api.Monetization {
    package_name = "value"  # Required. The app package name.
}

```

---


### Base_plan

Deactivates a base plan. Once deactivated, the base plan will become unavailable to new subscribers, but existing subscribers will maintain their subscription

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `latency_tolerance` | String |  | Optional. The latency tolerance for the propagation of this product update. Defaults to latency-sensitive. |
| `base_plan_id` | String |  | Required. The unique base plan ID of the base plan to deactivate. |
| `package_name` | String |  | Required. The parent app (package name) of the base plan to deactivate. |
| `product_id` | String |  | Required. The parent subscription (ID) of the base plan to deactivate. |
| `product_id` | String | ✅ | Required. The parent subscription (ID) of the base plan to deactivate. |
| `base_plan_id` | String | ✅ | Required. The unique base plan ID of the base plan to deactivate. |
| `package_name` | String | ✅ | Required. The parent app (package name) of the base plan to deactivate. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create base_plan
base_plan = provider.androidpublisher_api.Base_plan {
    product_id = "value"  # Required. The parent subscription (ID) of the base plan to deactivate.
    base_plan_id = "value"  # Required. The unique base plan ID of the base plan to deactivate.
    package_name = "value"  # Required. The parent app (package name) of the base plan to deactivate.
}

```

---


### Voidedpurchase

Lists the purchases that were canceled, refunded or charged-back.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `token_pagination` | String | Pagination information for token pagination. |
| `page_info` | String | General pagination information. |
| `voided_purchases` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access voidedpurchase outputs
voidedpurchase_id = voidedpurchase.id
voidedpurchase_token_pagination = voidedpurchase.token_pagination
voidedpurchase_page_info = voidedpurchase.page_info
voidedpurchase_voided_purchases = voidedpurchase.voided_purchases
```

---


### Device_tier_config

Creates a new device tier config for an app.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_country_sets` | Vec<String> |  | Definition of user country sets for the app. |
| `device_tier_set` | String |  | Definition of the set of device tiers for the app. |
| `device_tier_config_id` | String |  | Output only. The device tier config ID. |
| `device_groups` | Vec<String> |  | Definition of device groups for the app. |
| `package_name` | String | ✅ | Package name of the app. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_country_sets` | Vec<String> | Definition of user country sets for the app. |
| `device_tier_set` | String | Definition of the set of device tiers for the app. |
| `device_tier_config_id` | String | Output only. The device tier config ID. |
| `device_groups` | Vec<String> | Definition of device groups for the app. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device_tier_config
device_tier_config = provider.androidpublisher_api.Device_tier_config {
    package_name = "value"  # Package name of the app.
}

# Access device_tier_config outputs
device_tier_config_id = device_tier_config.id
device_tier_config_user_country_sets = device_tier_config.user_country_sets
device_tier_config_device_tier_set = device_tier_config.device_tier_set
device_tier_config_device_tier_config_id = device_tier_config.device_tier_config_id
device_tier_config_device_groups = device_tier_config.device_groups
```

---


### Productsv2

Checks the purchase and consumption status of an inapp item.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `purchase_completion_time` | String | The time when the purchase was successful, i.e., when the PurchaseState has changed to PURCHASED. This field will not be present until the payment is complete. For example, if the user initiated a pending transaction (https://developer.android.com/google/play/billing/integrate#pending), this field will not be populated until the user successfully completes the steps required to complete the transaction. |
| `kind` | String | This kind represents a ProductPurchaseV2 object in the androidpublisher service. |
| `test_purchase_context` | String | Information related to test purchases. This will only be set for test purchases. |
| `order_id` | String | The order id associated with the purchase of the inapp product. May not be set if there is no order associated with the purchase. |
| `acknowledgement_state` | String | Output only. The acknowledgement state of the purchase. |
| `obfuscated_external_account_id` | String | An obfuscated version of the id that is uniquely associated with the user's account in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedaccountid when the purchase was made. |
| `product_line_item` | Vec<String> | Contains item-level info for a ProductPurchaseV2. |
| `obfuscated_external_profile_id` | String | An obfuscated version of the id that is uniquely associated with the user's profile in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedprofileid when the purchase was made. |
| `purchase_state_context` | String | Information about the purchase state of the purchase. |
| `region_code` | String | ISO 3166-1 alpha-2 billing region code of the user at the time the product was granted. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access productsv2 outputs
productsv2_id = productsv2.id
productsv2_purchase_completion_time = productsv2.purchase_completion_time
productsv2_kind = productsv2.kind
productsv2_test_purchase_context = productsv2.test_purchase_context
productsv2_order_id = productsv2.order_id
productsv2_acknowledgement_state = productsv2.acknowledgement_state
productsv2_obfuscated_external_account_id = productsv2.obfuscated_external_account_id
productsv2_product_line_item = productsv2.product_line_item
productsv2_obfuscated_external_profile_id = productsv2.obfuscated_external_profile_id
productsv2_purchase_state_context = productsv2.purchase_state_context
productsv2_region_code = productsv2.region_code
```

---


### Countryavailability

Gets country availability.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rest_of_world` | bool | Whether artifacts in this track are available to "rest of the world" countries. |
| `countries` | Vec<String> | A list of one or more countries where artifacts in this track are available. This list includes all countries that are targeted by the track, even if only specific carriers are targeted in that country. |
| `sync_with_production` | bool | Whether this track's availability is synced with the default production track. See https://support.google.com/googleplay/android-developer/answer/7550024 for more information on syncing country availability with production. Note that if this is true, the returned "countries" and "rest_of_world" fields will reflect the values for the default production track. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access countryavailability outputs
countryavailability_id = countryavailability.id
countryavailability_rest_of_world = countryavailability.rest_of_world
countryavailability_countries = countryavailability.countries
countryavailability_sync_with_production = countryavailability.sync_with_production
```

---


### Subscription

Creates a new subscription. Newly added base plans will remain in draft state until activated.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `listings` | Vec<String> |  | Required. List of localized listings for this subscription. Must contain at least an entry for the default language of the parent app. |
| `archived` | bool |  | Output only. Deprecated: subscription archiving is not supported. |
| `package_name` | String |  | Immutable. Package name of the parent app. |
| `restricted_payment_countries` | String |  | Optional. Countries where the purchase of this subscription is restricted to payment methods registered in the same country. If empty, no payment location restrictions are imposed. |
| `product_id` | String |  | Immutable. Unique product ID of the product. Unique within the parent app. Product IDs must be composed of lower-case letters (a-z), numbers (0-9), underscores (_) and dots (.). It must start with a lower-case letter or number, and be between 1 and 40 (inclusive) characters in length. |
| `tax_and_compliance_settings` | String |  | Details about taxes and legal compliance. |
| `base_plans` | Vec<String> |  | The set of base plans for this subscription. Represents the prices and duration of the subscription if no other offers apply. |
| `package_name` | String | ✅ | Required. The parent app (package name) for which the subscription should be created. Must be equal to the package_name field on the Subscription resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `linked_purchase_token` | String | The purchase token of the originating purchase if this subscription is one of the following: 0. Re-signup of a canceled but non-lapsed subscription 1. Upgrade/downgrade from a previous subscription For example, suppose a user originally signs up and you receive purchase token X, then the user cancels and goes through the resignup flow (before their subscription lapses) and you receive purchase token Y, and finally the user upgrades their subscription and you receive purchase token Z. If you call this API with purchase token Z, this field will be set to Y. If you call this API with purchase token Y, this field will be set to X. If you call this API with purchase token X, this field will not be set. |
| `given_name` | String | The given name of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'. |
| `introductory_price_info` | String | Introductory price information of the subscription. This is only present when the subscription was purchased with an introductory price. This field does not indicate the subscription is currently in introductory price period. |
| `cancel_reason` | i64 | The reason why a subscription was canceled or is not auto-renewing. Possible values are: 0. User canceled the subscription 1. Subscription was canceled by the system, for example because of a billing problem 2. Subscription was replaced with a new subscription 3. Subscription was canceled by the developer |
| `auto_renewing` | bool | Whether the subscription will automatically be renewed when it reaches its current expiry time. |
| `order_id` | String | The order id of the latest recurring order associated with the purchase of the subscription. If the subscription was canceled because payment was declined, this will be the order id from the payment declined order. |
| `email_address` | String | The email address of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'. |
| `family_name` | String | The family name of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'. |
| `user_cancellation_time_millis` | String | The time at which the subscription was canceled by the user, in milliseconds since the epoch. Only present if cancelReason is 0. |
| `price_amount_micros` | String | Price of the subscription, For tax exclusive countries, the price doesn't include tax. For tax inclusive countries, the price includes tax. Price is expressed in micro-units, where 1,000,000 micro-units represents one unit of the currency. For example, if the subscription price is €1.99, price_amount_micros is 1990000. |
| `country_code` | String | ISO 3166-1 alpha-2 billing country/region code of the user at the time the subscription was granted. |
| `kind` | String | This kind represents a subscriptionPurchase object in the androidpublisher service. |
| `auto_resume_time_millis` | String | Time at which the subscription will be automatically resumed, in milliseconds since the Epoch. Only present if the user has requested to pause the subscription. |
| `obfuscated_external_account_id` | String | An obfuscated version of the id that is uniquely associated with the user's account in your app. Present for the following purchases: * If account linking happened as part of the subscription purchase flow. * It was specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedaccountid when the purchase was made. |
| `price_change` | String | The latest price change information available. This is present only when there is an upcoming price change for the subscription yet to be applied. Once the subscription renews with the new price or the subscription is canceled, no price change information will be returned. |
| `promotion_type` | i64 | The type of promotion applied on this purchase. This field is only set if a promotion is applied when the subscription was purchased. Possible values are: 0. One time code 1. Vanity code |
| `profile_id` | String | The Google profile id of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'. |
| `external_account_id` | String | User account identifier in the third-party service. Only present if account linking happened as part of the subscription purchase flow. |
| `obfuscated_external_profile_id` | String | An obfuscated version of the id that is uniquely associated with the user's profile in your app. Only present if specified using https://developer.android.com/reference/com/android/billingclient/api/BillingFlowParams.Builder#setobfuscatedprofileid when the purchase was made. |
| `promotion_code` | String | The promotion code applied on this purchase. This field is only set if a vanity code promotion is applied when the subscription was purchased. |
| `expiry_time_millis` | String | Time at which the subscription will expire, in milliseconds since the Epoch. |
| `price_currency_code` | String | ISO 4217 currency code for the subscription price. For example, if the price is specified in British pounds sterling, price_currency_code is "GBP". |
| `developer_payload` | String | A developer-specified string that contains supplemental information about an order. |
| `start_time_millis` | String | Time at which the subscription was granted, in milliseconds since the Epoch. |
| `acknowledgement_state` | i64 | The acknowledgement state of the subscription product. Possible values are: 0. Yet to be acknowledged 1. Acknowledged |
| `payment_state` | i64 | The payment state of the subscription. Possible values are: 0. Payment pending 1. Payment received 2. Free trial 3. Pending deferred upgrade/downgrade Not present for canceled, expired subscriptions. |
| `purchase_type` | i64 | The type of purchase of the subscription. This field is only set if this purchase was not made using the standard in-app billing flow. Possible values are: 0. Test (i.e. purchased from a license testing account) 1. Promo (i.e. purchased using a promo code) |
| `cancel_survey_result` | String | Information provided by the user when they complete the subscription cancellation flow (cancellation reason survey). |
| `profile_name` | String | The profile name of the user when the subscription was purchased. Only present for purchases made with 'Subscribe with Google'. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subscription
subscription = provider.androidpublisher_api.Subscription {
    package_name = "value"  # Required. The parent app (package name) for which the subscription should be created. Must be equal to the package_name field on the Subscription resource.
}

# Access subscription outputs
subscription_id = subscription.id
subscription_linked_purchase_token = subscription.linked_purchase_token
subscription_given_name = subscription.given_name
subscription_introductory_price_info = subscription.introductory_price_info
subscription_cancel_reason = subscription.cancel_reason
subscription_auto_renewing = subscription.auto_renewing
subscription_order_id = subscription.order_id
subscription_email_address = subscription.email_address
subscription_family_name = subscription.family_name
subscription_user_cancellation_time_millis = subscription.user_cancellation_time_millis
subscription_price_amount_micros = subscription.price_amount_micros
subscription_country_code = subscription.country_code
subscription_kind = subscription.kind
subscription_auto_resume_time_millis = subscription.auto_resume_time_millis
subscription_obfuscated_external_account_id = subscription.obfuscated_external_account_id
subscription_price_change = subscription.price_change
subscription_promotion_type = subscription.promotion_type
subscription_profile_id = subscription.profile_id
subscription_external_account_id = subscription.external_account_id
subscription_obfuscated_external_profile_id = subscription.obfuscated_external_profile_id
subscription_promotion_code = subscription.promotion_code
subscription_expiry_time_millis = subscription.expiry_time_millis
subscription_price_currency_code = subscription.price_currency_code
subscription_developer_payload = subscription.developer_payload
subscription_start_time_millis = subscription.start_time_millis
subscription_acknowledgement_state = subscription.acknowledgement_state
subscription_payment_state = subscription.payment_state
subscription_purchase_type = subscription.purchase_type
subscription_cancel_survey_result = subscription.cancel_survey_result
subscription_profile_name = subscription.profile_name
```

---


### Order

Refunds a user's subscription or in-app purchase order. Orders older than 3 years cannot be refunded.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `package_name` | String | ✅ | The package name of the application for which this subscription or in-app item was purchased (for example, 'com.some.thing'). |
| `order_id` | String | ✅ | The order ID provided to the user when the subscription or in-app order was purchased. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `buyer_address` | String | Address information for the customer, for use in tax computation. When Google is the Merchant of Record for the order, only country is shown. |
| `developer_revenue_in_buyer_currency` | String | Your revenue for this order in the buyer's currency, including deductions of partial refunds, taxes and fees. Google deducts standard transaction and third party fees from each sale, including VAT in some regions. |
| `create_time` | String | The time when the order was created. |
| `line_items` | Vec<String> | The individual line items making up this order. |
| `last_event_time` | String | The time of the last event that occurred on the order. |
| `order_details` | String | Detailed information about the order at creation time. |
| `order_history` | String | Details about events which modified the order. |
| `order_id` | String | The order ID. |
| `state` | String | The state of the order. |
| `tax` | String | The total tax paid as a part of this order. |
| `points_details` | String | Play points applied to the order, including offer information, discount rate and point values. |
| `total` | String | The final amount paid by the customer, taking into account discounts and taxes. |
| `purchase_token` | String | The token provided to the user's device when the subscription or item was purchased. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create order
order = provider.androidpublisher_api.Order {
    package_name = "value"  # The package name of the application for which this subscription or in-app item was purchased (for example, 'com.some.thing').
    order_id = "value"  # The order ID provided to the user when the subscription or in-app order was purchased.
}

# Access order outputs
order_id = order.id
order_buyer_address = order.buyer_address
order_developer_revenue_in_buyer_currency = order.developer_revenue_in_buyer_currency
order_create_time = order.create_time
order_line_items = order.line_items
order_last_event_time = order.last_event_time
order_order_details = order.order_details
order_order_history = order.order_history
order_order_id = order.order_id
order_state = order.state
order_tax = order.tax
order_points_details = order.points_details
order_total = order.total
order_purchase_token = order.purchase_token
```

---


### Image

Uploads an image of the specified language and image type, and adds to the edit.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `package_name` | String | ✅ | Package name of the app. |
| `edit_id` | String | ✅ | Identifier of the edit. |
| `image_type` | String | ✅ | Type of the Image. |
| `language` | String | ✅ | Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German). Providing a language that is not supported by the App is a no-op. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `images` | Vec<String> | All listed Images. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create image
image = provider.androidpublisher_api.Image {
    package_name = "value"  # Package name of the app.
    edit_id = "value"  # Identifier of the edit.
    image_type = "value"  # Type of the Image.
    language = "value"  # Language localization code (a BCP-47 language tag; for example, "de-AT" for Austrian German). Providing a language that is not supported by the App is a no-op.
}

# Access image outputs
image_id = image.id
image_images = image.images
```

---


### Review

Replies to a single review, or updates an existing reply.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reply_text` | String |  | The text to set as the reply. Replies of more than approximately 350 characters will be rejected. HTML tags will be stripped. |
| `package_name` | String | ✅ | Package name of the app. |
| `review_id` | String | ✅ | Unique identifier for a review. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `comments` | Vec<String> | A repeated field containing comments for the review. |
| `review_id` | String | Unique identifier for this review. |
| `author_name` | String | The name of the user who wrote the review. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create review
review = provider.androidpublisher_api.Review {
    package_name = "value"  # Package name of the app.
    review_id = "value"  # Unique identifier for a review.
}

# Access review outputs
review_id = review.id
review_comments = review.comments
review_review_id = review.review_id
review_author_name = review.author_name
```

---


### Tester

Gets testers. Note: Testers resource does not support email lists.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `google_groups` | Vec<String> |  | All testing Google Groups, as email addresses. |
| `track` | String | ✅ | The track to update. |
| `edit_id` | String | ✅ | Identifier of the edit. |
| `package_name` | String | ✅ | Package name of the app. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `google_groups` | Vec<String> | All testing Google Groups, as email addresses. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access tester outputs
tester_id = tester.id
tester_google_groups = tester.google_groups
```

---


### Offer

Creates a new subscription offer. Only auto-renewing base plans can have subscription offers. The offer state will be DRAFT until it is activated.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_id` | String |  | Required. Immutable. The ID of the parent subscription this offer belongs to. |
| `offer_id` | String |  | Required. Immutable. Unique ID of this subscription offer. Must be unique within the base plan. |
| `package_name` | String |  | Required. Immutable. The package name of the app the parent subscription belongs to. |
| `offer_tags` | Vec<String> |  | List of up to 20 custom tags specified for this offer, and returned to the app through the billing library. |
| `targeting` | String |  | The requirements that users need to fulfil to be eligible for this offer. Represents the requirements that Play will evaluate to decide whether an offer should be returned. Developers may further filter these offers themselves. |
| `phases` | Vec<String> |  | Required. The phases of this subscription offer. Must contain at least one and at most two entries. Users will always receive all these phases in the specified order. |
| `base_plan_id` | String |  | Required. Immutable. The ID of the base plan to which this offer is an extension. |
| `state` | String |  | Output only. The current state of this offer. Can be changed using Activate and Deactivate actions. NB: the base plan state supersedes this state, so an active offer may not be available if the base plan is not active. |
| `other_regions_config` | String |  | The configuration for any new locations Play may launch in the future. |
| `regional_configs` | Vec<String> |  | Required. The region-specific configuration of this offer. Must contain at least one entry. |
| `base_plan_id` | String | ✅ | Required. The parent base plan (ID) for which the offer should be created. Must be equal to the base_plan_id field on the SubscriptionOffer resource. |
| `package_name` | String | ✅ | Required. The parent app (package name) for which the offer should be created. Must be equal to the package_name field on the Subscription resource. |
| `product_id` | String | ✅ | Required. The parent subscription (ID) for which the offer should be created. Must be equal to the product_id field on the SubscriptionOffer resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `product_id` | String | Required. Immutable. The ID of the parent subscription this offer belongs to. |
| `offer_id` | String | Required. Immutable. Unique ID of this subscription offer. Must be unique within the base plan. |
| `package_name` | String | Required. Immutable. The package name of the app the parent subscription belongs to. |
| `offer_tags` | Vec<String> | List of up to 20 custom tags specified for this offer, and returned to the app through the billing library. |
| `targeting` | String | The requirements that users need to fulfil to be eligible for this offer. Represents the requirements that Play will evaluate to decide whether an offer should be returned. Developers may further filter these offers themselves. |
| `phases` | Vec<String> | Required. The phases of this subscription offer. Must contain at least one and at most two entries. Users will always receive all these phases in the specified order. |
| `base_plan_id` | String | Required. Immutable. The ID of the base plan to which this offer is an extension. |
| `state` | String | Output only. The current state of this offer. Can be changed using Activate and Deactivate actions. NB: the base plan state supersedes this state, so an active offer may not be available if the base plan is not active. |
| `other_regions_config` | String | The configuration for any new locations Play may launch in the future. |
| `regional_configs` | Vec<String> | Required. The region-specific configuration of this offer. Must contain at least one entry. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create offer
offer = provider.androidpublisher_api.Offer {
    base_plan_id = "value"  # Required. The parent base plan (ID) for which the offer should be created. Must be equal to the base_plan_id field on the SubscriptionOffer resource.
    package_name = "value"  # Required. The parent app (package name) for which the offer should be created. Must be equal to the package_name field on the Subscription resource.
    product_id = "value"  # Required. The parent subscription (ID) for which the offer should be created. Must be equal to the product_id field on the SubscriptionOffer resource.
}

# Access offer outputs
offer_id = offer.id
offer_product_id = offer.product_id
offer_offer_id = offer.offer_id
offer_package_name = offer.package_name
offer_offer_tags = offer.offer_tags
offer_targeting = offer.targeting
offer_phases = offer.phases
offer_base_plan_id = offer.base_plan_id
offer_state = offer.state
offer_other_regions_config = offer.other_regions_config
offer_regional_configs = offer.regional_configs
```

---


### Generatedapk

Downloads a single signed APK generated from an app bundle.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access generatedapk outputs
generatedapk_id = generatedapk.id
```

---


### Apprecovery

Create an app recovery action with recovery status as DRAFT. Note that this action does not execute the recovery action.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `targeting` | String |  | Specifies targeting criteria for the recovery action such as regions, android sdk versions, app versions etc. |
| `remote_in_app_update` | String |  | Action type is remote in-app update. As a consequence of this action, a downloadable recovery module is also created for testing purposes. |
| `package_name` | String | ✅ | Required. Package name of the app on which recovery action is performed. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recovery_actions` | Vec<String> | List of recovery actions associated with the requested package name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create apprecovery
apprecovery = provider.androidpublisher_api.Apprecovery {
    package_name = "value"  # Required. Package name of the app on which recovery action is performed.
}

# Access apprecovery outputs
apprecovery_id = apprecovery.id
apprecovery_recovery_actions = apprecovery.recovery_actions
```

---


### Purchase_option

Activates or deactivates purchase options across one or multiple one-time products.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. The update request list of up to 100 elements. All requests must update different purchase options. |
| `package_name` | String | ✅ | Required. The parent app (package name) of the updated purchase options. |
| `product_id` | String | ✅ | Required. The product ID of the parent one-time product, if all updated purchase options belong to the same one-time product. If this batch update spans multiple one-time products, set this field to "-". |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create purchase_option
purchase_option = provider.androidpublisher_api.Purchase_option {
    package_name = "value"  # Required. The parent app (package name) of the updated purchase options.
    product_id = "value"  # Required. The product ID of the parent one-time product, if all updated purchase options belong to the same one-time product. If this batch update spans multiple one-time products, set this field to "-".
}

```

---


### Deobfuscationfile

Uploads a new deobfuscation file and attaches to the specified APK.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deobfuscation_file_type` | String | ✅ | The type of the deobfuscation file. |
| `apk_version_code` | i64 | ✅ | The version code of the APK whose Deobfuscation File is being uploaded. |
| `edit_id` | String | ✅ | Unique identifier for this edit. |
| `package_name` | String | ✅ | Unique identifier for the Android app. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deobfuscationfile
deobfuscationfile = provider.androidpublisher_api.Deobfuscationfile {
    deobfuscation_file_type = "value"  # The type of the deobfuscation file.
    apk_version_code = "value"  # The version code of the APK whose Deobfuscation File is being uploaded.
    edit_id = "value"  # Unique identifier for this edit.
    package_name = "value"  # Unique identifier for the Android app.
}

```

---


### Onetimeproduct

Deletes one or more one-time products.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. A list of delete requests of up to 100 elements. All requests must delete different one-time products. |
| `package_name` | String | ✅ | Required. The parent app (package name) for which the one-time products should be deleted. Must be equal to the package_name field on all the OneTimeProduct resources. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `purchase_options` | Vec<String> | Required. The set of purchase options for this one-time product. |
| `listings` | Vec<String> | Required. Set of localized title and description data. Must not have duplicate entries with the same language_code. |
| `regions_version` | String | Output only. The version of the regions configuration that was used to generate the one-time product. |
| `restricted_payment_countries` | String | Optional. Countries where the purchase of this one-time product is restricted to payment methods registered in the same country. If empty, no payment location restrictions are imposed. |
| `package_name` | String | Required. Immutable. Package name of the parent app. |
| `product_id` | String | Required. Immutable. Unique product ID of the product. Unique within the parent app. Product IDs must start with a number or lowercase letter, and can contain numbers (0-9), lowercase letters (a-z), underscores (_), and periods (.). |
| `tax_and_compliance_settings` | String | Details about taxes and legal compliance. |
| `offer_tags` | Vec<String> | Optional. List of up to 20 custom tags specified for this one-time product, and returned to the app through the billing library. Purchase options and offers for this product will also receive these tags in the billing library. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create onetimeproduct
onetimeproduct = provider.androidpublisher_api.Onetimeproduct {
    package_name = "value"  # Required. The parent app (package name) for which the one-time products should be deleted. Must be equal to the package_name field on all the OneTimeProduct resources.
}

# Access onetimeproduct outputs
onetimeproduct_id = onetimeproduct.id
onetimeproduct_purchase_options = onetimeproduct.purchase_options
onetimeproduct_listings = onetimeproduct.listings
onetimeproduct_regions_version = onetimeproduct.regions_version
onetimeproduct_restricted_payment_countries = onetimeproduct.restricted_payment_countries
onetimeproduct_package_name = onetimeproduct.package_name
onetimeproduct_product_id = onetimeproduct.product_id
onetimeproduct_tax_and_compliance_settings = onetimeproduct.tax_and_compliance_settings
onetimeproduct_offer_tags = onetimeproduct.offer_tags
```

---


### Apk

Creates a new APK without uploading the APK itself to Google Play, instead hosting the APK at a specified URL. This function is only available to organizations using Managed Play whose application is configured to restrict distribution to the organizations.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `externally_hosted_apk` | String |  | The definition of the externally-hosted APK and where it is located. |
| `package_name` | String | ✅ | Package name of the app. |
| `edit_id` | String | ✅ | Identifier of the edit. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `apks` | Vec<String> | All APKs. |
| `kind` | String | The kind of this response ("androidpublisher#apksListResponse"). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create apk
apk = provider.androidpublisher_api.Apk {
    package_name = "value"  # Package name of the app.
    edit_id = "value"  # Identifier of the edit.
}

# Access apk outputs
apk_id = apk.id
apk_apks = apk.apks
apk_kind = apk.kind
```

---


### Edit

Creates a new edit for an app.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Output only. Identifier of the edit. Can be used in subsequent API calls. |
| `expiry_time_seconds` | String |  | Output only. The time (as seconds since Epoch) at which the edit will expire and will be no longer valid for use. |
| `package_name` | String | ✅ | Package name of the app. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Output only. Identifier of the edit. Can be used in subsequent API calls. |
| `expiry_time_seconds` | String | Output only. The time (as seconds since Epoch) at which the edit will expire and will be no longer valid for use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create edit
edit = provider.androidpublisher_api.Edit {
    package_name = "value"  # Package name of the app.
}

# Access edit outputs
edit_id = edit.id
edit_id = edit.id
edit_expiry_time_seconds = edit.expiry_time_seconds
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple inapppurchase resources
inapppurchase_0 = provider.androidpublisher_api.Inapppurchase {
}
inapppurchase_1 = provider.androidpublisher_api.Inapppurchase {
}
inapppurchase_2 = provider.androidpublisher_api.Inapppurchase {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    inapppurchase = provider.androidpublisher_api.Inapppurchase {
    }
```

---

## Related Documentation

- [GCP Androidpublisher_api Documentation](https://cloud.google.com/androidpublisher_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
