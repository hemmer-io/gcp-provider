# Transcoder_api Service



**Resources**: 4

---

## Overview

The transcoder_api service provides access to 4 resource types:

- [Job](#job) [CRD]
- [Job_template](#job_template) [CRD]
- [Job](#job) [CRD]
- [Job_template](#job_template) [CRD]

---

## Resources


### Job

Creates a job in the specified region.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `batch_mode_priority` | i64 |  | The processing priority of a batch job. This field can only be set for batch mode jobs. The default value is 0. This value cannot be negative. Higher values correspond to higher priorities for the job. |
| `fill_content_gaps` | bool |  | Optional. Insert silence and duplicate frames when timestamp gaps are detected in a given stream. |
| `optimization` | String |  | Optional. The optimization strategy of the job. The default is `AUTODETECT`. |
| `state` | String |  | Output only. The current state of the job. |
| `input_uri` | String |  | Input only. Specify the `input_uri` to populate empty `uri` fields in each element of `Job.config.inputs` or `JobTemplate.config.inputs` when using template. URI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, `gs://bucket/inputs/file.mp4`). See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats). |
| `end_time` | String |  | Output only. The time the transcoding finished. |
| `labels` | HashMap<String, String> |  | The labels associated with this job. You can use these to organize and group your jobs. |
| `name` | String |  | The resource name of the job. Format: `projects/{project_number}/locations/{location}/jobs/{job}` |
| `error` | String |  | Output only. An error object that describes the reason for the failure. This property is always present when ProcessingState is `FAILED`. |
| `config` | String |  | The configuration for this job. |
| `create_time` | String |  | Output only. The time the job was created. |
| `output_uri` | String |  | Input only. Specify the `output_uri` to populate an empty `Job.config.output.uri` or `JobTemplate.config.output.uri` when using template. URI for the output file(s). For example, `gs://my-bucket/outputs/`. See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats). |
| `template_id` | String |  | Input only. Specify the `template_id` to use for populating `Job.config`. The default is `preset/web-hd`, which is the only supported preset. User defined JobTemplate: `{job_template_id}` |
| `start_time` | String |  | Output only. The time the transcoding started. |
| `mode` | String |  | The processing mode of the job. The default is `PROCESSING_MODE_INTERACTIVE`. |
| `ttl_after_completion_days` | i64 |  | Job time to live value in days, which will be effective after job completion. Job should be deleted automatically after the given TTL. Enter a value between 1 and 90. The default is 30. |
| `parent` | String | ✅ | Required. The parent location to create and process this job. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `batch_mode_priority` | i64 | The processing priority of a batch job. This field can only be set for batch mode jobs. The default value is 0. This value cannot be negative. Higher values correspond to higher priorities for the job. |
| `fill_content_gaps` | bool | Optional. Insert silence and duplicate frames when timestamp gaps are detected in a given stream. |
| `optimization` | String | Optional. The optimization strategy of the job. The default is `AUTODETECT`. |
| `state` | String | Output only. The current state of the job. |
| `input_uri` | String | Input only. Specify the `input_uri` to populate empty `uri` fields in each element of `Job.config.inputs` or `JobTemplate.config.inputs` when using template. URI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, `gs://bucket/inputs/file.mp4`). See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats). |
| `end_time` | String | Output only. The time the transcoding finished. |
| `labels` | HashMap<String, String> | The labels associated with this job. You can use these to organize and group your jobs. |
| `name` | String | The resource name of the job. Format: `projects/{project_number}/locations/{location}/jobs/{job}` |
| `error` | String | Output only. An error object that describes the reason for the failure. This property is always present when ProcessingState is `FAILED`. |
| `config` | String | The configuration for this job. |
| `create_time` | String | Output only. The time the job was created. |
| `output_uri` | String | Input only. Specify the `output_uri` to populate an empty `Job.config.output.uri` or `JobTemplate.config.output.uri` when using template. URI for the output file(s). For example, `gs://my-bucket/outputs/`. See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats). |
| `template_id` | String | Input only. Specify the `template_id` to use for populating `Job.config`. The default is `preset/web-hd`, which is the only supported preset. User defined JobTemplate: `{job_template_id}` |
| `start_time` | String | Output only. The time the transcoding started. |
| `mode` | String | The processing mode of the job. The default is `PROCESSING_MODE_INTERACTIVE`. |
| `ttl_after_completion_days` | i64 | Job time to live value in days, which will be effective after job completion. Job should be deleted automatically after the given TTL. Enter a value between 1 and 90. The default is 30. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.transcoder_api.Job {
    parent = "value"  # Required. The parent location to create and process this job. Format: `projects/{project}/locations/{location}`
}

