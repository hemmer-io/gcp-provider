# Workflowexecutions_api Service



**Resources**: 5

---

## Overview

The workflowexecutions_api service provides access to 5 resource types:

- [Workflow](#workflow) [C]
- [Step_entrie](#step_entrie) [R]
- [Execution](#execution) [CR]
- [Callback](#callback) [R]
- [Execution](#execution) [CR]

---

## Resources


### Workflow

Triggers a new execution using the latest revision of the given workflow by a Pub/Sub push notification.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delivery_attempt` | i64 |  | The number of attempts that have been made to deliver this message. This is set by Pub/Sub for subscriptions that have the "dead letter" feature enabled, and hence provided here for compatibility, but is ignored by Workflows. |
| `subscription` | String |  | Required. The subscription of the Pub/Sub push notification. Format: projects/{project}/subscriptions/{sub} |
| `gcp_cloud_events_mode` | String |  | Required. LINT: LEGACY_NAMES The query parameter value for __GCP_CloudEventsMode, set by the Eventarc service when configuring triggers. |
| `message` | String |  | Required. The message of the Pub/Sub push notification. |
| `workflow` | String | ✅ | Required. Name of the workflow for which an execution should be created. Format: projects/{project}/locations/{location}/workflows/{workflow} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workflow
workflow = provider.workflowexecutions_api.Workflow {
    workflow = "value"  # Required. Name of the workflow for which an execution should be created. Format: projects/{project}/locations/{location}/workflows/{workflow}
}

```

---


### Step_entrie

Gets a step entry.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `routine` | String | Output only. The name of the routine this step entry belongs to. A routine name is the subworkflow name defined in the YAML source code. The top level routine name is `main`. |
| `navigation_info` | String | Output only. The NavigationInfo associated with this step. |
| `name` | String | Output only. The full resource name of the step entry. Each step entry has a unique entry ID, which is a monotonically increasing counter. Step entry names have the format: `projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}/stepEntries/{step_entry}`. |
| `entry_id` | String | Output only. The numeric ID of this step entry, used for navigation. |
| `exception` | String | Output only. The exception thrown by the step entry. |
| `step_entry_metadata` | String | Output only. The StepEntryMetadata associated with this step. |
| `step_type` | String | Output only. The type of the step this step entry belongs to. |
| `create_time` | String | Output only. The creation time of the step entry. |
| `step` | String | Output only. The name of the step this step entry belongs to. |
| `update_time` | String | Output only. The most recently updated time of the step entry. |
| `variable_data` | String | Output only. The VariableData associated with this step. |
| `state` | String | Output only. The state of the step entry. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access step_entrie outputs
step_entrie_id = step_entrie.id
step_entrie_routine = step_entrie.routine
step_entrie_navigation_info = step_entrie.navigation_info
step_entrie_name = step_entrie.name
step_entrie_entry_id = step_entrie.entry_id
step_entrie_exception = step_entrie.exception
step_entrie_step_entry_metadata = step_entrie.step_entry_metadata
step_entrie_step_type = step_entrie.step_type
step_entrie_create_time = step_entrie.create_time
step_entrie_step = step_entrie.step
step_entrie_update_time = step_entrie.update_time
step_entrie_variable_data = step_entrie.variable_data
step_entrie_state = step_entrie.state
```

---


### Execution

Creates a new execution using the latest revision of the given workflow. For more information, see Execute a workflow.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_time` | String |  | Output only. Marks the beginning of execution. Note that this will be the same as `createTime` for executions that start immediately. |
| `create_time` | String |  | Output only. Marks the creation of the execution. |
| `labels` | HashMap<String, String> |  | Labels associated with this execution. Labels can contain at most 64 entries. Keys and values can be no longer than 63 characters and can only contain lowercase letters, numeric characters, underscores, and dashes. Label keys must start with a letter. International characters are allowed. By default, labels are inherited from the workflow but are overridden by any labels associated with the execution. |
| `duration` | String |  | Output only. Measures the duration of the execution. |
| `disable_concurrency_quota_overflow_buffering` | bool |  | Optional. If set to true, the execution will not be backlogged when the concurrency quota is exhausted. The backlog execution starts when the concurrency quota becomes available. |
| `execution_history_level` | String |  | Optional. Describes the execution history level to apply to this execution. If not specified, the execution history level is determined by its workflow's execution history level. If the levels are different, the executionHistoryLevel overrides the workflow's execution history level for this execution. |
| `end_time` | String |  | Output only. Marks the end of execution, successful or not. |
| `workflow_revision_id` | String |  | Output only. Revision of the workflow this execution is using. |
| `argument` | String |  | Input parameters of the execution represented as a JSON string. The size limit is 32KB. *Note*: If you are using the REST API directly to run your workflow, you must escape any JSON string value of `argument`. Example: `'{"argument":"{\"firstName\":\"FIRST\",\"lastName\":\"LAST\"}"}'` |
| `state` | String |  | Output only. Current state of the execution. |
| `state_error` | String |  | Output only. Error regarding the state of the Execution resource. For example, this field will have error details if the execution data is unavailable due to revoked KMS key permissions. |
| `name` | String |  | Output only. The resource name of the execution. Format: projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution} |
| `call_log_level` | String |  | The call logging level associated to this execution. |
| `result` | String |  | Output only. Output of the execution represented as a JSON string. The value can only be present if the execution's state is `SUCCEEDED`. |
| `status` | String |  | Output only. Status tracks the current steps and progress data of this execution. |
| `error` | String |  | Output only. The error which caused the execution to finish prematurely. The value is only present if the execution's state is `FAILED` or `CANCELLED`. |
| `parent` | String | ✅ | Required. Name of the workflow for which an execution should be created. Format: projects/{project}/locations/{location}/workflows/{workflow} The latest revision of the workflow will be used. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | Output only. Marks the beginning of execution. Note that this will be the same as `createTime` for executions that start immediately. |
| `create_time` | String | Output only. Marks the creation of the execution. |
| `labels` | HashMap<String, String> | Labels associated with this execution. Labels can contain at most 64 entries. Keys and values can be no longer than 63 characters and can only contain lowercase letters, numeric characters, underscores, and dashes. Label keys must start with a letter. International characters are allowed. By default, labels are inherited from the workflow but are overridden by any labels associated with the execution. |
| `duration` | String | Output only. Measures the duration of the execution. |
| `disable_concurrency_quota_overflow_buffering` | bool | Optional. If set to true, the execution will not be backlogged when the concurrency quota is exhausted. The backlog execution starts when the concurrency quota becomes available. |
| `execution_history_level` | String | Optional. Describes the execution history level to apply to this execution. If not specified, the execution history level is determined by its workflow's execution history level. If the levels are different, the executionHistoryLevel overrides the workflow's execution history level for this execution. |
| `end_time` | String | Output only. Marks the end of execution, successful or not. |
| `workflow_revision_id` | String | Output only. Revision of the workflow this execution is using. |
| `argument` | String | Input parameters of the execution represented as a JSON string. The size limit is 32KB. *Note*: If you are using the REST API directly to run your workflow, you must escape any JSON string value of `argument`. Example: `'{"argument":"{\"firstName\":\"FIRST\",\"lastName\":\"LAST\"}"}'` |
| `state` | String | Output only. Current state of the execution. |
| `state_error` | String | Output only. Error regarding the state of the Execution resource. For example, this field will have error details if the execution data is unavailable due to revoked KMS key permissions. |
| `name` | String | Output only. The resource name of the execution. Format: projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution} |
| `call_log_level` | String | The call logging level associated to this execution. |
| `result` | String | Output only. Output of the execution represented as a JSON string. The value can only be present if the execution's state is `SUCCEEDED`. |
| `status` | String | Output only. Status tracks the current steps and progress data of this execution. |
| `error` | String | Output only. The error which caused the execution to finish prematurely. The value is only present if the execution's state is `FAILED` or `CANCELLED`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create execution
execution = provider.workflowexecutions_api.Execution {
    parent = "value"  # Required. Name of the workflow for which an execution should be created. Format: projects/{project}/locations/{location}/workflows/{workflow} The latest revision of the workflow will be used.
}

# Access execution outputs
execution_id = execution.id
execution_start_time = execution.start_time
execution_create_time = execution.create_time
execution_labels = execution.labels
execution_duration = execution.duration
execution_disable_concurrency_quota_overflow_buffering = execution.disable_concurrency_quota_overflow_buffering
execution_execution_history_level = execution.execution_history_level
execution_end_time = execution.end_time
execution_workflow_revision_id = execution.workflow_revision_id
execution_argument = execution.argument
execution_state = execution.state
execution_state_error = execution.state_error
execution_name = execution.name
execution_call_log_level = execution.call_log_level
execution_result = execution.result
execution_status = execution.status
execution_error = execution.error
```

---


### Callback

Returns a list of active callbacks that belong to the execution with the given name. The returned callbacks are ordered by callback ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `callbacks` | Vec<String> | The callbacks which match the request. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access callback outputs
callback_id = callback.id
callback_callbacks = callback.callbacks
callback_next_page_token = callback.next_page_token
```

---


### Execution

Creates a new execution using the latest revision of the given workflow.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `argument` | String |  | Input parameters of the execution represented as a JSON string. The size limit is 32KB. *Note*: If you are using the REST API directly to run your workflow, you must escape any JSON string value of `argument`. Example: `'{"argument":"{\"firstName\":\"FIRST\",\"lastName\":\"LAST\"}"}'` |
| `status` | String |  | Output only. Status tracks the current steps and progress data of this execution. |
| `name` | String |  | Output only. The resource name of the execution. Format: projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution} |
| `workflow_revision_id` | String |  | Output only. Revision of the workflow this execution is using. |
| `call_log_level` | String |  | The call logging level associated to this execution. |
| `error` | String |  | Output only. The error which caused the execution to finish prematurely. The value is only present if the execution's state is `FAILED` or `CANCELLED`. |
| `end_time` | String |  | Output only. Marks the end of execution, successful or not. |
| `start_time` | String |  | Output only. Marks the beginning of execution. |
| `state` | String |  | Output only. Current state of the execution. |
| `result` | String |  | Output only. Output of the execution represented as a JSON string. The value can only be present if the execution's state is `SUCCEEDED`. |
| `parent` | String | ✅ | Required. Name of the workflow for which an execution should be created. Format: projects/{project}/locations/{location}/workflows/{workflow} The latest revision of the workflow will be used. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `argument` | String | Input parameters of the execution represented as a JSON string. The size limit is 32KB. *Note*: If you are using the REST API directly to run your workflow, you must escape any JSON string value of `argument`. Example: `'{"argument":"{\"firstName\":\"FIRST\",\"lastName\":\"LAST\"}"}'` |
| `status` | String | Output only. Status tracks the current steps and progress data of this execution. |
| `name` | String | Output only. The resource name of the execution. Format: projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution} |
| `workflow_revision_id` | String | Output only. Revision of the workflow this execution is using. |
| `call_log_level` | String | The call logging level associated to this execution. |
| `error` | String | Output only. The error which caused the execution to finish prematurely. The value is only present if the execution's state is `FAILED` or `CANCELLED`. |
| `end_time` | String | Output only. Marks the end of execution, successful or not. |
| `start_time` | String | Output only. Marks the beginning of execution. |
| `state` | String | Output only. Current state of the execution. |
| `result` | String | Output only. Output of the execution represented as a JSON string. The value can only be present if the execution's state is `SUCCEEDED`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create execution
execution = provider.workflowexecutions_api.Execution {
    parent = "value"  # Required. Name of the workflow for which an execution should be created. Format: projects/{project}/locations/{location}/workflows/{workflow} The latest revision of the workflow will be used.
}

# Access execution outputs
execution_id = execution.id
execution_argument = execution.argument
execution_status = execution.status
execution_name = execution.name
execution_workflow_revision_id = execution.workflow_revision_id
execution_call_log_level = execution.call_log_level
execution_error = execution.error
execution_end_time = execution.end_time
execution_start_time = execution.start_time
execution_state = execution.state
execution_result = execution.result
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple workflow resources
workflow_0 = provider.workflowexecutions_api.Workflow {
    workflow = "value-0"
}
workflow_1 = provider.workflowexecutions_api.Workflow {
    workflow = "value-1"
}
workflow_2 = provider.workflowexecutions_api.Workflow {
    workflow = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    workflow = provider.workflowexecutions_api.Workflow {
        workflow = "production-value"
    }
```

---

## Related Documentation

- [GCP Workflowexecutions_api Documentation](https://cloud.google.com/workflowexecutions_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
