# Analytics_api Service



**Resources**: 32

---

## Overview

The analytics_api service provides access to 32 resource types:

- [Experiment](#experiment) [CRUD]
- [Mcf](#mcf) [R]
- [Provisioning](#provisioning) [C]
- [Account_summarie](#account_summarie) [R]
- [Unsampled_report](#unsampled_report) [CRD]
- [Account](#account) [R]
- [Realtime](#realtime) [R]
- [Goal](#goal) [CRU]
- [Profile_user_link](#profile_user_link) [CRUD]
- [Segment](#segment) [R]
- [Profile_filter_link](#profile_filter_link) [CRUD]
- [Upload](#upload) [CR]
- [Filter](#filter) [CRUD]
- [Webpropertie](#webpropertie) [CRU]
- [Custom_dimension](#custom_dimension) [CRU]
- [Custom_data_source](#custom_data_source) [R]
- [Column](#column) [R]
- [Client_id](#client_id) [C]
- [Remarketing_audience](#remarketing_audience) [CRUD]
- [Webproperty_user_link](#webproperty_user_link) [CRUD]
- [Custom_metric](#custom_metric) [CRU]
- [User_deletion_request](#user_deletion_request) [C]
- [Web_property_ad_words_link](#web_property_ad_words_link) [CRUD]
- [Ga](#ga) [R]
- [Account_user_link](#account_user_link) [CRUD]
- [Profile](#profile) [CRUD]
- [Profile](#profile) [R]
- [Data](#data) [R]
- [Goal](#goal) [R]
- [Account](#account) [R]
- [Segment](#segment) [R]
- [Webpropertie](#webpropertie) [R]

---

## Resources


### Experiment

Create a new experiment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String |  | Account ID to which this experiment belongs. This field is read-only. |
| `name` | String |  | Experiment name. This field may not be changed for an experiment whose status is ENDED. This field is required when creating an experiment. |
| `objective_metric` | String |  | The metric that the experiment is optimizing. Valid values: "ga:goal(n)Completions", "ga:adsenseAdsClicks", "ga:adsenseAdsViewed", "ga:adsenseRevenue", "ga:bounces", "ga:pageviews", "ga:sessionDuration", "ga:transactions", "ga:transactionRevenue". This field is required if status is "RUNNING" and servingFramework is one of "REDIRECT" or "API". |
| `description` | String |  | Notes about this experiment. |
| `reason_experiment_ended` | String |  | Why the experiment ended. Possible values: "STOPPED_BY_USER", "WINNER_FOUND", "EXPERIMENT_EXPIRED", "ENDED_WITH_NO_WINNER", "GOAL_OBJECTIVE_CHANGED". "ENDED_WITH_NO_WINNER" means that the experiment didn't expire but no winner was projected to be found. If the experiment status is changed via the API to ENDED this field is set to STOPPED_BY_USER. This field is read-only. |
| `minimum_experiment_length_in_days` | i64 |  | An integer number in [3, 90]. Specifies the minimum length of the experiment. Can be changed for a running experiment. This field may not be changed for an experiments whose status is ENDED. |
| `internal_web_property_id` | String |  | Internal ID for the web property to which this experiment belongs. This field is read-only. |
| `traffic_coverage` | f64 |  | A floating-point number in (0, 1]. Specifies the fraction of the traffic that participates in the experiment. Can be changed for a running experiment. This field may not be changed for an experiments whose status is ENDED. |
| `optimization_type` | String |  | Whether the objectiveMetric should be minimized or maximized. Possible values: "MAXIMUM", "MINIMUM". Optional--defaults to "MAXIMUM". Cannot be specified without objectiveMetric. Cannot be modified when status is "RUNNING" or "ENDED". |
| `parent_link` | String |  | Parent link for an experiment. Points to the view (profile) to which this experiment belongs. |
| `serving_framework` | String |  | The framework used to serve the experiment variations and evaluate the results. One of:  
- REDIRECT: Google Analytics redirects traffic to different variation pages, reports the chosen variation and evaluates the results.
- API: Google Analytics chooses and reports the variation to serve and evaluates the results; the caller is responsible for serving the selected variation.
- EXTERNAL: The variations will be served externally and the chosen variation reported to Google Analytics. The caller is responsible for serving the selected variation and evaluating the results. |
| `status` | String |  | Experiment status. Possible values: "DRAFT", "READY_TO_RUN", "RUNNING", "ENDED". Experiments can be created in the "DRAFT", "READY_TO_RUN" or "RUNNING" state. This field is required when creating an experiment. |
| `equal_weighting` | bool |  | Boolean specifying whether to distribute traffic evenly across all variations. If the value is False, content experiments follows the default behavior of adjusting traffic dynamically based on variation performance. Optional -- defaults to False. This field may not be changed for an experiment whose status is ENDED. |
| `start_time` | String |  | The starting time of the experiment (the time the status changed from READY_TO_RUN to RUNNING). This field is present only if the experiment has started. This field is read-only. |
| `snippet` | String |  | The snippet of code to include on the control page(s). This field is read-only. |
| `end_time` | String |  | The ending time of the experiment (the time the status changed from RUNNING to ENDED). This field is present only if the experiment has ended. This field is read-only. |
| `kind` | String |  | Resource type for an Analytics experiment. This field is read-only. |
| `id` | String |  | Experiment ID. Required for patch and update. Disallowed for create. |
| `updated` | String |  | Time the experiment was last modified. This field is read-only. |
| `rewrite_variation_urls_as_original` | bool |  | Boolean specifying whether variations URLS are rewritten to match those of the original. This field may not be changed for an experiments whose status is ENDED. |
| `created` | String |  | Time the experiment was created. This field is read-only. |
| `editable_in_ga_ui` | bool |  | If true, the end user will be able to edit the experiment via the Google Analytics user interface. |
| `profile_id` | String |  | View (Profile) ID to which this experiment belongs. This field is read-only. |
| `web_property_id` | String |  | Web property ID to which this experiment belongs. The web property ID is of the form UA-XXXXX-YY. This field is read-only. |
| `variations` | Vec<String> |  | Array of variations. The first variation in the array is the original. The number of variations may not change once an experiment is in the RUNNING state. At least two variations are required before status can be set to RUNNING. |
| `winner_confidence_level` | f64 |  | A floating-point number in (0, 1). Specifies the necessary confidence level to choose a winner. This field may not be changed for an experiments whose status is ENDED. |
| `winner_found` | bool |  | Boolean specifying whether a winner has been found for this experiment. This field is read-only. |
| `self_link` | String |  | Link for this experiment. This field is read-only. |
| `account_id` | String | ✅ | Account ID to create the experiment for. |
| `profile_id` | String | ✅ | View (Profile) ID to create the experiment for. |
| `web_property_id` | String | ✅ | Web property ID to create the experiment for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | Account ID to which this experiment belongs. This field is read-only. |
| `name` | String | Experiment name. This field may not be changed for an experiment whose status is ENDED. This field is required when creating an experiment. |
| `objective_metric` | String | The metric that the experiment is optimizing. Valid values: "ga:goal(n)Completions", "ga:adsenseAdsClicks", "ga:adsenseAdsViewed", "ga:adsenseRevenue", "ga:bounces", "ga:pageviews", "ga:sessionDuration", "ga:transactions", "ga:transactionRevenue". This field is required if status is "RUNNING" and servingFramework is one of "REDIRECT" or "API". |
| `description` | String | Notes about this experiment. |
| `reason_experiment_ended` | String | Why the experiment ended. Possible values: "STOPPED_BY_USER", "WINNER_FOUND", "EXPERIMENT_EXPIRED", "ENDED_WITH_NO_WINNER", "GOAL_OBJECTIVE_CHANGED". "ENDED_WITH_NO_WINNER" means that the experiment didn't expire but no winner was projected to be found. If the experiment status is changed via the API to ENDED this field is set to STOPPED_BY_USER. This field is read-only. |
| `minimum_experiment_length_in_days` | i64 | An integer number in [3, 90]. Specifies the minimum length of the experiment. Can be changed for a running experiment. This field may not be changed for an experiments whose status is ENDED. |
| `internal_web_property_id` | String | Internal ID for the web property to which this experiment belongs. This field is read-only. |
| `traffic_coverage` | f64 | A floating-point number in (0, 1]. Specifies the fraction of the traffic that participates in the experiment. Can be changed for a running experiment. This field may not be changed for an experiments whose status is ENDED. |
| `optimization_type` | String | Whether the objectiveMetric should be minimized or maximized. Possible values: "MAXIMUM", "MINIMUM". Optional--defaults to "MAXIMUM". Cannot be specified without objectiveMetric. Cannot be modified when status is "RUNNING" or "ENDED". |
| `parent_link` | String | Parent link for an experiment. Points to the view (profile) to which this experiment belongs. |
| `serving_framework` | String | The framework used to serve the experiment variations and evaluate the results. One of:  
- REDIRECT: Google Analytics redirects traffic to different variation pages, reports the chosen variation and evaluates the results.
- API: Google Analytics chooses and reports the variation to serve and evaluates the results; the caller is responsible for serving the selected variation.
- EXTERNAL: The variations will be served externally and the chosen variation reported to Google Analytics. The caller is responsible for serving the selected variation and evaluating the results. |
| `status` | String | Experiment status. Possible values: "DRAFT", "READY_TO_RUN", "RUNNING", "ENDED". Experiments can be created in the "DRAFT", "READY_TO_RUN" or "RUNNING" state. This field is required when creating an experiment. |
| `equal_weighting` | bool | Boolean specifying whether to distribute traffic evenly across all variations. If the value is False, content experiments follows the default behavior of adjusting traffic dynamically based on variation performance. Optional -- defaults to False. This field may not be changed for an experiment whose status is ENDED. |
| `start_time` | String | The starting time of the experiment (the time the status changed from READY_TO_RUN to RUNNING). This field is present only if the experiment has started. This field is read-only. |
| `snippet` | String | The snippet of code to include on the control page(s). This field is read-only. |
| `end_time` | String | The ending time of the experiment (the time the status changed from RUNNING to ENDED). This field is present only if the experiment has ended. This field is read-only. |
| `kind` | String | Resource type for an Analytics experiment. This field is read-only. |
| `id` | String | Experiment ID. Required for patch and update. Disallowed for create. |
| `updated` | String | Time the experiment was last modified. This field is read-only. |
| `rewrite_variation_urls_as_original` | bool | Boolean specifying whether variations URLS are rewritten to match those of the original. This field may not be changed for an experiments whose status is ENDED. |
| `created` | String | Time the experiment was created. This field is read-only. |
| `editable_in_ga_ui` | bool | If true, the end user will be able to edit the experiment via the Google Analytics user interface. |
| `profile_id` | String | View (Profile) ID to which this experiment belongs. This field is read-only. |
| `web_property_id` | String | Web property ID to which this experiment belongs. The web property ID is of the form UA-XXXXX-YY. This field is read-only. |
| `variations` | Vec<String> | Array of variations. The first variation in the array is the original. The number of variations may not change once an experiment is in the RUNNING state. At least two variations are required before status can be set to RUNNING. |
| `winner_confidence_level` | f64 | A floating-point number in (0, 1). Specifies the necessary confidence level to choose a winner. This field may not be changed for an experiments whose status is ENDED. |
| `winner_found` | bool | Boolean specifying whether a winner has been found for this experiment. This field is read-only. |
| `self_link` | String | Link for this experiment. This field is read-only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create experiment
experiment = provider.analytics_api.Experiment {
    account_id = "value"  # Account ID to create the experiment for.
    profile_id = "value"  # View (Profile) ID to create the experiment for.
    web_property_id = "value"  # Web property ID to create the experiment for.
}

# Access experiment outputs
experiment_id = experiment.id
experiment_account_id = experiment.account_id
experiment_name = experiment.name
experiment_objective_metric = experiment.objective_metric
experiment_description = experiment.description
experiment_reason_experiment_ended = experiment.reason_experiment_ended
experiment_minimum_experiment_length_in_days = experiment.minimum_experiment_length_in_days
experiment_internal_web_property_id = experiment.internal_web_property_id
experiment_traffic_coverage = experiment.traffic_coverage
experiment_optimization_type = experiment.optimization_type
experiment_parent_link = experiment.parent_link
experiment_serving_framework = experiment.serving_framework
experiment_status = experiment.status
experiment_equal_weighting = experiment.equal_weighting
experiment_start_time = experiment.start_time
experiment_snippet = experiment.snippet
experiment_end_time = experiment.end_time
experiment_kind = experiment.kind
experiment_id = experiment.id
experiment_updated = experiment.updated
experiment_rewrite_variation_urls_as_original = experiment.rewrite_variation_urls_as_original
experiment_created = experiment.created
experiment_editable_in_ga_ui = experiment.editable_in_ga_ui
experiment_profile_id = experiment.profile_id
experiment_web_property_id = experiment.web_property_id
experiment_variations = experiment.variations
experiment_winner_confidence_level = experiment.winner_confidence_level
experiment_winner_found = experiment.winner_found
experiment_self_link = experiment.self_link
```

---


### Mcf

Returns Analytics Multi-Channel Funnels data for a view (profile).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `contains_sampled_data` | bool | Determines if the Analytics data contains sampled data. |
| `items_per_page` | i64 | The maximum number of rows the response can contain, regardless of the actual number of rows returned. Its value ranges from 1 to 10,000 with a value of 1000 by default, or otherwise specified by the max-results query parameter. |
| `next_link` | String | Link to next page for this Analytics data query. |
| `kind` | String | Resource type. |
| `sample_size` | String | The number of samples used to calculate the result. |
| `self_link` | String | Link to this page. |
| `previous_link` | String | Link to previous page for this Analytics data query. |
| `column_headers` | Vec<String> | Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request. |
| `sample_space` | String | Total size of the sample space from which the samples were selected. |
| `id` | String | Unique ID for this data response. |
| `total_results` | i64 | The total number of rows for the query, regardless of the number of rows in the response. |
| `query` | String | Analytics data request query parameters. |
| `profile_info` | String | Information for the view (profile), for which the Analytics data was requested. |
| `rows` | Vec<Vec<String>> | Analytics data rows, where each row contains a list of dimension values followed by the metric values. The order of dimensions and metrics is same as specified in the request. |
| `totals_for_all_results` | HashMap<String, String> | Total values for the requested metrics over all the results, not just the results returned in this response. The order of the metric totals is same as the metric order specified in the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access mcf outputs
mcf_id = mcf.id
mcf_contains_sampled_data = mcf.contains_sampled_data
mcf_items_per_page = mcf.items_per_page
mcf_next_link = mcf.next_link
mcf_kind = mcf.kind
mcf_sample_size = mcf.sample_size
mcf_self_link = mcf.self_link
mcf_previous_link = mcf.previous_link
mcf_column_headers = mcf.column_headers
mcf_sample_space = mcf.sample_space
mcf_id = mcf.id
mcf_total_results = mcf.total_results
mcf_query = mcf.query
mcf_profile_info = mcf.profile_info
mcf_rows = mcf.rows
mcf_totals_for_all_results = mcf.totals_for_all_results
```

---


### Provisioning

Creates an account ticket.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Account ticket ID used to access the account ticket. |
| `redirect_uri` | String |  | Redirect URI where the user will be sent after accepting Terms of Service. Must be configured in APIs console as a callback URL. |
| `account` | String |  | Account for this ticket. |
| `kind` | String |  | Resource type for account ticket. |
| `profile` | String |  | View (Profile) for the account. |
| `webproperty` | String |  | Web property for the account. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create provisioning
provisioning = provider.analytics_api.Provisioning {
}

```

---


### Account_summarie

Lists account summaries (lightweight tree comprised of accounts/properties/profiles) to which the user has access.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Collection type. |
| `next_link` | String | Link to next page for this AccountSummary collection. |
| `start_index` | i64 | The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter. |
| `items_per_page` | i64 | The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter. |
| `username` | String | Email ID of the authenticated user |
| `previous_link` | String | Link to previous page for this AccountSummary collection. |
| `items` | Vec<String> | A list of AccountSummaries. |
| `total_results` | i64 | The total number of results for the query, regardless of the number of results in the response. |


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
account_summarie_kind = account_summarie.kind
account_summarie_next_link = account_summarie.next_link
account_summarie_start_index = account_summarie.start_index
account_summarie_items_per_page = account_summarie.items_per_page
account_summarie_username = account_summarie.username
account_summarie_previous_link = account_summarie.previous_link
account_summarie_items = account_summarie.items
account_summarie_total_results = account_summarie.total_results
```

---


### Unsampled_report

Create a new unsampled report.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Resource type for an Analytics unsampled report. |
| `filters` | String |  | The filters for the unsampled report. |
| `end_date` | String |  | The end date for the unsampled report. |
| `cloud_storage_download_details` | String |  | Download details for a file stored in Google Cloud Storage. |
| `start_date` | String |  | The start date for the unsampled report. |
| `web_property_id` | String |  | Web property ID to which this unsampled report belongs. The web property ID is of the form UA-XXXXX-YY. |
| `segment` | String |  | The segment for the unsampled report. |
| `account_id` | String |  | Account ID to which this unsampled report belongs. |
| `updated` | String |  | Time this unsampled report was last modified. |
| `status` | String |  | Status of this unsampled report. Possible values are PENDING, COMPLETED, or FAILED. |
| `dimensions` | String |  | The dimensions for the unsampled report. |
| `download_type` | String |  | The type of download you need to use for the report data file. Possible values include `GOOGLE_DRIVE` and `GOOGLE_CLOUD_STORAGE`. If the value is `GOOGLE_DRIVE`, see the `driveDownloadDetails` field. If the value is `GOOGLE_CLOUD_STORAGE`, see the `cloudStorageDownloadDetails` field. |
| `title` | String |  | Title of the unsampled report. |
| `id` | String |  | Unsampled report ID. |
| `metrics` | String |  | The metrics for the unsampled report. |
| `self_link` | String |  | Link for this unsampled report. |
| `created` | String |  | Time this unsampled report was created. |
| `drive_download_details` | String |  | Download details for a file stored in Google Drive. |
| `profile_id` | String |  | View (Profile) ID to which this unsampled report belongs. |
| `account_id` | String | ✅ | Account ID to create the unsampled report for. |
| `web_property_id` | String | ✅ | Web property ID to create the unsampled report for. |
| `profile_id` | String | ✅ | View (Profile) ID to create the unsampled report for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Resource type for an Analytics unsampled report. |
| `filters` | String | The filters for the unsampled report. |
| `end_date` | String | The end date for the unsampled report. |
| `cloud_storage_download_details` | String | Download details for a file stored in Google Cloud Storage. |
| `start_date` | String | The start date for the unsampled report. |
| `web_property_id` | String | Web property ID to which this unsampled report belongs. The web property ID is of the form UA-XXXXX-YY. |
| `segment` | String | The segment for the unsampled report. |
| `account_id` | String | Account ID to which this unsampled report belongs. |
| `updated` | String | Time this unsampled report was last modified. |
| `status` | String | Status of this unsampled report. Possible values are PENDING, COMPLETED, or FAILED. |
| `dimensions` | String | The dimensions for the unsampled report. |
| `download_type` | String | The type of download you need to use for the report data file. Possible values include `GOOGLE_DRIVE` and `GOOGLE_CLOUD_STORAGE`. If the value is `GOOGLE_DRIVE`, see the `driveDownloadDetails` field. If the value is `GOOGLE_CLOUD_STORAGE`, see the `cloudStorageDownloadDetails` field. |
| `title` | String | Title of the unsampled report. |
| `id` | String | Unsampled report ID. |
| `metrics` | String | The metrics for the unsampled report. |
| `self_link` | String | Link for this unsampled report. |
| `created` | String | Time this unsampled report was created. |
| `drive_download_details` | String | Download details for a file stored in Google Drive. |
| `profile_id` | String | View (Profile) ID to which this unsampled report belongs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create unsampled_report
unsampled_report = provider.analytics_api.Unsampled_report {
    account_id = "value"  # Account ID to create the unsampled report for.
    web_property_id = "value"  # Web property ID to create the unsampled report for.
    profile_id = "value"  # View (Profile) ID to create the unsampled report for.
}

# Access unsampled_report outputs
unsampled_report_id = unsampled_report.id
unsampled_report_kind = unsampled_report.kind
unsampled_report_filters = unsampled_report.filters
unsampled_report_end_date = unsampled_report.end_date
unsampled_report_cloud_storage_download_details = unsampled_report.cloud_storage_download_details
unsampled_report_start_date = unsampled_report.start_date
unsampled_report_web_property_id = unsampled_report.web_property_id
unsampled_report_segment = unsampled_report.segment
unsampled_report_account_id = unsampled_report.account_id
unsampled_report_updated = unsampled_report.updated
unsampled_report_status = unsampled_report.status
unsampled_report_dimensions = unsampled_report.dimensions
unsampled_report_download_type = unsampled_report.download_type
unsampled_report_title = unsampled_report.title
unsampled_report_id = unsampled_report.id
unsampled_report_metrics = unsampled_report.metrics
unsampled_report_self_link = unsampled_report.self_link
unsampled_report_created = unsampled_report.created
unsampled_report_drive_download_details = unsampled_report.drive_download_details
unsampled_report_profile_id = unsampled_report.profile_id
```

---


### Account

Lists all accounts to which the user has access.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_link` | String | Next link for this account collection. |
| `total_results` | i64 | The total number of results for the query, regardless of the number of results in the response. |
| `kind` | String | Collection type. |
| `items_per_page` | i64 | The maximum number of entries the response can contain, regardless of the actual number of entries returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter. |
| `items` | Vec<String> | A list of accounts. |
| `username` | String | Email ID of the authenticated user |
| `previous_link` | String | Previous link for this account collection. |
| `start_index` | i64 | The starting index of the entries, which is 1 by default or otherwise specified by the start-index query parameter. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account outputs
account_id = account.id
account_next_link = account.next_link
account_total_results = account.total_results
account_kind = account.kind
account_items_per_page = account.items_per_page
account_items = account.items
account_username = account.username
account_previous_link = account.previous_link
account_start_index = account.start_index
```

---


### Realtime

Returns real time data for a view (profile).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `totals_for_all_results` | HashMap<String, String> | Total values for the requested metrics over all the results, not just the results returned in this response. The order of the metric totals is same as the metric order specified in the request. |
| `id` | String | Unique ID for this data response. |
| `rows` | Vec<Vec<String>> | Real time data rows, where each row contains a list of dimension values followed by the metric values. The order of dimensions and metrics is same as specified in the request. |
| `self_link` | String | Link to this page. |
| `kind` | String | Resource type. |
| `total_results` | i64 | The total number of rows for the query, regardless of the number of rows in the response. |
| `column_headers` | Vec<String> | Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request. |
| `query` | String | Real time data request query parameters. |
| `profile_info` | String | Information for the view (profile), for which the real time data was requested. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access realtime outputs
realtime_id = realtime.id
realtime_totals_for_all_results = realtime.totals_for_all_results
realtime_id = realtime.id
realtime_rows = realtime.rows
realtime_self_link = realtime.self_link
realtime_kind = realtime.kind
realtime_total_results = realtime.total_results
realtime_column_headers = realtime.column_headers
realtime_query = realtime.query
realtime_profile_info = realtime.profile_info
```

---


### Goal

Create a new goal.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Goal type. Possible values are URL_DESTINATION, VISIT_TIME_ON_SITE, VISIT_NUM_PAGES, AND EVENT. |
| `internal_web_property_id` | String |  | Internal ID for the web property to which this goal belongs. |
| `event_details` | String |  | Details for the goal of the type EVENT. |
| `name` | String |  | Goal name. |
| `parent_link` | String |  | Parent link for a goal. Points to the view (profile) to which this goal belongs. |
| `self_link` | String |  | Link for this goal. |
| `created` | String |  | Time this goal was created. |
| `visit_num_pages_details` | String |  | Details for the goal of the type VISIT_NUM_PAGES. |
| `id` | String |  | Goal ID. |
| `web_property_id` | String |  | Web property ID to which this goal belongs. The web property ID is of the form UA-XXXXX-YY. |
| `kind` | String |  | Resource type for an Analytics goal. |
| `updated` | String |  | Time this goal was last modified. |
| `account_id` | String |  | Account ID to which this goal belongs. |
| `url_destination_details` | String |  | Details for the goal of the type URL_DESTINATION. |
| `profile_id` | String |  | View (Profile) ID to which this goal belongs. |
| `value` | f64 |  | Goal value. |
| `active` | bool |  | Determines whether this goal is active. |
| `visit_time_on_site_details` | String |  | Details for the goal of the type VISIT_TIME_ON_SITE. |
| `profile_id` | String | ✅ | View (Profile) ID to create the goal for. |
| `account_id` | String | ✅ | Account ID to create the goal for. |
| `web_property_id` | String | ✅ | Web property ID to create the goal for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Goal type. Possible values are URL_DESTINATION, VISIT_TIME_ON_SITE, VISIT_NUM_PAGES, AND EVENT. |
| `internal_web_property_id` | String | Internal ID for the web property to which this goal belongs. |
| `event_details` | String | Details for the goal of the type EVENT. |
| `name` | String | Goal name. |
| `parent_link` | String | Parent link for a goal. Points to the view (profile) to which this goal belongs. |
| `self_link` | String | Link for this goal. |
| `created` | String | Time this goal was created. |
| `visit_num_pages_details` | String | Details for the goal of the type VISIT_NUM_PAGES. |
| `id` | String | Goal ID. |
| `web_property_id` | String | Web property ID to which this goal belongs. The web property ID is of the form UA-XXXXX-YY. |
| `kind` | String | Resource type for an Analytics goal. |
| `updated` | String | Time this goal was last modified. |
| `account_id` | String | Account ID to which this goal belongs. |
| `url_destination_details` | String | Details for the goal of the type URL_DESTINATION. |
| `profile_id` | String | View (Profile) ID to which this goal belongs. |
| `value` | f64 | Goal value. |
| `active` | bool | Determines whether this goal is active. |
| `visit_time_on_site_details` | String | Details for the goal of the type VISIT_TIME_ON_SITE. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create goal
goal = provider.analytics_api.Goal {
    profile_id = "value"  # View (Profile) ID to create the goal for.
    account_id = "value"  # Account ID to create the goal for.
    web_property_id = "value"  # Web property ID to create the goal for.
}

# Access goal outputs
goal_id = goal.id
goal_type = goal.type
goal_internal_web_property_id = goal.internal_web_property_id
goal_event_details = goal.event_details
goal_name = goal.name
goal_parent_link = goal.parent_link
goal_self_link = goal.self_link
goal_created = goal.created
goal_visit_num_pages_details = goal.visit_num_pages_details
goal_id = goal.id
goal_web_property_id = goal.web_property_id
goal_kind = goal.kind
goal_updated = goal.updated
goal_account_id = goal.account_id
goal_url_destination_details = goal.url_destination_details
goal_profile_id = goal.profile_id
goal_value = goal.value
goal_active = goal.active
goal_visit_time_on_site_details = goal.visit_time_on_site_details
```

---


### Profile_user_link

Adds a new user to the given view (profile).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_ref` | String |  | User reference. |
| `self_link` | String |  | Self link for this resource. |
| `entity` | String |  | Entity for this link. It can be an account, a web property, or a view (profile). |
| `id` | String |  | Entity user link ID |
| `permissions` | String |  | Permissions the user has for this entity. |
| `kind` | String |  | Resource type for entity user link. |
| `account_id` | String | ✅ | Account ID to create the user link for. |
| `profile_id` | String | ✅ | View (Profile) ID to create the user link for. |
| `web_property_id` | String | ✅ | Web Property ID to create the user link for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of entity user links. |
| `next_link` | String | Next link for this account collection. |
| `items_per_page` | i64 | The maximum number of entries the response can contain, regardless of the actual number of entries returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter. |
| `kind` | String | Collection type. |
| `previous_link` | String | Previous link for this account collection. |
| `total_results` | i64 | The total number of results for the query, regardless of the number of results in the response. |
| `start_index` | i64 | The starting index of the entries, which is 1 by default or otherwise specified by the start-index query parameter. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create profile_user_link
profile_user_link = provider.analytics_api.Profile_user_link {
    account_id = "value"  # Account ID to create the user link for.
    profile_id = "value"  # View (Profile) ID to create the user link for.
    web_property_id = "value"  # Web Property ID to create the user link for.
}

# Access profile_user_link outputs
profile_user_link_id = profile_user_link.id
profile_user_link_items = profile_user_link.items
profile_user_link_next_link = profile_user_link.next_link
profile_user_link_items_per_page = profile_user_link.items_per_page
profile_user_link_kind = profile_user_link.kind
profile_user_link_previous_link = profile_user_link.previous_link
profile_user_link_total_results = profile_user_link.total_results
profile_user_link_start_index = profile_user_link.start_index
```

---


### Segment

Lists segments to which the user has access.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Collection type for segments. |
| `items` | Vec<String> | A list of segments. |
| `items_per_page` | i64 | The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter. |
| `next_link` | String | Link to next page for this segment collection. |
| `start_index` | i64 | The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter. |
| `total_results` | i64 | The total number of results for the query, regardless of the number of results in the response. |
| `username` | String | Email ID of the authenticated user |
| `previous_link` | String | Link to previous page for this segment collection. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access segment outputs
segment_id = segment.id
segment_kind = segment.kind
segment_items = segment.items
segment_items_per_page = segment.items_per_page
segment_next_link = segment.next_link
segment_start_index = segment.start_index
segment_total_results = segment.total_results
segment_username = segment.username
segment_previous_link = segment.previous_link
```

---


### Profile_filter_link

Create a new profile filter link.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rank` | i64 |  | The rank of this profile filter link relative to the other filters linked to the same profile.
For readonly (i.e., list and get) operations, the rank always starts at 1.
For write (i.e., create, update, or delete) operations, you may specify a value between 0 and 255 inclusively, [0, 255]. In order to insert a link at the end of the list, either don't specify a rank or set a rank to a number greater than the largest rank in the list. In order to insert a link to the beginning of the list specify a rank that is less than or equal to 1. The new link will move all existing filters with the same or lower rank down the list. After the link is inserted/updated/deleted all profile filter links will be renumbered starting at 1. |
| `profile_ref` | String |  | View (Profile) for this link. |
| `id` | String |  | Profile filter link ID. |
| `self_link` | String |  | Link for this profile filter link. |
| `kind` | String |  | Resource type for Analytics filter. |
| `filter_ref` | String |  | Filter for this link. |
| `account_id` | String | ✅ | Account ID to create profile filter link for. |
| `profile_id` | String | ✅ | Profile ID to create filter link for. |
| `web_property_id` | String | ✅ | Web property Id to create profile filter link for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rank` | i64 | The rank of this profile filter link relative to the other filters linked to the same profile.
For readonly (i.e., list and get) operations, the rank always starts at 1.
For write (i.e., create, update, or delete) operations, you may specify a value between 0 and 255 inclusively, [0, 255]. In order to insert a link at the end of the list, either don't specify a rank or set a rank to a number greater than the largest rank in the list. In order to insert a link to the beginning of the list specify a rank that is less than or equal to 1. The new link will move all existing filters with the same or lower rank down the list. After the link is inserted/updated/deleted all profile filter links will be renumbered starting at 1. |
| `profile_ref` | String | View (Profile) for this link. |
| `id` | String | Profile filter link ID. |
| `self_link` | String | Link for this profile filter link. |
| `kind` | String | Resource type for Analytics filter. |
| `filter_ref` | String | Filter for this link. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create profile_filter_link
profile_filter_link = provider.analytics_api.Profile_filter_link {
    account_id = "value"  # Account ID to create profile filter link for.
    profile_id = "value"  # Profile ID to create filter link for.
    web_property_id = "value"  # Web property Id to create profile filter link for.
}

# Access profile_filter_link outputs
profile_filter_link_id = profile_filter_link.id
profile_filter_link_rank = profile_filter_link.rank
profile_filter_link_profile_ref = profile_filter_link.profile_ref
profile_filter_link_id = profile_filter_link.id
profile_filter_link_self_link = profile_filter_link.self_link
profile_filter_link_kind = profile_filter_link.kind
profile_filter_link_filter_ref = profile_filter_link.filter_ref
```

---


### Upload

Upload data for a custom data source.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String | ✅ | Account Id associated with the upload. |
| `custom_data_source_id` | String | ✅ | Custom data source Id to which the data being uploaded belongs. |
| `web_property_id` | String | ✅ | Web property UA-string associated with the upload. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | A unique ID for this upload. |
| `status` | String | Upload status. Possible values: PENDING, COMPLETED, FAILED, DELETING, DELETED. |
| `errors` | Vec<String> | Data import errors collection. |
| `kind` | String | Resource type for Analytics upload. |
| `upload_time` | String | Time this file is uploaded. |
| `account_id` | String | Account Id to which this upload belongs. |
| `custom_data_source_id` | String | Custom data source Id to which this data import belongs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create upload
upload = provider.analytics_api.Upload {
    account_id = "value"  # Account Id associated with the upload.
    custom_data_source_id = "value"  # Custom data source Id to which the data being uploaded belongs.
    web_property_id = "value"  # Web property UA-string associated with the upload.
}

# Access upload outputs
upload_id = upload.id
upload_id = upload.id
upload_status = upload.status
upload_errors = upload.errors
upload_kind = upload.kind
upload_upload_time = upload.upload_time
upload_account_id = upload.account_id
upload_custom_data_source_id = upload.custom_data_source_id
```

---


### Filter

Create a new filter.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uppercase_details` | String |  | Details for the filter of the type UPPER. |
| `include_details` | String |  | Details for the filter of the type INCLUDE. |
| `name` | String |  | Name of this filter. |
| `parent_link` | String |  | Parent link for this filter. Points to the account to which this filter belongs. |
| `type` | String |  | Type of this filter. Possible values are INCLUDE, EXCLUDE, LOWERCASE, UPPERCASE, SEARCH_AND_REPLACE and ADVANCED. |
| `id` | String |  | Filter ID. |
| `exclude_details` | String |  | Details for the filter of the type EXCLUDE. |
| `lowercase_details` | String |  | Details for the filter of the type LOWER. |
| `advanced_details` | String |  | Details for the filter of the type ADVANCED. |
| `updated` | String |  | Time this filter was last modified. |
| `created` | String |  | Time this filter was created. |
| `self_link` | String |  | Link for this filter. |
| `account_id` | String |  | Account ID to which this filter belongs. |
| `kind` | String |  | Resource type for Analytics filter. |
| `search_and_replace_details` | String |  | Details for the filter of the type SEARCH_AND_REPLACE. |
| `account_id` | String | ✅ | Account ID to create filter for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uppercase_details` | String | Details for the filter of the type UPPER. |
| `include_details` | String | Details for the filter of the type INCLUDE. |
| `name` | String | Name of this filter. |
| `parent_link` | String | Parent link for this filter. Points to the account to which this filter belongs. |
| `type` | String | Type of this filter. Possible values are INCLUDE, EXCLUDE, LOWERCASE, UPPERCASE, SEARCH_AND_REPLACE and ADVANCED. |
| `id` | String | Filter ID. |
| `exclude_details` | String | Details for the filter of the type EXCLUDE. |
| `lowercase_details` | String | Details for the filter of the type LOWER. |
| `advanced_details` | String | Details for the filter of the type ADVANCED. |
| `updated` | String | Time this filter was last modified. |
| `created` | String | Time this filter was created. |
| `self_link` | String | Link for this filter. |
| `account_id` | String | Account ID to which this filter belongs. |
| `kind` | String | Resource type for Analytics filter. |
| `search_and_replace_details` | String | Details for the filter of the type SEARCH_AND_REPLACE. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create filter
filter = provider.analytics_api.Filter {
    account_id = "value"  # Account ID to create filter for.
}

# Access filter outputs
filter_id = filter.id
filter_uppercase_details = filter.uppercase_details
filter_include_details = filter.include_details
filter_name = filter.name
filter_parent_link = filter.parent_link
filter_type = filter.type
filter_id = filter.id
filter_exclude_details = filter.exclude_details
filter_lowercase_details = filter.lowercase_details
filter_advanced_details = filter.advanced_details
filter_updated = filter.updated
filter_created = filter.created
filter_self_link = filter.self_link
filter_account_id = filter.account_id
filter_kind = filter.kind
filter_search_and_replace_details = filter.search_and_replace_details
```

---


### Webpropertie

Create a new property if the account has fewer than 20 properties. Web properties are visible in the Google Analytics interface only if they have at least one profile.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_retention_ttl` | String |  | The length of time for which user and event data is retained.
This property cannot be set on insert. |
| `permissions` | String |  | Permissions the user has for this web property. |
| `level` | String |  | Level for this web property. Possible values are STANDARD or PREMIUM. |
| `child_link` | String |  | Child link for this web property. Points to the list of views (profiles) for this web property. |
| `website_url` | String |  | Website url for this web property. |
| `id` | String |  | Web property ID of the form UA-XXXXX-YY. |
| `default_profile_id` | String |  | Default view (profile) ID. |
| `kind` | String |  | Resource type for Analytics WebProperty. |
| `name` | String |  | Name of this web property. |
| `profile_count` | i64 |  | View (Profile) count for this web property. |
| `starred` | bool |  | Indicates whether this web property is starred or not. |
| `updated` | String |  | Time this web property was last modified. |
| `created` | String |  | Time this web property was created. |
| `data_retention_reset_on_new_activity` | bool |  | Set to true to reset the retention period of the user identifier with each new event from that user (thus setting the expiration date to current time plus retention period).
Set to false to delete data associated with the user identifier automatically after the rentention period.
This property cannot be set on insert. |
| `internal_web_property_id` | String |  | Internal ID for this web property. |
| `self_link` | String |  | Link for this web property. |
| `parent_link` | String |  | Parent link for this web property. Points to the account to which this web property belongs. |
| `account_id` | String |  | Account ID to which this web property belongs. |
| `industry_vertical` | String |  | The industry vertical/category selected for this web property. |
| `account_id` | String | ✅ | Account ID to create the web property for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_retention_ttl` | String | The length of time for which user and event data is retained.
This property cannot be set on insert. |
| `permissions` | String | Permissions the user has for this web property. |
| `level` | String | Level for this web property. Possible values are STANDARD or PREMIUM. |
| `child_link` | String | Child link for this web property. Points to the list of views (profiles) for this web property. |
| `website_url` | String | Website url for this web property. |
| `id` | String | Web property ID of the form UA-XXXXX-YY. |
| `default_profile_id` | String | Default view (profile) ID. |
| `kind` | String | Resource type for Analytics WebProperty. |
| `name` | String | Name of this web property. |
| `profile_count` | i64 | View (Profile) count for this web property. |
| `starred` | bool | Indicates whether this web property is starred or not. |
| `updated` | String | Time this web property was last modified. |
| `created` | String | Time this web property was created. |
| `data_retention_reset_on_new_activity` | bool | Set to true to reset the retention period of the user identifier with each new event from that user (thus setting the expiration date to current time plus retention period).
Set to false to delete data associated with the user identifier automatically after the rentention period.
This property cannot be set on insert. |
| `internal_web_property_id` | String | Internal ID for this web property. |
| `self_link` | String | Link for this web property. |
| `parent_link` | String | Parent link for this web property. Points to the account to which this web property belongs. |
| `account_id` | String | Account ID to which this web property belongs. |
| `industry_vertical` | String | The industry vertical/category selected for this web property. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create webpropertie
webpropertie = provider.analytics_api.Webpropertie {
    account_id = "value"  # Account ID to create the web property for.
}

# Access webpropertie outputs
webpropertie_id = webpropertie.id
webpropertie_data_retention_ttl = webpropertie.data_retention_ttl
webpropertie_permissions = webpropertie.permissions
webpropertie_level = webpropertie.level
webpropertie_child_link = webpropertie.child_link
webpropertie_website_url = webpropertie.website_url
webpropertie_id = webpropertie.id
webpropertie_default_profile_id = webpropertie.default_profile_id
webpropertie_kind = webpropertie.kind
webpropertie_name = webpropertie.name
webpropertie_profile_count = webpropertie.profile_count
webpropertie_starred = webpropertie.starred
webpropertie_updated = webpropertie.updated
webpropertie_created = webpropertie.created
webpropertie_data_retention_reset_on_new_activity = webpropertie.data_retention_reset_on_new_activity
webpropertie_internal_web_property_id = webpropertie.internal_web_property_id
webpropertie_self_link = webpropertie.self_link
webpropertie_parent_link = webpropertie.parent_link
webpropertie_account_id = webpropertie.account_id
webpropertie_industry_vertical = webpropertie.industry_vertical
```

---


### Custom_dimension

Create a new custom dimension.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Kind value for a custom dimension. Set to "analytics#customDimension". It is a read-only field. |
| `parent_link` | String |  | Parent link for the custom dimension. Points to the property to which the custom dimension belongs. |
| `account_id` | String |  | Account ID. |
| `index` | i64 |  | Index of the custom dimension. |
| `active` | bool |  | Boolean indicating whether the custom dimension is active. |
| `scope` | String |  | Scope of the custom dimension: HIT, SESSION, USER or PRODUCT. |
| `self_link` | String |  | Link for the custom dimension |
| `updated` | String |  | Time the custom dimension was last modified. |
| `name` | String |  | Name of the custom dimension. |
| `created` | String |  | Time the custom dimension was created. |
| `web_property_id` | String |  | Property ID. |
| `id` | String |  | Custom dimension ID. |
| `account_id` | String | ✅ | Account ID for the custom dimension to create. |
| `web_property_id` | String | ✅ | Web property ID for the custom dimension to create. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind value for a custom dimension. Set to "analytics#customDimension". It is a read-only field. |
| `parent_link` | String | Parent link for the custom dimension. Points to the property to which the custom dimension belongs. |
| `account_id` | String | Account ID. |
| `index` | i64 | Index of the custom dimension. |
| `active` | bool | Boolean indicating whether the custom dimension is active. |
| `scope` | String | Scope of the custom dimension: HIT, SESSION, USER or PRODUCT. |
| `self_link` | String | Link for the custom dimension |
| `updated` | String | Time the custom dimension was last modified. |
| `name` | String | Name of the custom dimension. |
| `created` | String | Time the custom dimension was created. |
| `web_property_id` | String | Property ID. |
| `id` | String | Custom dimension ID. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_dimension
custom_dimension = provider.analytics_api.Custom_dimension {
    account_id = "value"  # Account ID for the custom dimension to create.
    web_property_id = "value"  # Web property ID for the custom dimension to create.
}

# Access custom_dimension outputs
custom_dimension_id = custom_dimension.id
custom_dimension_kind = custom_dimension.kind
custom_dimension_parent_link = custom_dimension.parent_link
custom_dimension_account_id = custom_dimension.account_id
custom_dimension_index = custom_dimension.index
custom_dimension_active = custom_dimension.active
custom_dimension_scope = custom_dimension.scope
custom_dimension_self_link = custom_dimension.self_link
custom_dimension_updated = custom_dimension.updated
custom_dimension_name = custom_dimension.name
custom_dimension_created = custom_dimension.created
custom_dimension_web_property_id = custom_dimension.web_property_id
custom_dimension_id = custom_dimension.id
```

---


### Custom_data_source

List custom data sources to which the user has access.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `username` | String | Email ID of the authenticated user |
| `previous_link` | String | Link to previous page for this custom data source collection. |
| `total_results` | i64 | The total number of results for the query, regardless of the number of results in the response. |
| `kind` | String | Collection type. |
| `items` | Vec<String> | Collection of custom data sources. |
| `start_index` | i64 | The starting index of the resources, which is 1 by default or otherwise specified by the start-index query parameter. |
| `items_per_page` | i64 | The maximum number of resources the response can contain, regardless of the actual number of resources returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter. |
| `next_link` | String | Link to next page for this custom data source collection. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access custom_data_source outputs
custom_data_source_id = custom_data_source.id
custom_data_source_username = custom_data_source.username
custom_data_source_previous_link = custom_data_source.previous_link
custom_data_source_total_results = custom_data_source.total_results
custom_data_source_kind = custom_data_source.kind
custom_data_source_items = custom_data_source.items
custom_data_source_start_index = custom_data_source.start_index
custom_data_source_items_per_page = custom_data_source.items_per_page
custom_data_source_next_link = custom_data_source.next_link
```

---


### Column

Lists all columns for a report type

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | List of columns for a report type. |
| `kind` | String | Collection type. |
| `etag` | String | Etag of collection. This etag can be compared with the last response etag to check if response has changed. |
| `attribute_names` | Vec<String> | List of attributes names returned by columns. |
| `total_results` | i64 | Total number of columns returned in the response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access column outputs
column_id = column.id
column_items = column.items
column_kind = column.kind
column_etag = column.etag
column_attribute_names = column.attribute_names
column_total_results = column.total_results
```

---


### Client_id

Hashes the given Client ID.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  |  |
| `web_property_id` | String |  |  |
| `client_id` | String |  |  |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create client_id
client_id = provider.analytics_api.Client_id {
}

```

---


### Remarketing_audience

Creates a new remarketing audience.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `audience_definition` | String |  | The simple audience definition that will cause a user to be added to an audience. |
| `created` | String |  | Time this remarketing audience was created. |
| `account_id` | String |  | Account ID to which this remarketing audience belongs. |
| `state_based_audience_definition` | String |  | A state based audience definition that will cause a user to be added or removed from an audience. |
| `id` | String |  | Remarketing Audience ID. |
| `updated` | String |  | Time this remarketing audience was last modified. |
| `web_property_id` | String |  | Web property ID of the form UA-XXXXX-YY to which this remarketing audience belongs. |
| `linked_ad_accounts` | Vec<String> |  | The linked ad accounts associated with this remarketing audience. A remarketing audience can have only one linkedAdAccount currently. |
| `audience_type` | String |  | The type of audience, either SIMPLE or STATE_BASED. |
| `internal_web_property_id` | String |  | Internal ID for the web property to which this remarketing audience belongs. |
| `description` | String |  | The description of this remarketing audience. |
| `kind` | String |  | Collection type. |
| `linked_views` | Vec<String> |  | The views (profiles) that this remarketing audience is linked to. |
| `name` | String |  | The name of this remarketing audience. |
| `account_id` | String | ✅ | The account ID for which to create the remarketing audience. |
| `web_property_id` | String | ✅ | Web property ID for which to create the remarketing audience. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audience_definition` | String | The simple audience definition that will cause a user to be added to an audience. |
| `created` | String | Time this remarketing audience was created. |
| `account_id` | String | Account ID to which this remarketing audience belongs. |
| `state_based_audience_definition` | String | A state based audience definition that will cause a user to be added or removed from an audience. |
| `id` | String | Remarketing Audience ID. |
| `updated` | String | Time this remarketing audience was last modified. |
| `web_property_id` | String | Web property ID of the form UA-XXXXX-YY to which this remarketing audience belongs. |
| `linked_ad_accounts` | Vec<String> | The linked ad accounts associated with this remarketing audience. A remarketing audience can have only one linkedAdAccount currently. |
| `audience_type` | String | The type of audience, either SIMPLE or STATE_BASED. |
| `internal_web_property_id` | String | Internal ID for the web property to which this remarketing audience belongs. |
| `description` | String | The description of this remarketing audience. |
| `kind` | String | Collection type. |
| `linked_views` | Vec<String> | The views (profiles) that this remarketing audience is linked to. |
| `name` | String | The name of this remarketing audience. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create remarketing_audience
remarketing_audience = provider.analytics_api.Remarketing_audience {
    account_id = "value"  # The account ID for which to create the remarketing audience.
    web_property_id = "value"  # Web property ID for which to create the remarketing audience.
}

# Access remarketing_audience outputs
remarketing_audience_id = remarketing_audience.id
remarketing_audience_audience_definition = remarketing_audience.audience_definition
remarketing_audience_created = remarketing_audience.created
remarketing_audience_account_id = remarketing_audience.account_id
remarketing_audience_state_based_audience_definition = remarketing_audience.state_based_audience_definition
remarketing_audience_id = remarketing_audience.id
remarketing_audience_updated = remarketing_audience.updated
remarketing_audience_web_property_id = remarketing_audience.web_property_id
remarketing_audience_linked_ad_accounts = remarketing_audience.linked_ad_accounts
remarketing_audience_audience_type = remarketing_audience.audience_type
remarketing_audience_internal_web_property_id = remarketing_audience.internal_web_property_id
remarketing_audience_description = remarketing_audience.description
remarketing_audience_kind = remarketing_audience.kind
remarketing_audience_linked_views = remarketing_audience.linked_views
remarketing_audience_name = remarketing_audience.name
```

---


### Webproperty_user_link

Adds a new user to the given web property.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_ref` | String |  | User reference. |
| `self_link` | String |  | Self link for this resource. |
| `entity` | String |  | Entity for this link. It can be an account, a web property, or a view (profile). |
| `id` | String |  | Entity user link ID |
| `permissions` | String |  | Permissions the user has for this entity. |
| `kind` | String |  | Resource type for entity user link. |
| `account_id` | String | ✅ | Account ID to create the user link for. |
| `web_property_id` | String | ✅ | Web Property ID to create the user link for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of entity user links. |
| `next_link` | String | Next link for this account collection. |
| `items_per_page` | i64 | The maximum number of entries the response can contain, regardless of the actual number of entries returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter. |
| `kind` | String | Collection type. |
| `previous_link` | String | Previous link for this account collection. |
| `total_results` | i64 | The total number of results for the query, regardless of the number of results in the response. |
| `start_index` | i64 | The starting index of the entries, which is 1 by default or otherwise specified by the start-index query parameter. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create webproperty_user_link
webproperty_user_link = provider.analytics_api.Webproperty_user_link {
    account_id = "value"  # Account ID to create the user link for.
    web_property_id = "value"  # Web Property ID to create the user link for.
}

# Access webproperty_user_link outputs
webproperty_user_link_id = webproperty_user_link.id
webproperty_user_link_items = webproperty_user_link.items
webproperty_user_link_next_link = webproperty_user_link.next_link
webproperty_user_link_items_per_page = webproperty_user_link.items_per_page
webproperty_user_link_kind = webproperty_user_link.kind
webproperty_user_link_previous_link = webproperty_user_link.previous_link
webproperty_user_link_total_results = webproperty_user_link.total_results
webproperty_user_link_start_index = webproperty_user_link.start_index
```

---


### Custom_metric

Create a new custom metric.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `index` | i64 |  | Index of the custom metric. |
| `kind` | String |  | Kind value for a custom metric. Set to "analytics#customMetric". It is a read-only field. |
| `self_link` | String |  | Link for the custom metric |
| `created` | String |  | Time the custom metric was created. |
| `type` | String |  | Data type of custom metric. |
| `updated` | String |  | Time the custom metric was last modified. |
| `web_property_id` | String |  | Property ID. |
| `name` | String |  | Name of the custom metric. |
| `max_value` | String |  | Max value of custom metric. |
| `account_id` | String |  | Account ID. |
| `id` | String |  | Custom metric ID. |
| `scope` | String |  | Scope of the custom metric: HIT or PRODUCT. |
| `min_value` | String |  | Min value of custom metric. |
| `active` | bool |  | Boolean indicating whether the custom metric is active. |
| `parent_link` | String |  | Parent link for the custom metric. Points to the property to which the custom metric belongs. |
| `web_property_id` | String | ✅ | Web property ID for the custom dimension to create. |
| `account_id` | String | ✅ | Account ID for the custom metric to create. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `index` | i64 | Index of the custom metric. |
| `kind` | String | Kind value for a custom metric. Set to "analytics#customMetric". It is a read-only field. |
| `self_link` | String | Link for the custom metric |
| `created` | String | Time the custom metric was created. |
| `type` | String | Data type of custom metric. |
| `updated` | String | Time the custom metric was last modified. |
| `web_property_id` | String | Property ID. |
| `name` | String | Name of the custom metric. |
| `max_value` | String | Max value of custom metric. |
| `account_id` | String | Account ID. |
| `id` | String | Custom metric ID. |
| `scope` | String | Scope of the custom metric: HIT or PRODUCT. |
| `min_value` | String | Min value of custom metric. |
| `active` | bool | Boolean indicating whether the custom metric is active. |
| `parent_link` | String | Parent link for the custom metric. Points to the property to which the custom metric belongs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_metric
custom_metric = provider.analytics_api.Custom_metric {
    web_property_id = "value"  # Web property ID for the custom dimension to create.
    account_id = "value"  # Account ID for the custom metric to create.
}

# Access custom_metric outputs
custom_metric_id = custom_metric.id
custom_metric_index = custom_metric.index
custom_metric_kind = custom_metric.kind
custom_metric_self_link = custom_metric.self_link
custom_metric_created = custom_metric.created
custom_metric_type = custom_metric.type
custom_metric_updated = custom_metric.updated
custom_metric_web_property_id = custom_metric.web_property_id
custom_metric_name = custom_metric.name
custom_metric_max_value = custom_metric.max_value
custom_metric_account_id = custom_metric.account_id
custom_metric_id = custom_metric.id
custom_metric_scope = custom_metric.scope
custom_metric_min_value = custom_metric.min_value
custom_metric_active = custom_metric.active
custom_metric_parent_link = custom_metric.parent_link
```

---


### User_deletion_request

Insert or update a user deletion requests.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deletion_request_time` | String |  | This marks the point in time for which all user data before should be deleted |
| `kind` | String |  | Value is "analytics#userDeletionRequest". |
| `firebase_project_id` | String |  | Firebase Project Id |
| `property_id` | String |  | Property ID |
| `web_property_id` | String |  | Web property ID of the form UA-XXXXX-YY. |
| `id` | String |  | User ID. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_deletion_request
user_deletion_request = provider.analytics_api.User_deletion_request {
}

```

---


### Web_property_ad_words_link

Creates a webProperty-Google Ads link.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ad_words_accounts` | Vec<String> |  | A list of Google Ads client accounts. These cannot be MCC accounts. This field is required when creating a Google Ads link. It cannot be empty. |
| `profile_ids` | Vec<String> |  | IDs of linked Views (Profiles) represented as strings. |
| `self_link` | String |  | URL link for this Google Analytics - Google Ads link. |
| `id` | String |  | Entity Google Ads link ID |
| `entity` | String |  | Web property being linked. |
| `kind` | String |  | Resource type for entity Google Ads link. |
| `name` | String |  | Name of the link. This field is required when creating a Google Ads link. |
| `account_id` | String | ✅ | ID of the Google Analytics account to create the link for. |
| `web_property_id` | String | ✅ | Web property ID to create the link for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ad_words_accounts` | Vec<String> | A list of Google Ads client accounts. These cannot be MCC accounts. This field is required when creating a Google Ads link. It cannot be empty. |
| `profile_ids` | Vec<String> | IDs of linked Views (Profiles) represented as strings. |
| `self_link` | String | URL link for this Google Analytics - Google Ads link. |
| `id` | String | Entity Google Ads link ID |
| `entity` | String | Web property being linked. |
| `kind` | String | Resource type for entity Google Ads link. |
| `name` | String | Name of the link. This field is required when creating a Google Ads link. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create web_property_ad_words_link
web_property_ad_words_link = provider.analytics_api.Web_property_ad_words_link {
    account_id = "value"  # ID of the Google Analytics account to create the link for.
    web_property_id = "value"  # Web property ID to create the link for.
}

# Access web_property_ad_words_link outputs
web_property_ad_words_link_id = web_property_ad_words_link.id
web_property_ad_words_link_ad_words_accounts = web_property_ad_words_link.ad_words_accounts
web_property_ad_words_link_profile_ids = web_property_ad_words_link.profile_ids
web_property_ad_words_link_self_link = web_property_ad_words_link.self_link
web_property_ad_words_link_id = web_property_ad_words_link.id
web_property_ad_words_link_entity = web_property_ad_words_link.entity
web_property_ad_words_link_kind = web_property_ad_words_link.kind
web_property_ad_words_link_name = web_property_ad_words_link.name
```

---


### Ga

Returns Analytics data for a view (profile).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `totals_for_all_results` | HashMap<String, String> | Total values for the requested metrics over all the results, not just the results returned in this response. The order of the metric totals is same as the metric order specified in the request. |
| `items_per_page` | i64 | The maximum number of rows the response can contain, regardless of the actual number of rows returned. Its value ranges from 1 to 10,000 with a value of 1000 by default, or otherwise specified by the max-results query parameter. |
| `id` | String | Unique ID for this data response. |
| `contains_sampled_data` | bool | Determines if Analytics data contains samples. |
| `data_last_refreshed` | String | The last refreshed time in seconds for Analytics data. |
| `previous_link` | String | Link to previous page for this Analytics data query. |
| `next_link` | String | Link to next page for this Analytics data query. |
| `total_results` | i64 | The total number of rows for the query, regardless of the number of rows in the response. |
| `profile_info` | String | Information for the view (profile), for which the Analytics data was requested. |
| `data_table` | String |  |
| `sample_size` | String | The number of samples used to calculate the result. |
| `query` | String | Analytics data request query parameters. |
| `kind` | String | Resource type. |
| `rows` | Vec<Vec<String>> | Analytics data rows, where each row contains a list of dimension values followed by the metric values. The order of dimensions and metrics is same as specified in the request. |
| `column_headers` | Vec<String> | Column headers that list dimension names followed by the metric names. The order of dimensions and metrics is same as specified in the request. |
| `self_link` | String | Link to this page. |
| `sample_space` | String | Total size of the sample space from which the samples were selected. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access ga outputs
ga_id = ga.id
ga_totals_for_all_results = ga.totals_for_all_results
ga_items_per_page = ga.items_per_page
ga_id = ga.id
ga_contains_sampled_data = ga.contains_sampled_data
ga_data_last_refreshed = ga.data_last_refreshed
ga_previous_link = ga.previous_link
ga_next_link = ga.next_link
ga_total_results = ga.total_results
ga_profile_info = ga.profile_info
ga_data_table = ga.data_table
ga_sample_size = ga.sample_size
ga_query = ga.query
ga_kind = ga.kind
ga_rows = ga.rows
ga_column_headers = ga.column_headers
ga_self_link = ga.self_link
ga_sample_space = ga.sample_space
```

---


### Account_user_link

Adds a new user to the given account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_ref` | String |  | User reference. |
| `self_link` | String |  | Self link for this resource. |
| `entity` | String |  | Entity for this link. It can be an account, a web property, or a view (profile). |
| `id` | String |  | Entity user link ID |
| `permissions` | String |  | Permissions the user has for this entity. |
| `kind` | String |  | Resource type for entity user link. |
| `account_id` | String | ✅ | Account ID to create the user link for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of entity user links. |
| `next_link` | String | Next link for this account collection. |
| `items_per_page` | i64 | The maximum number of entries the response can contain, regardless of the actual number of entries returned. Its value ranges from 1 to 1000 with a value of 1000 by default, or otherwise specified by the max-results query parameter. |
| `kind` | String | Collection type. |
| `previous_link` | String | Previous link for this account collection. |
| `total_results` | i64 | The total number of results for the query, regardless of the number of results in the response. |
| `start_index` | i64 | The starting index of the entries, which is 1 by default or otherwise specified by the start-index query parameter. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account_user_link
account_user_link = provider.analytics_api.Account_user_link {
    account_id = "value"  # Account ID to create the user link for.
}

# Access account_user_link outputs
account_user_link_id = account_user_link.id
account_user_link_items = account_user_link.items
account_user_link_next_link = account_user_link.next_link
account_user_link_items_per_page = account_user_link.items_per_page
account_user_link_kind = account_user_link.kind
account_user_link_previous_link = account_user_link.previous_link
account_user_link_total_results = account_user_link.total_results
account_user_link_start_index = account_user_link.start_index
```

---


### Profile

Create a new view (profile).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bot_filtering_enabled` | bool |  | Indicates whether bot filtering is enabled for this view (profile). |
| `site_search_category_parameters` | String |  | Site search category parameters for this view (profile). |
| `currency` | String |  | The currency type associated with this view (profile), defaults to USD. The supported values are:
USD, JPY, EUR, GBP, AUD, KRW, BRL, CNY, DKK, RUB, SEK, NOK, PLN, TRY, TWD, HKD, THB, IDR, ARS, MXN, VND, PHP, INR, CHF, CAD, CZK, NZD, HUF, BGN, LTL, ZAR, UAH, AED, BOB, CLP, COP, EGP, HRK, ILS, MAD, MYR, PEN, PKR, RON, RSD, SAR, SGD, VEF, LVL |
| `strip_site_search_category_parameters` | bool |  | Whether or not Analytics will strip search category parameters from the URLs in your reports. |
| `strip_site_search_query_parameters` | bool |  | Whether or not Analytics will strip search query parameters from the URLs in your reports. |
| `timezone` | String |  | Time zone for which this view (profile) has been configured. Time zones are identified by strings from the TZ database. |
| `updated` | String |  | Time this view (profile) was last modified. |
| `id` | String |  | View (Profile) ID. |
| `web_property_id` | String |  | Web property ID of the form UA-XXXXX-YY to which this view (profile) belongs. |
| `child_link` | String |  | Child link for this view (profile). Points to the list of goals for this view (profile). |
| `default_page` | String |  | Default page for this view (profile). |
| `enhanced_e_commerce_tracking` | bool |  | Indicates whether enhanced ecommerce tracking is enabled for this view (profile). This property can only be enabled if ecommerce tracking is enabled. |
| `self_link` | String |  | Link for this view (profile). |
| `type` | String |  | View (Profile) type. Supported types: WEB or APP. |
| `e_commerce_tracking` | bool |  | Indicates whether ecommerce tracking is enabled for this view (profile). |
| `parent_link` | String |  | Parent link for this view (profile). Points to the web property to which this view (profile) belongs. |
| `site_search_query_parameters` | String |  | The site search query parameters for this view (profile). |
| `website_url` | String |  | Website URL for this view (profile). |
| `name` | String |  | Name of this view (profile). |
| `account_id` | String |  | Account ID to which this view (profile) belongs. |
| `internal_web_property_id` | String |  | Internal ID for the web property to which this view (profile) belongs. |
| `permissions` | String |  | Permissions the user has for this view (profile). |
| `starred` | bool |  | Indicates whether this view (profile) is starred or not. |
| `created` | String |  | Time this view (profile) was created. |
| `kind` | String |  | Resource type for Analytics view (profile). |
| `exclude_query_parameters` | String |  | The query parameters that are excluded from this view (profile). |
| `web_property_id` | String | ✅ | Web property ID to create the view (profile) for. |
| `account_id` | String | ✅ | Account ID to create the view (profile) for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bot_filtering_enabled` | bool | Indicates whether bot filtering is enabled for this view (profile). |
| `site_search_category_parameters` | String | Site search category parameters for this view (profile). |
| `currency` | String | The currency type associated with this view (profile), defaults to USD. The supported values are:
USD, JPY, EUR, GBP, AUD, KRW, BRL, CNY, DKK, RUB, SEK, NOK, PLN, TRY, TWD, HKD, THB, IDR, ARS, MXN, VND, PHP, INR, CHF, CAD, CZK, NZD, HUF, BGN, LTL, ZAR, UAH, AED, BOB, CLP, COP, EGP, HRK, ILS, MAD, MYR, PEN, PKR, RON, RSD, SAR, SGD, VEF, LVL |
| `strip_site_search_category_parameters` | bool | Whether or not Analytics will strip search category parameters from the URLs in your reports. |
| `strip_site_search_query_parameters` | bool | Whether or not Analytics will strip search query parameters from the URLs in your reports. |
| `timezone` | String | Time zone for which this view (profile) has been configured. Time zones are identified by strings from the TZ database. |
| `updated` | String | Time this view (profile) was last modified. |
| `id` | String | View (Profile) ID. |
| `web_property_id` | String | Web property ID of the form UA-XXXXX-YY to which this view (profile) belongs. |
| `child_link` | String | Child link for this view (profile). Points to the list of goals for this view (profile). |
| `default_page` | String | Default page for this view (profile). |
| `enhanced_e_commerce_tracking` | bool | Indicates whether enhanced ecommerce tracking is enabled for this view (profile). This property can only be enabled if ecommerce tracking is enabled. |
| `self_link` | String | Link for this view (profile). |
| `type` | String | View (Profile) type. Supported types: WEB or APP. |
| `e_commerce_tracking` | bool | Indicates whether ecommerce tracking is enabled for this view (profile). |
| `parent_link` | String | Parent link for this view (profile). Points to the web property to which this view (profile) belongs. |
| `site_search_query_parameters` | String | The site search query parameters for this view (profile). |
| `website_url` | String | Website URL for this view (profile). |
| `name` | String | Name of this view (profile). |
| `account_id` | String | Account ID to which this view (profile) belongs. |
| `internal_web_property_id` | String | Internal ID for the web property to which this view (profile) belongs. |
| `permissions` | String | Permissions the user has for this view (profile). |
| `starred` | bool | Indicates whether this view (profile) is starred or not. |
| `created` | String | Time this view (profile) was created. |
| `kind` | String | Resource type for Analytics view (profile). |
| `exclude_query_parameters` | String | The query parameters that are excluded from this view (profile). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create profile
profile = provider.analytics_api.Profile {
    web_property_id = "value"  # Web property ID to create the view (profile) for.
    account_id = "value"  # Account ID to create the view (profile) for.
}

# Access profile outputs
profile_id = profile.id
profile_bot_filtering_enabled = profile.bot_filtering_enabled
profile_site_search_category_parameters = profile.site_search_category_parameters
profile_currency = profile.currency
profile_strip_site_search_category_parameters = profile.strip_site_search_category_parameters
profile_strip_site_search_query_parameters = profile.strip_site_search_query_parameters
profile_timezone = profile.timezone
profile_updated = profile.updated
profile_id = profile.id
profile_web_property_id = profile.web_property_id
profile_child_link = profile.child_link
profile_default_page = profile.default_page
profile_enhanced_e_commerce_tracking = profile.enhanced_e_commerce_tracking
profile_self_link = profile.self_link
profile_type = profile.type
profile_e_commerce_tracking = profile.e_commerce_tracking
profile_parent_link = profile.parent_link
profile_site_search_query_parameters = profile.site_search_query_parameters
profile_website_url = profile.website_url
profile_name = profile.name
profile_account_id = profile.account_id
profile_internal_web_property_id = profile.internal_web_property_id
profile_permissions = profile.permissions
profile_starred = profile.starred
profile_created = profile.created
profile_kind = profile.kind
profile_exclude_query_parameters = profile.exclude_query_parameters
```

---


### Profile

Lists views (profiles) to which the user has access.

**Operations**: ✅ Read

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

# Access profile outputs
profile_id = profile.id
```

---


### Data

Returns Analytics report data for a view (profile).

**Operations**: ✅ Read

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

# Access data outputs
data_id = data.id
```

---


### Goal

Lists goals to which the user has access.

**Operations**: ✅ Read

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

# Access goal outputs
goal_id = goal.id
```

---


### Account

Lists all accounts to which the user has access.

**Operations**: ✅ Read

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

# Access account outputs
account_id = account.id
```

---


### Segment

Lists advanced segments to which the user has access.

**Operations**: ✅ Read

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

# Access segment outputs
segment_id = segment.id
```

---


### Webpropertie

Lists web properties to which the user has access.

**Operations**: ✅ Read

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

# Access webpropertie outputs
webpropertie_id = webpropertie.id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple experiment resources
experiment_0 = provider.analytics_api.Experiment {
    account_id = "value-0"
    profile_id = "value-0"
    web_property_id = "value-0"
}
experiment_1 = provider.analytics_api.Experiment {
    account_id = "value-1"
    profile_id = "value-1"
    web_property_id = "value-1"
}
experiment_2 = provider.analytics_api.Experiment {
    account_id = "value-2"
    profile_id = "value-2"
    web_property_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    experiment = provider.analytics_api.Experiment {
        account_id = "production-value"
        profile_id = "production-value"
        web_property_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Analytics_api Documentation](https://cloud.google.com/analytics_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