# Access job outputs
job_id = job.id
job_batch_mode_priority = job.batch_mode_priority
job_fill_content_gaps = job.fill_content_gaps
job_optimization = job.optimization
job_state = job.state
job_input_uri = job.input_uri
job_end_time = job.end_time
job_labels = job.labels
job_name = job.name
job_error = job.error
job_config = job.config
job_create_time = job.create_time
job_output_uri = job.output_uri
job_template_id = job.template_id
job_start_time = job.start_time
job_mode = job.mode
job_ttl_after_completion_days = job.ttl_after_completion_days
```

---


### Job_template

Creates a job template in the specified region.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `config` | String |  | The configuration for this template. |
| `labels` | HashMap<String, String> |  | The labels associated with this job template. You can use these to organize and group your job templates. |
| `name` | String |  | The resource name of the job template. Format: `projects/{project_number}/locations/{location}/jobTemplates/{job_template}` |
| `parent` | String | ✅ | Required. The parent location to create this job template. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `config` | String | The configuration for this template. |
| `labels` | HashMap<String, String> | The labels associated with this job template. You can use these to organize and group your job templates. |
| `name` | String | The resource name of the job template. Format: `projects/{project_number}/locations/{location}/jobTemplates/{job_template}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job_template
job_template = provider.transcoder_api.Job_template {
    parent = "value"  # Required. The parent location to create this job template. Format: `projects/{project}/locations/{location}`
}

