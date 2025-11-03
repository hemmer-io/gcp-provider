# Monitoring_api Service



**Resources**: 22

---

## Overview

The monitoring_api service provides access to 22 resource types:

- [Metadata](#metadata) [R]
- [V1](#v1) [C]
- [Metrics_scope](#metrics_scope) [R]
- [Project](#project) [CD]
- [Dashboard](#dashboard) [CRUD]
- [Operation](#operation) [R]
- [Label](#label) [R]
- [Member](#member) [R]
- [Alert_policie](#alert_policie) [CRUD]
- [Service_level_objective](#service_level_objective) [CRUD]
- [Notification_channel_descriptor](#notification_channel_descriptor) [R]
- [Uptime_check_config](#uptime_check_config) [CRUD]
- [Service](#service) [CRUD]
- [Group](#group) [CRUD]
- [Time_serie](#time_serie) [CR]
- [Uptime_check_ip](#uptime_check_ip) [R]
- [Metric_descriptor](#metric_descriptor) [CRD]
- [Alert](#alert) [R]
- [Collectd_time_serie](#collectd_time_serie) [C]
- [Snooze](#snooze) [CRU]
- [Notification_channel](#notification_channel) [CRUD]
- [Monitored_resource_descriptor](#monitored_resource_descriptor) [R]

---

## Resources


### Metadata

Lists metadata for metrics.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `data` | String | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access metadata outputs
metadata_id = metadata.id
metadata_content_type = metadata.content_type
metadata_data = metadata.data
metadata_extensions = metadata.extensions
```

---


### V1

Evaluate a PromQL query at a single point in time.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query` | String |  | A PromQL query string. Query language documentation: https://prometheus.io/docs/prometheus/latest/querying/basics/. |
| `time` | String |  | The single point in time to evaluate the query for. Either floating point UNIX seconds or RFC3339 formatted timestamp. |
| `timeout` | String |  | An upper bound timeout for the query. Either a Prometheus duration string (https://prometheus.io/docs/prometheus/latest/querying/basics/#time-durations) or floating point seconds. This non-standard encoding must be used for compatibility with the open source API. Clients may still implement timeouts at the connection level while ignoring this field. |
| `location` | String | ✅ | Location of the resource information. Has to be "global" now. |
| `name` | String | ✅ | Required. The project on which to execute the request. Data associcated with the project's workspace stored under the The format is: projects/PROJECT_ID_OR_NUMBER. Open source API but used as a request path prefix to distinguish different virtual Prometheus instances of Google Prometheus Engine. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create v1
v1 = provider.monitoring_api.V1 {
    location = "value"  # Location of the resource information. Has to be "global" now.
    name = "value"  # Required. The project on which to execute the request. Data associcated with the project's workspace stored under the The format is: projects/PROJECT_ID_OR_NUMBER. Open source API but used as a request path prefix to distinguish different virtual Prometheus instances of Google Prometheus Engine.
}

```

---


### Metrics_scope

Returns a specific Metrics Scope, including the list of projects monitored by the specified Metrics Scope.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time when this Metrics Scope record was last updated. |
| `create_time` | String | Output only. The time when this Metrics Scope was created. |
| `monitored_projects` | Vec<String> | Output only. The list of projects monitored by this Metrics Scope. |
| `name` | String | Immutable. The resource name of the Monitoring Metrics Scope. On input, the resource name can be specified with the scoping project ID or number. On output, the resource name is specified with the scoping project number. Example: locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access metrics_scope outputs
metrics_scope_id = metrics_scope.id
metrics_scope_update_time = metrics_scope.update_time
metrics_scope_create_time = metrics_scope.create_time
metrics_scope_monitored_projects = metrics_scope.monitored_projects
metrics_scope_name = metrics_scope.name
```

---


### Project

Adds a MonitoredProject with the given project ID to the specified Metrics Scope.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The resource name of the MonitoredProject. On input, the resource name includes the scoping project ID and monitored project ID. On output, it contains the equivalent project numbers. Example: locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}/projects/{MONITORED_PROJECT_ID_OR_NUMBER} |
| `is_tombstoned` | bool |  | Output only. Set if the project has been tombstoned by the user. |
| `create_time` | String |  | Output only. The time when this MonitoredProject was created. |
| `parent` | String | ✅ | Required. The resource name of the existing Metrics Scope that will monitor this project. Example: locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.monitoring_api.Project {
    parent = "value"  # Required. The resource name of the existing Metrics Scope that will monitor this project. Example: locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}
}

```

---


### Dashboard

Creates a new custom dashboard. For examples on how you can use this API to create dashboards, see Managing dashboards by API (https://cloud.google.com/monitoring/dashboards/api-dashboard). This method requires the monitoring.dashboards.create permission on the specified project. For more information about permissions, see Cloud Identity and Access Management (https://cloud.google.com/iam).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Labels applied to the dashboard |
| `etag` | String |  | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. An etag is returned in the response to GetDashboard, and users are expected to put that etag in the request to UpdateDashboard to ensure that their change will be applied to the same version of the Dashboard configuration. The field should not be passed during dashboard creation. |
| `grid_layout` | String |  | Content is arranged with a basic layout that re-flows a simple list of informational elements like widgets or tiles. |
| `column_layout` | String |  | The content is divided into equally spaced columns and the widgets are arranged vertically. |
| `annotations` | String |  | Configuration for event annotations to display on this dashboard. |
| `mosaic_layout` | String |  | The content is arranged as a grid of tiles, with each content widget occupying one or more grid blocks. |
| `display_name` | String |  | Required. The mutable, human-readable name. |
| `name` | String |  | Identifier. The resource name of the dashboard. |
| `row_layout` | String |  | The content is divided into equally spaced rows and the widgets are arranged horizontally. |
| `dashboard_filters` | Vec<String> |  | Filters to reduce the amount of data charted based on the filter criteria. |
| `parent` | String | ✅ | Required. The project on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] The [PROJECT_ID_OR_NUMBER] must match the dashboard resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Labels applied to the dashboard |
| `etag` | String | etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. An etag is returned in the response to GetDashboard, and users are expected to put that etag in the request to UpdateDashboard to ensure that their change will be applied to the same version of the Dashboard configuration. The field should not be passed during dashboard creation. |
| `grid_layout` | String | Content is arranged with a basic layout that re-flows a simple list of informational elements like widgets or tiles. |
| `column_layout` | String | The content is divided into equally spaced columns and the widgets are arranged vertically. |
| `annotations` | String | Configuration for event annotations to display on this dashboard. |
| `mosaic_layout` | String | The content is arranged as a grid of tiles, with each content widget occupying one or more grid blocks. |
| `display_name` | String | Required. The mutable, human-readable name. |
| `name` | String | Identifier. The resource name of the dashboard. |
| `row_layout` | String | The content is divided into equally spaced rows and the widgets are arranged horizontally. |
| `dashboard_filters` | Vec<String> | Filters to reduce the amount of data charted based on the filter criteria. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dashboard
dashboard = provider.monitoring_api.Dashboard {
    parent = "value"  # Required. The project on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] The [PROJECT_ID_OR_NUMBER] must match the dashboard resource name.
}

# Access dashboard outputs
dashboard_id = dashboard.id
dashboard_labels = dashboard.labels
dashboard_etag = dashboard.etag
dashboard_grid_layout = dashboard.grid_layout
dashboard_column_layout = dashboard.column_layout
dashboard_annotations = dashboard.annotations
dashboard_mosaic_layout = dashboard.mosaic_layout
dashboard_display_name = dashboard.display_name
dashboard_name = dashboard.name
dashboard_row_layout = dashboard.row_layout
dashboard_dashboard_filters = dashboard.dashboard_filters
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
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |


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
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Label

Lists possible values for a given label name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type` | String | The HTTP Content-Type header value specifying the content type of the body. |
| `data` | String | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> | Application specific response metadata. Must be set in the first response for streaming APIs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access label outputs
label_id = label.id
label_content_type = label.content_type
label_data = label.data
label_extensions = label.extensions
```

---


### Member

Lists the monitored resources that are members of a group.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `members` | Vec<String> | A set of monitored resources in the group. |
| `next_page_token` | String | If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method. |
| `total_size` | i64 | The total number of elements matching this request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access member outputs
member_id = member.id
member_members = member.members
member_next_page_token = member.next_page_token
member_total_size = member.total_size
```

---


### Alert_policie

Creates a new alerting policy.Design your application to single-thread API calls that modify the state of alerting policies in a single project. This includes calls to CreateAlertPolicy, DeleteAlertPolicy and UpdateAlertPolicy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creation_record` | String |  | A read-only record of the creation of the alerting policy. If provided in a call to create or update, this field will be ignored. |
| `name` | String |  | Identifier. Required if the policy exists. The resource name for this policy. The format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID] [ALERT_POLICY_ID] is assigned by Cloud Monitoring when the policy is created. When calling the alertPolicies.create method, do not include the name field in the alerting policy passed as part of the request. |
| `combiner` | String |  | How to combine the results of multiple conditions to determine if an incident should be opened. If condition_time_series_query_language is present, this must be COMBINE_UNSPECIFIED. |
| `documentation` | String |  | Documentation that is included with notifications and incidents related to this policy. Best practice is for the documentation to include information to help responders understand, mitigate, escalate, and correct the underlying problems detected by the alerting policy. Notification channels that have limited capacity might not show this documentation. |
| `notification_channels` | Vec<String> |  | Identifies the notification channels to which notifications should be sent when incidents are opened or closed or when new violations occur on an already opened incident. Each element of this array corresponds to the name field in each of the NotificationChannel objects that are returned from the ListNotificationChannels method. The format of the entries in this field is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]  |
| `validity` | String |  | Read-only description of how the alerting policy is invalid. This field is only set when the alerting policy is invalid. An invalid alerting policy will not generate incidents. |
| `severity` | String |  | Optional. The severity of an alerting policy indicates how important incidents generated by that policy are. The severity level will be displayed on the Incident detail page and in notifications. |
| `conditions` | Vec<String> |  | A list of conditions for the policy. The conditions are combined by AND or OR according to the combiner field. If the combined conditions evaluate to true, then an incident is created. A policy can have from one to six conditions. If condition_time_series_query_language is present, it must be the only condition. If condition_monitoring_query_language is present, it must be the only condition. |
| `display_name` | String |  | A short name or phrase used to identify the policy in dashboards, notifications, and incidents. To avoid confusion, don't use the same display name for multiple policies in the same project. The name is limited to 512 Unicode characters.The convention for the display_name of a PrometheusQueryLanguageCondition is "{rule group name}/{alert name}", where the {rule group name} and {alert name} should be taken from the corresponding Prometheus configuration file. This convention is not enforced. In any case the display_name is not a unique key of the AlertPolicy. |
| `mutation_record` | String |  | A read-only record of the most recent change to the alerting policy. If provided in a call to create or update, this field will be ignored. |
| `user_labels` | HashMap<String, String> |  | User-supplied key/value data to be used for organizing and identifying the AlertPolicy objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.Note that Prometheus {alert name} is a valid Prometheus label names (https://prometheus.io/docs/concepts/data_model/#metric-names-and-labels), whereas Prometheus {rule group} is an unrestricted UTF-8 string. This means that they cannot be stored as-is in user labels, because they may contain characters that are not allowed in user-label values. |
| `alert_strategy` | String |  | Control over how this alerting policy's notification channels are notified. |
| `enabled` | bool |  | Whether or not the policy is enabled. On write, the default interpretation if unset is that the policy is enabled. On read, clients should not make any assumption about the state if it has not been populated. The field should always be populated on List and Get operations, unless a field projection has been specified that strips it out. |
| `name` | String | ✅ | Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the alerting policy. The format is: projects/[PROJECT_ID_OR_NUMBER] Note that this field names the parent container in which the alerting policy will be written, not the name of the created policy. |name| must be a host project of a Metrics Scope, otherwise INVALID_ARGUMENT error will return. The alerting policy that is returned will have a name that contains a normalized representation of this name as a prefix but adds a suffix of the form /alertPolicies/[ALERT_POLICY_ID], identifying the policy in the container. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_record` | String | A read-only record of the creation of the alerting policy. If provided in a call to create or update, this field will be ignored. |
| `name` | String | Identifier. Required if the policy exists. The resource name for this policy. The format is: projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID] [ALERT_POLICY_ID] is assigned by Cloud Monitoring when the policy is created. When calling the alertPolicies.create method, do not include the name field in the alerting policy passed as part of the request. |
| `combiner` | String | How to combine the results of multiple conditions to determine if an incident should be opened. If condition_time_series_query_language is present, this must be COMBINE_UNSPECIFIED. |
| `documentation` | String | Documentation that is included with notifications and incidents related to this policy. Best practice is for the documentation to include information to help responders understand, mitigate, escalate, and correct the underlying problems detected by the alerting policy. Notification channels that have limited capacity might not show this documentation. |
| `notification_channels` | Vec<String> | Identifies the notification channels to which notifications should be sent when incidents are opened or closed or when new violations occur on an already opened incident. Each element of this array corresponds to the name field in each of the NotificationChannel objects that are returned from the ListNotificationChannels method. The format of the entries in this field is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]  |
| `validity` | String | Read-only description of how the alerting policy is invalid. This field is only set when the alerting policy is invalid. An invalid alerting policy will not generate incidents. |
| `severity` | String | Optional. The severity of an alerting policy indicates how important incidents generated by that policy are. The severity level will be displayed on the Incident detail page and in notifications. |
| `conditions` | Vec<String> | A list of conditions for the policy. The conditions are combined by AND or OR according to the combiner field. If the combined conditions evaluate to true, then an incident is created. A policy can have from one to six conditions. If condition_time_series_query_language is present, it must be the only condition. If condition_monitoring_query_language is present, it must be the only condition. |
| `display_name` | String | A short name or phrase used to identify the policy in dashboards, notifications, and incidents. To avoid confusion, don't use the same display name for multiple policies in the same project. The name is limited to 512 Unicode characters.The convention for the display_name of a PrometheusQueryLanguageCondition is "{rule group name}/{alert name}", where the {rule group name} and {alert name} should be taken from the corresponding Prometheus configuration file. This convention is not enforced. In any case the display_name is not a unique key of the AlertPolicy. |
| `mutation_record` | String | A read-only record of the most recent change to the alerting policy. If provided in a call to create or update, this field will be ignored. |
| `user_labels` | HashMap<String, String> | User-supplied key/value data to be used for organizing and identifying the AlertPolicy objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter.Note that Prometheus {alert name} is a valid Prometheus label names (https://prometheus.io/docs/concepts/data_model/#metric-names-and-labels), whereas Prometheus {rule group} is an unrestricted UTF-8 string. This means that they cannot be stored as-is in user labels, because they may contain characters that are not allowed in user-label values. |
| `alert_strategy` | String | Control over how this alerting policy's notification channels are notified. |
| `enabled` | bool | Whether or not the policy is enabled. On write, the default interpretation if unset is that the policy is enabled. On read, clients should not make any assumption about the state if it has not been populated. The field should always be populated on List and Get operations, unless a field projection has been specified that strips it out. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create alert_policie
alert_policie = provider.monitoring_api.Alert_policie {
    name = "value"  # Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the alerting policy. The format is: projects/[PROJECT_ID_OR_NUMBER] Note that this field names the parent container in which the alerting policy will be written, not the name of the created policy. |name| must be a host project of a Metrics Scope, otherwise INVALID_ARGUMENT error will return. The alerting policy that is returned will have a name that contains a normalized representation of this name as a prefix but adds a suffix of the form /alertPolicies/[ALERT_POLICY_ID], identifying the policy in the container.
}

# Access alert_policie outputs
alert_policie_id = alert_policie.id
alert_policie_creation_record = alert_policie.creation_record
alert_policie_name = alert_policie.name
alert_policie_combiner = alert_policie.combiner
alert_policie_documentation = alert_policie.documentation
alert_policie_notification_channels = alert_policie.notification_channels
alert_policie_validity = alert_policie.validity
alert_policie_severity = alert_policie.severity
alert_policie_conditions = alert_policie.conditions
alert_policie_display_name = alert_policie.display_name
alert_policie_mutation_record = alert_policie.mutation_record
alert_policie_user_labels = alert_policie.user_labels
alert_policie_alert_strategy = alert_policie.alert_strategy
alert_policie_enabled = alert_policie.enabled
```

---


### Service_level_objective

Create a ServiceLevelObjective for the given Service.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Name used for UI elements listing this SLO. |
| `goal` | f64 |  | The fraction of service that must be good in order for this objective to be met. 0 < goal <= 0.9999. |
| `rolling_period` | String |  | A rolling time period, semantically "in the past ". Must be an integer multiple of 1 day no larger than 30 days. |
| `user_labels` | HashMap<String, String> |  | Labels which have been used to annotate the service-level objective. Label keys must start with a letter. Label keys and values may contain lowercase letters, numbers, underscores, and dashes. Label keys and values have a maximum length of 63 characters, and must be less than 128 bytes in size. Up to 64 label entries may be stored. For labels which do not have a semantic value, the empty string may be supplied for the label value. |
| `service_level_indicator` | String |  | The definition of good service, used to measure and calculate the quality of the Service's performance with respect to a single aspect of service quality. |
| `calendar_period` | String |  | A calendar period, semantically "since the start of the current ". At this time, only DAY, WEEK, FORTNIGHT, and MONTH are supported. |
| `name` | String |  | Identifier. Resource name for this ServiceLevelObjective. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME]  |
| `parent` | String | ✅ | Required. Resource name of the parent Service. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Name used for UI elements listing this SLO. |
| `goal` | f64 | The fraction of service that must be good in order for this objective to be met. 0 < goal <= 0.9999. |
| `rolling_period` | String | A rolling time period, semantically "in the past ". Must be an integer multiple of 1 day no larger than 30 days. |
| `user_labels` | HashMap<String, String> | Labels which have been used to annotate the service-level objective. Label keys must start with a letter. Label keys and values may contain lowercase letters, numbers, underscores, and dashes. Label keys and values have a maximum length of 63 characters, and must be less than 128 bytes in size. Up to 64 label entries may be stored. For labels which do not have a semantic value, the empty string may be supplied for the label value. |
| `service_level_indicator` | String | The definition of good service, used to measure and calculate the quality of the Service's performance with respect to a single aspect of service quality. |
| `calendar_period` | String | A calendar period, semantically "since the start of the current ". At this time, only DAY, WEEK, FORTNIGHT, and MONTH are supported. |
| `name` | String | Identifier. Resource name for this ServiceLevelObjective. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME]  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_level_objective
service_level_objective = provider.monitoring_api.Service_level_objective {
    parent = "value"  # Required. Resource name of the parent Service. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID] 
}

# Access service_level_objective outputs
service_level_objective_id = service_level_objective.id
service_level_objective_display_name = service_level_objective.display_name
service_level_objective_goal = service_level_objective.goal
service_level_objective_rolling_period = service_level_objective.rolling_period
service_level_objective_user_labels = service_level_objective.user_labels
service_level_objective_service_level_indicator = service_level_objective.service_level_indicator
service_level_objective_calendar_period = service_level_objective.calendar_period
service_level_objective_name = service_level_objective.name
```

---


### Notification_channel_descriptor

Gets a single channel descriptor. The descriptor indicates which fields are expected / permitted for a notification channel of the given type.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | A human-readable name for the notification channel type. This form of the name is suitable for a user interface. |
| `labels` | Vec<String> | The set of labels that must be defined to identify a particular channel of the corresponding type. Each label includes a description for how that field should be populated. |
| `supported_tiers` | Vec<String> | The tiers that support this notification channel; the project service tier must be one of the supported_tiers. |
| `type` | String | The type of notification channel, such as "email" and "sms". To view the full list of channels, see Channel descriptors (https://cloud.google.com/monitoring/alerts/using-channels-api#ncd). Notification channel types are globally unique. |
| `launch_stage` | String | The product launch stage for channels of this type. |
| `description` | String | A human-readable description of the notification channel type. The description may include a description of the properties of the channel and pointers to external documentation. |
| `name` | String | The full REST resource name for this descriptor. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannelDescriptors/[TYPE] In the above, [TYPE] is the value of the type field. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access notification_channel_descriptor outputs
notification_channel_descriptor_id = notification_channel_descriptor.id
notification_channel_descriptor_display_name = notification_channel_descriptor.display_name
notification_channel_descriptor_labels = notification_channel_descriptor.labels
notification_channel_descriptor_supported_tiers = notification_channel_descriptor.supported_tiers
notification_channel_descriptor_type = notification_channel_descriptor.type
notification_channel_descriptor_launch_stage = notification_channel_descriptor.launch_stage
notification_channel_descriptor_description = notification_channel_descriptor.description
notification_channel_descriptor_name = notification_channel_descriptor.name
```

---


### Uptime_check_config

Creates a new Uptime check configuration.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. A unique resource name for this Uptime check configuration. The format is: projects/[PROJECT_ID_OR_NUMBER]/uptimeCheckConfigs/[UPTIME_CHECK_ID] [PROJECT_ID_OR_NUMBER] is the Workspace host project associated with the Uptime check.This field should be omitted when creating the Uptime check configuration; on create, the resource name is assigned by the server and included in the response. |
| `http_check` | String |  | Contains information needed to make an HTTP or HTTPS check. |
| `is_internal` | bool |  | If this is true, then checks are made only from the 'internal_checkers'. If it is false, then checks are made only from the 'selected_regions'. It is an error to provide 'selected_regions' when is_internal is true, or to provide 'internal_checkers' when is_internal is false. |
| `tcp_check` | String |  | Contains information needed to make a TCP check. |
| `timeout` | String |  | The maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). Required. |
| `synthetic_monitor` | String |  | Specifies a Synthetic Monitor to invoke. |
| `display_name` | String |  | A human-friendly name for the Uptime check configuration. The display name should be unique within a Cloud Monitoring Workspace in order to make it easier to identify; however, uniqueness is not enforced. Required. |
| `period` | String |  | How often, in seconds, the Uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 60s. |
| `user_labels` | HashMap<String, String> |  | User-supplied key/value data to be used for organizing and identifying the UptimeCheckConfig objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter. |
| `log_check_failures` | bool |  | To specify whether to log the results of failed probes to Cloud Logging. |
| `internal_checkers` | Vec<String> |  | The internal checkers that this check will egress from. If is_internal is true and this list is empty, the check will egress from all the InternalCheckers configured for the project that owns this UptimeCheckConfig. |
| `monitored_resource` | String |  | The monitored resource (https://cloud.google.com/monitoring/api/resources) associated with the configuration. The following monitored resource types are valid for this field: uptime_url, gce_instance, gae_app, aws_ec2_instance, aws_elb_load_balancer k8s_service servicedirectory_service cloud_run_revision |
| `resource_group` | String |  | The group resource associated with the configuration. |
| `content_matchers` | Vec<String> |  | The content that is expected to appear in the data returned by the target server against which the check is run. Currently, only the first entry in the content_matchers list is supported, and additional entries will be ignored. This field is optional and should only be specified if a content match is required as part of the/ Uptime check. |
| `selected_regions` | Vec<String> |  | The list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions must be provided to include a minimum of 3 locations. Not specifying this field will result in Uptime checks running from all available regions. |
| `disabled` | bool |  | Whether the check is disabled or not. |
| `checker_type` | String |  | The type of checkers to use to execute the Uptime check. |
| `parent` | String | ✅ | Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the Uptime check. The format is: projects/[PROJECT_ID_OR_NUMBER]  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. A unique resource name for this Uptime check configuration. The format is: projects/[PROJECT_ID_OR_NUMBER]/uptimeCheckConfigs/[UPTIME_CHECK_ID] [PROJECT_ID_OR_NUMBER] is the Workspace host project associated with the Uptime check.This field should be omitted when creating the Uptime check configuration; on create, the resource name is assigned by the server and included in the response. |
| `http_check` | String | Contains information needed to make an HTTP or HTTPS check. |
| `is_internal` | bool | If this is true, then checks are made only from the 'internal_checkers'. If it is false, then checks are made only from the 'selected_regions'. It is an error to provide 'selected_regions' when is_internal is true, or to provide 'internal_checkers' when is_internal is false. |
| `tcp_check` | String | Contains information needed to make a TCP check. |
| `timeout` | String | The maximum amount of time to wait for the request to complete (must be between 1 and 60 seconds). Required. |
| `synthetic_monitor` | String | Specifies a Synthetic Monitor to invoke. |
| `display_name` | String | A human-friendly name for the Uptime check configuration. The display name should be unique within a Cloud Monitoring Workspace in order to make it easier to identify; however, uniqueness is not enforced. Required. |
| `period` | String | How often, in seconds, the Uptime check is performed. Currently, the only supported values are 60s (1 minute), 300s (5 minutes), 600s (10 minutes), and 900s (15 minutes). Optional, defaults to 60s. |
| `user_labels` | HashMap<String, String> | User-supplied key/value data to be used for organizing and identifying the UptimeCheckConfig objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter. |
| `log_check_failures` | bool | To specify whether to log the results of failed probes to Cloud Logging. |
| `internal_checkers` | Vec<String> | The internal checkers that this check will egress from. If is_internal is true and this list is empty, the check will egress from all the InternalCheckers configured for the project that owns this UptimeCheckConfig. |
| `monitored_resource` | String | The monitored resource (https://cloud.google.com/monitoring/api/resources) associated with the configuration. The following monitored resource types are valid for this field: uptime_url, gce_instance, gae_app, aws_ec2_instance, aws_elb_load_balancer k8s_service servicedirectory_service cloud_run_revision |
| `resource_group` | String | The group resource associated with the configuration. |
| `content_matchers` | Vec<String> | The content that is expected to appear in the data returned by the target server against which the check is run. Currently, only the first entry in the content_matchers list is supported, and additional entries will be ignored. This field is optional and should only be specified if a content match is required as part of the/ Uptime check. |
| `selected_regions` | Vec<String> | The list of regions from which the check will be run. Some regions contain one location, and others contain more than one. If this field is specified, enough regions must be provided to include a minimum of 3 locations. Not specifying this field will result in Uptime checks running from all available regions. |
| `disabled` | bool | Whether the check is disabled or not. |
| `checker_type` | String | The type of checkers to use to execute the Uptime check. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create uptime_check_config
uptime_check_config = provider.monitoring_api.Uptime_check_config {
    parent = "value"  # Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the Uptime check. The format is: projects/[PROJECT_ID_OR_NUMBER] 
}

# Access uptime_check_config outputs
uptime_check_config_id = uptime_check_config.id
uptime_check_config_name = uptime_check_config.name
uptime_check_config_http_check = uptime_check_config.http_check
uptime_check_config_is_internal = uptime_check_config.is_internal
uptime_check_config_tcp_check = uptime_check_config.tcp_check
uptime_check_config_timeout = uptime_check_config.timeout
uptime_check_config_synthetic_monitor = uptime_check_config.synthetic_monitor
uptime_check_config_display_name = uptime_check_config.display_name
uptime_check_config_period = uptime_check_config.period
uptime_check_config_user_labels = uptime_check_config.user_labels
uptime_check_config_log_check_failures = uptime_check_config.log_check_failures
uptime_check_config_internal_checkers = uptime_check_config.internal_checkers
uptime_check_config_monitored_resource = uptime_check_config.monitored_resource
uptime_check_config_resource_group = uptime_check_config.resource_group
uptime_check_config_content_matchers = uptime_check_config.content_matchers
uptime_check_config_selected_regions = uptime_check_config.selected_regions
uptime_check_config_disabled = uptime_check_config.disabled
uptime_check_config_checker_type = uptime_check_config.checker_type
```

---


### Service

Create a Service.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_labels` | HashMap<String, String> |  | Labels which have been used to annotate the service. Label keys must start with a letter. Label keys and values may contain lowercase letters, numbers, underscores, and dashes. Label keys and values have a maximum length of 63 characters, and must be less than 128 bytes in size. Up to 64 label entries may be stored. For labels which do not have a semantic value, the empty string may be supplied for the label value. |
| `gke_namespace` | String |  | Type used for GKE Namespaces. |
| `telemetry` | String |  | Configuration for how to query telemetry on a Service. |
| `display_name` | String |  | Name used for UI elements listing this Service. |
| `custom` | String |  | Custom service type. |
| `gke_service` | String |  | Type used for GKE Services (the Kubernetes concept of a service). |
| `cloud_run` | String |  | Type used for Cloud Run services. |
| `istio_canonical_service` | String |  | Type used for canonical services scoped to an Istio mesh. Metrics for Istio are documented here (https://istio.io/latest/docs/reference/config/metrics/) |
| `gke_workload` | String |  | Type used for GKE Workloads. |
| `cloud_endpoints` | String |  | Type used for Cloud Endpoints services. |
| `name` | String |  | Identifier. Resource name for this Service. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]  |
| `basic_service` | String |  | Message that contains the service type and service labels of this service if it is a basic service. Documentation and examples here (https://cloud.google.com/stackdriver/docs/solutions/slo-monitoring/api/api-structures#basic-svc-w-basic-sli). |
| `app_engine` | String |  | Type used for App Engine services. |
| `cluster_istio` | String |  | Type used for Istio services that live in a Kubernetes cluster. |
| `mesh_istio` | String |  | Type used for Istio services scoped to an Istio mesh. |
| `parent` | String | ✅ | Required. Resource name (https://cloud.google.com/monitoring/api/v3#project_name) of the parent Metrics Scope. The format is: projects/[PROJECT_ID_OR_NUMBER]  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_labels` | HashMap<String, String> | Labels which have been used to annotate the service. Label keys must start with a letter. Label keys and values may contain lowercase letters, numbers, underscores, and dashes. Label keys and values have a maximum length of 63 characters, and must be less than 128 bytes in size. Up to 64 label entries may be stored. For labels which do not have a semantic value, the empty string may be supplied for the label value. |
| `gke_namespace` | String | Type used for GKE Namespaces. |
| `telemetry` | String | Configuration for how to query telemetry on a Service. |
| `display_name` | String | Name used for UI elements listing this Service. |
| `custom` | String | Custom service type. |
| `gke_service` | String | Type used for GKE Services (the Kubernetes concept of a service). |
| `cloud_run` | String | Type used for Cloud Run services. |
| `istio_canonical_service` | String | Type used for canonical services scoped to an Istio mesh. Metrics for Istio are documented here (https://istio.io/latest/docs/reference/config/metrics/) |
| `gke_workload` | String | Type used for GKE Workloads. |
| `cloud_endpoints` | String | Type used for Cloud Endpoints services. |
| `name` | String | Identifier. Resource name for this Service. The format is: projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]  |
| `basic_service` | String | Message that contains the service type and service labels of this service if it is a basic service. Documentation and examples here (https://cloud.google.com/stackdriver/docs/solutions/slo-monitoring/api/api-structures#basic-svc-w-basic-sli). |
| `app_engine` | String | Type used for App Engine services. |
| `cluster_istio` | String | Type used for Istio services that live in a Kubernetes cluster. |
| `mesh_istio` | String | Type used for Istio services scoped to an Istio mesh. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.monitoring_api.Service {
    parent = "value"  # Required. Resource name (https://cloud.google.com/monitoring/api/v3#project_name) of the parent Metrics Scope. The format is: projects/[PROJECT_ID_OR_NUMBER] 
}

# Access service outputs
service_id = service.id
service_user_labels = service.user_labels
service_gke_namespace = service.gke_namespace
service_telemetry = service.telemetry
service_display_name = service.display_name
service_custom = service.custom
service_gke_service = service.gke_service
service_cloud_run = service.cloud_run
service_istio_canonical_service = service.istio_canonical_service
service_gke_workload = service.gke_workload
service_cloud_endpoints = service.cloud_endpoints
service_name = service.name
service_basic_service = service.basic_service
service_app_engine = service.app_engine
service_cluster_istio = service.cluster_istio
service_mesh_istio = service.mesh_istio
```

---


### Group

Creates a new group.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The name of this group. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] When creating a group, this field is ignored and a new name is created consisting of the project specified in the call to CreateGroup and a unique [GROUP_ID] that is generated automatically. |
| `filter` | String |  | The filter used to determine which monitored resources belong to this group. |
| `display_name` | String |  | A user-assigned name for this group, used only for display purposes. |
| `is_cluster` | bool |  | If true, the members of this group are considered to be a cluster. The system can perform additional analysis on groups that are clusters. |
| `parent_name` | String |  | The name of the group's parent, if it has one. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] For groups with no parent, parent_name is the empty string, "". |
| `name` | String | ✅ | Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the group. The format is: projects/[PROJECT_ID_OR_NUMBER]  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The name of this group. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] When creating a group, this field is ignored and a new name is created consisting of the project specified in the call to CreateGroup and a unique [GROUP_ID] that is generated automatically. |
| `filter` | String | The filter used to determine which monitored resources belong to this group. |
| `display_name` | String | A user-assigned name for this group, used only for display purposes. |
| `is_cluster` | bool | If true, the members of this group are considered to be a cluster. The system can perform additional analysis on groups that are clusters. |
| `parent_name` | String | The name of the group's parent, if it has one. The format is: projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID] For groups with no parent, parent_name is the empty string, "". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.monitoring_api.Group {
    name = "value"  # Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the group. The format is: projects/[PROJECT_ID_OR_NUMBER] 
}

# Access group outputs
group_id = group.id
group_name = group.name
group_filter = group.filter
group_display_name = group.display_name
group_is_cluster = group.is_cluster
group_parent_name = group.parent_name
```

---


### Time_serie

Creates or adds data to one or more time series. The response is empty if all time series in the request were written. If any time series could not be written, a corresponding failure message is included in the error response. This method does not support resource locations constraint of an organization policy (https://cloud.google.com/resource-manager/docs/organization-policy/defining-locations#setting_the_organization_policy).

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `time_series` | Vec<String> |  | Required. The new data to be added to a list of time series. Adds at most one data point to each of several time series. The new data point must be more recent than any other point in its time series. Each TimeSeries value must fully specify a unique time series by supplying all label values for the metric and the monitored resource.The maximum number of TimeSeries objects per Create request is 200. |
| `name` | String | ✅ | Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER]  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If there are more results than have been returned, then this field is set to a non-empty value. To see the additional results, use that value as page_token in the next call to this method. |
| `execution_errors` | Vec<String> | Query execution errors that may have caused the time series data returned to be incomplete. |
| `unit` | String | The unit in which all time_series point values are reported. unit follows the UCUM format for units as seen in https://unitsofmeasure.org/ucum.html. If different time_series have different units (for example, because they come from different metric types, or a unit is absent), then unit will be "{not_a_unit}". |
| `time_series` | Vec<String> | One or more time series that match the filter included in the request. |
| `unreachable` | Vec<String> | Cloud regions that were unreachable which may have caused incomplete data to be returned. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create time_serie
time_serie = provider.monitoring_api.Time_serie {
    name = "value"  # Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] 
}

# Access time_serie outputs
time_serie_id = time_serie.id
time_serie_next_page_token = time_serie.next_page_token
time_serie_execution_errors = time_serie.execution_errors
time_serie_unit = time_serie.unit
time_serie_time_series = time_serie.time_series
time_serie_unreachable = time_serie.unreachable
```

---


### Uptime_check_ip

Returns the list of IP addresses that checkers run from.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | This field represents the pagination token to retrieve the next page of results. If the value is empty, it means no further results for the request. To retrieve the next page of results, the value of the next_page_token is passed to the subsequent List method call (in the request message's page_token field). NOTE: this field is not yet implemented |
| `uptime_check_ips` | Vec<String> | The returned list of IP addresses (including region and location) that the checkers run from. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access uptime_check_ip outputs
uptime_check_ip_id = uptime_check_ip.id
uptime_check_ip_next_page_token = uptime_check_ip.next_page_token
uptime_check_ip_uptime_check_ips = uptime_check_ip.uptime_check_ips
```

---


### Metric_descriptor

Creates a new metric descriptor. The creation is executed asynchronously. User-created metric descriptors define custom metrics (https://cloud.google.com/monitoring/custom-metrics). The metric descriptor is updated if it already exists, except that metric labels are never removed.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `monitored_resource_types` | Vec<String> |  | Read-only. If present, then a time series, which is identified partially by a metric type and a MonitoredResourceDescriptor, that is associated with this metric type can only be associated with one of the monitored resource types listed here. |
| `unit` | String |  | The units in which the metric value is reported. It is only applicable if the value_type is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of the stored metric values.Different systems might scale the values to be more easily displayed (so a value of 0.02kBy might be displayed as 20By, and a value of 3523kBy might be displayed as 3.5MBy). However, if the unit is kBy, then the value of the metric is always in thousands of bytes, no matter how it might be displayed.If you want a custom metric to record the exact number of CPU-seconds used by a job, you can create an INT64 CUMULATIVE metric whose unit is s{CPU} (or equivalently 1s{CPU} or just s). If the job uses 12,005 CPU-seconds, then the value is written as 12005.Alternatively, if you want a custom metric to record data in a more granular way, you can create a DOUBLE CUMULATIVE metric whose unit is ks{CPU}, and then write the value 12.005 (which is 12005/1000), or use Kis{CPU} and write 11.723 (which is 12005/1024).The supported units are a subset of The Unified Code for Units of Measure (https://unitsofmeasure.org/ucum.html) standard:Basic units (UNIT) bit bit By byte s second min minute h hour d day 1 dimensionlessPrefixes (PREFIX) k kilo (10^3) M mega (10^6) G giga (10^9) T tera (10^12) P peta (10^15) E exa (10^18) Z zetta (10^21) Y yotta (10^24) m milli (10^-3) u micro (10^-6) n nano (10^-9) p pico (10^-12) f femto (10^-15) a atto (10^-18) z zepto (10^-21) y yocto (10^-24) Ki kibi (2^10) Mi mebi (2^20) Gi gibi (2^30) Ti tebi (2^40) Pi pebi (2^50)GrammarThe grammar also includes these connectors: / division or ratio (as an infix operator). For examples, kBy/{email} or MiBy/10ms (although you should almost never have /s in a metric unit; rates should always be computed at query time from the underlying cumulative or delta value). . multiplication or composition (as an infix operator). For examples, GBy.d or k{watt}.h.The grammar for a unit is as follows: Expression = Component { "." Component } { "/" Component } ; Component = ( [ PREFIX ] UNIT | "%" ) [ Annotation ] | Annotation | "1" ; Annotation = "{" NAME "}" ; Notes: Annotation is just a comment if it follows a UNIT. If the annotation is used alone, then the unit is equivalent to 1. For examples, {request}/s == 1/s, By{transmitted}/s == By/s. NAME is a sequence of non-blank printable ASCII characters not containing { or }. 1 represents a unitary dimensionless unit (https://en.wikipedia.org/wiki/Dimensionless_quantity) of 1, such as in 1/s. It is typically used when none of the basic units are appropriate. For example, "new users per day" can be represented as 1/d or {new-users}/d (and a metric value 5 would mean "5 new users). Alternatively, "thousands of page views per day" would be represented as 1000/d or k1/d or k{page_views}/d (and a metric value of 5.3 would mean "5300 page views per day"). % represents dimensionless value of 1/100, and annotates values giving a percentage (so the metric values are typically in the range of 0..100, and a metric value 3 means "3 percent"). 10^2.% indicates a metric contains a ratio, typically in the range 0..1, that will be multiplied by 100 and displayed as a percentage (so a metric value 0.03 means "3 percent"). |
| `name` | String |  | The resource name of the metric descriptor. |
| `metric_kind` | String |  | Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metric_kind and value_type might not be supported. |
| `display_name` | String |  | A concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example "Request count". This field is optional but it is recommended to be set for any metrics associated with user-visible concepts, such as Quota. |
| `value_type` | String |  | Whether the measurement is an integer, a floating-point number, etc. Some combinations of metric_kind and value_type might not be supported. |
| `type` | String |  | The metric type, including its DNS name prefix. The type is not URL-encoded. All user-defined metric types have the DNS name custom.googleapis.com or external.googleapis.com. Metric types should use a natural hierarchical grouping. For example: "custom.googleapis.com/invoice/paid/amount" "external.googleapis.com/prometheus/up" "appengine.googleapis.com/http/server/response_latencies"  |
| `metadata` | String |  | Optional. Metadata which can be used to guide usage of the metric. |
| `description` | String |  | A detailed description of the metric, which can be used in documentation. |
| `labels` | Vec<String> |  | The set of labels that can be used to describe a specific instance of this metric type. For example, the appengine.googleapis.com/http/server/response_latencies metric type has a label for the HTTP response code, response_code, so you can look at latencies for successful responses or just for responses that failed. |
| `launch_stage` | String |  | Optional. The launch stage of the metric definition. |
| `name` | String | ✅ | Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: 4 projects/PROJECT_ID_OR_NUMBER |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `monitored_resource_types` | Vec<String> | Read-only. If present, then a time series, which is identified partially by a metric type and a MonitoredResourceDescriptor, that is associated with this metric type can only be associated with one of the monitored resource types listed here. |
| `unit` | String | The units in which the metric value is reported. It is only applicable if the value_type is INT64, DOUBLE, or DISTRIBUTION. The unit defines the representation of the stored metric values.Different systems might scale the values to be more easily displayed (so a value of 0.02kBy might be displayed as 20By, and a value of 3523kBy might be displayed as 3.5MBy). However, if the unit is kBy, then the value of the metric is always in thousands of bytes, no matter how it might be displayed.If you want a custom metric to record the exact number of CPU-seconds used by a job, you can create an INT64 CUMULATIVE metric whose unit is s{CPU} (or equivalently 1s{CPU} or just s). If the job uses 12,005 CPU-seconds, then the value is written as 12005.Alternatively, if you want a custom metric to record data in a more granular way, you can create a DOUBLE CUMULATIVE metric whose unit is ks{CPU}, and then write the value 12.005 (which is 12005/1000), or use Kis{CPU} and write 11.723 (which is 12005/1024).The supported units are a subset of The Unified Code for Units of Measure (https://unitsofmeasure.org/ucum.html) standard:Basic units (UNIT) bit bit By byte s second min minute h hour d day 1 dimensionlessPrefixes (PREFIX) k kilo (10^3) M mega (10^6) G giga (10^9) T tera (10^12) P peta (10^15) E exa (10^18) Z zetta (10^21) Y yotta (10^24) m milli (10^-3) u micro (10^-6) n nano (10^-9) p pico (10^-12) f femto (10^-15) a atto (10^-18) z zepto (10^-21) y yocto (10^-24) Ki kibi (2^10) Mi mebi (2^20) Gi gibi (2^30) Ti tebi (2^40) Pi pebi (2^50)GrammarThe grammar also includes these connectors: / division or ratio (as an infix operator). For examples, kBy/{email} or MiBy/10ms (although you should almost never have /s in a metric unit; rates should always be computed at query time from the underlying cumulative or delta value). . multiplication or composition (as an infix operator). For examples, GBy.d or k{watt}.h.The grammar for a unit is as follows: Expression = Component { "." Component } { "/" Component } ; Component = ( [ PREFIX ] UNIT | "%" ) [ Annotation ] | Annotation | "1" ; Annotation = "{" NAME "}" ; Notes: Annotation is just a comment if it follows a UNIT. If the annotation is used alone, then the unit is equivalent to 1. For examples, {request}/s == 1/s, By{transmitted}/s == By/s. NAME is a sequence of non-blank printable ASCII characters not containing { or }. 1 represents a unitary dimensionless unit (https://en.wikipedia.org/wiki/Dimensionless_quantity) of 1, such as in 1/s. It is typically used when none of the basic units are appropriate. For example, "new users per day" can be represented as 1/d or {new-users}/d (and a metric value 5 would mean "5 new users). Alternatively, "thousands of page views per day" would be represented as 1000/d or k1/d or k{page_views}/d (and a metric value of 5.3 would mean "5300 page views per day"). % represents dimensionless value of 1/100, and annotates values giving a percentage (so the metric values are typically in the range of 0..100, and a metric value 3 means "3 percent"). 10^2.% indicates a metric contains a ratio, typically in the range 0..1, that will be multiplied by 100 and displayed as a percentage (so a metric value 0.03 means "3 percent"). |
| `name` | String | The resource name of the metric descriptor. |
| `metric_kind` | String | Whether the metric records instantaneous values, changes to a value, etc. Some combinations of metric_kind and value_type might not be supported. |
| `display_name` | String | A concise name for the metric, which can be displayed in user interfaces. Use sentence case without an ending period, for example "Request count". This field is optional but it is recommended to be set for any metrics associated with user-visible concepts, such as Quota. |
| `value_type` | String | Whether the measurement is an integer, a floating-point number, etc. Some combinations of metric_kind and value_type might not be supported. |
| `type` | String | The metric type, including its DNS name prefix. The type is not URL-encoded. All user-defined metric types have the DNS name custom.googleapis.com or external.googleapis.com. Metric types should use a natural hierarchical grouping. For example: "custom.googleapis.com/invoice/paid/amount" "external.googleapis.com/prometheus/up" "appengine.googleapis.com/http/server/response_latencies"  |
| `metadata` | String | Optional. Metadata which can be used to guide usage of the metric. |
| `description` | String | A detailed description of the metric, which can be used in documentation. |
| `labels` | Vec<String> | The set of labels that can be used to describe a specific instance of this metric type. For example, the appengine.googleapis.com/http/server/response_latencies metric type has a label for the HTTP response code, response_code, so you can look at latencies for successful responses or just for responses that failed. |
| `launch_stage` | String | Optional. The launch stage of the metric definition. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create metric_descriptor
metric_descriptor = provider.monitoring_api.Metric_descriptor {
    name = "value"  # Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: 4 projects/PROJECT_ID_OR_NUMBER
}

# Access metric_descriptor outputs
metric_descriptor_id = metric_descriptor.id
metric_descriptor_monitored_resource_types = metric_descriptor.monitored_resource_types
metric_descriptor_unit = metric_descriptor.unit
metric_descriptor_name = metric_descriptor.name
metric_descriptor_metric_kind = metric_descriptor.metric_kind
metric_descriptor_display_name = metric_descriptor.display_name
metric_descriptor_value_type = metric_descriptor.value_type
metric_descriptor_type = metric_descriptor.type
metric_descriptor_metadata = metric_descriptor.metadata
metric_descriptor_description = metric_descriptor.description
metric_descriptor_labels = metric_descriptor.labels
metric_descriptor_launch_stage = metric_descriptor.launch_stage
```

---


### Alert

Gets a single alert.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metric` | String | The metric type and any metric labels preserved from the incident's generating condition. |
| `name` | String | Identifier. The name of the alert.The format is: projects/[PROJECT_ID_OR_NUMBER]/alerts/[ALERT_ID] The [ALERT_ID] is a system-assigned unique identifier for the alert. |
| `open_time` | String | The time when the alert was opened. |
| `metadata` | String | The metadata of the monitored resource. |
| `policy` | String | The snapshot of the alert policy that generated this alert. |
| `close_time` | String | The time when the alert was closed. |
| `resource` | String | The monitored resource type and any monitored resource labels preserved from the incident's generating condition. |
| `state` | String | Output only. The current state of the alert. |
| `log` | String | The log information associated with the alert. This field is only populated for log-based alerts. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access alert outputs
alert_id = alert.id
alert_metric = alert.metric
alert_name = alert.name
alert_open_time = alert.open_time
alert_metadata = alert.metadata
alert_policy = alert.policy
alert_close_time = alert.close_time
alert_resource = alert.resource
alert_state = alert.state
alert_log = alert.log
```

---


### Collectd_time_serie

Cloud Monitoring Agent only: Creates a new time series.This method is only for use by the Cloud Monitoring Agent. Use projects.timeSeries.create instead.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource` | String |  | The monitored resource associated with the time series. |
| `collectd_payloads` | Vec<String> |  | The collectd payloads representing the time series data. You must not include more than a single point for each time series, so no two payloads can have the same values for all of the fields plugin, plugin_instance, type, and type_instance. |
| `collectd_version` | String |  | The version of collectd that collected the data. Example: "5.3.0-192.el6". |
| `name` | String | ✅ | The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the time series. The format is: projects/[PROJECT_ID_OR_NUMBER]  |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create collectd_time_serie
collectd_time_serie = provider.monitoring_api.Collectd_time_serie {
    name = "value"  # The project (https://cloud.google.com/monitoring/api/v3#project_name) in which to create the time series. The format is: projects/[PROJECT_ID_OR_NUMBER] 
}

```

---


### Snooze

Creates a Snooze that will prevent alerts, which match the provided criteria, from being opened. The Snooze applies for a specific time interval.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `criteria` | String |  | Required. This defines the criteria for applying the Snooze. See Criteria for more information. |
| `name` | String |  | Required. Identifier. The name of the Snooze. The format is: projects/[PROJECT_ID_OR_NUMBER]/snoozes/[SNOOZE_ID] The ID of the Snooze will be generated by the system. |
| `interval` | String |  | Required. The Snooze will be active from interval.start_time through interval.end_time. interval.start_time cannot be in the past. There is a 15 second clock skew to account for the time it takes for a request to reach the API from the UI. |
| `display_name` | String |  | Required. A display name for the Snooze. This can be, at most, 512 unicode characters. |
| `parent` | String | ✅ | Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which a Snooze should be created. The format is: projects/[PROJECT_ID_OR_NUMBER]  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `criteria` | String | Required. This defines the criteria for applying the Snooze. See Criteria for more information. |
| `name` | String | Required. Identifier. The name of the Snooze. The format is: projects/[PROJECT_ID_OR_NUMBER]/snoozes/[SNOOZE_ID] The ID of the Snooze will be generated by the system. |
| `interval` | String | Required. The Snooze will be active from interval.start_time through interval.end_time. interval.start_time cannot be in the past. There is a 15 second clock skew to account for the time it takes for a request to reach the API from the UI. |
| `display_name` | String | Required. A display name for the Snooze. This can be, at most, 512 unicode characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create snooze
snooze = provider.monitoring_api.Snooze {
    parent = "value"  # Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) in which a Snooze should be created. The format is: projects/[PROJECT_ID_OR_NUMBER] 
}

# Access snooze outputs
snooze_id = snooze.id
snooze_criteria = snooze.criteria
snooze_name = snooze.name
snooze_interval = snooze.interval
snooze_display_name = snooze.display_name
```

---


### Notification_channel

Creates a new notification channel, representing a single notification endpoint such as an email address, SMS number, or PagerDuty service.Design your application to single-thread API calls that modify the state of notification channels in a single project. This includes calls to CreateNotificationChannel, DeleteNotificationChannel and UpdateNotificationChannel.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | An optional human-readable description of this notification channel. This description may provide additional details, beyond the display name, for the channel. This may not exceed 1024 Unicode characters. |
| `enabled` | bool |  | Whether notifications are forwarded to the described channel. This makes it possible to disable delivery of notifications to a particular channel without removing the channel from all alerting policies that reference the channel. This is a more convenient approach when the change is temporary and you want to receive notifications from the same set of alerting policies on the channel at some point in the future. |
| `type` | String |  | The type of the notification channel. This field matches the value of the NotificationChannelDescriptor.type field. |
| `user_labels` | HashMap<String, String> |  | User-supplied key/value data that does not need to conform to the corresponding NotificationChannelDescriptor's schema, unlike the labels field. This field is intended to be used for organizing and identifying the NotificationChannel objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter. |
| `display_name` | String |  | An optional human-readable name for this notification channel. It is recommended that you specify a non-empty and unique name in order to make it easier to identify the channels in your project, though this is not enforced. The display name is limited to 512 Unicode characters. |
| `creation_record` | String |  | Record of the creation of this channel. |
| `mutation_records` | Vec<String> |  | Records of the modification of this channel. |
| `name` | String |  | Identifier. The full REST resource name for this channel. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID] The [CHANNEL_ID] is automatically assigned by the server on creation. |
| `verification_status` | String |  | Indicates whether this channel has been verified or not. On a ListNotificationChannels or GetNotificationChannel operation, this field is expected to be populated.If the value is UNVERIFIED, then it indicates that the channel is non-functioning (it both requires verification and lacks verification); otherwise, it is assumed that the channel works.If the channel is neither VERIFIED nor UNVERIFIED, it implies that the channel is of a type that does not require verification or that this specific channel has been exempted from verification because it was created prior to verification being required for channels of this type.This field cannot be modified using a standard UpdateNotificationChannel operation. To change the value of this field, you must call VerifyNotificationChannel. |
| `labels` | HashMap<String, String> |  | Configuration fields that define the channel and its behavior. The permissible and required labels are specified in the NotificationChannelDescriptor.labels of the NotificationChannelDescriptor corresponding to the type field. |
| `name` | String | ✅ | Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] This names the container into which the channel will be written, this does not name the newly created channel. The resulting channel's name will have a normalized version of this field as a prefix, but will add /notificationChannels/[CHANNEL_ID] to identify the channel. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | An optional human-readable description of this notification channel. This description may provide additional details, beyond the display name, for the channel. This may not exceed 1024 Unicode characters. |
| `enabled` | bool | Whether notifications are forwarded to the described channel. This makes it possible to disable delivery of notifications to a particular channel without removing the channel from all alerting policies that reference the channel. This is a more convenient approach when the change is temporary and you want to receive notifications from the same set of alerting policies on the channel at some point in the future. |
| `type` | String | The type of the notification channel. This field matches the value of the NotificationChannelDescriptor.type field. |
| `user_labels` | HashMap<String, String> | User-supplied key/value data that does not need to conform to the corresponding NotificationChannelDescriptor's schema, unlike the labels field. This field is intended to be used for organizing and identifying the NotificationChannel objects.The field can contain up to 64 entries. Each key and value is limited to 63 Unicode characters or 128 bytes, whichever is smaller. Labels and values can contain only lowercase letters, numerals, underscores, and dashes. Keys must begin with a letter. |
| `display_name` | String | An optional human-readable name for this notification channel. It is recommended that you specify a non-empty and unique name in order to make it easier to identify the channels in your project, though this is not enforced. The display name is limited to 512 Unicode characters. |
| `creation_record` | String | Record of the creation of this channel. |
| `mutation_records` | Vec<String> | Records of the modification of this channel. |
| `name` | String | Identifier. The full REST resource name for this channel. The format is: projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID] The [CHANNEL_ID] is automatically assigned by the server on creation. |
| `verification_status` | String | Indicates whether this channel has been verified or not. On a ListNotificationChannels or GetNotificationChannel operation, this field is expected to be populated.If the value is UNVERIFIED, then it indicates that the channel is non-functioning (it both requires verification and lacks verification); otherwise, it is assumed that the channel works.If the channel is neither VERIFIED nor UNVERIFIED, it implies that the channel is of a type that does not require verification or that this specific channel has been exempted from verification because it was created prior to verification being required for channels of this type.This field cannot be modified using a standard UpdateNotificationChannel operation. To change the value of this field, you must call VerifyNotificationChannel. |
| `labels` | HashMap<String, String> | Configuration fields that define the channel and its behavior. The permissible and required labels are specified in the NotificationChannelDescriptor.labels of the NotificationChannelDescriptor corresponding to the type field. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notification_channel
notification_channel = provider.monitoring_api.Notification_channel {
    name = "value"  # Required. The project (https://cloud.google.com/monitoring/api/v3#project_name) on which to execute the request. The format is: projects/[PROJECT_ID_OR_NUMBER] This names the container into which the channel will be written, this does not name the newly created channel. The resulting channel's name will have a normalized version of this field as a prefix, but will add /notificationChannels/[CHANNEL_ID] to identify the channel.
}

# Access notification_channel outputs
notification_channel_id = notification_channel.id
notification_channel_description = notification_channel.description
notification_channel_enabled = notification_channel.enabled
notification_channel_type = notification_channel.type
notification_channel_user_labels = notification_channel.user_labels
notification_channel_display_name = notification_channel.display_name
notification_channel_creation_record = notification_channel.creation_record
notification_channel_mutation_records = notification_channel.mutation_records
notification_channel_name = notification_channel.name
notification_channel_verification_status = notification_channel.verification_status
notification_channel_labels = notification_channel.labels
```

---


### Monitored_resource_descriptor

Gets a single monitored resource descriptor.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Required. The monitored resource type. For example, the type "cloudsql_database" represents databases in Google Cloud SQL. For a list of types, see Monitored resource types (https://cloud.google.com/monitoring/api/resources) and Logging resource types (https://cloud.google.com/logging/docs/api/v2/resource-list). |
| `name` | String | Optional. The resource name of the monitored resource descriptor: "projects/{project_id}/monitoredResourceDescriptors/{type}" where {type} is the value of the type field in this object and {project_id} is a project ID that provides API-specific context for accessing the type. APIs that do not use project information can use the resource name format "monitoredResourceDescriptors/{type}". |
| `description` | String | Optional. A detailed description of the monitored resource type that might be used in documentation. |
| `display_name` | String | Optional. A concise name for the monitored resource type that might be displayed in user interfaces. It should be a Title Cased Noun Phrase, without any article or other determiners. For example, "Google Cloud SQL Database". |
| `labels` | Vec<String> | Required. A set of labels used to describe instances of this monitored resource type. For example, an individual Google Cloud SQL database is identified by values for the labels "database_id" and "zone". |
| `launch_stage` | String | Optional. The launch stage of the monitored resource definition. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access monitored_resource_descriptor outputs
monitored_resource_descriptor_id = monitored_resource_descriptor.id
monitored_resource_descriptor_type = monitored_resource_descriptor.type
monitored_resource_descriptor_name = monitored_resource_descriptor.name
monitored_resource_descriptor_description = monitored_resource_descriptor.description
monitored_resource_descriptor_display_name = monitored_resource_descriptor.display_name
monitored_resource_descriptor_labels = monitored_resource_descriptor.labels
monitored_resource_descriptor_launch_stage = monitored_resource_descriptor.launch_stage
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple metadata resources
metadata_0 = provider.monitoring_api.Metadata {
}
metadata_1 = provider.monitoring_api.Metadata {
}
metadata_2 = provider.monitoring_api.Metadata {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    metadata = provider.monitoring_api.Metadata {
    }
```

---

## Related Documentation

- [GCP Monitoring_api Documentation](https://cloud.google.com/monitoring_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
