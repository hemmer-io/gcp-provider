# Dlp_api Service



**Resources**: 14

---

## Overview

The dlp_api service provides access to 14 resource types:

- [Deidentify_template](#deidentify_template) [CRUD]
- [Column_data_profile](#column_data_profile) [R]
- [Discovery_config](#discovery_config) [CRUD]
- [Inspect_template](#inspect_template) [CRUD]
- [Dlp_job](#dlp_job) [CRD]
- [Connection](#connection) [CRUD]
- [Job_trigger](#job_trigger) [CRUD]
- [Project_data_profile](#project_data_profile) [R]
- [Table_data_profile](#table_data_profile) [RD]
- [Content](#content) [C]
- [Image](#image) [C]
- [Stored_info_type](#stored_info_type) [CRUD]
- [File_store_data_profile](#file_store_data_profile) [RD]
- [Info_type](#info_type) [R]

---

## Resources


### Deidentify_template

Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates-deid to learn more.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location_id` | String |  | Deprecated. This field has no effect. |
| `template_id` | String |  | The template id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one. |
| `deidentify_template` | String |  | Required. The DeidentifyTemplate to create. |
| `parent` | String | ✅ | Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` + Organizations scope, location specified: `organizations/{org_id}/locations/{location_id}` + Organizations scope, no location specified (defaults to global): `organizations/{org_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Display name (max 256 chars). |
| `create_time` | String | Output only. The creation timestamp of an inspectTemplate. |
| `description` | String | Short description (max 256 chars). |
| `name` | String | Output only. The template name. The template will have one of the following formats: `projects/PROJECT_ID/deidentifyTemplates/TEMPLATE_ID` OR `organizations/ORGANIZATION_ID/deidentifyTemplates/TEMPLATE_ID` |
| `update_time` | String | Output only. The last update timestamp of an inspectTemplate. |
| `deidentify_config` | String | The core content of the template. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deidentify_template
deidentify_template = provider.dlp_api.Deidentify_template {
    parent = "value"  # Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` + Organizations scope, location specified: `organizations/{org_id}/locations/{location_id}` + Organizations scope, no location specified (defaults to global): `organizations/{org_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
}

# Access deidentify_template outputs
deidentify_template_id = deidentify_template.id
deidentify_template_display_name = deidentify_template.display_name
deidentify_template_create_time = deidentify_template.create_time
deidentify_template_description = deidentify_template.description
deidentify_template_name = deidentify_template.name
deidentify_template_update_time = deidentify_template.update_time
deidentify_template_deidentify_config = deidentify_template.deidentify_config
```

---


### Column_data_profile

Gets a column data profile.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `table_id` | String | The table ID. |
| `estimated_uniqueness_score` | String | Approximate uniqueness of the column. |
| `dataset_id` | String | The BigQuery dataset ID, if the resource profiled is a BigQuery table. |
| `profile_status` | String | Success or error status from the most recent profile generation attempt. May be empty if the profile is still being generated. |
| `dataset_project_id` | String | The Google Cloud project ID that owns the profiled resource. |
| `name` | String | The name of the profile. |
| `free_text_score` | f64 | The likelihood that this column contains free-form text. A value close to 1 may indicate the column is likely to contain free-form or natural language text. Range in 0-1. |
| `dataset_location` | String | If supported, the location where the dataset's data is stored. See https://cloud.google.com/bigquery/docs/locations for supported BigQuery locations. |
| `table_data_profile` | String | The resource name of the table data profile. |
| `profile_last_generated` | String | The last time the profile was generated. |
| `table_full_resource` | String | The resource name of the resource this column is within. |
| `other_matches` | Vec<String> | Other types found within this column. List will be unordered. |
| `sensitivity_score` | String | The sensitivity of this column. |
| `state` | String | State of a profile. |
| `estimated_null_percentage` | String | Approximate percentage of entries being null in the column. |
| `data_risk_level` | String | The data risk level for this column. |
| `column` | String | The name of the column. |
| `policy_state` | String | Indicates if a policy tag has been applied to the column. |
| `column_type` | String | The data type of a given column. |
| `column_info_type` | String | If it's been determined this column can be identified as a single type, this will be set. Otherwise the column either has unidentifiable content or mixed types. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access column_data_profile outputs
column_data_profile_id = column_data_profile.id
column_data_profile_table_id = column_data_profile.table_id
column_data_profile_estimated_uniqueness_score = column_data_profile.estimated_uniqueness_score
column_data_profile_dataset_id = column_data_profile.dataset_id
column_data_profile_profile_status = column_data_profile.profile_status
column_data_profile_dataset_project_id = column_data_profile.dataset_project_id
column_data_profile_name = column_data_profile.name
column_data_profile_free_text_score = column_data_profile.free_text_score
column_data_profile_dataset_location = column_data_profile.dataset_location
column_data_profile_table_data_profile = column_data_profile.table_data_profile
column_data_profile_profile_last_generated = column_data_profile.profile_last_generated
column_data_profile_table_full_resource = column_data_profile.table_full_resource
column_data_profile_other_matches = column_data_profile.other_matches
column_data_profile_sensitivity_score = column_data_profile.sensitivity_score
column_data_profile_state = column_data_profile.state
column_data_profile_estimated_null_percentage = column_data_profile.estimated_null_percentage
column_data_profile_data_risk_level = column_data_profile.data_risk_level
column_data_profile_column = column_data_profile.column
column_data_profile_policy_state = column_data_profile.policy_state
column_data_profile_column_type = column_data_profile.column_type
column_data_profile_column_info_type = column_data_profile.column_info_type
```

---


### Discovery_config

Creates a config for discovery to scan and profile storage.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `config_id` | String |  | The config ID can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one. |
| `discovery_config` | String |  | Required. The DiscoveryConfig to create. |
| `parent` | String | ✅ | Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization): + Projects scope: `projects/{project_id}/locations/{location_id}` + Organizations scope: `organizations/{org_id}/locations/{location_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `org_config` | String | Only set when the parent is an org. |
| `status` | String | Required. A status for this configuration. |
| `name` | String | Unique resource name for the DiscoveryConfig, assigned by the service when the DiscoveryConfig is created, for example `projects/dlp-test-project/locations/global/discoveryConfigs/53234423`. |
| `create_time` | String | Output only. The creation timestamp of a DiscoveryConfig. |
| `update_time` | String | Output only. The last update timestamp of a DiscoveryConfig. |
| `actions` | Vec<String> | Actions to execute at the completion of scanning. |
| `targets` | Vec<String> | Target to match against for determining what to scan and how frequently. |
| `other_cloud_starting_location` | String | Must be set only when scanning other clouds. |
| `processing_location` | String | Optional. Processing location configuration. Vertex AI dataset scanning will set processing_location.image_fallback_type to MultiRegionProcessing by default. |
| `display_name` | String | Display name (max 100 chars) |
| `errors` | Vec<String> | Output only. A stream of errors encountered when the config was activated. Repeated errors may result in the config automatically being paused. Output only field. Will return the last 100 errors. Whenever the config is modified this list will be cleared. |
| `last_run_time` | String | Output only. The timestamp of the last time this config was executed. |
| `inspect_templates` | Vec<String> | Detection logic for profile generation. Not all template features are used by Discovery. FindingLimits, include_quote and exclude_info_types have no impact on Discovery. Multiple templates may be provided if there is data in multiple regions. At most one template must be specified per-region (including "global"). Each region is scanned using the applicable template. If no region-specific template is specified, but a "global" template is specified, it will be copied to that region and used instead. If no global or region-specific template is provided for a region with data, that region's data will not be scanned. For more information, see https://cloud.google.com/sensitive-data-protection/docs/data-profiles#data-residency. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create discovery_config
discovery_config = provider.dlp_api.Discovery_config {
    parent = "value"  # Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization): + Projects scope: `projects/{project_id}/locations/{location_id}` + Organizations scope: `organizations/{org_id}/locations/{location_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
}

# Access discovery_config outputs
discovery_config_id = discovery_config.id
discovery_config_org_config = discovery_config.org_config
discovery_config_status = discovery_config.status
discovery_config_name = discovery_config.name
discovery_config_create_time = discovery_config.create_time
discovery_config_update_time = discovery_config.update_time
discovery_config_actions = discovery_config.actions
discovery_config_targets = discovery_config.targets
discovery_config_other_cloud_starting_location = discovery_config.other_cloud_starting_location
discovery_config_processing_location = discovery_config.processing_location
discovery_config_display_name = discovery_config.display_name
discovery_config_errors = discovery_config.errors
discovery_config_last_run_time = discovery_config.last_run_time
discovery_config_inspect_templates = discovery_config.inspect_templates
```

---


### Inspect_template

Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/sensitive-data-protection/docs/creating-templates to learn more.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inspect_template` | String |  | Required. The InspectTemplate to create. |
| `location_id` | String |  | Deprecated. This field has no effect. |
| `template_id` | String |  | The template id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one. |
| `parent` | String | ✅ | Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` + Organizations scope, location specified: `organizations/{org_id}/locations/{location_id}` + Organizations scope, no location specified (defaults to global): `organizations/{org_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The creation timestamp of an inspectTemplate. |
| `name` | String | Output only. The template name. The template will have one of the following formats: `projects/PROJECT_ID/inspectTemplates/TEMPLATE_ID` OR `organizations/ORGANIZATION_ID/inspectTemplates/TEMPLATE_ID`; |
| `description` | String | Short description (max 256 chars). |
| `inspect_config` | String | The core content of the template. Configuration of the scanning process. |
| `update_time` | String | Output only. The last update timestamp of an inspectTemplate. |
| `display_name` | String | Display name (max 256 chars). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create inspect_template
inspect_template = provider.dlp_api.Inspect_template {
    parent = "value"  # Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` + Organizations scope, location specified: `organizations/{org_id}/locations/{location_id}` + Organizations scope, no location specified (defaults to global): `organizations/{org_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
}

# Access inspect_template outputs
inspect_template_id = inspect_template.id
inspect_template_create_time = inspect_template.create_time
inspect_template_name = inspect_template.name
inspect_template_description = inspect_template.description
inspect_template_inspect_config = inspect_template.inspect_config
inspect_template_update_time = inspect_template.update_time
inspect_template_display_name = inspect_template.display_name
```

---


### Dlp_job

Creates a new job to inspect storage or calculate risk metrics. See https://cloud.google.com/sensitive-data-protection/docs/inspecting-storage and https://cloud.google.com/sensitive-data-protection/docs/compute-risk-analysis to learn more. When no InfoTypes or CustomInfoTypes are specified in inspect jobs, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inspect_job` | String |  | An inspection job scans a storage repository for InfoTypes. |
| `job_id` | String |  | The job id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one. |
| `location_id` | String |  | Deprecated. This field has no effect. |
| `risk_job` | String |  | A risk analysis job calculates re-identification risk metrics for a BigQuery table. |
| `parent` | String | ✅ | Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action_details` | Vec<String> | Events that should occur after the job has completed. |
| `create_time` | String | Time when the job was created. |
| `state` | String | State of a job. |
| `type` | String | The type of job. |
| `name` | String | The server-assigned name. |
| `job_trigger_name` | String | If created by a job trigger, the resource name of the trigger that instantiated the job. |
| `last_modified` | String | Time when the job was last modified by the system. |
| `inspect_details` | String | Results from inspecting a data source. |
| `errors` | Vec<String> | A stream of errors encountered running the job. |
| `risk_details` | String | Results from analyzing risk of a data source. |
| `end_time` | String | Time when the job finished. |
| `start_time` | String | Time when the job started. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dlp_job
dlp_job = provider.dlp_api.Dlp_job {
    parent = "value"  # Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
}

# Access dlp_job outputs
dlp_job_id = dlp_job.id
dlp_job_action_details = dlp_job.action_details
dlp_job_create_time = dlp_job.create_time
dlp_job_state = dlp_job.state
dlp_job_type = dlp_job.type
dlp_job_name = dlp_job.name
dlp_job_job_trigger_name = dlp_job.job_trigger_name
dlp_job_last_modified = dlp_job.last_modified
dlp_job_inspect_details = dlp_job.inspect_details
dlp_job_errors = dlp_job.errors
dlp_job_risk_details = dlp_job.risk_details
dlp_job_end_time = dlp_job.end_time
dlp_job_start_time = dlp_job.start_time
```

---


### Connection

Create a Connection to an external data source.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connection` | String |  | Required. The connection resource. |
| `parent` | String | ✅ | Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization): + Projects scope: `projects/{project_id}/locations/{location_id}` + Organizations scope: `organizations/{org_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_sql` | String | Connect to a Cloud SQL instance. |
| `errors` | Vec<String> | Output only. Set if status == ERROR, to provide additional details. Will store the last 10 errors sorted with the most recent first. |
| `name` | String | Output only. Name of the connection: `projects/{project}/locations/{location}/connections/{name}`. |
| `state` | String | Required. The connection's state in its lifecycle. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection
connection = provider.dlp_api.Connection {
    parent = "value"  # Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization): + Projects scope: `projects/{project_id}/locations/{location_id}` + Organizations scope: `organizations/{org_id}/locations/{location_id}`
}

# Access connection outputs
connection_id = connection.id
connection_cloud_sql = connection.cloud_sql
connection_errors = connection.errors
connection_name = connection.name
connection_state = connection.state
```

---


### Job_trigger

Creates a job trigger to run DLP actions such as scanning storage for sensitive information on a set schedule. See https://cloud.google.com/sensitive-data-protection/docs/creating-job-triggers to learn more.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location_id` | String |  | Deprecated. This field has no effect. |
| `trigger_id` | String |  | The trigger id can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one. |
| `job_trigger` | String |  | Required. The JobTrigger to create. |
| `parent` | String | ✅ | Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | User provided description (max 256 chars) |
| `status` | String | Required. A status for this trigger. |
| `update_time` | String | Output only. The last update timestamp of a triggeredJob. |
| `name` | String | Unique resource name for the triggeredJob, assigned by the service when the triggeredJob is created, for example `projects/dlp-test-project/jobTriggers/53234423`. |
| `inspect_job` | String | For inspect jobs, a snapshot of the configuration. |
| `display_name` | String | Display name (max 100 chars) |
| `last_run_time` | String | Output only. The timestamp of the last time this trigger executed. |
| `errors` | Vec<String> | Output only. A stream of errors encountered when the trigger was activated. Repeated errors may result in the JobTrigger automatically being paused. Will return the last 100 errors. Whenever the JobTrigger is modified this list will be cleared. |
| `create_time` | String | Output only. The creation timestamp of a triggeredJob. |
| `triggers` | Vec<String> | A list of triggers which will be OR'ed together. Only one in the list needs to trigger for a job to be started. The list may contain only a single Schedule trigger and must have at least one object. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job_trigger
job_trigger = provider.dlp_api.Job_trigger {
    parent = "value"  # Required. Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
}

# Access job_trigger outputs
job_trigger_id = job_trigger.id
job_trigger_description = job_trigger.description
job_trigger_status = job_trigger.status
job_trigger_update_time = job_trigger.update_time
job_trigger_name = job_trigger.name
job_trigger_inspect_job = job_trigger.inspect_job
job_trigger_display_name = job_trigger.display_name
job_trigger_last_run_time = job_trigger.last_run_time
job_trigger_errors = job_trigger.errors
job_trigger_create_time = job_trigger.create_time
job_trigger_triggers = job_trigger.triggers
```

---


### Project_data_profile

Gets a project data profile.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `profile_last_generated` | String | The last time the profile was generated. |
| `data_risk_level` | String | The data risk level of this project. |
| `file_store_data_profile_count` | String | The number of file store data profiles generated for this project. |
| `name` | String | The resource name of the profile. |
| `project_id` | String | Project ID or account that was profiled. |
| `sensitivity_score` | String | The sensitivity score of this project. |
| `table_data_profile_count` | String | The number of table data profiles generated for this project. |
| `profile_status` | String | Success or error status of the last attempt to profile the project. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access project_data_profile outputs
project_data_profile_id = project_data_profile.id
project_data_profile_profile_last_generated = project_data_profile.profile_last_generated
project_data_profile_data_risk_level = project_data_profile.data_risk_level
project_data_profile_file_store_data_profile_count = project_data_profile.file_store_data_profile_count
project_data_profile_name = project_data_profile.name
project_data_profile_project_id = project_data_profile.project_id
project_data_profile_sensitivity_score = project_data_profile.sensitivity_score
project_data_profile_table_data_profile_count = project_data_profile.table_data_profile_count
project_data_profile_profile_status = project_data_profile.profile_status
```

---


### Table_data_profile

Gets a table data profile.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_visibility` | String | How broadly a resource has been shared. |
| `name` | String | The name of the profile. |
| `scanned_column_count` | String | The number of columns profiled in the table. |
| `full_resource` | String | The Cloud Asset Inventory resource that was profiled in order to generate this TableDataProfile. https://cloud.google.com/apis/design/resource_names#full_resource_name |
| `dataset_id` | String | If the resource is BigQuery, the dataset ID. |
| `config_snapshot` | String | The snapshot of the configurations used to generate the profile. |
| `predicted_info_types` | Vec<String> | The infoTypes predicted from this table's data. |
| `resource_labels` | HashMap<String, String> | The labels applied to the resource at the time the profile was generated. |
| `row_count` | String | Number of rows in the table when the profile was generated. This will not be populated for BigLake tables. |
| `dataset_project_id` | String | The Google Cloud project ID that owns the resource. |
| `data_source_type` | String | The resource type that was profiled. |
| `expiration_time` | String | Optional. The time when this table expires. |
| `create_time` | String | The time at which the table was created. |
| `domains` | Vec<String> | Domains associated with the profile. |
| `table_size_bytes` | String | The size of the table when the profile was generated. |
| `table_id` | String | The table ID. |
| `tags` | Vec<String> | The tags attached to the table, including any tags attached during profiling. Because tags are attached to Cloud SQL instances rather than Cloud SQL tables, this field is empty for Cloud SQL table profiles. |
| `other_info_types` | Vec<String> | Other infoTypes found in this table's data. |
| `sample_findings_table` | String | The BigQuery table to which the sample findings are written. |
| `profile_last_generated` | String | The last time the profile was generated. |
| `last_modified_time` | String | The time when this table was last modified |
| `state` | String | State of a profile. This will always be set to DONE when the table data profile is written to another service like BigQuery or Pub/Sub. |
| `encryption_status` | String | How the table is encrypted. |
| `failed_column_count` | String | The number of columns skipped in the table because of an error. |
| `profile_status` | String | Success or error status from the most recent profile generation attempt. May be empty if the profile is still being generated. |
| `project_data_profile` | String | The resource name of the project data profile for this table. |
| `data_risk_level` | String | The data risk level of this table. |
| `related_resources` | Vec<String> | Resources related to this profile. |
| `dataset_location` | String | If supported, the location where the dataset's data is stored. See https://cloud.google.com/bigquery/docs/locations for supported locations. |
| `sensitivity_score` | String | The sensitivity score of this table. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access table_data_profile outputs
table_data_profile_id = table_data_profile.id
table_data_profile_resource_visibility = table_data_profile.resource_visibility
table_data_profile_name = table_data_profile.name
table_data_profile_scanned_column_count = table_data_profile.scanned_column_count
table_data_profile_full_resource = table_data_profile.full_resource
table_data_profile_dataset_id = table_data_profile.dataset_id
table_data_profile_config_snapshot = table_data_profile.config_snapshot
table_data_profile_predicted_info_types = table_data_profile.predicted_info_types
table_data_profile_resource_labels = table_data_profile.resource_labels
table_data_profile_row_count = table_data_profile.row_count
table_data_profile_dataset_project_id = table_data_profile.dataset_project_id
table_data_profile_data_source_type = table_data_profile.data_source_type
table_data_profile_expiration_time = table_data_profile.expiration_time
table_data_profile_create_time = table_data_profile.create_time
table_data_profile_domains = table_data_profile.domains
table_data_profile_table_size_bytes = table_data_profile.table_size_bytes
table_data_profile_table_id = table_data_profile.table_id
table_data_profile_tags = table_data_profile.tags
table_data_profile_other_info_types = table_data_profile.other_info_types
table_data_profile_sample_findings_table = table_data_profile.sample_findings_table
table_data_profile_profile_last_generated = table_data_profile.profile_last_generated
table_data_profile_last_modified_time = table_data_profile.last_modified_time
table_data_profile_state = table_data_profile.state
table_data_profile_encryption_status = table_data_profile.encryption_status
table_data_profile_failed_column_count = table_data_profile.failed_column_count
table_data_profile_profile_status = table_data_profile.profile_status
table_data_profile_project_data_profile = table_data_profile.project_data_profile
table_data_profile_data_risk_level = table_data_profile.data_risk_level
table_data_profile_related_resources = table_data_profile.related_resources
table_data_profile_dataset_location = table_data_profile.dataset_location
table_data_profile_sensitivity_score = table_data_profile.sensitivity_score
```

---


### Content

De-identifies potentially sensitive info from a ContentItem. This method has limits on input size and output size. See https://cloud.google.com/sensitive-data-protection/docs/deidentify-sensitive-data to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deidentify_template_name` | String |  | Template to use. Any configuration directly specified in deidentify_config will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged. |
| `item` | String |  | The item to de-identify. Will be treated as text. This value must be of type Table if your deidentify_config is a RecordTransformations object. |
| `inspect_config` | String |  | Configuration for the inspector. Items specified here will override the template referenced by the inspect_template_name argument. |
| `location_id` | String |  | Deprecated. This field has no effect. |
| `deidentify_config` | String |  | Configuration for the de-identification of the content item. Items specified here will override the template referenced by the deidentify_template_name argument. |
| `inspect_template_name` | String |  | Template to use. Any configuration directly specified in inspect_config will override those set in the template. Singular fields that are set in this request will replace their corresponding fields in the template. Repeated fields are appended. Singular sub-messages and groups are recursively merged. |
| `parent` | String | ✅ | Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3 |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create content
content = provider.dlp_api.Content {
    parent = "value"  # Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
}

```

---


### Image

Redacts potentially sensitive info from an image. This method has limits on input size, processing time, and output size. See https://cloud.google.com/sensitive-data-protection/docs/redacting-sensitive-data-images to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated. Only the first frame of each multiframe image is redacted. Metadata and other frames are omitted in the response.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deidentify_template` | String |  | The full resource name of the de-identification template to use. Settings in the main `image_redaction_configs` field override the corresponding settings in this de-identification template. The request fails if the type of the template's deidentify_config is not image_transformations. |
| `image_redaction_configs` | Vec<String> |  | The configuration for specifying what content to redact from images. |
| `include_findings` | bool |  | Whether the response should include findings along with the redacted image. |
| `byte_item` | String |  | The content must be PNG, JPEG, SVG or BMP. |
| `location_id` | String |  | Deprecated. This field has no effect. |
| `inspect_config` | String |  | Configuration for the inspector. |
| `inspect_template` | String |  | The full resource name of the inspection template to use. Settings in the main `inspect_config` field override the corresponding settings in this inspection template. The merge behavior is as follows: - Singular field: The main field's value replaces the value of the corresponding field in the template. - Repeated fields: The field values are appended to the list defined in the template. - Sub-messages and groups: The fields are recursively merged. |
| `parent` | String | ✅ | Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3 |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create image
image = provider.dlp_api.Image {
    parent = "value"  # Parent resource name. The format of this value varies depending on whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
}

```

---


### Stored_info_type

Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/sensitive-data-protection/docs/creating-stored-infotypes to learn more.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `config` | String |  | Required. Configuration of the storedInfoType to create. |
| `stored_info_type_id` | String |  | The storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens; that is, it must match the regular expression: `[a-zA-Z\d-_]+`. The maximum length is 100 characters. Can be empty to allow the system to generate one. |
| `location_id` | String |  | Deprecated. This field has no effect. |
| `parent` | String | ✅ | Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` + Organizations scope, location specified: `organizations/{org_id}/locations/{location_id}` + Organizations scope, no location specified (defaults to global): `organizations/{org_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `current_version` | String | Current version of the stored info type. |
| `name` | String | Resource name. |
| `pending_versions` | Vec<String> | Pending versions of the stored info type. Empty if no versions are pending. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create stored_info_type
stored_info_type = provider.dlp_api.Stored_info_type {
    parent = "value"  # Required. Parent resource name. The format of this value varies depending on the scope of the request (project or organization) and whether you have [specified a processing location](https://cloud.google.com/sensitive-data-protection/docs/specifying-location): + Projects scope, location specified: `projects/{project_id}/locations/{location_id}` + Projects scope, no location specified (defaults to global): `projects/{project_id}` + Organizations scope, location specified: `organizations/{org_id}/locations/{location_id}` + Organizations scope, no location specified (defaults to global): `organizations/{org_id}` The following example `parent` string specifies a parent project with the identifier `example-project`, and specifies the `europe-west3` location for processing data: parent=projects/example-project/locations/europe-west3
}

# Access stored_info_type outputs
stored_info_type_id = stored_info_type.id
stored_info_type_current_version = stored_info_type.current_version
stored_info_type_name = stored_info_type.name
stored_info_type_pending_versions = stored_info_type.pending_versions
```

---


### File_store_data_profile

Gets a file store data profile.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `config_snapshot` | String | The snapshot of the configurations used to generate the profile. |
| `data_source_type` | String | The resource type that was profiled. |
| `file_store_is_empty` | bool | The file store does not have any files. If the profiling operation failed, this is false. |
| `data_risk_level` | String | The data risk level of this resource. |
| `file_cluster_summaries` | Vec<String> | FileClusterSummary per each cluster. |
| `file_store_path` | String | The file store path. * Cloud Storage: `gs://{bucket}` * Amazon S3: `s3://{bucket}` * Vertex AI dataset: `projects/{project_number}/locations/{location}/datasets/{dataset_id}` |
| `full_resource` | String | The resource name of the resource profiled. https://cloud.google.com/apis/design/resource_names#full_resource_name Example format of an S3 bucket full resource name: `//cloudasset.googleapis.com/organizations/{org_id}/otherCloudConnections/aws/arn:aws:s3:::{bucket_name}` |
| `project_id` | String | The Google Cloud project ID that owns the resource. For Amazon S3 buckets, this is the AWS Account Id. |
| `related_resources` | Vec<String> | Resources related to this profile. |
| `resource_visibility` | String | How broadly a resource has been shared. |
| `tags` | Vec<String> | The tags attached to the resource, including any tags attached during profiling. |
| `resource_labels` | HashMap<String, String> | The labels applied to the resource at the time the profile was generated. |
| `create_time` | String | The time the file store was first created. |
| `file_store_location` | String | The location of the file store. * Cloud Storage: https://cloud.google.com/storage/docs/locations#available-locations * Amazon S3: https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints |
| `profile_status` | String | Success or error status from the most recent profile generation attempt. May be empty if the profile is still being generated. |
| `resource_attributes` | HashMap<String, String> | Attributes of the resource being profiled. Currently used attributes: * customer_managed_encryption: boolean - true: the resource is encrypted with a customer-managed key. - false: the resource is encrypted with a provider-managed key. |
| `state` | String | State of a profile. |
| `file_store_info_type_summaries` | Vec<String> | InfoTypes detected in this file store. |
| `name` | String | The name of the profile. |
| `data_storage_locations` | Vec<String> | For resources that have multiple storage locations, these are those regions. For Cloud Storage this is the list of regions chosen for dual-region storage. `file_store_location` will normally be the corresponding multi-region for the list of individual locations. The first region is always picked as the processing and storage location for the data profile. |
| `last_modified_time` | String | The time the file store was last modified. |
| `domains` | Vec<String> | Domains associated with the profile. |
| `location_type` | String | The location type of the file store (region, dual-region, multi-region, etc). If dual-region, expect data_storage_locations to be populated. |
| `project_data_profile` | String | The resource name of the project data profile for this file store. |
| `sample_findings_table` | String | The BigQuery table to which the sample findings are written. |
| `sensitivity_score` | String | The sensitivity score of this resource. |
| `profile_last_generated` | String | The last time the profile was generated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access file_store_data_profile outputs
file_store_data_profile_id = file_store_data_profile.id
file_store_data_profile_config_snapshot = file_store_data_profile.config_snapshot
file_store_data_profile_data_source_type = file_store_data_profile.data_source_type
file_store_data_profile_file_store_is_empty = file_store_data_profile.file_store_is_empty
file_store_data_profile_data_risk_level = file_store_data_profile.data_risk_level
file_store_data_profile_file_cluster_summaries = file_store_data_profile.file_cluster_summaries
file_store_data_profile_file_store_path = file_store_data_profile.file_store_path
file_store_data_profile_full_resource = file_store_data_profile.full_resource
file_store_data_profile_project_id = file_store_data_profile.project_id
file_store_data_profile_related_resources = file_store_data_profile.related_resources
file_store_data_profile_resource_visibility = file_store_data_profile.resource_visibility
file_store_data_profile_tags = file_store_data_profile.tags
file_store_data_profile_resource_labels = file_store_data_profile.resource_labels
file_store_data_profile_create_time = file_store_data_profile.create_time
file_store_data_profile_file_store_location = file_store_data_profile.file_store_location
file_store_data_profile_profile_status = file_store_data_profile.profile_status
file_store_data_profile_resource_attributes = file_store_data_profile.resource_attributes
file_store_data_profile_state = file_store_data_profile.state
file_store_data_profile_file_store_info_type_summaries = file_store_data_profile.file_store_info_type_summaries
file_store_data_profile_name = file_store_data_profile.name
file_store_data_profile_data_storage_locations = file_store_data_profile.data_storage_locations
file_store_data_profile_last_modified_time = file_store_data_profile.last_modified_time
file_store_data_profile_domains = file_store_data_profile.domains
file_store_data_profile_location_type = file_store_data_profile.location_type
file_store_data_profile_project_data_profile = file_store_data_profile.project_data_profile
file_store_data_profile_sample_findings_table = file_store_data_profile.sample_findings_table
file_store_data_profile_sensitivity_score = file_store_data_profile.sensitivity_score
file_store_data_profile_profile_last_generated = file_store_data_profile.profile_last_generated
```

---


### Info_type

Returns a list of the sensitive information types that the DLP API supports. See https://cloud.google.com/sensitive-data-protection/docs/infotypes-reference to learn more.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `info_types` | Vec<String> | Set of sensitive infoTypes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access info_type outputs
info_type_id = info_type.id
info_type_info_types = info_type.info_types
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple deidentify_template resources
deidentify_template_0 = provider.dlp_api.Deidentify_template {
    parent = "value-0"
}
deidentify_template_1 = provider.dlp_api.Deidentify_template {
    parent = "value-1"
}
deidentify_template_2 = provider.dlp_api.Deidentify_template {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    deidentify_template = provider.dlp_api.Deidentify_template {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Dlp_api Documentation](https://cloud.google.com/dlp_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
