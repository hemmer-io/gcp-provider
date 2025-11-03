# Dataflow_api Service



**Resources**: 10

---

## Overview

The dataflow_api service provides access to 10 resource types:

- [Project](#project) [CD]
- [Work_item](#work_item) [C]
- [Location](#location) [C]
- [Stage](#stage) [R]
- [Template](#template) [CR]
- [Snapshot](#snapshot) [RD]
- [Message](#message) [R]
- [Debug](#debug) [C]
- [Flex_template](#flex_template) [C]
- [Job](#job) [CRU]

---

## Resources


### Project

Send a worker_message to the service.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location` | String |  | The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job. |
| `worker_messages` | Vec<String> |  | The WorkerMessages to send. |
| `project_id` | String | ✅ | The project to send the WorkerMessages to. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.dataflow_api.Project {
    project_id = "value"  # The project to send the WorkerMessages to.
}

```

---


### Work_item

Reports the status of dataflow WorkItems leased by a worker.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_number` | String |  | Optional. The project number of the project which owns the WorkItem's job. |
| `unified_worker_request` | HashMap<String, String> |  | Untranslated bag-of-bytes WorkProgressUpdateRequest from UnifiedWorker. |
| `work_item_statuses` | Vec<String> |  | The order is unimportant, except that the order of the WorkItemServiceState messages in the ReportWorkItemStatusResponse corresponds to the order of WorkItemStatus messages here. |
| `worker_id` | String |  | The ID of the worker reporting the WorkItem status. If this does not match the ID of the worker which the Dataflow service believes currently has the lease on the WorkItem, the report will be dropped (with an error response). |
| `current_worker_time` | String |  | The current timestamp at the worker. |
| `location` | String |  | The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the WorkItem's job. |
| `location` | String | ✅ | The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the WorkItem's job. |
| `project_id` | String | ✅ | The project which owns the WorkItem's job. |
| `job_id` | String | ✅ | The job which the WorkItem is part of. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create work_item
work_item = provider.dataflow_api.Work_item {
    location = "value"  # The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the WorkItem's job.
    project_id = "value"  # The project which owns the WorkItem's job.
    job_id = "value"  # The job which the WorkItem is part of.
}

```

---


### Location

Send a worker_message to the service.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location` | String |  | The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job. |
| `worker_messages` | Vec<String> |  | The WorkerMessages to send. |
| `project_id` | String | ✅ | The project to send the WorkerMessages to. |
| `location` | String | ✅ | The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.dataflow_api.Location {
    project_id = "value"  # The project to send the WorkerMessages to.
    location = "value"  # The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job.
}

```

---


### Stage

Request detailed information about the execution status of a stage of the job. EXPERIMENTAL. This API is subject to change or removal without notice.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workers` | Vec<String> | Workers that have done work on the stage. |
| `next_page_token` | String | If present, this response does not contain all requested tasks. To obtain the next page of results, repeat the request with page_token set to this value. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access stage outputs
stage_id = stage.id
stage_workers = stage.workers
stage_next_page_token = stage.next_page_token
```

---


### Template

Creates a Cloud Dataflow job from a template. Do not enter confidential information when you supply string values using the API. To create a job, we recommend using `projects.locations.templates.create` with a [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using `projects.templates.create` is not recommended, because your job will always start in `us-central1`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_path` | String |  | Required. A Cloud Storage path to the template from which to create the job. Must be a valid Cloud Storage URL, beginning with `gs://`. |
| `parameters` | HashMap<String, String> |  | The runtime parameters to pass to the job. |
| `location` | String |  | The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to which to direct the request. |
| `environment` | String |  | The runtime environment for the job. |
| `job_name` | String |  | Required. The job name to use for the created job. |
| `project_id` | String | ✅ | Required. The ID of the Cloud Platform project that the job belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | The template metadata describing the template name, available parameters, etc. |
| `runtime_metadata` | String | Describes the runtime metadata with SDKInfo and available parameters. |
| `template_type` | String | Template Type. |
| `status` | String | The status of the get template request. Any problems with the request will be indicated in the error_details. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create template
template = provider.dataflow_api.Template {
    project_id = "value"  # Required. The ID of the Cloud Platform project that the job belongs to.
}

# Access template outputs
template_id = template.id
template_metadata = template.metadata
template_runtime_metadata = template.runtime_metadata
template_template_type = template.template_type
template_status = template.status
```

---


### Snapshot

Gets information about a snapshot.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | User specified description of the snapshot. Maybe empty. |
| `disk_size_bytes` | String | The disk byte size of the snapshot. Only available for snapshots in READY state. |
| `id` | String | The unique ID of this snapshot. |
| `project_id` | String | The project this snapshot belongs to. |
| `creation_time` | String | The time this snapshot was created. |
| `region` | String | Cloud region where this snapshot lives in, e.g., "us-central1". |
| `state` | String | State of the snapshot. |
| `ttl` | String | The time after which this snapshot will be automatically deleted. |
| `source_job_id` | String | The job this snapshot was created from. |
| `pubsub_metadata` | Vec<String> | Pub/Sub snapshot metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access snapshot outputs
snapshot_id = snapshot.id
snapshot_description = snapshot.description
snapshot_disk_size_bytes = snapshot.disk_size_bytes
snapshot_id = snapshot.id
snapshot_project_id = snapshot.project_id
snapshot_creation_time = snapshot.creation_time
snapshot_region = snapshot.region
snapshot_state = snapshot.state
snapshot_ttl = snapshot.ttl
snapshot_source_job_id = snapshot.source_job_id
snapshot_pubsub_metadata = snapshot.pubsub_metadata
```

---


### Message

Request the job status. To request the status of a job, we recommend using `projects.locations.jobs.messages.list` with a [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using `projects.jobs.messages.list` is not recommended, as you can only request the status of jobs that are running in `us-central1`.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `autoscaling_events` | Vec<String> | Autoscaling events in ascending timestamp order. |
| `job_messages` | Vec<String> | Messages in ascending timestamp order. |
| `next_page_token` | String | The token to obtain the next page of results if there are more. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access message outputs
message_id = message.id
message_autoscaling_events = message.autoscaling_events
message_job_messages = message.job_messages
message_next_page_token = message.next_page_token
```

---


### Debug

Get encoded debug configuration for component. Not cacheable.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `worker_id` | String |  | The worker id, i.e., VM hostname. |
| `component_id` | String |  | The internal component id for which debug configuration is requested. |
| `location` | String |  | The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job specified by job_id. |
| `project_id` | String | ✅ | The project id. |
| `location` | String | ✅ | The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job specified by job_id. |
| `job_id` | String | ✅ | The job id. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create debug
debug = provider.dataflow_api.Debug {
    project_id = "value"  # The project id.
    location = "value"  # The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains the job specified by job_id.
    job_id = "value"  # The job id.
}

```

---


### Flex_template

Launch a job with a FlexTemplate.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `launch_parameter` | String |  | Required. Parameter to launch a job form Flex Template. |
| `validate_only` | bool |  | If true, the request is validated but not actually executed. Defaults to false. |
| `location` | String | ✅ | Required. The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to which to direct the request. E.g., us-central1, us-west1. |
| `project_id` | String | ✅ | Required. The ID of the Cloud Platform project that the job belongs to. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create flex_template
flex_template = provider.dataflow_api.Flex_template {
    location = "value"  # Required. The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to which to direct the request. E.g., us-central1, us-west1.
    project_id = "value"  # Required. The ID of the Cloud Platform project that the job belongs to.
}

```

---


### Job

Creates a Dataflow job. To create a job, we recommend using `projects.locations.jobs.create` with a [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using `projects.jobs.create` is not recommended, as your job will always start in `us-central1`. Do not enter confidential information when you supply string values using the API.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_request_id` | String |  | The client's unique identifier of the job, re-used across retried attempts. If this field is set, the service will ensure its uniqueness. The request to create a job will fail if the service has knowledge of a previously submitted job with the same client's ID and job name. The caller may use this field to ensure idempotence of job creation across retried attempts to create a job. By default, the field is empty and, in that case, the service ignores it. |
| `start_time` | String |  | The timestamp when the job was started (transitioned to JOB_STATE_PENDING). Flexible resource scheduling jobs are started with some delay after job creation, so start_time is unset before start and is updated when the job is started by the Cloud Dataflow service. For other jobs, start_time always equals to create_time and is immutable and set by the Cloud Dataflow service. |
| `service_resources` | String |  | Output only. Resources used by the Dataflow Service to run the job. |
| `execution_info` | String |  | Deprecated. |
| `location` | String |  | Optional. The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains this job. |
| `id` | String |  | The unique ID of this job. This field is set by the Dataflow service when the job is created, and is immutable for the life of the job. |
| `job_metadata` | String |  | This field is populated by the Dataflow service to support filtering jobs by the metadata values provided here. Populated for ListJobs and all GetJob views SUMMARY and higher. |
| `pipeline_description` | String |  | Preliminary field: The format of this data may change at any time. A description of the user pipeline and stages through which it is executed. Created by Cloud Dataflow service. Only retrieved with JOB_VIEW_DESCRIPTION or JOB_VIEW_ALL. |
| `current_state` | String |  | The current state of the job. Jobs are created in the `JOB_STATE_STOPPED` state unless otherwise specified. A job in the `JOB_STATE_RUNNING` state may asynchronously enter a terminal state. After a job has reached a terminal state, no further state updates may be made. This field might be mutated by the Dataflow service; callers cannot mutate it. |
| `create_time` | String |  | The timestamp when the job was initially created. Immutable and set by the Cloud Dataflow service. |
| `satisfies_pzs` | bool |  | Reserved for future use. This field is set only in responses from the server; it is ignored if it is set in any requests. |
| `stage_states` | Vec<String> |  | This field may be mutated by the Cloud Dataflow service; callers cannot mutate it. |
| `type` | String |  | Optional. The type of Dataflow job. |
| `name` | String |  | Optional. The user-specified Dataflow job name. Only one active job with a given name can exist in a project within one region at any given time. Jobs in different regions can have the same name. If a caller attempts to create a job with the same name as an active job that already exists, the attempt returns the existing job. The name must match the regular expression `[a-z]([-a-z0-9]{0,1022}[a-z0-9])?` |
| `current_state_time` | String |  | The timestamp associated with the current state. |
| `replaced_by_job_id` | String |  | If another job is an update of this job (and thus, this job is in `JOB_STATE_UPDATED`), this field contains the ID of that job. |
| `requested_state` | String |  | The job's requested state. Applies to `UpdateJob` requests. Set `requested_state` with `UpdateJob` requests to switch between the states `JOB_STATE_STOPPED` and `JOB_STATE_RUNNING`. You can also use `UpdateJob` requests to change a job's state from `JOB_STATE_RUNNING` to `JOB_STATE_CANCELLED`, `JOB_STATE_DONE`, or `JOB_STATE_DRAINED`. These states irrevocably terminate the job if it hasn't already reached a terminal state. This field has no effect on `CreateJob` requests. |
| `environment` | String |  | Optional. The environment for the job. |
| `labels` | HashMap<String, String> |  | User-defined labels for this job. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: * Keys must conform to regexp: \p{Ll}\p{Lo}{0,62} * Values must conform to regexp: [\p{Ll}\p{Lo}\p{N}_-]{0,63} * Both keys and values are additionally constrained to be <= 128 bytes in size. |
| `replace_job_id` | String |  | If this job is an update of an existing job, this field is the job ID of the job it replaced. When sending a `CreateJobRequest`, you can update a job by specifying it here. The job named here is stopped, and its intermediate state is transferred to this job. |
| `steps` | Vec<String> |  | Exactly one of step or steps_location should be specified. The top-level steps that constitute the entire job. Only retrieved with JOB_VIEW_ALL. |
| `project_id` | String |  | The ID of the Google Cloud project that the job belongs to. |
| `steps_location` | String |  | The Cloud Storage location where the steps are stored. |
| `created_from_snapshot_id` | String |  | If this is specified, the job's initial state is populated from the given snapshot. |
| `temp_files` | Vec<String> |  | A set of files the system should be aware of that are used for temporary storage. These temporary files will be removed on job completion. No duplicates are allowed. No file patterns are supported. The supported files are: Google Cloud Storage: storage.googleapis.com/{bucket}/{object} bucket.storage.googleapis.com/{object} |
| `transform_name_mapping` | HashMap<String, String> |  | Optional. The map of transform name prefixes of the job to be replaced to the corresponding name prefixes of the new job. |
| `runtime_updatable_params` | String |  | This field may ONLY be modified at runtime using the projects.jobs.update method to adjust job behavior. This field has no effect when specified at job creation. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. This field is set only in responses from the server; it is ignored if it is set in any requests. |
| `project_id` | String | ✅ | The ID of the Cloud Platform project that the job belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client_request_id` | String | The client's unique identifier of the job, re-used across retried attempts. If this field is set, the service will ensure its uniqueness. The request to create a job will fail if the service has knowledge of a previously submitted job with the same client's ID and job name. The caller may use this field to ensure idempotence of job creation across retried attempts to create a job. By default, the field is empty and, in that case, the service ignores it. |
| `start_time` | String | The timestamp when the job was started (transitioned to JOB_STATE_PENDING). Flexible resource scheduling jobs are started with some delay after job creation, so start_time is unset before start and is updated when the job is started by the Cloud Dataflow service. For other jobs, start_time always equals to create_time and is immutable and set by the Cloud Dataflow service. |
| `service_resources` | String | Output only. Resources used by the Dataflow Service to run the job. |
| `execution_info` | String | Deprecated. |
| `location` | String | Optional. The [regional endpoint] (https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that contains this job. |
| `id` | String | The unique ID of this job. This field is set by the Dataflow service when the job is created, and is immutable for the life of the job. |
| `job_metadata` | String | This field is populated by the Dataflow service to support filtering jobs by the metadata values provided here. Populated for ListJobs and all GetJob views SUMMARY and higher. |
| `pipeline_description` | String | Preliminary field: The format of this data may change at any time. A description of the user pipeline and stages through which it is executed. Created by Cloud Dataflow service. Only retrieved with JOB_VIEW_DESCRIPTION or JOB_VIEW_ALL. |
| `current_state` | String | The current state of the job. Jobs are created in the `JOB_STATE_STOPPED` state unless otherwise specified. A job in the `JOB_STATE_RUNNING` state may asynchronously enter a terminal state. After a job has reached a terminal state, no further state updates may be made. This field might be mutated by the Dataflow service; callers cannot mutate it. |
| `create_time` | String | The timestamp when the job was initially created. Immutable and set by the Cloud Dataflow service. |
| `satisfies_pzs` | bool | Reserved for future use. This field is set only in responses from the server; it is ignored if it is set in any requests. |
| `stage_states` | Vec<String> | This field may be mutated by the Cloud Dataflow service; callers cannot mutate it. |
| `type` | String | Optional. The type of Dataflow job. |
| `name` | String | Optional. The user-specified Dataflow job name. Only one active job with a given name can exist in a project within one region at any given time. Jobs in different regions can have the same name. If a caller attempts to create a job with the same name as an active job that already exists, the attempt returns the existing job. The name must match the regular expression `[a-z]([-a-z0-9]{0,1022}[a-z0-9])?` |
| `current_state_time` | String | The timestamp associated with the current state. |
| `replaced_by_job_id` | String | If another job is an update of this job (and thus, this job is in `JOB_STATE_UPDATED`), this field contains the ID of that job. |
| `requested_state` | String | The job's requested state. Applies to `UpdateJob` requests. Set `requested_state` with `UpdateJob` requests to switch between the states `JOB_STATE_STOPPED` and `JOB_STATE_RUNNING`. You can also use `UpdateJob` requests to change a job's state from `JOB_STATE_RUNNING` to `JOB_STATE_CANCELLED`, `JOB_STATE_DONE`, or `JOB_STATE_DRAINED`. These states irrevocably terminate the job if it hasn't already reached a terminal state. This field has no effect on `CreateJob` requests. |
| `environment` | String | Optional. The environment for the job. |
| `labels` | HashMap<String, String> | User-defined labels for this job. The labels map can contain no more than 64 entries. Entries of the labels map are UTF8 strings that comply with the following restrictions: * Keys must conform to regexp: \p{Ll}\p{Lo}{0,62} * Values must conform to regexp: [\p{Ll}\p{Lo}\p{N}_-]{0,63} * Both keys and values are additionally constrained to be <= 128 bytes in size. |
| `replace_job_id` | String | If this job is an update of an existing job, this field is the job ID of the job it replaced. When sending a `CreateJobRequest`, you can update a job by specifying it here. The job named here is stopped, and its intermediate state is transferred to this job. |
| `steps` | Vec<String> | Exactly one of step or steps_location should be specified. The top-level steps that constitute the entire job. Only retrieved with JOB_VIEW_ALL. |
| `project_id` | String | The ID of the Google Cloud project that the job belongs to. |
| `steps_location` | String | The Cloud Storage location where the steps are stored. |
| `created_from_snapshot_id` | String | If this is specified, the job's initial state is populated from the given snapshot. |
| `temp_files` | Vec<String> | A set of files the system should be aware of that are used for temporary storage. These temporary files will be removed on job completion. No duplicates are allowed. No file patterns are supported. The supported files are: Google Cloud Storage: storage.googleapis.com/{bucket}/{object} bucket.storage.googleapis.com/{object} |
| `transform_name_mapping` | HashMap<String, String> | Optional. The map of transform name prefixes of the job to be replaced to the corresponding name prefixes of the new job. |
| `runtime_updatable_params` | String | This field may ONLY be modified at runtime using the projects.jobs.update method to adjust job behavior. This field has no effect when specified at job creation. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. This field is set only in responses from the server; it is ignored if it is set in any requests. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.dataflow_api.Job {
    project_id = "value"  # The ID of the Cloud Platform project that the job belongs to.
}

# Access job outputs
job_id = job.id
job_client_request_id = job.client_request_id
job_start_time = job.start_time
job_service_resources = job.service_resources
job_execution_info = job.execution_info
job_location = job.location
job_id = job.id
job_job_metadata = job.job_metadata
job_pipeline_description = job.pipeline_description
job_current_state = job.current_state
job_create_time = job.create_time
job_satisfies_pzs = job.satisfies_pzs
job_stage_states = job.stage_states
job_type = job.type
job_name = job.name
job_current_state_time = job.current_state_time
job_replaced_by_job_id = job.replaced_by_job_id
job_requested_state = job.requested_state
job_environment = job.environment
job_labels = job.labels
job_replace_job_id = job.replace_job_id
job_steps = job.steps
job_project_id = job.project_id
job_steps_location = job.steps_location
job_created_from_snapshot_id = job.created_from_snapshot_id
job_temp_files = job.temp_files
job_transform_name_mapping = job.transform_name_mapping
job_runtime_updatable_params = job.runtime_updatable_params
job_satisfies_pzi = job.satisfies_pzi
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple project resources
project_0 = provider.dataflow_api.Project {
    project_id = "value-0"
}
project_1 = provider.dataflow_api.Project {
    project_id = "value-1"
}
project_2 = provider.dataflow_api.Project {
    project_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    project = provider.dataflow_api.Project {
        project_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Dataflow_api Documentation](https://cloud.google.com/dataflow_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
