# Apigee_api Service



**Resources**: 64

---

## Overview

The apigee_api service provides access to 64 resource types:

- [Reference](#reference) [CRUD]
- [Security_incident](#security_incident) [CRU]
- [Data](#data) [R]
- [Security_monitoring_condition](#security_monitoring_condition) [CRUD]
- [Apidoc](#apidoc) [CRUD]
- [Datastore](#datastore) [CRUD]
- [Api](#api) [CRUD]
- [Resourcefile](#resourcefile) [CRUD]
- [Stat](#stat) [R]
- [Security_report](#security_report) [CR]
- [Developer](#developer) [CRUD]
- [Subscription](#subscription) [CR]
- [Security_assessment_result](#security_assessment_result) [C]
- [Targetserver](#targetserver) [CRUD]
- [Optimized_host_stat](#optimized_host_stat) [R]
- [Querie](#querie) [CR]
- [Security_stat](#security_stat) [C]
- [Optimized_stat](#optimized_stat) [R]
- [Envgroup](#envgroup) [CRUD]
- [Entrie](#entrie) [CRUD]
- [Archive_deployment](#archive_deployment) [CRUD]
- [Apicategorie](#apicategorie) [CRUD]
- [Host_querie](#host_querie) [CR]
- [Nat_addresse](#nat_addresse) [CRD]
- [Attribute](#attribute) [CRD]
- [Security_action](#security_action) [CRUD]
- [Debugsession](#debugsession) [CRD]
- [App](#app) [CRUD]
- [Issuer](#issuer) [R]
- [Keyvaluemap](#keyvaluemap) [CRUD]
- [Project](#project) [C]
- [Admin](#admin) [R]
- [Security_feedback](#security_feedback) [CRUD]
- [Appgroup](#appgroup) [CRUD]
- [Addons_config](#addons_config) [C]
- [Rateplan](#rateplan) [CRUD]
- [Sharedflow](#sharedflow) [CRD]
- [Organization](#organization) [CRUD]
- [Export](#export) [CR]
- [Space](#space) [CRUD]
- [Apiproduct](#apiproduct) [CRUD]
- [Create](#create) [C]
- [Key](#key) [CRUD]
- [Environment](#environment) [CRUD]
- [Datacollector](#datacollector) [CRUD]
- [Endpoint_attachment](#endpoint_attachment) [CRD]
- [Keystore](#keystore) [CRD]
- [Flowhook](#flowhook) [RUD]
- [Cache](#cache) [D]
- [Host_security_report](#host_security_report) [CR]
- [Operation](#operation) [R]
- [Revision](#revision) [CRD]
- [Attachment](#attachment) [CRD]
- [Balance](#balance) [C]
- [Canaryevaluation](#canaryevaluation) [CR]
- [Dns_zone](#dns_zone) [CRD]
- [Override](#override) [CRUD]
- [Aliase](#aliase) [CRUD]
- [Security_profile](#security_profile) [CRUD]
- [Instance](#instance) [CRUD]
- [Host_stat](#host_stat) [R]
- [Security_profiles_v2](#security_profiles_v2) [CRUD]
- [Report](#report) [CRUD]
- [Deployment](#deployment) [CR]

---

## Resources


### Reference

Creates a Reference in the specified environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The resource id of this reference. Values must match the regular expression [\w\s\-.]+. |
| `resource_type` | String |  | The type of resource referred to by this reference. Valid values are 'KeyStore' or 'TrustStore'. |
| `refers` | String |  | Required. The id of the resource to which this reference refers. Must be the id of a resource that exists in the parent environment and is of the given resource_type. |
| `description` | String |  | Optional. A human-readable description of this reference. |
| `parent` | String | ✅ | Required. The parent environment name under which the Reference will be created. Must be of the form `organizations/{org}/environments/{env}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The resource id of this reference. Values must match the regular expression [\w\s\-.]+. |
| `resource_type` | String | The type of resource referred to by this reference. Valid values are 'KeyStore' or 'TrustStore'. |
| `refers` | String | Required. The id of the resource to which this reference refers. Must be the id of a resource that exists in the parent environment and is of the given resource_type. |
| `description` | String | Optional. A human-readable description of this reference. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reference
reference = provider.apigee_api.Reference {
    parent = "value"  # Required. The parent environment name under which the Reference will be created. Must be of the form `organizations/{org}/environments/{env}`.
}

# Access reference outputs
reference_id = reference.id
reference_name = reference.name
reference_resource_type = reference.resource_type
reference_refers = reference.refers
reference_description = reference.description
```

---


### Security_incident

BatchUpdateSecurityIncident updates multiple existing security incidents.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Optional. Required. The request message specifying the resources to update. A maximum of 1000 can be modified in a batch. |
| `parent` | String | ✅ | Optional. The parent resource shared by all security incidents being updated. If this is set, the parent field in the UpdateSecurityIncidentRequest messages must either be empty or match this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `detection_types` | Vec<String> | Output only. Detection types which are part of the incident. Examples: Flooder, OAuth Abuser, Static Content Scraper, Anomaly Detection. |
| `observability` | String | Optional. Indicates if the user archived this incident. |
| `last_observability_change_time` | String | Output only. The time when the incident observability was last changed. |
| `name` | String | Immutable. Name of the security incident resource. Format: organizations/{org}/environments/{environment}/securityIncidents/{incident} Example: organizations/apigee-org/environments/dev/securityIncidents/1234-5678-9101-1111 |
| `risk_level` | String | Output only. Risk level of the incident. |
| `display_name` | String | Optional. Display name of the security incident. |
| `traffic_count` | String | Total traffic detected as part of the incident. |
| `first_detected_time` | String | Output only. The time when events associated with the incident were first detected. |
| `last_detected_time` | String | Output only. The time when events associated with the incident were last detected. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_incident
security_incident = provider.apigee_api.Security_incident {
    parent = "value"  # Optional. The parent resource shared by all security incidents being updated. If this is set, the parent field in the UpdateSecurityIncidentRequest messages must either be empty or match this field.
}

# Access security_incident outputs
security_incident_id = security_incident.id
security_incident_detection_types = security_incident.detection_types
security_incident_observability = security_incident.observability
security_incident_last_observability_change_time = security_incident.last_observability_change_time
security_incident_name = security_incident.name
security_incident_risk_level = security_incident.risk_level
security_incident_display_name = security_incident.display_name
security_incident_traffic_count = security_incident.traffic_count
security_incident_first_detected_time = security_incident.first_detected_time
security_incident_last_detected_time = security_incident.last_detected_time
```

---


### Data

Gets the debug data from a transaction.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `point` | Vec<String> | List of debug data collected by runtime plane at various defined points in the flow. |
| `completed` | bool | Flag indicating whether a transaction is completed or not |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access data outputs
data_id = data.id
data_point = data.point
data_completed = data.completed
```

---


### Security_monitoring_condition

Create a security monitoring condition.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time of the security monitoring condition creation. |
| `name` | String |  | Identifier. Name of the security monitoring condition resource. Format: organizations/{org}/securityMonitoringConditions/{security_monitoring_condition} |
| `total_monitored_resources` | i64 |  | Output only. Total number of monitored resources within this condition. |
| `total_deployed_resources` | i64 |  | Output only. Total number of deployed resources within scope. |
| `include` | String |  | Include only these resources. |
| `include_all_resources` | String |  | Include all resources under the scope. |
| `scope` | String |  | Optional. Scope of the security monitoring condition. For Apigee, the environment is the scope of the resources. |
| `profile` | String |  | Required. ID of security profile of the security monitoring condition. |
| `update_time` | String |  | Output only. The time of the security monitoring condition update. |
| `parent` | String | ✅ | Required. The parent resource name. Format: `organizations/{org}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time of the security monitoring condition creation. |
| `name` | String | Identifier. Name of the security monitoring condition resource. Format: organizations/{org}/securityMonitoringConditions/{security_monitoring_condition} |
| `total_monitored_resources` | i64 | Output only. Total number of monitored resources within this condition. |
| `total_deployed_resources` | i64 | Output only. Total number of deployed resources within scope. |
| `include` | String | Include only these resources. |
| `include_all_resources` | String | Include all resources under the scope. |
| `scope` | String | Optional. Scope of the security monitoring condition. For Apigee, the environment is the scope of the resources. |
| `profile` | String | Required. ID of security profile of the security monitoring condition. |
| `update_time` | String | Output only. The time of the security monitoring condition update. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_monitoring_condition
security_monitoring_condition = provider.apigee_api.Security_monitoring_condition {
    parent = "value"  # Required. The parent resource name. Format: `organizations/{org}`
}

# Access security_monitoring_condition outputs
security_monitoring_condition_id = security_monitoring_condition.id
security_monitoring_condition_create_time = security_monitoring_condition.create_time
security_monitoring_condition_name = security_monitoring_condition.name
security_monitoring_condition_total_monitored_resources = security_monitoring_condition.total_monitored_resources
security_monitoring_condition_total_deployed_resources = security_monitoring_condition.total_deployed_resources
security_monitoring_condition_include = security_monitoring_condition.include
security_monitoring_condition_include_all_resources = security_monitoring_condition.include_all_resources
security_monitoring_condition_scope = security_monitoring_condition.scope
security_monitoring_condition_profile = security_monitoring_condition.profile
security_monitoring_condition_update_time = security_monitoring_condition.update_time
```

---


### Apidoc

Creates a new catalog item.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `graphql_endpoint_url` | String |  | Optional. DEPRECATED: manage documentation through the `getDocumentation` and `updateDocumentation` methods |
| `graphql_schema` | String |  | Optional. DEPRECATED: manage documentation through the `getDocumentation` and `updateDocumentation` methods |
| `id` | String |  | Output only. The ID of the catalog item. |
| `api_product_name` | String |  | Required. Immutable. The `name` field of the associated [API product](/apigee/docs/reference/apis/apigee/rest/v1/organizations.apiproducts). A portal may have only one catalog item associated with a given API product. |
| `graphql_schema_display_name` | String |  | Optional. DEPRECATED: manage documentation through the `getDocumentation` and `updateDocumentation` methods |
| `image_url` | String |  | Optional. Location of the image used for the catalog item in the catalog. This can be either an image with an external URL or a file path for [image files stored in the portal](/apigee/docs/api-platform/publish/portal/portal-files"), for example, `/files/book-tree.jpg`. When specifying the URL of an external image, the image won't be uploaded to your assets; additionally, loading the image in the integrated portal will be subject to its availability, which may be blocked or restricted by [content security policies](/apigee/docs/api-platform/publish/portal/csp). Max length of file path is 2,083 characters. |
| `anon_allowed` | bool |  | Optional. Boolean flag that manages user access to the catalog item. When true, the catalog item has public visibility and can be viewed anonymously; otherwise, only registered users may view it. Note: when the parent portal is enrolled in the [audience management feature](https://cloud.google.com/apigee/docs/api-platform/publish/portal/portal-audience#enrolling_in_the_beta_release_of_the_audience_management_feature), and this flag is set to false, visibility is set to an indeterminate state and must be explicitly specified in the management UI (see [Manage the visibility of an API in your portal](https://cloud.google.com/apigee/docs/api-platform/publish/portal/publish-apis#visibility)). Additionally, when enrolled in the audience management feature, updates to this flag will be ignored as visibility permissions must be updated in the management UI. |
| `require_callback_url` | bool |  | Optional. Whether a callback URL is required when this catalog item's API product is enabled in a developer app. When true, a portal user will be required to input a URL when managing the app (this is typically used for the app's OAuth flow). |
| `visibility` | bool |  | Optional. DEPRECATED: use the `published` field instead |
| `site_id` | String |  | Output only. The ID of the parent portal. |
| `published` | bool |  | Optional. Denotes whether the catalog item is published to the portal or is in a draft state. When the parent portal is enrolled in the [audience management feature](https://cloud.google.com/apigee/docs/api-platform/publish/portal/portal-audience#enrolling_in_the_beta_release_of_the_audience_management_feature), the visibility can be set to public on creation by setting the anonAllowed flag to true or further managed in the management UI (see [Manage the visibility of an API in your portal](https://cloud.google.com/apigee/docs/api-platform/publish/portal/publish-apis#visibility)) before it can be visible to any users. If not enrolled in the audience management feature, the visibility is managed by the `anonAllowed` flag. |
| `category_ids` | Vec<String> |  | Optional. The IDs of the API categories to which this catalog item belongs. |
| `description` | String |  | Optional. Description of the catalog item. Max length is 10,000 characters. |
| `spec_id` | String |  | Optional. DEPRECATED: DO NOT USE |
| `edge_api_product_name` | String |  | Optional. Immutable. DEPRECATED: use the `apiProductName` field instead |
| `modified` | String |  | Output only. Time the catalog item was last modified in milliseconds since epoch. |
| `title` | String |  | Required. The user-facing name of the catalog item. `title` must be a non-empty string with a max length of 255 characters. |
| `parent` | String | ✅ | Required. Name of the portal. Use the following structure in your request: `organizations/{org}/sites/{site}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | Status of the operation. |
| `data` | String | The catalog item resource. |
| `error_code` | String | Unique error code for the request, if any. |
| `message` | String | Description of the operation. |
| `request_id` | String | Unique ID of the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create apidoc
apidoc = provider.apigee_api.Apidoc {
    parent = "value"  # Required. Name of the portal. Use the following structure in your request: `organizations/{org}/sites/{site}`
}

# Access apidoc outputs
apidoc_id = apidoc.id
apidoc_status = apidoc.status
apidoc_data = apidoc.data
apidoc_error_code = apidoc.error_code
apidoc_message = apidoc.message
apidoc_request_id = apidoc.request_id
```

---


### Datastore

Create a Datastore for an org

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `datastore_config` | String |  | Datastore Configurations. |
| `org` | String |  | Output only. Organization that the datastore belongs to |
| `target_type` | String |  | Destination storage type. Supported types `gcs` or `bigquery`. |
| `create_time` | String |  | Output only. Datastore create time, in milliseconds since the epoch of 1970-01-01T00:00:00Z |
| `last_update_time` | String |  | Output only. Datastore last update time, in milliseconds since the epoch of 1970-01-01T00:00:00Z |
| `display_name` | String |  | Required. Display name in UI |
| `self` | String |  | Output only. Resource link of Datastore. Example: `/organizations/{org}/analytics/datastores/{uuid}` |
| `parent` | String | ✅ | Required. The parent organization name. Must be of the form `organizations/{org}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `datastore_config` | String | Datastore Configurations. |
| `org` | String | Output only. Organization that the datastore belongs to |
| `target_type` | String | Destination storage type. Supported types `gcs` or `bigquery`. |
| `create_time` | String | Output only. Datastore create time, in milliseconds since the epoch of 1970-01-01T00:00:00Z |
| `last_update_time` | String | Output only. Datastore last update time, in milliseconds since the epoch of 1970-01-01T00:00:00Z |
| `display_name` | String | Required. Display name in UI |
| `self` | String | Output only. Resource link of Datastore. Example: `/organizations/{org}/analytics/datastores/{uuid}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create datastore
datastore = provider.apigee_api.Datastore {
    parent = "value"  # Required. The parent organization name. Must be of the form `organizations/{org}`.
}

# Access datastore outputs
datastore_id = datastore.id
datastore_datastore_config = datastore.datastore_config
datastore_org = datastore.org
datastore_target_type = datastore.target_type
datastore_create_time = datastore.create_time
datastore_last_update_time = datastore.last_update_time
datastore_display_name = datastore.display_name
datastore_self = datastore.self
```

---


### Api

Creates an API proxy. The API proxy created will not be accessible at runtime until it is deployed to an environment. Create a new API proxy by setting the `name` query parameter to the name of the API proxy. Import an API proxy configuration bundle stored in zip format on your local machine to your organization by doing the following: * Set the `name` query parameter to the name of the API proxy. * Set the `action` query parameter to `import`. * Set the `Content-Type` header to `multipart/form-data`. * Pass as a file the name of API proxy configuration bundle stored in zip format on your local machine using the `file` form field. **Note**: To validate the API proxy configuration bundle only without importing it, set the `action` query parameter to `validate`. When importing an API proxy configuration bundle, if the API proxy does not exist, it will be created. If the API proxy exists, then a new revision is created. Invalid API proxy configurations are rejected, and a list of validation errors is returned to the client.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `data` | String |  | The HTTP request/response body as raw binary. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `parent` | String | ✅ | Required. Name of the organization in the following format: `organizations/{org}` If the API Proxy resource has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `latest_revision_id` | String | Output only. The id of the most recently created revision for this api proxy. |
| `space` | String | Optional. The id of the space this proxy is associated with. Any IAM policies applied to the space will control access to this proxy. To learn how Spaces can be used to manage resources, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview). |
| `meta_data` | String | Output only. Metadata describing the API proxy. |
| `api_proxy_type` | String | Output only. The type of the API proxy. |
| `read_only` | bool | Output only. Whether this proxy is read-only. A read-only proxy cannot have new revisions created through calls to CreateApiProxyRevision. A proxy is read-only if it was generated by an archive. |
| `labels` | HashMap<String, String> | User labels applied to this API Proxy. |
| `name` | String | Output only. Name of the API proxy. |
| `revision` | Vec<String> | Output only. List of revisions defined for the API proxy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create api
api = provider.apigee_api.Api {
    parent = "value"  # Required. Name of the organization in the following format: `organizations/{org}` If the API Proxy resource has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview).
}

# Access api outputs
api_id = api.id
api_latest_revision_id = api.latest_revision_id
api_space = api.space
api_meta_data = api.meta_data
api_api_proxy_type = api.api_proxy_type
api_read_only = api.read_only
api_labels = api.labels
api_name = api.name
api_revision = api.revision
```

---


### Resourcefile

Creates a resource file. Specify the `Content-Type` as `application/octet-stream` or `multipart/form-data`. For more information about resource files, see [Resource files](https://cloud.google.com/apigee/docs/api-platform/develop/resource-files).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `data` | String |  | The HTTP request/response body as raw binary. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `parent` | String | ✅ | Required. Name of the environment in which to create the resource file in the following format: `organizations/{org}/environments/{env}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create resourcefile
resourcefile = provider.apigee_api.Resourcefile {
    parent = "value"  # Required. Name of the environment in which to create the resource file in the following format: `organizations/{org}/environments/{env}`.
}

# Access resourcefile outputs
resourcefile_id = resourcefile.id
resourcefile_extensions = resourcefile.extensions
resourcefile_data = resourcefile.data
resourcefile_content_type = resourcefile.content_type
```

---


### Stat

Retrieve metrics grouped by dimensions. The types of metrics you can retrieve include traffic, message counts, API call latency, response size, and cache hits and counts. Dimensions let you view metrics in meaningful groups. You can optionally pass dimensions as path parameters to the `stats` API. If dimensions are not specified, the metrics are computed on the entire set of data for the given time range.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hosts` | Vec<String> | List of query results grouped by host. |
| `environments` | Vec<String> | List of query results on the environment level. |
| `meta_data` | String | Metadata information. |


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
stat_hosts = stat.hosts
stat_environments = stat.environments
stat_meta_data = stat.meta_data
```

---


### Security_report

Submit a report request to be processed in the background. If the submission succeeds, the API returns a 200 status and an ID that refer to the report request. In addition to the HTTP status 200, the `state` of "enqueued" means that the request succeeded.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `time_range` | String |  | Required. Time range for the query. Can use the following predefined strings to specify the time range: `last60minutes` `last24hours` `last7days` Or, specify the timeRange as a structure describing start and end timestamps in the ISO format: yyyy-mm-ddThh:mm:ssZ. Example: "timeRange": { "start": "2018-07-29T00:13:00Z", "end": "2018-08-01T00:18:00Z" } |
| `mime_type` | String |  | Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the csvDelimiter property. |
| `filter` | String |  | Boolean expression that can be used to filter data. Filter expressions can be combined using AND/OR terms and should be fully parenthesized to avoid ambiguity. See Analytics metrics, dimensions, and filters reference https://docs.apigee.com/api-platform/analytics/analytics-reference for more information on the fields available to filter on. For more information on the tokens that you use to build filter expressions, see Filter expression syntax. https://docs.apigee.com/api-platform/analytics/asynch-reports-api#filter-expression-syntax |
| `csv_delimiter` | String |  | Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\t`). |
| `dimensions` | Vec<String> |  | A list of dimensions. https://docs.apigee.com/api-platform/analytics/analytics-reference#dimensions |
| `limit` | i64 |  | Maximum number of rows that can be returned in the result. |
| `group_by_time_unit` | String |  | Time unit used to group the result set. Valid values include: second, minute, hour, day, week, or month. If a query includes groupByTimeUnit, then the result is an aggregation based on the specified time unit and the resultant timestamp does not include milliseconds precision. If a query omits groupByTimeUnit, then the resultant timestamp includes milliseconds precision. |
| `metrics` | Vec<String> |  | A list of Metrics. |
| `display_name` | String |  | Security Report display name which users can specify. |
| `envgroup_hostname` | String |  | Hostname needs to be specified if query intends to run at host level. This field is only allowed when query is submitted by CreateHostSecurityReport where analytics data will be grouped by organization and hostname. |
| `report_definition_id` | String |  | Report Definition ID. |
| `parent` | String | ✅ | Required. The parent resource name. Must be of the form `organizations/{org}/environments/{env}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `result` | String | Result is available only after the query is completed. |
| `report_definition_id` | String | Report Definition ID. |
| `created` | String | Creation time of the query. |
| `result_file_size` | String | ResultFileSize is available only after the query is completed. |
| `execution_time` | String | ExecutionTime is available only after the query is completed. |
| `envgroup_hostname` | String | Hostname is available only when query is executed at host level. |
| `state` | String | Query state could be "enqueued", "running", "completed", "expired" and "failed". |
| `error` | String | Error is set when query fails. |
| `query_params` | String | Contains information like metrics, dimenstions etc of the Security Report. |
| `result_rows` | String | ResultRows is available only after the query is completed. |
| `updated` | String | Output only. Last updated timestamp for the query. |
| `self` | String | Self link of the query. Example: `/organizations/myorg/environments/myenv/securityReports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` or following format if query is running at host level: `/organizations/myorg/hostSecurityReports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` |
| `display_name` | String | Display Name specified by the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_report
security_report = provider.apigee_api.Security_report {
    parent = "value"  # Required. The parent resource name. Must be of the form `organizations/{org}/environments/{env}`.
}

# Access security_report outputs
security_report_id = security_report.id
security_report_result = security_report.result
security_report_report_definition_id = security_report.report_definition_id
security_report_created = security_report.created
security_report_result_file_size = security_report.result_file_size
security_report_execution_time = security_report.execution_time
security_report_envgroup_hostname = security_report.envgroup_hostname
security_report_state = security_report.state
security_report_error = security_report.error
security_report_query_params = security_report.query_params
security_report_result_rows = security_report.result_rows
security_report_updated = security_report.updated
security_report_self = security_report.self
security_report_display_name = security_report.display_name
```

---


### Developer

Creates a developer. Once created, the developer can register an app and obtain an API key. At creation time, a developer is set as `active`. To change the developer status, use the SetDeveloperStatus API.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `first_name` | String |  | Required. First name of the developer. |
| `status` | String |  | Output only. Status of the developer. Valid values are `active` and `inactive`. |
| `user_name` | String |  | Required. User name of the developer. Not used by Apigee hybrid. |
| `companies` | Vec<String> |  | List of companies associated with the developer. |
| `last_name` | String |  | Required. Last name of the developer. |
| `app_family` | String |  | Developer app family. |
| `developer_id` | String |  | ID of the developer. **Note**: IDs are generated internally by Apigee and are not guaranteed to stay the same over time. |
| `organization_name` | String |  | Output only. Name of the Apigee organization in which the developer resides. |
| `email` | String |  | Required. Email address of the developer. This value is used to uniquely identify the developer in Apigee hybrid. Note that the email address has to be in lowercase only. |
| `apps` | Vec<String> |  | List of apps associated with the developer. |
| `created_at` | String |  | Output only. Time at which the developer was created in milliseconds since epoch. |
| `attributes` | Vec<String> |  | Optional. Developer attributes (name/value pairs). The custom attribute limit is 18. |
| `access_type` | String |  | Access type. |
| `last_modified_at` | String |  | Output only. Time at which the developer was last modified in milliseconds since epoch. |
| `parent` | String | ✅ | Required. Name of the Apigee organization in which the developer is created. Use the following structure in your request: `organizations/{org}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `first_name` | String | Required. First name of the developer. |
| `status` | String | Output only. Status of the developer. Valid values are `active` and `inactive`. |
| `user_name` | String | Required. User name of the developer. Not used by Apigee hybrid. |
| `companies` | Vec<String> | List of companies associated with the developer. |
| `last_name` | String | Required. Last name of the developer. |
| `app_family` | String | Developer app family. |
| `developer_id` | String | ID of the developer. **Note**: IDs are generated internally by Apigee and are not guaranteed to stay the same over time. |
| `organization_name` | String | Output only. Name of the Apigee organization in which the developer resides. |
| `email` | String | Required. Email address of the developer. This value is used to uniquely identify the developer in Apigee hybrid. Note that the email address has to be in lowercase only. |
| `apps` | Vec<String> | List of apps associated with the developer. |
| `created_at` | String | Output only. Time at which the developer was created in milliseconds since epoch. |
| `attributes` | Vec<String> | Optional. Developer attributes (name/value pairs). The custom attribute limit is 18. |
| `access_type` | String | Access type. |
| `last_modified_at` | String | Output only. Time at which the developer was last modified in milliseconds since epoch. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create developer
developer = provider.apigee_api.Developer {
    parent = "value"  # Required. Name of the Apigee organization in which the developer is created. Use the following structure in your request: `organizations/{org}`.
}

# Access developer outputs
developer_id = developer.id
developer_first_name = developer.first_name
developer_status = developer.status
developer_user_name = developer.user_name
developer_companies = developer.companies
developer_last_name = developer.last_name
developer_app_family = developer.app_family
developer_developer_id = developer.developer_id
developer_organization_name = developer.organization_name
developer_email = developer.email
developer_apps = developer.apps
developer_created_at = developer.created_at
developer_attributes = developer.attributes
developer_access_type = developer.access_type
developer_last_modified_at = developer.last_modified_at
```

---


### Subscription

Creates a subscription to an API product. 

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `apiproduct` | String |  | Name of the API product for which the developer is purchasing a subscription. |
| `name` | String |  | Output only. Name of the API product subscription. |
| `start_time` | String |  | Time when the API product subscription starts in milliseconds since epoch. |
| `created_at` | String |  | Output only. Time when the API product subscription was created in milliseconds since epoch. |
| `end_time` | String |  | Time when the API product subscription ends in milliseconds since epoch. |
| `last_modified_at` | String |  | Output only. Time when the API product subscription was last modified in milliseconds since epoch. |
| `parent` | String | ✅ | Required. Email address of the developer that is purchasing a subscription to the API product. Use the following structure in your request: `organizations/{org}/developers/{developer_email}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `apiproduct` | String | Name of the API product for which the developer is purchasing a subscription. |
| `name` | String | Output only. Name of the API product subscription. |
| `start_time` | String | Time when the API product subscription starts in milliseconds since epoch. |
| `created_at` | String | Output only. Time when the API product subscription was created in milliseconds since epoch. |
| `end_time` | String | Time when the API product subscription ends in milliseconds since epoch. |
| `last_modified_at` | String | Output only. Time when the API product subscription was last modified in milliseconds since epoch. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subscription
subscription = provider.apigee_api.Subscription {
    parent = "value"  # Required. Email address of the developer that is purchasing a subscription to the API product. Use the following structure in your request: `organizations/{org}/developers/{developer_email}`
}

# Access subscription outputs
subscription_id = subscription.id
subscription_apiproduct = subscription.apiproduct
subscription_name = subscription.name
subscription_start_time = subscription.start_time
subscription_created_at = subscription.created_at
subscription_end_time = subscription.end_time
subscription_last_modified_at = subscription.last_modified_at
```

---


### Security_assessment_result

Compute RAV2 security scores for a set of resources.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `profile` | String |  | Required. Name of the profile that is used for computation. |
| `include` | String |  | Include only these resources. |
| `page_size` | i64 |  | Optional. The maximum number of results to return. The service may return fewer than this value. If unspecified, at most 50 results will be returned. |
| `scope` | String |  | Optional. Scope of the resources for the computation. For Apigee, the environment is the scope of the resources. |
| `page_token` | String |  | Optional. A page token, received from a previous `BatchComputeSecurityAssessmentResults` call. Provide this to retrieve the subsequent page. |
| `include_all_resources` | String |  | Include all resources under the scope. |
| `name` | String | ✅ | Required. Name of the organization for which the score needs to be computed in the following format: `organizations/{org}/securityAssessmentResults` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_assessment_result
security_assessment_result = provider.apigee_api.Security_assessment_result {
    name = "value"  # Required. Name of the organization for which the score needs to be computed in the following format: `organizations/{org}/securityAssessmentResults`
}

```

---


### Targetserver

Creates a TargetServer in the specified environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `protocol` | String |  | Immutable. The protocol used by this TargetServer. |
| `s_sl_info` | String |  | Optional. Specifies TLS configuration info for this TargetServer. The JSON name is `sSLInfo` for legacy/backwards compatibility reasons -- Edge originally supported SSL, and the name is still used for TLS configuration. |
| `port` | i64 |  | Required. The port number this target connects to on the given host. Value must be between 1 and 65535, inclusive. |
| `description` | String |  | Optional. A human-readable description of this TargetServer. |
| `host` | String |  | Required. The host name this target connects to. Value must be a valid hostname as described by RFC-1123. |
| `is_enabled` | bool |  | Optional. Enabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true. |
| `name` | String |  | Required. The resource id of this target server. Values must match the regular expression  |
| `parent` | String | ✅ | Required. The parent environment name under which the TargetServer will be created. Must be of the form `organizations/{org}/environments/{env}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `protocol` | String | Immutable. The protocol used by this TargetServer. |
| `s_sl_info` | String | Optional. Specifies TLS configuration info for this TargetServer. The JSON name is `sSLInfo` for legacy/backwards compatibility reasons -- Edge originally supported SSL, and the name is still used for TLS configuration. |
| `port` | i64 | Required. The port number this target connects to on the given host. Value must be between 1 and 65535, inclusive. |
| `description` | String | Optional. A human-readable description of this TargetServer. |
| `host` | String | Required. The host name this target connects to. Value must be a valid hostname as described by RFC-1123. |
| `is_enabled` | bool | Optional. Enabling/disabling a TargetServer is useful when TargetServers are used in load balancing configurations, and one or more TargetServers need to taken out of rotation periodically. Defaults to true. |
| `name` | String | Required. The resource id of this target server. Values must match the regular expression  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create targetserver
targetserver = provider.apigee_api.Targetserver {
    parent = "value"  # Required. The parent environment name under which the TargetServer will be created. Must be of the form `organizations/{org}/environments/{env}`.
}

# Access targetserver outputs
targetserver_id = targetserver.id
targetserver_protocol = targetserver.protocol
targetserver_s_sl_info = targetserver.s_sl_info
targetserver_port = targetserver.port
targetserver_description = targetserver.description
targetserver_host = targetserver.host
targetserver_is_enabled = targetserver.is_enabled
targetserver_name = targetserver.name
```

---


### Optimized_host_stat

Similar to GetHostStats except that the response is less verbose.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | String | Wraps the `stats` response for JavaScript Optimized Scenario with a response key. For example: ```{ "Response": { "TimeUnit": [], "metaData": { "errors": [], "notices": [ "Source:Postgres", "Table used: edge.api.aaxgroup001.agg_api", "PG Host:ruappg08-ro.production.apigeeks.net", "query served by:80c4ebca-6a10-4a2e-8faf-c60c1ee306ca" ] }, "resultTruncated": false, "stats": { "data": [ { "identifier": { "names": [ "apiproxy" ], "values": [ "sirjee" ] }, "metric": [ { "env": "prod", "name": "sum(message_count)", "values": [ 36.0 ] }, { "env": "prod", "name": "sum(is_error)", "values": [ 36.0 ] } ] } ] } } }``` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access optimized_host_stat outputs
optimized_host_stat_id = optimized_host_stat.id
optimized_host_stat_response = optimized_host_stat.response
```

---


### Querie

Submit a query to be processed in the background. If the submission of the query succeeds, the API returns a 201 status and an ID that refer to the query. In addition to the HTTP status 201, the `state` of "enqueued" means that the request succeeded.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_by_time_unit` | String |  | Time unit used to group the result set. Valid values include: second, minute, hour, day, week, or month. If a query includes groupByTimeUnit, then the result is an aggregation based on the specified time unit and the resultant timestamp does not include milliseconds precision. If a query omits groupByTimeUnit, then the resultant timestamp includes milliseconds precision. |
| `envgroup_hostname` | String |  | Hostname needs to be specified if query intends to run at host level. This field is only allowed when query is submitted by CreateHostAsyncQuery where analytics data will be grouped by organization and hostname. |
| `output_format` | String |  | Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the csvDelimiter property. |
| `metrics` | Vec<String> |  | A list of Metrics. |
| `name` | String |  | Asynchronous Query Name. |
| `time_range` | String |  | Required. Time range for the query. Can use the following predefined strings to specify the time range: `last60minutes` `last24hours` `last7days` Or, specify the timeRange as a structure describing start and end timestamps in the ISO format: yyyy-mm-ddThh:mm:ssZ. Example: "timeRange": { "start": "2018-07-29T00:13:00Z", "end": "2018-08-01T00:18:00Z" } |
| `limit` | i64 |  | Maximum number of rows that can be returned in the result. |
| `dimensions` | Vec<String> |  | A list of dimensions. https://docs.apigee.com/api-platform/analytics/analytics-reference#dimensions |
| `report_definition_id` | String |  | Asynchronous Report ID. |
| `filter` | String |  | Boolean expression that can be used to filter data. Filter expressions can be combined using AND/OR terms and should be fully parenthesized to avoid ambiguity. See Analytics metrics, dimensions, and filters reference https://docs.apigee.com/api-platform/analytics/analytics-reference for more information on the fields available to filter on. For more information on the tokens that you use to build filter expressions, see Filter expression syntax. https://docs.apigee.com/api-platform/analytics/asynch-reports-api#filter-expression-syntax |
| `csv_delimiter` | String |  | Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\t`). |
| `parent` | String | ✅ | Required. The parent resource name. Must be of the form `organizations/{org}/environments/{env}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `report_definition_id` | String | Asynchronous Report ID. |
| `result` | String | Result is available only after the query is completed. |
| `query_params` | String | Contains information like metrics, dimenstions etc of the AsyncQuery. |
| `state` | String | Query state could be "enqueued", "running", "completed", "failed". |
| `self` | String | Self link of the query. Example: `/organizations/myorg/environments/myenv/queries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` or following format if query is running at host level: `/organizations/myorg/hostQueries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` |
| `updated` | String | Last updated timestamp for the query. |
| `created` | String | Creation time of the query. |
| `execution_time` | String | ExecutionTime is available only after the query is completed. |
| `result_file_size` | String | ResultFileSize is available only after the query is completed. |
| `name` | String | Asynchronous Query Name. |
| `error` | String | Error is set when query fails. |
| `envgroup_hostname` | String | Hostname is available only when query is executed at host level. |
| `result_rows` | String | ResultRows is available only after the query is completed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create querie
querie = provider.apigee_api.Querie {
    parent = "value"  # Required. The parent resource name. Must be of the form `organizations/{org}/environments/{env}`.
}

# Access querie outputs
querie_id = querie.id
querie_report_definition_id = querie.report_definition_id
querie_result = querie.result
querie_query_params = querie.query_params
querie_state = querie.state
querie_self = querie.self
querie_updated = querie.updated
querie_created = querie.created
querie_execution_time = querie.execution_time
querie_result_file_size = querie.result_file_size
querie_name = querie.name
querie_error = querie.error
querie_envgroup_hostname = querie.envgroup_hostname
querie_result_rows = querie.result_rows
```

---


### Security_stat

Retrieve security statistics as a collection of time series.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_token` | String |  | Page token stands for a specific collection of time series sequences. |
| `timestamp_order` | String |  | Order the sequences in increasing or decreasing order of timestamps. Default is descending order of timestamps (latest first). |
| `dimensions` | Vec<String> |  | List of dimension names to group the aggregations by. If no dimensions are passed, a single trend line representing the requested metric aggregations grouped by environment is returned. |
| `filter` | String |  | Filter further on specific dimension values. Follows the same grammar as custom report's filter expressions. Example, apiproxy eq 'foobar'. https://cloud.google.com/apigee/docs/api-platform/analytics/analytics-reference#filters |
| `metrics` | Vec<String> |  | Required. List of metrics and their aggregations. |
| `time_range` | String |  | Required. Time range for the stats. |
| `page_size` | i64 |  | Page size represents the number of time series sequences, one per unique set of dimensions and their values. |
| `window_size` | String |  | Time buckets to group the stats by. |
| `orgenv` | String | ✅ | Required. Should be of the form organizations//environments/. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_stat
security_stat = provider.apigee_api.Security_stat {
    orgenv = "value"  # Required. Should be of the form organizations//environments/.
}

```

---


### Optimized_stat

Similar to GetStats except that the response is less verbose.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | String | Wraps the `stats` response for JavaScript Optimized Scenario with a response key. For example: ```{ "Response": { "TimeUnit": [], "metaData": { "errors": [], "notices": [ "Source:Postgres", "Table used: edge.api.aaxgroup001.agg_api", "PG Host:ruappg08-ro.production.apigeeks.net", "query served by:80c4ebca-6a10-4a2e-8faf-c60c1ee306ca" ] }, "resultTruncated": false, "stats": { "data": [ { "identifier": { "names": [ "apiproxy" ], "values": [ "sirjee" ] }, "metric": [ { "env": "prod", "name": "sum(message_count)", "values": [ 36.0 ] }, { "env": "prod", "name": "sum(is_error)", "values": [ 36.0 ] } ] } ] } } }``` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access optimized_stat outputs
optimized_stat_id = optimized_stat.id
optimized_stat_response = optimized_stat.response
```

---


### Envgroup

Creates a new environment group.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hostnames` | Vec<String> |  | Required. Host names for this environment group. |
| `created_at` | String |  | Output only. The time at which the environment group was created as milliseconds since epoch. |
| `last_modified_at` | String |  | Output only. The time at which the environment group was last updated as milliseconds since epoch. |
| `name` | String |  | ID of the environment group. |
| `state` | String |  | Output only. State of the environment group. Values other than ACTIVE means the resource is not ready to use. |
| `parent` | String | ✅ | Required. Name of the organization in which to create the environment group in the following format: `organizations/{org}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hostnames` | Vec<String> | Required. Host names for this environment group. |
| `created_at` | String | Output only. The time at which the environment group was created as milliseconds since epoch. |
| `last_modified_at` | String | Output only. The time at which the environment group was last updated as milliseconds since epoch. |
| `name` | String | ID of the environment group. |
| `state` | String | Output only. State of the environment group. Values other than ACTIVE means the resource is not ready to use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create envgroup
envgroup = provider.apigee_api.Envgroup {
    parent = "value"  # Required. Name of the organization in which to create the environment group in the following format: `organizations/{org}`.
}

# Access envgroup outputs
envgroup_id = envgroup.id
envgroup_hostnames = envgroup.hostnames
envgroup_created_at = envgroup.created_at
envgroup_last_modified_at = envgroup.last_modified_at
envgroup_name = envgroup.name
envgroup_state = envgroup.state
```

---


### Entrie

Creates key value entries in a key value map scoped to an organization, environment, or API proxy. **Note**: Supported for Apigee hybrid 1.8.x and higher.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `value` | String |  | Required. Data or payload that is being retrieved and associated with the unique key. |
| `name` | String |  | Resource URI that can be used to identify the scope of the key value map entries. |
| `parent` | String | ✅ | Required. Scope as indicated by the URI in which to create the key value map entry. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}`. If the KeyValueMap is under an API Proxy resource that has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `value` | String | Required. Data or payload that is being retrieved and associated with the unique key. |
| `name` | String | Resource URI that can be used to identify the scope of the key value map entries. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entrie
entrie = provider.apigee_api.Entrie {
    parent = "value"  # Required. Scope as indicated by the URI in which to create the key value map entry. Use **one** of the following structures in your request: * `organizations/{organization}/apis/{api}/keyvaluemaps/{keyvaluemap}`. * `organizations/{organization}/environments/{environment}/keyvaluemaps/{keyvaluemap}` * `organizations/{organization}/keyvaluemaps/{keyvaluemap}`. If the KeyValueMap is under an API Proxy resource that has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview).
}

# Access entrie outputs
entrie_id = entrie.id
entrie_value = entrie.value
entrie_name = entrie.name
```

---


### Archive_deployment

Creates a new ArchiveDeployment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Name of the Archive Deployment in the following format: `organizations/{org}/environments/{env}/archiveDeployments/{id}`. |
| `updated_at` | String |  | Output only. The time at which the Archive Deployment was updated in milliseconds since the epoch. |
| `operation` | String |  | Output only. A reference to the LRO that created this Archive Deployment in the following format: `organizations/{org}/operations/{id}` |
| `created_at` | String |  | Output only. The time at which the Archive Deployment was created in milliseconds since the epoch. |
| `gcs_uri` | String |  | Input only. The Google Cloud Storage signed URL returned from GenerateUploadUrl and used to upload the Archive zip file. |
| `labels` | HashMap<String, String> |  | User-supplied key-value pairs used to organize ArchiveDeployments. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |
| `parent` | String | ✅ | Required. The Environment this Archive Deployment will be created in. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Name of the Archive Deployment in the following format: `organizations/{org}/environments/{env}/archiveDeployments/{id}`. |
| `updated_at` | String | Output only. The time at which the Archive Deployment was updated in milliseconds since the epoch. |
| `operation` | String | Output only. A reference to the LRO that created this Archive Deployment in the following format: `organizations/{org}/operations/{id}` |
| `created_at` | String | Output only. The time at which the Archive Deployment was created in milliseconds since the epoch. |
| `gcs_uri` | String | Input only. The Google Cloud Storage signed URL returned from GenerateUploadUrl and used to upload the Archive zip file. |
| `labels` | HashMap<String, String> | User-supplied key-value pairs used to organize ArchiveDeployments. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: \p{Ll}\p{Lo}{0,62} Label values must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: [\p{Ll}\p{Lo}\p{N}_-]{0,63} No more than 64 labels can be associated with a given store. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create archive_deployment
archive_deployment = provider.apigee_api.Archive_deployment {
    parent = "value"  # Required. The Environment this Archive Deployment will be created in.
}

# Access archive_deployment outputs
archive_deployment_id = archive_deployment.id
archive_deployment_name = archive_deployment.name
archive_deployment_updated_at = archive_deployment.updated_at
archive_deployment_operation = archive_deployment.operation
archive_deployment_created_at = archive_deployment.created_at
archive_deployment_gcs_uri = archive_deployment.gcs_uri
archive_deployment_labels = archive_deployment.labels
```

---


### Apicategorie

Creates a new API category.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | ID of the category (a UUID). |
| `update_time` | String |  | Time the category was last modified in milliseconds since epoch. |
| `site_id` | String |  | Name of the portal. |
| `name` | String |  | Name of the category. |
| `parent` | String | ✅ | Required. Name of the portal. Use the following structure in your request: `organizations/{org}/sites/{site}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_id` | String | Unique ID of the request. |
| `status` | String | Status of the operation. |
| `message` | String | Description of the operation. |
| `error_code` | String | Unique error code for the request, if any. |
| `data` | String | The API category resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create apicategorie
apicategorie = provider.apigee_api.Apicategorie {
    parent = "value"  # Required. Name of the portal. Use the following structure in your request: `organizations/{org}/sites/{site}`
}

# Access apicategorie outputs
apicategorie_id = apicategorie.id
apicategorie_request_id = apicategorie.request_id
apicategorie_status = apicategorie.status
apicategorie_message = apicategorie.message
apicategorie_error_code = apicategorie.error_code
apicategorie_data = apicategorie.data
```

---


### Host_querie

Submit a query at host level to be processed in the background. If the submission of the query succeeds, the API returns a 201 status and an ID that refer to the query. In addition to the HTTP status 201, the `state` of "enqueued" means that the request succeeded.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group_by_time_unit` | String |  | Time unit used to group the result set. Valid values include: second, minute, hour, day, week, or month. If a query includes groupByTimeUnit, then the result is an aggregation based on the specified time unit and the resultant timestamp does not include milliseconds precision. If a query omits groupByTimeUnit, then the resultant timestamp includes milliseconds precision. |
| `envgroup_hostname` | String |  | Hostname needs to be specified if query intends to run at host level. This field is only allowed when query is submitted by CreateHostAsyncQuery where analytics data will be grouped by organization and hostname. |
| `output_format` | String |  | Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the csvDelimiter property. |
| `metrics` | Vec<String> |  | A list of Metrics. |
| `name` | String |  | Asynchronous Query Name. |
| `time_range` | String |  | Required. Time range for the query. Can use the following predefined strings to specify the time range: `last60minutes` `last24hours` `last7days` Or, specify the timeRange as a structure describing start and end timestamps in the ISO format: yyyy-mm-ddThh:mm:ssZ. Example: "timeRange": { "start": "2018-07-29T00:13:00Z", "end": "2018-08-01T00:18:00Z" } |
| `limit` | i64 |  | Maximum number of rows that can be returned in the result. |
| `dimensions` | Vec<String> |  | A list of dimensions. https://docs.apigee.com/api-platform/analytics/analytics-reference#dimensions |
| `report_definition_id` | String |  | Asynchronous Report ID. |
| `filter` | String |  | Boolean expression that can be used to filter data. Filter expressions can be combined using AND/OR terms and should be fully parenthesized to avoid ambiguity. See Analytics metrics, dimensions, and filters reference https://docs.apigee.com/api-platform/analytics/analytics-reference for more information on the fields available to filter on. For more information on the tokens that you use to build filter expressions, see Filter expression syntax. https://docs.apigee.com/api-platform/analytics/asynch-reports-api#filter-expression-syntax |
| `csv_delimiter` | String |  | Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\t`). |
| `parent` | String | ✅ | Required. The parent resource name. Must be of the form `organizations/{org}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `report_definition_id` | String | Asynchronous Report ID. |
| `result` | String | Result is available only after the query is completed. |
| `query_params` | String | Contains information like metrics, dimenstions etc of the AsyncQuery. |
| `state` | String | Query state could be "enqueued", "running", "completed", "failed". |
| `self` | String | Self link of the query. Example: `/organizations/myorg/environments/myenv/queries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` or following format if query is running at host level: `/organizations/myorg/hostQueries/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` |
| `updated` | String | Last updated timestamp for the query. |
| `created` | String | Creation time of the query. |
| `execution_time` | String | ExecutionTime is available only after the query is completed. |
| `result_file_size` | String | ResultFileSize is available only after the query is completed. |
| `name` | String | Asynchronous Query Name. |
| `error` | String | Error is set when query fails. |
| `envgroup_hostname` | String | Hostname is available only when query is executed at host level. |
| `result_rows` | String | ResultRows is available only after the query is completed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create host_querie
host_querie = provider.apigee_api.Host_querie {
    parent = "value"  # Required. The parent resource name. Must be of the form `organizations/{org}`.
}

# Access host_querie outputs
host_querie_id = host_querie.id
host_querie_report_definition_id = host_querie.report_definition_id
host_querie_result = host_querie.result
host_querie_query_params = host_querie.query_params
host_querie_state = host_querie.state
host_querie_self = host_querie.self
host_querie_updated = host_querie.updated
host_querie_created = host_querie.created
host_querie_execution_time = host_querie.execution_time
host_querie_result_file_size = host_querie.result_file_size
host_querie_name = host_querie.name
host_querie_error = host_querie.error
host_querie_envgroup_hostname = host_querie.envgroup_hostname
host_querie_result_rows = host_querie.result_rows
```

---


### Nat_addresse

Creates a NAT address. The address is created in the RESERVED state and a static external IP address will be provisioned. At this time, the instance will not use this IP address for Internet egress traffic. The address can be activated for use once any required firewall IP whitelisting has been completed. **Note:** Not supported for Apigee hybrid.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the nat address. |
| `name` | String |  | Required. Resource ID of the NAT address. |
| `ip_address` | String |  | Output only. The static IPV4 address. |
| `parent` | String | ✅ | Required. Name of the instance. Use the following structure in your request: `organizations/{org}/instances/{instance}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the nat address. |
| `name` | String | Required. Resource ID of the NAT address. |
| `ip_address` | String | Output only. The static IPV4 address. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create nat_addresse
nat_addresse = provider.apigee_api.Nat_addresse {
    parent = "value"  # Required. Name of the instance. Use the following structure in your request: `organizations/{org}/instances/{instance}`
}

# Access nat_addresse outputs
nat_addresse_id = nat_addresse.id
nat_addresse_state = nat_addresse.state
nat_addresse_name = nat_addresse.name
nat_addresse_ip_address = nat_addresse.ip_address
```

---


### Attribute

Updates a developer attribute. **Note**: OAuth access tokens and Key Management Service (KMS) entities (apps, developers, and API products) are cached for 180 seconds (default). Any custom attributes associated with these entities are cached for at least 180 seconds after the entity is accessed at runtime. Therefore, an `ExpiresIn` element on the OAuthV2 policy won't be able to expire an access token in less than 180 seconds.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `value` | String |  | Value of the attribute. |
| `name` | String |  | API key of the attribute. |
| `name` | String | ✅ | Required. Name of the developer attribute. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/attributes/{attribute}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `value` | String | Value of the attribute. |
| `name` | String | API key of the attribute. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attribute
attribute = provider.apigee_api.Attribute {
    name = "value"  # Required. Name of the developer attribute. Use the following structure in your request: `organizations/{org}/developers/{developer_email}/attributes/{attribute}`
}

# Access attribute outputs
attribute_id = attribute.id
attribute_value = attribute.value
attribute_name = attribute.name
```

---


### Security_action

CreateSecurityAction creates a SecurityAction.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. An optional user provided description of the SecurityAction. |
| `flag` | String |  | Flag a request through if it matches this SecurityAction. |
| `api_proxies` | Vec<String> |  | Optional. If unset, this would apply to all proxies in the environment. If set, this action is enforced only if at least one proxy in the repeated list is deployed at the time of enforcement. If set, several restrictions are enforced on SecurityActions. There can be at most 100 enabled actions with proxies set in an env. Several other restrictions apply on conditions and are detailed later. |
| `deny` | String |  | Deny a request through if it matches this SecurityAction. |
| `allow` | String |  | Allow a request through if it matches this SecurityAction. |
| `create_time` | String |  | Output only. The create time for this SecurityAction. |
| `condition_config` | String |  | Required. A valid SecurityAction must contain at least one condition. |
| `expire_time` | String |  | The expiration for this SecurityAction. |
| `name` | String |  | Immutable. This field is ignored during creation as per AIP-133. Please set the `security_action_id` field in the CreateSecurityActionRequest when creating a new SecurityAction. Format: organizations/{org}/environments/{env}/securityActions/{security_action} |
| `ttl` | String |  | Input only. The TTL for this SecurityAction. |
| `state` | String |  | Required. Only an ENABLED SecurityAction is enforced. An ENABLED SecurityAction past its expiration time will not be enforced. |
| `update_time` | String |  | Output only. The update time for this SecurityAction. This reflects when this SecurityAction changed states. |
| `parent` | String | ✅ | Required. The organization and environment that this SecurityAction applies to. Format: organizations/{org}/environments/{env} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. An optional user provided description of the SecurityAction. |
| `flag` | String | Flag a request through if it matches this SecurityAction. |
| `api_proxies` | Vec<String> | Optional. If unset, this would apply to all proxies in the environment. If set, this action is enforced only if at least one proxy in the repeated list is deployed at the time of enforcement. If set, several restrictions are enforced on SecurityActions. There can be at most 100 enabled actions with proxies set in an env. Several other restrictions apply on conditions and are detailed later. |
| `deny` | String | Deny a request through if it matches this SecurityAction. |
| `allow` | String | Allow a request through if it matches this SecurityAction. |
| `create_time` | String | Output only. The create time for this SecurityAction. |
| `condition_config` | String | Required. A valid SecurityAction must contain at least one condition. |
| `expire_time` | String | The expiration for this SecurityAction. |
| `name` | String | Immutable. This field is ignored during creation as per AIP-133. Please set the `security_action_id` field in the CreateSecurityActionRequest when creating a new SecurityAction. Format: organizations/{org}/environments/{env}/securityActions/{security_action} |
| `ttl` | String | Input only. The TTL for this SecurityAction. |
| `state` | String | Required. Only an ENABLED SecurityAction is enforced. An ENABLED SecurityAction past its expiration time will not be enforced. |
| `update_time` | String | Output only. The update time for this SecurityAction. This reflects when this SecurityAction changed states. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_action
security_action = provider.apigee_api.Security_action {
    parent = "value"  # Required. The organization and environment that this SecurityAction applies to. Format: organizations/{org}/environments/{env}
}

# Access security_action outputs
security_action_id = security_action.id
security_action_description = security_action.description
security_action_flag = security_action.flag
security_action_api_proxies = security_action.api_proxies
security_action_deny = security_action.deny
security_action_allow = security_action.allow
security_action_create_time = security_action.create_time
security_action_condition_config = security_action.condition_config
security_action_expire_time = security_action.expire_time
security_action_name = security_action.name
security_action_ttl = security_action.ttl
security_action_state = security_action.state
security_action_update_time = security_action.update_time
```

---


### Debugsession

Creates a debug session for a deployed API Proxy revision.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | Optional. A conditional statement which is evaluated against the request message to determine if it should be traced. Syntax matches that of on API Proxy bundle flow Condition. |
| `validity` | i64 |  | Optional. The length of time, in seconds, that this debug session is valid, starting from when it's received in the control plane. Min = 1, Max = 15, Default = 10. |
| `tracesize` | i64 |  | Optional. The maximum number of bytes captured from the response payload. Min = 0, Max = 5120, Default = 5120. |
| `timeout` | String |  | Optional. The time in seconds after which this DebugSession should end. This value will override the value in query param, if both are provided. |
| `name` | String |  | A unique ID for this DebugSession. |
| `count` | i64 |  | Optional. The number of request to be traced. Min = 1, Max = 15, Default = 10. |
| `create_time` | String |  | Output only. The first transaction creation timestamp, recorded by UAP. |
| `parent` | String | ✅ | Required. The resource name of the API Proxy revision deployment for which to create the DebugSession. Must be of the form `organizations/{organization}/environments/{environment}/apis/{api}/revisions/{revision}`. If the API proxy resource has the `space` attribute set, IAM permissions are checked differently . To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `filter` | String | Optional. A conditional statement which is evaluated against the request message to determine if it should be traced. Syntax matches that of on API Proxy bundle flow Condition. |
| `validity` | i64 | Optional. The length of time, in seconds, that this debug session is valid, starting from when it's received in the control plane. Min = 1, Max = 15, Default = 10. |
| `tracesize` | i64 | Optional. The maximum number of bytes captured from the response payload. Min = 0, Max = 5120, Default = 5120. |
| `timeout` | String | Optional. The time in seconds after which this DebugSession should end. This value will override the value in query param, if both are provided. |
| `name` | String | A unique ID for this DebugSession. |
| `count` | i64 | Optional. The number of request to be traced. Min = 1, Max = 15, Default = 10. |
| `create_time` | String | Output only. The first transaction creation timestamp, recorded by UAP. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create debugsession
debugsession = provider.apigee_api.Debugsession {
    parent = "value"  # Required. The resource name of the API Proxy revision deployment for which to create the DebugSession. Must be of the form `organizations/{organization}/environments/{environment}/apis/{api}/revisions/{revision}`. If the API proxy resource has the `space` attribute set, IAM permissions are checked differently . To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview).
}

# Access debugsession outputs
debugsession_id = debugsession.id
debugsession_filter = debugsession.filter
debugsession_validity = debugsession.validity
debugsession_tracesize = debugsession.tracesize
debugsession_timeout = debugsession.timeout
debugsession_name = debugsession.name
debugsession_count = debugsession.count
debugsession_create_time = debugsession.create_time
```

---


### App

Creates an app associated with a developer. This API associates the developer app with the specified API product and auto-generates an API key for the app to use in calls to API proxies inside that API product. The `name` is the unique ID of the app that you can use in API calls. The `DisplayName` (set as an attribute) appears in the UI. If you don't set the `DisplayName` attribute, the `name` appears in the UI.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_products` | Vec<String> |  | List of API products associated with the developer app. |
| `name` | String |  | Name of the developer app. |
| `last_modified_at` | String |  | Output only. Time the developer app was modified in milliseconds since epoch. |
| `scopes` | Vec<String> |  | Scopes to apply to the developer app. The specified scopes must already exist for the API product that you associate with the developer app. |
| `app_family` | String |  | Developer app family. |
| `attributes` | Vec<String> |  | List of attributes for the developer app. |
| `created_at` | String |  | Output only. Time the developer app was created in milliseconds since epoch. |
| `key_expires_in` | String |  | Expiration time, in milliseconds, for the consumer key that is generated for the developer app. If not set or left to the default value of `-1`, the API key never expires. The expiration time can't be updated after it is set. |
| `credentials` | Vec<String> |  | Output only. Set of credentials for the developer app consisting of the consumer key/secret pairs associated with the API products. |
| `app_id` | String |  | ID of the developer app. This ID is not user specified but is automatically generated on app creation. appId is a UUID. |
| `callback_url` | String |  | Callback URL used by OAuth 2.0 authorization servers to communicate authorization codes back to developer apps. |
| `developer_id` | String |  | ID of the developer. |
| `status` | String |  | Status of the credential. Valid values include `approved` or `revoked`. |
| `parent` | String | ✅ | Required. Name of the developer. Use the following structure in your request: `organizations/{org}/developers/{developer_email}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_products` | Vec<String> | List of API products associated with the developer app. |
| `name` | String | Name of the developer app. |
| `last_modified_at` | String | Output only. Time the developer app was modified in milliseconds since epoch. |
| `scopes` | Vec<String> | Scopes to apply to the developer app. The specified scopes must already exist for the API product that you associate with the developer app. |
| `app_family` | String | Developer app family. |
| `attributes` | Vec<String> | List of attributes for the developer app. |
| `created_at` | String | Output only. Time the developer app was created in milliseconds since epoch. |
| `key_expires_in` | String | Expiration time, in milliseconds, for the consumer key that is generated for the developer app. If not set or left to the default value of `-1`, the API key never expires. The expiration time can't be updated after it is set. |
| `credentials` | Vec<String> | Output only. Set of credentials for the developer app consisting of the consumer key/secret pairs associated with the API products. |
| `app_id` | String | ID of the developer app. This ID is not user specified but is automatically generated on app creation. appId is a UUID. |
| `callback_url` | String | Callback URL used by OAuth 2.0 authorization servers to communicate authorization codes back to developer apps. |
| `developer_id` | String | ID of the developer. |
| `status` | String | Status of the credential. Valid values include `approved` or `revoked`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app
app = provider.apigee_api.App {
    parent = "value"  # Required. Name of the developer. Use the following structure in your request: `organizations/{org}/developers/{developer_email}`
}

# Access app outputs
app_id = app.id
app_api_products = app.api_products
app_name = app.name
app_last_modified_at = app.last_modified_at
app_scopes = app.scopes
app_app_family = app.app_family
app_attributes = app.attributes
app_created_at = app.created_at
app_key_expires_in = app.key_expires_in
app_credentials = app.credentials
app_app_id = app.app_id
app_callback_url = app.callback_url
app_developer_id = app.developer_id
app_status = app.status
```

---


### Issuer

Lists hybrid services and its trusted issuers service account ids. This api is authenticated and unauthorized(allow all the users) and used by runtime authn-authz service to query control plane's issuer service account ids.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `issuers` | Vec<String> | Lists of hybrid services and its trusted issuer email ids. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access issuer outputs
issuer_id = issuer.id
issuer_issuers = issuer.issuers
```

---


### Keyvaluemap

Creates a key value map in an organization.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. ID of the key value map. |
| `encrypted` | bool |  | Required. Flag that specifies whether entry values will be encrypted. This field is retained for backward compatibility and the value of encrypted will always be `true`. Apigee X and hybrid do not support unencrypted key value maps. |
| `parent` | String | ✅ | Required. Name of the organization in which to create the key value map file. Use the following structure in your request: `organizations/{org}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. ID of the key value map. |
| `encrypted` | bool | Required. Flag that specifies whether entry values will be encrypted. This field is retained for backward compatibility and the value of encrypted will always be `true`. Apigee X and hybrid do not support unencrypted key value maps. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create keyvaluemap
keyvaluemap = provider.apigee_api.Keyvaluemap {
    parent = "value"  # Required. Name of the organization in which to create the key value map file. Use the following structure in your request: `organizations/{org}`
}

# Access keyvaluemap outputs
keyvaluemap_id = keyvaluemap.id
keyvaluemap_name = keyvaluemap.name
keyvaluemap_encrypted = keyvaluemap.encrypted
```

---


### Project

Provisions a new Apigee organization with a functioning runtime. This is the standard way to create trial organizations for a free Apigee trial.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authorized_network` | String |  | Compute Engine network used for Service Networking to be peered with Apigee runtime instances. See [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started). Apigee also supports shared VPC (that is, the host network project is not the same as the one that is peering with Apigee). See [Shared VPC overview](https://cloud.google.com/vpc/docs/shared-vpc). To use a shared VPC network, use the following format: `projects/{host-project-id}/{region}/networks/{network-name}`. For example: `projects/my-sharedvpc-host/global/networks/mynetwork` |
| `disable_vpc_peering` | bool |  | Optional. Flag that specifies whether the VPC Peering through Private Google Access should be disabled between the consumer network and Apigee. Required if an authorizedNetwork on the consumer project is not provided, in which case the flag should be set to true. The value must be set before the creation of any Apigee runtime instance and can be updated only when there are no runtime instances. **Note:** Apigee will be deprecating the vpc peering model that requires you to provide 'authorizedNetwork', by making the non-peering model as the default way of provisioning Apigee organization in future. So, this will be a temporary flag to enable the transition. Not supported for Apigee hybrid. |
| `analytics_region` | String |  | Primary Cloud Platform region for analytics data storage. For valid values, see [Create an organization](https://cloud.google.com/apigee/docs/hybrid/latest/precog-provision). Defaults to `us-west1`. |
| `runtime_location` | String |  | Cloud Platform location for the runtime instance. Defaults to zone `us-west1-a`. If a region is provided, `EVAL` organizations will use the region for automatically selecting a zone for the runtime instance. |
| `project` | String | ✅ | Required. Name of the GCP project with which to associate the Apigee organization. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.apigee_api.Project {
    project = "value"  # Required. Name of the GCP project with which to associate the Apigee organization.
}

```

---


### Admin

Gets a list of metrics and dimensions that can be used to create analytics queries and reports. Each schema element contains the name of the field, its associated type, and a flag indicating whether it is a standard or custom field.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `meta` | Vec<String> | Additional metadata associated with schema. This is a legacy field and usually consists of an empty array of strings. |
| `metrics` | Vec<String> | List of schema fields grouped as dimensions that can be used with an aggregate function such as `sum`, `avg`, `min`, and `max`. |
| `dimensions` | Vec<String> | List of schema fields grouped as dimensions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access admin outputs
admin_id = admin.id
admin_meta = admin.meta
admin_metrics = admin.metrics
admin_dimensions = admin.dimensions
```

---


### Security_feedback

Creates a new report containing customer feedback.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `feedback_contexts` | Vec<String> |  | Required. One or more attribute/value pairs for constraining the feedback. |
| `comment` | String |  | Optional. Optional text the user can provide for additional, unstructured context. |
| `name` | String |  | Output only. Identifier. The feedback name is intended to be a system-generated uuid. |
| `reason` | String |  | Optional. The reason for the feedback. |
| `update_time` | String |  | Output only. The time when this specific feedback id was updated. |
| `create_time` | String |  | Output only. The time when this specific feedback id was created. |
| `display_name` | String |  | Optional. The display name of the feedback. |
| `feedback_type` | String |  | Required. The type of feedback being submitted. |
| `parent` | String | ✅ | Required. Name of the organization. Use the following structure in your request: `organizations/{org}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `feedback_contexts` | Vec<String> | Required. One or more attribute/value pairs for constraining the feedback. |
| `comment` | String | Optional. Optional text the user can provide for additional, unstructured context. |
| `name` | String | Output only. Identifier. The feedback name is intended to be a system-generated uuid. |
| `reason` | String | Optional. The reason for the feedback. |
| `update_time` | String | Output only. The time when this specific feedback id was updated. |
| `create_time` | String | Output only. The time when this specific feedback id was created. |
| `display_name` | String | Optional. The display name of the feedback. |
| `feedback_type` | String | Required. The type of feedback being submitted. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_feedback
security_feedback = provider.apigee_api.Security_feedback {
    parent = "value"  # Required. Name of the organization. Use the following structure in your request: `organizations/{org}`.
}

# Access security_feedback outputs
security_feedback_id = security_feedback.id
security_feedback_feedback_contexts = security_feedback.feedback_contexts
security_feedback_comment = security_feedback.comment
security_feedback_name = security_feedback.name
security_feedback_reason = security_feedback.reason
security_feedback_update_time = security_feedback.update_time
security_feedback_create_time = security_feedback.create_time
security_feedback_display_name = security_feedback.display_name
security_feedback_feedback_type = security_feedback.feedback_type
```

---


### Appgroup

Creates an AppGroup. Once created, user can register apps under the AppGroup to obtain secret key and password. At creation time, the AppGroup's state is set as `active`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `organization` | String |  | Immutable. the org the app group is created |
| `display_name` | String |  | app group name displayed in the UI |
| `app_group_id` | String |  | Output only. Internal identifier that cannot be edited |
| `attributes` | Vec<String> |  | A list of attributes |
| `last_modified_at` | String |  | Output only. Modified time as milliseconds since epoch. |
| `channel_uri` | String |  | A reference to the associated storefront/marketplace. |
| `created_at` | String |  | Output only. Created time as milliseconds since epoch. |
| `channel_id` | String |  | channel identifier identifies the owner maintaing this grouping. |
| `name` | String |  | Immutable. Name of the AppGroup. Characters you can use in the name are restricted to: A-Z0-9._\-$ %. |
| `status` | String |  | Valid values are `active` or `inactive`. Note that the status of the AppGroup should be updated via UpdateAppGroupRequest by setting the action as `active` or `inactive`. |
| `parent` | String | ✅ | Required. Name of the Apigee organization in which the AppGroup is created. Use the following structure in your request: `organizations/{org}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `organization` | String | Immutable. the org the app group is created |
| `display_name` | String | app group name displayed in the UI |
| `app_group_id` | String | Output only. Internal identifier that cannot be edited |
| `attributes` | Vec<String> | A list of attributes |
| `last_modified_at` | String | Output only. Modified time as milliseconds since epoch. |
| `channel_uri` | String | A reference to the associated storefront/marketplace. |
| `created_at` | String | Output only. Created time as milliseconds since epoch. |
| `channel_id` | String | channel identifier identifies the owner maintaing this grouping. |
| `name` | String | Immutable. Name of the AppGroup. Characters you can use in the name are restricted to: A-Z0-9._\-$ %. |
| `status` | String | Valid values are `active` or `inactive`. Note that the status of the AppGroup should be updated via UpdateAppGroupRequest by setting the action as `active` or `inactive`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create appgroup
appgroup = provider.apigee_api.Appgroup {
    parent = "value"  # Required. Name of the Apigee organization in which the AppGroup is created. Use the following structure in your request: `organizations/{org}`.
}

# Access appgroup outputs
appgroup_id = appgroup.id
appgroup_organization = appgroup.organization
appgroup_display_name = appgroup.display_name
appgroup_app_group_id = appgroup.app_group_id
appgroup_attributes = appgroup.attributes
appgroup_last_modified_at = appgroup.last_modified_at
appgroup_channel_uri = appgroup.channel_uri
appgroup_created_at = appgroup.created_at
appgroup_channel_id = appgroup.channel_id
appgroup_name = appgroup.name
appgroup_status = appgroup.status
```

---


### Addons_config

Updates an add-on enablement status of an environment.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_security_enabled` | bool |  | If the API Security should be enabled in the environment. |
| `analytics_enabled` | bool |  | If the Analytics should be enabled in the environment. |
| `name` | String | ✅ | Required. Name of the add-ons config. Must be in the format of `/organizations/{org}/environments/{env}/addonsConfig` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create addons_config
addons_config = provider.apigee_api.Addons_config {
    name = "value"  # Required. Name of the add-ons config. Must be in the format of `/organizations/{org}/environments/{env}/addonsConfig`
}

```

---


### Rateplan

Create a rate plan that is associated with an API product in an organization. Using rate plans, API product owners can monetize their API products by configuring one or more of the following: - Billing frequency - Initial setup fees for using an API product - Payment funding model (postpaid only) - Fixed recurring or consumption-based charges for using an API product - Revenue sharing with developer partners An API product can have multiple rate plans associated with it but *only one* rate plan can be active at any point of time. **Note: From the developer's perspective, they purchase API products not rate plans.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Display name of the rate plan. |
| `fixed_recurring_fee` | String |  | Fixed amount that is charged at a defined interval and billed in advance of use of the API product. The fee will be prorated for the first billing period. |
| `payment_funding_model` | String |  | DEPRECATED: This field is no longer supported and will eventually be removed when Apigee Hybrid 1.5/1.6 is no longer supported. Instead, use the `billingType` field inside `DeveloperMonetizationConfig` resource. Flag that specifies the billing account type, prepaid or postpaid. |
| `consumption_pricing_type` | String |  | Pricing model used for consumption-based charges. |
| `setup_fee` | String |  | Initial, one-time fee paid when purchasing the API product. |
| `currency_code` | String |  | Currency to be used for billing. Consists of a three-letter code as defined by the [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) standard. |
| `start_time` | String |  | Time when the rate plan becomes active in milliseconds since epoch. |
| `apiproduct` | String |  | Name of the API product that the rate plan is associated with. |
| `end_time` | String |  | Time when the rate plan will expire in milliseconds since epoch. Set to 0 or `null` to indicate that the rate plan should never expire. |
| `revenue_share_type` | String |  | Method used to calculate the revenue that is shared with developers. |
| `description` | String |  | Description of the rate plan. |
| `billing_period` | String |  | Frequency at which the customer will be billed. |
| `fixed_fee_frequency` | i64 |  | Frequency at which the fixed fee is charged. |
| `state` | String |  | Current state of the rate plan (draft or published). |
| `revenue_share_rates` | Vec<String> |  | Details of the revenue sharing model. |
| `created_at` | String |  | Output only. Time that the rate plan was created in milliseconds since epoch. |
| `consumption_pricing_rates` | Vec<String> |  | API call volume ranges and the fees charged when the total number of API calls is within a given range. The method used to calculate the final fee depends on the selected pricing model. For example, if the pricing model is `BANDED` and the ranges are defined as follows: ``` { "start": 1, "end": 100, "fee": 2 }, { "start": 101, "end": 200, "fee": 1.50 }, { "start": 201, "end": 0, "fee": 1 }, } ``` Then the following fees would be charged based on the total number of API calls (assuming the currency selected is `USD`): * 50 calls cost 50 x $2 = $100 * 150 calls cost 100 x $2 + 50 x $1.5 = $275 * 250 calls cost 100 x $2 + 100 x $1.5 + 50 x $1 = $400 * 500 calls cost 100 x $2 + 100 x $1.5 + 300 x $1 = $650 |
| `name` | String |  | Output only. Name of the rate plan. |
| `last_modified_at` | String |  | Output only. Time the rate plan was last modified in milliseconds since epoch. |
| `parent` | String | ✅ | Required. Name of the API product that is associated with the rate plan. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}` If the API Product resource has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Display name of the rate plan. |
| `fixed_recurring_fee` | String | Fixed amount that is charged at a defined interval and billed in advance of use of the API product. The fee will be prorated for the first billing period. |
| `payment_funding_model` | String | DEPRECATED: This field is no longer supported and will eventually be removed when Apigee Hybrid 1.5/1.6 is no longer supported. Instead, use the `billingType` field inside `DeveloperMonetizationConfig` resource. Flag that specifies the billing account type, prepaid or postpaid. |
| `consumption_pricing_type` | String | Pricing model used for consumption-based charges. |
| `setup_fee` | String | Initial, one-time fee paid when purchasing the API product. |
| `currency_code` | String | Currency to be used for billing. Consists of a three-letter code as defined by the [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) standard. |
| `start_time` | String | Time when the rate plan becomes active in milliseconds since epoch. |
| `apiproduct` | String | Name of the API product that the rate plan is associated with. |
| `end_time` | String | Time when the rate plan will expire in milliseconds since epoch. Set to 0 or `null` to indicate that the rate plan should never expire. |
| `revenue_share_type` | String | Method used to calculate the revenue that is shared with developers. |
| `description` | String | Description of the rate plan. |
| `billing_period` | String | Frequency at which the customer will be billed. |
| `fixed_fee_frequency` | i64 | Frequency at which the fixed fee is charged. |
| `state` | String | Current state of the rate plan (draft or published). |
| `revenue_share_rates` | Vec<String> | Details of the revenue sharing model. |
| `created_at` | String | Output only. Time that the rate plan was created in milliseconds since epoch. |
| `consumption_pricing_rates` | Vec<String> | API call volume ranges and the fees charged when the total number of API calls is within a given range. The method used to calculate the final fee depends on the selected pricing model. For example, if the pricing model is `BANDED` and the ranges are defined as follows: ``` { "start": 1, "end": 100, "fee": 2 }, { "start": 101, "end": 200, "fee": 1.50 }, { "start": 201, "end": 0, "fee": 1 }, } ``` Then the following fees would be charged based on the total number of API calls (assuming the currency selected is `USD`): * 50 calls cost 50 x $2 = $100 * 150 calls cost 100 x $2 + 50 x $1.5 = $275 * 250 calls cost 100 x $2 + 100 x $1.5 + 50 x $1 = $400 * 500 calls cost 100 x $2 + 100 x $1.5 + 300 x $1 = $650 |
| `name` | String | Output only. Name of the rate plan. |
| `last_modified_at` | String | Output only. Time the rate plan was last modified in milliseconds since epoch. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rateplan
rateplan = provider.apigee_api.Rateplan {
    parent = "value"  # Required. Name of the API product that is associated with the rate plan. Use the following structure in your request: `organizations/{org}/apiproducts/{apiproduct}` If the API Product resource has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview).
}

# Access rateplan outputs
rateplan_id = rateplan.id
rateplan_display_name = rateplan.display_name
rateplan_fixed_recurring_fee = rateplan.fixed_recurring_fee
rateplan_payment_funding_model = rateplan.payment_funding_model
rateplan_consumption_pricing_type = rateplan.consumption_pricing_type
rateplan_setup_fee = rateplan.setup_fee
rateplan_currency_code = rateplan.currency_code
rateplan_start_time = rateplan.start_time
rateplan_apiproduct = rateplan.apiproduct
rateplan_end_time = rateplan.end_time
rateplan_revenue_share_type = rateplan.revenue_share_type
rateplan_description = rateplan.description
rateplan_billing_period = rateplan.billing_period
rateplan_fixed_fee_frequency = rateplan.fixed_fee_frequency
rateplan_state = rateplan.state
rateplan_revenue_share_rates = rateplan.revenue_share_rates
rateplan_created_at = rateplan.created_at
rateplan_consumption_pricing_rates = rateplan.consumption_pricing_rates
rateplan_name = rateplan.name
rateplan_last_modified_at = rateplan.last_modified_at
```

---


### Sharedflow

Uploads a ZIP-formatted shared flow configuration bundle to an organization. If the shared flow already exists, this creates a new revision of it. If the shared flow does not exist, this creates it. Once imported, the shared flow revision must be deployed before it can be accessed at runtime. The size limit of a shared flow bundle is 15 MB.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `data` | String |  | The HTTP request/response body as raw binary. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `parent` | String | ✅ | Required. The name of the parent organization under which to create the shared flow. Must be of the form: `organizations/{organization_id}` If the resource has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `meta_data` | String | Metadata describing the shared flow. |
| `latest_revision_id` | String | The id of the most recently created revision for this shared flow. |
| `revision` | Vec<String> | A list of revisions of this shared flow. |
| `space` | String | Optional. The ID of the space associated with this shared flow. Any IAM policies applied to the space will control access to this shared flow. To learn how Spaces can be used to manage resources, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview). |
| `name` | String | The ID of the shared flow. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sharedflow
sharedflow = provider.apigee_api.Sharedflow {
    parent = "value"  # Required. The name of the parent organization under which to create the shared flow. Must be of the form: `organizations/{organization_id}` If the resource has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview).
}

# Access sharedflow outputs
sharedflow_id = sharedflow.id
sharedflow_meta_data = sharedflow.meta_data
sharedflow_latest_revision_id = sharedflow.latest_revision_id
sharedflow_revision = sharedflow.revision
sharedflow_space = sharedflow.space
sharedflow_name = sharedflow.name
```

---


### Organization

Creates an Apigee organization. See [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_consumer_data_encryption_key_name` | String |  | Optional. Cloud KMS key name used for encrypting API consumer data. If not specified or [BillingType](#BillingType) is `EVALUATION`, a Google-Managed encryption key will be used. Format: `projects/*/locations/*/keyRings/*/cryptoKeys/*` |
| `disable_vpc_peering` | bool |  | Optional. Flag that specifies whether the VPC Peering through Private Google Access should be disabled between the consumer network and Apigee. Valid only when RuntimeType is set to CLOUD. Required if an authorizedNetwork on the consumer project is not provided, in which case the flag should be set to true. The value must be set before the creation of any Apigee runtime instance and can be updated only when there are no runtime instances. **Note:** Apigee will be deprecating the vpc peering model that requires you to provide 'authorizedNetwork', by making the non-peering model as the default way of provisioning Apigee organization in future. So, this will be a temporary flag to enable the transition. Not supported for Apigee hybrid. |
| `attributes` | Vec<String> |  | Not used by Apigee. |
| `api_consumer_data_location` | String |  | Optional. This field is needed only for customers using non-default data residency regions. Apigee stores some control plane data only in single region. This field determines which single region Apigee should use. For example: "us-west1" when control plane is in US or "europe-west2" when control plane is in EU. |
| `control_plane_encryption_key_name` | String |  | Optional. Cloud KMS key name used for encrypting control plane data that is stored in a multi region. Only used for the data residency region "US" or "EU". If not specified or [BillingType](#BillingType) is `EVALUATION`, a Google-Managed encryption key will be used. Format: `projects/*/locations/*/keyRings/*/cryptoKeys/*` |
| `description` | String |  | Optional. Description of the Apigee organization. |
| `authorized_network` | String |  | Optional. Compute Engine network used for Service Networking to be peered with Apigee runtime instances. See [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started). Valid only when [RuntimeType](#RuntimeType) is set to `CLOUD`. The value must be set before the creation of a runtime instance and can be updated only when there are no runtime instances. For example: `default`. When changing authorizedNetwork, you must reconfigure VPC peering. After VPC peering with previous network is deleted, [run the following command](https://cloud.google.com/sdk/gcloud/reference/services/vpc-peerings/delete): `gcloud services vpc-peerings delete --network=NETWORK`, where `NETWORK` is the name of the previous network. This will delete the previous Service Networking. Otherwise, you will get the following error: `The resource 'projects/...-tp' is already linked to another shared VPC host 'projects/...-tp`. Apigee also supports shared VPC (that is, the host network project is not the same as the one that is peering with Apigee). See [Shared VPC overview](https://cloud.google.com/vpc/docs/shared-vpc). To use a shared VPC network, use the following format: `projects/{host-project-id}/{region}/networks/{network-name}`. For example: `projects/my-sharedvpc-host/global/networks/mynetwork` **Note:** Not supported for Apigee hybrid. |
| `last_modified_at` | String |  | Output only. Time that the Apigee organization was last modified in milliseconds since epoch. |
| `name` | String |  | Output only. Name of the Apigee organization. |
| `runtime_type` | String |  | Required. Runtime type of the Apigee organization based on the Apigee subscription purchased. |
| `properties` | String |  | Optional. Properties defined in the Apigee organization profile. |
| `apigee_project_id` | String |  | Output only. Apigee Project ID associated with the organization. Use this project to allowlist Apigee in the Service Attachment when using private service connect with Apigee. |
| `type` | String |  | Not used by Apigee. |
| `network_egress_restricted` | bool |  | Optional. Flag that specifies if internet egress is restricted for VPC Service Controls. Valid only when runtime_type is `CLOUD` and disable_vpc_peering is `true`. |
| `project_id` | String |  | Output only. Project ID associated with the Apigee organization. |
| `expires_at` | String |  | Output only. Time that the Apigee organization is scheduled for deletion. |
| `environments` | Vec<String> |  | Output only. List of environments in the Apigee organization. |
| `state` | String |  | Output only. State of the organization. Values other than ACTIVE means the resource is not ready to use. |
| `created_at` | String |  | Output only. Time that the Apigee organization was created in milliseconds since epoch. |
| `subscription_plan` | String |  | Output only. Subscription plan that the customer has purchased. Output only. |
| `portal_disabled` | bool |  | Optional. Configuration for the Portals settings. |
| `analytics_region` | String |  | Required. DEPRECATED: This field will eventually be deprecated and replaced with a differently-named field. Primary Google Cloud region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org). |
| `addons_config` | String |  | Optional. Addon configurations of the Apigee organization. |
| `ca_certificate` | String |  | Output only. Base64-encoded public certificate for the root CA of the Apigee organization. Valid only when [RuntimeType](#RuntimeType) is `CLOUD`. |
| `billing_type` | String |  | Optional. Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing). |
| `display_name` | String |  | Optional. Display name for the Apigee organization. Unused, but reserved for future use. |
| `customer_name` | String |  | Not used by Apigee. |
| `subscription_type` | String |  | Output only. DEPRECATED: This will eventually be replaced by BillingType. Subscription type of the Apigee organization. Valid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased). See [Apigee pricing](https://cloud.google.com/apigee/pricing/). |
| `runtime_database_encryption_key_name` | String |  | Optional. Cloud KMS key name used for encrypting the data that is stored and replicated across runtime instances. Update is not allowed after the organization is created. If not specified or [RuntimeType](#RuntimeType) is `TRIAL`, a Google-Managed encryption key will be used. For example: "projects/foo/locations/us/keyRings/bar/cryptoKeys/baz". **Note:** Not supported for Apigee hybrid. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_consumer_data_encryption_key_name` | String | Optional. Cloud KMS key name used for encrypting API consumer data. If not specified or [BillingType](#BillingType) is `EVALUATION`, a Google-Managed encryption key will be used. Format: `projects/*/locations/*/keyRings/*/cryptoKeys/*` |
| `disable_vpc_peering` | bool | Optional. Flag that specifies whether the VPC Peering through Private Google Access should be disabled between the consumer network and Apigee. Valid only when RuntimeType is set to CLOUD. Required if an authorizedNetwork on the consumer project is not provided, in which case the flag should be set to true. The value must be set before the creation of any Apigee runtime instance and can be updated only when there are no runtime instances. **Note:** Apigee will be deprecating the vpc peering model that requires you to provide 'authorizedNetwork', by making the non-peering model as the default way of provisioning Apigee organization in future. So, this will be a temporary flag to enable the transition. Not supported for Apigee hybrid. |
| `attributes` | Vec<String> | Not used by Apigee. |
| `api_consumer_data_location` | String | Optional. This field is needed only for customers using non-default data residency regions. Apigee stores some control plane data only in single region. This field determines which single region Apigee should use. For example: "us-west1" when control plane is in US or "europe-west2" when control plane is in EU. |
| `control_plane_encryption_key_name` | String | Optional. Cloud KMS key name used for encrypting control plane data that is stored in a multi region. Only used for the data residency region "US" or "EU". If not specified or [BillingType](#BillingType) is `EVALUATION`, a Google-Managed encryption key will be used. Format: `projects/*/locations/*/keyRings/*/cryptoKeys/*` |
| `description` | String | Optional. Description of the Apigee organization. |
| `authorized_network` | String | Optional. Compute Engine network used for Service Networking to be peered with Apigee runtime instances. See [Getting started with the Service Networking API](https://cloud.google.com/service-infrastructure/docs/service-networking/getting-started). Valid only when [RuntimeType](#RuntimeType) is set to `CLOUD`. The value must be set before the creation of a runtime instance and can be updated only when there are no runtime instances. For example: `default`. When changing authorizedNetwork, you must reconfigure VPC peering. After VPC peering with previous network is deleted, [run the following command](https://cloud.google.com/sdk/gcloud/reference/services/vpc-peerings/delete): `gcloud services vpc-peerings delete --network=NETWORK`, where `NETWORK` is the name of the previous network. This will delete the previous Service Networking. Otherwise, you will get the following error: `The resource 'projects/...-tp' is already linked to another shared VPC host 'projects/...-tp`. Apigee also supports shared VPC (that is, the host network project is not the same as the one that is peering with Apigee). See [Shared VPC overview](https://cloud.google.com/vpc/docs/shared-vpc). To use a shared VPC network, use the following format: `projects/{host-project-id}/{region}/networks/{network-name}`. For example: `projects/my-sharedvpc-host/global/networks/mynetwork` **Note:** Not supported for Apigee hybrid. |
| `last_modified_at` | String | Output only. Time that the Apigee organization was last modified in milliseconds since epoch. |
| `name` | String | Output only. Name of the Apigee organization. |
| `runtime_type` | String | Required. Runtime type of the Apigee organization based on the Apigee subscription purchased. |
| `properties` | String | Optional. Properties defined in the Apigee organization profile. |
| `apigee_project_id` | String | Output only. Apigee Project ID associated with the organization. Use this project to allowlist Apigee in the Service Attachment when using private service connect with Apigee. |
| `type` | String | Not used by Apigee. |
| `network_egress_restricted` | bool | Optional. Flag that specifies if internet egress is restricted for VPC Service Controls. Valid only when runtime_type is `CLOUD` and disable_vpc_peering is `true`. |
| `project_id` | String | Output only. Project ID associated with the Apigee organization. |
| `expires_at` | String | Output only. Time that the Apigee organization is scheduled for deletion. |
| `environments` | Vec<String> | Output only. List of environments in the Apigee organization. |
| `state` | String | Output only. State of the organization. Values other than ACTIVE means the resource is not ready to use. |
| `created_at` | String | Output only. Time that the Apigee organization was created in milliseconds since epoch. |
| `subscription_plan` | String | Output only. Subscription plan that the customer has purchased. Output only. |
| `portal_disabled` | bool | Optional. Configuration for the Portals settings. |
| `analytics_region` | String | Required. DEPRECATED: This field will eventually be deprecated and replaced with a differently-named field. Primary Google Cloud region for analytics data storage. For valid values, see [Create an Apigee organization](https://cloud.google.com/apigee/docs/api-platform/get-started/create-org). |
| `addons_config` | String | Optional. Addon configurations of the Apigee organization. |
| `ca_certificate` | String | Output only. Base64-encoded public certificate for the root CA of the Apigee organization. Valid only when [RuntimeType](#RuntimeType) is `CLOUD`. |
| `billing_type` | String | Optional. Billing type of the Apigee organization. See [Apigee pricing](https://cloud.google.com/apigee/pricing). |
| `display_name` | String | Optional. Display name for the Apigee organization. Unused, but reserved for future use. |
| `customer_name` | String | Not used by Apigee. |
| `subscription_type` | String | Output only. DEPRECATED: This will eventually be replaced by BillingType. Subscription type of the Apigee organization. Valid values include trial (free, limited, and for evaluation purposes only) or paid (full subscription has been purchased). See [Apigee pricing](https://cloud.google.com/apigee/pricing/). |
| `runtime_database_encryption_key_name` | String | Optional. Cloud KMS key name used for encrypting the data that is stored and replicated across runtime instances. Update is not allowed after the organization is created. If not specified or [RuntimeType](#RuntimeType) is `TRIAL`, a Google-Managed encryption key will be used. For example: "projects/foo/locations/us/keyRings/bar/cryptoKeys/baz". **Note:** Not supported for Apigee hybrid. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create organization
organization = provider.apigee_api.Organization {
}

# Access organization outputs
organization_id = organization.id
organization_api_consumer_data_encryption_key_name = organization.api_consumer_data_encryption_key_name
organization_disable_vpc_peering = organization.disable_vpc_peering
organization_attributes = organization.attributes
organization_api_consumer_data_location = organization.api_consumer_data_location
organization_control_plane_encryption_key_name = organization.control_plane_encryption_key_name
organization_description = organization.description
organization_authorized_network = organization.authorized_network
organization_last_modified_at = organization.last_modified_at
organization_name = organization.name
organization_runtime_type = organization.runtime_type
organization_properties = organization.properties
organization_apigee_project_id = organization.apigee_project_id
organization_type = organization.type
organization_network_egress_restricted = organization.network_egress_restricted
organization_project_id = organization.project_id
organization_expires_at = organization.expires_at
organization_environments = organization.environments
organization_state = organization.state
organization_created_at = organization.created_at
organization_subscription_plan = organization.subscription_plan
organization_portal_disabled = organization.portal_disabled
organization_analytics_region = organization.analytics_region
organization_addons_config = organization.addons_config
organization_ca_certificate = organization.ca_certificate
organization_billing_type = organization.billing_type
organization_display_name = organization.display_name
organization_customer_name = organization.customer_name
organization_subscription_type = organization.subscription_type
organization_runtime_database_encryption_key_name = organization.runtime_database_encryption_key_name
```

---


### Export

Submit a data export job to be processed in the background. If the request is successful, the API returns a 201 status, a URI that can be used to retrieve the status of the export job, and the `state` value of "enqueued".

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `csv_delimiter` | String |  | Optional. Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\t`). |
| `date_range` | String |  | Required. Date range of the data to export. |
| `datastore_name` | String |  | Required. Name of the preconfigured datastore. |
| `description` | String |  | Optional. Description of the export job. |
| `name` | String |  | Required. Display name of the export job. |
| `output_format` | String |  | Optional. Output format of the export. Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the `csvDelimiter` property. |
| `parent` | String | ✅ | Required. Names of the parent organization and environment. Must be of the form `organizations/{org}/environments/{env}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created` | String | Output only. Time the export job was created. |
| `self` | String | Output only. Self link of the export job. A URI that can be used to retrieve the status of an export job. Example: `/organizations/myorg/environments/myenv/analytics/exports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` |
| `state` | String | Output only. Status of the export job. Valid values include `enqueued`, `running`, `completed`, and `failed`. |
| `updated` | String | Output only. Time the export job was last updated. |
| `datastore_name` | String | Name of the datastore that is the destination of the export job [datastore] |
| `name` | String | Display name of the export job. |
| `description` | String | Description of the export job. |
| `execution_time` | String | Output only. Execution time for this export job. If the job is still in progress, it will be set to the amount of time that has elapsed since`created`, in seconds. Else, it will set to (`updated` - `created`), in seconds. |
| `error` | String | Output only. Error is set when export fails |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create export
export = provider.apigee_api.Export {
    parent = "value"  # Required. Names of the parent organization and environment. Must be of the form `organizations/{org}/environments/{env}`.
}

# Access export outputs
export_id = export.id
export_created = export.created
export_self = export.self
export_state = export.state
export_updated = export.updated
export_datastore_name = export.datastore_name
export_name = export.name
export_description = export.description
export_execution_time = export.execution_time
export_error = export.error
```

---


### Space

Create a space under an organization.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. Display name of the space. |
| `name` | String |  | Output only. Identifier. Id of the space. This field is used as the resource name, and must follow [AIP-122](https://google.aip.dev/122) guidelines. |
| `create_time` | String |  | Output only. Create timestamp of the space. |
| `update_time` | String |  | Output only. Last modified timestamp of the space. |
| `parent` | String | ✅ | Required. Name of the Google Cloud project in which to associate the Apigee space. Pass the information as a query parameter using the following structure in your request: `organizations/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. Display name of the space. |
| `name` | String | Output only. Identifier. Id of the space. This field is used as the resource name, and must follow [AIP-122](https://google.aip.dev/122) guidelines. |
| `create_time` | String | Output only. Create timestamp of the space. |
| `update_time` | String | Output only. Last modified timestamp of the space. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create space
space = provider.apigee_api.Space {
    parent = "value"  # Required. Name of the Google Cloud project in which to associate the Apigee space. Pass the information as a query parameter using the following structure in your request: `organizations/`
}

# Access space outputs
space_id = space.id
space_display_name = space.display_name
space_name = space.name
space_create_time = space.create_time
space_update_time = space.update_time
```

---


### Apiproduct

Creates an API product in an organization. You create API products after you have proxied backend services using API proxies. An API product is a collection of API resources combined with quota settings and metadata that you can use to deliver customized and productized API bundles to your developer community. This metadata can include: - Scope - Environments - API proxies - Extensible profile API products enable you repackage APIs on the fly, without having to do any additional coding or configuration. Apigee recommends that you start with a simple API product including only required elements. You then provision credentials to apps to enable them to start testing your APIs. After you have authentication and authorization working against a simple API product, you can iterate to create finer-grained API products, defining different sets of API resources for each API product. **WARNING:** - If you don't specify an API proxy in the request body, *any* app associated with the product can make calls to *any* API in your entire organization. - If you don't specify an environment in the request body, the product allows access to all environments. For more information, see What is an API product?

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `space` | String |  | Optional. The resource ID of the parent Space. If not set, the parent resource will be the Organization. To learn how Spaces can be used to manage resources, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview). |
| `description` | String |  | Description of the API product. Include key information about the API product that is not captured by other fields. |
| `scopes` | Vec<String> |  | Comma-separated list of OAuth scopes that are validated at runtime. Apigee validates that the scopes in any access token presented match the scopes defined in the OAuth policy associated with the API product. |
| `last_modified_at` | String |  | Response only. Modified time of this environment as milliseconds since epoch. |
| `attributes` | Vec<String> |  | Array of attributes that may be used to extend the default API product profile with customer-specific metadata. You can specify a maximum of 18 attributes. Use this property to specify the access level of the API product as either `public`, `private`, or `internal`. Only products marked `public` are available to developers in the Apigee developer portal. For example, you can set a product to `internal` while it is in development and then change access to `public` when it is ready to release on the portal. API products marked as `private` do not appear on the portal, but can be accessed by external developers. |
| `quota` | String |  | Number of request messages permitted per app by this API product for the specified `quotaInterval` and `quotaTimeUnit`. For example, a `quota` of 50, for a `quotaInterval` of 12 and a `quotaTimeUnit` of hours means 50 requests are allowed every 12 hours. |
| `operation_group` | String |  | Configuration used to group Apigee proxies or remote services with resources, method types, and quotas. The resource refers to the resource URI (excluding the base path). With this grouping, the API product creator is able to fine-tune and give precise control over which REST methods have access to specific resources and how many calls can be made (using the `quota` setting). **Note:** The `api_resources` setting cannot be specified for both the API product and operation group; otherwise the call will fail. |
| `environments` | Vec<String> |  | Comma-separated list of environment names to which the API product is bound. Requests to environments that are not listed are rejected. By specifying one or more environments, you can bind the resources listed in the API product to a specific environment, preventing developers from accessing those resources through API proxies deployed in another environment. This setting is used, for example, to prevent resources associated with API proxies in `prod` from being accessed by API proxies deployed in `test`. |
| `approval_type` | String |  | Flag that specifies how API keys are approved to access the APIs defined by the API product. If set to `manual`, the consumer key is generated and returned in "pending" state. In this case, the API keys won't work until they have been explicitly approved. If set to `auto`, the consumer key is generated and returned in "approved" state and can be used immediately. **Note:** Typically, `auto` is used to provide access to free or trial API products that provide limited quota or capabilities. |
| `graphql_operation_group` | String |  | Configuration used to group Apigee proxies or remote services with graphQL operation name, graphQL operation type and quotas. This grouping allows us to precisely set quota for a particular combination of graphQL name and operation type for a particular proxy request. If graphQL name is not set, this would imply quota will be applied on all graphQL requests matching the operation type. |
| `created_at` | String |  | Response only. Creation time of this environment as milliseconds since epoch. |
| `grpc_operation_group` | String |  | Optional. Configuration used to group Apigee proxies with gRPC services and method names. This grouping allows us to set quota for a particular proxy with the gRPC service name and method. If a method name is not set, this implies quota and authorization are applied to all gRPC methods implemented by that proxy for that particular gRPC service. |
| `api_resources` | Vec<String> |  | Comma-separated list of API resources to be bundled in the API product. By default, the resource paths are mapped from the `proxy.pathsuffix` variable. The proxy path suffix is defined as the URI fragment following the ProxyEndpoint base path. For example, if the `apiResources` element is defined to be `/forecastrss` and the base path defined for the API proxy is `/weather`, then only requests to `/weather/forecastrss` are permitted by the API product. You can select a specific path, or you can select all subpaths with the following wildcard: - `/**`: Indicates that all sub-URIs are included. - `/*` : Indicates that only URIs one level down are included. By default, / supports the same resources as /** as well as the base path defined by the API proxy. For example, if the base path of the API proxy is `/v1/weatherapikey`, then the API product supports requests to `/v1/weatherapikey` and to any sub-URIs, such as `/v1/weatherapikey/forecastrss`, `/v1/weatherapikey/region/CA`, and so on. For more information, see Managing API products. |
| `display_name` | String |  | Name displayed in the UI or developer portal to developers registering for API access. |
| `proxies` | Vec<String> |  | Comma-separated list of API proxy names to which this API product is bound. By specifying API proxies, you can associate resources in the API product with specific API proxies, preventing developers from accessing those resources through other API proxies. Apigee rejects requests to API proxies that are not listed. **Note:** The API proxy names must already exist in the specified environment as they will be validated upon creation. |
| `quota_counter_scope` | String |  | Scope of the quota decides how the quota counter gets applied and evaluate for quota violation. If the Scope is set as PROXY, then all the operations defined for the APIproduct that are associated with the same proxy will share the same quota counter set at the APIproduct level, making it a global counter at a proxy level. If the Scope is set as OPERATION, then each operations get the counter set at the API product dedicated, making it a local counter. Note that, the QuotaCounterScope applies only when an operation does not have dedicated quota set for itself. |
| `quota_interval` | String |  | Time interval over which the number of request messages is calculated. |
| `quota_time_unit` | String |  | Time unit defined for the `quotaInterval`. Valid values include `minute`, `hour`, `day`, or `month`. |
| `name` | String |  | Internal name of the API product. Characters you can use in the name are restricted to: `A-Z0-9._\-$ %`. **Note:** The internal name cannot be edited when updating the API product. |
| `parent` | String | ✅ | Required. Name of the organization in which the API product will be created. Use the following structure in your request: `organizations/{org}` If the resource has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `space` | String | Optional. The resource ID of the parent Space. If not set, the parent resource will be the Organization. To learn how Spaces can be used to manage resources, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview). |
| `description` | String | Description of the API product. Include key information about the API product that is not captured by other fields. |
| `scopes` | Vec<String> | Comma-separated list of OAuth scopes that are validated at runtime. Apigee validates that the scopes in any access token presented match the scopes defined in the OAuth policy associated with the API product. |
| `last_modified_at` | String | Response only. Modified time of this environment as milliseconds since epoch. |
| `attributes` | Vec<String> | Array of attributes that may be used to extend the default API product profile with customer-specific metadata. You can specify a maximum of 18 attributes. Use this property to specify the access level of the API product as either `public`, `private`, or `internal`. Only products marked `public` are available to developers in the Apigee developer portal. For example, you can set a product to `internal` while it is in development and then change access to `public` when it is ready to release on the portal. API products marked as `private` do not appear on the portal, but can be accessed by external developers. |
| `quota` | String | Number of request messages permitted per app by this API product for the specified `quotaInterval` and `quotaTimeUnit`. For example, a `quota` of 50, for a `quotaInterval` of 12 and a `quotaTimeUnit` of hours means 50 requests are allowed every 12 hours. |
| `operation_group` | String | Configuration used to group Apigee proxies or remote services with resources, method types, and quotas. The resource refers to the resource URI (excluding the base path). With this grouping, the API product creator is able to fine-tune and give precise control over which REST methods have access to specific resources and how many calls can be made (using the `quota` setting). **Note:** The `api_resources` setting cannot be specified for both the API product and operation group; otherwise the call will fail. |
| `environments` | Vec<String> | Comma-separated list of environment names to which the API product is bound. Requests to environments that are not listed are rejected. By specifying one or more environments, you can bind the resources listed in the API product to a specific environment, preventing developers from accessing those resources through API proxies deployed in another environment. This setting is used, for example, to prevent resources associated with API proxies in `prod` from being accessed by API proxies deployed in `test`. |
| `approval_type` | String | Flag that specifies how API keys are approved to access the APIs defined by the API product. If set to `manual`, the consumer key is generated and returned in "pending" state. In this case, the API keys won't work until they have been explicitly approved. If set to `auto`, the consumer key is generated and returned in "approved" state and can be used immediately. **Note:** Typically, `auto` is used to provide access to free or trial API products that provide limited quota or capabilities. |
| `graphql_operation_group` | String | Configuration used to group Apigee proxies or remote services with graphQL operation name, graphQL operation type and quotas. This grouping allows us to precisely set quota for a particular combination of graphQL name and operation type for a particular proxy request. If graphQL name is not set, this would imply quota will be applied on all graphQL requests matching the operation type. |
| `created_at` | String | Response only. Creation time of this environment as milliseconds since epoch. |
| `grpc_operation_group` | String | Optional. Configuration used to group Apigee proxies with gRPC services and method names. This grouping allows us to set quota for a particular proxy with the gRPC service name and method. If a method name is not set, this implies quota and authorization are applied to all gRPC methods implemented by that proxy for that particular gRPC service. |
| `api_resources` | Vec<String> | Comma-separated list of API resources to be bundled in the API product. By default, the resource paths are mapped from the `proxy.pathsuffix` variable. The proxy path suffix is defined as the URI fragment following the ProxyEndpoint base path. For example, if the `apiResources` element is defined to be `/forecastrss` and the base path defined for the API proxy is `/weather`, then only requests to `/weather/forecastrss` are permitted by the API product. You can select a specific path, or you can select all subpaths with the following wildcard: - `/**`: Indicates that all sub-URIs are included. - `/*` : Indicates that only URIs one level down are included. By default, / supports the same resources as /** as well as the base path defined by the API proxy. For example, if the base path of the API proxy is `/v1/weatherapikey`, then the API product supports requests to `/v1/weatherapikey` and to any sub-URIs, such as `/v1/weatherapikey/forecastrss`, `/v1/weatherapikey/region/CA`, and so on. For more information, see Managing API products. |
| `display_name` | String | Name displayed in the UI or developer portal to developers registering for API access. |
| `proxies` | Vec<String> | Comma-separated list of API proxy names to which this API product is bound. By specifying API proxies, you can associate resources in the API product with specific API proxies, preventing developers from accessing those resources through other API proxies. Apigee rejects requests to API proxies that are not listed. **Note:** The API proxy names must already exist in the specified environment as they will be validated upon creation. |
| `quota_counter_scope` | String | Scope of the quota decides how the quota counter gets applied and evaluate for quota violation. If the Scope is set as PROXY, then all the operations defined for the APIproduct that are associated with the same proxy will share the same quota counter set at the APIproduct level, making it a global counter at a proxy level. If the Scope is set as OPERATION, then each operations get the counter set at the API product dedicated, making it a local counter. Note that, the QuotaCounterScope applies only when an operation does not have dedicated quota set for itself. |
| `quota_interval` | String | Time interval over which the number of request messages is calculated. |
| `quota_time_unit` | String | Time unit defined for the `quotaInterval`. Valid values include `minute`, `hour`, `day`, or `month`. |
| `name` | String | Internal name of the API product. Characters you can use in the name are restricted to: `A-Z0-9._\-$ %`. **Note:** The internal name cannot be edited when updating the API product. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create apiproduct
apiproduct = provider.apigee_api.Apiproduct {
    parent = "value"  # Required. Name of the organization in which the API product will be created. Use the following structure in your request: `organizations/{org}` If the resource has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview).
}

# Access apiproduct outputs
apiproduct_id = apiproduct.id
apiproduct_space = apiproduct.space
apiproduct_description = apiproduct.description
apiproduct_scopes = apiproduct.scopes
apiproduct_last_modified_at = apiproduct.last_modified_at
apiproduct_attributes = apiproduct.attributes
apiproduct_quota = apiproduct.quota
apiproduct_operation_group = apiproduct.operation_group
apiproduct_environments = apiproduct.environments
apiproduct_approval_type = apiproduct.approval_type
apiproduct_graphql_operation_group = apiproduct.graphql_operation_group
apiproduct_created_at = apiproduct.created_at
apiproduct_grpc_operation_group = apiproduct.grpc_operation_group
apiproduct_api_resources = apiproduct.api_resources
apiproduct_display_name = apiproduct.display_name
apiproduct_proxies = apiproduct.proxies
apiproduct_quota_counter_scope = apiproduct.quota_counter_scope
apiproduct_quota_interval = apiproduct.quota_interval
apiproduct_quota_time_unit = apiproduct.quota_time_unit
apiproduct_name = apiproduct.name
```

---


### Create

Creates a custom consumer key and secret for a developer app. This is particularly useful if you want to migrate existing consumer keys and secrets to Apigee from another system. Consumer keys and secrets can contain letters, numbers, underscores, and hyphens. No other special characters are allowed. To avoid service disruptions, a consumer key and secret should not exceed 2 KBs each. **Note**: When creating the consumer key and secret, an association to API products will not be made. Therefore, you should not specify the associated API products in your request. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created. If a consumer key and secret already exist, you can keep them or delete them using the DeleteDeveloperAppKey API. **Note**: All keys start out with status=approved, even if status=revoked is passed when the key is created. To revoke a key, use the UpdateDeveloperAppKey API.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `consumer_secret` | String |  | Secret key. |
| `attributes` | Vec<String> |  | List of attributes associated with the credential. |
| `expires_at` | String |  | Time the developer app expires in milliseconds since epoch. |
| `expires_in_seconds` | String |  | Input only. Expiration time, in seconds, for the consumer key. If not set or left to the default value of `-1`, the API key never expires. The expiration time can't be updated after it is set. |
| `consumer_key` | String |  | Consumer key. |
| `issued_at` | String |  | Time the developer app was created in milliseconds since epoch. |
| `scopes` | Vec<String> |  | Scopes to apply to the app. The specified scope names must already be defined for the API product that you associate with the app. |
| `api_products` | Vec<String> |  | List of API products for which the credential can be used. **Note**: Do not specify the list of API products when creating a consumer key and secret for a developer app. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created. |
| `status` | String |  | Status of the credential. Valid values include `approved` or `revoked`. |
| `parent` | String | ✅ | Parent of the developer app key. Use the following structure in your request: 'organizations/{org}/developers/{developerEmail}/apps/{appName}' |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create create
create = provider.apigee_api.Create {
    parent = "value"  # Parent of the developer app key. Use the following structure in your request: 'organizations/{org}/developers/{developerEmail}/apps/{appName}'
}

```

---


### Key

Creates a custom consumer key and secret for a developer app. This is particularly useful if you want to migrate existing consumer keys and secrets to Apigee from another system. Consumer keys and secrets can contain letters, numbers, underscores, and hyphens. No other special characters are allowed. To avoid service disruptions, a consumer key and secret should not exceed 2 KBs each. **Note**: When creating the consumer key and secret, an association to API products will not be made. Therefore, you should not specify the associated API products in your request. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created. If a consumer key and secret already exist, you can keep them or delete them using the DeleteDeveloperAppKey API. **Note**: All keys start out with status=approved, even if status=revoked is passed when the key is created. To revoke a key, use the UpdateDeveloperAppKey API.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `consumer_secret` | String |  | Secret key. |
| `attributes` | Vec<String> |  | List of attributes associated with the credential. |
| `expires_at` | String |  | Time the developer app expires in milliseconds since epoch. |
| `expires_in_seconds` | String |  | Input only. Expiration time, in seconds, for the consumer key. If not set or left to the default value of `-1`, the API key never expires. The expiration time can't be updated after it is set. |
| `consumer_key` | String |  | Consumer key. |
| `issued_at` | String |  | Time the developer app was created in milliseconds since epoch. |
| `scopes` | Vec<String> |  | Scopes to apply to the app. The specified scope names must already be defined for the API product that you associate with the app. |
| `api_products` | Vec<String> |  | List of API products for which the credential can be used. **Note**: Do not specify the list of API products when creating a consumer key and secret for a developer app. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created. |
| `status` | String |  | Status of the credential. Valid values include `approved` or `revoked`. |
| `parent` | String | ✅ | Parent of the developer app key. Use the following structure in your request: 'organizations/{org}/developers/{developerEmail}/apps/{appName}' |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `consumer_secret` | String | Secret key. |
| `attributes` | Vec<String> | List of attributes associated with the credential. |
| `expires_at` | String | Time the developer app expires in milliseconds since epoch. |
| `expires_in_seconds` | String | Input only. Expiration time, in seconds, for the consumer key. If not set or left to the default value of `-1`, the API key never expires. The expiration time can't be updated after it is set. |
| `consumer_key` | String | Consumer key. |
| `issued_at` | String | Time the developer app was created in milliseconds since epoch. |
| `scopes` | Vec<String> | Scopes to apply to the app. The specified scope names must already be defined for the API product that you associate with the app. |
| `api_products` | Vec<String> | List of API products for which the credential can be used. **Note**: Do not specify the list of API products when creating a consumer key and secret for a developer app. Instead, use the UpdateDeveloperAppKey API to make the association after the consumer key and secret are created. |
| `status` | String | Status of the credential. Valid values include `approved` or `revoked`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create key
key = provider.apigee_api.Key {
    parent = "value"  # Parent of the developer app key. Use the following structure in your request: 'organizations/{org}/developers/{developerEmail}/apps/{appName}'
}

# Access key outputs
key_id = key.id
key_consumer_secret = key.consumer_secret
key_attributes = key.attributes
key_expires_at = key.expires_at
key_expires_in_seconds = key.expires_in_seconds
key_consumer_key = key.consumer_key
key_issued_at = key.issued_at
key_scopes = key.scopes
key_api_products = key.api_products
key_status = key.status
```

---


### Environment

CreateSecurityProfileEnvironmentAssociation creates profile environment association i.e. attaches environment to security profile.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_profile_revision_id` | String |  | DEPRECATED: DO NOT USE Revision ID of the security profile. |
| `attach_time` | String |  | Output only. The time when environment was attached to the security profile. |
| `name` | String |  | Immutable. Name of the environment that the profile is attached to. |
| `parent` | String | ✅ | Required. Name of organization and security profile ID. Format: organizations/{org}/securityProfiles/{profile} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_modified_at` | String | Output only. Last modification time of this environment as milliseconds since epoch. |
| `created_at` | String | Output only. Creation time of this environment as milliseconds since epoch. |
| `deployment_type` | String | Optional. Deployment type supported by the environment. The deployment type can be set when creating the environment and cannot be changed. When you enable archive deployment, you will be **prevented from performing** a [subset of actions](/apigee/docs/api-platform/local-development/overview#prevented-actions) within the environment, including: * Managing the deployment of API proxy or shared flow revisions * Creating, updating, or deleting resource files * Creating, updating, or deleting target servers |
| `display_name` | String | Optional. Display name for this environment. |
| `name` | String | Required. Name of the environment. Values must match the regular expression `^[.\\p{Alnum}-_]{1,255}$` |
| `forward_proxy_uri` | String | Optional. URI of the forward proxy to be applied to the runtime instances in this environment. Must be in the format of {scheme}://{hostname}:{port}. Note that the only supported scheme is "http". The port must be supplied. To remove a forward proxy setting, update the field to an empty value. Note: At this time, PUT operations to add forwardProxyUri to an existing environment fail if the environment has nodeConfig set up. To successfully add the forwardProxyUri setting in this case, include the NodeConfig details with the request. |
| `client_ip_resolution_config` | String | Optional. The algorithm to resolve IP. This will affect Analytics, API Security, and other features that use the client ip. To remove a client ip resolution config, update the field to an empty value. Example: '{ "clientIpResolutionConfig" = {} }' For more information, see: https://cloud.google.com/apigee/docs/api-platform/system-administration/client-ip-resolution. |
| `type` | String | Optional. EnvironmentType selected for the environment. |
| `state` | String | Output only. State of the environment. Values other than ACTIVE means the resource is not ready to use. |
| `node_config` | String | Optional. NodeConfig of the environment. |
| `has_attached_flow_hooks` | bool |  |
| `description` | String | Optional. Description of the environment. |
| `api_proxy_type` | String | Optional. API Proxy type supported by the environment. The type can be set when creating the Environment and cannot be changed. |
| `properties` | String | Optional. Key-value pairs that may be used for customizing the environment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.apigee_api.Environment {
    parent = "value"  # Required. Name of organization and security profile ID. Format: organizations/{org}/securityProfiles/{profile}
}

# Access environment outputs
environment_id = environment.id
environment_last_modified_at = environment.last_modified_at
environment_created_at = environment.created_at
environment_deployment_type = environment.deployment_type
environment_display_name = environment.display_name
environment_name = environment.name
environment_forward_proxy_uri = environment.forward_proxy_uri
environment_client_ip_resolution_config = environment.client_ip_resolution_config
environment_type = environment.type
environment_state = environment.state
environment_node_config = environment.node_config
environment_has_attached_flow_hooks = environment.has_attached_flow_hooks
environment_description = environment.description
environment_api_proxy_type = environment.api_proxy_type
environment_properties = environment.properties
```

---


### Datacollector

Creates a new data collector.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | ID of the data collector. Must begin with `dc_`. |
| `last_modified_at` | String |  | Output only. The time at which the Data Collector was last updated in milliseconds since the epoch. |
| `description` | String |  | A description of the data collector. |
| `type` | String |  | Immutable. The type of data this data collector will collect. |
| `created_at` | String |  | Output only. The time at which the data collector was created in milliseconds since the epoch. |
| `parent` | String | ✅ | Required. Name of the organization in which to create the data collector in the following format: `organizations/{org}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | ID of the data collector. Must begin with `dc_`. |
| `last_modified_at` | String | Output only. The time at which the Data Collector was last updated in milliseconds since the epoch. |
| `description` | String | A description of the data collector. |
| `type` | String | Immutable. The type of data this data collector will collect. |
| `created_at` | String | Output only. The time at which the data collector was created in milliseconds since the epoch. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create datacollector
datacollector = provider.apigee_api.Datacollector {
    parent = "value"  # Required. Name of the organization in which to create the data collector in the following format: `organizations/{org}`.
}

# Access datacollector outputs
datacollector_id = datacollector.id
datacollector_name = datacollector.name
datacollector_last_modified_at = datacollector.last_modified_at
datacollector_description = datacollector.description
datacollector_type = datacollector.type
datacollector_created_at = datacollector.created_at
```

---


### Endpoint_attachment

Creates an endpoint attachment. **Note:** Not supported for Apigee hybrid.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `host` | String |  | Output only. Host that can be used in either the HTTP target endpoint directly or as the host in target server. |
| `name` | String |  | Name of the endpoint attachment. Use the following structure in your request: `organizations/{org}/endpointAttachments/{endpoint_attachment}` |
| `state` | String |  | Output only. State of the endpoint attachment. Values other than `ACTIVE` mean the resource is not ready to use. |
| `connection_state` | String |  | Output only. State of the endpoint attachment connection to the service attachment. |
| `location` | String |  | Required. Location of the endpoint attachment. |
| `service_attachment` | String |  | Format: projects/*/regions/*/serviceAttachments/* |
| `parent` | String | ✅ | Required. Organization the endpoint attachment will be created in. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `host` | String | Output only. Host that can be used in either the HTTP target endpoint directly or as the host in target server. |
| `name` | String | Name of the endpoint attachment. Use the following structure in your request: `organizations/{org}/endpointAttachments/{endpoint_attachment}` |
| `state` | String | Output only. State of the endpoint attachment. Values other than `ACTIVE` mean the resource is not ready to use. |
| `connection_state` | String | Output only. State of the endpoint attachment connection to the service attachment. |
| `location` | String | Required. Location of the endpoint attachment. |
| `service_attachment` | String | Format: projects/*/regions/*/serviceAttachments/* |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create endpoint_attachment
endpoint_attachment = provider.apigee_api.Endpoint_attachment {
    parent = "value"  # Required. Organization the endpoint attachment will be created in.
}

# Access endpoint_attachment outputs
endpoint_attachment_id = endpoint_attachment.id
endpoint_attachment_host = endpoint_attachment.host
endpoint_attachment_name = endpoint_attachment.name
endpoint_attachment_state = endpoint_attachment.state
endpoint_attachment_connection_state = endpoint_attachment.connection_state
endpoint_attachment_location = endpoint_attachment.location
endpoint_attachment_service_attachment = endpoint_attachment.service_attachment
```

---


### Keystore

Creates a keystore or truststore. - Keystore: Contains certificates and their associated keys. - Truststore: Contains trusted certificates used to validate a server's certificate. These certificates are typically self-signed certificates or certificates that are not signed by a trusted CA.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `aliases` | Vec<String> |  | Output only. Aliases in this keystore. |
| `name` | String |  | Required. Resource ID for this keystore. Values must match the regular expression `[\w[:space:].-]{1,255}`. |
| `parent` | String | ✅ | Required. Name of the environment in which to create the keystore. Use the following format in your request: `organizations/{org}/environments/{env}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `aliases` | Vec<String> | Output only. Aliases in this keystore. |
| `name` | String | Required. Resource ID for this keystore. Values must match the regular expression `[\w[:space:].-]{1,255}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create keystore
keystore = provider.apigee_api.Keystore {
    parent = "value"  # Required. Name of the environment in which to create the keystore. Use the following format in your request: `organizations/{org}/environments/{env}`
}

# Access keystore outputs
keystore_id = keystore.id
keystore_aliases = keystore.aliases
keystore_name = keystore.name
```

---


### Flowhook

Returns the name of the shared flow attached to the specified flow hook. If there's no shared flow attached to the flow hook, the API does not return an error; it simply does not return a name in the response.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Description of the flow hook. |
| `continue_on_error` | bool |  | Optional. Flag that specifies whether execution should continue if the flow hook throws an exception. Set to `true` to continue execution. Set to `false` to stop execution if the flow hook throws an exception. Defaults to `true`. |
| `shared_flow` | String |  | Shared flow attached to this flow hook, or empty if there is none attached. |
| `flow_hook_point` | String |  | Output only. Where in the API call flow the flow hook is invoked. Must be one of `PreProxyFlowHook`, `PostProxyFlowHook`, `PreTargetFlowHook`, or `PostTargetFlowHook`. |
| `name` | String | ✅ | Required. Name of the flow hook to which the shared flow should be attached in the following format: `organizations/{org}/environments/{env}/flowhooks/{flowhook}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Description of the flow hook. |
| `continue_on_error` | bool | Optional. Flag that specifies whether execution should continue if the flow hook throws an exception. Set to `true` to continue execution. Set to `false` to stop execution if the flow hook throws an exception. Defaults to `true`. |
| `shared_flow` | String | Shared flow attached to this flow hook, or empty if there is none attached. |
| `flow_hook_point` | String | Output only. Where in the API call flow the flow hook is invoked. Must be one of `PreProxyFlowHook`, `PostProxyFlowHook`, `PreTargetFlowHook`, or `PostTargetFlowHook`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access flowhook outputs
flowhook_id = flowhook.id
flowhook_description = flowhook.description
flowhook_continue_on_error = flowhook.continue_on_error
flowhook_shared_flow = flowhook.shared_flow
flowhook_flow_hook_point = flowhook.flow_hook_point
```

---


### Cache



**Operations**: ✅ Delete

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

```

---


### Host_security_report

Submit a query at host level to be processed in the background. If the submission of the query succeeds, the API returns a 201 status and an ID that refer to the query. In addition to the HTTP status 201, the `state` of "enqueued" means that the request succeeded.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `time_range` | String |  | Required. Time range for the query. Can use the following predefined strings to specify the time range: `last60minutes` `last24hours` `last7days` Or, specify the timeRange as a structure describing start and end timestamps in the ISO format: yyyy-mm-ddThh:mm:ssZ. Example: "timeRange": { "start": "2018-07-29T00:13:00Z", "end": "2018-08-01T00:18:00Z" } |
| `mime_type` | String |  | Valid values include: `csv` or `json`. Defaults to `json`. Note: Configure the delimiter for CSV output using the csvDelimiter property. |
| `filter` | String |  | Boolean expression that can be used to filter data. Filter expressions can be combined using AND/OR terms and should be fully parenthesized to avoid ambiguity. See Analytics metrics, dimensions, and filters reference https://docs.apigee.com/api-platform/analytics/analytics-reference for more information on the fields available to filter on. For more information on the tokens that you use to build filter expressions, see Filter expression syntax. https://docs.apigee.com/api-platform/analytics/asynch-reports-api#filter-expression-syntax |
| `csv_delimiter` | String |  | Delimiter used in the CSV file, if `outputFormat` is set to `csv`. Defaults to the `,` (comma) character. Supported delimiter characters include comma (`,`), pipe (`|`), and tab (`\t`). |
| `dimensions` | Vec<String> |  | A list of dimensions. https://docs.apigee.com/api-platform/analytics/analytics-reference#dimensions |
| `limit` | i64 |  | Maximum number of rows that can be returned in the result. |
| `group_by_time_unit` | String |  | Time unit used to group the result set. Valid values include: second, minute, hour, day, week, or month. If a query includes groupByTimeUnit, then the result is an aggregation based on the specified time unit and the resultant timestamp does not include milliseconds precision. If a query omits groupByTimeUnit, then the resultant timestamp includes milliseconds precision. |
| `metrics` | Vec<String> |  | A list of Metrics. |
| `display_name` | String |  | Security Report display name which users can specify. |
| `envgroup_hostname` | String |  | Hostname needs to be specified if query intends to run at host level. This field is only allowed when query is submitted by CreateHostSecurityReport where analytics data will be grouped by organization and hostname. |
| `report_definition_id` | String |  | Report Definition ID. |
| `parent` | String | ✅ | Required. The parent resource name. Must be of the form `organizations/{org}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `result` | String | Result is available only after the query is completed. |
| `report_definition_id` | String | Report Definition ID. |
| `created` | String | Creation time of the query. |
| `result_file_size` | String | ResultFileSize is available only after the query is completed. |
| `execution_time` | String | ExecutionTime is available only after the query is completed. |
| `envgroup_hostname` | String | Hostname is available only when query is executed at host level. |
| `state` | String | Query state could be "enqueued", "running", "completed", "expired" and "failed". |
| `error` | String | Error is set when query fails. |
| `query_params` | String | Contains information like metrics, dimenstions etc of the Security Report. |
| `result_rows` | String | ResultRows is available only after the query is completed. |
| `updated` | String | Output only. Last updated timestamp for the query. |
| `self` | String | Self link of the query. Example: `/organizations/myorg/environments/myenv/securityReports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` or following format if query is running at host level: `/organizations/myorg/hostSecurityReports/9cfc0d85-0f30-46d6-ae6f-318d0cb961bd` |
| `display_name` | String | Display Name specified by the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create host_security_report
host_security_report = provider.apigee_api.Host_security_report {
    parent = "value"  # Required. The parent resource name. Must be of the form `organizations/{org}`.
}

# Access host_security_report outputs
host_security_report_id = host_security_report.id
host_security_report_result = host_security_report.result
host_security_report_report_definition_id = host_security_report.report_definition_id
host_security_report_created = host_security_report.created
host_security_report_result_file_size = host_security_report.result_file_size
host_security_report_execution_time = host_security_report.execution_time
host_security_report_envgroup_hostname = host_security_report.envgroup_hostname
host_security_report_state = host_security_report.state
host_security_report_error = host_security_report.error
host_security_report_query_params = host_security_report.query_params
host_security_report_result_rows = host_security_report.result_rows
host_security_report_updated = host_security_report.updated
host_security_report_self = host_security_report.self
host_security_report_display_name = host_security_report.display_name
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


### Revision

Updates an existing API proxy revision by uploading the API proxy configuration bundle as a zip file from your local machine. You can update only API proxy revisions that have never been deployed. After deployment, an API proxy revision becomes immutable, even if it is undeployed. Set the `Content-Type` header to either `multipart/form-data` or `application/octet-stream`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `data` | String |  | The HTTP request/response body as raw binary. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `name` | String | ✅ | Required. API proxy revision to update in the following format: `organizations/{org}/apis/{api}/revisions/{rev}` If the API Proxy resource has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `data` | String | The HTTP request/response body as raw binary. |
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create revision
revision = provider.apigee_api.Revision {
    name = "value"  # Required. API proxy revision to update in the following format: `organizations/{org}/apis/{api}/revisions/{rev}` If the API Proxy resource has the `space` attribute set, IAM permissions are checked against the Space resource path. To learn more, read the [Apigee Spaces Overview](https://cloud.google.com/apigee/docs/api-platform/system-administration/spaces/apigee-spaces-overview).
}

# Access revision outputs
revision_id = revision.id
revision_extensions = revision.extensions
revision_data = revision.data
revision_content_type = revision.content_type
```

---


### Attachment

Creates a new attachment of an environment to an instance. **Note:** Not supported for Apigee hybrid.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. ID of the attachment. |
| `created_at` | String |  | Output only. Time the attachment was created in milliseconds since epoch. |
| `environment` | String |  | ID of the attached environment. |
| `parent` | String | ✅ | Required. Name of the instance. Use the following structure in your request: `organizations/{org}/instances/{instance}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. ID of the attachment. |
| `created_at` | String | Output only. Time the attachment was created in milliseconds since epoch. |
| `environment` | String | ID of the attached environment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attachment
attachment = provider.apigee_api.Attachment {
    parent = "value"  # Required. Name of the instance. Use the following structure in your request: `organizations/{org}/instances/{instance}`.
}

# Access attachment outputs
attachment_id = attachment.id
attachment_name = attachment.name
attachment_created_at = attachment.created_at
attachment_environment = attachment.environment
```

---


### Balance

Adjust the prepaid balance for the developer. This API will be used in scenarios where the developer has been under-charged or over-charged.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `adjustment` | String |  | * A positive value of `adjustment` means that that the API provider wants to adjust the balance for an under-charged developer i.e. the balance of the developer will decrease. * A negative value of `adjustment` means that that the API provider wants to adjust the balance for an over-charged developer i.e. the balance of the developer will increase. NOTE: An adjustment cannot increase the balance of the developer beyond the balance as of the most recent credit. For example, if a developer's balance is updated to be $100, and they spend $10, a negative adjustment can only increase the balance of the developer to $100. |
| `name` | String | ✅ | Required. Account balance for the developer. Use the following structure in your request: `organizations/{org}/developers/{developer}/balance` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create balance
balance = provider.apigee_api.Balance {
    name = "value"  # Required. Account balance for the developer. Use the following structure in your request: `organizations/{org}/developers/{developer}/balance`
}

```

---


### Canaryevaluation

Creates a new canary evaluation for an organization.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `treatment` | String |  | Required. The newer version that is serving requests. |
| `control` | String |  | Required. The stable version that is serving requests. |
| `create_time` | String |  | Output only. Create time of the canary evaluation. |
| `metric_labels` | String |  | Required. Labels used to filter the metrics used for a canary evaluation. |
| `end_time` | String |  | Required. End time for the evaluation's analysis. |
| `state` | String |  | Output only. The current state of the canary evaluation. |
| `name` | String |  | Output only. Name of the canary evalution. |
| `start_time` | String |  | Required. Start time for the canary evaluation's analysis. |
| `verdict` | String |  | Output only. The resulting verdict of the canary evaluations: NONE, PASS, or FAIL. |
| `parent` | String | ✅ | Required. Name of the organization. Use the following structure in your request: `organizations/{org}/instances/{instance}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `treatment` | String | Required. The newer version that is serving requests. |
| `control` | String | Required. The stable version that is serving requests. |
| `create_time` | String | Output only. Create time of the canary evaluation. |
| `metric_labels` | String | Required. Labels used to filter the metrics used for a canary evaluation. |
| `end_time` | String | Required. End time for the evaluation's analysis. |
| `state` | String | Output only. The current state of the canary evaluation. |
| `name` | String | Output only. Name of the canary evalution. |
| `start_time` | String | Required. Start time for the canary evaluation's analysis. |
| `verdict` | String | Output only. The resulting verdict of the canary evaluations: NONE, PASS, or FAIL. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create canaryevaluation
canaryevaluation = provider.apigee_api.Canaryevaluation {
    parent = "value"  # Required. Name of the organization. Use the following structure in your request: `organizations/{org}/instances/{instance}`.
}

# Access canaryevaluation outputs
canaryevaluation_id = canaryevaluation.id
canaryevaluation_treatment = canaryevaluation.treatment
canaryevaluation_control = canaryevaluation.control
canaryevaluation_create_time = canaryevaluation.create_time
canaryevaluation_metric_labels = canaryevaluation.metric_labels
canaryevaluation_end_time = canaryevaluation.end_time
canaryevaluation_state = canaryevaluation.state
canaryevaluation_name = canaryevaluation.name
canaryevaluation_start_time = canaryevaluation.start_time
canaryevaluation_verdict = canaryevaluation.verdict
```

---


### Dns_zone

Creates a new DNS zone.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the DNS Peering. Values other than `ACTIVE` mean the resource is not ready to use. |
| `update_time` | String |  | Output only. The time that this resource was updated on the server. |
| `name` | String |  | Identifier. Unique name for the resource. Defined by the server Format: "organizations/{organization}/dnsZones/{dns_zone}". |
| `create_time` | String |  | Output only. The time that this resource was created on the server. |
| `description` | String |  | Required. Description of the resource. String of at most 1024 characters associated with this resource for the user's convenience. |
| `domain` | String |  | Required. The domain name for hosts in this private zone, for instance "example.com.". |
| `peering_config` | String |  | DNS PEERING zone configuration. |
| `parent` | String | ✅ | Required. Organization where the DNS zone will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the DNS Peering. Values other than `ACTIVE` mean the resource is not ready to use. |
| `update_time` | String | Output only. The time that this resource was updated on the server. |
| `name` | String | Identifier. Unique name for the resource. Defined by the server Format: "organizations/{organization}/dnsZones/{dns_zone}". |
| `create_time` | String | Output only. The time that this resource was created on the server. |
| `description` | String | Required. Description of the resource. String of at most 1024 characters associated with this resource for the user's convenience. |
| `domain` | String | Required. The domain name for hosts in this private zone, for instance "example.com.". |
| `peering_config` | String | DNS PEERING zone configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dns_zone
dns_zone = provider.apigee_api.Dns_zone {
    parent = "value"  # Required. Organization where the DNS zone will be created.
}

# Access dns_zone outputs
dns_zone_id = dns_zone.id
dns_zone_state = dns_zone.state
dns_zone_update_time = dns_zone.update_time
dns_zone_name = dns_zone.name
dns_zone_create_time = dns_zone.create_time
dns_zone_description = dns_zone.description
dns_zone_domain = dns_zone.domain
dns_zone_peering_config = dns_zone.peering_config
```

---


### Override

Creates a trace configuration override. The response contains a system-generated UUID, that can be used to view, update, or delete the configuration override. Use the List API to view the existing trace configuration overrides.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sampling_config` | String |  | Trace configuration to override. |
| `name` | String |  | ID of the trace configuration override specified as a system-generated UUID. |
| `api_proxy` | String |  | ID of the API proxy that will have its trace configuration overridden. |
| `parent` | String | ✅ | Required. Parent resource of the trace configuration override. Use the following structure in your request. "organizations/*/environments/*/traceConfig". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sampling_config` | String | Trace configuration to override. |
| `name` | String | ID of the trace configuration override specified as a system-generated UUID. |
| `api_proxy` | String | ID of the API proxy that will have its trace configuration overridden. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create override
override = provider.apigee_api.Override {
    parent = "value"  # Required. Parent resource of the trace configuration override. Use the following structure in your request. "organizations/*/environments/*/traceConfig".
}

# Access override outputs
override_id = override.id
override_sampling_config = override.sampling_config
override_name = override.name
override_api_proxy = override.api_proxy
```

---


### Aliase

Creates an alias from a key/certificate pair. The structure of the request is controlled by the `format` query parameter: - `keycertfile` - Separate PEM-encoded key and certificate files are uploaded. Set `Content-Type: multipart/form-data` and include the `keyFile`, `certFile`, and `password` (if keys are encrypted) fields in the request body. If uploading to a truststore, omit `keyFile`. - `pkcs12` - A PKCS12 file is uploaded. Set `Content-Type: multipart/form-data`, provide the file in the `file` field, and include the `password` field if the file is encrypted in the request body. - `selfsignedcert` - A new private key and certificate are generated. Set `Content-Type: application/json` and include CertificateGenerationSpec in the request body.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `data` | String |  | The HTTP request/response body as raw binary. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `parent` | String | ✅ | Required. Name of the keystore. Use the following format in your request: `organizations/{org}/environments/{env}/keystores/{keystore}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certs_info` | String | Chain of certificates under this alias. |
| `type` | String | Type of alias. |
| `alias` | String | Resource ID for this alias. Values must match the regular expression `[^/]{1,255}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create aliase
aliase = provider.apigee_api.Aliase {
    parent = "value"  # Required. Name of the keystore. Use the following format in your request: `organizations/{org}/environments/{env}/keystores/{keystore}`.
}

# Access aliase outputs
aliase_id = aliase.id
aliase_certs_info = aliase.certs_info
aliase_type = aliase.type
aliase_alias = aliase.alias
```

---


### Security_profile

CreateSecurityProfile create a new custom security profile.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | DEPRECATED: DO NOT USE Display name of the security profile. |
| `revision_id` | String |  | Output only. Revision ID of the security profile. |
| `revision_update_time` | String |  | Output only. The time when revision was updated. |
| `revision_publish_time` | String |  | Output only. DEPRECATED: DO NOT USE The time when revision was published. Once published, the security profile revision cannot be updated further and can be attached to environments. |
| `scoring_configs` | Vec<String> |  | List of profile scoring configs in this revision. |
| `environments` | Vec<String> |  | List of environments attached to security profile. |
| `max_score` | i64 |  | Output only. Maximum security score that can be generated by this profile. |
| `name` | String |  | Immutable. Name of the security profile resource. Format: organizations/{org}/securityProfiles/{profile} |
| `profile_config` | String |  | Required. Customized profile configuration that computes the security score. |
| `description` | String |  | Description of the security profile. |
| `min_score` | i64 |  | Output only. Minimum security score that can be generated by this profile. |
| `revision_create_time` | String |  | Output only. The time when revision was created. |
| `parent` | String | ✅ | Required. Name of organization. Format: organizations/{org} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | DEPRECATED: DO NOT USE Display name of the security profile. |
| `revision_id` | String | Output only. Revision ID of the security profile. |
| `revision_update_time` | String | Output only. The time when revision was updated. |
| `revision_publish_time` | String | Output only. DEPRECATED: DO NOT USE The time when revision was published. Once published, the security profile revision cannot be updated further and can be attached to environments. |
| `scoring_configs` | Vec<String> | List of profile scoring configs in this revision. |
| `environments` | Vec<String> | List of environments attached to security profile. |
| `max_score` | i64 | Output only. Maximum security score that can be generated by this profile. |
| `name` | String | Immutable. Name of the security profile resource. Format: organizations/{org}/securityProfiles/{profile} |
| `profile_config` | String | Required. Customized profile configuration that computes the security score. |
| `description` | String | Description of the security profile. |
| `min_score` | i64 | Output only. Minimum security score that can be generated by this profile. |
| `revision_create_time` | String | Output only. The time when revision was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_profile
security_profile = provider.apigee_api.Security_profile {
    parent = "value"  # Required. Name of organization. Format: organizations/{org}
}

# Access security_profile outputs
security_profile_id = security_profile.id
security_profile_display_name = security_profile.display_name
security_profile_revision_id = security_profile.revision_id
security_profile_revision_update_time = security_profile.revision_update_time
security_profile_revision_publish_time = security_profile.revision_publish_time
security_profile_scoring_configs = security_profile.scoring_configs
security_profile_environments = security_profile.environments
security_profile_max_score = security_profile.max_score
security_profile_name = security_profile.name
security_profile_profile_config = security_profile.profile_config
security_profile_description = security_profile.description
security_profile_min_score = security_profile.min_score
security_profile_revision_create_time = security_profile.revision_create_time
```

---


### Instance

Creates an Apigee runtime instance. The instance is accessible from the authorized network configured on the organization. **Note:** Not supported for Apigee hybrid.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scheduled_maintenance` | String |  | Output only. Time and date of the scheduled maintenance for this instance. This field is only populated for instances that have opted into Maintenance Window and if there is an upcoming maintenance. Cleared once the maintenance is complete. |
| `name` | String |  | Required. Resource ID of the instance. Values must match the regular expression `^a-z{0,30}[a-z\d]$`. |
| `last_modified_at` | String |  | Output only. Time the instance was last modified in milliseconds since epoch. |
| `description` | String |  | Optional. Description of the instance. |
| `disk_encryption_key_name` | String |  | Optional. Customer Managed Encryption Key (CMEK) used for disk and volume encryption. If not specified, a Google-Managed encryption key will be used. Use the following format: `projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)` |
| `maintenance_update_policy` | String |  | Optional. Apigee customers can set the preferred window to perform maintenance on the instance (day of the week and time of day). |
| `runtime_version` | String |  | Output only. Version of the runtime system running in the instance. The runtime system is the set of components that serve the API Proxy traffic in your Environments. |
| `host` | String |  | Output only. Internal hostname or IP address of the Apigee endpoint used by clients to connect to the service. |
| `location` | String |  | Required. Compute Engine location where the instance resides. |
| `port` | String |  | Output only. Port number of the exposed Apigee endpoint. |
| `peering_cidr_range` | String |  | Optional. Size of the CIDR block range that will be reserved by the instance. PAID organizations support `SLASH_16` to `SLASH_20` and defaults to `SLASH_16`. Evaluation organizations support only `SLASH_23`. |
| `ip_range` | String |  | Optional. Comma-separated list of CIDR blocks of length 22 and/or 28 used to create the Apigee instance. Providing CIDR ranges is optional. You can provide just /22 or /28 or both (or neither). Ranges you provide should be freely available as part of a larger named range you have allocated to the Service Networking peering. If this parameter is not provided, Apigee automatically requests an available /22 and /28 CIDR block from Service Networking. Use the /22 CIDR block for configuring your firewall needs to allow traffic from Apigee. Input formats: `a.b.c.d/22` or `e.f.g.h/28` or `a.b.c.d/22,e.f.g.h/28` |
| `consumer_accept_list` | Vec<String> |  | Optional. Customer accept list represents the list of projects (id/number) on customer side that can privately connect to the service attachment. It is an optional field which the customers can provide during the instance creation. By default, the customer project associated with the Apigee organization will be included to the list. |
| `created_at` | String |  | Output only. Time the instance was created in milliseconds since epoch. |
| `display_name` | String |  | Optional. Display name for the instance. |
| `service_attachment` | String |  | Output only. Resource name of the service attachment created for the instance in the format: `projects/*/regions/*/serviceAttachments/*` Apigee customers can privately forward traffic to this service attachment using the PSC endpoints. |
| `is_version_locked` | bool |  | Output only. Indicates whether the instance is version locked. If true, the instance will not be updated by automated runtime rollouts. This is only supported for Apigee X instances. |
| `access_logging_config` | String |  | Optional. Access logging configuration enables the access logging feature at the instance. Apigee customers can enable access logging to ship the access logs to their own project's cloud logging. |
| `state` | String |  | Output only. State of the instance. Values other than `ACTIVE` means the resource is not ready to use. |
| `parent` | String | ✅ | Required. Name of the organization. Use the following structure in your request: `organizations/{org}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scheduled_maintenance` | String | Output only. Time and date of the scheduled maintenance for this instance. This field is only populated for instances that have opted into Maintenance Window and if there is an upcoming maintenance. Cleared once the maintenance is complete. |
| `name` | String | Required. Resource ID of the instance. Values must match the regular expression `^a-z{0,30}[a-z\d]$`. |
| `last_modified_at` | String | Output only. Time the instance was last modified in milliseconds since epoch. |
| `description` | String | Optional. Description of the instance. |
| `disk_encryption_key_name` | String | Optional. Customer Managed Encryption Key (CMEK) used for disk and volume encryption. If not specified, a Google-Managed encryption key will be used. Use the following format: `projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)` |
| `maintenance_update_policy` | String | Optional. Apigee customers can set the preferred window to perform maintenance on the instance (day of the week and time of day). |
| `runtime_version` | String | Output only. Version of the runtime system running in the instance. The runtime system is the set of components that serve the API Proxy traffic in your Environments. |
| `host` | String | Output only. Internal hostname or IP address of the Apigee endpoint used by clients to connect to the service. |
| `location` | String | Required. Compute Engine location where the instance resides. |
| `port` | String | Output only. Port number of the exposed Apigee endpoint. |
| `peering_cidr_range` | String | Optional. Size of the CIDR block range that will be reserved by the instance. PAID organizations support `SLASH_16` to `SLASH_20` and defaults to `SLASH_16`. Evaluation organizations support only `SLASH_23`. |
| `ip_range` | String | Optional. Comma-separated list of CIDR blocks of length 22 and/or 28 used to create the Apigee instance. Providing CIDR ranges is optional. You can provide just /22 or /28 or both (or neither). Ranges you provide should be freely available as part of a larger named range you have allocated to the Service Networking peering. If this parameter is not provided, Apigee automatically requests an available /22 and /28 CIDR block from Service Networking. Use the /22 CIDR block for configuring your firewall needs to allow traffic from Apigee. Input formats: `a.b.c.d/22` or `e.f.g.h/28` or `a.b.c.d/22,e.f.g.h/28` |
| `consumer_accept_list` | Vec<String> | Optional. Customer accept list represents the list of projects (id/number) on customer side that can privately connect to the service attachment. It is an optional field which the customers can provide during the instance creation. By default, the customer project associated with the Apigee organization will be included to the list. |
| `created_at` | String | Output only. Time the instance was created in milliseconds since epoch. |
| `display_name` | String | Optional. Display name for the instance. |
| `service_attachment` | String | Output only. Resource name of the service attachment created for the instance in the format: `projects/*/regions/*/serviceAttachments/*` Apigee customers can privately forward traffic to this service attachment using the PSC endpoints. |
| `is_version_locked` | bool | Output only. Indicates whether the instance is version locked. If true, the instance will not be updated by automated runtime rollouts. This is only supported for Apigee X instances. |
| `access_logging_config` | String | Optional. Access logging configuration enables the access logging feature at the instance. Apigee customers can enable access logging to ship the access logs to their own project's cloud logging. |
| `state` | String | Output only. State of the instance. Values other than `ACTIVE` means the resource is not ready to use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.apigee_api.Instance {
    parent = "value"  # Required. Name of the organization. Use the following structure in your request: `organizations/{org}`.
}

# Access instance outputs
instance_id = instance.id
instance_scheduled_maintenance = instance.scheduled_maintenance
instance_name = instance.name
instance_last_modified_at = instance.last_modified_at
instance_description = instance.description
instance_disk_encryption_key_name = instance.disk_encryption_key_name
instance_maintenance_update_policy = instance.maintenance_update_policy
instance_runtime_version = instance.runtime_version
instance_host = instance.host
instance_location = instance.location
instance_port = instance.port
instance_peering_cidr_range = instance.peering_cidr_range
instance_ip_range = instance.ip_range
instance_consumer_accept_list = instance.consumer_accept_list
instance_created_at = instance.created_at
instance_display_name = instance.display_name
instance_service_attachment = instance.service_attachment
instance_is_version_locked = instance.is_version_locked
instance_access_logging_config = instance.access_logging_config
instance_state = instance.state
```

---


### Host_stat

Retrieve metrics grouped by dimensions in host level. The types of metrics you can retrieve include traffic, message counts, API call latency, response size, and cache hits and counts. Dimensions let you view metrics in meaningful groups. You can optionally pass dimensions as path parameters to the `stats` API. If dimensions are not specified, the metrics are computed on the entire set of data for the given time range.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hosts` | Vec<String> | List of query results grouped by host. |
| `environments` | Vec<String> | List of query results on the environment level. |
| `meta_data` | String | Metadata information. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access host_stat outputs
host_stat_id = host_stat.id
host_stat_hosts = host_stat.hosts
host_stat_environments = host_stat.environments
host_stat_meta_data = host_stat.meta_data
```

---


### Security_profiles_v2

Create a security profile v2.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time of the security profile creation. |
| `name` | String |  | Identifier. Name of the security profile v2 resource. Format: organizations/{org}/securityProfilesV2/{profile} |
| `google_defined` | bool |  | Output only. Whether the security profile is google defined. |
| `profile_assessment_configs` | HashMap<String, String> |  | Required. The configuration for each assessment in this profile. Key is the name/id of the assessment. |
| `update_time` | String |  | Output only. The time of the security profile update. |
| `description` | String |  | Optional. The description of the security profile. |
| `parent` | String | ✅ | Required. The parent resource name. Format: `organizations/{org}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time of the security profile creation. |
| `name` | String | Identifier. Name of the security profile v2 resource. Format: organizations/{org}/securityProfilesV2/{profile} |
| `google_defined` | bool | Output only. Whether the security profile is google defined. |
| `profile_assessment_configs` | HashMap<String, String> | Required. The configuration for each assessment in this profile. Key is the name/id of the assessment. |
| `update_time` | String | Output only. The time of the security profile update. |
| `description` | String | Optional. The description of the security profile. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_profiles_v2
security_profiles_v2 = provider.apigee_api.Security_profiles_v2 {
    parent = "value"  # Required. The parent resource name. Format: `organizations/{org}`
}

# Access security_profiles_v2 outputs
security_profiles_v2_id = security_profiles_v2.id
security_profiles_v2_create_time = security_profiles_v2.create_time
security_profiles_v2_name = security_profiles_v2.name
security_profiles_v2_google_defined = security_profiles_v2.google_defined
security_profiles_v2_profile_assessment_configs = security_profiles_v2.profile_assessment_configs
security_profiles_v2_update_time = security_profiles_v2.update_time
security_profiles_v2_description = security_profiles_v2.description
```

---


### Report

Creates a Custom Report for an Organization. A Custom Report provides Apigee Customers to create custom dashboards in addition to the standard dashboards which are provided. The Custom Report in its simplest form contains specifications about metrics, dimensions and filters. It is important to note that the custom report by itself does not provide an executable entity. The Edge UI converts the custom report definition into an analytics query and displays the result in a chart.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | This field contains the filter expression |
| `sort_order` | String |  | Legacy field: not used much. Contains the sort order for the sort columns |
| `comments` | Vec<String> |  | Legacy field: not used. This field contains a list of comments associated with custom report |
| `last_modified_at` | String |  | Output only. Modified time of this entity as milliseconds since epoch. json key: lastModifiedAt |
| `created_at` | String |  | Output only. Unix time when the app was created json key: createdAt |
| `name` | String |  | Required. Unique identifier for the report T his is a legacy field used to encode custom report unique id |
| `properties` | Vec<String> |  | This field contains report properties such as ui metadata etc. |
| `limit` | String |  | Legacy field: not used This field contains the limit for the result retrieved |
| `last_viewed_at` | String |  | Output only. Last viewed time of this entity as milliseconds since epoch |
| `organization` | String |  | Output only. Organization name |
| `tags` | Vec<String> |  | Legacy field: not used. This field contains a list of tags associated with custom report |
| `dimensions` | Vec<String> |  | This contains the list of dimensions for the report |
| `display_name` | String |  | This is the display name for the report |
| `time_unit` | String |  | This field contains the time unit of aggregation for the report |
| `to_time` | String |  | Legacy field: not used. Contains the end time for the report |
| `topk` | String |  | Legacy field: not used. This field contains the top k parameter value for restricting the result |
| `metrics` | Vec<String> |  | Required. This contains the list of metrics |
| `chart_type` | String |  | This field contains the chart type for the report |
| `environment` | String |  | Output only. Environment name |
| `from_time` | String |  | Legacy field: not used. Contains the from time for the report |
| `offset` | String |  | Legacy field: not used. This field contains the offset for the data |
| `sort_by_cols` | Vec<String> |  | Legacy field: not used much. Contains the list of sort by columns |
| `parent` | String | ✅ | Required. The parent organization name under which the Custom Report will be created. Must be of the form: `organizations/{organization_id}/reports` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `filter` | String | This field contains the filter expression |
| `sort_order` | String | Legacy field: not used much. Contains the sort order for the sort columns |
| `comments` | Vec<String> | Legacy field: not used. This field contains a list of comments associated with custom report |
| `last_modified_at` | String | Output only. Modified time of this entity as milliseconds since epoch. json key: lastModifiedAt |
| `created_at` | String | Output only. Unix time when the app was created json key: createdAt |
| `name` | String | Required. Unique identifier for the report T his is a legacy field used to encode custom report unique id |
| `properties` | Vec<String> | This field contains report properties such as ui metadata etc. |
| `limit` | String | Legacy field: not used This field contains the limit for the result retrieved |
| `last_viewed_at` | String | Output only. Last viewed time of this entity as milliseconds since epoch |
| `organization` | String | Output only. Organization name |
| `tags` | Vec<String> | Legacy field: not used. This field contains a list of tags associated with custom report |
| `dimensions` | Vec<String> | This contains the list of dimensions for the report |
| `display_name` | String | This is the display name for the report |
| `time_unit` | String | This field contains the time unit of aggregation for the report |
| `to_time` | String | Legacy field: not used. Contains the end time for the report |
| `topk` | String | Legacy field: not used. This field contains the top k parameter value for restricting the result |
| `metrics` | Vec<String> | Required. This contains the list of metrics |
| `chart_type` | String | This field contains the chart type for the report |
| `environment` | String | Output only. Environment name |
| `from_time` | String | Legacy field: not used. Contains the from time for the report |
| `offset` | String | Legacy field: not used. This field contains the offset for the data |
| `sort_by_cols` | Vec<String> | Legacy field: not used much. Contains the list of sort by columns |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report
report = provider.apigee_api.Report {
    parent = "value"  # Required. The parent organization name under which the Custom Report will be created. Must be of the form: `organizations/{organization_id}/reports`
}

# Access report outputs
report_id = report.id
report_filter = report.filter
report_sort_order = report.sort_order
report_comments = report.comments
report_last_modified_at = report.last_modified_at
report_created_at = report.created_at
report_name = report.name
report_properties = report.properties
report_limit = report.limit
report_last_viewed_at = report.last_viewed_at
report_organization = report.organization
report_tags = report.tags
report_dimensions = report.dimensions
report_display_name = report.display_name
report_time_unit = report.time_unit
report_to_time = report.to_time
report_topk = report.topk
report_metrics = report.metrics
report_chart_type = report.chart_type
report_environment = report.environment
report_from_time = report.from_time
report_offset = report.offset
report_sort_by_cols = report.sort_by_cols
```

---


### Deployment

Sets the IAM policy on a deployment, if the policy already exists it will be replaced. For more information, see [Manage users, roles, and permissions using the API](https://cloud.google.com/apigee/docs/api-platform/system-administration/manage-users-roles). You must have the `apigee.deployments.setIamPolicy` permission to call this API.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `instances` | Vec<String> | Status reported by each runtime instance. **Note**: This field is displayed only when viewing deployment status. |
| `environment` | String | Environment. |
| `api_proxy` | String | API proxy. |
| `proxy_deployment_type` | String | Output only. The type of the deployment (standard or extensible) Deployed proxy revision will be marked as extensible in following 2 cases. 1. The deployed proxy revision uses extensible policies. 2. If a environment supports flowhooks and flow hook is configured. |
| `errors` | Vec<String> | Errors reported for this deployment. Populated only when state == ERROR. **Note**: This field is displayed only when viewing deployment status. |
| `state` | String | Current state of the deployment. **Note**: This field is displayed only when viewing deployment status. |
| `revision` | String | API proxy revision. |
| `deploy_start_time` | String | Time the API proxy was marked `deployed` in the control plane in millisconds since epoch. |
| `route_conflicts` | Vec<String> | Conflicts in the desired state routing configuration. The presence of conflicts does not cause the state to be `ERROR`, but it will mean that some of the deployment's base paths are not routed to its environment. If the conflicts change, the state will transition to `PROGRESSING` until the latest configuration is rolled out to all instances. **Note**: This field is displayed only when viewing deployment status. |
| `service_account` | String | The full resource name of Cloud IAM Service Account that this deployment is using, eg, `projects/-/serviceAccounts/{email}`. |
| `pods` | Vec<String> | Status reported by runtime pods. **Note**: **This field is deprecated**. Runtime versions 1.3 and above report instance level status rather than pod status. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deployment
deployment = provider.apigee_api.Deployment {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access deployment outputs
deployment_id = deployment.id
deployment_instances = deployment.instances
deployment_environment = deployment.environment
deployment_api_proxy = deployment.api_proxy
deployment_proxy_deployment_type = deployment.proxy_deployment_type
deployment_errors = deployment.errors
deployment_state = deployment.state
deployment_revision = deployment.revision
deployment_deploy_start_time = deployment.deploy_start_time
deployment_route_conflicts = deployment.route_conflicts
deployment_service_account = deployment.service_account
deployment_pods = deployment.pods
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple reference resources
reference_0 = provider.apigee_api.Reference {
    parent = "value-0"
}
reference_1 = provider.apigee_api.Reference {
    parent = "value-1"
}
reference_2 = provider.apigee_api.Reference {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    reference = provider.apigee_api.Reference {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Apigee_api Documentation](https://cloud.google.com/apigee_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
