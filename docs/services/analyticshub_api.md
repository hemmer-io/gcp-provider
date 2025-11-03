# Analyticshub_api Service



**Resources**: 6

---

## Overview

The analyticshub_api service provides access to 6 resource types:

- [Subscription](#subscription) [CRD]
- [Listing](#listing) [CRUD]
- [Data_exchange](#data_exchange) [CRUD]
- [Query_template](#query_template) [CRUD]
- [Listing](#listing) [CRUD]
- [Data_exchange](#data_exchange) [CRUD]

---

## Resources


### Subscription

Revokes a given subscription.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `revoke_commercial` | bool |  | Optional. If the subscription is commercial then this field must be set to true, otherwise a failure is thrown. This acts as a safety guard to avoid revoking commercial subscriptions accidentally. |
| `name` | String | ✅ | Required. Resource name of the subscription to revoke. e.g. projects/123/locations/us/subscriptions/456 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_linked_dataset_query_user_email` | bool | Output only. By default, false. If true, the Subscriber agreed to the email sharing mandate that is enabled for DataExchange/Listing. |
| `data_exchange` | String | Output only. Resource name of the source Data Exchange. e.g. projects/123/locations/us/dataExchanges/456 |
| `linked_dataset_map` | HashMap<String, String> | Output only. Map of listing resource names to associated linked resource, e.g. projects/123/locations/us/dataExchanges/456/listings/789 -> projects/123/datasets/my_dataset For listing-level subscriptions, this is a map of size 1. Only contains values if state == STATE_ACTIVE. |
| `organization_id` | String | Output only. Organization of the project this subscription belongs to. |
| `resource_type` | String | Output only. Listing shared asset type. |
| `linked_resources` | Vec<String> | Output only. Linked resources created in the subscription. Only contains values if state = STATE_ACTIVE. |
| `organization_display_name` | String | Output only. Display name of the project of this subscription. |
| `listing` | String | Output only. Resource name of the source Listing. e.g. projects/123/locations/us/dataExchanges/456/listings/789 |
| `state` | String | Output only. Current state of the subscription. |
| `destination_dataset` | String | Optional. BigQuery destination dataset to create for the subscriber. |
| `creation_time` | String | Output only. Timestamp when the subscription was created. |
| `name` | String | Output only. The resource name of the subscription. e.g. `projects/myproject/locations/us/subscriptions/123`. |
| `subscriber_contact` | String | Output only. Email of the subscriber. |
| `last_modify_time` | String | Output only. Timestamp when the subscription was last modified. |
| `commercial_info` | String | Output only. This is set if this is a commercial subscription i.e. if this subscription was created from subscribing to a commercial listing. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subscription
subscription = provider.analyticshub_api.Subscription {
    name = "value"  # Required. Resource name of the subscription to revoke. e.g. projects/123/locations/us/subscriptions/456
}

# Access subscription outputs
subscription_id = subscription.id
subscription_log_linked_dataset_query_user_email = subscription.log_linked_dataset_query_user_email
subscription_data_exchange = subscription.data_exchange
subscription_linked_dataset_map = subscription.linked_dataset_map
subscription_organization_id = subscription.organization_id
subscription_resource_type = subscription.resource_type
subscription_linked_resources = subscription.linked_resources
subscription_organization_display_name = subscription.organization_display_name
subscription_listing = subscription.listing
subscription_state = subscription.state
subscription_destination_dataset = subscription.destination_dataset
subscription_creation_time = subscription.creation_time
subscription_name = subscription.name
subscription_subscriber_contact = subscription.subscriber_contact
subscription_last_modify_time = subscription.last_modify_time
subscription_commercial_info = subscription.commercial_info
```

---


### Listing

Creates a new listing.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `discovery_type` | String |  | Optional. Type of discovery of the listing on the discovery page. |
| `log_linked_dataset_query_user_email` | bool |  | Optional. By default, false. If true, the Listing has an email sharing mandate enabled. |
| `documentation` | String |  | Optional. Documentation describing the listing. |
| `resource_type` | String |  | Output only. Listing shared asset type. |
| `stored_procedure_config` | String |  | Optional. If set, stored procedure configuration will be propagated and enforced on the linked dataset. |
| `request_access` | String |  | Optional. Email or URL of the request access of the listing. Subscribers can use this reference to request access. Max Length: 1000 bytes. |
| `bigquery_dataset` | String |  | Shared dataset i.e. BigQuery dataset source. |
| `publisher` | String |  | Optional. Details of the publisher who owns the listing and who can share the source data. |
| `icon` | String |  | Optional. Base64 encoded image representing the listing. Max Size: 3.0MiB Expected image dimensions are 512x512 pixels, however the API only performs validation on size of the encoded data. Note: For byte fields, the contents of the field are base64-encoded (which increases the size of the data by 33-36%) when using JSON on the wire. |
| `restricted_export_config` | String |  | Optional. If set, restricted export configuration will be propagated and enforced on the linked dataset. |
| `primary_contact` | String |  | Optional. Email or URL of the primary point of contact of the listing. Max Length: 1000 bytes. |
| `display_name` | String |  | Required. Human-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces. Default value is an empty string. Max length: 63 bytes. |
| `allow_only_metadata_sharing` | bool |  | Optional. If true, the listing is only available to get the resource metadata. Listing is non subscribable. |
| `commercial_info` | String |  | Output only. Commercial info contains the information about the commercial data products associated with the listing. |
| `state` | String |  | Output only. Current state of the listing. |
| `description` | String |  | Optional. Short description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). Default value is an empty string. Max length: 2000 bytes. |
| `categories` | Vec<String> |  | Optional. Categories of the listing. Up to five categories are allowed. |
| `name` | String |  | Output only. The resource name of the listing. e.g. `projects/myproject/locations/us/dataExchanges/123/listings/456` |
| `pubsub_topic` | String |  | Pub/Sub topic source. |
| `data_provider` | String |  | Optional. Details of the data provider who owns the source data. |
| `parent` | String | ✅ | Required. The parent resource path of the listing. e.g. `projects/myproject/locations/us/dataExchanges/123`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `discovery_type` | String | Optional. Type of discovery of the listing on the discovery page. |
| `log_linked_dataset_query_user_email` | bool | Optional. By default, false. If true, the Listing has an email sharing mandate enabled. |
| `documentation` | String | Optional. Documentation describing the listing. |
| `resource_type` | String | Output only. Listing shared asset type. |
| `stored_procedure_config` | String | Optional. If set, stored procedure configuration will be propagated and enforced on the linked dataset. |
| `request_access` | String | Optional. Email or URL of the request access of the listing. Subscribers can use this reference to request access. Max Length: 1000 bytes. |
| `bigquery_dataset` | String | Shared dataset i.e. BigQuery dataset source. |
| `publisher` | String | Optional. Details of the publisher who owns the listing and who can share the source data. |
| `icon` | String | Optional. Base64 encoded image representing the listing. Max Size: 3.0MiB Expected image dimensions are 512x512 pixels, however the API only performs validation on size of the encoded data. Note: For byte fields, the contents of the field are base64-encoded (which increases the size of the data by 33-36%) when using JSON on the wire. |
| `restricted_export_config` | String | Optional. If set, restricted export configuration will be propagated and enforced on the linked dataset. |
| `primary_contact` | String | Optional. Email or URL of the primary point of contact of the listing. Max Length: 1000 bytes. |
| `display_name` | String | Required. Human-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces. Default value is an empty string. Max length: 63 bytes. |
| `allow_only_metadata_sharing` | bool | Optional. If true, the listing is only available to get the resource metadata. Listing is non subscribable. |
| `commercial_info` | String | Output only. Commercial info contains the information about the commercial data products associated with the listing. |
| `state` | String | Output only. Current state of the listing. |
| `description` | String | Optional. Short description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). Default value is an empty string. Max length: 2000 bytes. |
| `categories` | Vec<String> | Optional. Categories of the listing. Up to five categories are allowed. |
| `name` | String | Output only. The resource name of the listing. e.g. `projects/myproject/locations/us/dataExchanges/123/listings/456` |
| `pubsub_topic` | String | Pub/Sub topic source. |
| `data_provider` | String | Optional. Details of the data provider who owns the source data. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create listing
listing = provider.analyticshub_api.Listing {
    parent = "value"  # Required. The parent resource path of the listing. e.g. `projects/myproject/locations/us/dataExchanges/123`.
}

# Access listing outputs
listing_id = listing.id
listing_discovery_type = listing.discovery_type
listing_log_linked_dataset_query_user_email = listing.log_linked_dataset_query_user_email
listing_documentation = listing.documentation
listing_resource_type = listing.resource_type
listing_stored_procedure_config = listing.stored_procedure_config
listing_request_access = listing.request_access
listing_bigquery_dataset = listing.bigquery_dataset
listing_publisher = listing.publisher
listing_icon = listing.icon
listing_restricted_export_config = listing.restricted_export_config
listing_primary_contact = listing.primary_contact
listing_display_name = listing.display_name
listing_allow_only_metadata_sharing = listing.allow_only_metadata_sharing
listing_commercial_info = listing.commercial_info
listing_state = listing.state
listing_description = listing.description
listing_categories = listing.categories
listing_name = listing.name
listing_pubsub_topic = listing.pubsub_topic
listing_data_provider = listing.data_provider
```

---


### Data_exchange

Creates a new data exchange.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The resource name of the data exchange. e.g. `projects/myproject/locations/us/dataExchanges/123`. |
| `primary_contact` | String |  | Optional. Email or URL of the primary point of contact of the data exchange. Max Length: 1000 bytes. |
| `sharing_environment_config` | String |  | Optional. Configurable data sharing environment option for a data exchange. |
| `icon` | String |  | Optional. Base64 encoded image representing the data exchange. Max Size: 3.0MiB Expected image dimensions are 512x512 pixels, however the API only performs validation on size of the encoded data. Note: For byte fields, the content of the fields are base64-encoded (which increases the size of the data by 33-36%) when using JSON on the wire. |
| `display_name` | String |  | Required. Human-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and must not start or end with spaces. Default value is an empty string. Max length: 63 bytes. |
| `discovery_type` | String |  | Optional. Type of discovery on the discovery page for all the listings under this exchange. Updating this field also updates (overwrites) the discovery_type field for all the listings under this exchange. |
| `documentation` | String |  | Optional. Documentation describing the data exchange. |
| `listing_count` | i64 |  | Output only. Number of listings contained in the data exchange. |
| `log_linked_dataset_query_user_email` | bool |  | Optional. By default, false. If true, the DataExchange has an email sharing mandate enabled. |
| `description` | String |  | Optional. Description of the data exchange. The description must not contain Unicode non-characters as well as C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). Default value is an empty string. Max length: 2000 bytes. |
| `parent` | String | ✅ | Required. The parent resource path of the data exchange. e.g. `projects/myproject/locations/us`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name of the data exchange. e.g. `projects/myproject/locations/us/dataExchanges/123`. |
| `primary_contact` | String | Optional. Email or URL of the primary point of contact of the data exchange. Max Length: 1000 bytes. |
| `sharing_environment_config` | String | Optional. Configurable data sharing environment option for a data exchange. |
| `icon` | String | Optional. Base64 encoded image representing the data exchange. Max Size: 3.0MiB Expected image dimensions are 512x512 pixels, however the API only performs validation on size of the encoded data. Note: For byte fields, the content of the fields are base64-encoded (which increases the size of the data by 33-36%) when using JSON on the wire. |
| `display_name` | String | Required. Human-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and must not start or end with spaces. Default value is an empty string. Max length: 63 bytes. |
| `discovery_type` | String | Optional. Type of discovery on the discovery page for all the listings under this exchange. Updating this field also updates (overwrites) the discovery_type field for all the listings under this exchange. |
| `documentation` | String | Optional. Documentation describing the data exchange. |
| `listing_count` | i64 | Output only. Number of listings contained in the data exchange. |
| `log_linked_dataset_query_user_email` | bool | Optional. By default, false. If true, the DataExchange has an email sharing mandate enabled. |
| `description` | String | Optional. Description of the data exchange. The description must not contain Unicode non-characters as well as C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). Default value is an empty string. Max length: 2000 bytes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_exchange
data_exchange = provider.analyticshub_api.Data_exchange {
    parent = "value"  # Required. The parent resource path of the data exchange. e.g. `projects/myproject/locations/us`.
}

# Access data_exchange outputs
data_exchange_id = data_exchange.id
data_exchange_name = data_exchange.name
data_exchange_primary_contact = data_exchange.primary_contact
data_exchange_sharing_environment_config = data_exchange.sharing_environment_config
data_exchange_icon = data_exchange.icon
data_exchange_display_name = data_exchange.display_name
data_exchange_discovery_type = data_exchange.discovery_type
data_exchange_documentation = data_exchange.documentation
data_exchange_listing_count = data_exchange.listing_count
data_exchange_log_linked_dataset_query_user_email = data_exchange.log_linked_dataset_query_user_email
data_exchange_description = data_exchange.description
```

---


### Query_template

Creates a new QueryTemplate

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Timestamp when the QueryTemplate was last modified. |
| `proposer` | String |  | Optional. Will be deprecated. Email or URL of the primary point of contact of the QueryTemplate. Max Length: 1000 bytes. |
| `display_name` | String |  | Required. Human-readable display name of the QueryTemplate. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces. Default value is an empty string. Max length: 63 bytes. |
| `primary_contact` | String |  | Optional. Email or URL of the primary point of contact of the QueryTemplate. Max Length: 1000 bytes. |
| `state` | String |  | Output only. The QueryTemplate lifecycle state. |
| `routine` | String |  | Optional. The routine associated with the QueryTemplate. |
| `description` | String |  | Optional. Short description of the QueryTemplate. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). Default value is an empty string. Max length: 2000 bytes. |
| `create_time` | String |  | Output only. Timestamp when the QueryTemplate was created. |
| `documentation` | String |  | Optional. Documentation describing the QueryTemplate. |
| `name` | String |  | Output only. The resource name of the QueryTemplate. e.g. `projects/myproject/locations/us/dataExchanges/123/queryTemplates/456` |
| `parent` | String | ✅ | Required. The parent resource path of the QueryTemplate. e.g. `projects/myproject/locations/us/dataExchanges/123/queryTemplates/myQueryTemplate`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Timestamp when the QueryTemplate was last modified. |
| `proposer` | String | Optional. Will be deprecated. Email or URL of the primary point of contact of the QueryTemplate. Max Length: 1000 bytes. |
| `display_name` | String | Required. Human-readable display name of the QueryTemplate. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces. Default value is an empty string. Max length: 63 bytes. |
| `primary_contact` | String | Optional. Email or URL of the primary point of contact of the QueryTemplate. Max Length: 1000 bytes. |
| `state` | String | Output only. The QueryTemplate lifecycle state. |
| `routine` | String | Optional. The routine associated with the QueryTemplate. |
| `description` | String | Optional. Short description of the QueryTemplate. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). Default value is an empty string. Max length: 2000 bytes. |
| `create_time` | String | Output only. Timestamp when the QueryTemplate was created. |
| `documentation` | String | Optional. Documentation describing the QueryTemplate. |
| `name` | String | Output only. The resource name of the QueryTemplate. e.g. `projects/myproject/locations/us/dataExchanges/123/queryTemplates/456` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create query_template
query_template = provider.analyticshub_api.Query_template {
    parent = "value"  # Required. The parent resource path of the QueryTemplate. e.g. `projects/myproject/locations/us/dataExchanges/123/queryTemplates/myQueryTemplate`.
}

# Access query_template outputs
query_template_id = query_template.id
query_template_update_time = query_template.update_time
query_template_proposer = query_template.proposer
query_template_display_name = query_template.display_name
query_template_primary_contact = query_template.primary_contact
query_template_state = query_template.state
query_template_routine = query_template.routine
query_template_description = query_template.description
query_template_create_time = query_template.create_time
query_template_documentation = query_template.documentation
query_template_name = query_template.name
```

---


### Listing

Creates a new listing.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `publisher` | String |  | Optional. Details of the publisher who owns the listing and who can share the source data. |
| `request_access` | String |  | Optional. Email or URL of the request access of the listing. Subscribers can use this reference to request access. Max Length: 1000 bytes. |
| `data_provider` | String |  | Optional. Details of the data provider who owns the source data. |
| `categories` | Vec<String> |  | Optional. Categories of the listing. Up to five categories are allowed. |
| `name` | String |  | Output only. The resource name of the listing. e.g. `projects/myproject/locations/us/dataExchanges/123/listings/456` |
| `allow_only_metadata_sharing` | bool |  | Optional. If true, the listing is only available to get the resource metadata. Listing is non subscribable. |
| `restricted_export_config` | String |  | Optional. If set, restricted export configuration will be propagated and enforced on the linked dataset. This is a required field for data clean room exchanges. |
| `state` | String |  | Output only. Current state of the listing. |
| `icon` | String |  | Optional. Base64 encoded image representing the listing. Max Size: 3.0MiB Expected image dimensions are 512x512 pixels, however the API only performs validation on size of the encoded data. Note: For byte fields, the contents of the field are base64-encoded (which increases the size of the data by 33-36%) when using JSON on the wire. |
| `display_name` | String |  | Required. Human-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces. Default value is an empty string. Max length: 63 bytes. |
| `primary_contact` | String |  | Optional. Email or URL of the primary point of contact of the listing. Max Length: 1000 bytes. |
| `documentation` | String |  | Optional. Documentation describing the listing. |
| `description` | String |  | Optional. Short description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). Default value is an empty string. Max length: 2000 bytes. |
| `bigquery_dataset` | String |  | Required. Shared dataset i.e. BigQuery dataset source. |
| `parent` | String | ✅ | Required. The parent resource path of the listing. e.g. `projects/myproject/locations/us/dataExchanges/123`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `publisher` | String | Optional. Details of the publisher who owns the listing and who can share the source data. |
| `request_access` | String | Optional. Email or URL of the request access of the listing. Subscribers can use this reference to request access. Max Length: 1000 bytes. |
| `data_provider` | String | Optional. Details of the data provider who owns the source data. |
| `categories` | Vec<String> | Optional. Categories of the listing. Up to five categories are allowed. |
| `name` | String | Output only. The resource name of the listing. e.g. `projects/myproject/locations/us/dataExchanges/123/listings/456` |
| `allow_only_metadata_sharing` | bool | Optional. If true, the listing is only available to get the resource metadata. Listing is non subscribable. |
| `restricted_export_config` | String | Optional. If set, restricted export configuration will be propagated and enforced on the linked dataset. This is a required field for data clean room exchanges. |
| `state` | String | Output only. Current state of the listing. |
| `icon` | String | Optional. Base64 encoded image representing the listing. Max Size: 3.0MiB Expected image dimensions are 512x512 pixels, however the API only performs validation on size of the encoded data. Note: For byte fields, the contents of the field are base64-encoded (which increases the size of the data by 33-36%) when using JSON on the wire. |
| `display_name` | String | Required. Human-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces. Default value is an empty string. Max length: 63 bytes. |
| `primary_contact` | String | Optional. Email or URL of the primary point of contact of the listing. Max Length: 1000 bytes. |
| `documentation` | String | Optional. Documentation describing the listing. |
| `description` | String | Optional. Short description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). Default value is an empty string. Max length: 2000 bytes. |
| `bigquery_dataset` | String | Required. Shared dataset i.e. BigQuery dataset source. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create listing
listing = provider.analyticshub_api.Listing {
    parent = "value"  # Required. The parent resource path of the listing. e.g. `projects/myproject/locations/us/dataExchanges/123`.
}

# Access listing outputs
listing_id = listing.id
listing_publisher = listing.publisher
listing_request_access = listing.request_access
listing_data_provider = listing.data_provider
listing_categories = listing.categories
listing_name = listing.name
listing_allow_only_metadata_sharing = listing.allow_only_metadata_sharing
listing_restricted_export_config = listing.restricted_export_config
listing_state = listing.state
listing_icon = listing.icon
listing_display_name = listing.display_name
listing_primary_contact = listing.primary_contact
listing_documentation = listing.documentation
listing_description = listing.description
listing_bigquery_dataset = listing.bigquery_dataset
```

---


### Data_exchange

Creates a new data exchange.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. Description of the data exchange. The description must not contain Unicode non-characters as well as C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). Default value is an empty string. Max length: 2000 bytes. |
| `icon` | String |  | Optional. Base64 encoded image representing the data exchange. Max Size: 3.0MiB Expected image dimensions are 512x512 pixels, however the API only performs validation on size of the encoded data. Note: For byte fields, the content of the fields are base64-encoded (which increases the size of the data by 33-36%) when using JSON on the wire. |
| `listing_count` | i64 |  | Output only. Number of listings contained in the data exchange. |
| `display_name` | String |  | Required. Human-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and must not start or end with spaces. Default value is an empty string. Max length: 63 bytes. |
| `documentation` | String |  | Optional. Documentation describing the data exchange. |
| `name` | String |  | Output only. The resource name of the data exchange. e.g. `projects/myproject/locations/us/dataExchanges/123`. |
| `primary_contact` | String |  | Optional. Email or URL of the primary point of contact of the data exchange. Max Length: 1000 bytes. |
| `parent` | String | ✅ | Required. The parent resource path of the data exchange. e.g. `projects/myproject/locations/us`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. Description of the data exchange. The description must not contain Unicode non-characters as well as C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). Default value is an empty string. Max length: 2000 bytes. |
| `icon` | String | Optional. Base64 encoded image representing the data exchange. Max Size: 3.0MiB Expected image dimensions are 512x512 pixels, however the API only performs validation on size of the encoded data. Note: For byte fields, the content of the fields are base64-encoded (which increases the size of the data by 33-36%) when using JSON on the wire. |
| `listing_count` | i64 | Output only. Number of listings contained in the data exchange. |
| `display_name` | String | Required. Human-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and must not start or end with spaces. Default value is an empty string. Max length: 63 bytes. |
| `documentation` | String | Optional. Documentation describing the data exchange. |
| `name` | String | Output only. The resource name of the data exchange. e.g. `projects/myproject/locations/us/dataExchanges/123`. |
| `primary_contact` | String | Optional. Email or URL of the primary point of contact of the data exchange. Max Length: 1000 bytes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_exchange
data_exchange = provider.analyticshub_api.Data_exchange {
    parent = "value"  # Required. The parent resource path of the data exchange. e.g. `projects/myproject/locations/us`.
}

# Access data_exchange outputs
data_exchange_id = data_exchange.id
data_exchange_description = data_exchange.description
data_exchange_icon = data_exchange.icon
data_exchange_listing_count = data_exchange.listing_count
data_exchange_display_name = data_exchange.display_name
data_exchange_documentation = data_exchange.documentation
data_exchange_name = data_exchange.name
data_exchange_primary_contact = data_exchange.primary_contact
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
subscription_0 = provider.analyticshub_api.Subscription {
    name = "value-0"
}
subscription_1 = provider.analyticshub_api.Subscription {
    name = "value-1"
}
subscription_2 = provider.analyticshub_api.Subscription {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    subscription = provider.analyticshub_api.Subscription {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Analyticshub_api Documentation](https://cloud.google.com/analyticshub_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
