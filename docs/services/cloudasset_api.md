# Cloudasset_api Service



**Resources**: 16

---

## Overview

The cloudasset_api service provides access to 16 resource types:

- [Operation](#operation) [R]
- [Saved_querie](#saved_querie) [CRUD]
- [Feed](#feed) [CRUD]
- [Asset](#asset) [R]
- [Effective_iam_policie](#effective_iam_policie) [R]
- [Cloudasset](#cloudasset) [CR]
- [Asset](#asset) [R]
- [Cloudasset](#cloudasset) [CR]
- [Cloudasset](#cloudasset) [C]
- [Operation](#operation) [R]
- [Operation](#operation) [R]
- [Organization](#organization) [CR]
- [Project](#project) [CR]
- [Folder](#folder) [C]
- [Resource](#resource) [R]
- [Iam_policie](#iam_policie) [R]

---

## Resources


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
```

---


### Saved_querie

Creates a saved query in a parent project/folder/organization.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `last_updater` | String |  | Output only. The account's email address who has updated this saved query most recently. |
| `last_update_time` | String |  | Output only. The last update time of this saved query. |
| `create_time` | String |  | Output only. The create time of this saved query. |
| `content` | String |  | The query content. |
| `creator` | String |  | Output only. The account's email address who has created this saved query. |
| `name` | String |  | The resource name of the saved query. The format must be: * projects/project_number/savedQueries/saved_query_id * folders/folder_number/savedQueries/saved_query_id * organizations/organization_number/savedQueries/saved_query_id |
| `description` | String |  | The description of this saved query. This value should be fewer than 255 characters. |
| `labels` | HashMap<String, String> |  | Labels applied on the resource. This value should not contain more than 10 entries. The key and value of each entry must be non-empty and fewer than 64 characters. |
| `parent` | String | ✅ | Required. The name of the project/folder/organization where this saved_query should be created in. It can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_updater` | String | Output only. The account's email address who has updated this saved query most recently. |
| `last_update_time` | String | Output only. The last update time of this saved query. |
| `create_time` | String | Output only. The create time of this saved query. |
| `content` | String | The query content. |
| `creator` | String | Output only. The account's email address who has created this saved query. |
| `name` | String | The resource name of the saved query. The format must be: * projects/project_number/savedQueries/saved_query_id * folders/folder_number/savedQueries/saved_query_id * organizations/organization_number/savedQueries/saved_query_id |
| `description` | String | The description of this saved query. This value should be fewer than 255 characters. |
| `labels` | HashMap<String, String> | Labels applied on the resource. This value should not contain more than 10 entries. The key and value of each entry must be non-empty and fewer than 64 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create saved_querie
saved_querie = provider.cloudasset_api.Saved_querie {
    parent = "value"  # Required. The name of the project/folder/organization where this saved_query should be created in. It can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345").
}

# Access saved_querie outputs
saved_querie_id = saved_querie.id
saved_querie_last_updater = saved_querie.last_updater
saved_querie_last_update_time = saved_querie.last_update_time
saved_querie_create_time = saved_querie.create_time
saved_querie_content = saved_querie.content
saved_querie_creator = saved_querie.creator
saved_querie_name = saved_querie.name
saved_querie_description = saved_querie.description
saved_querie_labels = saved_querie.labels
```

---


### Feed

Creates a feed in a parent project/folder/organization to listen to its asset updates.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `feed` | String |  | Required. The feed details. The field `name` must be empty and it will be generated in the format of: projects/project_number/feeds/feed_id folders/folder_number/feeds/feed_id organizations/organization_number/feeds/feed_id |
| `feed_id` | String |  | Required. This is the client-assigned asset feed identifier and it needs to be unique under a specific parent project/folder/organization. |
| `parent` | String | ✅ | Required. The name of the project/folder/organization where this feed should be created in. It can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `condition` | String | A condition which determines whether an asset update should be published. If specified, an asset will be returned only when the expression evaluates to true. When set, `expression` field in the `Expr` must be a valid [CEL expression] (https://github.com/google/cel-spec) on a TemporalAsset with name `temporal_asset`. Example: a Feed with expression ("temporal_asset.deleted == true") will only publish Asset deletions. Other fields of `Expr` are optional. See our [user guide](https://cloud.google.com/asset-inventory/docs/monitoring-asset-changes-with-condition) for detailed instructions. |
| `asset_types` | Vec<String> | A list of types of the assets to receive updates. You must specify either or both of asset_names and asset_types. Only asset updates matching specified asset_names or asset_types are exported to the feed. Example: `"compute.googleapis.com/Disk"` For a list of all supported asset types, see [Supported asset types](/asset-inventory/docs/supported-asset-types). |
| `name` | String | Required. The format will be projects/{project_number}/feeds/{client-assigned_feed_identifier} or folders/{folder_number}/feeds/{client-assigned_feed_identifier} or organizations/{organization_number}/feeds/{client-assigned_feed_identifier} The client-assigned feed identifier must be unique within the parent project/folder/organization. |
| `asset_names` | Vec<String> | A list of the full names of the assets to receive updates. You must specify either or both of asset_names and asset_types. Only asset updates matching specified asset_names or asset_types are exported to the feed. Example: `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`. For a list of the full names for supported asset types, see [Resource name format](/asset-inventory/docs/resource-name-format). |
| `feed_output_config` | String | Required. Feed output configuration defining where the asset updates are published to. |
| `content_type` | String | Asset content type. If not specified, no content but the asset name and type will be returned. |
| `relationship_types` | Vec<String> | A list of relationship types to output, for example: `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if content_type=RELATIONSHIP. * If specified: it outputs specified relationship updates on the [asset_names] or the [asset_types]. It returns an error if any of the [relationship_types] doesn't belong to the supported relationship types of the [asset_names] or [asset_types], or any of the [asset_names] or the [asset_types] doesn't belong to the source types of the [relationship_types]. * Otherwise: it outputs the supported relationships of the types of [asset_names] and [asset_types] or returns an error if any of the [asset_names] or the [asset_types] has no replationship support. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types and relationship types. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feed
feed = provider.cloudasset_api.Feed {
    parent = "value"  # Required. The name of the project/folder/organization where this feed should be created in. It can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345").
}

# Access feed outputs
feed_id = feed.id
feed_condition = feed.condition
feed_asset_types = feed.asset_types
feed_name = feed.name
feed_asset_names = feed.asset_names
feed_feed_output_config = feed.feed_output_config
feed_content_type = feed.content_type
feed_relationship_types = feed.relationship_types
```

---


### Asset

Lists assets with time and resource types and returns paged results in response.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `read_time` | String | Time the snapshot was taken. |
| `next_page_token` | String | Token to retrieve the next page of results. It expires 72 hours after the page token for the first page is generated. Set to empty if there are no remaining results. |
| `assets` | Vec<String> | Assets. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access asset outputs
asset_id = asset.id
asset_read_time = asset.read_time
asset_next_page_token = asset.next_page_token
asset_assets = asset.assets
```

---


### Effective_iam_policie

Gets effective IAM policies for a batch of resources.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `policy_results` | Vec<String> | The effective policies for a batch of resources. Note that the results order is the same as the order of BatchGetEffectiveIamPoliciesRequest.names. When a resource does not have any effective IAM policies, its corresponding policy_result will contain empty EffectiveIamPolicy.policies. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access effective_iam_policie outputs
effective_iam_policie_id = effective_iam_policie.id
effective_iam_policie_policy_results = effective_iam_policie.policy_results
```

---


### Cloudasset

Issue a job that queries assets using a SQL statement compatible with [BigQuery SQL](https://cloud.google.com/bigquery/docs/introduction-sql). If the query execution finishes within timeout and there's no pagination, the full query results will be returned in the `QueryAssetsResponse`. Otherwise, full query results can be obtained by issuing extra requests with the `job_reference` from the a previous `QueryAssets` call. Note, the query result has approximately 10 GB limitation enforced by [BigQuery](https://cloud.google.com/bigquery/docs/best-practices-performance-output). Queries return larger results will result in errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_token` | String |  | Optional. A page token received from previous `QueryAssets`. The field will be ignored when [output_config] is specified. |
| `read_time_window` | String |  | Optional. [start_time] is required. [start_time] must be less than [end_time] Defaults [end_time] to now if [start_time] is set and [end_time] isn't. Maximum permitted time range is 7 days. |
| `statement` | String |  | Optional. A SQL statement that's compatible with [BigQuery SQL](https://cloud.google.com/bigquery/docs/introduction-sql). |
| `page_size` | i64 |  | Optional. The maximum number of rows to return in the results. Responses are limited to 10 MB and 1000 rows. By default, the maximum row count is 1000. When the byte or row count limit is reached, the rest of the query results will be paginated. The field will be ignored when [output_config] is specified. |
| `read_time` | String |  | Optional. Queries cloud assets as they appeared at the specified point in time. |
| `timeout` | String |  | Optional. Specifies the maximum amount of time that the client is willing to wait for the query to complete. By default, this limit is 5 min for the first query, and 1 minute for the following queries. If the query is complete, the `done` field in the `QueryAssetsResponse` is true, otherwise false. Like BigQuery [jobs.query API](https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/query#queryrequest) The call is not guaranteed to wait for the specified timeout; it typically returns after around 200 seconds (200,000 milliseconds), even if the query is not complete. The field will be ignored when [output_config] is specified. |
| `output_config` | String |  | Optional. Destination where the query results will be saved. When this field is specified, the query results won't be saved in the [QueryAssetsResponse.query_result]. Instead [QueryAssetsResponse.output_config] will be set. Meanwhile, [QueryAssetsResponse.job_reference] will be set and can be used to check the status of the query job when passed to a following [QueryAssets] API call. |
| `job_reference` | String |  | Optional. Reference to the query job, which is from the `QueryAssetsResponse` of previous `QueryAssets` call. |
| `parent` | String | ✅ | Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"), or a folder number (such as "folders/123"). Only assets belonging to the `parent` will be returned. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `results` | Vec<String> | A list of Resources that match the search query. It contains the resource standard metadata information. |
| `next_page_token` | String | If there are more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cloudasset
cloudasset = provider.cloudasset_api.Cloudasset {
    parent = "value"  # Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"), or a folder number (such as "folders/123"). Only assets belonging to the `parent` will be returned.
}

# Access cloudasset outputs
cloudasset_id = cloudasset.id
cloudasset_results = cloudasset.results
cloudasset_next_page_token = cloudasset.next_page_token
```

---


### Asset

Lists assets with time and resource types and returns paged results in response.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assets` | Vec<String> | Assets. |
| `next_page_token` | String | Token to retrieve the next page of results. It expires 72 hours after the page token for the first page is generated. Set to empty if there are no remaining results. |
| `read_time` | String | Time the snapshot was taken. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access asset outputs
asset_id = asset.id
asset_assets = asset.assets
asset_next_page_token = asset.next_page_token
asset_read_time = asset.read_time
```

---


### Cloudasset

Exports the answers of which identities have what accesses on which resources to a Google Cloud Storage destination. The output format is the JSON format that represents a AnalyzeIamPolicyResponse in the JSON format. This method implements the google.longrunning.Operation, which allows you to keep track of the export. We recommend intervals of at least 2 seconds with exponential retry to poll the export operation result. The metadata contains the request to help callers to map responses to requests.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `options` | String |  | Optional. The request options. |
| `output_config` | String |  | Required. Output configuration indicating where the results will be output to. |
| `analysis_query` | String |  | Required. The request query. |
| `parent` | String | ✅ | Required. The relative name of the root asset. Only resources and IAM policies within the parent will be analyzed. This can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"). To know how to get organization id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id). To know how to get folder or project id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `main_analysis` | String | The main analysis that matches the original request. |
| `fully_explored` | bool | Represents whether all entries in the main_analysis and service_account_impersonation_analysis have been fully explored to answer the query in the request. |
| `service_account_impersonation_analysis` | Vec<String> | The service account impersonation analysis if AnalyzeIamPolicyRequest.analyze_service_account_impersonation is enabled. |
| `non_critical_errors` | Vec<String> | A list of non-critical errors happened during the request handling to explain why `fully_explored` is false, or empty if no error happened. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cloudasset
cloudasset = provider.cloudasset_api.Cloudasset {
    parent = "value"  # Required. The relative name of the root asset. Only resources and IAM policies within the parent will be analyzed. This can only be an organization number (such as "organizations/123"), a folder number (such as "folders/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"). To know how to get organization id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id). To know how to get folder or project id, visit [here ](https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects).
}

# Access cloudasset outputs
cloudasset_id = cloudasset.id
cloudasset_main_analysis = cloudasset.main_analysis
cloudasset_fully_explored = cloudasset.fully_explored
cloudasset_service_account_impersonation_analysis = cloudasset.service_account_impersonation_analysis
cloudasset_non_critical_errors = cloudasset.non_critical_errors
```

---


### Cloudasset

Exports assets with time and resource types to a given Cloud Storage location/BigQuery table. For Cloud Storage location destinations, the output format is newline-delimited JSON. Each line represents a google.cloud.asset.v1p7beta1.Asset in the JSON format; for BigQuery table destinations, the output table stores the fields in asset proto as columns. This API implements the google.longrunning.Operation API , which allows you to keep track of the export. We recommend intervals of at least 2 seconds with exponential retry to poll the export operation result. For regular-size resource parent, the export operation usually finishes within 5 minutes.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `relationship_types` | Vec<String> |  | A list of relationship types to export, for example: `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if content_type=RELATIONSHIP. If specified, it will snapshot [asset_types]' specified relationships, or give errors if any relationship_types' supported types are not in [asset_types]. If not specified, it will snapshot all [asset_types]' supported relationships. An unspecified [asset_types] field means all supported asset_types. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types and relationship types. |
| `content_type` | String |  | Asset content type. If not specified, no content but the asset name will be returned. |
| `output_config` | String |  | Required. Output configuration indicating where the results will be output to. |
| `asset_types` | Vec<String> |  | A list of asset types to take a snapshot for. For example: "compute.googleapis.com/Disk". Regular expressions are also supported. For example: * "compute.googleapis.com.*" snapshots resources whose asset type starts with "compute.googleapis.com". * ".*Instance" snapshots resources whose asset type ends with "Instance". * ".*Instance.*" snapshots resources whose asset type contains "Instance". See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported regular expression syntax. If the regular expression does not match any supported asset type, an INVALID_ARGUMENT error will be returned. If specified, only matching assets will be returned, otherwise, it will snapshot all asset types. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/asset-inventory/docs/overview) for all supported asset types. |
| `read_time` | String |  | Timestamp to take an asset snapshot. This can only be set to a timestamp between the current time and the current time minus 35 days (inclusive). If not specified, the current time will be used. Due to delays in resource data collection and indexing, there is a volatile window during which running the same query may get different results. |
| `parent` | String | ✅ | Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"), or a folder number (such as "folders/123"). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cloudasset
cloudasset = provider.cloudasset_api.Cloudasset {
    parent = "value"  # Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), or a project number (such as "projects/12345"), or a folder number (such as "folders/123").
}

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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


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
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
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

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_response = operation.response
operation_name = operation.name
operation_metadata = operation.metadata
```

---


### Organization

Exports assets with time and resource types to a given Cloud Storage location. The output format is newline-delimited JSON. This API implements the google.longrunning.Operation API allowing you to keep track of the export. We recommend intervals of at least 2 seconds with exponential retry to poll the export operation result. For regular-size resource parent, the export operation usually finishes within 5 minutes.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_types` | Vec<String> |  | A list of asset types of which to take a snapshot for. For example: "google.compute.Disk". If specified, only matching assets will be returned. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/resource-manager/docs/cloud-asset-inventory/overview) for all supported asset types. |
| `read_time` | String |  | Timestamp to take an asset snapshot. This can only be set to a timestamp between 2018-10-02 UTC (inclusive) and the current time. If not specified, the current time will be used. Due to delays in resource data collection and indexing, there is a volatile window during which running the same query may get different results. |
| `content_type` | String |  | Asset content type. If not specified, no content but the asset name will be returned. |
| `output_config` | String |  | Required. Output configuration indicating where the results will be output to. All results will be in newline delimited JSON format. |
| `parent` | String | ✅ | Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), a project number (such as "projects/12345"), or a folder number (such as "folders/123"). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assets` | Vec<String> | A list of assets with valid time windows. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create organization
organization = provider.cloudasset_api.Organization {
    parent = "value"  # Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), a project number (such as "projects/12345"), or a folder number (such as "folders/123").
}

# Access organization outputs
organization_id = organization.id
organization_assets = organization.assets
```

---


### Project

Exports assets with time and resource types to a given Cloud Storage location. The output format is newline-delimited JSON. This API implements the google.longrunning.Operation API allowing you to keep track of the export. We recommend intervals of at least 2 seconds with exponential retry to poll the export operation result. For regular-size resource parent, the export operation usually finishes within 5 minutes.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_types` | Vec<String> |  | A list of asset types of which to take a snapshot for. For example: "google.compute.Disk". If specified, only matching assets will be returned. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/resource-manager/docs/cloud-asset-inventory/overview) for all supported asset types. |
| `read_time` | String |  | Timestamp to take an asset snapshot. This can only be set to a timestamp between 2018-10-02 UTC (inclusive) and the current time. If not specified, the current time will be used. Due to delays in resource data collection and indexing, there is a volatile window during which running the same query may get different results. |
| `content_type` | String |  | Asset content type. If not specified, no content but the asset name will be returned. |
| `output_config` | String |  | Required. Output configuration indicating where the results will be output to. All results will be in newline delimited JSON format. |
| `parent` | String | ✅ | Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), a project number (such as "projects/12345"), or a folder number (such as "folders/123"). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assets` | Vec<String> | A list of assets with valid time windows. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.cloudasset_api.Project {
    parent = "value"  # Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), a project number (such as "projects/12345"), or a folder number (such as "folders/123").
}

# Access project outputs
project_id = project.id
project_assets = project.assets
```

---


### Folder

Exports assets with time and resource types to a given Cloud Storage location. The output format is newline-delimited JSON. This API implements the google.longrunning.Operation API allowing you to keep track of the export. We recommend intervals of at least 2 seconds with exponential retry to poll the export operation result. For regular-size resource parent, the export operation usually finishes within 5 minutes.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_types` | Vec<String> |  | A list of asset types of which to take a snapshot for. For example: "google.compute.Disk". If specified, only matching assets will be returned. See [Introduction to Cloud Asset Inventory](https://cloud.google.com/resource-manager/docs/cloud-asset-inventory/overview) for all supported asset types. |
| `read_time` | String |  | Timestamp to take an asset snapshot. This can only be set to a timestamp between 2018-10-02 UTC (inclusive) and the current time. If not specified, the current time will be used. Due to delays in resource data collection and indexing, there is a volatile window during which running the same query may get different results. |
| `content_type` | String |  | Asset content type. If not specified, no content but the asset name will be returned. |
| `output_config` | String |  | Required. Output configuration indicating where the results will be output to. All results will be in newline delimited JSON format. |
| `parent` | String | ✅ | Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), a project number (such as "projects/12345"), or a folder number (such as "folders/123"). |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create folder
folder = provider.cloudasset_api.Folder {
    parent = "value"  # Required. The relative name of the root asset. This can only be an organization number (such as "organizations/123"), a project ID (such as "projects/my-project-id"), a project number (such as "projects/12345"), or a folder number (such as "folders/123").
}

```

---


### Resource

Searches all the resources within a given accessible Resource Manager scope (project/folder/organization). This RPC gives callers especially administrators the ability to search all the resources within a scope, even if they don't have `.get` permission of all the resources. Callers should have `cloudasset.assets.searchAllResources` permission on the requested scope, otherwise the request will be rejected.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `results` | Vec<String> | A list of resource that match the search query. |
| `next_page_token` | String | If there are more results than those appearing in this response, then `next_page_token` is included. To get the next set of results, call this method again using the value of `next_page_token` as `page_token`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access resource outputs
resource_id = resource.id
resource_results = resource.results
resource_next_page_token = resource.next_page_token
```

---


### Iam_policie

Searches all the IAM policies within a given accessible Resource Manager scope (project/folder/organization). This RPC gives callers especially administrators the ability to search all the IAM policies within a scope, even if they don't have `.getIamPolicy` permission of all the IAM policies. Callers should have `cloudasset.assets.searchAllIamPolicies` permission on the requested scope, otherwise the request will be rejected.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Set if there are more results than those appearing in this response; to get the next set of results, call this method again, using this value as the `page_token`. |
| `results` | Vec<String> | A list of IAM policies that match the search query. Related information such as the associated resource is returned along with the policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access iam_policie outputs
iam_policie_id = iam_policie.id
iam_policie_next_page_token = iam_policie.next_page_token
iam_policie_results = iam_policie.results
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple operation resources
operation_0 = provider.cloudasset_api.Operation {
}
operation_1 = provider.cloudasset_api.Operation {
}
operation_2 = provider.cloudasset_api.Operation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.cloudasset_api.Operation {
    }
```

---

## Related Documentation

- [GCP Cloudasset_api Documentation](https://cloud.google.com/cloudasset_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
