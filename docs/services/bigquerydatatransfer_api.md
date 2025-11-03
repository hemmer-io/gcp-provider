# Bigquerydatatransfer_api Service



**Resources**: 6

---

## Overview

The bigquerydatatransfer_api service provides access to 6 resource types:

- [Location](#location) [CR]
- [Transfer_config](#transfer_config) [CRUD]
- [Data_source](#data_source) [CR]
- [Project](#project) [C]
- [Run](#run) [RD]
- [Transfer_log](#transfer_log) [R]

---

## Resources


### Location

Unenroll data sources in a user project. This allows users to remove transfer configurations for these data sources. They will no longer appear in the ListDataSources RPC and will also no longer appear in the [BigQuery UI](https://console.cloud.google.com/bigquery). Data transfers configurations of unenrolled data sources will not be scheduled.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source_ids` | Vec<String> |  | Data sources that are unenrolled. It is required to provide at least one data source id. |
| `name` | String | ✅ | Required. The name of the project resource in the form: `projects/{project_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.bigquerydatatransfer_api.Location {
    name = "value"  # Required. The name of the project resource in the form: `projects/{project_id}`
}

# Access location outputs
location_id = location.id
location_labels = location.labels
location_display_name = location.display_name
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
```

---


### Transfer_config

Creates a new data transfer configuration.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_dataset_id` | String |  | The BigQuery target dataset id. |
| `managed_table_type` | String |  | The classification of the destination table. |
| `schedule` | String |  | Data transfer schedule. If the data source does not support a custom schedule, this should be empty. If it is empty, the default value for the data source will be used. The specified times are in UTC. Examples of valid format: `1st,3rd monday of month 15:30`, `every wed,fri of jan,jun 13:15`, and `first sunday of quarter 00:00`. See more explanation about the format here: https://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format NOTE: The minimum interval time between recurring transfers depends on the data source; refer to the documentation for your data source. |
| `user_id` | String |  | Deprecated. Unique ID of the user on whose behalf transfer is done. |
| `schedule_options` | String |  | Options customizing the data transfer schedule. |
| `update_time` | String |  | Output only. Data transfer modification time. Ignored by server on input. |
| `encryption_configuration` | String |  | The encryption configuration part. Currently, it is only used for the optional KMS key name. The BigQuery service account of your project must be granted permissions to use the key. Read methods will return the key name applied in effect. Write methods will apply the key if it is present, or otherwise try to apply project default keys if it is absent. |
| `params` | HashMap<String, String> |  | Parameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer' section for each data source. For example the parameters for Cloud Storage transfers are listed here: https://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq |
| `dataset_region` | String |  | Output only. Region in which BigQuery dataset is located. |
| `data_source_id` | String |  | Data source ID. This cannot be changed once data transfer is created. The full list of available data source IDs can be returned through an API call: https://cloud.google.com/bigquery-transfer/docs/reference/datatransfer/rest/v1/projects.locations.dataSources/list |
| `error` | String |  | Output only. Error code with detailed information about reason of the latest config failure. |
| `email_preferences` | String |  | Email notifications will be sent according to these preferences to the email address of the user who owns this transfer config. |
| `disabled` | bool |  | Is this config disabled. When set to true, no runs will be scheduled for this transfer config. |
| `display_name` | String |  | User specified display name for the data transfer. |
| `notification_pubsub_topic` | String |  | Pub/Sub topic where notifications will be sent after transfer runs associated with this transfer config finish. The format for specifying a pubsub topic is: `projects/{project_id}/topics/{topic_id}` |
| `schedule_options_v2` | String |  | Options customizing different types of data transfer schedule. This field replaces "schedule" and "schedule_options" fields. ScheduleOptionsV2 cannot be used together with ScheduleOptions/Schedule. |
| `state` | String |  | Output only. State of the most recently updated transfer run. |
| `owner_info` | String |  | Output only. Information about the user whose credentials are used to transfer data. Populated only for `transferConfigs.get` requests. In case the user information is not available, this field will not be populated. |
| `data_refresh_window_days` | i64 |  | The number of days to look back to automatically refresh the data. For example, if `data_refresh_window_days = 10`, then every day BigQuery reingests data for [today-10, today-1], rather than ingesting data for just [today-1]. Only valid if the data source supports the feature. Set the value to 0 to use the default value. |
| `name` | String |  | Identifier. The resource name of the transfer config. Transfer config names have the form either `projects/{project_id}/locations/{region}/transferConfigs/{config_id}` or `projects/{project_id}/transferConfigs/{config_id}`, where `config_id` is usually a UUID, even though it is not guaranteed or required. The name is ignored when creating a transfer config. |
| `next_run_time` | String |  | Output only. Next time when data transfer will run. |
| `parent` | String | ✅ | Required. The BigQuery project id where the transfer configuration should be created. Must be in the format projects/{project_id}/locations/{location_id} or projects/{project_id}. If specified location and location of the destination bigquery dataset do not match - the request will fail. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination_dataset_id` | String | The BigQuery target dataset id. |
| `managed_table_type` | String | The classification of the destination table. |
| `schedule` | String | Data transfer schedule. If the data source does not support a custom schedule, this should be empty. If it is empty, the default value for the data source will be used. The specified times are in UTC. Examples of valid format: `1st,3rd monday of month 15:30`, `every wed,fri of jan,jun 13:15`, and `first sunday of quarter 00:00`. See more explanation about the format here: https://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format NOTE: The minimum interval time between recurring transfers depends on the data source; refer to the documentation for your data source. |
| `user_id` | String | Deprecated. Unique ID of the user on whose behalf transfer is done. |
| `schedule_options` | String | Options customizing the data transfer schedule. |
| `update_time` | String | Output only. Data transfer modification time. Ignored by server on input. |
| `encryption_configuration` | String | The encryption configuration part. Currently, it is only used for the optional KMS key name. The BigQuery service account of your project must be granted permissions to use the key. Read methods will return the key name applied in effect. Write methods will apply the key if it is present, or otherwise try to apply project default keys if it is absent. |
| `params` | HashMap<String, String> | Parameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer' section for each data source. For example the parameters for Cloud Storage transfers are listed here: https://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq |
| `dataset_region` | String | Output only. Region in which BigQuery dataset is located. |
| `data_source_id` | String | Data source ID. This cannot be changed once data transfer is created. The full list of available data source IDs can be returned through an API call: https://cloud.google.com/bigquery-transfer/docs/reference/datatransfer/rest/v1/projects.locations.dataSources/list |
| `error` | String | Output only. Error code with detailed information about reason of the latest config failure. |
| `email_preferences` | String | Email notifications will be sent according to these preferences to the email address of the user who owns this transfer config. |
| `disabled` | bool | Is this config disabled. When set to true, no runs will be scheduled for this transfer config. |
| `display_name` | String | User specified display name for the data transfer. |
| `notification_pubsub_topic` | String | Pub/Sub topic where notifications will be sent after transfer runs associated with this transfer config finish. The format for specifying a pubsub topic is: `projects/{project_id}/topics/{topic_id}` |
| `schedule_options_v2` | String | Options customizing different types of data transfer schedule. This field replaces "schedule" and "schedule_options" fields. ScheduleOptionsV2 cannot be used together with ScheduleOptions/Schedule. |
| `state` | String | Output only. State of the most recently updated transfer run. |
| `owner_info` | String | Output only. Information about the user whose credentials are used to transfer data. Populated only for `transferConfigs.get` requests. In case the user information is not available, this field will not be populated. |
| `data_refresh_window_days` | i64 | The number of days to look back to automatically refresh the data. For example, if `data_refresh_window_days = 10`, then every day BigQuery reingests data for [today-10, today-1], rather than ingesting data for just [today-1]. Only valid if the data source supports the feature. Set the value to 0 to use the default value. |
| `name` | String | Identifier. The resource name of the transfer config. Transfer config names have the form either `projects/{project_id}/locations/{region}/transferConfigs/{config_id}` or `projects/{project_id}/transferConfigs/{config_id}`, where `config_id` is usually a UUID, even though it is not guaranteed or required. The name is ignored when creating a transfer config. |
| `next_run_time` | String | Output only. Next time when data transfer will run. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create transfer_config
transfer_config = provider.bigquerydatatransfer_api.Transfer_config {
    parent = "value"  # Required. The BigQuery project id where the transfer configuration should be created. Must be in the format projects/{project_id}/locations/{location_id} or projects/{project_id}. If specified location and location of the destination bigquery dataset do not match - the request will fail.
}

# Access transfer_config outputs
transfer_config_id = transfer_config.id
transfer_config_destination_dataset_id = transfer_config.destination_dataset_id
transfer_config_managed_table_type = transfer_config.managed_table_type
transfer_config_schedule = transfer_config.schedule
transfer_config_user_id = transfer_config.user_id
transfer_config_schedule_options = transfer_config.schedule_options
transfer_config_update_time = transfer_config.update_time
transfer_config_encryption_configuration = transfer_config.encryption_configuration
transfer_config_params = transfer_config.params
transfer_config_dataset_region = transfer_config.dataset_region
transfer_config_data_source_id = transfer_config.data_source_id
transfer_config_error = transfer_config.error
transfer_config_email_preferences = transfer_config.email_preferences
transfer_config_disabled = transfer_config.disabled
transfer_config_display_name = transfer_config.display_name
transfer_config_notification_pubsub_topic = transfer_config.notification_pubsub_topic
transfer_config_schedule_options_v2 = transfer_config.schedule_options_v2
transfer_config_state = transfer_config.state
transfer_config_owner_info = transfer_config.owner_info
transfer_config_data_refresh_window_days = transfer_config.data_refresh_window_days
transfer_config_name = transfer_config.name
transfer_config_next_run_time = transfer_config.next_run_time
```

---


### Data_source

Returns true if valid credentials exist for the given data source and requesting user.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the data source. If you are using the regionless method, the location must be `US` and the name should be in the following form: * `projects/{project_id}/dataSources/{data_source_id}` If you are using the regionalized method, the name should be in the following form: * `projects/{project_id}/locations/{location_id}/dataSources/{data_source_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `help_url` | String | Url for the help document for this data source. |
| `description` | String | User friendly data source description string. |
| `scopes` | Vec<String> | Api auth scopes for which refresh token needs to be obtained. These are scopes needed by a data source to prepare data and ingest them into BigQuery, e.g., https://www.googleapis.com/auth/bigquery |
| `default_schedule` | String | Default data transfer schedule. Examples of valid schedules include: `1st,3rd monday of month 15:30`, `every wed,fri of jan,jun 13:15`, and `first sunday of quarter 00:00`. |
| `name` | String | Output only. Data source resource name. |
| `transfer_type` | String | Deprecated. This field has no effect. |
| `parameters` | Vec<String> | Data source parameters. |
| `authorization_type` | String | Indicates the type of authorization. |
| `data_refresh_type` | String | Specifies whether the data source supports automatic data refresh for the past few days, and how it's supported. For some data sources, data might not be complete until a few days later, so it's useful to refresh data automatically. |
| `client_id` | String | Data source client id which should be used to receive refresh token. |
| `supports_custom_schedule` | bool | Specifies whether the data source supports a user defined schedule, or operates on the default schedule. When set to `true`, user can override default schedule. |
| `display_name` | String | User friendly data source name. |
| `update_deadline_seconds` | i64 | The number of seconds to wait for an update from the data source before the Data Transfer Service marks the transfer as FAILED. |
| `supports_multiple_transfers` | bool | Deprecated. This field has no effect. |
| `data_source_id` | String | Data source id. |
| `default_data_refresh_window_days` | i64 | Default data refresh window on days. Only meaningful when `data_refresh_type` = `SLIDING_WINDOW`. |
| `manual_runs_disabled` | bool | Disables backfilling and manual run scheduling for the data source. |
| `minimum_schedule_interval` | String | The minimum interval for scheduler to schedule runs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_source
data_source = provider.bigquerydatatransfer_api.Data_source {
    name = "value"  # Required. The name of the data source. If you are using the regionless method, the location must be `US` and the name should be in the following form: * `projects/{project_id}/dataSources/{data_source_id}` If you are using the regionalized method, the name should be in the following form: * `projects/{project_id}/locations/{location_id}/dataSources/{data_source_id}`
}

# Access data_source outputs
data_source_id = data_source.id
data_source_help_url = data_source.help_url
data_source_description = data_source.description
data_source_scopes = data_source.scopes
data_source_default_schedule = data_source.default_schedule
data_source_name = data_source.name
data_source_transfer_type = data_source.transfer_type
data_source_parameters = data_source.parameters
data_source_authorization_type = data_source.authorization_type
data_source_data_refresh_type = data_source.data_refresh_type
data_source_client_id = data_source.client_id
data_source_supports_custom_schedule = data_source.supports_custom_schedule
data_source_display_name = data_source.display_name
data_source_update_deadline_seconds = data_source.update_deadline_seconds
data_source_supports_multiple_transfers = data_source.supports_multiple_transfers
data_source_data_source_id = data_source.data_source_id
data_source_default_data_refresh_window_days = data_source.default_data_refresh_window_days
data_source_manual_runs_disabled = data_source.manual_runs_disabled
data_source_minimum_schedule_interval = data_source.minimum_schedule_interval
```

---


### Project

Enroll data sources in a user project. This allows users to create transfer configurations for these data sources. They will also appear in the ListDataSources RPC and as such, will appear in the [BigQuery UI](https://console.cloud.google.com/bigquery), and the documents can be found in the public guide for [BigQuery Web UI](https://cloud.google.com/bigquery/bigquery-web-ui) and [Data Transfer Service](https://cloud.google.com/bigquery/docs/working-with-transfers).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_source_ids` | Vec<String> |  | Data sources that are enrolled. It is required to provide at least one data source id. |
| `name` | String | ✅ | Required. The name of the project resource in the form: `projects/{project_id}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.bigquerydatatransfer_api.Project {
    name = "value"  # Required. The name of the project resource in the form: `projects/{project_id}`
}

```

---


### Run

Returns information about the particular transfer run.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `params` | HashMap<String, String> | Output only. Parameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer' section for each data source. For example the parameters for Cloud Storage transfers are listed here: https://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq |
| `notification_pubsub_topic` | String | Output only. Pub/Sub topic where a notification will be sent after this transfer run finishes. The format for specifying a pubsub topic is: `projects/{project_id}/topics/{topic_id}` |
| `update_time` | String | Output only. Last time the data transfer run state was updated. |
| `state` | String | Data transfer run state. Ignored for input requests. |
| `user_id` | String | Deprecated. Unique ID of the user on whose behalf transfer is done. |
| `schedule_time` | String | Minimum time after which a transfer run can be started. |
| `email_preferences` | String | Output only. Email notifications will be sent according to these preferences to the email address of the user who owns the transfer config this run was derived from. |
| `run_time` | String | For batch transfer runs, specifies the date and time of the data should be ingested. |
| `start_time` | String | Output only. Time when transfer run was started. Parameter ignored by server for input requests. |
| `error_status` | String | Status of the transfer run. |
| `destination_dataset_id` | String | Output only. The BigQuery target dataset id. |
| `data_source_id` | String | Output only. Data source id. |
| `schedule` | String | Output only. Describes the schedule of this transfer run if it was created as part of a regular schedule. For batch transfer runs that are scheduled manually, this is empty. NOTE: the system might choose to delay the schedule depending on the current load, so `schedule_time` doesn't always match this. |
| `end_time` | String | Output only. Time when transfer run ended. Parameter ignored by server for input requests. |
| `name` | String | Identifier. The resource name of the transfer run. Transfer run names have the form `projects/{project_id}/locations/{location}/transferConfigs/{config_id}/runs/{run_id}`. The name is ignored when creating a transfer run. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access run outputs
run_id = run.id
run_params = run.params
run_notification_pubsub_topic = run.notification_pubsub_topic
run_update_time = run.update_time
run_state = run.state
run_user_id = run.user_id
run_schedule_time = run.schedule_time
run_email_preferences = run.email_preferences
run_run_time = run.run_time
run_start_time = run.start_time
run_error_status = run.error_status
run_destination_dataset_id = run.destination_dataset_id
run_data_source_id = run.data_source_id
run_schedule = run.schedule
run_end_time = run.end_time
run_name = run.name
```

---


### Transfer_log

Returns log messages for the transfer run.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Output only. The next-pagination token. For multiple-page list results, this token can be used as the `GetTransferRunLogRequest.page_token` to request the next page of list results. |
| `transfer_messages` | Vec<String> | Output only. The stored pipeline transfer messages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access transfer_log outputs
transfer_log_id = transfer_log.id
transfer_log_next_page_token = transfer_log.next_page_token
transfer_log_transfer_messages = transfer_log.transfer_messages
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple location resources
location_0 = provider.bigquerydatatransfer_api.Location {
    name = "value-0"
}
location_1 = provider.bigquerydatatransfer_api.Location {
    name = "value-1"
}
location_2 = provider.bigquerydatatransfer_api.Location {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.bigquerydatatransfer_api.Location {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Bigquerydatatransfer_api Documentation](https://cloud.google.com/bigquerydatatransfer_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
