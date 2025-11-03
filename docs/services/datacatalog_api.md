# Datacatalog_api Service



**Resources**: 20

---

## Overview

The datacatalog_api service provides access to 20 resource types:

- [Tag_template](#tag_template) [CRUD]
- [Taxonomie](#taxonomie) [CRUD]
- [Field](#field) [CUD]
- [Entry_group](#entry_group) [CRUD]
- [Operation](#operation) [CRD]
- [Catalog](#catalog) [C]
- [Policy_tag](#policy_tag) [CRUD]
- [Location](#location) [CR]
- [Entrie](#entrie) [CRUD]
- [Tag](#tag) [CRUD]
- [Enum_value](#enum_value) [C]
- [Entry_group](#entry_group) [CRUD]
- [Enum_value](#enum_value) [C]
- [Tag](#tag) [CRUD]
- [Field](#field) [CUD]
- [Catalog](#catalog) [C]
- [Entrie](#entrie) [CRUD]
- [Policy_tag](#policy_tag) [CRUD]
- [Tag_template](#tag_template) [CRUD]
- [Taxonomie](#taxonomie) [CRUD]

---

## Resources


### Tag_template

Creates a tag template. You must enable the Data Catalog API in the project identified by the `parent` parameter. For more information, see [Data Catalog resource project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the tag template in URL format. Note: The tag template itself and its child resources might not be stored in the location specified in its name. |
| `dataplex_transfer_status` | String |  | Optional. Transfer status of the TagTemplate |
| `display_name` | String |  | Display name for this template. Defaults to an empty string. The name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and can't start or end with spaces. The maximum length is 200 characters. |
| `fields` | HashMap<String, String> |  | Required. Map of tag template field IDs to the settings for the field. This map is an exhaustive list of the allowed fields. The map must contain at least one field and at most 500 fields. The keys to this map are tag template field IDs. The IDs have the following limitations: * Can contain uppercase and lowercase letters, numbers (0-9) and underscores (_). * Must be at least 1 character and at most 64 characters long. * Must start with a letter or underscore. |
| `is_publicly_readable` | bool |  | Indicates whether tags created with this template are public. Public tags do not require tag template access to appear in ListTags API response. Additionally, you can search for a public tag by value with a simple search query in addition to using a ``tag:`` predicate. |
| `parent` | String | ✅ | Required. The name of the project and the template location [region](https://cloud.google.com/data-catalog/docs/concepts/regions). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the tag template in URL format. Note: The tag template itself and its child resources might not be stored in the location specified in its name. |
| `dataplex_transfer_status` | String | Optional. Transfer status of the TagTemplate |
| `display_name` | String | Display name for this template. Defaults to an empty string. The name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and can't start or end with spaces. The maximum length is 200 characters. |
| `fields` | HashMap<String, String> | Required. Map of tag template field IDs to the settings for the field. This map is an exhaustive list of the allowed fields. The map must contain at least one field and at most 500 fields. The keys to this map are tag template field IDs. The IDs have the following limitations: * Can contain uppercase and lowercase letters, numbers (0-9) and underscores (_). * Must be at least 1 character and at most 64 characters long. * Must start with a letter or underscore. |
| `is_publicly_readable` | bool | Indicates whether tags created with this template are public. Public tags do not require tag template access to appear in ListTags API response. Additionally, you can search for a public tag by value with a simple search query in addition to using a ``tag:`` predicate. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag_template
tag_template = provider.datacatalog_api.Tag_template {
    parent = "value"  # Required. The name of the project and the template location [region](https://cloud.google.com/data-catalog/docs/concepts/regions).
}

# Access tag_template outputs
tag_template_id = tag_template.id
tag_template_name = tag_template.name
tag_template_dataplex_transfer_status = tag_template.dataplex_transfer_status
tag_template_display_name = tag_template.display_name
tag_template_fields = tag_template.fields
tag_template_is_publicly_readable = tag_template.is_publicly_readable
```

---


### Taxonomie

Creates a taxonomy in a specified project. The taxonomy is initially empty, that is, it doesn't contain policy tags.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `taxonomy_timestamps` | String |  | Output only. Creation and modification timestamps of this taxonomy. |
| `name` | String |  | Identifier. Resource name of this taxonomy in URL format. Note: Policy tag manager generates unique taxonomy IDs. |
| `display_name` | String |  | Required. User-defined name of this taxonomy. The name can't start or end with spaces, must contain only Unicode letters, numbers, underscores, dashes, and spaces, and be at most 200 bytes long when encoded in UTF-8. The taxonomy display name must be unique within an organization. |
| `activated_policy_types` | Vec<String> |  | Optional. A list of policy types that are activated for this taxonomy. If not set, defaults to an empty list. |
| `service` | String |  | Output only. Identity of the service which owns the Taxonomy. This field is only populated when the taxonomy is created by a Google Cloud service. Currently only 'DATAPLEX' is supported. |
| `description` | String |  | Optional. Description of this taxonomy. If not set, defaults to empty. The description must contain only Unicode characters, tabs, newlines, carriage returns, and page breaks, and be at most 2000 bytes long when encoded in UTF-8. |
| `policy_tag_count` | i64 |  | Output only. Number of policy tags in this taxonomy. |
| `parent` | String | ✅ | Required. Resource name of the project that the taxonomy will belong to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `taxonomy_timestamps` | String | Output only. Creation and modification timestamps of this taxonomy. |
| `name` | String | Identifier. Resource name of this taxonomy in URL format. Note: Policy tag manager generates unique taxonomy IDs. |
| `display_name` | String | Required. User-defined name of this taxonomy. The name can't start or end with spaces, must contain only Unicode letters, numbers, underscores, dashes, and spaces, and be at most 200 bytes long when encoded in UTF-8. The taxonomy display name must be unique within an organization. |
| `activated_policy_types` | Vec<String> | Optional. A list of policy types that are activated for this taxonomy. If not set, defaults to an empty list. |
| `service` | String | Output only. Identity of the service which owns the Taxonomy. This field is only populated when the taxonomy is created by a Google Cloud service. Currently only 'DATAPLEX' is supported. |
| `description` | String | Optional. Description of this taxonomy. If not set, defaults to empty. The description must contain only Unicode characters, tabs, newlines, carriage returns, and page breaks, and be at most 2000 bytes long when encoded in UTF-8. |
| `policy_tag_count` | i64 | Output only. Number of policy tags in this taxonomy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create taxonomie
taxonomie = provider.datacatalog_api.Taxonomie {
    parent = "value"  # Required. Resource name of the project that the taxonomy will belong to.
}

# Access taxonomie outputs
taxonomie_id = taxonomie.id
taxonomie_taxonomy_timestamps = taxonomie.taxonomy_timestamps
taxonomie_name = taxonomie.name
taxonomie_display_name = taxonomie.display_name
taxonomie_activated_policy_types = taxonomie.activated_policy_types
taxonomie_service = taxonomie.service
taxonomie_description = taxonomie.description
taxonomie_policy_tag_count = taxonomie.policy_tag_count
```

---


### Field

Creates a field in a tag template. You must enable the Data Catalog API in the project identified by the `parent` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `order` | i64 |  | The order of this field with respect to other fields in this tag template. For example, a higher value can indicate a more important field. The value can be negative. Multiple fields can have the same order and field orders within a tag don't have to be sequential. |
| `display_name` | String |  | The display name for this field. Defaults to an empty string. The name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and can't start or end with spaces. The maximum length is 200 characters. |
| `is_required` | bool |  | If true, this field is required. Defaults to false. |
| `name` | String |  | Identifier. The resource name of the tag template field in URL format. Example: `projects/{PROJECT_ID}/locations/{LOCATION}/tagTemplates/{TAG_TEMPLATE}/fields/{FIELD}` Note: The tag template field itself might not be stored in the location specified in its name. The name must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_), and must start with a letter or underscore. The maximum length is 64 characters. |
| `type` | String |  | Required. The type of value this tag field can contain. |
| `description` | String |  | The description for this field. Defaults to an empty string. |
| `parent` | String | ✅ | Required. The name of the project and the template location [region](https://cloud.google.com/data-catalog/docs/concepts/regions). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create field
field = provider.datacatalog_api.Field {
    parent = "value"  # Required. The name of the project and the template location [region](https://cloud.google.com/data-catalog/docs/concepts/regions).
}

```

---


### Entry_group

Creates an entry group. An entry group contains logically related entries together with [Cloud Identity and Access Management](/data-catalog/docs/concepts/iam) policies. These policies specify users who can create, edit, and view entries within entry groups. Data Catalog automatically creates entry groups with names that start with the `@` symbol for the following resources: * BigQuery entries (`@bigquery`) * Pub/Sub topics (`@pubsub`) * Dataproc Metastore services (`@dataproc_metastore_{SERVICE_NAME_HASH}`) You can create your own entry groups for Cloud Storage fileset entries and custom entries together with the corresponding IAM policies. User-created entry groups can't contain the `@` symbol, it is reserved for automatically created groups. Entry groups, like entries, can be searched. A maximum of 10,000 entry groups may be created per organization across all locations. You must enable the Data Catalog API in the project identified by the `parent` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `transferred_to_dataplex` | bool |  | Optional. When set to [true], it means DataCatalog EntryGroup was transferred to Dataplex Universal Catalog. It makes EntryGroup and its Entries to be read-only in DataCatalog. However, new Tags on EntryGroup and its Entries can be created. After setting the flag to [true] it cannot be unset. |
| `display_name` | String |  | A short name to identify the entry group, for example, "analytics data - jan 2011". Default value is an empty string. |
| `data_catalog_timestamps` | String |  | Output only. Timestamps of the entry group. Default value is empty. |
| `name` | String |  | Identifier. The resource name of the entry group in URL format. Note: The entry group itself and its child resources might not be stored in the location specified in its name. |
| `description` | String |  | Entry group description. Can consist of several sentences or paragraphs that describe the entry group contents. Default value is an empty string. |
| `parent` | String | ✅ | Required. The names of the project and location that the new entry group belongs to. Note: The entry group itself and its child resources might not be stored in the location specified in its name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `transferred_to_dataplex` | bool | Optional. When set to [true], it means DataCatalog EntryGroup was transferred to Dataplex Universal Catalog. It makes EntryGroup and its Entries to be read-only in DataCatalog. However, new Tags on EntryGroup and its Entries can be created. After setting the flag to [true] it cannot be unset. |
| `display_name` | String | A short name to identify the entry group, for example, "analytics data - jan 2011". Default value is an empty string. |
| `data_catalog_timestamps` | String | Output only. Timestamps of the entry group. Default value is empty. |
| `name` | String | Identifier. The resource name of the entry group in URL format. Note: The entry group itself and its child resources might not be stored in the location specified in its name. |
| `description` | String | Entry group description. Can consist of several sentences or paragraphs that describe the entry group contents. Default value is an empty string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entry_group
entry_group = provider.datacatalog_api.Entry_group {
    parent = "value"  # Required. The names of the project and location that the new entry group belongs to. Note: The entry group itself and its child resources might not be stored in the location specified in its name.
}

# Access entry_group outputs
entry_group_id = entry_group.id
entry_group_transferred_to_dataplex = entry_group.transferred_to_dataplex
entry_group_display_name = entry_group.display_name
entry_group_data_catalog_timestamps = entry_group.data_catalog_timestamps
entry_group_name = entry_group.name
entry_group_description = entry_group.description
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.datacatalog_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_response = operation.response
operation_name = operation.name
operation_metadata = operation.metadata
```

---


### Catalog

Searches Data Catalog for multiple resources like entries and tags that match a query. This is a [Custom Method] (https://cloud.google.com/apis/design/custom_methods) that doesn't return all information on a resource, only its ID and high level fields. To get more information, you can subsequently call specific get methods. Note: Data Catalog search queries don't guarantee full recall. Results that match your query might not be returned, even in subsequent result pages. Additionally, returned (and not returned) results can vary if you repeat search queries. For more information, see [Data Catalog search syntax] (https://cloud.google.com/data-catalog/docs/how-to/search-reference).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_token` | String |  | Optional. Pagination token that, if specified, returns the next page of search results. If empty, returns the first page. This token is returned in the SearchCatalogResponse.next_page_token field of the response to a previous SearchCatalogRequest call. |
| `admin_search` | bool |  | Optional. If set, use searchAll permission granted on organizations from `include_org_ids` and projects from `include_project_ids` instead of the fine grained per resource permissions when filtering the search results. The only allowed `order_by` criteria for admin_search mode is `default`. Using this flags guarantees a full recall of the search results. |
| `scope` | String |  | Required. The scope of this search request. The `scope` is invalid if `include_org_ids`, `include_project_ids` are empty AND `include_gcp_public_datasets` is set to `false`. In this case, the request returns an error. |
| `page_size` | i64 |  | Upper bound on the number of results you can get in a single response. Can't be negative or 0, defaults to 10 in this case. The maximum number is 1000. If exceeded, throws an "invalid argument" exception. |
| `query` | String |  | Optional. The query string with a minimum of 3 characters and specific syntax. For more information, see [Data Catalog search syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference). An empty query string returns all data assets (in the specified scope) that you have access to. A query string can be a simple `xyz` or qualified by predicates: * `name:x` * `column:y` * `description:z` |
| `order_by` | String |  | Specifies the order of results. Currently supported case-sensitive values are: * `relevance` that can only be descending * `last_modified_timestamp [asc|desc]` with descending (`desc`) as default * `default` that can only be descending Search queries don't guarantee full recall. Results that match your query might not be returned, even in subsequent result pages. Additionally, returned (and not returned) results can vary if you repeat search queries. If you are experiencing recall issues and you don't have to fetch the results in any specific order, consider setting this parameter to `default`. If this parameter is omitted, it defaults to the descending `relevance`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create catalog
catalog = provider.datacatalog_api.Catalog {
}

```

---


### Policy_tag

Creates a policy tag in a taxonomy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Resource name of this policy tag in the URL format. The policy tag manager generates unique taxonomy IDs and policy tag IDs. |
| `description` | String |  | Description of this policy tag. If not set, defaults to empty. The description must contain only Unicode characters, tabs, newlines, carriage returns and page breaks, and be at most 2000 bytes long when encoded in UTF-8. |
| `display_name` | String |  | Required. User-defined name of this policy tag. The name can't start or end with spaces and must be unique within the parent taxonomy, contain only Unicode letters, numbers, underscores, dashes and spaces, and be at most 200 bytes long when encoded in UTF-8. |
| `parent_policy_tag` | String |  | Resource name of this policy tag's parent policy tag. If empty, this is a top level tag. If not set, defaults to an empty string. For example, for the "LatLong" policy tag in the example above, this field contains the resource name of the "Geolocation" policy tag, and, for "Geolocation", this field is empty. |
| `child_policy_tags` | Vec<String> |  | Output only. Resource names of child policy tags of this policy tag. |
| `parent` | String | ✅ | Required. Resource name of the taxonomy that the policy tag will belong to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Resource name of this policy tag in the URL format. The policy tag manager generates unique taxonomy IDs and policy tag IDs. |
| `description` | String | Description of this policy tag. If not set, defaults to empty. The description must contain only Unicode characters, tabs, newlines, carriage returns and page breaks, and be at most 2000 bytes long when encoded in UTF-8. |
| `display_name` | String | Required. User-defined name of this policy tag. The name can't start or end with spaces and must be unique within the parent taxonomy, contain only Unicode letters, numbers, underscores, dashes and spaces, and be at most 200 bytes long when encoded in UTF-8. |
| `parent_policy_tag` | String | Resource name of this policy tag's parent policy tag. If empty, this is a top level tag. If not set, defaults to an empty string. For example, for the "LatLong" policy tag in the example above, this field contains the resource name of the "Geolocation" policy tag, and, for "Geolocation", this field is empty. |
| `child_policy_tags` | Vec<String> | Output only. Resource names of child policy tags of this policy tag. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policy_tag
policy_tag = provider.datacatalog_api.Policy_tag {
    parent = "value"  # Required. Resource name of the taxonomy that the policy tag will belong to.
}

# Access policy_tag outputs
policy_tag_id = policy_tag.id
policy_tag_name = policy_tag.name
policy_tag_description = policy_tag.description
policy_tag_display_name = policy_tag.display_name
policy_tag_parent_policy_tag = policy_tag.parent_policy_tag
policy_tag_child_policy_tags = policy_tag.child_policy_tags
```

---


### Location

Sets the configuration related to the migration to Dataplex Universal Catalog for an organization or project.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `catalog_ui_experience` | String |  | Opt-in status for the UI switch to Dataplex Universal Catalog. |
| `tag_template_migration` | String |  | Opt-in status for the migration of Tag Templates to Dataplex Universal Catalog. |
| `name` | String | ✅ | Required. The organization or project whose config is being specified. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `catalog_ui_experience` | String | Opt-in status for the UI switch to Dataplex Universal Catalog. |
| `tag_template_migration` | String | Opt-in status for the migration of Tag Templates to Dataplex Universal Catalog. |
| `template_migration_enabled_time` | String | The time when the Tag Template migration was enabled. If the Tag Template migration is not enabled, this field is not set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.datacatalog_api.Location {
    name = "value"  # Required. The organization or project whose config is being specified.
}

# Access location outputs
location_id = location.id
location_catalog_ui_experience = location.catalog_ui_experience
location_tag_template_migration = location.tag_template_migration
location_template_migration_enabled_time = location.template_migration_enabled_time
```

---


### Entrie

Creates an entry. You can create entries only with 'FILESET', 'CLUSTER', 'DATA_STREAM', or custom types. Data Catalog automatically creates entries with other types during metadata ingestion from integrated systems. You must enable the Data Catalog API in the project identified by the `parent` parameter. For more information, see [Data Catalog resource project](https://cloud.google.com/data-catalog/docs/concepts/resource-project). An entry group can have a maximum of 100,000 entries.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `usage_signal` | String |  | Resource usage statistics. |
| `fully_qualified_name` | String |  | [Fully Qualified Name (FQN)](https://cloud.google.com//data-catalog/docs/fully-qualified-names) of the resource. Set automatically for entries representing resources from synced systems. Settable only during creation, and read-only later. Can be used for search and lookup of the entries.  |
| `bigquery_date_sharded_spec` | String |  | Output only. Specification for a group of BigQuery tables with the `[prefix]YYYYMMDD` name pattern. For more information, see [Introduction to partitioned tables] (https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding). |
| `user_specified_type` | String |  | Custom entry type that doesn't match any of the values allowed for input and listed in the `EntryType` enum. When creating an entry, first check the type values in the enum. If there are no appropriate types for the new entry, provide a custom value, for example, `my_special_type`. The `user_specified_type` string has the following limitations: * Is case insensitive. * Must begin with a letter or underscore. * Can only contain letters, numbers, and underscores. * Must be at least 1 character and at most 64 characters long. |
| `type` | String |  | The type of the entry. For details, see [`EntryType`](#entrytype). |
| `fileset_spec` | String |  | Specification that applies to a fileset resource. Valid only for entries with the `FILESET` type. |
| `looker_system_spec` | String |  | Specification that applies to Looker sysstem. Only settable when `user_specified_system` is equal to `LOOKER` |
| `model_spec` | String |  | Model specification. |
| `description` | String |  | Entry description that can consist of several sentences or paragraphs that describe entry contents. The description must not contain Unicode non-characters as well as C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). The maximum size is 2000 bytes when encoded in UTF-8. Default value is an empty string. |
| `integrated_system` | String |  | Output only. Indicates the entry's source system that Data Catalog integrates with, such as BigQuery, Pub/Sub, or Dataproc Metastore. |
| `schema` | String |  | Schema of the entry. An entry might not have any schema attached to it. |
| `labels` | HashMap<String, String> |  | Cloud labels attached to the entry. In Data Catalog, you can create and modify labels attached only to custom entries. Synced entries have unmodifiable labels that come from the source system. |
| `personal_details` | String |  | Output only. Additional information related to the entry. Private to the current user. |
| `feature_online_store_spec` | String |  | FeatureonlineStore spec for Vertex AI Feature Store. |
| `service_spec` | String |  | Specification that applies to a Service resource. |
| `cloud_bigtable_system_spec` | String |  | Specification that applies to Cloud Bigtable system. Only settable when `integrated_system` is equal to `CLOUD_BIGTABLE` |
| `name` | String |  | Output only. Identifier. The resource name of an entry in URL format. Note: The entry itself and its child resources might not be stored in the location specified in its name. |
| `source_system_timestamps` | String |  | Timestamps from the underlying resource, not from the Data Catalog entry. Output only when the entry has a system listed in the `IntegratedSystem` enum. For entries with `user_specified_system`, this field is optional and defaults to an empty timestamp. |
| `bigquery_table_spec` | String |  | Output only. Specification that applies to a BigQuery table. Valid only for entries with the `TABLE` type. |
| `data_source_connection_spec` | String |  | Specification that applies to a data source connection. Valid only for entries with the `DATA_SOURCE_CONNECTION` type. |
| `data_source` | String |  | Output only. Physical location of the entry. |
| `display_name` | String |  | Display name of an entry. The maximum size is 500 bytes when encoded in UTF-8. Default value is an empty string. |
| `routine_spec` | String |  | Specification that applies to a user-defined function or procedure. Valid only for entries with the `ROUTINE` type. |
| `user_specified_system` | String |  | Indicates the entry's source system that Data Catalog doesn't automatically integrate with. The `user_specified_system` string has the following limitations: * Is case insensitive. * Must begin with a letter or underscore. * Can only contain letters, numbers, and underscores. * Must be at least 1 character and at most 64 characters long. |
| `linked_resource` | String |  | The resource this metadata entry refers to. For Google Cloud Platform resources, `linked_resource` is the [Full Resource Name] (https://cloud.google.com/apis/design/resource_names#full_resource_name). For example, the `linked_resource` for a table resource from BigQuery is: `//bigquery.googleapis.com/projects/{PROJECT_ID}/datasets/{DATASET_ID}/tables/{TABLE_ID}` Output only when the entry is one of the types in the `EntryType` enum. For entries with a `user_specified_type`, this field is optional and defaults to an empty string. The resource string must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), periods (.), colons (:), slashes (/), dashes (-), and hashes (#). The maximum size is 200 bytes when encoded in UTF-8. |
| `business_context` | String |  | Business Context of the entry. Not supported for BigQuery datasets |
| `dataset_spec` | String |  | Specification that applies to a dataset. |
| `database_table_spec` | String |  | Specification that applies to a table resource. Valid only for entries with the `TABLE` or `EXPLORE` type. |
| `gcs_fileset_spec` | String |  | Specification that applies to a Cloud Storage fileset. Valid only for entries with the `FILESET` type. |
| `sql_database_system_spec` | String |  | Specification that applies to a relational database system. Only settable when `user_specified_system` is equal to `SQL_DATABASE` |
| `parent` | String | ✅ | Required. The name of the entry group this entry belongs to. Note: The entry itself and its child resources might not be stored in the location specified in its name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `usage_signal` | String | Resource usage statistics. |
| `fully_qualified_name` | String | [Fully Qualified Name (FQN)](https://cloud.google.com//data-catalog/docs/fully-qualified-names) of the resource. Set automatically for entries representing resources from synced systems. Settable only during creation, and read-only later. Can be used for search and lookup of the entries.  |
| `bigquery_date_sharded_spec` | String | Output only. Specification for a group of BigQuery tables with the `[prefix]YYYYMMDD` name pattern. For more information, see [Introduction to partitioned tables] (https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding). |
| `user_specified_type` | String | Custom entry type that doesn't match any of the values allowed for input and listed in the `EntryType` enum. When creating an entry, first check the type values in the enum. If there are no appropriate types for the new entry, provide a custom value, for example, `my_special_type`. The `user_specified_type` string has the following limitations: * Is case insensitive. * Must begin with a letter or underscore. * Can only contain letters, numbers, and underscores. * Must be at least 1 character and at most 64 characters long. |
| `type` | String | The type of the entry. For details, see [`EntryType`](#entrytype). |
| `fileset_spec` | String | Specification that applies to a fileset resource. Valid only for entries with the `FILESET` type. |
| `looker_system_spec` | String | Specification that applies to Looker sysstem. Only settable when `user_specified_system` is equal to `LOOKER` |
| `model_spec` | String | Model specification. |
| `description` | String | Entry description that can consist of several sentences or paragraphs that describe entry contents. The description must not contain Unicode non-characters as well as C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF). The maximum size is 2000 bytes when encoded in UTF-8. Default value is an empty string. |
| `integrated_system` | String | Output only. Indicates the entry's source system that Data Catalog integrates with, such as BigQuery, Pub/Sub, or Dataproc Metastore. |
| `schema` | String | Schema of the entry. An entry might not have any schema attached to it. |
| `labels` | HashMap<String, String> | Cloud labels attached to the entry. In Data Catalog, you can create and modify labels attached only to custom entries. Synced entries have unmodifiable labels that come from the source system. |
| `personal_details` | String | Output only. Additional information related to the entry. Private to the current user. |
| `feature_online_store_spec` | String | FeatureonlineStore spec for Vertex AI Feature Store. |
| `service_spec` | String | Specification that applies to a Service resource. |
| `cloud_bigtable_system_spec` | String | Specification that applies to Cloud Bigtable system. Only settable when `integrated_system` is equal to `CLOUD_BIGTABLE` |
| `name` | String | Output only. Identifier. The resource name of an entry in URL format. Note: The entry itself and its child resources might not be stored in the location specified in its name. |
| `source_system_timestamps` | String | Timestamps from the underlying resource, not from the Data Catalog entry. Output only when the entry has a system listed in the `IntegratedSystem` enum. For entries with `user_specified_system`, this field is optional and defaults to an empty timestamp. |
| `bigquery_table_spec` | String | Output only. Specification that applies to a BigQuery table. Valid only for entries with the `TABLE` type. |
| `data_source_connection_spec` | String | Specification that applies to a data source connection. Valid only for entries with the `DATA_SOURCE_CONNECTION` type. |
| `data_source` | String | Output only. Physical location of the entry. |
| `display_name` | String | Display name of an entry. The maximum size is 500 bytes when encoded in UTF-8. Default value is an empty string. |
| `routine_spec` | String | Specification that applies to a user-defined function or procedure. Valid only for entries with the `ROUTINE` type. |
| `user_specified_system` | String | Indicates the entry's source system that Data Catalog doesn't automatically integrate with. The `user_specified_system` string has the following limitations: * Is case insensitive. * Must begin with a letter or underscore. * Can only contain letters, numbers, and underscores. * Must be at least 1 character and at most 64 characters long. |
| `linked_resource` | String | The resource this metadata entry refers to. For Google Cloud Platform resources, `linked_resource` is the [Full Resource Name] (https://cloud.google.com/apis/design/resource_names#full_resource_name). For example, the `linked_resource` for a table resource from BigQuery is: `//bigquery.googleapis.com/projects/{PROJECT_ID}/datasets/{DATASET_ID}/tables/{TABLE_ID}` Output only when the entry is one of the types in the `EntryType` enum. For entries with a `user_specified_type`, this field is optional and defaults to an empty string. The resource string must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), periods (.), colons (:), slashes (/), dashes (-), and hashes (#). The maximum size is 200 bytes when encoded in UTF-8. |
| `business_context` | String | Business Context of the entry. Not supported for BigQuery datasets |
| `dataset_spec` | String | Specification that applies to a dataset. |
| `database_table_spec` | String | Specification that applies to a table resource. Valid only for entries with the `TABLE` or `EXPLORE` type. |
| `gcs_fileset_spec` | String | Specification that applies to a Cloud Storage fileset. Valid only for entries with the `FILESET` type. |
| `sql_database_system_spec` | String | Specification that applies to a relational database system. Only settable when `user_specified_system` is equal to `SQL_DATABASE` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entrie
entrie = provider.datacatalog_api.Entrie {
    parent = "value"  # Required. The name of the entry group this entry belongs to. Note: The entry itself and its child resources might not be stored in the location specified in its name.
}

# Access entrie outputs
entrie_id = entrie.id
entrie_usage_signal = entrie.usage_signal
entrie_fully_qualified_name = entrie.fully_qualified_name
entrie_bigquery_date_sharded_spec = entrie.bigquery_date_sharded_spec
entrie_user_specified_type = entrie.user_specified_type
entrie_type = entrie.type
entrie_fileset_spec = entrie.fileset_spec
entrie_looker_system_spec = entrie.looker_system_spec
entrie_model_spec = entrie.model_spec
entrie_description = entrie.description
entrie_integrated_system = entrie.integrated_system
entrie_schema = entrie.schema
entrie_labels = entrie.labels
entrie_personal_details = entrie.personal_details
entrie_feature_online_store_spec = entrie.feature_online_store_spec
entrie_service_spec = entrie.service_spec
entrie_cloud_bigtable_system_spec = entrie.cloud_bigtable_system_spec
entrie_name = entrie.name
entrie_source_system_timestamps = entrie.source_system_timestamps
entrie_bigquery_table_spec = entrie.bigquery_table_spec
entrie_data_source_connection_spec = entrie.data_source_connection_spec
entrie_data_source = entrie.data_source
entrie_display_name = entrie.display_name
entrie_routine_spec = entrie.routine_spec
entrie_user_specified_system = entrie.user_specified_system
entrie_linked_resource = entrie.linked_resource
entrie_business_context = entrie.business_context
entrie_dataset_spec = entrie.dataset_spec
entrie_database_table_spec = entrie.database_table_spec
entrie_gcs_fileset_spec = entrie.gcs_fileset_spec
entrie_sql_database_system_spec = entrie.sql_database_system_spec
```

---


### Tag

Creates a tag and assigns it to: * An Entry if the method name is `projects.locations.entryGroups.entries.tags.create`. * Or EntryGroupif the method name is `projects.locations.entryGroups.tags.create`. Note: The project identified by the `parent` parameter for the [tag] (https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries.tags/create#path-parameters) and the [tag template] (https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.tagTemplates/create#path-parameters) used to create the tag must be in the same organization.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dataplex_transfer_status` | String |  | Output only. Denotes the transfer status of the Tag Template. |
| `name` | String |  | Identifier. The resource name of the tag in URL format where tag ID is a system-generated identifier. Note: The tag itself might not be stored in the location specified in its name. |
| `template_display_name` | String |  | Output only. The display name of the tag template. |
| `template` | String |  | Required. The resource name of the tag template this tag uses. Example: `projects/{PROJECT_ID}/locations/{LOCATION}/tagTemplates/{TAG_TEMPLATE_ID}` This field cannot be modified after creation. |
| `column` | String |  | Resources like entry can have schemas associated with them. This scope allows you to attach tags to an individual column based on that schema. To attach a tag to a nested column, separate column names with a dot (`.`). Example: `column.nested_column`. |
| `fields` | HashMap<String, String> |  | Required. Maps the ID of a tag field to its value and additional information about that field. Tag template defines valid field IDs. A tag must have at least 1 field and at most 500 fields. |
| `parent` | String | ✅ | Required. The name of the resource to attach this tag to. Tags can be attached to entries or entry groups. An entry can have up to 1000 attached tags. Note: The tag and its child resources might not be stored in the location specified in its name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Pagination token of the next results page. Empty if there are no more items in results. |
| `tags` | Vec<String> | Tag details. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag
tag = provider.datacatalog_api.Tag {
    parent = "value"  # Required. The name of the resource to attach this tag to. Tags can be attached to entries or entry groups. An entry can have up to 1000 attached tags. Note: The tag and its child resources might not be stored in the location specified in its name.
}

# Access tag outputs
tag_id = tag.id
tag_next_page_token = tag.next_page_token
tag_tags = tag.tags
```

---


### Enum_value

Renames an enum value in a tag template. Within a single enum field, enum values must be unique.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `new_enum_value_display_name` | String |  | Required. The new display name of the enum value. For example, `my_new_enum_value`. |
| `name` | String | ✅ | Required. The name of the enum field value. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create enum_value
enum_value = provider.datacatalog_api.Enum_value {
    name = "value"  # Required. The name of the enum field value.
}

```

---


### Entry_group

A maximum of 10,000 entry groups may be created per organization across all locations. Users should enable the Data Catalog API in the project identified by the `parent` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the entry group in URL format. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id} Note that this EntryGroup and its child resources may not actually be stored in the location in this name. |
| `description` | String |  | Entry group description, which can consist of several sentences or paragraphs that describe entry group contents. Default value is an empty string. |
| `data_catalog_timestamps` | String |  | Output only. Timestamps about this EntryGroup. Default value is empty timestamps. |
| `display_name` | String |  | A short name to identify the entry group, for example, "analytics data - jan 2011". Default value is an empty string. |
| `parent` | String | ✅ | Required. The name of the project this entry group is in. Example: * projects/{project_id}/locations/{location} Note that this EntryGroup and its child resources may not actually be stored in the location in this name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the entry group in URL format. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id} Note that this EntryGroup and its child resources may not actually be stored in the location in this name. |
| `description` | String | Entry group description, which can consist of several sentences or paragraphs that describe entry group contents. Default value is an empty string. |
| `data_catalog_timestamps` | String | Output only. Timestamps about this EntryGroup. Default value is empty timestamps. |
| `display_name` | String | A short name to identify the entry group, for example, "analytics data - jan 2011". Default value is an empty string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entry_group
entry_group = provider.datacatalog_api.Entry_group {
    parent = "value"  # Required. The name of the project this entry group is in. Example: * projects/{project_id}/locations/{location} Note that this EntryGroup and its child resources may not actually be stored in the location in this name.
}

# Access entry_group outputs
entry_group_id = entry_group.id
entry_group_name = entry_group.name
entry_group_description = entry_group.description
entry_group_data_catalog_timestamps = entry_group.data_catalog_timestamps
entry_group_display_name = entry_group.display_name
```

---


### Enum_value

Renames an enum value in a tag template. The enum values have to be unique within one enum field. Thus, an enum value cannot be renamed with a name used in any other enum value within the same enum field.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `new_enum_value_display_name` | String |  | Required. The new display name of the enum value. For example, `my_new_enum_value`. |
| `name` | String | ✅ | Required. The name of the enum field value. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}/enumValues/{enum_value_display_name} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create enum_value
enum_value = provider.datacatalog_api.Enum_value {
    name = "value"  # Required. The name of the enum field value. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}/enumValues/{enum_value_display_name}
}

```

---


### Tag

Creates a tag on an Entry. Note: The project identified by the `parent` parameter for the [tag](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.entryGroups.entries.tags/create#path-parameters) and the [tag template](https://cloud.google.com/data-catalog/docs/reference/rest/v1beta1/projects.locations.tagTemplates/create#path-parameters) used to create the tag must be from the same organization.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template` | String |  | Required. The resource name of the tag template that this tag uses. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id} This field cannot be modified after creation. |
| `fields` | HashMap<String, String> |  | Required. This maps the ID of a tag field to the value of and additional information about that field. Valid field IDs are defined by the tag's template. A tag must have at least 1 field and at most 500 fields. |
| `name` | String |  | Identifier. The resource name of the tag in URL format. Example: * projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}/tags/{tag_id} where `tag_id` is a system-generated identifier. Note that this Tag may not actually be stored in the location in this name. |
| `column` | String |  | Resources like Entry can have schemas associated with them. This scope allows users to attach tags to an individual column based on that schema. For attaching a tag to a nested column, use `.` to separate the column names. Example: * `outer_column.inner_column` |
| `template_display_name` | String |  | Output only. The display name of the tag template. |
| `parent` | String | ✅ | Required. The name of the resource to attach this tag to. Tags can be attached to Entries. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id} Note that this Tag and its child resources may not actually be stored in the location in this name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results. It is set to empty if no items remain in results. |
| `tags` | Vec<String> | Tag details. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag
tag = provider.datacatalog_api.Tag {
    parent = "value"  # Required. The name of the resource to attach this tag to. Tags can be attached to Entries. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id} Note that this Tag and its child resources may not actually be stored in the location in this name.
}

# Access tag outputs
tag_id = tag.id
tag_next_page_token = tag.next_page_token
tag_tags = tag.tags
```

---


### Field

Creates a field in a tag template. The user should enable the Data Catalog API in the project identified by the `parent` parameter (see [Data Catalog Resource Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `is_required` | bool |  | Whether this is a required field. Defaults to false. |
| `description` | String |  | The description for this field. Defaults to an empty string. |
| `type` | String |  | Required. The type of value this tag field can contain. |
| `name` | String |  | Output only. Identifier. The resource name of the tag template field in URL format. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template}/fields/{field} Note that this TagTemplateField may not actually be stored in the location in this name. |
| `display_name` | String |  | The display name for this field. Defaults to an empty string. |
| `order` | i64 |  | The order of this field with respect to other fields in this tag template. A higher value indicates a more important field. The value can be negative. Multiple fields can have the same order, and field orders within a tag do not have to be sequential. |
| `parent` | String | ✅ | Required. The name of the project and the template location [region](https://cloud.google.com/data-catalog/docs/concepts/regions). Example: * projects/{project_id}/locations/us-central1/tagTemplates/{tag_template_id} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create field
field = provider.datacatalog_api.Field {
    parent = "value"  # Required. The name of the project and the template location [region](https://cloud.google.com/data-catalog/docs/concepts/regions). Example: * projects/{project_id}/locations/us-central1/tagTemplates/{tag_template_id}
}

```

---


### Catalog

Searches Data Catalog for multiple resources like entries, tags that match a query. This is a custom method (https://cloud.google.com/apis/design/custom_methods) and does not return the complete resource, only the resource identifier and high level fields. Clients can subsequently call `Get` methods. Note that Data Catalog search queries do not guarantee full recall. Query results that match your query may not be returned, even in subsequent result pages. Also note that results returned (and not returned) can vary across repeated search queries. See [Data Catalog Search Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference) for more information.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `order_by` | String |  | Specifies the ordering of results, currently supported case-sensitive choices are: * `relevance`, only supports descending * `last_modified_timestamp [asc|desc]`, defaults to descending if not specified * `default` that can only be descending If not specified, defaults to `relevance` descending. |
| `page_token` | String |  | Optional. Pagination token returned in an earlier SearchCatalogResponse.next_page_token, which indicates that this is a continuation of a prior SearchCatalogRequest call, and that the system should return the next page of data. If empty, the first page is returned. |
| `page_size` | i64 |  | Number of results in the search page. If <=0 then defaults to 10. Max limit for page_size is 1000. Throws an invalid argument for page_size > 1000. |
| `query` | String |  | Optional. The query string in search query syntax. An empty query string will result in all data assets (in the specified scope) that the user has access to. Query strings can be simple as "x" or more qualified as: * name:x * column:x * description:y Note: Query tokens need to have a minimum of 3 characters for substring matching to work correctly. See [Data Catalog Search Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference) for more information. |
| `scope` | String |  | Required. The scope of this search request. A `scope` that has empty `include_org_ids`, `include_project_ids` AND false `include_gcp_public_datasets` is considered invalid. Data Catalog will return an error in such a case. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create catalog
catalog = provider.datacatalog_api.Catalog {
}

```

---


### Entrie

Creates an entry. Only entries of 'FILESET' type or user-specified type can be created. Users should enable the Data Catalog API in the project identified by the `parent` parameter (see [Data Catalog Resource Project] (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information). A maximum of 100,000 entries may be created per entry group.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bigquery_date_sharded_spec` | String |  | Specification for a group of BigQuery tables with name pattern `[prefix]YYYYMMDD`. Context: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding. |
| `linked_resource` | String |  | The resource this metadata entry refers to. For Google Cloud Platform resources, `linked_resource` is the [full name of the resource](https://cloud.google.com/apis/design/resource_names#full_resource_name). For example, the `linked_resource` for a table resource from BigQuery is: * //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId Output only when Entry is of type in the EntryType enum. For entries with user_specified_type, this field is optional and defaults to an empty string. |
| `type` | String |  | The type of the entry. Only used for Entries with types in the EntryType enum. |
| `user_specified_system` | String |  | This field indicates the entry's source system that Data Catalog does not integrate with. `user_specified_system` strings must begin with a letter or underscore and can only contain letters, numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long. |
| `bigquery_table_spec` | String |  | Specification that applies to a BigQuery table. This is only valid on entries of type `TABLE`. |
| `integrated_system` | String |  | Output only. This field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub. |
| `name` | String |  | Output only. Identifier. The Data Catalog resource name of the entry in URL format. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id} Note that this Entry and its child resources may not actually be stored in the location in this name. |
| `usage_signal` | String |  | Output only. Statistics on the usage level of the resource. |
| `display_name` | String |  | Display information such as title and description. A short name to identify the entry, for example, "Analytics Data - Jan 2011". Default value is an empty string. |
| `schema` | String |  | Schema of the entry. An entry might not have any schema attached to it. |
| `source_system_timestamps` | String |  | Output only. Timestamps about the underlying resource, not about this Data Catalog entry. Output only when Entry is of type in the EntryType enum. For entries with user_specified_type, this field is optional and defaults to an empty timestamp. |
| `user_specified_type` | String |  | Entry type if it does not fit any of the input-allowed values listed in `EntryType` enum above. When creating an entry, users should check the enum values first, if nothing matches the entry to be created, then provide a custom value, for example "my_special_type". `user_specified_type` strings must begin with a letter or underscore and can only contain letters, numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long. Currently, only FILESET enum value is allowed. All other entries created through Data Catalog must use `user_specified_type`. |
| `gcs_fileset_spec` | String |  | Specification that applies to a Cloud Storage fileset. This is only valid on entries of type FILESET. |
| `description` | String |  | Entry description, which can consist of several sentences or paragraphs that describe entry contents. Default value is an empty string. |
| `parent` | String | ✅ | Required. The name of the entry group this entry is in. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id} Note that this Entry and its child resources may not actually be stored in the location in this name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bigquery_date_sharded_spec` | String | Specification for a group of BigQuery tables with name pattern `[prefix]YYYYMMDD`. Context: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding. |
| `linked_resource` | String | The resource this metadata entry refers to. For Google Cloud Platform resources, `linked_resource` is the [full name of the resource](https://cloud.google.com/apis/design/resource_names#full_resource_name). For example, the `linked_resource` for a table resource from BigQuery is: * //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId Output only when Entry is of type in the EntryType enum. For entries with user_specified_type, this field is optional and defaults to an empty string. |
| `type` | String | The type of the entry. Only used for Entries with types in the EntryType enum. |
| `user_specified_system` | String | This field indicates the entry's source system that Data Catalog does not integrate with. `user_specified_system` strings must begin with a letter or underscore and can only contain letters, numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long. |
| `bigquery_table_spec` | String | Specification that applies to a BigQuery table. This is only valid on entries of type `TABLE`. |
| `integrated_system` | String | Output only. This field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub. |
| `name` | String | Output only. Identifier. The Data Catalog resource name of the entry in URL format. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id} Note that this Entry and its child resources may not actually be stored in the location in this name. |
| `usage_signal` | String | Output only. Statistics on the usage level of the resource. |
| `display_name` | String | Display information such as title and description. A short name to identify the entry, for example, "Analytics Data - Jan 2011". Default value is an empty string. |
| `schema` | String | Schema of the entry. An entry might not have any schema attached to it. |
| `source_system_timestamps` | String | Output only. Timestamps about the underlying resource, not about this Data Catalog entry. Output only when Entry is of type in the EntryType enum. For entries with user_specified_type, this field is optional and defaults to an empty timestamp. |
| `user_specified_type` | String | Entry type if it does not fit any of the input-allowed values listed in `EntryType` enum above. When creating an entry, users should check the enum values first, if nothing matches the entry to be created, then provide a custom value, for example "my_special_type". `user_specified_type` strings must begin with a letter or underscore and can only contain letters, numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long. Currently, only FILESET enum value is allowed. All other entries created through Data Catalog must use `user_specified_type`. |
| `gcs_fileset_spec` | String | Specification that applies to a Cloud Storage fileset. This is only valid on entries of type FILESET. |
| `description` | String | Entry description, which can consist of several sentences or paragraphs that describe entry contents. Default value is an empty string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entrie
entrie = provider.datacatalog_api.Entrie {
    parent = "value"  # Required. The name of the entry group this entry is in. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id} Note that this Entry and its child resources may not actually be stored in the location in this name.
}

# Access entrie outputs
entrie_id = entrie.id
entrie_bigquery_date_sharded_spec = entrie.bigquery_date_sharded_spec
entrie_linked_resource = entrie.linked_resource
entrie_type = entrie.type
entrie_user_specified_system = entrie.user_specified_system
entrie_bigquery_table_spec = entrie.bigquery_table_spec
entrie_integrated_system = entrie.integrated_system
entrie_name = entrie.name
entrie_usage_signal = entrie.usage_signal
entrie_display_name = entrie.display_name
entrie_schema = entrie.schema
entrie_source_system_timestamps = entrie.source_system_timestamps
entrie_user_specified_type = entrie.user_specified_type
entrie_gcs_fileset_spec = entrie.gcs_fileset_spec
entrie_description = entrie.description
```

---


### Policy_tag

Creates a policy tag in the specified taxonomy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent_policy_tag` | String |  | Resource name of this policy tag's parent policy tag (e.g. for the "LatLong" policy tag in the example above, this field contains the resource name of the "Geolocation" policy tag). If empty, it means this policy tag is a top level policy tag (e.g. this field is empty for the "Geolocation" policy tag in the example above). If not set, defaults to an empty string. |
| `description` | String |  | Description of this policy tag. It must: contain only unicode characters, tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes long when encoded in UTF-8. If not set, defaults to an empty description. If not set, defaults to an empty description. |
| `child_policy_tags` | Vec<String> |  | Output only. Resource names of child policy tags of this policy tag. |
| `display_name` | String |  | Required. User defined name of this policy tag. It must: be unique within the parent taxonomy; contain only unicode letters, numbers, underscores, dashes and spaces; not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8. |
| `name` | String |  | Identifier. Resource name of this policy tag, whose format is: "projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{id}". |
| `parent` | String | ✅ | Required. Resource name of the taxonomy that the policy tag will belong to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parent_policy_tag` | String | Resource name of this policy tag's parent policy tag (e.g. for the "LatLong" policy tag in the example above, this field contains the resource name of the "Geolocation" policy tag). If empty, it means this policy tag is a top level policy tag (e.g. this field is empty for the "Geolocation" policy tag in the example above). If not set, defaults to an empty string. |
| `description` | String | Description of this policy tag. It must: contain only unicode characters, tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes long when encoded in UTF-8. If not set, defaults to an empty description. If not set, defaults to an empty description. |
| `child_policy_tags` | Vec<String> | Output only. Resource names of child policy tags of this policy tag. |
| `display_name` | String | Required. User defined name of this policy tag. It must: be unique within the parent taxonomy; contain only unicode letters, numbers, underscores, dashes and spaces; not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8. |
| `name` | String | Identifier. Resource name of this policy tag, whose format is: "projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{id}". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policy_tag
policy_tag = provider.datacatalog_api.Policy_tag {
    parent = "value"  # Required. Resource name of the taxonomy that the policy tag will belong to.
}

# Access policy_tag outputs
policy_tag_id = policy_tag.id
policy_tag_parent_policy_tag = policy_tag.parent_policy_tag
policy_tag_description = policy_tag.description
policy_tag_child_policy_tags = policy_tag.child_policy_tags
policy_tag_display_name = policy_tag.display_name
policy_tag_name = policy_tag.name
```

---


### Tag_template

Creates a tag template. The user should enable the Data Catalog API in the project identified by the `parent` parameter (see [Data Catalog Resource Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project) for more information).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The display name for this template. Defaults to an empty string. |
| `name` | String |  | Identifier. The resource name of the tag template in URL format. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id} Note that this TagTemplate and its child resources may not actually be stored in the location in this name. |
| `fields` | HashMap<String, String> |  | Required. Map of tag template field IDs to the settings for the field. This map is an exhaustive list of the allowed fields. This map must contain at least one field and at most 500 fields. The keys to this map are tag template field IDs. Field IDs can contain letters (both uppercase and lowercase), numbers (0-9) and underscores (_). Field IDs must be at least 1 character long and at most 64 characters long. Field IDs must start with a letter or underscore. |
| `dataplex_transfer_status` | String |  | Output only. Transfer status of the TagTemplate |
| `parent` | String | ✅ | Required. The name of the project and the template location [region](https://cloud.google.com/data-catalog/docs/concepts/regions. Example: * projects/{project_id}/locations/us-central1 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The display name for this template. Defaults to an empty string. |
| `name` | String | Identifier. The resource name of the tag template in URL format. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id} Note that this TagTemplate and its child resources may not actually be stored in the location in this name. |
| `fields` | HashMap<String, String> | Required. Map of tag template field IDs to the settings for the field. This map is an exhaustive list of the allowed fields. This map must contain at least one field and at most 500 fields. The keys to this map are tag template field IDs. Field IDs can contain letters (both uppercase and lowercase), numbers (0-9) and underscores (_). Field IDs must be at least 1 character long and at most 64 characters long. Field IDs must start with a letter or underscore. |
| `dataplex_transfer_status` | String | Output only. Transfer status of the TagTemplate |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag_template
tag_template = provider.datacatalog_api.Tag_template {
    parent = "value"  # Required. The name of the project and the template location [region](https://cloud.google.com/data-catalog/docs/concepts/regions. Example: * projects/{project_id}/locations/us-central1
}

# Access tag_template outputs
tag_template_id = tag_template.id
tag_template_display_name = tag_template.display_name
tag_template_name = tag_template.name
tag_template_fields = tag_template.fields
tag_template_dataplex_transfer_status = tag_template.dataplex_transfer_status
```

---


### Taxonomie

Creates a taxonomy in the specified project.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service` | String |  | Output only. Identity of the service which owns the Taxonomy. This field is only populated when the taxonomy is created by a Google Cloud service. Currently only 'DATAPLEX' is supported. |
| `display_name` | String |  | Required. User defined name of this taxonomy. It must: contain only unicode letters, numbers, underscores, dashes and spaces; not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8. The taxonomy display name must be unique within an organization. |
| `taxonomy_timestamps` | String |  | Output only. Timestamps about this taxonomy. Only create_time and update_time are used. |
| `description` | String |  | Optional. Description of this taxonomy. It must: contain only unicode characters, tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes long when encoded in UTF-8. If not set, defaults to an empty description. |
| `activated_policy_types` | Vec<String> |  | Optional. A list of policy types that are activated for this taxonomy. If not set, defaults to an empty list. |
| `name` | String |  | Identifier. Resource name of this taxonomy, whose format is: "projects/{project_number}/locations/{location_id}/taxonomies/{id}". |
| `policy_tag_count` | i64 |  | Output only. Number of policy tags contained in this taxonomy. |
| `parent` | String | ✅ | Required. Resource name of the project that the taxonomy will belong to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service` | String | Output only. Identity of the service which owns the Taxonomy. This field is only populated when the taxonomy is created by a Google Cloud service. Currently only 'DATAPLEX' is supported. |
| `display_name` | String | Required. User defined name of this taxonomy. It must: contain only unicode letters, numbers, underscores, dashes and spaces; not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8. The taxonomy display name must be unique within an organization. |
| `taxonomy_timestamps` | String | Output only. Timestamps about this taxonomy. Only create_time and update_time are used. |
| `description` | String | Optional. Description of this taxonomy. It must: contain only unicode characters, tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes long when encoded in UTF-8. If not set, defaults to an empty description. |
| `activated_policy_types` | Vec<String> | Optional. A list of policy types that are activated for this taxonomy. If not set, defaults to an empty list. |
| `name` | String | Identifier. Resource name of this taxonomy, whose format is: "projects/{project_number}/locations/{location_id}/taxonomies/{id}". |
| `policy_tag_count` | i64 | Output only. Number of policy tags contained in this taxonomy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create taxonomie
taxonomie = provider.datacatalog_api.Taxonomie {
    parent = "value"  # Required. Resource name of the project that the taxonomy will belong to.
}

# Access taxonomie outputs
taxonomie_id = taxonomie.id
taxonomie_service = taxonomie.service
taxonomie_display_name = taxonomie.display_name
taxonomie_taxonomy_timestamps = taxonomie.taxonomy_timestamps
taxonomie_description = taxonomie.description
taxonomie_activated_policy_types = taxonomie.activated_policy_types
taxonomie_name = taxonomie.name
taxonomie_policy_tag_count = taxonomie.policy_tag_count
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple tag_template resources
tag_template_0 = provider.datacatalog_api.Tag_template {
    parent = "value-0"
}
tag_template_1 = provider.datacatalog_api.Tag_template {
    parent = "value-1"
}
tag_template_2 = provider.datacatalog_api.Tag_template {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    tag_template = provider.datacatalog_api.Tag_template {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Datacatalog_api Documentation](https://cloud.google.com/datacatalog_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
