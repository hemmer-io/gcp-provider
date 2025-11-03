# Analyticsadmin_api Service



**Resources**: 39

---

## Overview

The analyticsadmin_api service provides access to 39 resource types:

- [Key_event](#key_event) [CRUD]
- [Reporting_data_annotation](#reporting_data_annotation) [CRUD]
- [Ad_sense_link](#ad_sense_link) [CRD]
- [Expanded_data_set](#expanded_data_set) [CRUD]
- [Google_ads_link](#google_ads_link) [CRUD]
- [Event_edit_rule](#event_edit_rule) [CRUD]
- [Propertie](#propertie) [CRUD]
- [Custom_metric](#custom_metric) [CRU]
- [Data_stream](#data_stream) [CRUD]
- [Event_create_rule](#event_create_rule) [CRUD]
- [Big_query_link](#big_query_link) [CRUD]
- [Channel_group](#channel_group) [CRUD]
- [Subproperty_sync_config](#subproperty_sync_config) [RU]
- [Access_binding](#access_binding) [CRUD]
- [Calculated_metric](#calculated_metric) [CRUD]
- [Conversion_event](#conversion_event) [CRUD]
- [Account_summarie](#account_summarie) [R]
- [Account](#account) [CRUD]
- [Sk_ad_network_conversion_value_schema](#sk_ad_network_conversion_value_schema) [CRUD]
- [Search_ads360_link](#search_ads360_link) [CRUD]
- [Display_video360_advertiser_link](#display_video360_advertiser_link) [CRUD]
- [Custom_dimension](#custom_dimension) [CRU]
- [Firebase_link](#firebase_link) [CRD]
- [Rollup_property_source_link](#rollup_property_source_link) [CRD]
- [Subproperty_event_filter](#subproperty_event_filter) [CRUD]
- [Display_video360_advertiser_link_proposal](#display_video360_advertiser_link_proposal) [CRD]
- [Audience](#audience) [CRU]
- [Measurement_protocol_secret](#measurement_protocol_secret) [CRUD]
- [Account](#account) [CRUD]
- [Custom_dimension](#custom_dimension) [CRU]
- [Key_event](#key_event) [CRUD]
- [Conversion_event](#conversion_event) [CRUD]
- [Firebase_link](#firebase_link) [CRD]
- [Propertie](#propertie) [CRUD]
- [Data_stream](#data_stream) [CRUD]
- [Custom_metric](#custom_metric) [CRU]
- [Account_summarie](#account_summarie) [R]
- [Measurement_protocol_secret](#measurement_protocol_secret) [CRUD]
- [Google_ads_link](#google_ads_link) [CRUD]

---

## Resources


### Key_event

Creates a Key Event.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `counting_method` | String |  | Required. The method by which Key Events will be counted across multiple events within a session. |
| `event_name` | String |  | Immutable. The event name for this key event. Examples: 'click', 'purchase' |
| `name` | String |  | Output only. Resource name of this key event. Format: properties/{property}/keyEvents/{key_event} |
| `deletable` | bool |  | Output only. If set to true, this event can be deleted. |
| `create_time` | String |  | Output only. Time when this key event was created in the property. |
| `custom` | bool |  | Output only. If set to true, this key event refers to a custom event. If set to false, this key event refers to a default event in GA. Default events typically have special meaning in GA. Default events are usually created for you by the GA system, but in some cases can be created by property admins. Custom events count towards the maximum number of custom key events that may be created per property. |
| `default_value` | String |  | Optional. Defines a default value/currency for a key event. |
| `parent` | String | ✅ | Required. The resource name of the parent property where this Key Event will be created. Format: properties/123 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `counting_method` | String | Required. The method by which Key Events will be counted across multiple events within a session. |
| `event_name` | String | Immutable. The event name for this key event. Examples: 'click', 'purchase' |
| `name` | String | Output only. Resource name of this key event. Format: properties/{property}/keyEvents/{key_event} |
| `deletable` | bool | Output only. If set to true, this event can be deleted. |
| `create_time` | String | Output only. Time when this key event was created in the property. |
| `custom` | bool | Output only. If set to true, this key event refers to a custom event. If set to false, this key event refers to a default event in GA. Default events typically have special meaning in GA. Default events are usually created for you by the GA system, but in some cases can be created by property admins. Custom events count towards the maximum number of custom key events that may be created per property. |
| `default_value` | String | Optional. Defines a default value/currency for a key event. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create key_event
key_event = provider.analyticsadmin_api.Key_event {
    parent = "value"  # Required. The resource name of the parent property where this Key Event will be created. Format: properties/123
}

# Access key_event outputs
key_event_id = key_event.id
key_event_counting_method = key_event.counting_method
key_event_event_name = key_event.event_name
key_event_name = key_event.name
key_event_deletable = key_event.deletable
key_event_create_time = key_event.create_time
key_event_custom = key_event.custom
key_event_default_value = key_event.default_value
```

---


### Reporting_data_annotation

Creates a Reporting Data Annotation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `system_generated` | bool |  | Output only. If true, this annotation was generated by the Google Analytics system. System-generated annotations cannot be updated or deleted. |
| `annotation_date` | String |  | If set, the Reporting Data Annotation is for a specific date represented by this field. The date must be a valid date with year, month and day set. The date may be in the past, present, or future. |
| `annotation_date_range` | String |  | If set, the Reporting Data Annotation is for a range of dates represented by this field. |
| `description` | String |  | Optional. Description for this Reporting Data Annotation. |
| `color` | String |  | Required. The color used for display of this Reporting Data Annotation. |
| `name` | String |  | Required. Identifier. Resource name of this Reporting Data Annotation. Format: 'properties/{property_id}/reportingDataAnnotations/{reporting_data_annotation}' Format: 'properties/123/reportingDataAnnotations/456' |
| `title` | String |  | Required. Human-readable title for this Reporting Data Annotation. |
| `parent` | String | ✅ | Required. The property for which to create a Reporting Data Annotation. Format: properties/property_id Example: properties/123 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `system_generated` | bool | Output only. If true, this annotation was generated by the Google Analytics system. System-generated annotations cannot be updated or deleted. |
| `annotation_date` | String | If set, the Reporting Data Annotation is for a specific date represented by this field. The date must be a valid date with year, month and day set. The date may be in the past, present, or future. |
| `annotation_date_range` | String | If set, the Reporting Data Annotation is for a range of dates represented by this field. |
| `description` | String | Optional. Description for this Reporting Data Annotation. |
| `color` | String | Required. The color used for display of this Reporting Data Annotation. |
| `name` | String | Required. Identifier. Resource name of this Reporting Data Annotation. Format: 'properties/{property_id}/reportingDataAnnotations/{reporting_data_annotation}' Format: 'properties/123/reportingDataAnnotations/456' |
| `title` | String | Required. Human-readable title for this Reporting Data Annotation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reporting_data_annotation
reporting_data_annotation = provider.analyticsadmin_api.Reporting_data_annotation {
    parent = "value"  # Required. The property for which to create a Reporting Data Annotation. Format: properties/property_id Example: properties/123
}

# Access reporting_data_annotation outputs
reporting_data_annotation_id = reporting_data_annotation.id
reporting_data_annotation_system_generated = reporting_data_annotation.system_generated
reporting_data_annotation_annotation_date = reporting_data_annotation.annotation_date
reporting_data_annotation_annotation_date_range = reporting_data_annotation.annotation_date_range
reporting_data_annotation_description = reporting_data_annotation.description
reporting_data_annotation_color = reporting_data_annotation.color
reporting_data_annotation_name = reporting_data_annotation.name
reporting_data_annotation_title = reporting_data_annotation.title
```

---


### Ad_sense_link

Creates an AdSenseLink.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ad_client_code` | String |  | Immutable. The AdSense ad client code that the Google Analytics property is linked to. Example format: "ca-pub-1234567890" |
| `name` | String |  | Output only. The resource name for this AdSense Link resource. Format: properties/{propertyId}/adSenseLinks/{linkId} Example: properties/1234/adSenseLinks/6789 |
| `parent` | String | ✅ | Required. The property for which to create an AdSense Link. Format: properties/{propertyId} Example: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ad_client_code` | String | Immutable. The AdSense ad client code that the Google Analytics property is linked to. Example format: "ca-pub-1234567890" |
| `name` | String | Output only. The resource name for this AdSense Link resource. Format: properties/{propertyId}/adSenseLinks/{linkId} Example: properties/1234/adSenseLinks/6789 |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ad_sense_link
ad_sense_link = provider.analyticsadmin_api.Ad_sense_link {
    parent = "value"  # Required. The property for which to create an AdSense Link. Format: properties/{propertyId} Example: properties/1234
}

# Access ad_sense_link outputs
ad_sense_link_id = ad_sense_link.id
ad_sense_link_ad_client_code = ad_sense_link.ad_client_code
ad_sense_link_name = ad_sense_link.name
```

---


### Expanded_data_set

Creates a ExpandedDataSet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dimension_filter_expression` | String |  | Immutable. A logical expression of ExpandedDataSet filters applied to dimension included in the ExpandedDataSet. This filter is used to reduce the number of rows and thus the chance of encountering `other` row. |
| `display_name` | String |  | Required. The display name of the ExpandedDataSet. Max 200 chars. |
| `data_collection_start_time` | String |  | Output only. Time when expanded data set began (or will begin) collecing data. |
| `description` | String |  | Optional. The description of the ExpandedDataSet. Max 50 chars. |
| `dimension_names` | Vec<String> |  | Immutable. The list of dimensions included in the ExpandedDataSet. See the [API Dimensions](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#dimensions) for the list of dimension names. |
| `metric_names` | Vec<String> |  | Immutable. The list of metrics included in the ExpandedDataSet. See the [API Metrics](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#metrics) for the list of dimension names. |
| `name` | String |  | Output only. The resource name for this ExpandedDataSet resource. Format: properties/{property_id}/expandedDataSets/{expanded_data_set} |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dimension_filter_expression` | String | Immutable. A logical expression of ExpandedDataSet filters applied to dimension included in the ExpandedDataSet. This filter is used to reduce the number of rows and thus the chance of encountering `other` row. |
| `display_name` | String | Required. The display name of the ExpandedDataSet. Max 200 chars. |
| `data_collection_start_time` | String | Output only. Time when expanded data set began (or will begin) collecing data. |
| `description` | String | Optional. The description of the ExpandedDataSet. Max 50 chars. |
| `dimension_names` | Vec<String> | Immutable. The list of dimensions included in the ExpandedDataSet. See the [API Dimensions](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#dimensions) for the list of dimension names. |
| `metric_names` | Vec<String> | Immutable. The list of metrics included in the ExpandedDataSet. See the [API Metrics](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#metrics) for the list of dimension names. |
| `name` | String | Output only. The resource name for this ExpandedDataSet resource. Format: properties/{property_id}/expandedDataSets/{expanded_data_set} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create expanded_data_set
expanded_data_set = provider.analyticsadmin_api.Expanded_data_set {
    parent = "value"  # Required. Example format: properties/1234
}

# Access expanded_data_set outputs
expanded_data_set_id = expanded_data_set.id
expanded_data_set_dimension_filter_expression = expanded_data_set.dimension_filter_expression
expanded_data_set_display_name = expanded_data_set.display_name
expanded_data_set_data_collection_start_time = expanded_data_set.data_collection_start_time
expanded_data_set_description = expanded_data_set.description
expanded_data_set_dimension_names = expanded_data_set.dimension_names
expanded_data_set_metric_names = expanded_data_set.metric_names
expanded_data_set_name = expanded_data_set.name
```

---


### Google_ads_link

Creates a GoogleAdsLink.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time when this link was originally created. |
| `name` | String |  | Output only. Format: properties/{propertyId}/googleAdsLinks/{googleAdsLinkId} Note: googleAdsLinkId is not the Google Ads customer ID. |
| `update_time` | String |  | Output only. Time when this link was last updated. |
| `ads_personalization_enabled` | bool |  | Enable personalized advertising features with this integration. Automatically publish my Google Analytics audience lists and Google Analytics remarketing events/parameters to the linked Google Ads account. If this field is not set on create/update, it will be defaulted to true. |
| `customer_id` | String |  | Immutable. Google Ads customer ID. |
| `creator_email_address` | String |  | Output only. Email address of the user that created the link. An empty string will be returned if the email address can't be retrieved. |
| `can_manage_clients` | bool |  | Output only. If true, this link is for a Google Ads manager account. |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `google_ads_links` | Vec<String> | List of GoogleAdsLinks. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create google_ads_link
google_ads_link = provider.analyticsadmin_api.Google_ads_link {
    parent = "value"  # Required. Example format: properties/1234
}

# Access google_ads_link outputs
google_ads_link_id = google_ads_link.id
google_ads_link_next_page_token = google_ads_link.next_page_token
google_ads_link_google_ads_links = google_ads_link.google_ads_links
```

---


### Event_edit_rule

Creates an EventEditRule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameter_mutations` | Vec<String> |  | Required. Parameter mutations define parameter behavior on the new event, and are applied in order. A maximum of 20 mutations can be applied. |
| `event_conditions` | Vec<String> |  | Required. Conditions on the source event must match for this rule to be applied. Must have at least one condition, and can have up to 10 max. |
| `processing_order` | String |  | Output only. The order for which this rule will be processed. Rules with an order value lower than this will be processed before this rule, rules with an order value higher than this will be processed after this rule. New event edit rules will be assigned an order value at the end of the order. This value does not apply to event create rules. |
| `display_name` | String |  | Required. The display name of this event edit rule. Maximum of 255 characters. |
| `name` | String |  | Identifier. Resource name for this EventEditRule resource. Format: properties/{property}/dataStreams/{data_stream}/eventEditRules/{event_edit_rule} |
| `parent` | String | ✅ | Required. Example format: properties/123/dataStreams/456 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameter_mutations` | Vec<String> | Required. Parameter mutations define parameter behavior on the new event, and are applied in order. A maximum of 20 mutations can be applied. |
| `event_conditions` | Vec<String> | Required. Conditions on the source event must match for this rule to be applied. Must have at least one condition, and can have up to 10 max. |
| `processing_order` | String | Output only. The order for which this rule will be processed. Rules with an order value lower than this will be processed before this rule, rules with an order value higher than this will be processed after this rule. New event edit rules will be assigned an order value at the end of the order. This value does not apply to event create rules. |
| `display_name` | String | Required. The display name of this event edit rule. Maximum of 255 characters. |
| `name` | String | Identifier. Resource name for this EventEditRule resource. Format: properties/{property}/dataStreams/{data_stream}/eventEditRules/{event_edit_rule} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create event_edit_rule
event_edit_rule = provider.analyticsadmin_api.Event_edit_rule {
    parent = "value"  # Required. Example format: properties/123/dataStreams/456
}

# Access event_edit_rule outputs
event_edit_rule_id = event_edit_rule.id
event_edit_rule_parameter_mutations = event_edit_rule.parameter_mutations
event_edit_rule_event_conditions = event_edit_rule.event_conditions
event_edit_rule_processing_order = event_edit_rule.processing_order
event_edit_rule_display_name = event_edit_rule.display_name
event_edit_rule_name = event_edit_rule.name
```

---


### Propertie

Creates a Google Analytics property with the specified location and attributes.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expire_time` | String |  | Output only. If set, the time at which this trashed property will be permanently deleted. If not set, then this property is not currently in the trash can and is not slated to be deleted. |
| `account` | String |  | Immutable. The resource name of the parent account Format: accounts/{account_id} Example: "accounts/123" |
| `name` | String |  | Output only. Resource name of this property. Format: properties/{property_id} Example: "properties/1000" |
| `time_zone` | String |  | Required. Reporting Time Zone, used as the day boundary for reports, regardless of where the data originates. If the time zone honors DST, Analytics will automatically adjust for the changes. NOTE: Changing the time zone only affects data going forward, and is not applied retroactively. Format: https://www.iana.org/time-zones Example: "America/Los_Angeles" |
| `currency_code` | String |  | The currency type used in reports involving monetary values. Format: https://en.wikipedia.org/wiki/ISO_4217 Examples: "USD", "EUR", "JPY" |
| `display_name` | String |  | Required. Human-readable display name for this property. The max allowed display name length is 100 UTF-16 code units. |
| `parent` | String |  | Immutable. Resource name of this property's logical parent. Note: The Property-Moving UI can be used to change the parent. Format: accounts/{account}, properties/{property} Example: "accounts/100", "properties/101" |
| `industry_category` | String |  | Industry associated with this property Example: AUTOMOTIVE, FOOD_AND_DRINK |
| `create_time` | String |  | Output only. Time when the entity was originally created. |
| `update_time` | String |  | Output only. Time when entity payload fields were last updated. |
| `property_type` | String |  | Immutable. The property type for this Property resource. When creating a property, if the type is "PROPERTY_TYPE_UNSPECIFIED", then "ORDINARY_PROPERTY" will be implied. |
| `delete_time` | String |  | Output only. If set, the time at which this property was trashed. If not set, then this property is not currently in the trash can. |
| `service_level` | String |  | Output only. The Google Analytics service level that applies to this property. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expire_time` | String | Output only. If set, the time at which this trashed property will be permanently deleted. If not set, then this property is not currently in the trash can and is not slated to be deleted. |
| `account` | String | Immutable. The resource name of the parent account Format: accounts/{account_id} Example: "accounts/123" |
| `name` | String | Output only. Resource name of this property. Format: properties/{property_id} Example: "properties/1000" |
| `time_zone` | String | Required. Reporting Time Zone, used as the day boundary for reports, regardless of where the data originates. If the time zone honors DST, Analytics will automatically adjust for the changes. NOTE: Changing the time zone only affects data going forward, and is not applied retroactively. Format: https://www.iana.org/time-zones Example: "America/Los_Angeles" |
| `currency_code` | String | The currency type used in reports involving monetary values. Format: https://en.wikipedia.org/wiki/ISO_4217 Examples: "USD", "EUR", "JPY" |
| `display_name` | String | Required. Human-readable display name for this property. The max allowed display name length is 100 UTF-16 code units. |
| `parent` | String | Immutable. Resource name of this property's logical parent. Note: The Property-Moving UI can be used to change the parent. Format: accounts/{account}, properties/{property} Example: "accounts/100", "properties/101" |
| `industry_category` | String | Industry associated with this property Example: AUTOMOTIVE, FOOD_AND_DRINK |
| `create_time` | String | Output only. Time when the entity was originally created. |
| `update_time` | String | Output only. Time when entity payload fields were last updated. |
| `property_type` | String | Immutable. The property type for this Property resource. When creating a property, if the type is "PROPERTY_TYPE_UNSPECIFIED", then "ORDINARY_PROPERTY" will be implied. |
| `delete_time` | String | Output only. If set, the time at which this property was trashed. If not set, then this property is not currently in the trash can. |
| `service_level` | String | Output only. The Google Analytics service level that applies to this property. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create propertie
propertie = provider.analyticsadmin_api.Propertie {
}

# Access propertie outputs
propertie_id = propertie.id
propertie_expire_time = propertie.expire_time
propertie_account = propertie.account
propertie_name = propertie.name
propertie_time_zone = propertie.time_zone
propertie_currency_code = propertie.currency_code
propertie_display_name = propertie.display_name
propertie_parent = propertie.parent
propertie_industry_category = propertie.industry_category
propertie_create_time = propertie.create_time
propertie_update_time = propertie.update_time
propertie_property_type = propertie.property_type
propertie_delete_time = propertie.delete_time
propertie_service_level = propertie.service_level
```

---


### Custom_metric

Creates a CustomMetric.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name for this CustomMetric resource. Format: properties/{property}/customMetrics/{customMetric} |
| `display_name` | String |  | Required. Display name for this custom metric as shown in the Analytics UI. Max length of 82 characters, alphanumeric plus space and underscore starting with a letter. Legacy system-generated display names may contain square brackets, but updates to this field will never permit square brackets. |
| `restricted_metric_type` | Vec<String> |  | Optional. Types of restricted data that this metric may contain. Required for metrics with CURRENCY measurement unit. Must be empty for metrics with a non-CURRENCY measurement unit. |
| `scope` | String |  | Required. Immutable. The scope of this custom metric. |
| `parameter_name` | String |  | Required. Immutable. Tagging name for this custom metric. If this is an event-scoped metric, then this is the event parameter name. May only contain alphanumeric and underscore charactes, starting with a letter. Max length of 40 characters for event-scoped metrics. |
| `description` | String |  | Optional. Description for this custom dimension. Max length of 150 characters. |
| `measurement_unit` | String |  | Required. The type for the custom metric's value. |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name for this CustomMetric resource. Format: properties/{property}/customMetrics/{customMetric} |
| `display_name` | String | Required. Display name for this custom metric as shown in the Analytics UI. Max length of 82 characters, alphanumeric plus space and underscore starting with a letter. Legacy system-generated display names may contain square brackets, but updates to this field will never permit square brackets. |
| `restricted_metric_type` | Vec<String> | Optional. Types of restricted data that this metric may contain. Required for metrics with CURRENCY measurement unit. Must be empty for metrics with a non-CURRENCY measurement unit. |
| `scope` | String | Required. Immutable. The scope of this custom metric. |
| `parameter_name` | String | Required. Immutable. Tagging name for this custom metric. If this is an event-scoped metric, then this is the event parameter name. May only contain alphanumeric and underscore charactes, starting with a letter. Max length of 40 characters for event-scoped metrics. |
| `description` | String | Optional. Description for this custom dimension. Max length of 150 characters. |
| `measurement_unit` | String | Required. The type for the custom metric's value. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_metric
custom_metric = provider.analyticsadmin_api.Custom_metric {
    parent = "value"  # Required. Example format: properties/1234
}

# Access custom_metric outputs
custom_metric_id = custom_metric.id
custom_metric_name = custom_metric.name
custom_metric_display_name = custom_metric.display_name
custom_metric_restricted_metric_type = custom_metric.restricted_metric_type
custom_metric_scope = custom_metric.scope
custom_metric_parameter_name = custom_metric.parameter_name
custom_metric_description = custom_metric.description
custom_metric_measurement_unit = custom_metric.measurement_unit
```

---


### Data_stream

Creates a DataStream.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Human-readable display name for the Data Stream. Required for web data streams. The max allowed display name length is 255 UTF-16 code units. |
| `android_app_stream_data` | String |  | Data specific to Android app streams. Must be populated if type is ANDROID_APP_DATA_STREAM. |
| `update_time` | String |  | Output only. Time when stream payload fields were last updated. |
| `type` | String |  | Required. Immutable. The type of this DataStream resource. |
| `ios_app_stream_data` | String |  | Data specific to iOS app streams. Must be populated if type is IOS_APP_DATA_STREAM. |
| `web_stream_data` | String |  | Data specific to web streams. Must be populated if type is WEB_DATA_STREAM. |
| `name` | String |  | Output only. Resource name of this Data Stream. Format: properties/{property_id}/dataStreams/{stream_id} Example: "properties/1000/dataStreams/2000" |
| `create_time` | String |  | Output only. Time when this stream was originally created. |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Human-readable display name for the Data Stream. Required for web data streams. The max allowed display name length is 255 UTF-16 code units. |
| `android_app_stream_data` | String | Data specific to Android app streams. Must be populated if type is ANDROID_APP_DATA_STREAM. |
| `update_time` | String | Output only. Time when stream payload fields were last updated. |
| `type` | String | Required. Immutable. The type of this DataStream resource. |
| `ios_app_stream_data` | String | Data specific to iOS app streams. Must be populated if type is IOS_APP_DATA_STREAM. |
| `web_stream_data` | String | Data specific to web streams. Must be populated if type is WEB_DATA_STREAM. |
| `name` | String | Output only. Resource name of this Data Stream. Format: properties/{property_id}/dataStreams/{stream_id} Example: "properties/1000/dataStreams/2000" |
| `create_time` | String | Output only. Time when this stream was originally created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_stream
data_stream = provider.analyticsadmin_api.Data_stream {
    parent = "value"  # Required. Example format: properties/1234
}

# Access data_stream outputs
data_stream_id = data_stream.id
data_stream_display_name = data_stream.display_name
data_stream_android_app_stream_data = data_stream.android_app_stream_data
data_stream_update_time = data_stream.update_time
data_stream_type = data_stream.type
data_stream_ios_app_stream_data = data_stream.ios_app_stream_data
data_stream_web_stream_data = data_stream.web_stream_data
data_stream_name = data_stream.name
data_stream_create_time = data_stream.create_time
```

---


### Event_create_rule

Creates an EventCreateRule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name for this EventCreateRule resource. Format: properties/{property}/dataStreams/{data_stream}/eventCreateRules/{event_create_rule} |
| `destination_event` | String |  | Required. The name of the new event to be created. This value must: * be less than 40 characters * consist only of letters, digits or _ (underscores) * start with a letter |
| `parameter_mutations` | Vec<String> |  | Parameter mutations define parameter behavior on the new event, and are applied in order. A maximum of 20 mutations can be applied. |
| `source_copy_parameters` | bool |  | If true, the source parameters are copied to the new event. If false, or unset, all non-internal parameters are not copied from the source event. Parameter mutations are applied after the parameters have been copied. |
| `event_conditions` | Vec<String> |  | Required. Must have at least one condition, and can have up to 10 max. Conditions on the source event must match for this rule to be applied. |
| `parent` | String | ✅ | Required. Example format: properties/123/dataStreams/456 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name for this EventCreateRule resource. Format: properties/{property}/dataStreams/{data_stream}/eventCreateRules/{event_create_rule} |
| `destination_event` | String | Required. The name of the new event to be created. This value must: * be less than 40 characters * consist only of letters, digits or _ (underscores) * start with a letter |
| `parameter_mutations` | Vec<String> | Parameter mutations define parameter behavior on the new event, and are applied in order. A maximum of 20 mutations can be applied. |
| `source_copy_parameters` | bool | If true, the source parameters are copied to the new event. If false, or unset, all non-internal parameters are not copied from the source event. Parameter mutations are applied after the parameters have been copied. |
| `event_conditions` | Vec<String> | Required. Must have at least one condition, and can have up to 10 max. Conditions on the source event must match for this rule to be applied. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create event_create_rule
event_create_rule = provider.analyticsadmin_api.Event_create_rule {
    parent = "value"  # Required. Example format: properties/123/dataStreams/456
}

# Access event_create_rule outputs
event_create_rule_id = event_create_rule.id
event_create_rule_name = event_create_rule.name
event_create_rule_destination_event = event_create_rule.destination_event
event_create_rule_parameter_mutations = event_create_rule.parameter_mutations
event_create_rule_source_copy_parameters = event_create_rule.source_copy_parameters
event_create_rule_event_conditions = event_create_rule.event_conditions
```

---


### Big_query_link

Creates a BigQueryLink.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `include_advertising_id` | bool |  | If set true, exported data will include advertising identifiers for mobile app streams. |
| `fresh_daily_export_enabled` | bool |  | If set true, enables fresh daily export to the linked Google Cloud project. |
| `streaming_export_enabled` | bool |  | If set true, enables streaming export to the linked Google Cloud project. |
| `excluded_events` | Vec<String> |  | The list of event names that will be excluded from exports. |
| `export_streams` | Vec<String> |  | The list of streams under the parent property for which data will be exported. Format: properties/{property_id}/dataStreams/{stream_id} Example: ['properties/1000/dataStreams/2000'] |
| `create_time` | String |  | Output only. Time when the link was created. |
| `dataset_location` | String |  | Required. Immutable. The geographic location where the created BigQuery dataset should reside. See https://cloud.google.com/bigquery/docs/locations for supported locations. |
| `project` | String |  | Immutable. The linked Google Cloud project. When creating a BigQueryLink, you may provide this resource name using either a project number or project ID. Once this resource has been created, the returned project will always have a project that contains a project number. Format: 'projects/{project number}' Example: 'projects/1234' |
| `daily_export_enabled` | bool |  | If set true, enables daily data export to the linked Google Cloud project. |
| `name` | String |  | Output only. Resource name of this BigQuery link. Format: 'properties/{property_id}/bigQueryLinks/{bigquery_link_id}' Format: 'properties/1234/bigQueryLinks/abc567' |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `include_advertising_id` | bool | If set true, exported data will include advertising identifiers for mobile app streams. |
| `fresh_daily_export_enabled` | bool | If set true, enables fresh daily export to the linked Google Cloud project. |
| `streaming_export_enabled` | bool | If set true, enables streaming export to the linked Google Cloud project. |
| `excluded_events` | Vec<String> | The list of event names that will be excluded from exports. |
| `export_streams` | Vec<String> | The list of streams under the parent property for which data will be exported. Format: properties/{property_id}/dataStreams/{stream_id} Example: ['properties/1000/dataStreams/2000'] |
| `create_time` | String | Output only. Time when the link was created. |
| `dataset_location` | String | Required. Immutable. The geographic location where the created BigQuery dataset should reside. See https://cloud.google.com/bigquery/docs/locations for supported locations. |
| `project` | String | Immutable. The linked Google Cloud project. When creating a BigQueryLink, you may provide this resource name using either a project number or project ID. Once this resource has been created, the returned project will always have a project that contains a project number. Format: 'projects/{project number}' Example: 'projects/1234' |
| `daily_export_enabled` | bool | If set true, enables daily data export to the linked Google Cloud project. |
| `name` | String | Output only. Resource name of this BigQuery link. Format: 'properties/{property_id}/bigQueryLinks/{bigquery_link_id}' Format: 'properties/1234/bigQueryLinks/abc567' |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create big_query_link
big_query_link = provider.analyticsadmin_api.Big_query_link {
    parent = "value"  # Required. Example format: properties/1234
}

# Access big_query_link outputs
big_query_link_id = big_query_link.id
big_query_link_include_advertising_id = big_query_link.include_advertising_id
big_query_link_fresh_daily_export_enabled = big_query_link.fresh_daily_export_enabled
big_query_link_streaming_export_enabled = big_query_link.streaming_export_enabled
big_query_link_excluded_events = big_query_link.excluded_events
big_query_link_export_streams = big_query_link.export_streams
big_query_link_create_time = big_query_link.create_time
big_query_link_dataset_location = big_query_link.dataset_location
big_query_link_project = big_query_link.project
big_query_link_daily_export_enabled = big_query_link.daily_export_enabled
big_query_link_name = big_query_link.name
```

---


### Channel_group

Creates a ChannelGroup.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The resource name for this Channel Group resource. Format: properties/{property}/channelGroups/{channel_group} |
| `primary` | bool |  | Optional. If true, this channel group will be used as the default channel group for reports. Only one channel group can be set as `primary` at any time. If the `primary` field gets set on a channel group, it will get unset on the previous primary channel group. The Google Analytics predefined channel group is the primary by default. |
| `system_defined` | bool |  | Output only. If true, then this channel group is the Default Channel Group predefined by Google Analytics. Display name and grouping rules cannot be updated for this channel group. |
| `description` | String |  | The description of the Channel Group. Max length of 256 characters. |
| `display_name` | String |  | Required. The display name of the Channel Group. Max length of 80 characters. |
| `grouping_rule` | Vec<String> |  | Required. The grouping rules of channels. Maximum number of rules is 50. |
| `parent` | String | ✅ | Required. The property for which to create a ChannelGroup. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name for this Channel Group resource. Format: properties/{property}/channelGroups/{channel_group} |
| `primary` | bool | Optional. If true, this channel group will be used as the default channel group for reports. Only one channel group can be set as `primary` at any time. If the `primary` field gets set on a channel group, it will get unset on the previous primary channel group. The Google Analytics predefined channel group is the primary by default. |
| `system_defined` | bool | Output only. If true, then this channel group is the Default Channel Group predefined by Google Analytics. Display name and grouping rules cannot be updated for this channel group. |
| `description` | String | The description of the Channel Group. Max length of 256 characters. |
| `display_name` | String | Required. The display name of the Channel Group. Max length of 80 characters. |
| `grouping_rule` | Vec<String> | Required. The grouping rules of channels. Maximum number of rules is 50. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel_group
channel_group = provider.analyticsadmin_api.Channel_group {
    parent = "value"  # Required. The property for which to create a ChannelGroup. Example format: properties/1234
}

# Access channel_group outputs
channel_group_id = channel_group.id
channel_group_name = channel_group.name
channel_group_primary = channel_group.primary
channel_group_system_defined = channel_group.system_defined
channel_group_description = channel_group.description
channel_group_display_name = channel_group.display_name
channel_group_grouping_rule = channel_group.grouping_rule
```

---


### Subproperty_sync_config

Lookup for a single `SubpropertySyncConfig`.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `apply_to_property` | String |  | Output only. Immutable. Resource name of the subproperty that these settings apply to. |
| `custom_dimension_and_metric_sync_mode` | String |  | Required. Specifies the Custom Dimension / Metric synchronization mode for the subproperty. If set to ALL, Custom Dimension / Metric synchronization will be immediately enabled. Local configuration of Custom Dimensions / Metrics will not be allowed on the subproperty so long as the synchronization mode is set to ALL. If set to NONE, Custom Dimensions / Metric synchronization is disabled. Custom Dimensions / Metrics must be configured explicitly on the Subproperty. |
| `name` | String |  | Output only. Identifier. Format: properties/{ordinary_property_id}/subpropertySyncConfigs/{subproperty_id} Example: properties/1234/subpropertySyncConfigs/5678 |
| `name` | String | ✅ | Output only. Identifier. Format: properties/{ordinary_property_id}/subpropertySyncConfigs/{subproperty_id} Example: properties/1234/subpropertySyncConfigs/5678 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `apply_to_property` | String | Output only. Immutable. Resource name of the subproperty that these settings apply to. |
| `custom_dimension_and_metric_sync_mode` | String | Required. Specifies the Custom Dimension / Metric synchronization mode for the subproperty. If set to ALL, Custom Dimension / Metric synchronization will be immediately enabled. Local configuration of Custom Dimensions / Metrics will not be allowed on the subproperty so long as the synchronization mode is set to ALL. If set to NONE, Custom Dimensions / Metric synchronization is disabled. Custom Dimensions / Metrics must be configured explicitly on the Subproperty. |
| `name` | String | Output only. Identifier. Format: properties/{ordinary_property_id}/subpropertySyncConfigs/{subproperty_id} Example: properties/1234/subpropertySyncConfigs/5678 |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access subproperty_sync_config outputs
subproperty_sync_config_id = subproperty_sync_config.id
subproperty_sync_config_apply_to_property = subproperty_sync_config.apply_to_property
subproperty_sync_config_custom_dimension_and_metric_sync_mode = subproperty_sync_config.custom_dimension_and_metric_sync_mode
subproperty_sync_config_name = subproperty_sync_config.name
```

---


### Access_binding

Creates an access binding on an account or property.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of this binding. Format: accounts/{account}/accessBindings/{access_binding} or properties/{property}/accessBindings/{access_binding} Example: "accounts/100/accessBindings/200" |
| `roles` | Vec<String> |  | A list of roles for to grant to the parent resource. Valid values: predefinedRoles/viewer predefinedRoles/analyst predefinedRoles/editor predefinedRoles/admin predefinedRoles/no-cost-data predefinedRoles/no-revenue-data For users, if an empty list of roles is set, this AccessBinding will be deleted. |
| `user` | String |  | If set, the email address of the user to set roles for. Format: "someuser@gmail.com" |
| `parent` | String | ✅ | Required. Formats: - accounts/{account} - properties/{property} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of this binding. Format: accounts/{account}/accessBindings/{access_binding} or properties/{property}/accessBindings/{access_binding} Example: "accounts/100/accessBindings/200" |
| `roles` | Vec<String> | A list of roles for to grant to the parent resource. Valid values: predefinedRoles/viewer predefinedRoles/analyst predefinedRoles/editor predefinedRoles/admin predefinedRoles/no-cost-data predefinedRoles/no-revenue-data For users, if an empty list of roles is set, this AccessBinding will be deleted. |
| `user` | String | If set, the email address of the user to set roles for. Format: "someuser@gmail.com" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create access_binding
access_binding = provider.analyticsadmin_api.Access_binding {
    parent = "value"  # Required. Formats: - accounts/{account} - properties/{property}
}

# Access access_binding outputs
access_binding_id = access_binding.id
access_binding_name = access_binding.name
access_binding_roles = access_binding.roles
access_binding_user = access_binding.user
```

---


### Calculated_metric

Creates a CalculatedMetric.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name for this CalculatedMetric. Format: 'properties/{property_id}/calculatedMetrics/{calculated_metric_id}' |
| `description` | String |  | Optional. Description for this calculated metric. Max length of 4096 characters. |
| `calculated_metric_id` | String |  | Output only. The ID to use for the calculated metric. In the UI, this is referred to as the "API name." The calculated_metric_id is used when referencing this calculated metric from external APIs. For example, "calcMetric:{calculated_metric_id}". |
| `formula` | String |  | Required. The calculated metric's definition. Maximum number of unique referenced custom metrics is 5. Formulas supports the following operations: + (addition), - (subtraction), - (negative), * (multiplication), / (division), () (parenthesis). Any valid real numbers are acceptable that fit in a Long (64bit integer) or a Double (64 bit floating point number). Example formula: "( customEvent:parameter_name + cartPurchaseQuantity ) / 2.0" |
| `restricted_metric_type` | Vec<String> |  | Output only. Types of restricted data that this metric contains. |
| `invalid_metric_reference` | bool |  | Output only. If true, this calculated metric has a invalid metric reference. Anything using a calculated metric with invalid_metric_reference set to true may fail, produce warnings, or produce unexpected results. |
| `display_name` | String |  | Required. Display name for this calculated metric as shown in the Google Analytics UI. Max length 82 characters. |
| `metric_unit` | String |  | Required. The type for the calculated metric's value. |
| `parent` | String | ✅ | Required. Format: properties/{property_id} Example: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name for this CalculatedMetric. Format: 'properties/{property_id}/calculatedMetrics/{calculated_metric_id}' |
| `description` | String | Optional. Description for this calculated metric. Max length of 4096 characters. |
| `calculated_metric_id` | String | Output only. The ID to use for the calculated metric. In the UI, this is referred to as the "API name." The calculated_metric_id is used when referencing this calculated metric from external APIs. For example, "calcMetric:{calculated_metric_id}". |
| `formula` | String | Required. The calculated metric's definition. Maximum number of unique referenced custom metrics is 5. Formulas supports the following operations: + (addition), - (subtraction), - (negative), * (multiplication), / (division), () (parenthesis). Any valid real numbers are acceptable that fit in a Long (64bit integer) or a Double (64 bit floating point number). Example formula: "( customEvent:parameter_name + cartPurchaseQuantity ) / 2.0" |
| `restricted_metric_type` | Vec<String> | Output only. Types of restricted data that this metric contains. |
| `invalid_metric_reference` | bool | Output only. If true, this calculated metric has a invalid metric reference. Anything using a calculated metric with invalid_metric_reference set to true may fail, produce warnings, or produce unexpected results. |
| `display_name` | String | Required. Display name for this calculated metric as shown in the Google Analytics UI. Max length 82 characters. |
| `metric_unit` | String | Required. The type for the calculated metric's value. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create calculated_metric
calculated_metric = provider.analyticsadmin_api.Calculated_metric {
    parent = "value"  # Required. Format: properties/{property_id} Example: properties/1234
}

# Access calculated_metric outputs
calculated_metric_id = calculated_metric.id
calculated_metric_name = calculated_metric.name
calculated_metric_description = calculated_metric.description
calculated_metric_calculated_metric_id = calculated_metric.calculated_metric_id
calculated_metric_formula = calculated_metric.formula
calculated_metric_restricted_metric_type = calculated_metric.restricted_metric_type
calculated_metric_invalid_metric_reference = calculated_metric.invalid_metric_reference
calculated_metric_display_name = calculated_metric.display_name
calculated_metric_metric_unit = calculated_metric.metric_unit
```

---


### Conversion_event

Deprecated: Use `CreateKeyEvent` instead. Creates a conversion event with the specified attributes.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of this conversion event. Format: properties/{property}/conversionEvents/{conversion_event} |
| `event_name` | String |  | Immutable. The event name for this conversion event. Examples: 'click', 'purchase' |
| `create_time` | String |  | Output only. Time when this conversion event was created in the property. |
| `default_conversion_value` | String |  | Optional. Defines a default value/currency for a conversion event. |
| `custom` | bool |  | Output only. If set to true, this conversion event refers to a custom event. If set to false, this conversion event refers to a default event in GA. Default events typically have special meaning in GA. Default events are usually created for you by the GA system, but in some cases can be created by property admins. Custom events count towards the maximum number of custom conversion events that may be created per property. |
| `counting_method` | String |  | Optional. The method by which conversions will be counted across multiple events within a session. If this value is not provided, it will be set to `ONCE_PER_EVENT`. |
| `deletable` | bool |  | Output only. If set, this event can currently be deleted with DeleteConversionEvent. |
| `parent` | String | ✅ | Required. The resource name of the parent property where this conversion event will be created. Format: properties/123 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of this conversion event. Format: properties/{property}/conversionEvents/{conversion_event} |
| `event_name` | String | Immutable. The event name for this conversion event. Examples: 'click', 'purchase' |
| `create_time` | String | Output only. Time when this conversion event was created in the property. |
| `default_conversion_value` | String | Optional. Defines a default value/currency for a conversion event. |
| `custom` | bool | Output only. If set to true, this conversion event refers to a custom event. If set to false, this conversion event refers to a default event in GA. Default events typically have special meaning in GA. Default events are usually created for you by the GA system, but in some cases can be created by property admins. Custom events count towards the maximum number of custom conversion events that may be created per property. |
| `counting_method` | String | Optional. The method by which conversions will be counted across multiple events within a session. If this value is not provided, it will be set to `ONCE_PER_EVENT`. |
| `deletable` | bool | Output only. If set, this event can currently be deleted with DeleteConversionEvent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversion_event
conversion_event = provider.analyticsadmin_api.Conversion_event {
    parent = "value"  # Required. The resource name of the parent property where this conversion event will be created. Format: properties/123
}

# Access conversion_event outputs
conversion_event_id = conversion_event.id
conversion_event_name = conversion_event.name
conversion_event_event_name = conversion_event.event_name
conversion_event_create_time = conversion_event.create_time
conversion_event_default_conversion_value = conversion_event.default_conversion_value
conversion_event_custom = conversion_event.custom
conversion_event_counting_method = conversion_event.counting_method
conversion_event_deletable = conversion_event.deletable
```

---


### Account_summarie

Returns summaries of all accounts accessible by the caller.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_summaries` | Vec<String> | Account summaries of all accounts the caller has access to. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account_summarie outputs
account_summarie_id = account_summarie.id
account_summarie_account_summaries = account_summarie.account_summaries
account_summarie_next_page_token = account_summarie.next_page_token
```

---


### Account

Searches through all changes to an account or its children given the specified set of filters. Only returns the subset of changes supported by the API. The UI may return additional changes.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_size` | i64 |  | Optional. The maximum number of ChangeHistoryEvent items to return. If unspecified, at most 50 items will be returned. The maximum value is 200 (higher values will be coerced to the maximum). Note that the service may return a page with fewer items than this value specifies (potentially even zero), and that there still may be additional pages. If you want a particular number of items, you'll need to continue requesting additional pages using `page_token` until you get the needed number. |
| `earliest_change_time` | String |  | Optional. If set, only return changes made after this time (inclusive). |
| `actor_email` | Vec<String> |  | Optional. If set, only return changes if they are made by a user in this list. |
| `action` | Vec<String> |  | Optional. If set, only return changes that match one or more of these types of actions. |
| `resource_type` | Vec<String> |  | Optional. If set, only return changes if they are for a resource that matches at least one of these types. |
| `page_token` | String |  | Optional. A page token, received from a previous `SearchChangeHistoryEvents` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `SearchChangeHistoryEvents` must match the call that provided the page token. |
| `latest_change_time` | String |  | Optional. If set, only return changes made before this time (inclusive). |
| `property` | String |  | Optional. Resource name for a child property. If set, only return changes made to this property or its child resources. Format: properties/{propertyId} Example: `properties/100` |
| `account` | String | ✅ | Required. The account resource for which to return change history resources. Format: accounts/{account} Example: `accounts/100` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time when this account was originally created. |
| `display_name` | String | Required. Human-readable display name for this account. |
| `update_time` | String | Output only. Time when account payload fields were last updated. |
| `region_code` | String | Country of business. Must be a Unicode CLDR region code. |
| `name` | String | Output only. Resource name of this account. Format: accounts/{account} Example: "accounts/100" |
| `deleted` | bool | Output only. Indicates whether this Account is soft-deleted or not. Deleted accounts are excluded from List results unless specifically requested. |
| `gmp_organization` | String | Output only. The URI for a Google Marketing Platform organization resource. Only set when this account is connected to a GMP organization. Format: marketingplatformadmin.googleapis.com/organizations/{org_id} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.analyticsadmin_api.Account {
    account = "value"  # Required. The account resource for which to return change history resources. Format: accounts/{account} Example: `accounts/100`
}

# Access account outputs
account_id = account.id
account_create_time = account.create_time
account_display_name = account.display_name
account_update_time = account.update_time
account_region_code = account.region_code
account_name = account.name
account_deleted = account.deleted
account_gmp_organization = account.gmp_organization
```

---


### Sk_ad_network_conversion_value_schema

Creates a SKAdNetworkConversionValueSchema.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `postback_window_one` | String |  | Required. The conversion value settings for the first postback window. These differ from values for postback window two and three in that they contain a "Fine" grained conversion value (a numeric value). Conversion values for this postback window must be set. The other windows are optional and may inherit this window's settings if unset or disabled. |
| `postback_window_three` | String |  | The conversion value settings for the third postback window. This field should only be set if the user chose to define different conversion values for this postback window. It is allowed to configure window 3 without setting window 2. In case window 1 & 2 settings are set and enable_postback_window_settings for this postback window is set to false, the schema will inherit settings from postback_window_two. |
| `apply_conversion_values` | bool |  | If enabled, the GA SDK will set conversion values using this schema definition, and schema will be exported to any Google Ads accounts linked to this property. If disabled, the GA SDK will not automatically set conversion values, and also the schema will not be exported to Ads. |
| `postback_window_two` | String |  | The conversion value settings for the second postback window. This field should only be configured if there is a need to define different conversion values for this postback window. If enable_postback_window_settings is set to false for this postback window, the values from postback_window_one will be used. |
| `name` | String |  | Output only. Resource name of the schema. This will be child of ONLY an iOS stream, and there can be at most one such child under an iOS stream. Format: properties/{property}/dataStreams/{dataStream}/sKAdNetworkConversionValueSchema |
| `parent` | String | ✅ | Required. The parent resource where this schema will be created. Format: properties/{property}/dataStreams/{dataStream} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `postback_window_one` | String | Required. The conversion value settings for the first postback window. These differ from values for postback window two and three in that they contain a "Fine" grained conversion value (a numeric value). Conversion values for this postback window must be set. The other windows are optional and may inherit this window's settings if unset or disabled. |
| `postback_window_three` | String | The conversion value settings for the third postback window. This field should only be set if the user chose to define different conversion values for this postback window. It is allowed to configure window 3 without setting window 2. In case window 1 & 2 settings are set and enable_postback_window_settings for this postback window is set to false, the schema will inherit settings from postback_window_two. |
| `apply_conversion_values` | bool | If enabled, the GA SDK will set conversion values using this schema definition, and schema will be exported to any Google Ads accounts linked to this property. If disabled, the GA SDK will not automatically set conversion values, and also the schema will not be exported to Ads. |
| `postback_window_two` | String | The conversion value settings for the second postback window. This field should only be configured if there is a need to define different conversion values for this postback window. If enable_postback_window_settings is set to false for this postback window, the values from postback_window_one will be used. |
| `name` | String | Output only. Resource name of the schema. This will be child of ONLY an iOS stream, and there can be at most one such child under an iOS stream. Format: properties/{property}/dataStreams/{dataStream}/sKAdNetworkConversionValueSchema |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sk_ad_network_conversion_value_schema
sk_ad_network_conversion_value_schema = provider.analyticsadmin_api.Sk_ad_network_conversion_value_schema {
    parent = "value"  # Required. The parent resource where this schema will be created. Format: properties/{property}/dataStreams/{dataStream}
}

# Access sk_ad_network_conversion_value_schema outputs
sk_ad_network_conversion_value_schema_id = sk_ad_network_conversion_value_schema.id
sk_ad_network_conversion_value_schema_postback_window_one = sk_ad_network_conversion_value_schema.postback_window_one
sk_ad_network_conversion_value_schema_postback_window_three = sk_ad_network_conversion_value_schema.postback_window_three
sk_ad_network_conversion_value_schema_apply_conversion_values = sk_ad_network_conversion_value_schema.apply_conversion_values
sk_ad_network_conversion_value_schema_postback_window_two = sk_ad_network_conversion_value_schema.postback_window_two
sk_ad_network_conversion_value_schema_name = sk_ad_network_conversion_value_schema.name
```

---


### Search_ads360_link

Creates a SearchAds360Link.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cost_data_sharing_enabled` | bool |  | Immutable. Enables the import of cost data from Search Ads 360 to the Google Analytics property. This can only be enabled if campaign_data_sharing_enabled is enabled. After link creation, this can only be updated from the Search Ads 360 product. If this field is not set on create, it will be defaulted to true. |
| `ads_personalization_enabled` | bool |  | Enables personalized advertising features with this integration. If this field is not set on create, it will be defaulted to true. |
| `site_stats_sharing_enabled` | bool |  | Enables export of site stats with this integration. If this field is not set on create, it will be defaulted to true. |
| `campaign_data_sharing_enabled` | bool |  | Immutable. Enables the import of campaign data from Search Ads 360 into the Google Analytics property. After link creation, this can only be updated from the Search Ads 360 product. If this field is not set on create, it will be defaulted to true. |
| `name` | String |  | Output only. The resource name for this SearchAds360Link resource. Format: properties/{propertyId}/searchAds360Links/{linkId} Note: linkId is not the Search Ads 360 advertiser ID |
| `advertiser_id` | String |  | Immutable. This field represents the Advertiser ID of the Search Ads 360 Advertiser. that has been linked. |
| `advertiser_display_name` | String |  | Output only. The display name of the Search Ads 360 Advertiser. Allows users to easily identify the linked resource. |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cost_data_sharing_enabled` | bool | Immutable. Enables the import of cost data from Search Ads 360 to the Google Analytics property. This can only be enabled if campaign_data_sharing_enabled is enabled. After link creation, this can only be updated from the Search Ads 360 product. If this field is not set on create, it will be defaulted to true. |
| `ads_personalization_enabled` | bool | Enables personalized advertising features with this integration. If this field is not set on create, it will be defaulted to true. |
| `site_stats_sharing_enabled` | bool | Enables export of site stats with this integration. If this field is not set on create, it will be defaulted to true. |
| `campaign_data_sharing_enabled` | bool | Immutable. Enables the import of campaign data from Search Ads 360 into the Google Analytics property. After link creation, this can only be updated from the Search Ads 360 product. If this field is not set on create, it will be defaulted to true. |
| `name` | String | Output only. The resource name for this SearchAds360Link resource. Format: properties/{propertyId}/searchAds360Links/{linkId} Note: linkId is not the Search Ads 360 advertiser ID |
| `advertiser_id` | String | Immutable. This field represents the Advertiser ID of the Search Ads 360 Advertiser. that has been linked. |
| `advertiser_display_name` | String | Output only. The display name of the Search Ads 360 Advertiser. Allows users to easily identify the linked resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create search_ads360_link
search_ads360_link = provider.analyticsadmin_api.Search_ads360_link {
    parent = "value"  # Required. Example format: properties/1234
}

# Access search_ads360_link outputs
search_ads360_link_id = search_ads360_link.id
search_ads360_link_cost_data_sharing_enabled = search_ads360_link.cost_data_sharing_enabled
search_ads360_link_ads_personalization_enabled = search_ads360_link.ads_personalization_enabled
search_ads360_link_site_stats_sharing_enabled = search_ads360_link.site_stats_sharing_enabled
search_ads360_link_campaign_data_sharing_enabled = search_ads360_link.campaign_data_sharing_enabled
search_ads360_link_name = search_ads360_link.name
search_ads360_link_advertiser_id = search_ads360_link.advertiser_id
search_ads360_link_advertiser_display_name = search_ads360_link.advertiser_display_name
```

---


### Display_video360_advertiser_link

Creates a DisplayVideo360AdvertiserLink. This can only be utilized by users who have proper authorization both on the Google Analytics property and on the Display & Video 360 advertiser. Users who do not have access to the Display & Video 360 advertiser should instead seek to create a DisplayVideo360LinkProposal.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `advertiser_display_name` | String |  | Output only. The display name of the Display & Video 360 Advertiser. |
| `ads_personalization_enabled` | bool |  | Enables personalized advertising features with this integration. If this field is not set on create/update, it will be defaulted to true. |
| `campaign_data_sharing_enabled` | bool |  | Immutable. Enables the import of campaign data from Display & Video 360 into the Google Analytics property. After link creation, this can only be updated from the Display & Video 360 product. If this field is not set on create, it will be defaulted to true. |
| `cost_data_sharing_enabled` | bool |  | Immutable. Enables the import of cost data from Display & Video 360 into the Google Analytics property. This can only be enabled if `campaign_data_sharing_enabled` is true. After link creation, this can only be updated from the Display & Video 360 product. If this field is not set on create, it will be defaulted to true. |
| `advertiser_id` | String |  | Immutable. The Display & Video 360 Advertiser's advertiser ID. |
| `name` | String |  | Output only. The resource name for this DisplayVideo360AdvertiserLink resource. Format: properties/{propertyId}/displayVideo360AdvertiserLinks/{linkId} Note: linkId is not the Display & Video 360 Advertiser ID |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `advertiser_display_name` | String | Output only. The display name of the Display & Video 360 Advertiser. |
| `ads_personalization_enabled` | bool | Enables personalized advertising features with this integration. If this field is not set on create/update, it will be defaulted to true. |
| `campaign_data_sharing_enabled` | bool | Immutable. Enables the import of campaign data from Display & Video 360 into the Google Analytics property. After link creation, this can only be updated from the Display & Video 360 product. If this field is not set on create, it will be defaulted to true. |
| `cost_data_sharing_enabled` | bool | Immutable. Enables the import of cost data from Display & Video 360 into the Google Analytics property. This can only be enabled if `campaign_data_sharing_enabled` is true. After link creation, this can only be updated from the Display & Video 360 product. If this field is not set on create, it will be defaulted to true. |
| `advertiser_id` | String | Immutable. The Display & Video 360 Advertiser's advertiser ID. |
| `name` | String | Output only. The resource name for this DisplayVideo360AdvertiserLink resource. Format: properties/{propertyId}/displayVideo360AdvertiserLinks/{linkId} Note: linkId is not the Display & Video 360 Advertiser ID |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create display_video360_advertiser_link
display_video360_advertiser_link = provider.analyticsadmin_api.Display_video360_advertiser_link {
    parent = "value"  # Required. Example format: properties/1234
}

# Access display_video360_advertiser_link outputs
display_video360_advertiser_link_id = display_video360_advertiser_link.id
display_video360_advertiser_link_advertiser_display_name = display_video360_advertiser_link.advertiser_display_name
display_video360_advertiser_link_ads_personalization_enabled = display_video360_advertiser_link.ads_personalization_enabled
display_video360_advertiser_link_campaign_data_sharing_enabled = display_video360_advertiser_link.campaign_data_sharing_enabled
display_video360_advertiser_link_cost_data_sharing_enabled = display_video360_advertiser_link.cost_data_sharing_enabled
display_video360_advertiser_link_advertiser_id = display_video360_advertiser_link.advertiser_id
display_video360_advertiser_link_name = display_video360_advertiser_link.name
```

---


### Custom_dimension

Creates a CustomDimension.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. Display name for this custom dimension as shown in the Analytics UI. Max length of 82 characters, alphanumeric plus space and underscore starting with a letter. Legacy system-generated display names may contain square brackets, but updates to this field will never permit square brackets. |
| `parameter_name` | String |  | Required. Immutable. Tagging parameter name for this custom dimension. If this is a user-scoped dimension, then this is the user property name. If this is an event-scoped dimension, then this is the event parameter name. If this is an item-scoped dimension, then this is the parameter name found in the eCommerce items array. May only contain alphanumeric and underscore characters, starting with a letter. Max length of 24 characters for user-scoped dimensions, 40 characters for event-scoped dimensions. |
| `description` | String |  | Optional. Description for this custom dimension. Max length of 150 characters. |
| `name` | String |  | Output only. Resource name for this CustomDimension resource. Format: properties/{property}/customDimensions/{customDimension} |
| `disallow_ads_personalization` | bool |  | Optional. If set to true, sets this dimension as NPA and excludes it from ads personalization. This is currently only supported by user-scoped custom dimensions. |
| `scope` | String |  | Required. Immutable. The scope of this dimension. |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. Display name for this custom dimension as shown in the Analytics UI. Max length of 82 characters, alphanumeric plus space and underscore starting with a letter. Legacy system-generated display names may contain square brackets, but updates to this field will never permit square brackets. |
| `parameter_name` | String | Required. Immutable. Tagging parameter name for this custom dimension. If this is a user-scoped dimension, then this is the user property name. If this is an event-scoped dimension, then this is the event parameter name. If this is an item-scoped dimension, then this is the parameter name found in the eCommerce items array. May only contain alphanumeric and underscore characters, starting with a letter. Max length of 24 characters for user-scoped dimensions, 40 characters for event-scoped dimensions. |
| `description` | String | Optional. Description for this custom dimension. Max length of 150 characters. |
| `name` | String | Output only. Resource name for this CustomDimension resource. Format: properties/{property}/customDimensions/{customDimension} |
| `disallow_ads_personalization` | bool | Optional. If set to true, sets this dimension as NPA and excludes it from ads personalization. This is currently only supported by user-scoped custom dimensions. |
| `scope` | String | Required. Immutable. The scope of this dimension. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_dimension
custom_dimension = provider.analyticsadmin_api.Custom_dimension {
    parent = "value"  # Required. Example format: properties/1234
}

# Access custom_dimension outputs
custom_dimension_id = custom_dimension.id
custom_dimension_display_name = custom_dimension.display_name
custom_dimension_parameter_name = custom_dimension.parameter_name
custom_dimension_description = custom_dimension.description
custom_dimension_name = custom_dimension.name
custom_dimension_disallow_ads_personalization = custom_dimension.disallow_ads_personalization
custom_dimension_scope = custom_dimension.scope
```

---


### Firebase_link

Creates a FirebaseLink. Properties can have at most one FirebaseLink.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project` | String |  | Immutable. Firebase project resource name. When creating a FirebaseLink, you may provide this resource name using either a project number or project ID. Once this resource has been created, returned FirebaseLinks will always have a project_name that contains a project number. Format: 'projects/{project number}' Example: 'projects/1234' |
| `name` | String |  | Output only. Example format: properties/1234/firebaseLinks/5678 |
| `create_time` | String |  | Output only. Time when this FirebaseLink was originally created. |
| `parent` | String | ✅ | Required. Format: properties/{property_id} Example: `properties/1234` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. Currently, Google Analytics supports only one FirebaseLink per property, so this will never be populated. |
| `firebase_links` | Vec<String> | List of FirebaseLinks. This will have at most one value. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create firebase_link
firebase_link = provider.analyticsadmin_api.Firebase_link {
    parent = "value"  # Required. Format: properties/{property_id} Example: `properties/1234`
}

# Access firebase_link outputs
firebase_link_id = firebase_link.id
firebase_link_next_page_token = firebase_link.next_page_token
firebase_link_firebase_links = firebase_link.firebase_links
```

---


### Rollup_property_source_link

Creates a roll-up property source link. Only roll-up properties can have source links, so this method will throw an error if used on other types of properties.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of this RollupPropertySourceLink. Format: 'properties/{property_id}/rollupPropertySourceLinks/{rollup_property_source_link}' Format: 'properties/123/rollupPropertySourceLinks/456' |
| `source_property` | String |  | Immutable. Resource name of the source property. Format: properties/{property_id} Example: "properties/789" |
| `parent` | String | ✅ | Required. Format: properties/{property_id} Example: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of this RollupPropertySourceLink. Format: 'properties/{property_id}/rollupPropertySourceLinks/{rollup_property_source_link}' Format: 'properties/123/rollupPropertySourceLinks/456' |
| `source_property` | String | Immutable. Resource name of the source property. Format: properties/{property_id} Example: "properties/789" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rollup_property_source_link
rollup_property_source_link = provider.analyticsadmin_api.Rollup_property_source_link {
    parent = "value"  # Required. Format: properties/{property_id} Example: properties/1234
}

# Access rollup_property_source_link outputs
rollup_property_source_link_id = rollup_property_source_link.id
rollup_property_source_link_name = rollup_property_source_link.name
rollup_property_source_link_source_property = rollup_property_source_link.source_property
```

---


### Subproperty_event_filter

Creates a subproperty Event Filter.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `apply_to_property` | String |  | Immutable. Resource name of the Subproperty that uses this filter. |
| `filter_clauses` | Vec<String> |  | Required. Unordered list. Filter clauses that define the SubpropertyEventFilter. All clauses are AND'ed together to determine what data is sent to the subproperty. |
| `name` | String |  | Output only. Format: properties/{ordinary_property_id}/subpropertyEventFilters/{sub_property_event_filter} Example: properties/1234/subpropertyEventFilters/5678 |
| `parent` | String | ✅ | Required. The ordinary property for which to create a subproperty event filter. Format: properties/property_id Example: properties/123 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `apply_to_property` | String | Immutable. Resource name of the Subproperty that uses this filter. |
| `filter_clauses` | Vec<String> | Required. Unordered list. Filter clauses that define the SubpropertyEventFilter. All clauses are AND'ed together to determine what data is sent to the subproperty. |
| `name` | String | Output only. Format: properties/{ordinary_property_id}/subpropertyEventFilters/{sub_property_event_filter} Example: properties/1234/subpropertyEventFilters/5678 |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subproperty_event_filter
subproperty_event_filter = provider.analyticsadmin_api.Subproperty_event_filter {
    parent = "value"  # Required. The ordinary property for which to create a subproperty event filter. Format: properties/property_id Example: properties/123
}

# Access subproperty_event_filter outputs
subproperty_event_filter_id = subproperty_event_filter.id
subproperty_event_filter_apply_to_property = subproperty_event_filter.apply_to_property
subproperty_event_filter_filter_clauses = subproperty_event_filter.filter_clauses
subproperty_event_filter_name = subproperty_event_filter.name
```

---


### Display_video360_advertiser_link_proposal

Creates a DisplayVideo360AdvertiserLinkProposal.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `advertiser_display_name` | String |  | Output only. The display name of the Display & Video Advertiser. Only populated for proposals that originated from Display & Video 360. |
| `advertiser_id` | String |  | Immutable. The Display & Video 360 Advertiser's advertiser ID. |
| `validation_email` | String |  | Input only. On a proposal being sent to Display & Video 360, this field must be set to the email address of an admin on the target advertiser. This is used to verify that the Google Analytics admin is aware of at least one admin on the Display & Video 360 Advertiser. This does not restrict approval of the proposal to a single user. Any admin on the Display & Video 360 Advertiser may approve the proposal. |
| `cost_data_sharing_enabled` | bool |  | Immutable. Enables the import of cost data from Display & Video 360. This can only be enabled if campaign_data_sharing_enabled is enabled. If this field is not set on create, it will be defaulted to true. |
| `ads_personalization_enabled` | bool |  | Immutable. Enables personalized advertising features with this integration. If this field is not set on create, it will be defaulted to true. |
| `link_proposal_status_details` | String |  | Output only. The status information for this link proposal. |
| `name` | String |  | Output only. The resource name for this DisplayVideo360AdvertiserLinkProposal resource. Format: properties/{propertyId}/displayVideo360AdvertiserLinkProposals/{proposalId} Note: proposalId is not the Display & Video 360 Advertiser ID |
| `campaign_data_sharing_enabled` | bool |  | Immutable. Enables the import of campaign data from Display & Video 360. If this field is not set on create, it will be defaulted to true. |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `advertiser_display_name` | String | Output only. The display name of the Display & Video Advertiser. Only populated for proposals that originated from Display & Video 360. |
| `advertiser_id` | String | Immutable. The Display & Video 360 Advertiser's advertiser ID. |
| `validation_email` | String | Input only. On a proposal being sent to Display & Video 360, this field must be set to the email address of an admin on the target advertiser. This is used to verify that the Google Analytics admin is aware of at least one admin on the Display & Video 360 Advertiser. This does not restrict approval of the proposal to a single user. Any admin on the Display & Video 360 Advertiser may approve the proposal. |
| `cost_data_sharing_enabled` | bool | Immutable. Enables the import of cost data from Display & Video 360. This can only be enabled if campaign_data_sharing_enabled is enabled. If this field is not set on create, it will be defaulted to true. |
| `ads_personalization_enabled` | bool | Immutable. Enables personalized advertising features with this integration. If this field is not set on create, it will be defaulted to true. |
| `link_proposal_status_details` | String | Output only. The status information for this link proposal. |
| `name` | String | Output only. The resource name for this DisplayVideo360AdvertiserLinkProposal resource. Format: properties/{propertyId}/displayVideo360AdvertiserLinkProposals/{proposalId} Note: proposalId is not the Display & Video 360 Advertiser ID |
| `campaign_data_sharing_enabled` | bool | Immutable. Enables the import of campaign data from Display & Video 360. If this field is not set on create, it will be defaulted to true. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create display_video360_advertiser_link_proposal
display_video360_advertiser_link_proposal = provider.analyticsadmin_api.Display_video360_advertiser_link_proposal {
    parent = "value"  # Required. Example format: properties/1234
}

# Access display_video360_advertiser_link_proposal outputs
display_video360_advertiser_link_proposal_id = display_video360_advertiser_link_proposal.id
display_video360_advertiser_link_proposal_advertiser_display_name = display_video360_advertiser_link_proposal.advertiser_display_name
display_video360_advertiser_link_proposal_advertiser_id = display_video360_advertiser_link_proposal.advertiser_id
display_video360_advertiser_link_proposal_validation_email = display_video360_advertiser_link_proposal.validation_email
display_video360_advertiser_link_proposal_cost_data_sharing_enabled = display_video360_advertiser_link_proposal.cost_data_sharing_enabled
display_video360_advertiser_link_proposal_ads_personalization_enabled = display_video360_advertiser_link_proposal.ads_personalization_enabled
display_video360_advertiser_link_proposal_link_proposal_status_details = display_video360_advertiser_link_proposal.link_proposal_status_details
display_video360_advertiser_link_proposal_name = display_video360_advertiser_link_proposal.name
display_video360_advertiser_link_proposal_campaign_data_sharing_enabled = display_video360_advertiser_link_proposal.campaign_data_sharing_enabled
```

---


### Audience

Creates an Audience.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ads_personalization_enabled` | bool |  | Output only. It is automatically set by GA to false if this is an NPA Audience and is excluded from ads personalization. |
| `display_name` | String |  | Required. The display name of the Audience. |
| `filter_clauses` | Vec<String> |  | Required. Immutable. Unordered list. Filter clauses that define the Audience. All clauses will be AND’ed together. |
| `membership_duration_days` | i64 |  | Required. Immutable. The duration a user should stay in an Audience. It cannot be set to more than 540 days. |
| `event_trigger` | String |  | Optional. Specifies an event to log when a user joins the Audience. If not set, no event is logged when a user joins the Audience. |
| `description` | String |  | Required. The description of the Audience. |
| `exclusion_duration_mode` | String |  | Immutable. Specifies how long an exclusion lasts for users that meet the exclusion filter. It is applied to all EXCLUDE filter clauses and is ignored when there is no EXCLUDE filter clause in the Audience. |
| `name` | String |  | Output only. The resource name for this Audience resource. Format: properties/{propertyId}/audiences/{audienceId} |
| `create_time` | String |  | Output only. Time when the Audience was created. |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ads_personalization_enabled` | bool | Output only. It is automatically set by GA to false if this is an NPA Audience and is excluded from ads personalization. |
| `display_name` | String | Required. The display name of the Audience. |
| `filter_clauses` | Vec<String> | Required. Immutable. Unordered list. Filter clauses that define the Audience. All clauses will be AND’ed together. |
| `membership_duration_days` | i64 | Required. Immutable. The duration a user should stay in an Audience. It cannot be set to more than 540 days. |
| `event_trigger` | String | Optional. Specifies an event to log when a user joins the Audience. If not set, no event is logged when a user joins the Audience. |
| `description` | String | Required. The description of the Audience. |
| `exclusion_duration_mode` | String | Immutable. Specifies how long an exclusion lasts for users that meet the exclusion filter. It is applied to all EXCLUDE filter clauses and is ignored when there is no EXCLUDE filter clause in the Audience. |
| `name` | String | Output only. The resource name for this Audience resource. Format: properties/{propertyId}/audiences/{audienceId} |
| `create_time` | String | Output only. Time when the Audience was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create audience
audience = provider.analyticsadmin_api.Audience {
    parent = "value"  # Required. Example format: properties/1234
}

# Access audience outputs
audience_id = audience.id
audience_ads_personalization_enabled = audience.ads_personalization_enabled
audience_display_name = audience.display_name
audience_filter_clauses = audience.filter_clauses
audience_membership_duration_days = audience.membership_duration_days
audience_event_trigger = audience.event_trigger
audience_description = audience.description
audience_exclusion_duration_mode = audience.exclusion_duration_mode
audience_name = audience.name
audience_create_time = audience.create_time
```

---


### Measurement_protocol_secret

Creates a measurement protocol secret.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `secret_value` | String |  | Output only. The measurement protocol secret value. Pass this value to the api_secret field of the Measurement Protocol API when sending hits to this secret's parent property. |
| `display_name` | String |  | Required. Human-readable display name for this secret. |
| `name` | String |  | Output only. Resource name of this secret. This secret may be a child of any type of stream. Format: properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret} |
| `parent` | String | ✅ | Required. The parent resource where this secret will be created. Format: properties/{property}/dataStreams/{dataStream} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `secret_value` | String | Output only. The measurement protocol secret value. Pass this value to the api_secret field of the Measurement Protocol API when sending hits to this secret's parent property. |
| `display_name` | String | Required. Human-readable display name for this secret. |
| `name` | String | Output only. Resource name of this secret. This secret may be a child of any type of stream. Format: properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create measurement_protocol_secret
measurement_protocol_secret = provider.analyticsadmin_api.Measurement_protocol_secret {
    parent = "value"  # Required. The parent resource where this secret will be created. Format: properties/{property}/dataStreams/{dataStream}
}

# Access measurement_protocol_secret outputs
measurement_protocol_secret_id = measurement_protocol_secret.id
measurement_protocol_secret_secret_value = measurement_protocol_secret.secret_value
measurement_protocol_secret_display_name = measurement_protocol_secret.display_name
measurement_protocol_secret_name = measurement_protocol_secret.name
```

---


### Account

Requests a ticket for creating an account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `redirect_uri` | String |  | Redirect URI where the user will be sent after accepting Terms of Service. Must be configured in Cloud Console as a Redirect URI. |
| `account` | String |  | The account to create. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gmp_organization` | String | Output only. The URI for a Google Marketing Platform organization resource. Only set when this account is connected to a GMP organization. Format: marketingplatformadmin.googleapis.com/organizations/{org_id} |
| `name` | String | Output only. Resource name of this account. Format: accounts/{account} Example: "accounts/100" |
| `update_time` | String | Output only. Time when account payload fields were last updated. |
| `region_code` | String | Country of business. Must be a Unicode CLDR region code. |
| `deleted` | bool | Output only. Indicates whether this Account is soft-deleted or not. Deleted accounts are excluded from List results unless specifically requested. |
| `display_name` | String | Required. Human-readable display name for this account. |
| `create_time` | String | Output only. Time when this account was originally created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.analyticsadmin_api.Account {
}

# Access account outputs
account_id = account.id
account_gmp_organization = account.gmp_organization
account_name = account.name
account_update_time = account.update_time
account_region_code = account.region_code
account_deleted = account.deleted
account_display_name = account.display_name
account_create_time = account.create_time
```

---


### Custom_dimension

Creates a CustomDimension.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disallow_ads_personalization` | bool |  | Optional. If set to true, sets this dimension as NPA and excludes it from ads personalization. This is currently only supported by user-scoped custom dimensions. |
| `scope` | String |  | Required. Immutable. The scope of this dimension. |
| `name` | String |  | Output only. Resource name for this CustomDimension resource. Format: properties/{property}/customDimensions/{customDimension} |
| `parameter_name` | String |  | Required. Immutable. Tagging parameter name for this custom dimension. If this is a user-scoped dimension, then this is the user property name. If this is an event-scoped dimension, then this is the event parameter name. If this is an item-scoped dimension, then this is the parameter name found in the eCommerce items array. May only contain alphanumeric and underscore characters, starting with a letter. Max length of 24 characters for user-scoped dimensions, 40 characters for event-scoped dimensions. |
| `display_name` | String |  | Required. Display name for this custom dimension as shown in the Analytics UI. Max length of 82 characters, alphanumeric plus space and underscore starting with a letter. Legacy system-generated display names may contain square brackets, but updates to this field will never permit square brackets. |
| `description` | String |  | Optional. Description for this custom dimension. Max length of 150 characters. |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disallow_ads_personalization` | bool | Optional. If set to true, sets this dimension as NPA and excludes it from ads personalization. This is currently only supported by user-scoped custom dimensions. |
| `scope` | String | Required. Immutable. The scope of this dimension. |
| `name` | String | Output only. Resource name for this CustomDimension resource. Format: properties/{property}/customDimensions/{customDimension} |
| `parameter_name` | String | Required. Immutable. Tagging parameter name for this custom dimension. If this is a user-scoped dimension, then this is the user property name. If this is an event-scoped dimension, then this is the event parameter name. If this is an item-scoped dimension, then this is the parameter name found in the eCommerce items array. May only contain alphanumeric and underscore characters, starting with a letter. Max length of 24 characters for user-scoped dimensions, 40 characters for event-scoped dimensions. |
| `display_name` | String | Required. Display name for this custom dimension as shown in the Analytics UI. Max length of 82 characters, alphanumeric plus space and underscore starting with a letter. Legacy system-generated display names may contain square brackets, but updates to this field will never permit square brackets. |
| `description` | String | Optional. Description for this custom dimension. Max length of 150 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_dimension
custom_dimension = provider.analyticsadmin_api.Custom_dimension {
    parent = "value"  # Required. Example format: properties/1234
}

# Access custom_dimension outputs
custom_dimension_id = custom_dimension.id
custom_dimension_disallow_ads_personalization = custom_dimension.disallow_ads_personalization
custom_dimension_scope = custom_dimension.scope
custom_dimension_name = custom_dimension.name
custom_dimension_parameter_name = custom_dimension.parameter_name
custom_dimension_display_name = custom_dimension.display_name
custom_dimension_description = custom_dimension.description
```

---


### Key_event

Creates a Key Event.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deletable` | bool |  | Output only. If set to true, this event can be deleted. |
| `name` | String |  | Output only. Resource name of this key event. Format: properties/{property}/keyEvents/{key_event} |
| `custom` | bool |  | Output only. If set to true, this key event refers to a custom event. If set to false, this key event refers to a default event in GA. Default events typically have special meaning in GA. Default events are usually created for you by the GA system, but in some cases can be created by property admins. Custom events count towards the maximum number of custom key events that may be created per property. |
| `event_name` | String |  | Immutable. The event name for this key event. Examples: 'click', 'purchase' |
| `create_time` | String |  | Output only. Time when this key event was created in the property. |
| `counting_method` | String |  | Required. The method by which Key Events will be counted across multiple events within a session. |
| `default_value` | String |  | Optional. Defines a default value/currency for a key event. |
| `parent` | String | ✅ | Required. The resource name of the parent property where this Key Event will be created. Format: properties/123 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deletable` | bool | Output only. If set to true, this event can be deleted. |
| `name` | String | Output only. Resource name of this key event. Format: properties/{property}/keyEvents/{key_event} |
| `custom` | bool | Output only. If set to true, this key event refers to a custom event. If set to false, this key event refers to a default event in GA. Default events typically have special meaning in GA. Default events are usually created for you by the GA system, but in some cases can be created by property admins. Custom events count towards the maximum number of custom key events that may be created per property. |
| `event_name` | String | Immutable. The event name for this key event. Examples: 'click', 'purchase' |
| `create_time` | String | Output only. Time when this key event was created in the property. |
| `counting_method` | String | Required. The method by which Key Events will be counted across multiple events within a session. |
| `default_value` | String | Optional. Defines a default value/currency for a key event. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create key_event
key_event = provider.analyticsadmin_api.Key_event {
    parent = "value"  # Required. The resource name of the parent property where this Key Event will be created. Format: properties/123
}

# Access key_event outputs
key_event_id = key_event.id
key_event_deletable = key_event.deletable
key_event_name = key_event.name
key_event_custom = key_event.custom
key_event_event_name = key_event.event_name
key_event_create_time = key_event.create_time
key_event_counting_method = key_event.counting_method
key_event_default_value = key_event.default_value
```

---


### Conversion_event

Deprecated: Use `CreateKeyEvent` instead. Creates a conversion event with the specified attributes.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of this conversion event. Format: properties/{property}/conversionEvents/{conversion_event} |
| `create_time` | String |  | Output only. Time when this conversion event was created in the property. |
| `default_conversion_value` | String |  | Optional. Defines a default value/currency for a conversion event. |
| `event_name` | String |  | Immutable. The event name for this conversion event. Examples: 'click', 'purchase' |
| `custom` | bool |  | Output only. If set to true, this conversion event refers to a custom event. If set to false, this conversion event refers to a default event in GA. Default events typically have special meaning in GA. Default events are usually created for you by the GA system, but in some cases can be created by property admins. Custom events count towards the maximum number of custom conversion events that may be created per property. |
| `deletable` | bool |  | Output only. If set, this event can currently be deleted with DeleteConversionEvent. |
| `counting_method` | String |  | Optional. The method by which conversions will be counted across multiple events within a session. If this value is not provided, it will be set to `ONCE_PER_EVENT`. |
| `parent` | String | ✅ | Required. The resource name of the parent property where this conversion event will be created. Format: properties/123 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of this conversion event. Format: properties/{property}/conversionEvents/{conversion_event} |
| `create_time` | String | Output only. Time when this conversion event was created in the property. |
| `default_conversion_value` | String | Optional. Defines a default value/currency for a conversion event. |
| `event_name` | String | Immutable. The event name for this conversion event. Examples: 'click', 'purchase' |
| `custom` | bool | Output only. If set to true, this conversion event refers to a custom event. If set to false, this conversion event refers to a default event in GA. Default events typically have special meaning in GA. Default events are usually created for you by the GA system, but in some cases can be created by property admins. Custom events count towards the maximum number of custom conversion events that may be created per property. |
| `deletable` | bool | Output only. If set, this event can currently be deleted with DeleteConversionEvent. |
| `counting_method` | String | Optional. The method by which conversions will be counted across multiple events within a session. If this value is not provided, it will be set to `ONCE_PER_EVENT`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create conversion_event
conversion_event = provider.analyticsadmin_api.Conversion_event {
    parent = "value"  # Required. The resource name of the parent property where this conversion event will be created. Format: properties/123
}

# Access conversion_event outputs
conversion_event_id = conversion_event.id
conversion_event_name = conversion_event.name
conversion_event_create_time = conversion_event.create_time
conversion_event_default_conversion_value = conversion_event.default_conversion_value
conversion_event_event_name = conversion_event.event_name
conversion_event_custom = conversion_event.custom
conversion_event_deletable = conversion_event.deletable
conversion_event_counting_method = conversion_event.counting_method
```

---


### Firebase_link

Creates a FirebaseLink. Properties can have at most one FirebaseLink.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time when this FirebaseLink was originally created. |
| `project` | String |  | Immutable. Firebase project resource name. When creating a FirebaseLink, you may provide this resource name using either a project number or project ID. Once this resource has been created, returned FirebaseLinks will always have a project_name that contains a project number. Format: 'projects/{project number}' Example: 'projects/1234' |
| `name` | String |  | Output only. Example format: properties/1234/firebaseLinks/5678 |
| `parent` | String | ✅ | Required. Format: properties/{property_id} Example: `properties/1234` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `firebase_links` | Vec<String> | List of FirebaseLinks. This will have at most one value. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. Currently, Google Analytics supports only one FirebaseLink per property, so this will never be populated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create firebase_link
firebase_link = provider.analyticsadmin_api.Firebase_link {
    parent = "value"  # Required. Format: properties/{property_id} Example: `properties/1234`
}

# Access firebase_link outputs
firebase_link_id = firebase_link.id
firebase_link_firebase_links = firebase_link.firebase_links
firebase_link_next_page_token = firebase_link.next_page_token
```

---


### Propertie

Creates a Google Analytics property with the specified location and attributes.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time when the entity was originally created. |
| `property_type` | String |  | Immutable. The property type for this Property resource. When creating a property, if the type is "PROPERTY_TYPE_UNSPECIFIED", then "ORDINARY_PROPERTY" will be implied. |
| `name` | String |  | Output only. Resource name of this property. Format: properties/{property_id} Example: "properties/1000" |
| `industry_category` | String |  | Industry associated with this property Example: AUTOMOTIVE, FOOD_AND_DRINK |
| `account` | String |  | Immutable. The resource name of the parent account Format: accounts/{account_id} Example: "accounts/123" |
| `delete_time` | String |  | Output only. If set, the time at which this property was trashed. If not set, then this property is not currently in the trash can. |
| `time_zone` | String |  | Required. Reporting Time Zone, used as the day boundary for reports, regardless of where the data originates. If the time zone honors DST, Analytics will automatically adjust for the changes. NOTE: Changing the time zone only affects data going forward, and is not applied retroactively. Format: https://www.iana.org/time-zones Example: "America/Los_Angeles" |
| `currency_code` | String |  | The currency type used in reports involving monetary values. Format: https://en.wikipedia.org/wiki/ISO_4217 Examples: "USD", "EUR", "JPY" |
| `update_time` | String |  | Output only. Time when entity payload fields were last updated. |
| `display_name` | String |  | Required. Human-readable display name for this property. The max allowed display name length is 100 UTF-16 code units. |
| `expire_time` | String |  | Output only. If set, the time at which this trashed property will be permanently deleted. If not set, then this property is not currently in the trash can and is not slated to be deleted. |
| `parent` | String |  | Immutable. Resource name of this property's logical parent. Note: The Property-Moving UI can be used to change the parent. Format: accounts/{account}, properties/{property} Example: "accounts/100", "properties/101" |
| `service_level` | String |  | Output only. The Google Analytics service level that applies to this property. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time when the entity was originally created. |
| `property_type` | String | Immutable. The property type for this Property resource. When creating a property, if the type is "PROPERTY_TYPE_UNSPECIFIED", then "ORDINARY_PROPERTY" will be implied. |
| `name` | String | Output only. Resource name of this property. Format: properties/{property_id} Example: "properties/1000" |
| `industry_category` | String | Industry associated with this property Example: AUTOMOTIVE, FOOD_AND_DRINK |
| `account` | String | Immutable. The resource name of the parent account Format: accounts/{account_id} Example: "accounts/123" |
| `delete_time` | String | Output only. If set, the time at which this property was trashed. If not set, then this property is not currently in the trash can. |
| `time_zone` | String | Required. Reporting Time Zone, used as the day boundary for reports, regardless of where the data originates. If the time zone honors DST, Analytics will automatically adjust for the changes. NOTE: Changing the time zone only affects data going forward, and is not applied retroactively. Format: https://www.iana.org/time-zones Example: "America/Los_Angeles" |
| `currency_code` | String | The currency type used in reports involving monetary values. Format: https://en.wikipedia.org/wiki/ISO_4217 Examples: "USD", "EUR", "JPY" |
| `update_time` | String | Output only. Time when entity payload fields were last updated. |
| `display_name` | String | Required. Human-readable display name for this property. The max allowed display name length is 100 UTF-16 code units. |
| `expire_time` | String | Output only. If set, the time at which this trashed property will be permanently deleted. If not set, then this property is not currently in the trash can and is not slated to be deleted. |
| `parent` | String | Immutable. Resource name of this property's logical parent. Note: The Property-Moving UI can be used to change the parent. Format: accounts/{account}, properties/{property} Example: "accounts/100", "properties/101" |
| `service_level` | String | Output only. The Google Analytics service level that applies to this property. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create propertie
propertie = provider.analyticsadmin_api.Propertie {
}

# Access propertie outputs
propertie_id = propertie.id
propertie_create_time = propertie.create_time
propertie_property_type = propertie.property_type
propertie_name = propertie.name
propertie_industry_category = propertie.industry_category
propertie_account = propertie.account
propertie_delete_time = propertie.delete_time
propertie_time_zone = propertie.time_zone
propertie_currency_code = propertie.currency_code
propertie_update_time = propertie.update_time
propertie_display_name = propertie.display_name
propertie_expire_time = propertie.expire_time
propertie_parent = propertie.parent
propertie_service_level = propertie.service_level
```

---


### Data_stream

Creates a DataStream.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Human-readable display name for the Data Stream. Required for web data streams. The max allowed display name length is 255 UTF-16 code units. |
| `update_time` | String |  | Output only. Time when stream payload fields were last updated. |
| `ios_app_stream_data` | String |  | Data specific to iOS app streams. Must be populated if type is IOS_APP_DATA_STREAM. |
| `android_app_stream_data` | String |  | Data specific to Android app streams. Must be populated if type is ANDROID_APP_DATA_STREAM. |
| `web_stream_data` | String |  | Data specific to web streams. Must be populated if type is WEB_DATA_STREAM. |
| `create_time` | String |  | Output only. Time when this stream was originally created. |
| `type` | String |  | Required. Immutable. The type of this DataStream resource. |
| `name` | String |  | Output only. Resource name of this Data Stream. Format: properties/{property_id}/dataStreams/{stream_id} Example: "properties/1000/dataStreams/2000" |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Human-readable display name for the Data Stream. Required for web data streams. The max allowed display name length is 255 UTF-16 code units. |
| `update_time` | String | Output only. Time when stream payload fields were last updated. |
| `ios_app_stream_data` | String | Data specific to iOS app streams. Must be populated if type is IOS_APP_DATA_STREAM. |
| `android_app_stream_data` | String | Data specific to Android app streams. Must be populated if type is ANDROID_APP_DATA_STREAM. |
| `web_stream_data` | String | Data specific to web streams. Must be populated if type is WEB_DATA_STREAM. |
| `create_time` | String | Output only. Time when this stream was originally created. |
| `type` | String | Required. Immutable. The type of this DataStream resource. |
| `name` | String | Output only. Resource name of this Data Stream. Format: properties/{property_id}/dataStreams/{stream_id} Example: "properties/1000/dataStreams/2000" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_stream
data_stream = provider.analyticsadmin_api.Data_stream {
    parent = "value"  # Required. Example format: properties/1234
}

# Access data_stream outputs
data_stream_id = data_stream.id
data_stream_display_name = data_stream.display_name
data_stream_update_time = data_stream.update_time
data_stream_ios_app_stream_data = data_stream.ios_app_stream_data
data_stream_android_app_stream_data = data_stream.android_app_stream_data
data_stream_web_stream_data = data_stream.web_stream_data
data_stream_create_time = data_stream.create_time
data_stream_type = data_stream.type
data_stream_name = data_stream.name
```

---


### Custom_metric

Creates a CustomMetric.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. Description for this custom dimension. Max length of 150 characters. |
| `name` | String |  | Output only. Resource name for this CustomMetric resource. Format: properties/{property}/customMetrics/{customMetric} |
| `display_name` | String |  | Required. Display name for this custom metric as shown in the Analytics UI. Max length of 82 characters, alphanumeric plus space and underscore starting with a letter. Legacy system-generated display names may contain square brackets, but updates to this field will never permit square brackets. |
| `measurement_unit` | String |  | Required. The type for the custom metric's value. |
| `scope` | String |  | Required. Immutable. The scope of this custom metric. |
| `parameter_name` | String |  | Required. Immutable. Tagging name for this custom metric. If this is an event-scoped metric, then this is the event parameter name. May only contain alphanumeric and underscore charactes, starting with a letter. Max length of 40 characters for event-scoped metrics. |
| `restricted_metric_type` | Vec<String> |  | Optional. Types of restricted data that this metric may contain. Required for metrics with CURRENCY measurement unit. Must be empty for metrics with a non-CURRENCY measurement unit. |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. Description for this custom dimension. Max length of 150 characters. |
| `name` | String | Output only. Resource name for this CustomMetric resource. Format: properties/{property}/customMetrics/{customMetric} |
| `display_name` | String | Required. Display name for this custom metric as shown in the Analytics UI. Max length of 82 characters, alphanumeric plus space and underscore starting with a letter. Legacy system-generated display names may contain square brackets, but updates to this field will never permit square brackets. |
| `measurement_unit` | String | Required. The type for the custom metric's value. |
| `scope` | String | Required. Immutable. The scope of this custom metric. |
| `parameter_name` | String | Required. Immutable. Tagging name for this custom metric. If this is an event-scoped metric, then this is the event parameter name. May only contain alphanumeric and underscore charactes, starting with a letter. Max length of 40 characters for event-scoped metrics. |
| `restricted_metric_type` | Vec<String> | Optional. Types of restricted data that this metric may contain. Required for metrics with CURRENCY measurement unit. Must be empty for metrics with a non-CURRENCY measurement unit. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_metric
custom_metric = provider.analyticsadmin_api.Custom_metric {
    parent = "value"  # Required. Example format: properties/1234
}

# Access custom_metric outputs
custom_metric_id = custom_metric.id
custom_metric_description = custom_metric.description
custom_metric_name = custom_metric.name
custom_metric_display_name = custom_metric.display_name
custom_metric_measurement_unit = custom_metric.measurement_unit
custom_metric_scope = custom_metric.scope
custom_metric_parameter_name = custom_metric.parameter_name
custom_metric_restricted_metric_type = custom_metric.restricted_metric_type
```

---


### Account_summarie

Returns summaries of all accounts accessible by the caller.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_summaries` | Vec<String> | Account summaries of all accounts the caller has access to. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account_summarie outputs
account_summarie_id = account_summarie.id
account_summarie_account_summaries = account_summarie.account_summaries
account_summarie_next_page_token = account_summarie.next_page_token
```

---


### Measurement_protocol_secret

Creates a measurement protocol secret.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. Human-readable display name for this secret. |
| `name` | String |  | Output only. Resource name of this secret. This secret may be a child of any type of stream. Format: properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret} |
| `secret_value` | String |  | Output only. The measurement protocol secret value. Pass this value to the api_secret field of the Measurement Protocol API when sending hits to this secret's parent property. |
| `parent` | String | ✅ | Required. The parent resource where this secret will be created. Format: properties/{property}/dataStreams/{dataStream} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. Human-readable display name for this secret. |
| `name` | String | Output only. Resource name of this secret. This secret may be a child of any type of stream. Format: properties/{property}/dataStreams/{dataStream}/measurementProtocolSecrets/{measurementProtocolSecret} |
| `secret_value` | String | Output only. The measurement protocol secret value. Pass this value to the api_secret field of the Measurement Protocol API when sending hits to this secret's parent property. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create measurement_protocol_secret
measurement_protocol_secret = provider.analyticsadmin_api.Measurement_protocol_secret {
    parent = "value"  # Required. The parent resource where this secret will be created. Format: properties/{property}/dataStreams/{dataStream}
}

# Access measurement_protocol_secret outputs
measurement_protocol_secret_id = measurement_protocol_secret.id
measurement_protocol_secret_display_name = measurement_protocol_secret.display_name
measurement_protocol_secret_name = measurement_protocol_secret.name
measurement_protocol_secret_secret_value = measurement_protocol_secret.secret_value
```

---


### Google_ads_link

Creates a GoogleAdsLink.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Format: properties/{propertyId}/googleAdsLinks/{googleAdsLinkId} Note: googleAdsLinkId is not the Google Ads customer ID. |
| `can_manage_clients` | bool |  | Output only. If true, this link is for a Google Ads manager account. |
| `customer_id` | String |  | Immutable. Google Ads customer ID. |
| `ads_personalization_enabled` | bool |  | Enable personalized advertising features with this integration. Automatically publish my Google Analytics audience lists and Google Analytics remarketing events/parameters to the linked Google Ads account. If this field is not set on create/update, it will be defaulted to true. |
| `create_time` | String |  | Output only. Time when this link was originally created. |
| `creator_email_address` | String |  | Output only. Email address of the user that created the link. An empty string will be returned if the email address can't be retrieved. |
| `update_time` | String |  | Output only. Time when this link was last updated. |
| `parent` | String | ✅ | Required. Example format: properties/1234 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `google_ads_links` | Vec<String> | List of GoogleAdsLinks. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create google_ads_link
google_ads_link = provider.analyticsadmin_api.Google_ads_link {
    parent = "value"  # Required. Example format: properties/1234
}

# Access google_ads_link outputs
google_ads_link_id = google_ads_link.id
google_ads_link_google_ads_links = google_ads_link.google_ads_links
google_ads_link_next_page_token = google_ads_link.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple key_event resources
key_event_0 = provider.analyticsadmin_api.Key_event {
    parent = "value-0"
}
key_event_1 = provider.analyticsadmin_api.Key_event {
    parent = "value-1"
}
key_event_2 = provider.analyticsadmin_api.Key_event {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    key_event = provider.analyticsadmin_api.Key_event {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Analyticsadmin_api Documentation](https://cloud.google.com/analyticsadmin_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
