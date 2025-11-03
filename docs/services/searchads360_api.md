# Searchads360_api Service



**Resources**: 4

---

## Overview

The searchads360_api service provides access to 4 resource types:

- [Customer](#customer) [R]
- [Search_ads360](#search_ads360) [C]
- [Search_ads360_field](#search_ads360_field) [CR]
- [Custom_column](#custom_column) [R]

---

## Resources


### Customer

Returns resource names of customers directly accessible by the user authenticating the call. List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QuotaError]() [RequestError]()

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_names` | Vec<String> | Resource name of customers directly accessible by the user authenticating the call. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access customer outputs
customer_id = customer.id
customer_resource_names = customer.resource_names
```

---


### Search_ads360

Returns all rows that match the search query. List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QueryError]() [QuotaError]() [RequestError]()

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `validate_only` | bool |  | If true, the request is validated but not executed. |
| `query` | String |  | Required. The query string. |
| `page_size` | i64 |  | Number of elements to retrieve in a single page. When too large a page is requested, the server may decide to further limit the number of returned resources. |
| `page_token` | String |  | Token of the page to retrieve. If not specified, the first page of results will be returned. Use the value obtained from `next_page_token` in the previous response in order to request the next page of results. |
| `return_total_results_count` | bool |  | If true, the total number of results that match the query ignoring the LIMIT clause will be included in the response. Default is false. |
| `summary_row_setting` | String |  | Determines whether a summary row will be returned. By default, summary row is not returned. If requested, the summary row will be sent in a response by itself after all other query results are returned. |
| `customer_id` | String | ✅ | Required. The ID of the customer being queried. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create search_ads360
search_ads360 = provider.searchads360_api.Search_ads360 {
    customer_id = "value"  # Required. The ID of the customer being queried.
}

```

---


### Search_ads360_field

Returns all fields that match the search [query](/search-ads/reporting/concepts/field-service#use_a_query_to_get_field_details). List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QueryError]() [QuotaError]() [RequestError]()

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_token` | String |  | Token of the page to retrieve. If not specified, the first page of results will be returned. Use the value obtained from `next_page_token` in the previous response in order to request the next page of results. |
| `query` | String |  | Required. The query string. |
| `page_size` | i64 |  | Number of elements to retrieve in a single page. When too large a page is requested, the server may decide to further limit the number of returned resources. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `segments` | Vec<String> | Output only. This field lists the names of all artifacts, whether a segment or another resource, that segment metrics when included in search queries and when the described artifact is used in the FROM clause. It is only set for artifacts whose category is RESOURCE. |
| `selectable` | bool | Output only. Whether the artifact can be used in a SELECT clause in search queries. |
| `attribute_resources` | Vec<String> | Output only. The names of all resources that are selectable with the described artifact. Fields from these resources do not segment metrics when included in search queries. This field is only set for artifacts whose category is RESOURCE. |
| `resource_name` | String | Output only. The resource name of the artifact. Artifact resource names have the form: `SearchAds360Fields/{name}` |
| `is_repeated` | bool | Output only. Whether the field artifact is repeated. |
| `enum_values` | Vec<String> | Output only. Values the artifact can assume if it is a field of type ENUM. This field is only set for artifacts of category SEGMENT or ATTRIBUTE. |
| `sortable` | bool | Output only. Whether the artifact can be used in a ORDER BY clause in search queries. |
| `name` | String | Output only. The name of the artifact. |
| `filterable` | bool | Output only. Whether the artifact can be used in a WHERE clause in search queries. |
| `metrics` | Vec<String> | Output only. This field lists the names of all metrics that are selectable with the described artifact when it is used in the FROM clause. It is only set for artifacts whose category is RESOURCE. |
| `category` | String | Output only. The category of the artifact. |
| `type_url` | String | Output only. The URL of proto describing the artifact's data type. |
| `selectable_with` | Vec<String> | Output only. The names of all resources, segments, and metrics that are selectable with the described artifact. |
| `data_type` | String | Output only. This field determines the operators that can be used with the artifact in WHERE clauses. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create search_ads360_field
search_ads360_field = provider.searchads360_api.Search_ads360_field {
}

# Access search_ads360_field outputs
search_ads360_field_id = search_ads360_field.id
search_ads360_field_segments = search_ads360_field.segments
search_ads360_field_selectable = search_ads360_field.selectable
search_ads360_field_attribute_resources = search_ads360_field.attribute_resources
search_ads360_field_resource_name = search_ads360_field.resource_name
search_ads360_field_is_repeated = search_ads360_field.is_repeated
search_ads360_field_enum_values = search_ads360_field.enum_values
search_ads360_field_sortable = search_ads360_field.sortable
search_ads360_field_name = search_ads360_field.name
search_ads360_field_filterable = search_ads360_field.filterable
search_ads360_field_metrics = search_ads360_field.metrics
search_ads360_field_category = search_ads360_field.category
search_ads360_field_type_url = search_ads360_field.type_url
search_ads360_field_selectable_with = search_ads360_field.selectable_with
search_ads360_field_data_type = search_ads360_field.data_type
```

---


### Custom_column

Returns the requested custom column in full detail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `queryable` | bool | Output only. True when the custom column is available to be used in the query of SearchAds360Service.Search and SearchAds360Service.SearchStream. |
| `references_attributes` | bool | Output only. True when the custom column is referring to one or more attributes. |
| `referenced_system_columns` | Vec<String> | Output only. The list of the referenced system columns of this custom column. For example, A custom column "sum of impressions and clicks" has referenced system columns of {"metrics.clicks", "metrics.impressions"}. |
| `render_type` | String | Output only. How the result value of the custom column should be interpreted. |
| `id` | String | Output only. ID of the custom column. |
| `references_metrics` | bool | Output only. True when the custom column is referring to one or more metrics. |
| `name` | String | Output only. User-defined name of the custom column. |
| `description` | String | Output only. User-defined description of the custom column. |
| `resource_name` | String | Immutable. The resource name of the custom column. Custom column resource names have the form: `customers/{customer_id}/customColumns/{custom_column_id}` |
| `value_type` | String | Output only. The type of the result value of the custom column. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access custom_column outputs
custom_column_id = custom_column.id
custom_column_queryable = custom_column.queryable
custom_column_references_attributes = custom_column.references_attributes
custom_column_referenced_system_columns = custom_column.referenced_system_columns
custom_column_render_type = custom_column.render_type
custom_column_id = custom_column.id
custom_column_references_metrics = custom_column.references_metrics
custom_column_name = custom_column.name
custom_column_description = custom_column.description
custom_column_resource_name = custom_column.resource_name
custom_column_value_type = custom_column.value_type
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple customer resources
customer_0 = provider.searchads360_api.Customer {
}
customer_1 = provider.searchads360_api.Customer {
}
customer_2 = provider.searchads360_api.Customer {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    customer = provider.searchads360_api.Customer {
    }
```

---

## Related Documentation

- [GCP Searchads360_api Documentation](https://cloud.google.com/searchads360_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
