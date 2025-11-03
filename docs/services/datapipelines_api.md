# Datapipelines_api Service



**Resources**: 2

---

## Overview

The datapipelines_api service provides access to 2 resource types:

- [Pipeline](#pipeline) [CRUD]
- [Job](#job) [R]

---

## Resources


### Pipeline

Creates a pipeline. For a batch pipeline, you can pass scheduler information. Data Pipelines uses the scheduler information to create an internal scheduler that runs jobs periodically. If the internal scheduler is not configured, you can use RunPipeline to run jobs.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Required. The state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through UpdatePipeline requests. |
| `workload` | String |  | Workload information for creating new jobs. |
| `create_time` | String |  | Output only. Immutable. The timestamp when the pipeline was initially created. Set by the Data Pipelines service. |
| `name` | String |  | The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`. * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects). * `LOCATION_ID` is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling `google.cloud.location.Locations.ListLocations`. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in [App Engine regions](https://cloud.google.com/about/locations#region). * `PIPELINE_ID` is the ID of the pipeline. Must be unique for the selected project and location. |
| `schedule_info` | String |  | Internal scheduling information for a pipeline. If this information is provided, periodic jobs will be created per the schedule. If not, users are responsible for creating jobs externally. |
| `pipeline_sources` | HashMap<String, String> |  | Immutable. The sources of the pipeline (for example, Dataplex). The keys and values are set by the corresponding sources during pipeline creation. |
| `display_name` | String |  | Required. The display name of the pipeline. It can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), and underscores (_). |
| `type` | String |  | Required. The type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline. |
| `job_count` | i64 |  | Output only. Number of jobs. |
| `last_update_time` | String |  | Output only. Immutable. The timestamp when the pipeline was last modified. Set by the Data Pipelines service. |
| `scheduler_service_account_email` | String |  | Optional. A service account email to be used with the Cloud Scheduler job. If not specified, the default compute engine service account will be used. |
| `parent` | String | ✅ | Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Required. The state of the pipeline. When the pipeline is created, the state is set to 'PIPELINE_STATE_ACTIVE' by default. State changes can be requested by setting the state to stopping, paused, or resuming. State cannot be changed through UpdatePipeline requests. |
| `workload` | String | Workload information for creating new jobs. |
| `create_time` | String | Output only. Immutable. The timestamp when the pipeline was initially created. Set by the Data Pipelines service. |
| `name` | String | The pipeline name. For example: `projects/PROJECT_ID/locations/LOCATION_ID/pipelines/PIPELINE_ID`. * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]), hyphens (-), colons (:), and periods (.). For more information, see [Identifying projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects). * `LOCATION_ID` is the canonical ID for the pipeline's location. The list of available locations can be obtained by calling `google.cloud.location.Locations.ListLocations`. Note that the Data Pipelines service is not available in all regions. It depends on Cloud Scheduler, an App Engine application, so it's only available in [App Engine regions](https://cloud.google.com/about/locations#region). * `PIPELINE_ID` is the ID of the pipeline. Must be unique for the selected project and location. |
| `schedule_info` | String | Internal scheduling information for a pipeline. If this information is provided, periodic jobs will be created per the schedule. If not, users are responsible for creating jobs externally. |
| `pipeline_sources` | HashMap<String, String> | Immutable. The sources of the pipeline (for example, Dataplex). The keys and values are set by the corresponding sources during pipeline creation. |
| `display_name` | String | Required. The display name of the pipeline. It can contain only letters ([A-Za-z]), numbers ([0-9]), hyphens (-), and underscores (_). |
| `type` | String | Required. The type of the pipeline. This field affects the scheduling of the pipeline and the type of metrics to show for the pipeline. |
| `job_count` | i64 | Output only. Number of jobs. |
| `last_update_time` | String | Output only. Immutable. The timestamp when the pipeline was last modified. Set by the Data Pipelines service. |
| `scheduler_service_account_email` | String | Optional. A service account email to be used with the Cloud Scheduler job. If not specified, the default compute engine service account will be used. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create pipeline
pipeline = provider.datapipelines_api.Pipeline {
    parent = "value"  # Required. The location name. For example: `projects/PROJECT_ID/locations/LOCATION_ID`.
}

# Access pipeline outputs
pipeline_id = pipeline.id
pipeline_state = pipeline.state
pipeline_workload = pipeline.workload
pipeline_create_time = pipeline.create_time
pipeline_name = pipeline.name
pipeline_schedule_info = pipeline.schedule_info
pipeline_pipeline_sources = pipeline.pipeline_sources
pipeline_display_name = pipeline.display_name
pipeline_type = pipeline.type
pipeline_job_count = pipeline.job_count
pipeline_last_update_time = pipeline.last_update_time
pipeline_scheduler_service_account_email = pipeline.scheduler_service_account_email
```

---


### Job

Lists jobs for a given pipeline. Throws a "FORBIDDEN" error if the caller doesn't have permission to access it.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `jobs` | Vec<String> | Results that were accessible to the caller. Results are always in descending order of job creation date. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access job outputs
job_id = job.id
job_jobs = job.jobs
job_next_page_token = job.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple pipeline resources
pipeline_0 = provider.datapipelines_api.Pipeline {
    parent = "value-0"
}
pipeline_1 = provider.datapipelines_api.Pipeline {
    parent = "value-1"
}
pipeline_2 = provider.datapipelines_api.Pipeline {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    pipeline = provider.datapipelines_api.Pipeline {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Datapipelines_api Documentation](https://cloud.google.com/datapipelines_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
