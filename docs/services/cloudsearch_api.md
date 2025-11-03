# Cloudsearch_api Service



**Resources**: 12

---

## Overview

The cloudsearch_api service provides access to 12 resource types:

- [Stat](#stat) [R]
- [Setting](#setting) [RU]
- [Lro](#lro) [R]
- [Item](#item) [CRD]
- [Datasource](#datasource) [CRUD]
- [Searchapplication](#searchapplication) [CRUD]
- [Query](#query) [C]
- [Unmappedid](#unmappedid) [R]
- [Media](#media) [C]
- [Cloudsearch](#cloudsearch) [C]
- [Source](#source) [R]
- [Operation](#operation) [R]

---

## Resources


### Stat

Gets indexed item statistics aggreggated across all data sources. This API only returns statistics for previous dates; it doesn't return statistics for the current day. **Note:** This API requires a standard end user account to execute.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `stats` | Vec<String> | Summary of indexed item counts, one for each day in the requested range. |
| `average_indexed_item_count` | String | Average item count for the given date range for which billing is done. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access stat outputs
stat_id = stat.id
stat_stats = stat.stats
stat_average_indexed_item_count = stat.average_indexed_item_count
```

---


### Setting

Get customer settings. **Note:** This API requires an admin account to execute.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `audit_logging_settings` | String |  | Audit Logging settings for the customer. If update_mask is empty then this field will be updated based on UpdateCustomerSettings request. |
| `vpc_settings` | String |  | VPC SC settings for the customer. If update_mask is empty then this field will be updated based on UpdateCustomerSettings request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_logging_settings` | String | Audit Logging settings for the customer. If update_mask is empty then this field will be updated based on UpdateCustomerSettings request. |
| `vpc_settings` | String | VPC SC settings for the customer. If update_mask is empty then this field will be updated based on UpdateCustomerSettings request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access setting outputs
setting_id = setting.id
setting_audit_logging_settings = setting.audit_logging_settings
setting_vpc_settings = setting.vpc_settings
```

---


### Lro

Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The standard List next-page token. |
| `operations` | Vec<String> | A list of operations that matches the specified filter in the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access lro outputs
lro_id = lro.id
lro_next_page_token = lro.next_page_token
lro_operations = lro.operations
```

---


### Item

Unreserves all items from a queue, making them all eligible to be polled. This method is useful for resetting the indexing queue after a connector has been restarted. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `debug_options` | String |  | Common debug options. |
| `connector_name` | String |  | The name of connector making this call. Format: datasources/{source_id}/connectors/{ID} |
| `queue` | String |  | The name of a queue to unreserve items from. |
| `name` | String | ✅ | The name of the Data Source to unreserve all items. Format: datasources/{source_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `structured_data` | String | The structured data for the item that should conform to a registered object definition in the schema for the data source. |
| `acl` | String | Access control list for this item. |
| `status` | String | Status of the item. Output only field. |
| `payload` | String | Additional state connector can store for this item. The maximum length is 10000 bytes. |
| `name` | String | The name of the Item. Format: datasources/{source_id}/items/{item_id} This is a required field. The maximum length is 1536 characters. |
| `version` | String | Required. The indexing system stores the version from the datasource as a byte string and compares the Item version in the index to the version of the queued Item using lexical ordering. Cloud Search Indexing won't index or delete any queued item with a version value that is less than or equal to the version of the currently indexed item. The maximum length for this field is 1024 bytes. For information on how item version affects the deletion process, refer to [Handle revisions after manual deletes](https://developers.google.com/cloud-search/docs/guides/operations). |
| `content` | String | Item content to be indexed and made text searchable. |
| `item_type` | String | The type for this item. |
| `metadata` | String | The metadata information. |
| `queue` | String | Queue this item belongs to. The maximum length is 100 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create item
item = provider.cloudsearch_api.Item {
    name = "value"  # The name of the Data Source to unreserve all items. Format: datasources/{source_id}
}

# Access item outputs
item_id = item.id
item_structured_data = item.structured_data
item_acl = item.acl
item_status = item.status
item_payload = item.payload
item_name = item.name
item_version = item.version
item_content = item.content
item_item_type = item.item_type
item_metadata = item.metadata
item_queue = item.queue
```

---


### Datasource

Creates a datasource. **Note:** This API requires an admin account to execute.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `short_name` | String |  | A short name or alias for the source. This value will be used to match the 'source' operator. For example, if the short name is *<value>* then queries like *source:<value>* will only return results for this source. The value must be unique across all datasources. The value must only contain alphanumeric characters (a-zA-Z0-9). The value cannot start with 'google' and cannot be one of the following: mail, gmail, docs, drive, groups, sites, calendar, hangouts, gplus, keep, people, teams. Its maximum length is 32 characters. |
| `operation_ids` | Vec<String> |  | IDs of the Long Running Operations (LROs) currently running for this schema. |
| `disable_modifications` | bool |  | If true, sets the datasource to read-only mode. In read-only mode, the Indexing API rejects any requests to index or delete items in this source. Enabling read-only mode does not stop the processing of previously accepted data. |
| `items_visibility` | Vec<String> |  | This field restricts visibility to items at the datasource level. Items within the datasource are restricted to the union of users and groups included in this field. Note that, this does not ensure access to a specific item, as users need to have ACL permissions on the contained items. This ensures a high level access on the entire datasource, and that the individual items are not shared outside this visibility. |
| `name` | String |  | The name of the datasource resource. Format: datasources/{source_id}. The name is ignored when creating a datasource. |
| `disable_serving` | bool |  | Disable serving any search or assist results. |
| `display_name` | String |  | Required. Display name of the datasource The maximum length is 300 characters. |
| `indexing_service_accounts` | Vec<String> |  | List of service accounts that have indexing access. |
| `return_thumbnail_urls` | bool |  | Can a user request to get thumbnail URI for Items indexed in this data source. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `average_indexed_item_count` | String | Average item count for the given date range for which billing is done. |
| `stats` | Vec<String> | Summary of indexed item counts, one for each day in the requested range. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create datasource
datasource = provider.cloudsearch_api.Datasource {
}

# Access datasource outputs
datasource_id = datasource.id
datasource_average_indexed_item_count = datasource.average_indexed_item_count
datasource_stats = datasource.stats
```

---


### Searchapplication

Creates a search application. **Note:** This API requires an admin account to execute.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_audit_log` | bool |  | Indicates whether audit logging is on/off for requests made for the search application in query APIs. |
| `default_sort_options` | String |  | The default options for sorting the search results |
| `name` | String |  | The name of the Search Application. Format: searchapplications/{application_id}. |
| `operation_ids` | Vec<String> |  | Output only. IDs of the Long Running Operations (LROs) currently running for this schema. Output only field. |
| `return_result_thumbnail_urls` | bool |  | With each result we should return the URI for its thumbnail (when applicable) |
| `query_interpretation_config` | String |  | The default options for query interpretation |
| `scoring_config` | String |  | Configuration for ranking results. |
| `source_config` | Vec<String> |  | Configuration for a sources specified in data_source_restrictions. |
| `default_facet_options` | Vec<String> |  | The default fields for returning facet results. The sources specified here also have been included in data_source_restrictions above. |
| `display_name` | String |  | Display name of the Search Application. The maximum length is 300 characters. |
| `data_source_restrictions` | Vec<String> |  | Retrictions applied to the configurations. The maximum number of elements is 10. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_query_count` | String | Total successful query count (status code 200) for the given date range. |
| `stats` | Vec<String> | Query stats per date for a search application. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create searchapplication
searchapplication = provider.cloudsearch_api.Searchapplication {
}

# Access searchapplication outputs
searchapplication_id = searchapplication.id
searchapplication_total_query_count = searchapplication.total_query_count
searchapplication_stats = searchapplication.stats
```

---


### Query

Provides suggestions for autocompleting the query. **Note:** This API requires a standard end user account to execute. A service account can't perform Query API requests directly; to use a service account to perform queries, set up [Google Workspace domain-wide delegation of authority](https://developers.google.com/cloud-search/docs/guides/delegation/).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source_restrictions` | Vec<String> |  | The sources to use for suggestions. If not specified, the data sources are taken from the current search application. NOTE: Suggestions are only supported for the following sources: * Third-party data sources * PredefinedSource.PERSON * PredefinedSource.GOOGLE_DRIVE |
| `query` | String |  | Partial query for which autocomplete suggestions will be shown. For example, if the query is "sea", then the server might return "season", "search", "seagull" and so on. |
| `request_options` | String |  | Request options, such as the search application and user timezone. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create query
query = provider.cloudsearch_api.Query {
}

```

---


### Unmappedid

List all unmapped identities for a specific item. **Note:** This API requires an admin account to execute.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results, or empty if there are no more results in the list. |
| `unmapped_identities` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access unmappedid outputs
unmappedid_id = unmappedid.id
unmappedid_next_page_token = unmappedid.next_page_token
unmappedid_unmapped_identities = unmappedid.unmapped_identities
```

---


### Media

Uploads media for indexing. The upload endpoint supports direct and resumable upload protocols and is intended for large items that can not be [inlined during index requests](https://developers.google.com/cloud-search/docs/reference/rest/v1/indexing.datasources.items#itemcontent). To index large content: 1. Call indexing.datasources.items.upload with the item name to begin an upload session and retrieve the UploadItemRef. 1. Call media.upload to upload the content, as a streaming request, using the same resource name from the UploadItemRef from step 1. 1. Call indexing.datasources.items.index to index the item. Populate the [ItemContent](/cloud-search/docs/reference/rest/v1/indexing.datasources.items#ItemContent) with the UploadItemRef from step 1. For additional information, see [Create a content connector using the REST API](https://developers.google.com/cloud-search/docs/guides/content-connector#rest). **Note:** This API requires a service account to execute.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_name` | String |  | Name of the media resource. |
| `resource_name` | String | ✅ | Name of the media that is being downloaded. See ReadRequest.resource_name. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.cloudsearch_api.Media {
    resource_name = "value"  # Name of the media that is being downloaded. See ReadRequest.resource_name.
}

```

---


### Cloudsearch

Enables `third party` support in Google Cloud Search. **Note:** This API requires an admin account to execute.

**Operations**: ✅ Create

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

# Create cloudsearch
cloudsearch = provider.cloudsearch_api.Cloudsearch {
}

```

---


### Source

Returns list of sources that user can use for Search and Suggest APIs. **Note:** This API requires a standard end user account to execute. A service account can't perform Query API requests directly; to use a service account to perform queries, set up [Google Workspace domain-wide delegation of authority](https://developers.google.com/cloud-search/docs/guides/delegation/).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String |  |
| `sources` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access source outputs
source_id = source.id
source_next_page_token = source.next_page_token
source_sources = source.sources
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
operation_response = operation.response
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple stat resources
stat_0 = provider.cloudsearch_api.Stat {
}
stat_1 = provider.cloudsearch_api.Stat {
}
stat_2 = provider.cloudsearch_api.Stat {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    stat = provider.cloudsearch_api.Stat {
    }
```

---

## Related Documentation

- [GCP Cloudsearch_api Documentation](https://cloud.google.com/cloudsearch_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
