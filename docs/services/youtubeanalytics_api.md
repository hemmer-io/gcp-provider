# Youtubeanalytics_api Service



**Resources**: 6

---

## Overview

The youtubeanalytics_api service provides access to 6 resource types:

- [Report](#report) [R]
- [Group](#group) [CRUD]
- [Group_item](#group_item) [CRD]
- [Group](#group) [CRUD]
- [Group_item](#group_item) [CRD]
- [Report](#report) [R]

---

## Resources


### Report

Retrieve your YouTube Analytics reports.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `errors` | String | When set, indicates that the operation failed. |
| `rows` | Vec<Vec<String>> | The list contains all rows of the result table. Each item in the list is an array that contains comma-delimited data corresponding to a single row of data. The order of the comma-delimited data fields will match the order of the columns listed in the `columnHeaders` field. If no data is available for the given query, the `rows` element will be omitted from the response. The response for a query with the `day` dimension will not contain rows for the most recent days. |
| `column_headers` | Vec<String> | This value specifies information about the data returned in the `rows` fields. Each item in the `columnHeaders` list identifies a field returned in the `rows` value, which contains a list of comma-delimited data. The `columnHeaders` list will begin with the dimensions specified in the API request, which will be followed by the metrics specified in the API request. The order of both dimensions and metrics will match the ordering in the API request. For example, if the API request contains the parameters `dimensions=ageGroup,gender&metrics=viewerPercentage`, the API response will return columns in this order: `ageGroup`, `gender`, `viewerPercentage`. |
| `kind` | String | This value specifies the type of data included in the API response. For the query method, the kind property value will be `youtubeAnalytics#resultTable`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access report outputs
report_id = report.id
report_errors = report.errors
report_rows = report.rows
report_column_headers = report.column_headers
report_kind = report.kind
```

---


### Group

Creates a group.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | The Etag of this resource. |
| `snippet` | String |  | The `snippet` object contains basic information about the group, including its creation date and name. |
| `errors` | String |  | Apiary error details |
| `content_details` | String |  | The `contentDetails` object contains additional information about the group, such as the number and type of items that it contains. |
| `kind` | String |  | Identifies the API resource's type. The value will be `youtube#group`. |
| `id` | String |  | The ID that YouTube uses to uniquely identify the group. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | The Etag of this resource. |
| `next_page_token` | String | The token that can be used as the value of the `pageToken` parameter to retrieve the next page in the result set. |
| `errors` | String | Apiary error details |
| `kind` | String | Identifies the API resource's type. The value will be `youtube#groupListResponse`. |
| `items` | Vec<String> | A list of groups that match the API request parameters. Each item in the list represents a `group` resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.youtubeanalytics_api.Group {
}

# Access group outputs
group_id = group.id
group_etag = group.etag
group_next_page_token = group.next_page_token
group_errors = group.errors
group_kind = group.kind
group_items = group.items
```

---


### Group_item

Creates a group item.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies the API resource's type. The value will be `youtube#groupItem`. |
| `id` | String |  | The ID that YouTube uses to uniquely identify the `channel`, `video`, `playlist`, or `asset` resource that is included in the group. Note that this ID refers specifically to the inclusion of that resource in a particular group and is different than the channel ID, video ID, playlist ID, or asset ID that uniquely identifies the resource itself. The `resource.id` property's value specifies the unique channel, video, playlist, or asset ID. |
| `resource` | String |  | The `resource` object contains information that identifies the item being added to the group. |
| `etag` | String |  | The Etag of this resource. |
| `errors` | String |  | Apiary error details |
| `group_id` | String |  | The ID that YouTube uses to uniquely identify the group that contains the item. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | The Etag of this resource. |
| `kind` | String | Identifies the API resource's type. The value will be `youtube#groupItemListResponse`. |
| `items` | Vec<String> | A list of groups that match the API request parameters. Each item in the list represents a `groupItem` resource. |
| `errors` | String | Apiary error details |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group_item
group_item = provider.youtubeanalytics_api.Group_item {
}

# Access group_item outputs
group_item_id = group_item.id
group_item_etag = group_item.etag
group_item_kind = group_item.kind
group_item_items = group_item.items
group_item_errors = group_item.errors
```

---


### Group

Creates a group.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  |  |
| `kind` | String |  |  |
| `etag` | String |  |  |
| `content_details` | String |  |  |
| `snippet` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String |  |
| `etag` | String |  |
| `kind` | String |  |
| `items` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.youtubeanalytics_api.Group {
}

# Access group outputs
group_id = group.id
group_next_page_token = group.next_page_token
group_etag = group.etag
group_kind = group.kind
group_items = group.items
```

---


### Group_item

Creates a group item.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource` | String |  |  |
| `id` | String |  |  |
| `group_id` | String |  |  |
| `etag` | String |  |  |
| `kind` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String |  |
| `items` | Vec<String> |  |
| `kind` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group_item
group_item = provider.youtubeanalytics_api.Group_item {
}

# Access group_item outputs
group_item_id = group_item.id
group_item_etag = group_item.etag
group_item_items = group_item.items
group_item_kind = group_item.kind
```

---


### Report

Retrieve your YouTube Analytics reports.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `column_headers` | Vec<String> | This value specifies information about the data returned in the rows fields. Each item in the columnHeaders list identifies a field returned in the rows value, which contains a list of comma-delimited data. The columnHeaders list will begin with the dimensions specified in the API request, which will be followed by the metrics specified in the API request. The order of both dimensions and metrics will match the ordering in the API request. For example, if the API request contains the parameters dimensions=ageGroup,gender&metrics=viewerPercentage, the API response will return columns in this order: ageGroup,gender,viewerPercentage. |
| `rows` | Vec<Vec<String>> | The list contains all rows of the result table. Each item in the list is an array that contains comma-delimited data corresponding to a single row of data. The order of the comma-delimited data fields will match the order of the columns listed in the columnHeaders field. If no data is available for the given query, the rows element will be omitted from the response. The response for a query with the day dimension will not contain rows for the most recent days. |
| `kind` | String | This value specifies the type of data included in the API response. For the query method, the kind property value will be youtubeAnalytics#resultTable. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access report outputs
report_id = report.id
report_column_headers = report.column_headers
report_rows = report.rows
report_kind = report.kind
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple report resources
report_0 = provider.youtubeanalytics_api.Report {
}
report_1 = provider.youtubeanalytics_api.Report {
}
report_2 = provider.youtubeanalytics_api.Report {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    report = provider.youtubeanalytics_api.Report {
    }
```

---

## Related Documentation

- [GCP Youtubeanalytics_api Documentation](https://cloud.google.com/youtubeanalytics_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