# Access job_template outputs
job_template_id = job_template.id
job_template_config = job_template.config
job_template_labels = job_template.labels
job_template_name = job_template.name
```

---


### Job

Creates a job in the specified region.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `progress` | String |  | Output only. Estimated fractional progress, from `0` to `1` for each step. *Note*: This feature is not yet available. |
| `failure_details` | Vec<String> |  | Output only. List of failure details. This property may contain additional information about the failure when `failure_reason` is present. *Note*: This feature is not yet available. |
| `priority` | i64 |  | Specify the priority of the job. Enter a value between 0 and 100, where 0 is the lowest priority and 100 is the highest priority. The default is 0. |
| `config` | String |  | The configuration for this job. |
| `start_time` | String |  | Output only. The time the transcoding started. |
| `end_time` | String |  | Output only. The time the transcoding finished. |
| `create_time` | String |  | Output only. The time the job was created. |
| `input_uri` | String |  | Input only. Specify the `input_uri` to populate empty `uri` fields in each element of `Job.config.inputs` or `JobTemplate.config.inputs` when using template. URI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, `gs://bucket/inputs/file.mp4`). |
| `origin_uri` | String |  | Output only. The origin URI. *Note*: This feature is not yet available. |
| `output_uri` | String |  | Input only. Specify the `output_uri` to populate an empty `Job.config.output.uri` or `JobTemplate.config.output.uri` when using template. URI for the output file(s). For example, `gs://my-bucket/outputs/`. |
| `failure_reason` | String |  | Output only. A description of the reason for the failure. This property is always present when `state` is `FAILED`. |
| `name` | String |  | The resource name of the job. Format: `projects/{project}/locations/{location}/jobs/{job}` |
| `template_id` | String |  | Input only. Specify the `template_id` to use for populating `Job.config`. The default is `preset/web-hd`. Preset Transcoder templates: - `preset/{preset_id}` - User defined JobTemplate: `{job_template_id}` |
| `state` | String |  | Output only. The current state of the job. |
| `ttl_after_completion_days` | i64 |  | Job time to live value in days, which will be effective after job completion. Job should be deleted automatically after the given TTL. Enter a value between 1 and 90. The default is 30. |
| `parent` | String | ✅ | Required. The parent location to create and process this job. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `progress` | String | Output only. Estimated fractional progress, from `0` to `1` for each step. *Note*: This feature is not yet available. |
| `failure_details` | Vec<String> | Output only. List of failure details. This property may contain additional information about the failure when `failure_reason` is present. *Note*: This feature is not yet available. |
| `priority` | i64 | Specify the priority of the job. Enter a value between 0 and 100, where 0 is the lowest priority and 100 is the highest priority. The default is 0. |
| `config` | String | The configuration for this job. |
| `start_time` | String | Output only. The time the transcoding started. |
| `end_time` | String | Output only. The time the transcoding finished. |
| `create_time` | String | Output only. The time the job was created. |
| `input_uri` | String | Input only. Specify the `input_uri` to populate empty `uri` fields in each element of `Job.config.inputs` or `JobTemplate.config.inputs` when using template. URI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, `gs://bucket/inputs/file.mp4`). |
| `origin_uri` | String | Output only. The origin URI. *Note*: This feature is not yet available. |
| `output_uri` | String | Input only. Specify the `output_uri` to populate an empty `Job.config.output.uri` or `JobTemplate.config.output.uri` when using template. URI for the output file(s). For example, `gs://my-bucket/outputs/`. |
| `failure_reason` | String | Output only. A description of the reason for the failure. This property is always present when `state` is `FAILED`. |
| `name` | String | The resource name of the job. Format: `projects/{project}/locations/{location}/jobs/{job}` |
| `template_id` | String | Input only. Specify the `template_id` to use for populating `Job.config`. The default is `preset/web-hd`. Preset Transcoder templates: - `preset/{preset_id}` - User defined JobTemplate: `{job_template_id}` |
| `state` | String | Output only. The current state of the job. |
| `ttl_after_completion_days` | i64 | Job time to live value in days, which will be effective after job completion. Job should be deleted automatically after the given TTL. Enter a value between 1 and 90. The default is 30. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.transcoder_api.Job {
    parent = "value"  # Required. The parent location to create and process this job. Format: `projects/{project}/locations/{location}`
}

# Access job outputs
job_id = job.id
job_progress = job.progress
job_failure_details = job.failure_details
job_priority = job.priority
job_config = job.config
job_start_time = job.start_time
job_end_time = job.end_time
job_create_time = job.create_time
job_input_uri = job.input_uri
job_origin_uri = job.origin_uri
job_output_uri = job.output_uri
job_failure_reason = job.failure_reason
job_name = job.name
job_template_id = job.template_id
job_state = job.state
job_ttl_after_completion_days = job.ttl_after_completion_days
```

---


### Job_template

Creates a job template in the specified region.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The resource name of the job template. Format: `projects/{project}/locations/{location}/jobTemplates/{job_template}` |
| `config` | String |  | The configuration for this template. |
| `parent` | String | ✅ | Required. The parent location to create this job template. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the job template. Format: `projects/{project}/locations/{location}/jobTemplates/{job_template}` |
| `config` | String | The configuration for this template. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job_template
job_template = provider.transcoder_api.Job_template {
    parent = "value"  # Required. The parent location to create this job template. Format: `projects/{project}/locations/{location}`
}

# Access job_template outputs
job_template_id = job_template.id
job_template_name = job_template.name
job_template_config = job_template.config
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple job resources
job_0 = provider.transcoder_api.Job {
    parent = "value-0"
}
job_1 = provider.transcoder_api.Job {
    parent = "value-1"
}
job_2 = provider.transcoder_api.Job {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    job = provider.transcoder_api.Job {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Transcoder_api Documentation](https://cloud.google.com/transcoder_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
