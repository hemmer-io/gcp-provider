# Toolresults_api Service



**Resources**: 11

---

## Overview

The toolresults_api service provides access to 11 resource types:

- [Perf_sample_serie](#perf_sample_serie) [CR]
- [Execution](#execution) [CRU]
- [Sample](#sample) [CR]
- [Perf_metrics_summary](#perf_metrics_summary) [C]
- [Project](#project) [CR]
- [Cluster](#cluster) [R]
- [Environment](#environment) [R]
- [Historie](#historie) [CR]
- [Thumbnail](#thumbnail) [R]
- [Test_case](#test_case) [R]
- [Step](#step) [CRU]

---

## Resources


### Perf_sample_serie

Creates a PerfSampleSeries. May return any of the following error code(s): - ALREADY_EXISTS - PerfMetricSummary already exists for the given Step - NOT_FOUND - The containing Step does not exist

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_id` | String |  | The cloud project @OutputOnly |
| `basic_perf_sample_series` | String |  | Basic series represented by a line chart |
| `execution_id` | String |  | A tool results execution ID. @OutputOnly |
| `sample_series_id` | String |  | A sample series id @OutputOnly |
| `history_id` | String |  | A tool results history ID. @OutputOnly |
| `step_id` | String |  | A tool results step ID. @OutputOnly |
| `execution_id` | String | ✅ | A tool results execution ID. |
| `project_id` | String | ✅ | The cloud project |
| `history_id` | String | ✅ | A tool results history ID. |
| `step_id` | String | ✅ | A tool results step ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `project_id` | String | The cloud project @OutputOnly |
| `basic_perf_sample_series` | String | Basic series represented by a line chart |
| `execution_id` | String | A tool results execution ID. @OutputOnly |
| `sample_series_id` | String | A sample series id @OutputOnly |
| `history_id` | String | A tool results history ID. @OutputOnly |
| `step_id` | String | A tool results step ID. @OutputOnly |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create perf_sample_serie
perf_sample_serie = provider.toolresults_api.Perf_sample_serie {
    execution_id = "value"  # A tool results execution ID.
    project_id = "value"  # The cloud project
    history_id = "value"  # A tool results history ID.
    step_id = "value"  # A tool results step ID.
}

# Access perf_sample_serie outputs
perf_sample_serie_id = perf_sample_serie.id
perf_sample_serie_project_id = perf_sample_serie.project_id
perf_sample_serie_basic_perf_sample_series = perf_sample_serie.basic_perf_sample_series
perf_sample_serie_execution_id = perf_sample_serie.execution_id
perf_sample_serie_sample_series_id = perf_sample_serie.sample_series_id
perf_sample_serie_history_id = perf_sample_serie.history_id
perf_sample_serie_step_id = perf_sample_serie.step_id
```

---


### Execution

Creates an Execution. The returned Execution will have the id set. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing History does not exist

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dimension_definitions` | Vec<String> |  | The dimensions along which different steps in this execution may vary. This must remain fixed over the life of the execution. Returns INVALID_ARGUMENT if this field is set in an update request. Returns INVALID_ARGUMENT if the same name occurs in more than one dimension_definition. Returns INVALID_ARGUMENT if the size of the list is over 100. - In response: present if set by create - In create request: optional - In update request: never set |
| `completion_time` | String |  | The time when the Execution status transitioned to COMPLETE. This value will be set automatically when state transitions to COMPLETE. - In response: set if the execution state is COMPLETE. - In create/update request: never set |
| `state` | String |  | The initial state is IN_PROGRESS. The only legal state transitions is from IN_PROGRESS to COMPLETE. A PRECONDITION_FAILED will be returned if an invalid transition is requested. The state can only be set to COMPLETE once. A FAILED_PRECONDITION will be returned if the state is set to COMPLETE multiple times. If the state is set to COMPLETE, all the in-progress steps within the execution will be set as COMPLETE. If the outcome of the step is not set, the outcome will be set to INCONCLUSIVE. - In response always set - In create/update request: optional |
| `specification` | String |  | Lightweight information about execution request. - In response: present if set by create - In create: optional - In update: optional |
| `execution_id` | String |  | A unique identifier within a History for this Execution. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response always set - In create/update request: never set |
| `test_execution_matrix_id` | String |  | TestExecution Matrix ID that the TestExecutionService uses. - In response: present if set by create - In create: optional - In update: never set |
| `outcome` | String |  | Classify the result, for example into SUCCESS or FAILURE - In response: present if set by create/update request - In create/update request: optional |
| `creation_time` | String |  | The time when the Execution was created. This value will be set automatically when CreateExecution is called. - In response: always set - In create/update request: never set |
| `history_id` | String | ✅ | A History id. Required. |
| `project_id` | String | ✅ | A Project id. Required. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dimension_definitions` | Vec<String> | The dimensions along which different steps in this execution may vary. This must remain fixed over the life of the execution. Returns INVALID_ARGUMENT if this field is set in an update request. Returns INVALID_ARGUMENT if the same name occurs in more than one dimension_definition. Returns INVALID_ARGUMENT if the size of the list is over 100. - In response: present if set by create - In create request: optional - In update request: never set |
| `completion_time` | String | The time when the Execution status transitioned to COMPLETE. This value will be set automatically when state transitions to COMPLETE. - In response: set if the execution state is COMPLETE. - In create/update request: never set |
| `state` | String | The initial state is IN_PROGRESS. The only legal state transitions is from IN_PROGRESS to COMPLETE. A PRECONDITION_FAILED will be returned if an invalid transition is requested. The state can only be set to COMPLETE once. A FAILED_PRECONDITION will be returned if the state is set to COMPLETE multiple times. If the state is set to COMPLETE, all the in-progress steps within the execution will be set as COMPLETE. If the outcome of the step is not set, the outcome will be set to INCONCLUSIVE. - In response always set - In create/update request: optional |
| `specification` | String | Lightweight information about execution request. - In response: present if set by create - In create: optional - In update: optional |
| `execution_id` | String | A unique identifier within a History for this Execution. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response always set - In create/update request: never set |
| `test_execution_matrix_id` | String | TestExecution Matrix ID that the TestExecutionService uses. - In response: present if set by create - In create: optional - In update: never set |
| `outcome` | String | Classify the result, for example into SUCCESS or FAILURE - In response: present if set by create/update request - In create/update request: optional |
| `creation_time` | String | The time when the Execution was created. This value will be set automatically when CreateExecution is called. - In response: always set - In create/update request: never set |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create execution
execution = provider.toolresults_api.Execution {
    history_id = "value"  # A History id. Required.
    project_id = "value"  # A Project id. Required.
}

# Access execution outputs
execution_id = execution.id
execution_dimension_definitions = execution.dimension_definitions
execution_completion_time = execution.completion_time
execution_state = execution.state
execution_specification = execution.specification
execution_execution_id = execution.execution_id
execution_test_execution_matrix_id = execution.test_execution_matrix_id
execution_outcome = execution.outcome
execution_creation_time = execution.creation_time
```

---


### Sample

Creates a batch of PerfSamples - a client can submit multiple batches of Perf Samples through repeated calls to this method in order to split up a large request payload - duplicates and existing timestamp entries will be ignored. - the batch operation may partially succeed - the set of elements successfully inserted is returned in the response (omits items which already existed in the database). May return any of the following canonical error codes: - NOT_FOUND - The containing PerfSampleSeries does not exist

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `perf_samples` | Vec<String> |  | The set of PerfSamples to create should not include existing timestamps |
| `project_id` | String | ✅ | The cloud project |
| `sample_series_id` | String | ✅ | A sample series id |
| `execution_id` | String | ✅ | A tool results execution ID. |
| `history_id` | String | ✅ | A tool results history ID. |
| `step_id` | String | ✅ | A tool results step ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `perf_samples` | Vec<String> |  |
| `next_page_token` | String | Optional, returned if result size exceeds the page size specified in the request (or the default page size, 500, if unspecified). It indicates the last sample timestamp to be used as page_token in subsequent request |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sample
sample = provider.toolresults_api.Sample {
    project_id = "value"  # The cloud project
    sample_series_id = "value"  # A sample series id
    execution_id = "value"  # A tool results execution ID.
    history_id = "value"  # A tool results history ID.
    step_id = "value"  # A tool results step ID.
}

# Access sample outputs
sample_id = sample.id
sample_perf_samples = sample.perf_samples
sample_next_page_token = sample.next_page_token
```

---


### Perf_metrics_summary

Creates a PerfMetricsSummary resource. Returns the existing one if it has already been created. May return any of the following error code(s): - NOT_FOUND - The containing Step does not exist

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `execution_id` | String |  | A tool results execution ID. @OutputOnly |
| `perf_environment` | String |  | Describes the environment in which the performance metrics were collected |
| `graphics_stats` | String |  | Graphics statistics for the entire run. Statistics are reset at the beginning of the run and collected at the end of the run. |
| `perf_metrics` | Vec<String> |  | Set of resource collected |
| `step_id` | String |  | A tool results step ID. @OutputOnly |
| `history_id` | String |  | A tool results history ID. @OutputOnly |
| `app_start_time` | String |  |  |
| `project_id` | String |  | The cloud project @OutputOnly |
| `project_id` | String | ✅ | The cloud project |
| `step_id` | String | ✅ | A tool results step ID. |
| `history_id` | String | ✅ | A tool results history ID. |
| `execution_id` | String | ✅ | A tool results execution ID. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create perf_metrics_summary
perf_metrics_summary = provider.toolresults_api.Perf_metrics_summary {
    project_id = "value"  # The cloud project
    step_id = "value"  # A tool results step ID.
    history_id = "value"  # A tool results history ID.
    execution_id = "value"  # A tool results execution ID.
}

```

---


### Project

Creates resources for settings which have not yet been set. Currently, this creates a single resource: a Google Cloud Storage bucket, to be used as the default bucket for this project. The bucket is created in an FTL-own storage project. Except for in rare cases, calling this method in parallel from multiple clients will only create a single bucket. In order to avoid unnecessary storage charges, the bucket is configured to automatically delete objects older than 90 days. The bucket is created with the following permissions: - Owner access for owners of central storage project (FTL-owned) - Writer access for owners/editors of customer project - Reader access for viewers of customer project The default ACL on objects created in the bucket is: - Owner access for owners of central storage project - Reader access for owners/editors/viewers of customer project See Google Cloud Storage documentation for more details. If there is already a default bucket set and the project can access the bucket, this call does nothing. However, if the project doesn't have the permission to access the bucket or the bucket is deleted, a new bucket will be created. May return any canonical error codes, including the following: - PERMISSION_DENIED - if the user is not authorized to write to project - Any error code raised by Google Cloud Storage

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_id` | String | ✅ | A Project id. Required. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_bucket` | String | The name of the Google Cloud Storage bucket to which results are written. By default, this is unset. In update request: optional In response: optional |
| `name` | String | The name of the project's settings. Always of the form: projects/{project-id}/settings In update request: never set In response: always set |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.toolresults_api.Project {
    project_id = "value"  # A Project id. Required.
}

# Access project outputs
project_id = project.id
project_default_bucket = project.default_bucket
project_name = project.name
```

---


### Cluster

Retrieves a single screenshot cluster by its ID

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `activity` | String | A string that describes the activity of every screen in the cluster. |
| `cluster_id` | String | A unique identifier for the cluster. @OutputOnly |
| `screens` | Vec<String> | Full list of screens. |
| `key_screen` | String | A singular screen that represents the cluster as a whole. This screen will act as the "cover" of the entire cluster. When users look at the clusters, only the key screen from each cluster will be shown. Which screen is the key screen is determined by the ClusteringAlgorithm |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access cluster outputs
cluster_id = cluster.id
cluster_activity = cluster.activity
cluster_cluster_id = cluster.cluster_id
cluster_screens = cluster.screens
cluster_key_screen = cluster.key_screen
```

---


### Environment

Gets an Environment. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Environment does not exist

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | Output only. The time when the Environment was created. |
| `completion_time` | String | Output only. The time when the Environment status was set to complete. This value will be set automatically when state transitions to COMPLETE. |
| `execution_id` | String | Output only. An Execution id. |
| `dimension_value` | Vec<String> | Dimension values describing the environment. Dimension values always consist of "Model", "Version", "Locale", and "Orientation". - In response: always set - In create request: always set - In update request: never set |
| `history_id` | String | Output only. A History id. |
| `results_storage` | String | The location where output files are stored in the user bucket. |
| `project_id` | String | Output only. A Project id. |
| `display_name` | String | A short human-readable name to display in the UI. Maximum of 100 characters. For example: Nexus 5, API 27. |
| `environment_id` | String | Output only. An Environment id. |
| `environment_result` | String | Merged result of the environment. |
| `shard_summaries` | Vec<String> | Output only. Summaries of shards. Only one shard will present unless sharding feature is enabled in TestExecutionService. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access environment outputs
environment_id = environment.id
environment_creation_time = environment.creation_time
environment_completion_time = environment.completion_time
environment_execution_id = environment.execution_id
environment_dimension_value = environment.dimension_value
environment_history_id = environment.history_id
environment_results_storage = environment.results_storage
environment_project_id = environment.project_id
environment_display_name = environment.display_name
environment_environment_id = environment.environment_id
environment_environment_result = environment.environment_result
environment_shard_summaries = environment.shard_summaries
```

---


### Historie

Creates a History. The returned History will have the id set. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing project does not exist

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | A name to uniquely identify a history within a project. Maximum of 200 characters. - In response always set - In create request: always set |
| `test_platform` | String |  | The platform of the test history. - In response: always set. Returns the platform of the last execution if unknown. |
| `display_name` | String |  | A short human-readable (plain text) name to display in the UI. Maximum of 100 characters. - In response: present if set during create. - In create request: optional |
| `history_id` | String |  | A unique identifier within a project for this History. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response always set - In create request: never set |
| `project_id` | String | ✅ | A Project id. Required. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | A name to uniquely identify a history within a project. Maximum of 200 characters. - In response always set - In create request: always set |
| `test_platform` | String | The platform of the test history. - In response: always set. Returns the platform of the last execution if unknown. |
| `display_name` | String | A short human-readable (plain text) name to display in the UI. Maximum of 100 characters. - In response: present if set during create. - In create request: optional |
| `history_id` | String | A unique identifier within a project for this History. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response always set - In create request: never set |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create historie
historie = provider.toolresults_api.Historie {
    project_id = "value"  # A Project id. Required.
}

# Access historie outputs
historie_id = historie.id
historie_name = historie.name
historie_test_platform = historie.test_platform
historie_display_name = historie.display_name
historie_history_id = historie.history_id
```

---


### Thumbnail

Lists thumbnails of images attached to a step. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read from the project, or from any of the images - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the step does not exist, or if any of the images do not exist

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `thumbnails` | Vec<String> | A list of image data. Images are returned in a deterministic order; they are ordered by these factors, in order of importance: * First, by their associated test case. Images without a test case are considered greater than images with one. * Second, by their creation time. Images without a creation time are greater than images with one. * Third, by the order in which they were added to the step (by calls to CreateStep or UpdateStep). |
| `next_page_token` | String | A continuation token to resume the query at the next item. If set, indicates that there are more thumbnails to read, by calling list again with this value in the page_token field. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access thumbnail outputs
thumbnail_id = thumbnail.id
thumbnail_thumbnails = thumbnail.thumbnails
thumbnail_next_page_token = thumbnail.next_page_token
```

---


### Test_case

Gets details of a Test Case for a Step. Experimental test cases API. Still in active development. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the containing Test Case does not exist

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `skipped_message` | String | Why the test case was skipped. Present only for skipped test case |
| `test_case_reference` | String | Test case reference, e.g. name, class name and test suite name. Required. |
| `status` | String | The status of the test case. Required. |
| `elapsed_time` | String | The elapsed run time of the test case. Required. |
| `stack_traces` | Vec<String> | The stack trace details if the test case failed or encountered an error. The maximum size of the stack traces is 100KiB, beyond which the stack track will be truncated. Zero if the test case passed. |
| `tool_outputs` | Vec<String> | References to opaque files of any format output by the tool execution. @OutputOnly |
| `test_case_id` | String | A unique identifier within a Step for this Test Case. |
| `end_time` | String | The end time of the test case. |
| `start_time` | String | The start time of the test case. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access test_case outputs
test_case_id = test_case.id
test_case_skipped_message = test_case.skipped_message
test_case_test_case_reference = test_case.test_case_reference
test_case_status = test_case.status
test_case_elapsed_time = test_case.elapsed_time
test_case_stack_traces = test_case.stack_traces
test_case_tool_outputs = test_case.tool_outputs
test_case_test_case_id = test_case.test_case_id
test_case_end_time = test_case.end_time
test_case_start_time = test_case.start_time
```

---


### Step

Creates a Step. The returned Step will have the id set. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed - FAILED_PRECONDITION - if the step is too large (more than 10Mib) - NOT_FOUND - if the containing Execution does not exist

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `multi_step` | String |  | Details when multiple steps are run with the same configuration as a group. These details can be used identify which group this step is part of. It also identifies the groups 'primary step' which indexes all the group members. - In response: present if previously set. - In create request: optional, set iff this step was performed more than once. - In update request: optional |
| `step_id` | String |  | A unique identifier within a Execution for this Step. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response: always set - In create/update request: never set |
| `labels` | Vec<String> |  | Arbitrary user-supplied key/value pairs that are associated with the step. Users are responsible for managing the key namespace such that keys don't accidentally collide. An INVALID_ARGUMENT will be returned if the number of labels exceeds 100 or if the length of any of the keys or values exceeds 100 characters. - In response: always set - In create request: optional - In update request: optional; any new key/value pair will be added to the map, and any new value for an existing key will update that key's value |
| `outcome` | String |  | Classification of the result, for example into SUCCESS or FAILURE - In response: present if set by create/update request - In create/update request: optional |
| `run_duration` | String |  | How long it took for this step to run. If unset, this is set to the difference between creation_time and completion_time when the step is set to the COMPLETE state. In some cases, it is appropriate to set this value separately: For instance, if a step is created, but the operation it represents is queued for a few minutes before it executes, it would be appropriate not to include the time spent queued in its run_duration. PRECONDITION_FAILED will be returned if one attempts to set a run_duration on a step which already has this field set. - In response: present if previously set; always present on COMPLETE step - In create request: optional - In update request: optional |
| `dimension_value` | Vec<String> |  | If the execution containing this step has any dimension_definition set, then this field allows the child to specify the values of the dimensions. The keys must exactly match the dimension_definition of the execution. For example, if the execution has `dimension_definition = ['attempt', 'device']` then a step must define values for those dimensions, eg. `dimension_value = ['attempt': '1', 'device': 'Nexus 6']` If a step does not participate in one dimension of the matrix, the value for that dimension should be empty string. For example, if one of the tests is executed by a runner which does not support retries, the step could have `dimension_value = ['attempt': '', 'device': 'Nexus 6']` If the step does not participate in any dimensions of the matrix, it may leave dimension_value unset. A PRECONDITION_FAILED will be returned if any of the keys do not exist in the dimension_definition of the execution. A PRECONDITION_FAILED will be returned if another step in this execution already has the same name and dimension_value, but differs on other data fields, for example, step field is different. A PRECONDITION_FAILED will be returned if dimension_value is set, and there is a dimension_definition in the execution which is not specified as one of the keys. - In response: present if set by create - In create request: optional - In update request: never set |
| `state` | String |  | The initial state is IN_PROGRESS. The only legal state transitions are * IN_PROGRESS -> COMPLETE A PRECONDITION_FAILED will be returned if an invalid transition is requested. It is valid to create Step with a state set to COMPLETE. The state can only be set to COMPLETE once. A PRECONDITION_FAILED will be returned if the state is set to COMPLETE multiple times. - In response: always set - In create/update request: optional |
| `has_images` | bool |  | Whether any of the outputs of this step are images whose thumbnails can be fetched with ListThumbnails. - In response: always set - In create/update request: never set |
| `test_execution_step` | String |  | An execution of a test runner. |
| `tool_execution_step` | String |  | An execution of a tool (used for steps we don't explicitly support). |
| `creation_time` | String |  | The time when the step was created. - In response: always set - In create/update request: never set |
| `completion_time` | String |  | The time when the step status was set to complete. This value will be set automatically when state transitions to COMPLETE. - In response: set if the execution state is COMPLETE. - In create/update request: never set |
| `device_usage_duration` | String |  | How much the device resource is used to perform the test. This is the device usage used for billing purpose, which is different from the run_duration, for example, infrastructure failure won't be charged for device usage. PRECONDITION_FAILED will be returned if one attempts to set a device_usage on a step which already has this field set. - In response: present if previously set. - In create request: optional - In update request: optional |
| `name` | String |  | A short human-readable name to display in the UI. Maximum of 100 characters. For example: Clean build A PRECONDITION_FAILED will be returned upon creating a new step if it shares its name and dimension_value with an existing step. If two steps represent a similar action, but have different dimension values, they should share the same name. For instance, if the same set of tests is run on two different platforms, the two steps should have the same name. - In response: always set - In create request: always set - In update request: never set |
| `description` | String |  | A description of this tool For example: mvn clean package -D skipTests=true - In response: present if set by create/update request - In create/update request: optional |
| `project_id` | String | ✅ | Required. A Project id. |
| `execution_id` | String | ✅ | Required. An Execution id. |
| `history_id` | String | ✅ | Required. A History id. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `multi_step` | String | Details when multiple steps are run with the same configuration as a group. These details can be used identify which group this step is part of. It also identifies the groups 'primary step' which indexes all the group members. - In response: present if previously set. - In create request: optional, set iff this step was performed more than once. - In update request: optional |
| `step_id` | String | A unique identifier within a Execution for this Step. Returns INVALID_ARGUMENT if this field is set or overwritten by the caller. - In response: always set - In create/update request: never set |
| `labels` | Vec<String> | Arbitrary user-supplied key/value pairs that are associated with the step. Users are responsible for managing the key namespace such that keys don't accidentally collide. An INVALID_ARGUMENT will be returned if the number of labels exceeds 100 or if the length of any of the keys or values exceeds 100 characters. - In response: always set - In create request: optional - In update request: optional; any new key/value pair will be added to the map, and any new value for an existing key will update that key's value |
| `outcome` | String | Classification of the result, for example into SUCCESS or FAILURE - In response: present if set by create/update request - In create/update request: optional |
| `run_duration` | String | How long it took for this step to run. If unset, this is set to the difference between creation_time and completion_time when the step is set to the COMPLETE state. In some cases, it is appropriate to set this value separately: For instance, if a step is created, but the operation it represents is queued for a few minutes before it executes, it would be appropriate not to include the time spent queued in its run_duration. PRECONDITION_FAILED will be returned if one attempts to set a run_duration on a step which already has this field set. - In response: present if previously set; always present on COMPLETE step - In create request: optional - In update request: optional |
| `dimension_value` | Vec<String> | If the execution containing this step has any dimension_definition set, then this field allows the child to specify the values of the dimensions. The keys must exactly match the dimension_definition of the execution. For example, if the execution has `dimension_definition = ['attempt', 'device']` then a step must define values for those dimensions, eg. `dimension_value = ['attempt': '1', 'device': 'Nexus 6']` If a step does not participate in one dimension of the matrix, the value for that dimension should be empty string. For example, if one of the tests is executed by a runner which does not support retries, the step could have `dimension_value = ['attempt': '', 'device': 'Nexus 6']` If the step does not participate in any dimensions of the matrix, it may leave dimension_value unset. A PRECONDITION_FAILED will be returned if any of the keys do not exist in the dimension_definition of the execution. A PRECONDITION_FAILED will be returned if another step in this execution already has the same name and dimension_value, but differs on other data fields, for example, step field is different. A PRECONDITION_FAILED will be returned if dimension_value is set, and there is a dimension_definition in the execution which is not specified as one of the keys. - In response: present if set by create - In create request: optional - In update request: never set |
| `state` | String | The initial state is IN_PROGRESS. The only legal state transitions are * IN_PROGRESS -> COMPLETE A PRECONDITION_FAILED will be returned if an invalid transition is requested. It is valid to create Step with a state set to COMPLETE. The state can only be set to COMPLETE once. A PRECONDITION_FAILED will be returned if the state is set to COMPLETE multiple times. - In response: always set - In create/update request: optional |
| `has_images` | bool | Whether any of the outputs of this step are images whose thumbnails can be fetched with ListThumbnails. - In response: always set - In create/update request: never set |
| `test_execution_step` | String | An execution of a test runner. |
| `tool_execution_step` | String | An execution of a tool (used for steps we don't explicitly support). |
| `creation_time` | String | The time when the step was created. - In response: always set - In create/update request: never set |
| `completion_time` | String | The time when the step status was set to complete. This value will be set automatically when state transitions to COMPLETE. - In response: set if the execution state is COMPLETE. - In create/update request: never set |
| `device_usage_duration` | String | How much the device resource is used to perform the test. This is the device usage used for billing purpose, which is different from the run_duration, for example, infrastructure failure won't be charged for device usage. PRECONDITION_FAILED will be returned if one attempts to set a device_usage on a step which already has this field set. - In response: present if previously set. - In create request: optional - In update request: optional |
| `name` | String | A short human-readable name to display in the UI. Maximum of 100 characters. For example: Clean build A PRECONDITION_FAILED will be returned upon creating a new step if it shares its name and dimension_value with an existing step. If two steps represent a similar action, but have different dimension values, they should share the same name. For instance, if the same set of tests is run on two different platforms, the two steps should have the same name. - In response: always set - In create request: always set - In update request: never set |
| `description` | String | A description of this tool For example: mvn clean package -D skipTests=true - In response: present if set by create/update request - In create/update request: optional |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create step
step = provider.toolresults_api.Step {
    project_id = "value"  # Required. A Project id.
    execution_id = "value"  # Required. An Execution id.
    history_id = "value"  # Required. A History id.
}

# Access step outputs
step_id = step.id
step_multi_step = step.multi_step
step_step_id = step.step_id
step_labels = step.labels
step_outcome = step.outcome
step_run_duration = step.run_duration
step_dimension_value = step.dimension_value
step_state = step.state
step_has_images = step.has_images
step_test_execution_step = step.test_execution_step
step_tool_execution_step = step.tool_execution_step
step_creation_time = step.creation_time
step_completion_time = step.completion_time
step_device_usage_duration = step.device_usage_duration
step_name = step.name
step_description = step.description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple perf_sample_serie resources
perf_sample_serie_0 = provider.toolresults_api.Perf_sample_serie {
    execution_id = "value-0"
    project_id = "value-0"
    history_id = "value-0"
    step_id = "value-0"
}
perf_sample_serie_1 = provider.toolresults_api.Perf_sample_serie {
    execution_id = "value-1"
    project_id = "value-1"
    history_id = "value-1"
    step_id = "value-1"
}
perf_sample_serie_2 = provider.toolresults_api.Perf_sample_serie {
    execution_id = "value-2"
    project_id = "value-2"
    history_id = "value-2"
    step_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    perf_sample_serie = provider.toolresults_api.Perf_sample_serie {
        execution_id = "production-value"
        project_id = "production-value"
        history_id = "production-value"
        step_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Toolresults_api Documentation](https://cloud.google.com/toolresults_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
