# Workloadmanager_api Service



**Resources**: 9

---

## Overview

The workloadmanager_api service provides access to 9 resource types:

- [Discoveredprofile](#discoveredprofile) [R]
- [Scanned_resource](#scanned_resource) [R]
- [Operation](#operation) [CRD]
- [Evaluation](#evaluation) [CRUD]
- [Insight](#insight) [CD]
- [Result](#result) [R]
- [Location](#location) [R]
- [Execution](#execution) [CRD]
- [Rule](#rule) [R]

---

## Resources


### Discoveredprofile

List discovered workload profiles

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Output only. A token identifying a page of results the server should return |
| `unreachable` | Vec<String> | Locations that could not be reached. |
| `workload_profiles` | Vec<String> | Output only. The list of workload profiles |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access discoveredprofile outputs
discoveredprofile_id = discoveredprofile.id
discoveredprofile_next_page_token = discoveredprofile.next_page_token
discoveredprofile_unreachable = discoveredprofile.unreachable
discoveredprofile_workload_profiles = discoveredprofile.workload_profiles
```

---


### Scanned_resource

List all scanned resources for a single Execution.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `scanned_resources` | Vec<String> | All scanned resources in response |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access scanned_resource outputs
scanned_resource_id = scanned_resource.id
scanned_resource_next_page_token = scanned_resource.next_page_token
scanned_resource_scanned_resources = scanned_resource.scanned_resources
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.workloadmanager_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_done = operation.done
```

---


### Evaluation

Creates a new Evaluation in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_rules_bucket` | String |  | The Cloud Storage bucket name for custom rules. |
| `description` | String |  | Description of the Evaluation |
| `kms_key` | String |  | Optional. Immutable. Customer-managed encryption key name, in the format projects/*/locations/*/keyRings/*/cryptoKeys/*. |
| `name` | String |  | name of resource names have the form 'projects/{project_id}/locations/{location_id}/evaluations/{evaluation_id}' |
| `resource_status` | String |  | Output only. [Output only] The updated rule ids if exist. |
| `evaluation_type` | String |  | Evaluation type |
| `rule_names` | Vec<String> |  | the name of the rule |
| `rule_versions` | Vec<String> |  | Output only. [Output only] The updated rule ids if exist. |
| `big_query_destination` | String |  | Optional. BigQuery destination |
| `schedule` | String |  | crontab format schedule for scheduled evaluation, currently only support the following schedule: "0 */1 * * *", "0 */6 * * *", "0 */12 * * *", "0 0 */1 * *", "0 0 */7 * *", |
| `update_time` | String |  | Output only. [Output only] Update time stamp |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `resource_filter` | String |  | annotations as key value pairs |
| `create_time` | String |  | Output only. [Output only] Create time stamp |
| `parent` | String | ✅ | Required. The resource prefix of the evaluation location using the form: `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_rules_bucket` | String | The Cloud Storage bucket name for custom rules. |
| `description` | String | Description of the Evaluation |
| `kms_key` | String | Optional. Immutable. Customer-managed encryption key name, in the format projects/*/locations/*/keyRings/*/cryptoKeys/*. |
| `name` | String | name of resource names have the form 'projects/{project_id}/locations/{location_id}/evaluations/{evaluation_id}' |
| `resource_status` | String | Output only. [Output only] The updated rule ids if exist. |
| `evaluation_type` | String | Evaluation type |
| `rule_names` | Vec<String> | the name of the rule |
| `rule_versions` | Vec<String> | Output only. [Output only] The updated rule ids if exist. |
| `big_query_destination` | String | Optional. BigQuery destination |
| `schedule` | String | crontab format schedule for scheduled evaluation, currently only support the following schedule: "0 */1 * * *", "0 */6 * * *", "0 */12 * * *", "0 0 */1 * *", "0 0 */7 * *", |
| `update_time` | String | Output only. [Output only] Update time stamp |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `resource_filter` | String | annotations as key value pairs |
| `create_time` | String | Output only. [Output only] Create time stamp |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation
evaluation = provider.workloadmanager_api.Evaluation {
    parent = "value"  # Required. The resource prefix of the evaluation location using the form: `projects/{project_id}/locations/{location_id}`
}

# Access evaluation outputs
evaluation_id = evaluation.id
evaluation_custom_rules_bucket = evaluation.custom_rules_bucket
evaluation_description = evaluation.description
evaluation_kms_key = evaluation.kms_key
evaluation_name = evaluation.name
evaluation_resource_status = evaluation.resource_status
evaluation_evaluation_type = evaluation.evaluation_type
evaluation_rule_names = evaluation.rule_names
evaluation_rule_versions = evaluation.rule_versions
evaluation_big_query_destination = evaluation.big_query_destination
evaluation_schedule = evaluation.schedule
evaluation_update_time = evaluation.update_time
evaluation_labels = evaluation.labels
evaluation_resource_filter = evaluation.resource_filter
evaluation_create_time = evaluation.create_time
```

---


### Insight

Write the data insights to workload manager data warehouse.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `agent_version` | String |  | Optional. The agent version collected this data point. |
| `insight` | String |  | Required. The metrics data details. |
| `request_id` | String |  | Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000). |
| `location` | String | ✅ | Required. The GCP location. The format is: projects/{project}/locations/{location}. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create insight
insight = provider.workloadmanager_api.Insight {
    location = "value"  # Required. The GCP location. The format is: projects/{project}/locations/{location}.
}

```

---


### Result

Lists the result of a single evaluation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution_results` | Vec<String> | The versions from the specified publisher. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access result outputs
result_id = result.id
result_execution_results = result.execution_results
result_next_page_token = result.next_page_token
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_metadata = location.metadata
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
location_name = location.name
```

---


### Execution

Creates a new Execution in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `execution_id` | String |  | Required. Id of the requesting object If auto-generating Id server-side, remove this field and execution_id from the method_signature of Create RPC |
| `request_id` | String |  | Optional. An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. The server will guarantee that for at least 60 minutes since the first request. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported (00000000-0000-0000-0000-000000000000). |
| `execution` | String |  | Required. The resource being created |
| `name` | String | ✅ | Required. The resource name of the Execution using the form: 'projects/{project}/locations/{location}/evaluations/{evaluation}/executions/{execution}' |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notices` | Vec<String> | Output only. Additional information generated by the execution |
| `state` | String | Output only. [Output only] State |
| `result_summary` | String | Output only. [Output only] Result summary for the execution |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `engine` | String | Optional. Engine |
| `external_data_sources` | Vec<String> | Optional. External data sources |
| `end_time` | String | Output only. [Output only] End time stamp |
| `rule_results` | Vec<String> | Output only. execution result summary per rule |
| `name` | String | The name of execution resource. The format is projects/{project}/locations/{location}/evaluations/{evaluation}/executions/{execution} |
| `start_time` | String | Output only. [Output only] Start time stamp |
| `run_type` | String | type represent whether the execution executed directly by user or scheduled according evaluation.schedule field. |
| `inventory_time` | String | Output only. [Output only] Inventory time stamp |
| `evaluation_id` | String | Output only. [Output only] Evaluation ID |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create execution
execution = provider.workloadmanager_api.Execution {
    name = "value"  # Required. The resource name of the Execution using the form: 'projects/{project}/locations/{location}/evaluations/{evaluation}/executions/{execution}'
}

# Access execution outputs
execution_id = execution.id
execution_notices = execution.notices
execution_state = execution.state
execution_result_summary = execution.result_summary
execution_labels = execution.labels
execution_engine = execution.engine
execution_external_data_sources = execution.external_data_sources
execution_end_time = execution.end_time
execution_rule_results = execution.rule_results
execution_name = execution.name
execution_start_time = execution.start_time
execution_run_type = execution.run_type
execution_inventory_time = execution.inventory_time
execution_evaluation_id = execution.evaluation_id
```

---


### Rule

Lists rules in a given project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invalid_rules_wrapper` | String | A wrapper of the invalid rules that failed to be validated. |
| `rules` | Vec<String> | all rules in response |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access rule outputs
rule_id = rule.id
rule_invalid_rules_wrapper = rule.invalid_rules_wrapper
rule_rules = rule.rules
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple discoveredprofile resources
discoveredprofile_0 = provider.workloadmanager_api.Discoveredprofile {
}
discoveredprofile_1 = provider.workloadmanager_api.Discoveredprofile {
}
discoveredprofile_2 = provider.workloadmanager_api.Discoveredprofile {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    discoveredprofile = provider.workloadmanager_api.Discoveredprofile {
    }
```

---

## Related Documentation

- [GCP Workloadmanager_api Documentation](https://cloud.google.com/workloadmanager_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
