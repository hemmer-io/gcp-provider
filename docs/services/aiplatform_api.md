# Aiplatform_api Service



**Resources**: 133

---

## Overview

The aiplatform_api service provides access to 133 resource types:

- [Model](#model) [CRUD]
- [Metadata_schema](#metadata_schema) [CR]
- [Hyperparameter_tuning_job](#hyperparameter_tuning_job) [CRD]
- [Training_pipeline](#training_pipeline) [CRD]
- [Studie](#studie) [CRD]
- [Feature_view](#feature_view) [CRUD]
- [Notebook_execution_job](#notebook_execution_job) [CRD]
- [Slice](#slice) [CR]
- [Data_labeling_job](#data_labeling_job) [CRD]
- [Annotation_spec](#annotation_spec) [R]
- [Chat](#chat) [C]
- [Notebook_runtime](#notebook_runtime) [CRD]
- [Evaluation_set](#evaluation_set) [CRUD]
- [Nas_trial_detail](#nas_trial_detail) [R]
- [Invoke](#invoke) [C]
- [Openapi](#openapi) [C]
- [Evaluation](#evaluation) [CR]
- [Annotation](#annotation) [R]
- [Tensorboard](#tensorboard) [CRUD]
- [Custom_job](#custom_job) [CRD]
- [Rag_file](#rag_file) [CRD]
- [Feature_view_sync](#feature_view_sync) [R]
- [Rag_corpora](#rag_corpora) [CRUD]
- [Feature](#feature) [CRUD]
- [Operation](#operation) [CRD]
- [Notebook_runtime_template](#notebook_runtime_template) [CRUD]
- [Specialist_pool](#specialist_pool) [CRUD]
- [Migratable_resource](#migratable_resource) [C]
- [Index_endpoint](#index_endpoint) [CRUD]
- [Feature_group](#feature_group) [CRUD]
- [Artifact](#artifact) [CRUD]
- [Media](#media) [C]
- [Entity_type](#entity_type) [CRUD]
- [Persistent_resource](#persistent_resource) [CRUD]
- [Dataset](#dataset) [CRUD]
- [Data_item](#data_item) [R]
- [Deployment_resource_pool](#deployment_resource_pool) [CRUD]
- [Evaluation_run](#evaluation_run) [CRD]
- [Trial](#trial) [CRD]
- [Pipeline_job](#pipeline_job) [CRD]
- [Nas_job](#nas_job) [CRD]
- [Endpoint](#endpoint) [CRUD]
- [Saved_querie](#saved_querie) [RD]
- [Indexe](#indexe) [CRUD]
- [Execution](#execution) [CRUD]
- [Schedule](#schedule) [CRUD]
- [Experiment](#experiment) [CRUD]
- [Dataset_version](#dataset_version) [CRUD]
- [Run](#run) [CRUD]
- [Featurestore](#featurestore) [CRUD]
- [Cached_content](#cached_content) [CRUD]
- [Batch_prediction_job](#batch_prediction_job) [CRD]
- [Feature_online_store](#feature_online_store) [CRUD]
- [Reasoning_engine](#reasoning_engine) [CRUD]
- [Context](#context) [CRUD]
- [Location](#location) [CRU]
- [Project](#project) [RU]
- [Time_serie](#time_serie) [CRUD]
- [Tuning_job](#tuning_job) [CR]
- [Model_deployment_monitoring_job](#model_deployment_monitoring_job) [CRUD]
- [Metadata_store](#metadata_store) [CRD]
- [Evaluation_item](#evaluation_item) [CRD]
- [Event](#event) [R]
- [Nas_job](#nas_job) [CRD]
- [Cached_content](#cached_content) [CRUD]
- [Hyperparameter_tuning_job](#hyperparameter_tuning_job) [CRD]
- [Metadata_schema](#metadata_schema) [CR]
- [Feature_online_store](#feature_online_store) [CRUD]
- [Trial](#trial) [CRD]
- [Extension](#extension) [CRUD]
- [Tuning_job](#tuning_job) [CR]
- [Feature_monitor](#feature_monitor) [CRUD]
- [Pipeline_job](#pipeline_job) [CRD]
- [Feature_view](#feature_view) [CRUD]
- [Sandbox_environment](#sandbox_environment) [CRD]
- [Time_serie](#time_serie) [CRUD]
- [Evaluation_item](#evaluation_item) [CRD]
- [Media](#media) [C]
- [Model_deployment_monitoring_job](#model_deployment_monitoring_job) [CRUD]
- [Dataset_version](#dataset_version) [CRUD]
- [Evaluation_set](#evaluation_set) [CRUD]
- [Chat](#chat) [C]
- [Reasoning_engine](#reasoning_engine) [CRUD]
- [Migratable_resource](#migratable_resource) [C]
- [Schedule](#schedule) [CRUD]
- [Artifact](#artifact) [CRUD]
- [Tensorboard](#tensorboard) [CRUD]
- [Feature_monitor_job](#feature_monitor_job) [CR]
- [Notebook_execution_job](#notebook_execution_job) [CRD]
- [Operation](#operation) [CRD]
- [Model](#model) [CRUD]
- [Training_pipeline](#training_pipeline) [CRD]
- [Model_monitor](#model_monitor) [CRUD]
- [Feature_view_sync](#feature_view_sync) [R]
- [Context](#context) [CRUD]
- [Location](#location) [CRU]
- [Session](#session) [CRUD]
- [Feature_group](#feature_group) [CRUD]
- [Model_garden_eula](#model_garden_eula) [C]
- [Deployment_resource_pool](#deployment_resource_pool) [CRUD]
- [Memorie](#memorie) [CRUD]
- [Featurestore](#featurestore) [CRUD]
- [Endpoint](#endpoint) [CRUD]
- [Experiment](#experiment) [CRUD]
- [Persistent_resource](#persistent_resource) [CRUD]
- [Custom_job](#custom_job) [CRD]
- [Saved_querie](#saved_querie) [RD]
- [Evaluation_run](#evaluation_run) [CRD]
- [Index_endpoint](#index_endpoint) [CRUD]
- [Notebook_runtime_template](#notebook_runtime_template) [CRUD]
- [Dataset](#dataset) [CRUD]
- [Notebook_runtime](#notebook_runtime) [CRD]
- [Project](#project) [CRU]
- [Rag_corpora](#rag_corpora) [CRUD]
- [Indexe](#indexe) [CRUD]
- [Example_store](#example_store) [CRUD]
- [Data_item](#data_item) [R]
- [Annotation](#annotation) [R]
- [Model_monitoring_job](#model_monitoring_job) [CRD]
- [Execution](#execution) [CRUD]
- [Batch_prediction_job](#batch_prediction_job) [CRD]
- [Run](#run) [CRUD]
- [Slice](#slice) [CR]
- [Nas_trial_detail](#nas_trial_detail) [R]
- [Specialist_pool](#specialist_pool) [CRUD]
- [Data_labeling_job](#data_labeling_job) [CRD]
- [Feature](#feature) [CRUD]
- [Metadata_store](#metadata_store) [CRD]
- [Studie](#studie) [CRD]
- [Entity_type](#entity_type) [CRUD]
- [Annotation_spec](#annotation_spec) [R]
- [Evaluation](#evaluation) [CR]
- [Rag_file](#rag_file) [CRD]

---

## Resources


### Model



**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | String |  | Optional. The parameters that govern the prediction. The schema of the parameters may be specified via Endpoint's DeployedModels' Model's PredictSchemata's parameters_schema_uri. |
| `instances` | Vec<String> |  | Required. The instances that are the input to the prediction call. A DeployedModel may have an upper limit on the number of instances it supports per request, and when it is exceeded the prediction call errors in case of AutoML Models, or, in case of customer created Models, the behaviour is as documented by that Model. The schema of any single instance may be specified via Endpoint's DeployedModels' Model's PredictSchemata's instance_schema_uri. |
| `endpoint` | String | ✅ | Required. The name of the Endpoint requested to serve the prediction. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}` or `projects/{project}/locations/{location}/publishers/{publisher}/models/{model}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `frameworks` | Vec<String> | Optional. Additional information about the model's Frameworks. |
| `name` | String | Output only. The resource name of the PublisherModel. |
| `launch_stage` | String | Optional. Indicates the launch stage of the model. |
| `supported_actions` | String | Optional. Supported call-to-action options. |
| `version_id` | String | Output only. Immutable. The version ID of the PublisherModel. A new version is committed when a new model version is uploaded under an existing model id. It is an auto-incrementing decimal number in string representation. |
| `version_state` | String | Optional. Indicates the state of the model version. |
| `publisher_model_template` | String | Optional. Output only. Immutable. Used to indicate this model has a publisher model and provide the template of the publisher model resource name. |
| `open_source_category` | String | Required. Indicates the open source category of the publisher model. |
| `predict_schemata` | String | Optional. The schemata that describes formats of the PublisherModel's predictions and explanations as given and returned via PredictionService.Predict. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model
model = provider.aiplatform_api.Model {
    endpoint = "value"  # Required. The name of the Endpoint requested to serve the prediction. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}` or `projects/{project}/locations/{location}/publishers/{publisher}/models/{model}`
}

# Access model outputs
model_id = model.id
model_frameworks = model.frameworks
model_name = model.name
model_launch_stage = model.launch_stage
model_supported_actions = model.supported_actions
model_version_id = model.version_id
model_version_state = model.version_state
model_publisher_model_template = model.publisher_model_template
model_open_source_category = model.open_source_category
model_predict_schemata = model.predict_schemata
```

---


### Metadata_schema

Creates a MetadataSchema.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_type` | String |  | The type of the MetadataSchema. This is a property that identifies which metadata types will use the MetadataSchema. |
| `description` | String |  | Description of the Metadata Schema |
| `schema` | String |  | Required. The raw YAML string representation of the MetadataSchema. The combination of [MetadataSchema.version] and the schema name given by `title` in [MetadataSchema.schema] must be unique within a MetadataStore. The schema is defined as an OpenAPI 3.0.2 [MetadataSchema Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.2.md#schemaObject) |
| `name` | String |  | Output only. The resource name of the MetadataSchema. |
| `create_time` | String |  | Output only. Timestamp when this MetadataSchema was created. |
| `schema_version` | String |  | The version of the MetadataSchema. The version's format must match the following regular expression: `^[0-9]+.+.+$`, which would allow to order/compare different versions. Example: 1.0.0, 1.0.1, etc. |
| `parent` | String | ✅ | Required. The resource name of the MetadataStore where the MetadataSchema should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema_type` | String | The type of the MetadataSchema. This is a property that identifies which metadata types will use the MetadataSchema. |
| `description` | String | Description of the Metadata Schema |
| `schema` | String | Required. The raw YAML string representation of the MetadataSchema. The combination of [MetadataSchema.version] and the schema name given by `title` in [MetadataSchema.schema] must be unique within a MetadataStore. The schema is defined as an OpenAPI 3.0.2 [MetadataSchema Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.2.md#schemaObject) |
| `name` | String | Output only. The resource name of the MetadataSchema. |
| `create_time` | String | Output only. Timestamp when this MetadataSchema was created. |
| `schema_version` | String | The version of the MetadataSchema. The version's format must match the following regular expression: `^[0-9]+.+.+$`, which would allow to order/compare different versions. Example: 1.0.0, 1.0.1, etc. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create metadata_schema
metadata_schema = provider.aiplatform_api.Metadata_schema {
    parent = "value"  # Required. The resource name of the MetadataStore where the MetadataSchema should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}`
}

# Access metadata_schema outputs
metadata_schema_id = metadata_schema.id
metadata_schema_schema_type = metadata_schema.schema_type
metadata_schema_description = metadata_schema.description
metadata_schema_schema = metadata_schema.schema
metadata_schema_name = metadata_schema.name
metadata_schema_create_time = metadata_schema.create_time
metadata_schema_schema_version = metadata_schema.schema_version
```

---


### Hyperparameter_tuning_job

Creates a HyperparameterTuningJob

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The display name of the HyperparameterTuningJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `update_time` | String |  | Output only. Time when the HyperparameterTuningJob was most recently updated. |
| `create_time` | String |  | Output only. Time when the HyperparameterTuningJob was created. |
| `max_failed_trial_count` | i64 |  | The number of failed Trials that need to be seen before failing the HyperparameterTuningJob. If set to 0, Vertex AI decides how many Trials must fail before the whole job fails. |
| `encryption_spec` | String |  | Customer-managed encryption key options for a HyperparameterTuningJob. If this is set, then all resources created by the HyperparameterTuningJob will be encrypted with the provided encryption key. |
| `error` | String |  | Output only. Only populated when job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `trials` | Vec<String> |  | Output only. Trials of the HyperparameterTuningJob. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `end_time` | String |  | Output only. Time when the HyperparameterTuningJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `state` | String |  | Output only. The detailed state of the job. |
| `trial_job_spec` | String |  | Required. The spec of a trial job. The same spec applies to the CustomJobs created in all the trials. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize HyperparameterTuningJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `max_trial_count` | i64 |  | Required. The desired total number of Trials. |
| `study_spec` | String |  | Required. Study configuration of the HyperparameterTuningJob. |
| `start_time` | String |  | Output only. Time when the HyperparameterTuningJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `name` | String |  | Output only. Resource name of the HyperparameterTuningJob. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `parallel_trial_count` | i64 |  | Required. The desired number of Trials to run in parallel. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the HyperparameterTuningJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the HyperparameterTuningJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `update_time` | String | Output only. Time when the HyperparameterTuningJob was most recently updated. |
| `create_time` | String | Output only. Time when the HyperparameterTuningJob was created. |
| `max_failed_trial_count` | i64 | The number of failed Trials that need to be seen before failing the HyperparameterTuningJob. If set to 0, Vertex AI decides how many Trials must fail before the whole job fails. |
| `encryption_spec` | String | Customer-managed encryption key options for a HyperparameterTuningJob. If this is set, then all resources created by the HyperparameterTuningJob will be encrypted with the provided encryption key. |
| `error` | String | Output only. Only populated when job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `trials` | Vec<String> | Output only. Trials of the HyperparameterTuningJob. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `end_time` | String | Output only. Time when the HyperparameterTuningJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `state` | String | Output only. The detailed state of the job. |
| `trial_job_spec` | String | Required. The spec of a trial job. The same spec applies to the CustomJobs created in all the trials. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize HyperparameterTuningJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `max_trial_count` | i64 | Required. The desired total number of Trials. |
| `study_spec` | String | Required. Study configuration of the HyperparameterTuningJob. |
| `start_time` | String | Output only. Time when the HyperparameterTuningJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `name` | String | Output only. Resource name of the HyperparameterTuningJob. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `parallel_trial_count` | i64 | Required. The desired number of Trials to run in parallel. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create hyperparameter_tuning_job
hyperparameter_tuning_job = provider.aiplatform_api.Hyperparameter_tuning_job {
    parent = "value"  # Required. The resource name of the Location to create the HyperparameterTuningJob in. Format: `projects/{project}/locations/{location}`
}

# Access hyperparameter_tuning_job outputs
hyperparameter_tuning_job_id = hyperparameter_tuning_job.id
hyperparameter_tuning_job_display_name = hyperparameter_tuning_job.display_name
hyperparameter_tuning_job_update_time = hyperparameter_tuning_job.update_time
hyperparameter_tuning_job_create_time = hyperparameter_tuning_job.create_time
hyperparameter_tuning_job_max_failed_trial_count = hyperparameter_tuning_job.max_failed_trial_count
hyperparameter_tuning_job_encryption_spec = hyperparameter_tuning_job.encryption_spec
hyperparameter_tuning_job_error = hyperparameter_tuning_job.error
hyperparameter_tuning_job_trials = hyperparameter_tuning_job.trials
hyperparameter_tuning_job_satisfies_pzi = hyperparameter_tuning_job.satisfies_pzi
hyperparameter_tuning_job_end_time = hyperparameter_tuning_job.end_time
hyperparameter_tuning_job_state = hyperparameter_tuning_job.state
hyperparameter_tuning_job_trial_job_spec = hyperparameter_tuning_job.trial_job_spec
hyperparameter_tuning_job_labels = hyperparameter_tuning_job.labels
hyperparameter_tuning_job_max_trial_count = hyperparameter_tuning_job.max_trial_count
hyperparameter_tuning_job_study_spec = hyperparameter_tuning_job.study_spec
hyperparameter_tuning_job_start_time = hyperparameter_tuning_job.start_time
hyperparameter_tuning_job_name = hyperparameter_tuning_job.name
hyperparameter_tuning_job_satisfies_pzs = hyperparameter_tuning_job.satisfies_pzs
hyperparameter_tuning_job_parallel_trial_count = hyperparameter_tuning_job.parallel_trial_count
```

---


### Training_pipeline

Creates a TrainingPipeline. A created TrainingPipeline right away will be attempted to be run.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_to_upload` | String |  | Describes the Model that may be uploaded (via ModelService.UploadModel) by this TrainingPipeline. The TrainingPipeline's training_task_definition should make clear whether this Model description should be populated, and if there are any special requirements regarding how it should be filled. If nothing is mentioned in the training_task_definition, then it should be assumed that this field should not be filled and the training task either uploads the Model without a need of this information, or that training task does not support uploading a Model as part of the pipeline. When the Pipeline's state becomes `PIPELINE_STATE_SUCCEEDED` and the trained Model had been uploaded into Vertex AI, then the model_to_upload's resource name is populated. The Model is always uploaded into the Project and Location in which this pipeline is. |
| `start_time` | String |  | Output only. Time when the TrainingPipeline for the first time entered the `PIPELINE_STATE_RUNNING` state. |
| `input_data_config` | String |  | Specifies Vertex AI owned input data that may be used for training the Model. The TrainingPipeline's training_task_definition should make clear whether this config is used and if there are any special requirements on how it should be filled. If nothing about this config is mentioned in the training_task_definition, then it should be assumed that the TrainingPipeline does not depend on this configuration. |
| `training_task_metadata` | String |  | Output only. The metadata information as specified in the training_task_definition's `metadata`. This metadata is an auxiliary runtime and final information about the training task. While the pipeline is running this information is populated only at a best effort basis. Only present if the pipeline's training_task_definition contains `metadata` object. |
| `update_time` | String |  | Output only. Time when the TrainingPipeline was most recently updated. |
| `state` | String |  | Output only. The detailed state of the pipeline. |
| `end_time` | String |  | Output only. Time when the TrainingPipeline entered any of the following states: `PIPELINE_STATE_SUCCEEDED`, `PIPELINE_STATE_FAILED`, `PIPELINE_STATE_CANCELLED`. |
| `parent_model` | String |  | Optional. When specify this field, the `model_to_upload` will not be uploaded as a new model, instead, it will become a new version of this `parent_model`. |
| `training_task_inputs` | String |  | Required. The training task's parameter(s), as specified in the training_task_definition's `inputs`. |
| `display_name` | String |  | Required. The user-defined name of this TrainingPipeline. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize TrainingPipelines. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `error` | String |  | Output only. Only populated when the pipeline's state is `PIPELINE_STATE_FAILED` or `PIPELINE_STATE_CANCELLED`. |
| `model_id` | String |  | Optional. The ID to use for the uploaded Model, which will become the final component of the model resource name. This value may be up to 63 characters, and valid characters are `[a-z0-9_-]`. The first character cannot be a number or hyphen. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a TrainingPipeline. If set, this TrainingPipeline will be secured by this key. Note: Model trained by this TrainingPipeline is also secured by this key if model_to_upload is not set separately. |
| `training_task_definition` | String |  | Required. A Google Cloud Storage path to the YAML file that defines the training task which is responsible for producing the model artifact, and may also include additional auxiliary work. The definition files that can be used here are found in gs://google-cloud-aiplatform/schema/trainingjob/definition/. Note: The URI given on output will be immutable and probably different, including the URI scheme, than the one given on input. The output URI will point to a location where the user only has a read access. |
| `create_time` | String |  | Output only. Time when the TrainingPipeline was created. |
| `name` | String |  | Output only. Resource name of the TrainingPipeline. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the TrainingPipeline in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_to_upload` | String | Describes the Model that may be uploaded (via ModelService.UploadModel) by this TrainingPipeline. The TrainingPipeline's training_task_definition should make clear whether this Model description should be populated, and if there are any special requirements regarding how it should be filled. If nothing is mentioned in the training_task_definition, then it should be assumed that this field should not be filled and the training task either uploads the Model without a need of this information, or that training task does not support uploading a Model as part of the pipeline. When the Pipeline's state becomes `PIPELINE_STATE_SUCCEEDED` and the trained Model had been uploaded into Vertex AI, then the model_to_upload's resource name is populated. The Model is always uploaded into the Project and Location in which this pipeline is. |
| `start_time` | String | Output only. Time when the TrainingPipeline for the first time entered the `PIPELINE_STATE_RUNNING` state. |
| `input_data_config` | String | Specifies Vertex AI owned input data that may be used for training the Model. The TrainingPipeline's training_task_definition should make clear whether this config is used and if there are any special requirements on how it should be filled. If nothing about this config is mentioned in the training_task_definition, then it should be assumed that the TrainingPipeline does not depend on this configuration. |
| `training_task_metadata` | String | Output only. The metadata information as specified in the training_task_definition's `metadata`. This metadata is an auxiliary runtime and final information about the training task. While the pipeline is running this information is populated only at a best effort basis. Only present if the pipeline's training_task_definition contains `metadata` object. |
| `update_time` | String | Output only. Time when the TrainingPipeline was most recently updated. |
| `state` | String | Output only. The detailed state of the pipeline. |
| `end_time` | String | Output only. Time when the TrainingPipeline entered any of the following states: `PIPELINE_STATE_SUCCEEDED`, `PIPELINE_STATE_FAILED`, `PIPELINE_STATE_CANCELLED`. |
| `parent_model` | String | Optional. When specify this field, the `model_to_upload` will not be uploaded as a new model, instead, it will become a new version of this `parent_model`. |
| `training_task_inputs` | String | Required. The training task's parameter(s), as specified in the training_task_definition's `inputs`. |
| `display_name` | String | Required. The user-defined name of this TrainingPipeline. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize TrainingPipelines. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `error` | String | Output only. Only populated when the pipeline's state is `PIPELINE_STATE_FAILED` or `PIPELINE_STATE_CANCELLED`. |
| `model_id` | String | Optional. The ID to use for the uploaded Model, which will become the final component of the model resource name. This value may be up to 63 characters, and valid characters are `[a-z0-9_-]`. The first character cannot be a number or hyphen. |
| `encryption_spec` | String | Customer-managed encryption key spec for a TrainingPipeline. If set, this TrainingPipeline will be secured by this key. Note: Model trained by this TrainingPipeline is also secured by this key if model_to_upload is not set separately. |
| `training_task_definition` | String | Required. A Google Cloud Storage path to the YAML file that defines the training task which is responsible for producing the model artifact, and may also include additional auxiliary work. The definition files that can be used here are found in gs://google-cloud-aiplatform/schema/trainingjob/definition/. Note: The URI given on output will be immutable and probably different, including the URI scheme, than the one given on input. The output URI will point to a location where the user only has a read access. |
| `create_time` | String | Output only. Time when the TrainingPipeline was created. |
| `name` | String | Output only. Resource name of the TrainingPipeline. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create training_pipeline
training_pipeline = provider.aiplatform_api.Training_pipeline {
    parent = "value"  # Required. The resource name of the Location to create the TrainingPipeline in. Format: `projects/{project}/locations/{location}`
}

# Access training_pipeline outputs
training_pipeline_id = training_pipeline.id
training_pipeline_model_to_upload = training_pipeline.model_to_upload
training_pipeline_start_time = training_pipeline.start_time
training_pipeline_input_data_config = training_pipeline.input_data_config
training_pipeline_training_task_metadata = training_pipeline.training_task_metadata
training_pipeline_update_time = training_pipeline.update_time
training_pipeline_state = training_pipeline.state
training_pipeline_end_time = training_pipeline.end_time
training_pipeline_parent_model = training_pipeline.parent_model
training_pipeline_training_task_inputs = training_pipeline.training_task_inputs
training_pipeline_display_name = training_pipeline.display_name
training_pipeline_labels = training_pipeline.labels
training_pipeline_error = training_pipeline.error
training_pipeline_model_id = training_pipeline.model_id
training_pipeline_encryption_spec = training_pipeline.encryption_spec
training_pipeline_training_task_definition = training_pipeline.training_task_definition
training_pipeline_create_time = training_pipeline.create_time
training_pipeline_name = training_pipeline.name
```

---


### Studie

Creates a Study. A resource name will be generated after creation of the Study.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. Describes the Study, default value is empty string. |
| `name` | String |  | Output only. The name of a study. The study's globally unique identifier. Format: `projects/{project}/locations/{location}/studies/{study}` |
| `create_time` | String |  | Output only. Time at which the study was created. |
| `inactive_reason` | String |  | Output only. A human readable reason why the Study is inactive. This should be empty if a study is ACTIVE or COMPLETED. |
| `state` | String |  | Output only. The detailed state of a Study. |
| `study_spec` | String |  | Required. Configuration of the Study. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the CustomJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. Describes the Study, default value is empty string. |
| `name` | String | Output only. The name of a study. The study's globally unique identifier. Format: `projects/{project}/locations/{location}/studies/{study}` |
| `create_time` | String | Output only. Time at which the study was created. |
| `inactive_reason` | String | Output only. A human readable reason why the Study is inactive. This should be empty if a study is ACTIVE or COMPLETED. |
| `state` | String | Output only. The detailed state of a Study. |
| `study_spec` | String | Required. Configuration of the Study. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create studie
studie = provider.aiplatform_api.Studie {
    parent = "value"  # Required. The resource name of the Location to create the CustomJob in. Format: `projects/{project}/locations/{location}`
}

# Access studie outputs
studie_id = studie.id
studie_display_name = studie.display_name
studie_name = studie.name
studie_create_time = studie.create_time
studie_inactive_reason = studie.inactive_reason
studie_state = studie.state
studie_study_spec = studie.study_spec
```

---


### Feature_view

Creates a new FeatureView in a given FeatureOnlineStore.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when this FeatureView was created. |
| `vertex_rag_source` | String |  | Optional. The Vertex RAG Source that the FeatureView is linked to. |
| `feature_registry_source` | String |  | Optional. Configures the features from a Feature Registry source that need to be loaded onto the FeatureOnlineStore. |
| `service_agent_type` | String |  | Optional. Service agent type used during data sync. By default, the Vertex AI Service Agent is used. When using an IAM Policy to isolate this FeatureView within a project, a separate service account should be provisioned by setting this field to `SERVICE_AGENT_TYPE_FEATURE_VIEW`. This will generate a separate service account to access the BigQuery source table. |
| `index_config` | String |  | Optional. Configuration for index preparation for vector search. It contains the required configurations to create an index from source data, so that approximate nearest neighbor (a.k.a ANN) algorithms search can be performed during online serving. |
| `service_account_email` | String |  | Output only. A Service Account unique to this FeatureView. The role bigquery.dataViewer should be granted to this service account to allow Vertex AI Feature Store to sync data to the online store. |
| `bigtable_metadata` | String |  | Output only. Metadata containing information about the Cloud Bigtable. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your FeatureViews. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureOnlineStore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `sync_config` | String |  | Configures when data is to be synced/updated for this FeatureView. At the end of the sync the latest featureValues for each entityId of this FeatureView are made ready for online serving. |
| `big_query_source` | String |  | Optional. Configures how data is supposed to be extracted from a BigQuery source to be loaded onto the FeatureOnlineStore. |
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String |  | Identifier. Name of the FeatureView. Format: `projects/{project}/locations/{location}/featureOnlineStores/{feature_online_store}/featureViews/{feature_view}` |
| `optimized_config` | String |  | Optional. Configuration for FeatureView created under Optimized FeatureOnlineStore. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this FeatureView was last updated. |
| `parent` | String | ✅ | Required. The resource name of the FeatureOnlineStore to create FeatureViews. Format: `projects/{project}/locations/{location}/featureOnlineStores/{feature_online_store}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when this FeatureView was created. |
| `vertex_rag_source` | String | Optional. The Vertex RAG Source that the FeatureView is linked to. |
| `feature_registry_source` | String | Optional. Configures the features from a Feature Registry source that need to be loaded onto the FeatureOnlineStore. |
| `service_agent_type` | String | Optional. Service agent type used during data sync. By default, the Vertex AI Service Agent is used. When using an IAM Policy to isolate this FeatureView within a project, a separate service account should be provisioned by setting this field to `SERVICE_AGENT_TYPE_FEATURE_VIEW`. This will generate a separate service account to access the BigQuery source table. |
| `index_config` | String | Optional. Configuration for index preparation for vector search. It contains the required configurations to create an index from source data, so that approximate nearest neighbor (a.k.a ANN) algorithms search can be performed during online serving. |
| `service_account_email` | String | Output only. A Service Account unique to this FeatureView. The role bigquery.dataViewer should be granted to this service account to allow Vertex AI Feature Store to sync data to the online store. |
| `bigtable_metadata` | String | Output only. Metadata containing information about the Cloud Bigtable. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your FeatureViews. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureOnlineStore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `sync_config` | String | Configures when data is to be synced/updated for this FeatureView. At the end of the sync the latest featureValues for each entityId of this FeatureView are made ready for online serving. |
| `big_query_source` | String | Optional. Configures how data is supposed to be extracted from a BigQuery source to be loaded onto the FeatureOnlineStore. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String | Identifier. Name of the FeatureView. Format: `projects/{project}/locations/{location}/featureOnlineStores/{feature_online_store}/featureViews/{feature_view}` |
| `optimized_config` | String | Optional. Configuration for FeatureView created under Optimized FeatureOnlineStore. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this FeatureView was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature_view
feature_view = provider.aiplatform_api.Feature_view {
    parent = "value"  # Required. The resource name of the FeatureOnlineStore to create FeatureViews. Format: `projects/{project}/locations/{location}/featureOnlineStores/{feature_online_store}`
}

# Access feature_view outputs
feature_view_id = feature_view.id
feature_view_create_time = feature_view.create_time
feature_view_vertex_rag_source = feature_view.vertex_rag_source
feature_view_feature_registry_source = feature_view.feature_registry_source
feature_view_service_agent_type = feature_view.service_agent_type
feature_view_index_config = feature_view.index_config
feature_view_service_account_email = feature_view.service_account_email
feature_view_bigtable_metadata = feature_view.bigtable_metadata
feature_view_labels = feature_view.labels
feature_view_sync_config = feature_view.sync_config
feature_view_big_query_source = feature_view.big_query_source
feature_view_etag = feature_view.etag
feature_view_name = feature_view.name
feature_view_optimized_config = feature_view.optimized_config
feature_view_satisfies_pzi = feature_view.satisfies_pzi
feature_view_satisfies_pzs = feature_view.satisfies_pzs
feature_view_update_time = feature_view.update_time
```

---


### Notebook_execution_job

Creates a NotebookExecutionJob.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notebook_runtime_template_resource_name` | String |  | The NotebookRuntimeTemplate to source compute configuration from. |
| `job_state` | String |  | Output only. The state of the NotebookExecutionJob. |
| `execution_user` | String |  | The user email to run the execution as. Only supported by Colab runtimes. |
| `workbench_runtime` | String |  | The Workbench runtime configuration to use for the notebook execution. |
| `execution_timeout` | String |  | Max running time of the execution job in seconds (default 86400s / 24 hrs). |
| `gcs_notebook_source` | String |  | The Cloud Storage url pointing to the ipynb file. Format: `gs://bucket/notebook_file.ipynb` |
| `name` | String |  | Output only. The resource name of this NotebookExecutionJob. Format: `projects/{project_id}/locations/{location}/notebookExecutionJobs/{job_id}` |
| `update_time` | String |  | Output only. Timestamp when this NotebookExecutionJob was most recently updated. |
| `gcs_output_uri` | String |  | The Cloud Storage location to upload the result to. Format: `gs://bucket-name` |
| `custom_environment_spec` | String |  | The custom compute configuration for an execution job. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize NotebookExecutionJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for the notebook execution job. This field is auto-populated if the NotebookRuntimeTemplate has an encryption spec. |
| `dataform_repository_source` | String |  | The Dataform Repository pointing to a single file notebook repository. |
| `schedule_resource_name` | String |  | The Schedule resource name if this job is triggered by one. Format: `projects/{project_id}/locations/{location}/schedules/{schedule_id}` |
| `service_account` | String |  | The service account to run the execution as. |
| `kernel_name` | String |  | The name of the kernel to use during notebook execution. If unset, the default kernel is used. |
| `display_name` | String |  | The display name of the NotebookExecutionJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `status` | String |  | Output only. Populated when the NotebookExecutionJob is completed. When there is an error during notebook execution, the error details are populated. |
| `create_time` | String |  | Output only. Timestamp when this NotebookExecutionJob was created. |
| `direct_notebook_source` | String |  | The contents of an input notebook file. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the NotebookExecutionJob. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notebook_runtime_template_resource_name` | String | The NotebookRuntimeTemplate to source compute configuration from. |
| `job_state` | String | Output only. The state of the NotebookExecutionJob. |
| `execution_user` | String | The user email to run the execution as. Only supported by Colab runtimes. |
| `workbench_runtime` | String | The Workbench runtime configuration to use for the notebook execution. |
| `execution_timeout` | String | Max running time of the execution job in seconds (default 86400s / 24 hrs). |
| `gcs_notebook_source` | String | The Cloud Storage url pointing to the ipynb file. Format: `gs://bucket/notebook_file.ipynb` |
| `name` | String | Output only. The resource name of this NotebookExecutionJob. Format: `projects/{project_id}/locations/{location}/notebookExecutionJobs/{job_id}` |
| `update_time` | String | Output only. Timestamp when this NotebookExecutionJob was most recently updated. |
| `gcs_output_uri` | String | The Cloud Storage location to upload the result to. Format: `gs://bucket-name` |
| `custom_environment_spec` | String | The custom compute configuration for an execution job. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize NotebookExecutionJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `encryption_spec` | String | Customer-managed encryption key spec for the notebook execution job. This field is auto-populated if the NotebookRuntimeTemplate has an encryption spec. |
| `dataform_repository_source` | String | The Dataform Repository pointing to a single file notebook repository. |
| `schedule_resource_name` | String | The Schedule resource name if this job is triggered by one. Format: `projects/{project_id}/locations/{location}/schedules/{schedule_id}` |
| `service_account` | String | The service account to run the execution as. |
| `kernel_name` | String | The name of the kernel to use during notebook execution. If unset, the default kernel is used. |
| `display_name` | String | The display name of the NotebookExecutionJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `status` | String | Output only. Populated when the NotebookExecutionJob is completed. When there is an error during notebook execution, the error details are populated. |
| `create_time` | String | Output only. Timestamp when this NotebookExecutionJob was created. |
| `direct_notebook_source` | String | The contents of an input notebook file. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notebook_execution_job
notebook_execution_job = provider.aiplatform_api.Notebook_execution_job {
    parent = "value"  # Required. The resource name of the Location to create the NotebookExecutionJob. Format: `projects/{project}/locations/{location}`
}

# Access notebook_execution_job outputs
notebook_execution_job_id = notebook_execution_job.id
notebook_execution_job_notebook_runtime_template_resource_name = notebook_execution_job.notebook_runtime_template_resource_name
notebook_execution_job_job_state = notebook_execution_job.job_state
notebook_execution_job_execution_user = notebook_execution_job.execution_user
notebook_execution_job_workbench_runtime = notebook_execution_job.workbench_runtime
notebook_execution_job_execution_timeout = notebook_execution_job.execution_timeout
notebook_execution_job_gcs_notebook_source = notebook_execution_job.gcs_notebook_source
notebook_execution_job_name = notebook_execution_job.name
notebook_execution_job_update_time = notebook_execution_job.update_time
notebook_execution_job_gcs_output_uri = notebook_execution_job.gcs_output_uri
notebook_execution_job_custom_environment_spec = notebook_execution_job.custom_environment_spec
notebook_execution_job_labels = notebook_execution_job.labels
notebook_execution_job_encryption_spec = notebook_execution_job.encryption_spec
notebook_execution_job_dataform_repository_source = notebook_execution_job.dataform_repository_source
notebook_execution_job_schedule_resource_name = notebook_execution_job.schedule_resource_name
notebook_execution_job_service_account = notebook_execution_job.service_account
notebook_execution_job_kernel_name = notebook_execution_job.kernel_name
notebook_execution_job_display_name = notebook_execution_job.display_name
notebook_execution_job_status = notebook_execution_job.status
notebook_execution_job_create_time = notebook_execution_job.create_time
notebook_execution_job_direct_notebook_source = notebook_execution_job.direct_notebook_source
```

---


### Slice

Imports a list of externally generated EvaluatedAnnotations.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `evaluated_annotations` | Vec<String> |  | Required. Evaluated annotations resource to be imported. |
| `parent` | String | ✅ | Required. The name of the parent ModelEvaluationSlice resource. Format: `projects/{project}/locations/{location}/models/{model}/evaluations/{evaluation}/slices/{slice}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metrics` | String | Output only. Sliced evaluation metrics of the Model. The schema of the metrics is stored in metrics_schema_uri |
| `model_explanation` | String | Output only. Aggregated explanation metrics for the Model's prediction output over the data this ModelEvaluation uses. This field is populated only if the Model is evaluated with explanations, and only for tabular Models. |
| `name` | String | Output only. The resource name of the ModelEvaluationSlice. |
| `metrics_schema_uri` | String | Output only. Points to a YAML file stored on Google Cloud Storage describing the metrics of this ModelEvaluationSlice. The schema is defined as an OpenAPI 3.0.2 [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.2.md#schemaObject). |
| `slice` | String | Output only. The slice of the test data that is used to evaluate the Model. |
| `create_time` | String | Output only. Timestamp when this ModelEvaluationSlice was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create slice
slice = provider.aiplatform_api.Slice {
    parent = "value"  # Required. The name of the parent ModelEvaluationSlice resource. Format: `projects/{project}/locations/{location}/models/{model}/evaluations/{evaluation}/slices/{slice}`
}

# Access slice outputs
slice_id = slice.id
slice_metrics = slice.metrics
slice_model_explanation = slice.model_explanation
slice_name = slice.name
slice_metrics_schema_uri = slice.metrics_schema_uri
slice_slice = slice.slice
slice_create_time = slice.create_time
```

---


### Data_labeling_job

Creates a DataLabelingJob.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inputs` | String |  | Required. Input config parameters for the DataLabelingJob. |
| `error` | String |  | Output only. DataLabelingJob errors. It is only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `labeling_progress` | i64 |  | Output only. Current labeling job progress percentage scaled in interval [0, 100], indicating the percentage of DataItems that has been finished. |
| `display_name` | String |  | Required. The user-defined name of the DataLabelingJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. Display name of a DataLabelingJob. |
| `active_learning_config` | String |  | Parameters that configure the active learning pipeline. Active learning will label the data incrementally via several iterations. For every iteration, it will select a batch of data based on the sampling strategy. |
| `specialist_pools` | Vec<String> |  | The SpecialistPools' resource names associated with this job. |
| `state` | String |  | Output only. The detailed state of the job. |
| `labeler_count` | i64 |  | Required. Number of labelers to work on each DataItem. |
| `current_spend` | String |  | Output only. Estimated cost(in US dollars) that the DataLabelingJob has incurred to date. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your DataLabelingJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. Following system labels exist for each DataLabelingJob: * "aiplatform.googleapis.com/schema": output only, its value is the inputs_schema's title. |
| `instruction_uri` | String |  | Required. The Google Cloud Storage location of the instruction pdf. This pdf is shared with labelers, and provides detailed description on how to label DataItems in Datasets. |
| `create_time` | String |  | Output only. Timestamp when this DataLabelingJob was created. |
| `inputs_schema_uri` | String |  | Required. Points to a YAML file stored on Google Cloud Storage describing the config for a specific type of DataLabelingJob. The schema files that can be used here are found in the https://storage.googleapis.com/google-cloud-aiplatform bucket in the /schema/datalabelingjob/inputs/ folder. |
| `name` | String |  | Output only. Resource name of the DataLabelingJob. |
| `update_time` | String |  | Output only. Timestamp when this DataLabelingJob was updated most recently. |
| `annotation_labels` | HashMap<String, String> |  | Labels to assign to annotations generated by this DataLabelingJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `datasets` | Vec<String> |  | Required. Dataset resource names. Right now we only support labeling from a single Dataset. Format: `projects/{project}/locations/{location}/datasets/{dataset}` |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a DataLabelingJob. If set, this DataLabelingJob will be secured by this key. Note: Annotations created in the DataLabelingJob are associated with the EncryptionSpec of the Dataset they are exported to. |
| `parent` | String | ✅ | Required. The parent of the DataLabelingJob. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `inputs` | String | Required. Input config parameters for the DataLabelingJob. |
| `error` | String | Output only. DataLabelingJob errors. It is only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `labeling_progress` | i64 | Output only. Current labeling job progress percentage scaled in interval [0, 100], indicating the percentage of DataItems that has been finished. |
| `display_name` | String | Required. The user-defined name of the DataLabelingJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. Display name of a DataLabelingJob. |
| `active_learning_config` | String | Parameters that configure the active learning pipeline. Active learning will label the data incrementally via several iterations. For every iteration, it will select a batch of data based on the sampling strategy. |
| `specialist_pools` | Vec<String> | The SpecialistPools' resource names associated with this job. |
| `state` | String | Output only. The detailed state of the job. |
| `labeler_count` | i64 | Required. Number of labelers to work on each DataItem. |
| `current_spend` | String | Output only. Estimated cost(in US dollars) that the DataLabelingJob has incurred to date. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your DataLabelingJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. Following system labels exist for each DataLabelingJob: * "aiplatform.googleapis.com/schema": output only, its value is the inputs_schema's title. |
| `instruction_uri` | String | Required. The Google Cloud Storage location of the instruction pdf. This pdf is shared with labelers, and provides detailed description on how to label DataItems in Datasets. |
| `create_time` | String | Output only. Timestamp when this DataLabelingJob was created. |
| `inputs_schema_uri` | String | Required. Points to a YAML file stored on Google Cloud Storage describing the config for a specific type of DataLabelingJob. The schema files that can be used here are found in the https://storage.googleapis.com/google-cloud-aiplatform bucket in the /schema/datalabelingjob/inputs/ folder. |
| `name` | String | Output only. Resource name of the DataLabelingJob. |
| `update_time` | String | Output only. Timestamp when this DataLabelingJob was updated most recently. |
| `annotation_labels` | HashMap<String, String> | Labels to assign to annotations generated by this DataLabelingJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `datasets` | Vec<String> | Required. Dataset resource names. Right now we only support labeling from a single Dataset. Format: `projects/{project}/locations/{location}/datasets/{dataset}` |
| `encryption_spec` | String | Customer-managed encryption key spec for a DataLabelingJob. If set, this DataLabelingJob will be secured by this key. Note: Annotations created in the DataLabelingJob are associated with the EncryptionSpec of the Dataset they are exported to. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_labeling_job
data_labeling_job = provider.aiplatform_api.Data_labeling_job {
    parent = "value"  # Required. The parent of the DataLabelingJob. Format: `projects/{project}/locations/{location}`
}

# Access data_labeling_job outputs
data_labeling_job_id = data_labeling_job.id
data_labeling_job_inputs = data_labeling_job.inputs
data_labeling_job_error = data_labeling_job.error
data_labeling_job_labeling_progress = data_labeling_job.labeling_progress
data_labeling_job_display_name = data_labeling_job.display_name
data_labeling_job_active_learning_config = data_labeling_job.active_learning_config
data_labeling_job_specialist_pools = data_labeling_job.specialist_pools
data_labeling_job_state = data_labeling_job.state
data_labeling_job_labeler_count = data_labeling_job.labeler_count
data_labeling_job_current_spend = data_labeling_job.current_spend
data_labeling_job_labels = data_labeling_job.labels
data_labeling_job_instruction_uri = data_labeling_job.instruction_uri
data_labeling_job_create_time = data_labeling_job.create_time
data_labeling_job_inputs_schema_uri = data_labeling_job.inputs_schema_uri
data_labeling_job_name = data_labeling_job.name
data_labeling_job_update_time = data_labeling_job.update_time
data_labeling_job_annotation_labels = data_labeling_job.annotation_labels
data_labeling_job_datasets = data_labeling_job.datasets
data_labeling_job_encryption_spec = data_labeling_job.encryption_spec
```

---


### Annotation_spec

Gets an AnnotationSpec.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the AnnotationSpec. |
| `create_time` | String | Output only. Timestamp when this AnnotationSpec was created. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `update_time` | String | Output only. Timestamp when AnnotationSpec was last updated. |
| `display_name` | String | Required. The user-defined name of the AnnotationSpec. The name can be up to 128 characters long and can consist of any UTF-8 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access annotation_spec outputs
annotation_spec_id = annotation_spec.id
annotation_spec_name = annotation_spec.name
annotation_spec_create_time = annotation_spec.create_time
annotation_spec_etag = annotation_spec.etag
annotation_spec_update_time = annotation_spec.update_time
annotation_spec_display_name = annotation_spec.display_name
```

---


### Chat

Exposes an OpenAI-compatible endpoint for chat completions.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | String |  | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `endpoint` | String | ✅ | Required. The name of the endpoint requested to serve the prediction. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create chat
chat = provider.aiplatform_api.Chat {
    endpoint = "value"  # Required. The name of the endpoint requested to serve the prediction. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}`
}

```

---


### Notebook_runtime

Starts a NotebookRuntime.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the NotebookRuntime resource to be started. Instead of checking whether the name is in valid NotebookRuntime resource name format, directly throw NotFound exception if there is no such NotebookRuntime in spanner. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the NotebookRuntime. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `health_state` | String | Output only. The health state of the NotebookRuntime. |
| `idle_shutdown_config` | String | Output only. The idle shutdown configuration of the notebook runtime. |
| `network_tags` | Vec<String> | Optional. The Compute Engine tags to add to runtime (see [Tagging instances](https://cloud.google.com/vpc/docs/add-remove-network-tags)). |
| `reservation_affinity` | String | Output only. Reservation Affinity of the notebook runtime. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `name` | String | Output only. The resource name of the NotebookRuntime. |
| `is_upgradable` | bool | Output only. Whether NotebookRuntime is upgradable. |
| `euc_config` | String | Output only. EUC configuration of the notebook runtime. |
| `update_time` | String | Output only. Timestamp when this NotebookRuntime was most recently updated. |
| `version` | String | Output only. The VM os image version of NotebookRuntime. |
| `network_spec` | String | Output only. Network spec of the notebook runtime. |
| `notebook_runtime_template_ref` | String | Output only. The pointer to NotebookRuntimeTemplate this NotebookRuntime is created from. |
| `software_config` | String | Output only. Software config of the notebook runtime. |
| `encryption_spec` | String | Output only. Customer-managed encryption key spec for the notebook runtime. |
| `runtime_user` | String | Required. The user email of the NotebookRuntime. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `runtime_state` | String | Output only. The runtime (instance) state of the NotebookRuntime. |
| `data_persistent_disk_spec` | String | Output only. The specification of persistent disk attached to the notebook runtime as data disk storage. |
| `proxy_uri` | String | Output only. The proxy endpoint used to access the NotebookRuntime. |
| `machine_spec` | String | Output only. The specification of a single machine used by the notebook runtime. |
| `service_account` | String | Output only. Deprecated: This field is no longer used and the "Vertex AI Notebook Service Account" (service-PROJECT_NUMBER@gcp-sa-aiplatform-vm.iam.gserviceaccount.com) is used for the runtime workload identity. See https://cloud.google.com/iam/docs/service-agents#vertex-ai-notebook-service-account for more details. The service account that the NotebookRuntime workload runs as. |
| `shielded_vm_config` | String | Output only. Runtime Shielded VM spec. |
| `expiration_time` | String | Output only. Timestamp when this NotebookRuntime will be expired: 1. System Predefined NotebookRuntime: 24 hours after creation. After expiration, system predifined runtime will be deleted. 2. User created NotebookRuntime: 6 months after last upgrade. After expiration, user created runtime will be stopped and allowed for upgrade. |
| `description` | String | The description of the NotebookRuntime. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your NotebookRuntime. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one NotebookRuntime (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. Following system labels exist for NotebookRuntime: * "aiplatform.googleapis.com/notebook_runtime_gce_instance_id": output only, its value is the Compute Engine instance id. * "aiplatform.googleapis.com/colab_enterprise_entry_service": its value is either "bigquery" or "vertex"; if absent, it should be "vertex". This is to describe the entry service, either BigQuery or Vertex. |
| `notebook_runtime_type` | String | Output only. The type of the notebook runtime. |
| `create_time` | String | Output only. Timestamp when this NotebookRuntime was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notebook_runtime
notebook_runtime = provider.aiplatform_api.Notebook_runtime {
    name = "value"  # Required. The name of the NotebookRuntime resource to be started. Instead of checking whether the name is in valid NotebookRuntime resource name format, directly throw NotFound exception if there is no such NotebookRuntime in spanner.
}

# Access notebook_runtime outputs
notebook_runtime_id = notebook_runtime.id
notebook_runtime_display_name = notebook_runtime.display_name
notebook_runtime_health_state = notebook_runtime.health_state
notebook_runtime_idle_shutdown_config = notebook_runtime.idle_shutdown_config
notebook_runtime_network_tags = notebook_runtime.network_tags
notebook_runtime_reservation_affinity = notebook_runtime.reservation_affinity
notebook_runtime_satisfies_pzs = notebook_runtime.satisfies_pzs
notebook_runtime_name = notebook_runtime.name
notebook_runtime_is_upgradable = notebook_runtime.is_upgradable
notebook_runtime_euc_config = notebook_runtime.euc_config
notebook_runtime_update_time = notebook_runtime.update_time
notebook_runtime_version = notebook_runtime.version
notebook_runtime_network_spec = notebook_runtime.network_spec
notebook_runtime_notebook_runtime_template_ref = notebook_runtime.notebook_runtime_template_ref
notebook_runtime_software_config = notebook_runtime.software_config
notebook_runtime_encryption_spec = notebook_runtime.encryption_spec
notebook_runtime_runtime_user = notebook_runtime.runtime_user
notebook_runtime_satisfies_pzi = notebook_runtime.satisfies_pzi
notebook_runtime_runtime_state = notebook_runtime.runtime_state
notebook_runtime_data_persistent_disk_spec = notebook_runtime.data_persistent_disk_spec
notebook_runtime_proxy_uri = notebook_runtime.proxy_uri
notebook_runtime_machine_spec = notebook_runtime.machine_spec
notebook_runtime_service_account = notebook_runtime.service_account
notebook_runtime_shielded_vm_config = notebook_runtime.shielded_vm_config
notebook_runtime_expiration_time = notebook_runtime.expiration_time
notebook_runtime_description = notebook_runtime.description
notebook_runtime_labels = notebook_runtime.labels
notebook_runtime_notebook_runtime_type = notebook_runtime.notebook_runtime_type
notebook_runtime_create_time = notebook_runtime.create_time
```

---


### Evaluation_set

Creates an Evaluation Set.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Timestamp when this item was last updated. |
| `create_time` | String |  | Output only. Timestamp when this item was created. |
| `evaluation_items` | Vec<String> |  | Required. The EvaluationItems that are part of this dataset. |
| `name` | String |  | Identifier. The resource name of the EvaluationSet. Format: `projects/{project}/locations/{location}/evaluationSets/{evaluation_set}` |
| `display_name` | String |  | Required. The display name of the EvaluationSet. |
| `metadata` | String |  | Optional. Metadata for the EvaluationSet. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Evaluation Set in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Timestamp when this item was last updated. |
| `create_time` | String | Output only. Timestamp when this item was created. |
| `evaluation_items` | Vec<String> | Required. The EvaluationItems that are part of this dataset. |
| `name` | String | Identifier. The resource name of the EvaluationSet. Format: `projects/{project}/locations/{location}/evaluationSets/{evaluation_set}` |
| `display_name` | String | Required. The display name of the EvaluationSet. |
| `metadata` | String | Optional. Metadata for the EvaluationSet. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation_set
evaluation_set = provider.aiplatform_api.Evaluation_set {
    parent = "value"  # Required. The resource name of the Location to create the Evaluation Set in. Format: `projects/{project}/locations/{location}`
}

# Access evaluation_set outputs
evaluation_set_id = evaluation_set.id
evaluation_set_update_time = evaluation_set.update_time
evaluation_set_create_time = evaluation_set.create_time
evaluation_set_evaluation_items = evaluation_set.evaluation_items
evaluation_set_name = evaluation_set.name
evaluation_set_display_name = evaluation_set.display_name
evaluation_set_metadata = evaluation_set.metadata
```

---


### Nas_trial_detail

Gets a NasTrialDetail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the NasTrialDetail. |
| `search_trial` | String | The requested search NasTrial. |
| `parameters` | String | The parameters for the NasJob NasTrial. |
| `train_trial` | String | The train NasTrial corresponding to search_trial. Only populated if search_trial is used for training. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access nas_trial_detail outputs
nas_trial_detail_id = nas_trial_detail.id
nas_trial_detail_name = nas_trial_detail.name
nas_trial_detail_search_trial = nas_trial_detail.search_trial
nas_trial_detail_parameters = nas_trial_detail.parameters
nas_trial_detail_train_trial = nas_trial_detail.train_trial
```

---


### Invoke

Forwards arbitrary HTTP requests for both streaming and non-streaming cases. To use this method, invoke_route_prefix must be set to allow the paths that will be specified in the request.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `http_body` | String |  | The invoke method input. Supports HTTP headers and arbitrary data payload. |
| `deployed_model_id` | String |  | ID of the DeployedModel that serves the invoke request. |
| `invoke_id` | String | ✅ |  |
| `deployed_model_id` | String | ✅ | ID of the DeployedModel that serves the invoke request. |
| `endpoint` | String | ✅ | Required. The name of the Endpoint requested to serve the prediction. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create invoke
invoke = provider.aiplatform_api.Invoke {
    invoke_id = "value"  # Required field
    deployed_model_id = "value"  # ID of the DeployedModel that serves the invoke request.
    endpoint = "value"  # Required. The name of the Endpoint requested to serve the prediction. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}`
}

```

---


### Openapi

Forwards arbitrary HTTP requests for both streaming and non-streaming cases. To use this method, invoke_route_prefix must be set to allow the paths that will be specified in the request.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data` | String |  | The HTTP request/response body as raw binary. |
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `endpoint` | String | ✅ | Required. The name of the Endpoint requested to serve the prediction. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create openapi
openapi = provider.aiplatform_api.Openapi {
    endpoint = "value"  # Required. The name of the Endpoint requested to serve the prediction. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}`
}

```

---


### Evaluation

Imports an externally generated ModelEvaluation.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_evaluation` | String |  | Required. Model evaluation resource to be imported. |
| `parent` | String | ✅ | Required. The name of the parent model resource. Format: `projects/{project}/locations/{location}/models/{model}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | The metadata of the ModelEvaluation. For the ModelEvaluation uploaded from Managed Pipeline, metadata contains a structured value with keys of "pipeline_job_id", "evaluation_dataset_type", "evaluation_dataset_path", "row_based_metrics_path". |
| `metrics_schema_uri` | String | Points to a YAML file stored on Google Cloud Storage describing the metrics of this ModelEvaluation. The schema is defined as an OpenAPI 3.0.2 [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.2.md#schemaObject). |
| `create_time` | String | Output only. Timestamp when this ModelEvaluation was created. |
| `name` | String | Output only. The resource name of the ModelEvaluation. |
| `model_explanation` | String | Aggregated explanation metrics for the Model's prediction output over the data this ModelEvaluation uses. This field is populated only if the Model is evaluated with explanations, and only for AutoML tabular Models.  |
| `metrics` | String | Evaluation metrics of the Model. The schema of the metrics is stored in metrics_schema_uri |
| `explanation_specs` | Vec<String> | Describes the values of ExplanationSpec that are used for explaining the predicted values on the evaluated data. |
| `annotation_schema_uri` | String | Points to a YAML file stored on Google Cloud Storage describing EvaluatedDataItemView.predictions, EvaluatedDataItemView.ground_truths, EvaluatedAnnotation.predictions, and EvaluatedAnnotation.ground_truths. The schema is defined as an OpenAPI 3.0.2 [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.2.md#schemaObject). This field is not populated if there are neither EvaluatedDataItemViews nor EvaluatedAnnotations under this ModelEvaluation. |
| `display_name` | String | The display name of the ModelEvaluation. |
| `slice_dimensions` | Vec<String> | All possible dimensions of ModelEvaluationSlices. The dimensions can be used as the filter of the ModelService.ListModelEvaluationSlices request, in the form of `slice.dimension = `. |
| `data_item_schema_uri` | String | Points to a YAML file stored on Google Cloud Storage describing EvaluatedDataItemView.data_item_payload and EvaluatedAnnotation.data_item_payload. The schema is defined as an OpenAPI 3.0.2 [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.2.md#schemaObject). This field is not populated if there are neither EvaluatedDataItemViews nor EvaluatedAnnotations under this ModelEvaluation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation
evaluation = provider.aiplatform_api.Evaluation {
    parent = "value"  # Required. The name of the parent model resource. Format: `projects/{project}/locations/{location}/models/{model}`
}

# Access evaluation outputs
evaluation_id = evaluation.id
evaluation_metadata = evaluation.metadata
evaluation_metrics_schema_uri = evaluation.metrics_schema_uri
evaluation_create_time = evaluation.create_time
evaluation_name = evaluation.name
evaluation_model_explanation = evaluation.model_explanation
evaluation_metrics = evaluation.metrics
evaluation_explanation_specs = evaluation.explanation_specs
evaluation_annotation_schema_uri = evaluation.annotation_schema_uri
evaluation_display_name = evaluation.display_name
evaluation_slice_dimensions = evaluation.slice_dimensions
evaluation_data_item_schema_uri = evaluation.data_item_schema_uri
```

---


### Annotation

Lists Annotations belongs to a dataitem.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | Vec<String> | A list of Annotations that matches the specified filter in the request. |
| `next_page_token` | String | The standard List next-page token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access annotation outputs
annotation_id = annotation.id
annotation_annotations = annotation.annotations
annotation_next_page_token = annotation.next_page_token
```

---


### Tensorboard

Creates a Tensorboard.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `is_default` | bool |  | Used to indicate if the TensorBoard instance is the default one. Each project & region can have at most one default TensorBoard instance. Creation of a default TensorBoard instance and updating an existing TensorBoard instance to be default will mark all other TensorBoard instances (if any) as non default. |
| `description` | String |  | Description of this Tensorboard. |
| `display_name` | String |  | Required. User provided name of this Tensorboard. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Tensorboards. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Tensorboard (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `etag` | String |  | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this Tensorboard was last updated. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `blob_storage_path_prefix` | String |  | Output only. Consumer project Cloud Storage path prefix used to store blob data, which can either be a bucket or directory. Does not end with a '/'. |
| `name` | String |  | Output only. Name of the Tensorboard. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}` |
| `run_count` | i64 |  | Output only. The number of Runs stored in this Tensorboard. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a Tensorboard. If set, this Tensorboard and all sub-resources of this Tensorboard will be secured by this key. |
| `create_time` | String |  | Output only. Timestamp when this Tensorboard was created. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Tensorboard in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_default` | bool | Used to indicate if the TensorBoard instance is the default one. Each project & region can have at most one default TensorBoard instance. Creation of a default TensorBoard instance and updating an existing TensorBoard instance to be default will mark all other TensorBoard instances (if any) as non default. |
| `description` | String | Description of this Tensorboard. |
| `display_name` | String | Required. User provided name of this Tensorboard. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Tensorboards. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Tensorboard (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `etag` | String | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this Tensorboard was last updated. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `blob_storage_path_prefix` | String | Output only. Consumer project Cloud Storage path prefix used to store blob data, which can either be a bucket or directory. Does not end with a '/'. |
| `name` | String | Output only. Name of the Tensorboard. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}` |
| `run_count` | i64 | Output only. The number of Runs stored in this Tensorboard. |
| `encryption_spec` | String | Customer-managed encryption key spec for a Tensorboard. If set, this Tensorboard and all sub-resources of this Tensorboard will be secured by this key. |
| `create_time` | String | Output only. Timestamp when this Tensorboard was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tensorboard
tensorboard = provider.aiplatform_api.Tensorboard {
    parent = "value"  # Required. The resource name of the Location to create the Tensorboard in. Format: `projects/{project}/locations/{location}`
}

# Access tensorboard outputs
tensorboard_id = tensorboard.id
tensorboard_is_default = tensorboard.is_default
tensorboard_description = tensorboard.description
tensorboard_display_name = tensorboard.display_name
tensorboard_labels = tensorboard.labels
tensorboard_etag = tensorboard.etag
tensorboard_satisfies_pzs = tensorboard.satisfies_pzs
tensorboard_update_time = tensorboard.update_time
tensorboard_satisfies_pzi = tensorboard.satisfies_pzi
tensorboard_blob_storage_path_prefix = tensorboard.blob_storage_path_prefix
tensorboard_name = tensorboard.name
tensorboard_run_count = tensorboard.run_count
tensorboard_encryption_spec = tensorboard.encryption_spec
tensorboard_create_time = tensorboard.create_time
```

---


### Custom_job

Creates a CustomJob. A created CustomJob right away will be attempted to be run.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_spec` | String |  | Customer-managed encryption key options for a CustomJob. If this is set, then all resources created by the CustomJob will be encrypted with the provided encryption key. |
| `display_name` | String |  | Required. The display name of the CustomJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize CustomJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `state` | String |  | Output only. The detailed state of the job. |
| `create_time` | String |  | Output only. Time when the CustomJob was created. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `end_time` | String |  | Output only. Time when the CustomJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `start_time` | String |  | Output only. Time when the CustomJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `web_access_uris` | HashMap<String, String> |  | Output only. URIs for accessing [interactive shells](https://cloud.google.com/vertex-ai/docs/training/monitor-debug-interactive-shell) (one URI for each training node). Only available if job_spec.enable_web_access is `true`. The keys are names of each node in the training job; for example, `workerpool0-0` for the primary node, `workerpool1-0` for the first node in the second worker pool, and `workerpool1-1` for the second node in the second worker pool. The values are the URIs for each node's interactive shell. |
| `error` | String |  | Output only. Only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `name` | String |  | Output only. Resource name of a CustomJob. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Time when the CustomJob was most recently updated. |
| `job_spec` | String |  | Required. Job spec. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the CustomJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_spec` | String | Customer-managed encryption key options for a CustomJob. If this is set, then all resources created by the CustomJob will be encrypted with the provided encryption key. |
| `display_name` | String | Required. The display name of the CustomJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize CustomJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `state` | String | Output only. The detailed state of the job. |
| `create_time` | String | Output only. Time when the CustomJob was created. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `end_time` | String | Output only. Time when the CustomJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `start_time` | String | Output only. Time when the CustomJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `web_access_uris` | HashMap<String, String> | Output only. URIs for accessing [interactive shells](https://cloud.google.com/vertex-ai/docs/training/monitor-debug-interactive-shell) (one URI for each training node). Only available if job_spec.enable_web_access is `true`. The keys are names of each node in the training job; for example, `workerpool0-0` for the primary node, `workerpool1-0` for the first node in the second worker pool, and `workerpool1-1` for the second node in the second worker pool. The values are the URIs for each node's interactive shell. |
| `error` | String | Output only. Only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `name` | String | Output only. Resource name of a CustomJob. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Time when the CustomJob was most recently updated. |
| `job_spec` | String | Required. Job spec. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_job
custom_job = provider.aiplatform_api.Custom_job {
    parent = "value"  # Required. The resource name of the Location to create the CustomJob in. Format: `projects/{project}/locations/{location}`
}

# Access custom_job outputs
custom_job_id = custom_job.id
custom_job_encryption_spec = custom_job.encryption_spec
custom_job_display_name = custom_job.display_name
custom_job_labels = custom_job.labels
custom_job_state = custom_job.state
custom_job_create_time = custom_job.create_time
custom_job_satisfies_pzs = custom_job.satisfies_pzs
custom_job_end_time = custom_job.end_time
custom_job_start_time = custom_job.start_time
custom_job_web_access_uris = custom_job.web_access_uris
custom_job_error = custom_job.error
custom_job_name = custom_job.name
custom_job_satisfies_pzi = custom_job.satisfies_pzi
custom_job_update_time = custom_job.update_time
custom_job_job_spec = custom_job.job_spec
```

---


### Rag_file

Import files from Google Cloud Storage or Google Drive into a RagCorpus.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `import_rag_files_config` | String |  | Required. The config for the RagFiles to be synced and imported into the RagCorpus. VertexRagDataService.ImportRagFiles. |
| `parent` | String | ✅ | Required. The name of the RagCorpus resource into which to import files. Format: `projects/{project}/locations/{location}/ragCorpora/{rag_corpus}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the RagFile. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `file_status` | String | Output only. State of the RagFile. |
| `name` | String | Output only. The resource name of the RagFile. |
| `slack_source` | String | The RagFile is imported from a Slack channel. |
| `update_time` | String | Output only. Timestamp when this RagFile was last updated. |
| `user_metadata` | String | Output only. The metadata for metadata search. The user_metadata Needs to be in JSON format. |
| `share_point_sources` | String | The RagFile is imported from a SharePoint source. |
| `google_drive_source` | String | Output only. Google Drive location. Supports importing individual files as well as Google Drive folders. |
| `direct_upload_source` | String | Output only. The RagFile is encapsulated and uploaded in the UploadRagFile request. |
| `description` | String | Optional. The description of the RagFile. |
| `create_time` | String | Output only. Timestamp when this RagFile was created. |
| `jira_source` | String | The RagFile is imported from a Jira query. |
| `gcs_source` | String | Output only. Google Cloud Storage location of the RagFile. It does not support wildcards in the Cloud Storage uri for now. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rag_file
rag_file = provider.aiplatform_api.Rag_file {
    parent = "value"  # Required. The name of the RagCorpus resource into which to import files. Format: `projects/{project}/locations/{location}/ragCorpora/{rag_corpus}`
}

# Access rag_file outputs
rag_file_id = rag_file.id
rag_file_display_name = rag_file.display_name
rag_file_file_status = rag_file.file_status
rag_file_name = rag_file.name
rag_file_slack_source = rag_file.slack_source
rag_file_update_time = rag_file.update_time
rag_file_user_metadata = rag_file.user_metadata
rag_file_share_point_sources = rag_file.share_point_sources
rag_file_google_drive_source = rag_file.google_drive_source
rag_file_direct_upload_source = rag_file.direct_upload_source
rag_file_description = rag_file.description
rag_file_create_time = rag_file.create_time
rag_file_jira_source = rag_file.jira_source
rag_file_gcs_source = rag_file.gcs_source
```

---


### Feature_view_sync

Gets details of a single FeatureViewSync.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `sync_summary` | String | Output only. Summary of the sync job. |
| `name` | String | Identifier. Name of the FeatureViewSync. Format: `projects/{project}/locations/{location}/featureOnlineStores/{feature_online_store}/featureViews/{feature_view}/featureViewSyncs/{feature_view_sync}` |
| `create_time` | String | Output only. Time when this FeatureViewSync is created. Creation of a FeatureViewSync means that the job is pending / waiting for sufficient resources but may not have started the actual data transfer yet. |
| `run_time` | String | Output only. Time when this FeatureViewSync is finished. |
| `final_status` | String | Output only. Final status of the FeatureViewSync. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access feature_view_sync outputs
feature_view_sync_id = feature_view_sync.id
feature_view_sync_satisfies_pzs = feature_view_sync.satisfies_pzs
feature_view_sync_sync_summary = feature_view_sync.sync_summary
feature_view_sync_name = feature_view_sync.name
feature_view_sync_create_time = feature_view_sync.create_time
feature_view_sync_run_time = feature_view_sync.run_time
feature_view_sync_final_status = feature_view_sync.final_status
feature_view_sync_satisfies_pzi = feature_view_sync.satisfies_pzi
```

---


### Rag_corpora

Creates a RagCorpus.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when this RagCorpus was created. |
| `update_time` | String |  | Output only. Timestamp when this RagCorpus was last updated. |
| `corpus_status` | String |  | Output only. RagCorpus state. |
| `encryption_spec` | String |  | Optional. Immutable. The CMEK key name used to encrypt at-rest data related to this Corpus. Only applicable to RagManagedDb option for Vector DB. This field can only be set at corpus creation time, and cannot be updated or deleted. |
| `vertex_ai_search_config` | String |  | Optional. Immutable. The config for the Vertex AI Search. |
| `display_name` | String |  | Required. The display name of the RagCorpus. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `description` | String |  | Optional. The description of the RagCorpus. |
| `vector_db_config` | String |  | Optional. Immutable. The config for the Vector DBs. |
| `name` | String |  | Output only. The resource name of the RagCorpus. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the RagCorpus in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when this RagCorpus was created. |
| `update_time` | String | Output only. Timestamp when this RagCorpus was last updated. |
| `corpus_status` | String | Output only. RagCorpus state. |
| `encryption_spec` | String | Optional. Immutable. The CMEK key name used to encrypt at-rest data related to this Corpus. Only applicable to RagManagedDb option for Vector DB. This field can only be set at corpus creation time, and cannot be updated or deleted. |
| `vertex_ai_search_config` | String | Optional. Immutable. The config for the Vertex AI Search. |
| `display_name` | String | Required. The display name of the RagCorpus. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `description` | String | Optional. The description of the RagCorpus. |
| `vector_db_config` | String | Optional. Immutable. The config for the Vector DBs. |
| `name` | String | Output only. The resource name of the RagCorpus. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rag_corpora
rag_corpora = provider.aiplatform_api.Rag_corpora {
    parent = "value"  # Required. The resource name of the Location to create the RagCorpus in. Format: `projects/{project}/locations/{location}`
}

# Access rag_corpora outputs
rag_corpora_id = rag_corpora.id
rag_corpora_create_time = rag_corpora.create_time
rag_corpora_update_time = rag_corpora.update_time
rag_corpora_corpus_status = rag_corpora.corpus_status
rag_corpora_encryption_spec = rag_corpora.encryption_spec
rag_corpora_vertex_ai_search_config = rag_corpora.vertex_ai_search_config
rag_corpora_display_name = rag_corpora.display_name
rag_corpora_description = rag_corpora.description
rag_corpora_vector_db_config = rag_corpora.vector_db_config
rag_corpora_name = rag_corpora.name
```

---


### Feature

Creates a new Feature in a given FeatureGroup.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disable_monitoring` | bool |  | Optional. Only applicable for Vertex AI Feature Store (Legacy). If not set, use the monitoring_config defined for the EntityType this Feature belongs to. Only Features with type (Feature.ValueType) BOOL, STRING, DOUBLE or INT64 can enable monitoring. If set to true, all types of data monitoring are disabled despite the config on EntityType. |
| `point_of_contact` | String |  | Entity responsible for maintaining this feature. Can be comma separated list of email addresses or URIs. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your Features. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one Feature (System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `name` | String |  | Immutable. Name of the Feature. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}/features/{feature}` `projects/{project}/locations/{location}/featureGroups/{feature_group}/features/{feature}` The last part feature is assigned by the client. The feature can be up to 64 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscore(_), and ASCII digits 0-9 starting with a letter. The value will be unique given an entity type. |
| `update_time` | String |  | Output only. Only applicable for Vertex AI Feature Store (Legacy). Timestamp when this EntityType was most recently updated. |
| `version_column_name` | String |  | Only applicable for Vertex AI Feature Store. The name of the BigQuery Table/View column hosting data for this version. If no value is provided, will use feature_id. |
| `description` | String |  | Description of the Feature. |
| `value_type` | String |  | Immutable. Only applicable for Vertex AI Feature Store (Legacy). Type of Feature value. |
| `etag` | String |  | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `create_time` | String |  | Output only. Only applicable for Vertex AI Feature Store (Legacy). Timestamp when this EntityType was created. |
| `monitoring_stats_anomalies` | Vec<String> |  | Output only. Only applicable for Vertex AI Feature Store (Legacy). The list of historical stats and anomalies with specified objectives. |
| `parent` | String | ✅ | Required. The resource name of the EntityType or FeatureGroup to create a Feature. Format for entity_type as parent: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}` Format for feature_group as parent: `projects/{project}/locations/{location}/featureGroups/{feature_group}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disable_monitoring` | bool | Optional. Only applicable for Vertex AI Feature Store (Legacy). If not set, use the monitoring_config defined for the EntityType this Feature belongs to. Only Features with type (Feature.ValueType) BOOL, STRING, DOUBLE or INT64 can enable monitoring. If set to true, all types of data monitoring are disabled despite the config on EntityType. |
| `point_of_contact` | String | Entity responsible for maintaining this feature. Can be comma separated list of email addresses or URIs. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your Features. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one Feature (System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `name` | String | Immutable. Name of the Feature. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}/features/{feature}` `projects/{project}/locations/{location}/featureGroups/{feature_group}/features/{feature}` The last part feature is assigned by the client. The feature can be up to 64 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscore(_), and ASCII digits 0-9 starting with a letter. The value will be unique given an entity type. |
| `update_time` | String | Output only. Only applicable for Vertex AI Feature Store (Legacy). Timestamp when this EntityType was most recently updated. |
| `version_column_name` | String | Only applicable for Vertex AI Feature Store. The name of the BigQuery Table/View column hosting data for this version. If no value is provided, will use feature_id. |
| `description` | String | Description of the Feature. |
| `value_type` | String | Immutable. Only applicable for Vertex AI Feature Store (Legacy). Type of Feature value. |
| `etag` | String | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `create_time` | String | Output only. Only applicable for Vertex AI Feature Store (Legacy). Timestamp when this EntityType was created. |
| `monitoring_stats_anomalies` | Vec<String> | Output only. Only applicable for Vertex AI Feature Store (Legacy). The list of historical stats and anomalies with specified objectives. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature
feature = provider.aiplatform_api.Feature {
    parent = "value"  # Required. The resource name of the EntityType or FeatureGroup to create a Feature. Format for entity_type as parent: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}` Format for feature_group as parent: `projects/{project}/locations/{location}/featureGroups/{feature_group}`
}

# Access feature outputs
feature_id = feature.id
feature_disable_monitoring = feature.disable_monitoring
feature_point_of_contact = feature.point_of_contact
feature_labels = feature.labels
feature_name = feature.name
feature_update_time = feature.update_time
feature_version_column_name = feature.version_column_name
feature_description = feature.description
feature_value_type = feature.value_type
feature_etag = feature.etag
feature_create_time = feature.create_time
feature_monitoring_stats_anomalies = feature.monitoring_stats_anomalies
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.aiplatform_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_name = operation.name
```

---


### Notebook_runtime_template

Creates a NotebookRuntimeTemplate.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description of the NotebookRuntimeTemplate. |
| `display_name` | String |  | Required. The display name of the NotebookRuntimeTemplate. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `data_persistent_disk_spec` | String |  | Optional. The specification of persistent disk attached to the runtime as data disk storage. |
| `is_default` | bool |  | Output only. Deprecated: This field has no behavior. Use notebook_runtime_type = 'ONE_CLICK' instead. The default template to use if not specified. |
| `reservation_affinity` | String |  | Optional. Reservation Affinity of the notebook runtime template. |
| `notebook_runtime_type` | String |  | Optional. Immutable. The type of the notebook runtime template. |
| `machine_spec` | String |  | Optional. Immutable. The specification of a single machine for the template. |
| `network_tags` | Vec<String> |  | Optional. The Compute Engine tags to add to runtime (see [Tagging instances](https://cloud.google.com/vpc/docs/add-remove-network-tags)). |
| `service_account` | String |  | Deprecated: This field is ignored and the "Vertex AI Notebook Service Account" (service-PROJECT_NUMBER@gcp-sa-aiplatform-vm.iam.gserviceaccount.com) is used for the runtime workload identity. See https://cloud.google.com/iam/docs/service-agents#vertex-ai-notebook-service-account for more details. For NotebookExecutionJob, use NotebookExecutionJob.service_account instead. The service account that the runtime workload runs as. You can use any service account within the same project, but you must have the service account user permission to use the instance. If not specified, the [Compute Engine default service account](https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used. |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `network_spec` | String |  | Optional. Network spec. |
| `name` | String |  | The resource name of the NotebookRuntimeTemplate. |
| `software_config` | String |  | Optional. The notebook software configuration of the notebook runtime. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for the notebook runtime. |
| `idle_shutdown_config` | String |  | The idle shutdown configuration of NotebookRuntimeTemplate. This config will only be set when idle shutdown is enabled. |
| `shielded_vm_config` | String |  | Optional. Immutable. Runtime Shielded VM spec. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize the NotebookRuntimeTemplates. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `create_time` | String |  | Output only. Timestamp when this NotebookRuntimeTemplate was created. |
| `update_time` | String |  | Output only. Timestamp when this NotebookRuntimeTemplate was most recently updated. |
| `euc_config` | String |  | EUC configuration of the NotebookRuntimeTemplate. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the NotebookRuntimeTemplate. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | The description of the NotebookRuntimeTemplate. |
| `display_name` | String | Required. The display name of the NotebookRuntimeTemplate. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `data_persistent_disk_spec` | String | Optional. The specification of persistent disk attached to the runtime as data disk storage. |
| `is_default` | bool | Output only. Deprecated: This field has no behavior. Use notebook_runtime_type = 'ONE_CLICK' instead. The default template to use if not specified. |
| `reservation_affinity` | String | Optional. Reservation Affinity of the notebook runtime template. |
| `notebook_runtime_type` | String | Optional. Immutable. The type of the notebook runtime template. |
| `machine_spec` | String | Optional. Immutable. The specification of a single machine for the template. |
| `network_tags` | Vec<String> | Optional. The Compute Engine tags to add to runtime (see [Tagging instances](https://cloud.google.com/vpc/docs/add-remove-network-tags)). |
| `service_account` | String | Deprecated: This field is ignored and the "Vertex AI Notebook Service Account" (service-PROJECT_NUMBER@gcp-sa-aiplatform-vm.iam.gserviceaccount.com) is used for the runtime workload identity. See https://cloud.google.com/iam/docs/service-agents#vertex-ai-notebook-service-account for more details. For NotebookExecutionJob, use NotebookExecutionJob.service_account instead. The service account that the runtime workload runs as. You can use any service account within the same project, but you must have the service account user permission to use the instance. If not specified, the [Compute Engine default service account](https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used. |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `network_spec` | String | Optional. Network spec. |
| `name` | String | The resource name of the NotebookRuntimeTemplate. |
| `software_config` | String | Optional. The notebook software configuration of the notebook runtime. |
| `encryption_spec` | String | Customer-managed encryption key spec for the notebook runtime. |
| `idle_shutdown_config` | String | The idle shutdown configuration of NotebookRuntimeTemplate. This config will only be set when idle shutdown is enabled. |
| `shielded_vm_config` | String | Optional. Immutable. Runtime Shielded VM spec. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize the NotebookRuntimeTemplates. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `create_time` | String | Output only. Timestamp when this NotebookRuntimeTemplate was created. |
| `update_time` | String | Output only. Timestamp when this NotebookRuntimeTemplate was most recently updated. |
| `euc_config` | String | EUC configuration of the NotebookRuntimeTemplate. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notebook_runtime_template
notebook_runtime_template = provider.aiplatform_api.Notebook_runtime_template {
    parent = "value"  # Required. The resource name of the Location to create the NotebookRuntimeTemplate. Format: `projects/{project}/locations/{location}`
}

# Access notebook_runtime_template outputs
notebook_runtime_template_id = notebook_runtime_template.id
notebook_runtime_template_description = notebook_runtime_template.description
notebook_runtime_template_display_name = notebook_runtime_template.display_name
notebook_runtime_template_data_persistent_disk_spec = notebook_runtime_template.data_persistent_disk_spec
notebook_runtime_template_is_default = notebook_runtime_template.is_default
notebook_runtime_template_reservation_affinity = notebook_runtime_template.reservation_affinity
notebook_runtime_template_notebook_runtime_type = notebook_runtime_template.notebook_runtime_type
notebook_runtime_template_machine_spec = notebook_runtime_template.machine_spec
notebook_runtime_template_network_tags = notebook_runtime_template.network_tags
notebook_runtime_template_service_account = notebook_runtime_template.service_account
notebook_runtime_template_etag = notebook_runtime_template.etag
notebook_runtime_template_network_spec = notebook_runtime_template.network_spec
notebook_runtime_template_name = notebook_runtime_template.name
notebook_runtime_template_software_config = notebook_runtime_template.software_config
notebook_runtime_template_encryption_spec = notebook_runtime_template.encryption_spec
notebook_runtime_template_idle_shutdown_config = notebook_runtime_template.idle_shutdown_config
notebook_runtime_template_shielded_vm_config = notebook_runtime_template.shielded_vm_config
notebook_runtime_template_labels = notebook_runtime_template.labels
notebook_runtime_template_create_time = notebook_runtime_template.create_time
notebook_runtime_template_update_time = notebook_runtime_template.update_time
notebook_runtime_template_euc_config = notebook_runtime_template.euc_config
```

---


### Specialist_pool

Creates a SpecialistPool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pending_data_labeling_jobs` | Vec<String> |  | Output only. The resource name of the pending data labeling jobs. |
| `specialist_worker_emails` | Vec<String> |  | The email addresses of workers in the SpecialistPool. |
| `display_name` | String |  | Required. The user-defined name of the SpecialistPool. The name can be up to 128 characters long and can consist of any UTF-8 characters. This field should be unique on project-level. |
| `specialist_manager_emails` | Vec<String> |  | The email addresses of the managers in the SpecialistPool. |
| `name` | String |  | Required. The resource name of the SpecialistPool. |
| `specialist_managers_count` | i64 |  | Output only. The number of managers in this SpecialistPool. |
| `parent` | String | ✅ | Required. The parent Project name for the new SpecialistPool. The form is `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pending_data_labeling_jobs` | Vec<String> | Output only. The resource name of the pending data labeling jobs. |
| `specialist_worker_emails` | Vec<String> | The email addresses of workers in the SpecialistPool. |
| `display_name` | String | Required. The user-defined name of the SpecialistPool. The name can be up to 128 characters long and can consist of any UTF-8 characters. This field should be unique on project-level. |
| `specialist_manager_emails` | Vec<String> | The email addresses of the managers in the SpecialistPool. |
| `name` | String | Required. The resource name of the SpecialistPool. |
| `specialist_managers_count` | i64 | Output only. The number of managers in this SpecialistPool. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create specialist_pool
specialist_pool = provider.aiplatform_api.Specialist_pool {
    parent = "value"  # Required. The parent Project name for the new SpecialistPool. The form is `projects/{project}/locations/{location}`.
}

# Access specialist_pool outputs
specialist_pool_id = specialist_pool.id
specialist_pool_pending_data_labeling_jobs = specialist_pool.pending_data_labeling_jobs
specialist_pool_specialist_worker_emails = specialist_pool.specialist_worker_emails
specialist_pool_display_name = specialist_pool.display_name
specialist_pool_specialist_manager_emails = specialist_pool.specialist_manager_emails
specialist_pool_name = specialist_pool.name
specialist_pool_specialist_managers_count = specialist_pool.specialist_managers_count
```

---


### Migratable_resource

Batch migrates resources from ml.googleapis.com, automl.googleapis.com, and datalabeling.googleapis.com to Vertex AI.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `migrate_resource_requests` | Vec<String> |  | Required. The request messages specifying the resources to migrate. They must be in the same location as the destination. Up to 50 resources can be migrated in one batch. |
| `parent` | String | ✅ | Required. The location of the migrated resource will live in. Format: `projects/{project}/locations/{location}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create migratable_resource
migratable_resource = provider.aiplatform_api.Migratable_resource {
    parent = "value"  # Required. The location of the migrated resource will live in. Format: `projects/{project}/locations/{location}`
}

```

---


### Index_endpoint

Creates an IndexEndpoint.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `private_service_connect_config` | String |  | Optional. Configuration for private service connect. network and private_service_connect_config are mutually exclusive. |
| `deployed_indexes` | Vec<String> |  | Output only. The indexes deployed in this endpoint. |
| `encryption_spec` | String |  | Immutable. Customer-managed encryption key spec for an IndexEndpoint. If set, this IndexEndpoint and all sub-resources of this IndexEndpoint will be secured by this key. |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `display_name` | String |  | Required. The display name of the IndexEndpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `name` | String |  | Output only. The resource name of the IndexEndpoint. |
| `update_time` | String |  | Output only. Timestamp when this IndexEndpoint was last updated. This timestamp is not updated when the endpoint's DeployedIndexes are updated, e.g. due to updates of the original Indexes they are the deployments of. |
| `description` | String |  | The description of the IndexEndpoint. |
| `create_time` | String |  | Output only. Timestamp when this IndexEndpoint was created. |
| `public_endpoint_domain_name` | String |  | Output only. If public_endpoint_enabled is true, this field will be populated with the domain name to use for this index endpoint. |
| `enable_private_service_connect` | bool |  | Optional. Deprecated: If true, expose the IndexEndpoint via private service connect. Only one of the fields, network or enable_private_service_connect, can be set. |
| `public_endpoint_enabled` | bool |  | Optional. If true, the deployed index will be accessible through public endpoint. |
| `network` | String |  | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the IndexEndpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. network and private_service_connect_config are mutually exclusive. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in '12345', and {network} is network name. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your IndexEndpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the IndexEndpoint in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `private_service_connect_config` | String | Optional. Configuration for private service connect. network and private_service_connect_config are mutually exclusive. |
| `deployed_indexes` | Vec<String> | Output only. The indexes deployed in this endpoint. |
| `encryption_spec` | String | Immutable. Customer-managed encryption key spec for an IndexEndpoint. If set, this IndexEndpoint and all sub-resources of this IndexEndpoint will be secured by this key. |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `display_name` | String | Required. The display name of the IndexEndpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `name` | String | Output only. The resource name of the IndexEndpoint. |
| `update_time` | String | Output only. Timestamp when this IndexEndpoint was last updated. This timestamp is not updated when the endpoint's DeployedIndexes are updated, e.g. due to updates of the original Indexes they are the deployments of. |
| `description` | String | The description of the IndexEndpoint. |
| `create_time` | String | Output only. Timestamp when this IndexEndpoint was created. |
| `public_endpoint_domain_name` | String | Output only. If public_endpoint_enabled is true, this field will be populated with the domain name to use for this index endpoint. |
| `enable_private_service_connect` | bool | Optional. Deprecated: If true, expose the IndexEndpoint via private service connect. Only one of the fields, network or enable_private_service_connect, can be set. |
| `public_endpoint_enabled` | bool | Optional. If true, the deployed index will be accessible through public endpoint. |
| `network` | String | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the IndexEndpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. network and private_service_connect_config are mutually exclusive. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in '12345', and {network} is network name. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your IndexEndpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create index_endpoint
index_endpoint = provider.aiplatform_api.Index_endpoint {
    parent = "value"  # Required. The resource name of the Location to create the IndexEndpoint in. Format: `projects/{project}/locations/{location}`
}

# Access index_endpoint outputs
index_endpoint_id = index_endpoint.id
index_endpoint_private_service_connect_config = index_endpoint.private_service_connect_config
index_endpoint_deployed_indexes = index_endpoint.deployed_indexes
index_endpoint_encryption_spec = index_endpoint.encryption_spec
index_endpoint_etag = index_endpoint.etag
index_endpoint_display_name = index_endpoint.display_name
index_endpoint_name = index_endpoint.name
index_endpoint_update_time = index_endpoint.update_time
index_endpoint_description = index_endpoint.description
index_endpoint_create_time = index_endpoint.create_time
index_endpoint_public_endpoint_domain_name = index_endpoint.public_endpoint_domain_name
index_endpoint_enable_private_service_connect = index_endpoint.enable_private_service_connect
index_endpoint_public_endpoint_enabled = index_endpoint.public_endpoint_enabled
index_endpoint_network = index_endpoint.network
index_endpoint_satisfies_pzs = index_endpoint.satisfies_pzs
index_endpoint_satisfies_pzi = index_endpoint.satisfies_pzi
index_endpoint_labels = index_endpoint.labels
```

---


### Feature_group

Creates a new FeatureGroup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. Description of the FeatureGroup. |
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `create_time` | String |  | Output only. Timestamp when this FeatureGroup was created. |
| `service_account_email` | String |  | Output only. A Service Account unique to this FeatureGroup. The role bigquery.dataViewer should be granted to this service account to allow Vertex AI Feature Store to access source data while running jobs under this FeatureGroup. |
| `big_query` | String |  | Indicates that features for this group come from BigQuery Table/View. By default treats the source as a sparse time series source. The BigQuery source table or view must have at least one entity ID column and a column named `feature_timestamp`. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your FeatureGroup. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureGroup(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `name` | String |  | Identifier. Name of the FeatureGroup. Format: `projects/{project}/locations/{location}/featureGroups/{featureGroup}` |
| `update_time` | String |  | Output only. Timestamp when this FeatureGroup was last updated. |
| `service_agent_type` | String |  | Optional. Service agent type used during jobs under a FeatureGroup. By default, the Vertex AI Service Agent is used. When using an IAM Policy to isolate this FeatureGroup within a project, a separate service account should be provisioned by setting this field to `SERVICE_AGENT_TYPE_FEATURE_GROUP`. This will generate a separate service account to access the BigQuery source table. |
| `parent` | String | ✅ | Required. The resource name of the Location to create FeatureGroups. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. Description of the FeatureGroup. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `create_time` | String | Output only. Timestamp when this FeatureGroup was created. |
| `service_account_email` | String | Output only. A Service Account unique to this FeatureGroup. The role bigquery.dataViewer should be granted to this service account to allow Vertex AI Feature Store to access source data while running jobs under this FeatureGroup. |
| `big_query` | String | Indicates that features for this group come from BigQuery Table/View. By default treats the source as a sparse time series source. The BigQuery source table or view must have at least one entity ID column and a column named `feature_timestamp`. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your FeatureGroup. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureGroup(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `name` | String | Identifier. Name of the FeatureGroup. Format: `projects/{project}/locations/{location}/featureGroups/{featureGroup}` |
| `update_time` | String | Output only. Timestamp when this FeatureGroup was last updated. |
| `service_agent_type` | String | Optional. Service agent type used during jobs under a FeatureGroup. By default, the Vertex AI Service Agent is used. When using an IAM Policy to isolate this FeatureGroup within a project, a separate service account should be provisioned by setting this field to `SERVICE_AGENT_TYPE_FEATURE_GROUP`. This will generate a separate service account to access the BigQuery source table. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature_group
feature_group = provider.aiplatform_api.Feature_group {
    parent = "value"  # Required. The resource name of the Location to create FeatureGroups. Format: `projects/{project}/locations/{location}`
}

# Access feature_group outputs
feature_group_id = feature_group.id
feature_group_description = feature_group.description
feature_group_etag = feature_group.etag
feature_group_create_time = feature_group.create_time
feature_group_service_account_email = feature_group.service_account_email
feature_group_big_query = feature_group.big_query
feature_group_labels = feature_group.labels
feature_group_name = feature_group.name
feature_group_update_time = feature_group.update_time
feature_group_service_agent_type = feature_group.service_agent_type
```

---


### Artifact

Creates an Artifact associated with a MetadataStore.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Artifacts. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Artifact (System labels are excluded). |
| `display_name` | String |  | User provided display name of the Artifact. May be up to 128 Unicode characters. |
| `schema_version` | String |  | The version of the schema in schema_name to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `metadata` | HashMap<String, String> |  | Properties of the Artifact. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |
| `schema_title` | String |  | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `update_time` | String |  | Output only. Timestamp when this Artifact was last updated. |
| `uri` | String |  | The uniform resource identifier of the artifact file. May be empty if there is no actual artifact file. |
| `etag` | String |  | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String |  | Output only. The resource name of the Artifact. |
| `state` | String |  | The state of this Artifact. This is a property of the Artifact, and does not imply or capture any ongoing process. This property is managed by clients (such as Vertex AI Pipelines), and the system does not prescribe or check the validity of state transitions. |
| `create_time` | String |  | Output only. Timestamp when this Artifact was created. |
| `description` | String |  | Description of the Artifact |
| `parent` | String | ✅ | Required. The resource name of the MetadataStore where the Artifact should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Artifacts. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Artifact (System labels are excluded). |
| `display_name` | String | User provided display name of the Artifact. May be up to 128 Unicode characters. |
| `schema_version` | String | The version of the schema in schema_name to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `metadata` | HashMap<String, String> | Properties of the Artifact. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |
| `schema_title` | String | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `update_time` | String | Output only. Timestamp when this Artifact was last updated. |
| `uri` | String | The uniform resource identifier of the artifact file. May be empty if there is no actual artifact file. |
| `etag` | String | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String | Output only. The resource name of the Artifact. |
| `state` | String | The state of this Artifact. This is a property of the Artifact, and does not imply or capture any ongoing process. This property is managed by clients (such as Vertex AI Pipelines), and the system does not prescribe or check the validity of state transitions. |
| `create_time` | String | Output only. Timestamp when this Artifact was created. |
| `description` | String | Description of the Artifact |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create artifact
artifact = provider.aiplatform_api.Artifact {
    parent = "value"  # Required. The resource name of the MetadataStore where the Artifact should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}`
}

# Access artifact outputs
artifact_id = artifact.id
artifact_labels = artifact.labels
artifact_display_name = artifact.display_name
artifact_schema_version = artifact.schema_version
artifact_metadata = artifact.metadata
artifact_schema_title = artifact.schema_title
artifact_update_time = artifact.update_time
artifact_uri = artifact.uri
artifact_etag = artifact.etag
artifact_name = artifact.name
artifact_state = artifact.state
artifact_create_time = artifact.create_time
artifact_description = artifact.description
```

---


### Media

Upload a file into a RagCorpus.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rag_file` | String |  | Required. The RagFile to upload. |
| `upload_rag_file_config` | String |  | Required. The config for the RagFiles to be uploaded into the RagCorpus. VertexRagDataService.UploadRagFile. |
| `parent` | String | ✅ | Required. The name of the RagCorpus resource into which to upload the file. Format: `projects/{project}/locations/{location}/ragCorpora/{rag_corpus}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.aiplatform_api.Media {
    parent = "value"  # Required. The name of the RagCorpus resource into which to upload the file. Format: `projects/{project}/locations/{location}/ragCorpora/{rag_corpus}`
}

```

---


### Entity_type

Creates a new EntityType in a given Featurestore.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `create_time` | String |  | Output only. Timestamp when this EntityType was created. |
| `name` | String |  | Immutable. Name of the EntityType. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}` The last part entity_type is assigned by the client. The entity_type can be up to 64 characters long and can consist only of ASCII Latin letters A-Z and a-z and underscore(_), and ASCII digits 0-9 starting with a letter. The value will be unique given a featurestore. |
| `update_time` | String |  | Output only. Timestamp when this EntityType was most recently updated. |
| `monitoring_config` | String |  | Optional. The default monitoring configuration for all Features with value type (Feature.ValueType) BOOL, STRING, DOUBLE or INT64 under this EntityType. If this is populated with [FeaturestoreMonitoringConfig.monitoring_interval] specified, snapshot analysis monitoring is enabled. Otherwise, snapshot analysis monitoring is disabled. |
| `offline_storage_ttl_days` | i64 |  | Optional. Config for data retention policy in offline storage. TTL in days for feature values that will be stored in offline storage. The Feature Store offline storage periodically removes obsolete feature values older than `offline_storage_ttl_days` since the feature generation time. If unset (or explicitly set to 0), default to 4000 days TTL. |
| `etag` | String |  | Optional. Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `description` | String |  | Optional. Description of the EntityType. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your EntityTypes. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one EntityType (System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `parent` | String | ✅ | Required. The resource name of the Featurestore to create EntityTypes. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `create_time` | String | Output only. Timestamp when this EntityType was created. |
| `name` | String | Immutable. Name of the EntityType. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}` The last part entity_type is assigned by the client. The entity_type can be up to 64 characters long and can consist only of ASCII Latin letters A-Z and a-z and underscore(_), and ASCII digits 0-9 starting with a letter. The value will be unique given a featurestore. |
| `update_time` | String | Output only. Timestamp when this EntityType was most recently updated. |
| `monitoring_config` | String | Optional. The default monitoring configuration for all Features with value type (Feature.ValueType) BOOL, STRING, DOUBLE or INT64 under this EntityType. If this is populated with [FeaturestoreMonitoringConfig.monitoring_interval] specified, snapshot analysis monitoring is enabled. Otherwise, snapshot analysis monitoring is disabled. |
| `offline_storage_ttl_days` | i64 | Optional. Config for data retention policy in offline storage. TTL in days for feature values that will be stored in offline storage. The Feature Store offline storage periodically removes obsolete feature values older than `offline_storage_ttl_days` since the feature generation time. If unset (or explicitly set to 0), default to 4000 days TTL. |
| `etag` | String | Optional. Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `description` | String | Optional. Description of the EntityType. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your EntityTypes. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one EntityType (System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entity_type
entity_type = provider.aiplatform_api.Entity_type {
    parent = "value"  # Required. The resource name of the Featurestore to create EntityTypes. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}`
}

# Access entity_type outputs
entity_type_id = entity_type.id
entity_type_satisfies_pzs = entity_type.satisfies_pzs
entity_type_create_time = entity_type.create_time
entity_type_name = entity_type.name
entity_type_update_time = entity_type.update_time
entity_type_monitoring_config = entity_type.monitoring_config
entity_type_offline_storage_ttl_days = entity_type.offline_storage_ttl_days
entity_type_etag = entity_type.etag
entity_type_description = entity_type.description
entity_type_satisfies_pzi = entity_type.satisfies_pzi
entity_type_labels = entity_type.labels
```

---


### Persistent_resource

Creates a PersistentResource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_runtime` | String |  | Output only. Runtime information of the Persistent Resource. |
| `resource_runtime_spec` | String |  | Optional. Persistent Resource runtime spec. For example, used for Ray cluster configuration. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `display_name` | String |  | Optional. The display name of the PersistentResource. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize PersistentResource. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `psc_interface_config` | String |  | Optional. Configuration for PSC-I for PersistentResource. |
| `encryption_spec` | String |  | Optional. Customer-managed encryption key spec for a PersistentResource. If set, this PersistentResource and all sub-resources of this PersistentResource will be secured by this key. |
| `create_time` | String |  | Output only. Time when the PersistentResource was created. |
| `resource_pools` | Vec<String> |  | Required. The spec of the pools of different resources. |
| `error` | String |  | Output only. Only populated when persistent resource's state is `STOPPING` or `ERROR`. |
| `reserved_ip_ranges` | Vec<String> |  | Optional. A list of names for the reserved IP ranges under the VPC network that can be used for this persistent resource. If set, we will deploy the persistent resource within the provided IP ranges. Otherwise, the persistent resource is deployed to any IP ranges under the provided VPC network. Example: ['vertex-ai-ip-range']. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `start_time` | String |  | Output only. Time when the PersistentResource for the first time entered the `RUNNING` state. |
| `state` | String |  | Output only. The detailed state of a Study. |
| `update_time` | String |  | Output only. Time when the PersistentResource was most recently updated. |
| `network` | String |  | Optional. The full name of the Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to peered with Vertex AI to host the persistent resources. For example, `projects/12345/global/networks/myVPC`. [Format](/compute/docs/reference/rest/v1/networks/insert) is of the form `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in `12345`, and {network} is a network name. To specify this field, you must have already [configured VPC Network Peering for Vertex AI](https://cloud.google.com/vertex-ai/docs/general/vpc-peering). If this field is left unspecified, the resources aren't peered with any network. |
| `name` | String |  | Immutable. Resource name of a PersistentResource. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the PersistentResource in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_runtime` | String | Output only. Runtime information of the Persistent Resource. |
| `resource_runtime_spec` | String | Optional. Persistent Resource runtime spec. For example, used for Ray cluster configuration. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `display_name` | String | Optional. The display name of the PersistentResource. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize PersistentResource. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `psc_interface_config` | String | Optional. Configuration for PSC-I for PersistentResource. |
| `encryption_spec` | String | Optional. Customer-managed encryption key spec for a PersistentResource. If set, this PersistentResource and all sub-resources of this PersistentResource will be secured by this key. |
| `create_time` | String | Output only. Time when the PersistentResource was created. |
| `resource_pools` | Vec<String> | Required. The spec of the pools of different resources. |
| `error` | String | Output only. Only populated when persistent resource's state is `STOPPING` or `ERROR`. |
| `reserved_ip_ranges` | Vec<String> | Optional. A list of names for the reserved IP ranges under the VPC network that can be used for this persistent resource. If set, we will deploy the persistent resource within the provided IP ranges. Otherwise, the persistent resource is deployed to any IP ranges under the provided VPC network. Example: ['vertex-ai-ip-range']. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `start_time` | String | Output only. Time when the PersistentResource for the first time entered the `RUNNING` state. |
| `state` | String | Output only. The detailed state of a Study. |
| `update_time` | String | Output only. Time when the PersistentResource was most recently updated. |
| `network` | String | Optional. The full name of the Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to peered with Vertex AI to host the persistent resources. For example, `projects/12345/global/networks/myVPC`. [Format](/compute/docs/reference/rest/v1/networks/insert) is of the form `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in `12345`, and {network} is a network name. To specify this field, you must have already [configured VPC Network Peering for Vertex AI](https://cloud.google.com/vertex-ai/docs/general/vpc-peering). If this field is left unspecified, the resources aren't peered with any network. |
| `name` | String | Immutable. Resource name of a PersistentResource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create persistent_resource
persistent_resource = provider.aiplatform_api.Persistent_resource {
    parent = "value"  # Required. The resource name of the Location to create the PersistentResource in. Format: `projects/{project}/locations/{location}`
}

# Access persistent_resource outputs
persistent_resource_id = persistent_resource.id
persistent_resource_resource_runtime = persistent_resource.resource_runtime
persistent_resource_resource_runtime_spec = persistent_resource.resource_runtime_spec
persistent_resource_satisfies_pzi = persistent_resource.satisfies_pzi
persistent_resource_display_name = persistent_resource.display_name
persistent_resource_labels = persistent_resource.labels
persistent_resource_psc_interface_config = persistent_resource.psc_interface_config
persistent_resource_encryption_spec = persistent_resource.encryption_spec
persistent_resource_create_time = persistent_resource.create_time
persistent_resource_resource_pools = persistent_resource.resource_pools
persistent_resource_error = persistent_resource.error
persistent_resource_reserved_ip_ranges = persistent_resource.reserved_ip_ranges
persistent_resource_satisfies_pzs = persistent_resource.satisfies_pzs
persistent_resource_start_time = persistent_resource.start_time
persistent_resource_state = persistent_resource.state
persistent_resource_update_time = persistent_resource.update_time
persistent_resource_network = persistent_resource.network
persistent_resource_name = persistent_resource.name
```

---


### Dataset

Creates a Dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description of the Dataset. |
| `metadata` | String |  | Required. Additional information about the Dataset. |
| `display_name` | String |  | Required. The user-defined name of the Dataset. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `saved_queries` | Vec<String> |  | All SavedQueries belong to the Dataset will be returned in List/Get Dataset response. The annotation_specs field will not be populated except for UI cases which will only use annotation_spec_count. In CreateDataset request, a SavedQuery is created together if this field is set, up to one SavedQuery can be set in CreateDatasetRequest. The SavedQuery should not contain any AnnotationSpec. |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `model_reference` | String |  | Optional. Reference to the public base model last used by the dataset. Only set for prompt datasets. |
| `metadata_schema_uri` | String |  | Required. Points to a YAML file stored on Google Cloud Storage describing additional information about the Dataset. The schema is defined as an OpenAPI 3.0.2 Schema Object. The schema files that can be used here are found in gs://google-cloud-aiplatform/schema/dataset/metadata/. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Datasets. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Dataset (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. Following system labels exist for each Dataset: * "aiplatform.googleapis.com/dataset_metadata_schema": output only, its value is the metadata_schema's title. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a Dataset. If set, this Dataset and all sub-resources of this Dataset will be secured by this key. |
| `data_item_count` | String |  | Output only. The number of DataItems in this Dataset. Only apply for non-structured Dataset. |
| `update_time` | String |  | Output only. Timestamp when this Dataset was last updated. |
| `create_time` | String |  | Output only. Timestamp when this Dataset was created. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `metadata_artifact` | String |  | Output only. The resource name of the Artifact that was created in MetadataStore when creating the Dataset. The Artifact resource name pattern is `projects/{project}/locations/{location}/metadataStores/{metadata_store}/artifacts/{artifact}`. |
| `name` | String |  | Output only. Identifier. The resource name of the Dataset. Format: `projects/{project}/locations/{location}/datasets/{dataset}` |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Dataset in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | The description of the Dataset. |
| `metadata` | String | Required. Additional information about the Dataset. |
| `display_name` | String | Required. The user-defined name of the Dataset. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `saved_queries` | Vec<String> | All SavedQueries belong to the Dataset will be returned in List/Get Dataset response. The annotation_specs field will not be populated except for UI cases which will only use annotation_spec_count. In CreateDataset request, a SavedQuery is created together if this field is set, up to one SavedQuery can be set in CreateDatasetRequest. The SavedQuery should not contain any AnnotationSpec. |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `model_reference` | String | Optional. Reference to the public base model last used by the dataset. Only set for prompt datasets. |
| `metadata_schema_uri` | String | Required. Points to a YAML file stored on Google Cloud Storage describing additional information about the Dataset. The schema is defined as an OpenAPI 3.0.2 Schema Object. The schema files that can be used here are found in gs://google-cloud-aiplatform/schema/dataset/metadata/. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Datasets. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Dataset (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. Following system labels exist for each Dataset: * "aiplatform.googleapis.com/dataset_metadata_schema": output only, its value is the metadata_schema's title. |
| `encryption_spec` | String | Customer-managed encryption key spec for a Dataset. If set, this Dataset and all sub-resources of this Dataset will be secured by this key. |
| `data_item_count` | String | Output only. The number of DataItems in this Dataset. Only apply for non-structured Dataset. |
| `update_time` | String | Output only. Timestamp when this Dataset was last updated. |
| `create_time` | String | Output only. Timestamp when this Dataset was created. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `metadata_artifact` | String | Output only. The resource name of the Artifact that was created in MetadataStore when creating the Dataset. The Artifact resource name pattern is `projects/{project}/locations/{location}/metadataStores/{metadata_store}/artifacts/{artifact}`. |
| `name` | String | Output only. Identifier. The resource name of the Dataset. Format: `projects/{project}/locations/{location}/datasets/{dataset}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset
dataset = provider.aiplatform_api.Dataset {
    parent = "value"  # Required. The resource name of the Location to create the Dataset in. Format: `projects/{project}/locations/{location}`
}

# Access dataset outputs
dataset_id = dataset.id
dataset_description = dataset.description
dataset_metadata = dataset.metadata
dataset_display_name = dataset.display_name
dataset_saved_queries = dataset.saved_queries
dataset_etag = dataset.etag
dataset_model_reference = dataset.model_reference
dataset_metadata_schema_uri = dataset.metadata_schema_uri
dataset_labels = dataset.labels
dataset_encryption_spec = dataset.encryption_spec
dataset_data_item_count = dataset.data_item_count
dataset_update_time = dataset.update_time
dataset_create_time = dataset.create_time
dataset_satisfies_pzs = dataset.satisfies_pzs
dataset_satisfies_pzi = dataset.satisfies_pzi
dataset_metadata_artifact = dataset.metadata_artifact
dataset_name = dataset.name
```

---


### Data_item

Lists DataItems in a Dataset.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The standard List next-page token. |
| `data_items` | Vec<String> | A list of DataItems that matches the specified filter in the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access data_item outputs
data_item_id = data_item.id
data_item_next_page_token = data_item.next_page_token
data_item_data_items = data_item.data_items
```

---


### Deployment_resource_pool

Create a DeploymentResourcePool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deployment_resource_pool_id` | String |  | Required. The ID to use for the DeploymentResourcePool, which will become the final component of the DeploymentResourcePool's resource name. The maximum length is 63 characters, and valid characters are `/^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$/`. |
| `deployment_resource_pool` | String |  | Required. The DeploymentResourcePool to create. |
| `parent` | String | ✅ | Required. The parent location resource where this DeploymentResourcePool will be created. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `dedicated_resources` | String | Required. The underlying DedicatedResources that the DeploymentResourcePool uses. |
| `encryption_spec` | String | Customer-managed encryption key spec for a DeploymentResourcePool. If set, this DeploymentResourcePool will be secured by this key. Endpoints and the DeploymentResourcePool they deploy in need to have the same EncryptionSpec. |
| `service_account` | String | The service account that the DeploymentResourcePool's container(s) run as. Specify the email address of the service account. If this service account is not specified, the container(s) run as a service account that doesn't have access to the resource project. Users deploying the Models to this DeploymentResourcePool must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `disable_container_logging` | bool | If the DeploymentResourcePool is deployed with custom-trained Models or AutoML Tabular Models, the container(s) of the DeploymentResourcePool will send `stderr` and `stdout` streams to Cloud Logging by default. Please note that the logs incur cost, which are subject to [Cloud Logging pricing](https://cloud.google.com/logging/pricing). User can disable container logging by setting this flag to true. |
| `create_time` | String | Output only. Timestamp when this DeploymentResourcePool was created. |
| `name` | String | Immutable. The resource name of the DeploymentResourcePool. Format: `projects/{project}/locations/{location}/deploymentResourcePools/{deployment_resource_pool}` |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deployment_resource_pool
deployment_resource_pool = provider.aiplatform_api.Deployment_resource_pool {
    parent = "value"  # Required. The parent location resource where this DeploymentResourcePool will be created. Format: `projects/{project}/locations/{location}`
}

# Access deployment_resource_pool outputs
deployment_resource_pool_id = deployment_resource_pool.id
deployment_resource_pool_satisfies_pzs = deployment_resource_pool.satisfies_pzs
deployment_resource_pool_dedicated_resources = deployment_resource_pool.dedicated_resources
deployment_resource_pool_encryption_spec = deployment_resource_pool.encryption_spec
deployment_resource_pool_service_account = deployment_resource_pool.service_account
deployment_resource_pool_disable_container_logging = deployment_resource_pool.disable_container_logging
deployment_resource_pool_create_time = deployment_resource_pool.create_time
deployment_resource_pool_name = deployment_resource_pool.name
deployment_resource_pool_satisfies_pzi = deployment_resource_pool.satisfies_pzi
```

---


### Evaluation_run

Creates an Evaluation Run.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Labels for the evaluation run. |
| `state` | String |  | Output only. The state of the evaluation run. |
| `metadata` | String |  | Optional. Metadata about the evaluation run, can be used by the caller to store additional tracking information about the evaluation run. |
| `inference_configs` | HashMap<String, String> |  | Optional. The candidate to inference config map for the evaluation run. The candidate can be up to 128 characters long and can consist of any UTF-8 characters. |
| `data_source` | String |  | Required. The data source for the evaluation run. |
| `evaluation_config` | String |  | Required. The configuration used for the evaluation. |
| `name` | String |  | Identifier. The resource name of the EvaluationRun. This is a unique identifier. Format: `projects/{project}/locations/{location}/evaluationRuns/{evaluation_run}` |
| `error` | String |  | Output only. Only populated when the evaluation run's state is FAILED or CANCELLED. |
| `evaluation_results` | String |  | Output only. The results of the evaluation run. Only populated when the evaluation run's state is SUCCEEDED. |
| `evaluation_set_snapshot` | String |  | Output only. The specific evaluation set of the evaluation run. For runs with an evaluation set input, this will be that same set. For runs with BigQuery input, it's the sampled BigQuery dataset. |
| `completion_time` | String |  | Output only. Time when the evaluation run was completed. |
| `create_time` | String |  | Output only. Time when the evaluation run was created. |
| `display_name` | String |  | Required. The display name of the Evaluation Run. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Evaluation Run in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Labels for the evaluation run. |
| `state` | String | Output only. The state of the evaluation run. |
| `metadata` | String | Optional. Metadata about the evaluation run, can be used by the caller to store additional tracking information about the evaluation run. |
| `inference_configs` | HashMap<String, String> | Optional. The candidate to inference config map for the evaluation run. The candidate can be up to 128 characters long and can consist of any UTF-8 characters. |
| `data_source` | String | Required. The data source for the evaluation run. |
| `evaluation_config` | String | Required. The configuration used for the evaluation. |
| `name` | String | Identifier. The resource name of the EvaluationRun. This is a unique identifier. Format: `projects/{project}/locations/{location}/evaluationRuns/{evaluation_run}` |
| `error` | String | Output only. Only populated when the evaluation run's state is FAILED or CANCELLED. |
| `evaluation_results` | String | Output only. The results of the evaluation run. Only populated when the evaluation run's state is SUCCEEDED. |
| `evaluation_set_snapshot` | String | Output only. The specific evaluation set of the evaluation run. For runs with an evaluation set input, this will be that same set. For runs with BigQuery input, it's the sampled BigQuery dataset. |
| `completion_time` | String | Output only. Time when the evaluation run was completed. |
| `create_time` | String | Output only. Time when the evaluation run was created. |
| `display_name` | String | Required. The display name of the Evaluation Run. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation_run
evaluation_run = provider.aiplatform_api.Evaluation_run {
    parent = "value"  # Required. The resource name of the Location to create the Evaluation Run in. Format: `projects/{project}/locations/{location}`
}

# Access evaluation_run outputs
evaluation_run_id = evaluation_run.id
evaluation_run_labels = evaluation_run.labels
evaluation_run_state = evaluation_run.state
evaluation_run_metadata = evaluation_run.metadata
evaluation_run_inference_configs = evaluation_run.inference_configs
evaluation_run_data_source = evaluation_run.data_source
evaluation_run_evaluation_config = evaluation_run.evaluation_config
evaluation_run_name = evaluation_run.name
evaluation_run_error = evaluation_run.error
evaluation_run_evaluation_results = evaluation_run.evaluation_results
evaluation_run_evaluation_set_snapshot = evaluation_run.evaluation_set_snapshot
evaluation_run_completion_time = evaluation_run.completion_time
evaluation_run_create_time = evaluation_run.create_time
evaluation_run_display_name = evaluation_run.display_name
```

---


### Trial

Adds a user provided Trial to a Study.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Output only. The identifier of the Trial assigned by the service. |
| `client_id` | String |  | Output only. The identifier of the client that originally requested this Trial. Each client is identified by a unique client_id. When a client asks for a suggestion, Vertex AI Vizier will assign it a Trial. The client should evaluate the Trial, complete it, and report back to Vertex AI Vizier. If suggestion is asked again by same client_id before the Trial is completed, the same Trial will be returned. Multiple clients with different client_ids can ask for suggestions simultaneously, each of them will get their own Trial. |
| `custom_job` | String |  | Output only. The CustomJob name linked to the Trial. It's set for a HyperparameterTuningJob's Trial. |
| `end_time` | String |  | Output only. Time when the Trial's status changed to `SUCCEEDED` or `INFEASIBLE`. |
| `final_measurement` | String |  | Output only. The final measurement containing the objective value. |
| `infeasible_reason` | String |  | Output only. A human readable string describing why the Trial is infeasible. This is set only if Trial state is `INFEASIBLE`. |
| `name` | String |  | Output only. Resource name of the Trial assigned by the service. |
| `web_access_uris` | HashMap<String, String> |  | Output only. URIs for accessing [interactive shells](https://cloud.google.com/vertex-ai/docs/training/monitor-debug-interactive-shell) (one URI for each training node). Only available if this trial is part of a HyperparameterTuningJob and the job's trial_job_spec.enable_web_access field is `true`. The keys are names of each node used for the trial; for example, `workerpool0-0` for the primary node, `workerpool1-0` for the first node in the second worker pool, and `workerpool1-1` for the second node in the second worker pool. The values are the URIs for each node's interactive shell. |
| `start_time` | String |  | Output only. Time when the Trial was started. |
| `parameters` | Vec<String> |  | Output only. The parameters of the Trial. |
| `measurements` | Vec<String> |  | Output only. A list of measurements that are strictly lexicographically ordered by their induced tuples (steps, elapsed_duration). These are used for early stopping computations. |
| `state` | String |  | Output only. The detailed state of the Trial. |
| `parent` | String | ✅ | Required. The resource name of the Study to create the Trial in. Format: `projects/{project}/locations/{location}/studies/{study}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Output only. The identifier of the Trial assigned by the service. |
| `client_id` | String | Output only. The identifier of the client that originally requested this Trial. Each client is identified by a unique client_id. When a client asks for a suggestion, Vertex AI Vizier will assign it a Trial. The client should evaluate the Trial, complete it, and report back to Vertex AI Vizier. If suggestion is asked again by same client_id before the Trial is completed, the same Trial will be returned. Multiple clients with different client_ids can ask for suggestions simultaneously, each of them will get their own Trial. |
| `custom_job` | String | Output only. The CustomJob name linked to the Trial. It's set for a HyperparameterTuningJob's Trial. |
| `end_time` | String | Output only. Time when the Trial's status changed to `SUCCEEDED` or `INFEASIBLE`. |
| `final_measurement` | String | Output only. The final measurement containing the objective value. |
| `infeasible_reason` | String | Output only. A human readable string describing why the Trial is infeasible. This is set only if Trial state is `INFEASIBLE`. |
| `name` | String | Output only. Resource name of the Trial assigned by the service. |
| `web_access_uris` | HashMap<String, String> | Output only. URIs for accessing [interactive shells](https://cloud.google.com/vertex-ai/docs/training/monitor-debug-interactive-shell) (one URI for each training node). Only available if this trial is part of a HyperparameterTuningJob and the job's trial_job_spec.enable_web_access field is `true`. The keys are names of each node used for the trial; for example, `workerpool0-0` for the primary node, `workerpool1-0` for the first node in the second worker pool, and `workerpool1-1` for the second node in the second worker pool. The values are the URIs for each node's interactive shell. |
| `start_time` | String | Output only. Time when the Trial was started. |
| `parameters` | Vec<String> | Output only. The parameters of the Trial. |
| `measurements` | Vec<String> | Output only. A list of measurements that are strictly lexicographically ordered by their induced tuples (steps, elapsed_duration). These are used for early stopping computations. |
| `state` | String | Output only. The detailed state of the Trial. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trial
trial = provider.aiplatform_api.Trial {
    parent = "value"  # Required. The resource name of the Study to create the Trial in. Format: `projects/{project}/locations/{location}/studies/{study}`
}

# Access trial outputs
trial_id = trial.id
trial_id = trial.id
trial_client_id = trial.client_id
trial_custom_job = trial.custom_job
trial_end_time = trial.end_time
trial_final_measurement = trial.final_measurement
trial_infeasible_reason = trial.infeasible_reason
trial_name = trial.name
trial_web_access_uris = trial.web_access_uris
trial_start_time = trial.start_time
trial_parameters = trial.parameters
trial_measurements = trial.measurements
trial_state = trial.state
```

---


### Pipeline_job

Creates a PipelineJob. A PipelineJob will run immediately when created.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `preflight_validations` | bool |  | Optional. Whether to do component level validations before job creation. |
| `display_name` | String |  | The display name of the Pipeline. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `service_account` | String |  | The service account that the pipeline workload runs as. If not specified, the Compute Engine default service account in the project will be used. See https://cloud.google.com/compute/docs/access/service-accounts#default_service_account Users starting the pipeline must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `template_metadata` | String |  | Output only. Pipeline template metadata. Will fill up fields if PipelineJob.template_uri is from supported template registry. |
| `template_uri` | String |  | A template uri from where the PipelineJob.pipeline_spec, if empty, will be downloaded. Currently, only uri from Vertex Template Registry & Gallery is supported. Reference to https://cloud.google.com/vertex-ai/docs/pipelines/create-pipeline-template. |
| `create_time` | String |  | Output only. Pipeline creation time. |
| `end_time` | String |  | Output only. Pipeline end time. |
| `job_detail` | String |  | Output only. The details of pipeline run. Not available in the list view. |
| `update_time` | String |  | Output only. Timestamp when this PipelineJob was most recently updated. |
| `state` | String |  | Output only. The detailed state of the job. |
| `runtime_config` | String |  | Runtime config of the pipeline. |
| `start_time` | String |  | Output only. Pipeline start time. |
| `pipeline_spec` | HashMap<String, String> |  | The spec of the pipeline. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a pipelineJob. If set, this PipelineJob and all of its sub-resources will be secured by this key. |
| `error` | String |  | Output only. The error that occurred during pipeline execution. Only populated when the pipeline's state is FAILED or CANCELLED. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize PipelineJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. Note there is some reserved label key for Vertex AI Pipelines. - `vertex-ai-pipelines-run-billing-id`, user set value will get overrided. |
| `psc_interface_config` | String |  | Optional. Configuration for PSC-I for PipelineJob. |
| `network` | String |  | The full name of the Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to which the Pipeline Job's workload should be peered. For example, `projects/12345/global/networks/myVPC`. [Format](/compute/docs/reference/rest/v1/networks/insert) is of the form `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in `12345`, and {network} is a network name. Private services access must already be configured for the network. Pipeline job will apply the network configuration to the Google Cloud resources being launched, if applied, such as Vertex AI Training or Dataflow job. If left unspecified, the workload is not peered with any network. |
| `name` | String |  | Output only. The resource name of the PipelineJob. |
| `reserved_ip_ranges` | Vec<String> |  | A list of names for the reserved ip ranges under the VPC network that can be used for this Pipeline Job's workload. If set, we will deploy the Pipeline Job's workload within the provided ip ranges. Otherwise, the job will be deployed to any ip ranges under the provided VPC network. Example: ['vertex-ai-ip-range']. |
| `schedule_name` | String |  | Output only. The schedule resource name. Only returned if the Pipeline is created by Schedule API. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the PipelineJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `preflight_validations` | bool | Optional. Whether to do component level validations before job creation. |
| `display_name` | String | The display name of the Pipeline. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `service_account` | String | The service account that the pipeline workload runs as. If not specified, the Compute Engine default service account in the project will be used. See https://cloud.google.com/compute/docs/access/service-accounts#default_service_account Users starting the pipeline must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `template_metadata` | String | Output only. Pipeline template metadata. Will fill up fields if PipelineJob.template_uri is from supported template registry. |
| `template_uri` | String | A template uri from where the PipelineJob.pipeline_spec, if empty, will be downloaded. Currently, only uri from Vertex Template Registry & Gallery is supported. Reference to https://cloud.google.com/vertex-ai/docs/pipelines/create-pipeline-template. |
| `create_time` | String | Output only. Pipeline creation time. |
| `end_time` | String | Output only. Pipeline end time. |
| `job_detail` | String | Output only. The details of pipeline run. Not available in the list view. |
| `update_time` | String | Output only. Timestamp when this PipelineJob was most recently updated. |
| `state` | String | Output only. The detailed state of the job. |
| `runtime_config` | String | Runtime config of the pipeline. |
| `start_time` | String | Output only. Pipeline start time. |
| `pipeline_spec` | HashMap<String, String> | The spec of the pipeline. |
| `encryption_spec` | String | Customer-managed encryption key spec for a pipelineJob. If set, this PipelineJob and all of its sub-resources will be secured by this key. |
| `error` | String | Output only. The error that occurred during pipeline execution. Only populated when the pipeline's state is FAILED or CANCELLED. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize PipelineJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. Note there is some reserved label key for Vertex AI Pipelines. - `vertex-ai-pipelines-run-billing-id`, user set value will get overrided. |
| `psc_interface_config` | String | Optional. Configuration for PSC-I for PipelineJob. |
| `network` | String | The full name of the Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to which the Pipeline Job's workload should be peered. For example, `projects/12345/global/networks/myVPC`. [Format](/compute/docs/reference/rest/v1/networks/insert) is of the form `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in `12345`, and {network} is a network name. Private services access must already be configured for the network. Pipeline job will apply the network configuration to the Google Cloud resources being launched, if applied, such as Vertex AI Training or Dataflow job. If left unspecified, the workload is not peered with any network. |
| `name` | String | Output only. The resource name of the PipelineJob. |
| `reserved_ip_ranges` | Vec<String> | A list of names for the reserved ip ranges under the VPC network that can be used for this Pipeline Job's workload. If set, we will deploy the Pipeline Job's workload within the provided ip ranges. Otherwise, the job will be deployed to any ip ranges under the provided VPC network. Example: ['vertex-ai-ip-range']. |
| `schedule_name` | String | Output only. The schedule resource name. Only returned if the Pipeline is created by Schedule API. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create pipeline_job
pipeline_job = provider.aiplatform_api.Pipeline_job {
    parent = "value"  # Required. The resource name of the Location to create the PipelineJob in. Format: `projects/{project}/locations/{location}`
}

# Access pipeline_job outputs
pipeline_job_id = pipeline_job.id
pipeline_job_preflight_validations = pipeline_job.preflight_validations
pipeline_job_display_name = pipeline_job.display_name
pipeline_job_service_account = pipeline_job.service_account
pipeline_job_template_metadata = pipeline_job.template_metadata
pipeline_job_template_uri = pipeline_job.template_uri
pipeline_job_create_time = pipeline_job.create_time
pipeline_job_end_time = pipeline_job.end_time
pipeline_job_job_detail = pipeline_job.job_detail
pipeline_job_update_time = pipeline_job.update_time
pipeline_job_state = pipeline_job.state
pipeline_job_runtime_config = pipeline_job.runtime_config
pipeline_job_start_time = pipeline_job.start_time
pipeline_job_pipeline_spec = pipeline_job.pipeline_spec
pipeline_job_encryption_spec = pipeline_job.encryption_spec
pipeline_job_error = pipeline_job.error
pipeline_job_labels = pipeline_job.labels
pipeline_job_psc_interface_config = pipeline_job.psc_interface_config
pipeline_job_network = pipeline_job.network
pipeline_job_name = pipeline_job.name
pipeline_job_reserved_ip_ranges = pipeline_job.reserved_ip_ranges
pipeline_job_schedule_name = pipeline_job.schedule_name
```

---


### Nas_job

Creates a NasJob

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize NasJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `enable_restricted_image_training` | bool |  | Optional. Enable a separation of Custom model training and restricted image training for tenant project. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `end_time` | String |  | Output only. Time when the NasJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `display_name` | String |  | Required. The display name of the NasJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `create_time` | String |  | Output only. Time when the NasJob was created. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `nas_job_output` | String |  | Output only. Output of the NasJob. |
| `encryption_spec` | String |  | Customer-managed encryption key options for a NasJob. If this is set, then all resources created by the NasJob will be encrypted with the provided encryption key. |
| `error` | String |  | Output only. Only populated when job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `name` | String |  | Output only. Resource name of the NasJob. |
| `update_time` | String |  | Output only. Time when the NasJob was most recently updated. |
| `nas_job_spec` | String |  | Required. The specification of a NasJob. |
| `start_time` | String |  | Output only. Time when the NasJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `state` | String |  | Output only. The detailed state of the job. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the NasJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize NasJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `enable_restricted_image_training` | bool | Optional. Enable a separation of Custom model training and restricted image training for tenant project. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `end_time` | String | Output only. Time when the NasJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `display_name` | String | Required. The display name of the NasJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `create_time` | String | Output only. Time when the NasJob was created. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `nas_job_output` | String | Output only. Output of the NasJob. |
| `encryption_spec` | String | Customer-managed encryption key options for a NasJob. If this is set, then all resources created by the NasJob will be encrypted with the provided encryption key. |
| `error` | String | Output only. Only populated when job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `name` | String | Output only. Resource name of the NasJob. |
| `update_time` | String | Output only. Time when the NasJob was most recently updated. |
| `nas_job_spec` | String | Required. The specification of a NasJob. |
| `start_time` | String | Output only. Time when the NasJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `state` | String | Output only. The detailed state of the job. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create nas_job
nas_job = provider.aiplatform_api.Nas_job {
    parent = "value"  # Required. The resource name of the Location to create the NasJob in. Format: `projects/{project}/locations/{location}`
}

# Access nas_job outputs
nas_job_id = nas_job.id
nas_job_labels = nas_job.labels
nas_job_enable_restricted_image_training = nas_job.enable_restricted_image_training
nas_job_satisfies_pzs = nas_job.satisfies_pzs
nas_job_end_time = nas_job.end_time
nas_job_display_name = nas_job.display_name
nas_job_create_time = nas_job.create_time
nas_job_satisfies_pzi = nas_job.satisfies_pzi
nas_job_nas_job_output = nas_job.nas_job_output
nas_job_encryption_spec = nas_job.encryption_spec
nas_job_error = nas_job.error
nas_job_name = nas_job.name
nas_job_update_time = nas_job.update_time
nas_job_nas_job_spec = nas_job.nas_job_spec
nas_job_start_time = nas_job.start_time
nas_job_state = nas_job.state
```

---


### Endpoint

Creates an Endpoint.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dedicated_endpoint_dns` | String |  | Output only. DNS of the dedicated endpoint. Will only be populated if dedicated_endpoint_enabled is true. Depending on the features enabled, uid might be a random number or a string. For example, if fast_tryout is enabled, uid will be fasttryout. Format: `https://{endpoint_id}.{region}-{uid}.prediction.vertexai.goog`. |
| `dedicated_endpoint_enabled` | bool |  | If true, the endpoint will be exposed through a dedicated DNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS will be isolated from other users' traffic and will have better performance and reliability. Note: Once you enabled dedicated endpoint, you won't be able to send request to the shared DNS {region}-aiplatform.googleapis.com. The limitation will be removed soon. |
| `name` | String |  | Output only. The resource name of the Endpoint. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this Endpoint was last updated. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `traffic_split` | HashMap<String, i64> |  | A map from a DeployedModel's ID to the percentage of this Endpoint's traffic that should be forwarded to that DeployedModel. If a DeployedModel's ID is not listed in this map, then it receives no traffic. The traffic percentage values must add up to 100, or map must be empty if the Endpoint is to not accept any traffic at a moment. |
| `predict_request_response_logging_config` | String |  | Configures the request-response logging for online prediction. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `deployed_models` | Vec<String> |  | Output only. The models deployed in this Endpoint. To add or remove DeployedModels use EndpointService.DeployModel and EndpointService.UndeployModel respectively. |
| `client_connection_config` | String |  | Configurations that are applied to the endpoint for online prediction. |
| `gen_ai_advanced_features_config` | String |  | Optional. Configuration for GenAiAdvancedFeatures. If the endpoint is serving GenAI models, advanced features like native RAG integration can be configured. Currently, only Model Garden models are supported. |
| `private_service_connect_config` | String |  | Optional. Configuration for private service connect. network and private_service_connect_config are mutually exclusive. |
| `create_time` | String |  | Output only. Timestamp when this Endpoint was created. |
| `network` | String |  | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where `{project}` is a project number, as in `12345`, and `{network}` is network name. |
| `enable_private_service_connect` | bool |  | Deprecated: If true, expose the Endpoint via private service connect. Only one of the fields, network or enable_private_service_connect, can be set. |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `gdc_config` | String |  | Configures the Google Distributed Cloud (GDC) environment for online prediction. Only set this field when the Endpoint is to be deployed in a GDC environment. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for an Endpoint. If set, this Endpoint and all sub-resources of this Endpoint will be secured by this key. |
| `model_deployment_monitoring_job` | String |  | Output only. Resource name of the Model Monitoring job associated with this Endpoint if monitoring is enabled by JobService.CreateModelDeploymentMonitoringJob. Format: `projects/{project}/locations/{location}/modelDeploymentMonitoringJobs/{model_deployment_monitoring_job}` |
| `display_name` | String |  | Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `description` | String |  | The description of the Endpoint. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Endpoint in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dedicated_endpoint_dns` | String | Output only. DNS of the dedicated endpoint. Will only be populated if dedicated_endpoint_enabled is true. Depending on the features enabled, uid might be a random number or a string. For example, if fast_tryout is enabled, uid will be fasttryout. Format: `https://{endpoint_id}.{region}-{uid}.prediction.vertexai.goog`. |
| `dedicated_endpoint_enabled` | bool | If true, the endpoint will be exposed through a dedicated DNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS will be isolated from other users' traffic and will have better performance and reliability. Note: Once you enabled dedicated endpoint, you won't be able to send request to the shared DNS {region}-aiplatform.googleapis.com. The limitation will be removed soon. |
| `name` | String | Output only. The resource name of the Endpoint. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this Endpoint was last updated. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `traffic_split` | HashMap<String, i64> | A map from a DeployedModel's ID to the percentage of this Endpoint's traffic that should be forwarded to that DeployedModel. If a DeployedModel's ID is not listed in this map, then it receives no traffic. The traffic percentage values must add up to 100, or map must be empty if the Endpoint is to not accept any traffic at a moment. |
| `predict_request_response_logging_config` | String | Configures the request-response logging for online prediction. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `deployed_models` | Vec<String> | Output only. The models deployed in this Endpoint. To add or remove DeployedModels use EndpointService.DeployModel and EndpointService.UndeployModel respectively. |
| `client_connection_config` | String | Configurations that are applied to the endpoint for online prediction. |
| `gen_ai_advanced_features_config` | String | Optional. Configuration for GenAiAdvancedFeatures. If the endpoint is serving GenAI models, advanced features like native RAG integration can be configured. Currently, only Model Garden models are supported. |
| `private_service_connect_config` | String | Optional. Configuration for private service connect. network and private_service_connect_config are mutually exclusive. |
| `create_time` | String | Output only. Timestamp when this Endpoint was created. |
| `network` | String | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where `{project}` is a project number, as in `12345`, and `{network}` is network name. |
| `enable_private_service_connect` | bool | Deprecated: If true, expose the Endpoint via private service connect. Only one of the fields, network or enable_private_service_connect, can be set. |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `gdc_config` | String | Configures the Google Distributed Cloud (GDC) environment for online prediction. Only set this field when the Endpoint is to be deployed in a GDC environment. |
| `encryption_spec` | String | Customer-managed encryption key spec for an Endpoint. If set, this Endpoint and all sub-resources of this Endpoint will be secured by this key. |
| `model_deployment_monitoring_job` | String | Output only. Resource name of the Model Monitoring job associated with this Endpoint if monitoring is enabled by JobService.CreateModelDeploymentMonitoringJob. Format: `projects/{project}/locations/{location}/modelDeploymentMonitoringJobs/{model_deployment_monitoring_job}` |
| `display_name` | String | Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `description` | String | The description of the Endpoint. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create endpoint
endpoint = provider.aiplatform_api.Endpoint {
    parent = "value"  # Required. The resource name of the Location to create the Endpoint in. Format: `projects/{project}/locations/{location}`
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_dedicated_endpoint_dns = endpoint.dedicated_endpoint_dns
endpoint_dedicated_endpoint_enabled = endpoint.dedicated_endpoint_enabled
endpoint_name = endpoint.name
endpoint_satisfies_pzi = endpoint.satisfies_pzi
endpoint_update_time = endpoint.update_time
endpoint_satisfies_pzs = endpoint.satisfies_pzs
endpoint_traffic_split = endpoint.traffic_split
endpoint_predict_request_response_logging_config = endpoint.predict_request_response_logging_config
endpoint_labels = endpoint.labels
endpoint_deployed_models = endpoint.deployed_models
endpoint_client_connection_config = endpoint.client_connection_config
endpoint_gen_ai_advanced_features_config = endpoint.gen_ai_advanced_features_config
endpoint_private_service_connect_config = endpoint.private_service_connect_config
endpoint_create_time = endpoint.create_time
endpoint_network = endpoint.network
endpoint_enable_private_service_connect = endpoint.enable_private_service_connect
endpoint_etag = endpoint.etag
endpoint_gdc_config = endpoint.gdc_config
endpoint_encryption_spec = endpoint.encryption_spec
endpoint_model_deployment_monitoring_job = endpoint.model_deployment_monitoring_job
endpoint_display_name = endpoint.display_name
endpoint_description = endpoint.description
```

---


### Saved_querie

Lists SavedQueries in a Dataset.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `saved_queries` | Vec<String> | A list of SavedQueries that match the specified filter in the request. |
| `next_page_token` | String | The standard List next-page token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access saved_querie outputs
saved_querie_id = saved_querie.id
saved_querie_saved_queries = saved_querie.saved_queries
saved_querie_next_page_token = saved_querie.next_page_token
```

---


### Indexe

Creates an Index.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deployed_indexes` | Vec<String> |  | Output only. The pointers to DeployedIndexes created from this Index. An Index can be only deleted if all its DeployedIndexes had been undeployed first. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `display_name` | String |  | Required. The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `metadata` | String |  | An additional information about the Index; the schema of the metadata can be found in metadata_schema. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this Index was most recently updated. This also includes any update to the contents of the Index. Note that Operations working on this Index may have their Operations.metadata.generic_metadata.update_time a little after the value of this timestamp, yet that does not mean their results are not already reflected in the Index. Result of any successfully completed Operation on the Index is reflected in it. |
| `create_time` | String |  | Output only. Timestamp when this Index was created. |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String |  | Output only. The resource name of the Index. |
| `metadata_schema_uri` | String |  | Immutable. Points to a YAML file stored on Google Cloud Storage describing additional information about the Index, that is specific to it. Unset if the Index does not have any additional information. The schema is defined as an OpenAPI 3.0.2 [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.2.md#schemaObject). Note: The URI given on output will be immutable and probably different, including the URI scheme, than the one given on input. The output URI will point to a location where the user only has a read access. |
| `index_stats` | String |  | Output only. Stats of the index resource. |
| `description` | String |  | The description of the Index. |
| `encryption_spec` | String |  | Immutable. Customer-managed encryption key spec for an Index. If set, this Index and all sub-resources of this Index will be secured by this key. |
| `index_update_method` | String |  | Immutable. The update method to use with this Index. If not set, BATCH_UPDATE will be used by default. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Indexes. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Index in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployed_indexes` | Vec<String> | Output only. The pointers to DeployedIndexes created from this Index. An Index can be only deleted if all its DeployedIndexes had been undeployed first. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `display_name` | String | Required. The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `metadata` | String | An additional information about the Index; the schema of the metadata can be found in metadata_schema. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this Index was most recently updated. This also includes any update to the contents of the Index. Note that Operations working on this Index may have their Operations.metadata.generic_metadata.update_time a little after the value of this timestamp, yet that does not mean their results are not already reflected in the Index. Result of any successfully completed Operation on the Index is reflected in it. |
| `create_time` | String | Output only. Timestamp when this Index was created. |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String | Output only. The resource name of the Index. |
| `metadata_schema_uri` | String | Immutable. Points to a YAML file stored on Google Cloud Storage describing additional information about the Index, that is specific to it. Unset if the Index does not have any additional information. The schema is defined as an OpenAPI 3.0.2 [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.2.md#schemaObject). Note: The URI given on output will be immutable and probably different, including the URI scheme, than the one given on input. The output URI will point to a location where the user only has a read access. |
| `index_stats` | String | Output only. Stats of the index resource. |
| `description` | String | The description of the Index. |
| `encryption_spec` | String | Immutable. Customer-managed encryption key spec for an Index. If set, this Index and all sub-resources of this Index will be secured by this key. |
| `index_update_method` | String | Immutable. The update method to use with this Index. If not set, BATCH_UPDATE will be used by default. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Indexes. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create indexe
indexe = provider.aiplatform_api.Indexe {
    parent = "value"  # Required. The resource name of the Location to create the Index in. Format: `projects/{project}/locations/{location}`
}

# Access indexe outputs
indexe_id = indexe.id
indexe_deployed_indexes = indexe.deployed_indexes
indexe_satisfies_pzs = indexe.satisfies_pzs
indexe_display_name = indexe.display_name
indexe_metadata = indexe.metadata
indexe_satisfies_pzi = indexe.satisfies_pzi
indexe_update_time = indexe.update_time
indexe_create_time = indexe.create_time
indexe_etag = indexe.etag
indexe_name = indexe.name
indexe_metadata_schema_uri = indexe.metadata_schema_uri
indexe_index_stats = indexe.index_stats
indexe_description = indexe.description
indexe_encryption_spec = indexe.encryption_spec
indexe_index_update_method = indexe.index_update_method
indexe_labels = indexe.labels
```

---


### Execution

Creates an Execution associated with a MetadataStore.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_version` | String |  | The version of the schema in `schema_title` to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `state` | String |  | The state of this Execution. This is a property of the Execution, and does not imply or capture any ongoing process. This property is managed by clients (such as Vertex AI Pipelines) and the system does not prescribe or check the validity of state transitions. |
| `description` | String |  | Description of the Execution |
| `etag` | String |  | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `schema_title` | String |  | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `create_time` | String |  | Output only. Timestamp when this Execution was created. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Executions. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Execution (System labels are excluded). |
| `display_name` | String |  | User provided display name of the Execution. May be up to 128 Unicode characters. |
| `name` | String |  | Output only. The resource name of the Execution. |
| `metadata` | HashMap<String, String> |  | Properties of the Execution. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |
| `update_time` | String |  | Output only. Timestamp when this Execution was last updated. |
| `parent` | String | ✅ | Required. The resource name of the MetadataStore where the Execution should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema_version` | String | The version of the schema in `schema_title` to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `state` | String | The state of this Execution. This is a property of the Execution, and does not imply or capture any ongoing process. This property is managed by clients (such as Vertex AI Pipelines) and the system does not prescribe or check the validity of state transitions. |
| `description` | String | Description of the Execution |
| `etag` | String | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `schema_title` | String | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `create_time` | String | Output only. Timestamp when this Execution was created. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Executions. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Execution (System labels are excluded). |
| `display_name` | String | User provided display name of the Execution. May be up to 128 Unicode characters. |
| `name` | String | Output only. The resource name of the Execution. |
| `metadata` | HashMap<String, String> | Properties of the Execution. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |
| `update_time` | String | Output only. Timestamp when this Execution was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create execution
execution = provider.aiplatform_api.Execution {
    parent = "value"  # Required. The resource name of the MetadataStore where the Execution should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}`
}

# Access execution outputs
execution_id = execution.id
execution_schema_version = execution.schema_version
execution_state = execution.state
execution_description = execution.description
execution_etag = execution.etag
execution_schema_title = execution.schema_title
execution_create_time = execution.create_time
execution_labels = execution.labels
execution_display_name = execution.display_name
execution_name = execution.name
execution_metadata = execution.metadata
execution_update_time = execution.update_time
```

---


### Schedule

Creates a Schedule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `max_concurrent_run_count` | String |  | Required. Maximum number of runs that can be started concurrently for this Schedule. This is the limit for starting the scheduled requests and not the execution of the operations/jobs created by the requests (if applicable). |
| `next_run_time` | String |  | Output only. Timestamp when this Schedule should schedule the next run. Having a next_run_time in the past means the runs are being started behind schedule. |
| `last_pause_time` | String |  | Output only. Timestamp when this Schedule was last paused. Unset if never paused. |
| `max_run_count` | String |  | Optional. Maximum run count of the schedule. If specified, The schedule will be completed when either started_run_count >= max_run_count or when end_time is reached. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified. |
| `allow_queueing` | bool |  | Optional. Whether new scheduled runs can be queued when max_concurrent_runs limit is reached. If set to true, new runs will be queued instead of skipped. Default to false. |
| `last_scheduled_run_response` | String |  | Output only. Response of the last scheduled run. This is the response for starting the scheduled requests and not the execution of the operations/jobs created by the requests (if applicable). Unset if no run has been scheduled yet. |
| `create_pipeline_job_request` | String |  | Request for PipelineService.CreatePipelineJob. CreatePipelineJobRequest.parent field is required (format: projects/{project}/locations/{location}). |
| `start_time` | String |  | Optional. Timestamp after which the first run can be scheduled. Default to Schedule create time if not specified. |
| `state` | String |  | Output only. The state of this Schedule. |
| `create_time` | String |  | Output only. Timestamp when this Schedule was created. |
| `create_notebook_execution_job_request` | String |  | Request for NotebookService.CreateNotebookExecutionJob. |
| `cron` | String |  | Cron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or "TZ=${IANA_TIME_ZONE}". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, "CRON_TZ=America/New_York 1 * * * *", or "TZ=America/New_York 1 * * * *". |
| `started_run_count` | String |  | Output only. The number of runs started by this schedule. |
| `update_time` | String |  | Output only. Timestamp when this Schedule was updated. |
| `display_name` | String |  | Required. User provided name of the Schedule. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `catch_up` | bool |  | Output only. Whether to backfill missed runs when the schedule is resumed from PAUSED state. If set to true, all missed runs will be scheduled. New runs will be scheduled after the backfill is complete. Default to false. |
| `end_time` | String |  | Optional. Timestamp after which no new runs can be scheduled. If specified, The schedule will be completed when either end_time is reached or when scheduled_run_count >= max_run_count. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified. |
| `name` | String |  | Immutable. The resource name of the Schedule. |
| `last_resume_time` | String |  | Output only. Timestamp when this Schedule was last resumed. Unset if never resumed from pause. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Schedule in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `max_concurrent_run_count` | String | Required. Maximum number of runs that can be started concurrently for this Schedule. This is the limit for starting the scheduled requests and not the execution of the operations/jobs created by the requests (if applicable). |
| `next_run_time` | String | Output only. Timestamp when this Schedule should schedule the next run. Having a next_run_time in the past means the runs are being started behind schedule. |
| `last_pause_time` | String | Output only. Timestamp when this Schedule was last paused. Unset if never paused. |
| `max_run_count` | String | Optional. Maximum run count of the schedule. If specified, The schedule will be completed when either started_run_count >= max_run_count or when end_time is reached. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified. |
| `allow_queueing` | bool | Optional. Whether new scheduled runs can be queued when max_concurrent_runs limit is reached. If set to true, new runs will be queued instead of skipped. Default to false. |
| `last_scheduled_run_response` | String | Output only. Response of the last scheduled run. This is the response for starting the scheduled requests and not the execution of the operations/jobs created by the requests (if applicable). Unset if no run has been scheduled yet. |
| `create_pipeline_job_request` | String | Request for PipelineService.CreatePipelineJob. CreatePipelineJobRequest.parent field is required (format: projects/{project}/locations/{location}). |
| `start_time` | String | Optional. Timestamp after which the first run can be scheduled. Default to Schedule create time if not specified. |
| `state` | String | Output only. The state of this Schedule. |
| `create_time` | String | Output only. Timestamp when this Schedule was created. |
| `create_notebook_execution_job_request` | String | Request for NotebookService.CreateNotebookExecutionJob. |
| `cron` | String | Cron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or "TZ=${IANA_TIME_ZONE}". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, "CRON_TZ=America/New_York 1 * * * *", or "TZ=America/New_York 1 * * * *". |
| `started_run_count` | String | Output only. The number of runs started by this schedule. |
| `update_time` | String | Output only. Timestamp when this Schedule was updated. |
| `display_name` | String | Required. User provided name of the Schedule. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `catch_up` | bool | Output only. Whether to backfill missed runs when the schedule is resumed from PAUSED state. If set to true, all missed runs will be scheduled. New runs will be scheduled after the backfill is complete. Default to false. |
| `end_time` | String | Optional. Timestamp after which no new runs can be scheduled. If specified, The schedule will be completed when either end_time is reached or when scheduled_run_count >= max_run_count. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified. |
| `name` | String | Immutable. The resource name of the Schedule. |
| `last_resume_time` | String | Output only. Timestamp when this Schedule was last resumed. Unset if never resumed from pause. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schedule
schedule = provider.aiplatform_api.Schedule {
    parent = "value"  # Required. The resource name of the Location to create the Schedule in. Format: `projects/{project}/locations/{location}`
}

# Access schedule outputs
schedule_id = schedule.id
schedule_max_concurrent_run_count = schedule.max_concurrent_run_count
schedule_next_run_time = schedule.next_run_time
schedule_last_pause_time = schedule.last_pause_time
schedule_max_run_count = schedule.max_run_count
schedule_allow_queueing = schedule.allow_queueing
schedule_last_scheduled_run_response = schedule.last_scheduled_run_response
schedule_create_pipeline_job_request = schedule.create_pipeline_job_request
schedule_start_time = schedule.start_time
schedule_state = schedule.state
schedule_create_time = schedule.create_time
schedule_create_notebook_execution_job_request = schedule.create_notebook_execution_job_request
schedule_cron = schedule.cron
schedule_started_run_count = schedule.started_run_count
schedule_update_time = schedule.update_time
schedule_display_name = schedule.display_name
schedule_catch_up = schedule.catch_up
schedule_end_time = schedule.end_time
schedule_name = schedule.name
schedule_last_resume_time = schedule.last_resume_time
```

---


### Experiment

Creates a TensorboardExperiment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Name of the TensorboardExperiment. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}` |
| `display_name` | String |  | User provided name of this TensorboardExperiment. |
| `create_time` | String |  | Output only. Timestamp when this TensorboardExperiment was created. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your TensorboardExperiment. Label keys and values cannot be longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Dataset (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `aiplatform.googleapis.com/` and are immutable. The following system labels exist for each Dataset: * `aiplatform.googleapis.com/dataset_metadata_schema`: output only. Its value is the metadata_schema's title. |
| `source` | String |  | Immutable. Source of the TensorboardExperiment. Example: a custom training job. |
| `description` | String |  | Description of this TensorboardExperiment. |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `update_time` | String |  | Output only. Timestamp when this TensorboardExperiment was last updated. |
| `parent` | String | ✅ | Required. The resource name of the Tensorboard to create the TensorboardExperiment in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Name of the TensorboardExperiment. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}` |
| `display_name` | String | User provided name of this TensorboardExperiment. |
| `create_time` | String | Output only. Timestamp when this TensorboardExperiment was created. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your TensorboardExperiment. Label keys and values cannot be longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Dataset (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `aiplatform.googleapis.com/` and are immutable. The following system labels exist for each Dataset: * `aiplatform.googleapis.com/dataset_metadata_schema`: output only. Its value is the metadata_schema's title. |
| `source` | String | Immutable. Source of the TensorboardExperiment. Example: a custom training job. |
| `description` | String | Description of this TensorboardExperiment. |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `update_time` | String | Output only. Timestamp when this TensorboardExperiment was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create experiment
experiment = provider.aiplatform_api.Experiment {
    parent = "value"  # Required. The resource name of the Tensorboard to create the TensorboardExperiment in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}`
}

# Access experiment outputs
experiment_id = experiment.id
experiment_name = experiment.name
experiment_display_name = experiment.display_name
experiment_create_time = experiment.create_time
experiment_labels = experiment.labels
experiment_source = experiment.source
experiment_description = experiment.description
experiment_etag = experiment.etag
experiment_update_time = experiment.update_time
```

---


### Dataset_version

Create a version from a Dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `big_query_dataset_name` | String |  | Output only. Name of the associated BigQuery dataset. |
| `model_reference` | String |  | Output only. Reference to the public base model last used by the dataset version. Only set for prompt dataset versions. |
| `name` | String |  | Output only. Identifier. The resource name of the DatasetVersion. Format: `projects/{project}/locations/{location}/datasets/{dataset}/datasetVersions/{dataset_version}` |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `metadata` | String |  | Required. Output only. Additional information about the DatasetVersion. |
| `display_name` | String |  | The user-defined name of the DatasetVersion. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `create_time` | String |  | Output only. Timestamp when this DatasetVersion was created. |
| `update_time` | String |  | Output only. Timestamp when this DatasetVersion was last updated. |
| `parent` | String | ✅ | Required. The name of the Dataset resource. Format: `projects/{project}/locations/{location}/datasets/{dataset}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `big_query_dataset_name` | String | Output only. Name of the associated BigQuery dataset. |
| `model_reference` | String | Output only. Reference to the public base model last used by the dataset version. Only set for prompt dataset versions. |
| `name` | String | Output only. Identifier. The resource name of the DatasetVersion. Format: `projects/{project}/locations/{location}/datasets/{dataset}/datasetVersions/{dataset_version}` |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `metadata` | String | Required. Output only. Additional information about the DatasetVersion. |
| `display_name` | String | The user-defined name of the DatasetVersion. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `create_time` | String | Output only. Timestamp when this DatasetVersion was created. |
| `update_time` | String | Output only. Timestamp when this DatasetVersion was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset_version
dataset_version = provider.aiplatform_api.Dataset_version {
    parent = "value"  # Required. The name of the Dataset resource. Format: `projects/{project}/locations/{location}/datasets/{dataset}`
}

# Access dataset_version outputs
dataset_version_id = dataset_version.id
dataset_version_etag = dataset_version.etag
dataset_version_big_query_dataset_name = dataset_version.big_query_dataset_name
dataset_version_model_reference = dataset_version.model_reference
dataset_version_name = dataset_version.name
dataset_version_satisfies_pzi = dataset_version.satisfies_pzi
dataset_version_satisfies_pzs = dataset_version.satisfies_pzs
dataset_version_metadata = dataset_version.metadata
dataset_version_display_name = dataset_version.display_name
dataset_version_create_time = dataset_version.create_time
dataset_version_update_time = dataset_version.update_time
```

---


### Run

Creates a TensorboardRun.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Description of this TensorboardRun. |
| `create_time` | String |  | Output only. Timestamp when this TensorboardRun was created. |
| `display_name` | String |  | Required. User provided name of this TensorboardRun. This value must be unique among all TensorboardRuns belonging to the same parent TensorboardExperiment. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your TensorboardRuns. This field will be used to filter and visualize Runs in the Tensorboard UI. For example, a Vertex AI training job can set a label aiplatform.googleapis.com/training_job_id=xxxxx to all the runs created within that job. An end user can set a label experiment_id=xxxxx for all the runs produced in a Jupyter notebook. These runs can be grouped by a label value and visualized together in the Tensorboard UI. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one TensorboardRun (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `name` | String |  | Output only. Name of the TensorboardRun. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}/runs/{run}` |
| `etag` | String |  | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `update_time` | String |  | Output only. Timestamp when this TensorboardRun was last updated. |
| `parent` | String | ✅ | Required. The resource name of the TensorboardExperiment to create the TensorboardRun in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Description of this TensorboardRun. |
| `create_time` | String | Output only. Timestamp when this TensorboardRun was created. |
| `display_name` | String | Required. User provided name of this TensorboardRun. This value must be unique among all TensorboardRuns belonging to the same parent TensorboardExperiment. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your TensorboardRuns. This field will be used to filter and visualize Runs in the Tensorboard UI. For example, a Vertex AI training job can set a label aiplatform.googleapis.com/training_job_id=xxxxx to all the runs created within that job. An end user can set a label experiment_id=xxxxx for all the runs produced in a Jupyter notebook. These runs can be grouped by a label value and visualized together in the Tensorboard UI. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one TensorboardRun (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `name` | String | Output only. Name of the TensorboardRun. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}/runs/{run}` |
| `etag` | String | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `update_time` | String | Output only. Timestamp when this TensorboardRun was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create run
run = provider.aiplatform_api.Run {
    parent = "value"  # Required. The resource name of the TensorboardExperiment to create the TensorboardRun in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}`
}

# Access run outputs
run_id = run.id
run_description = run.description
run_create_time = run.create_time
run_display_name = run.display_name
run_labels = run.labels
run_name = run.name
run_etag = run.etag
run_update_time = run.update_time
```

---


### Featurestore

Creates a new Featurestore in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this Featurestore was last updated. |
| `state` | String |  | Output only. State of the featurestore. |
| `create_time` | String |  | Output only. Timestamp when this Featurestore was created. |
| `online_storage_ttl_days` | i64 |  | Optional. TTL in days for feature values that will be stored in online serving storage. The Feature Store online storage periodically removes obsolete feature values older than `online_storage_ttl_days` since the feature generation time. Note that `online_storage_ttl_days` should be less than or equal to `offline_storage_ttl_days` for each EntityType under a featurestore. If not set, default to 4000 days |
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String |  | Output only. Name of the Featurestore. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}` |
| `encryption_spec` | String |  | Optional. Customer-managed encryption key spec for data storage. If set, both of the online and offline data storage will be secured by this key. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your Featurestore. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one Featurestore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `online_serving_config` | String |  | Optional. Config for online storage resources. The field should not co-exist with the field of `OnlineStoreReplicationConfig`. If both of it and OnlineStoreReplicationConfig are unset, the feature store will not have an online store and cannot be used for online serving. |
| `parent` | String | ✅ | Required. The resource name of the Location to create Featurestores. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this Featurestore was last updated. |
| `state` | String | Output only. State of the featurestore. |
| `create_time` | String | Output only. Timestamp when this Featurestore was created. |
| `online_storage_ttl_days` | i64 | Optional. TTL in days for feature values that will be stored in online serving storage. The Feature Store online storage periodically removes obsolete feature values older than `online_storage_ttl_days` since the feature generation time. Note that `online_storage_ttl_days` should be less than or equal to `offline_storage_ttl_days` for each EntityType under a featurestore. If not set, default to 4000 days |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String | Output only. Name of the Featurestore. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}` |
| `encryption_spec` | String | Optional. Customer-managed encryption key spec for data storage. If set, both of the online and offline data storage will be secured by this key. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your Featurestore. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one Featurestore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `online_serving_config` | String | Optional. Config for online storage resources. The field should not co-exist with the field of `OnlineStoreReplicationConfig`. If both of it and OnlineStoreReplicationConfig are unset, the feature store will not have an online store and cannot be used for online serving. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create featurestore
featurestore = provider.aiplatform_api.Featurestore {
    parent = "value"  # Required. The resource name of the Location to create Featurestores. Format: `projects/{project}/locations/{location}`
}

# Access featurestore outputs
featurestore_id = featurestore.id
featurestore_satisfies_pzi = featurestore.satisfies_pzi
featurestore_satisfies_pzs = featurestore.satisfies_pzs
featurestore_update_time = featurestore.update_time
featurestore_state = featurestore.state
featurestore_create_time = featurestore.create_time
featurestore_online_storage_ttl_days = featurestore.online_storage_ttl_days
featurestore_etag = featurestore.etag
featurestore_name = featurestore.name
featurestore_encryption_spec = featurestore.encryption_spec
featurestore_labels = featurestore.labels
featurestore_online_serving_config = featurestore.online_serving_config
```

---


### Cached_content

Creates cached content, this call will initialize the cached content in the data storage, and users need to pay for the cache data storage.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model` | String |  | Immutable. The name of the `Model` to use for cached content. Currently, only the published Gemini base models are supported, in form of projects/{PROJECT}/locations/{LOCATION}/publishers/google/models/{MODEL} |
| `display_name` | String |  | Optional. Immutable. The user-generated meaningful display name of the cached content. |
| `create_time` | String |  | Output only. Creation time of the cache entry. |
| `encryption_spec` | String |  | Input only. Immutable. Customer-managed encryption key spec for a `CachedContent`. If set, this `CachedContent` and all its sub-resources will be secured by this key. |
| `tool_config` | String |  | Optional. Input only. Immutable. Tool config. This config is shared for all tools |
| `tools` | Vec<String> |  | Optional. Input only. Immutable. A list of `Tools` the model may use to generate the next response |
| `name` | String |  | Immutable. Identifier. The server-generated resource name of the cached content Format: projects/{project}/locations/{location}/cachedContents/{cached_content} |
| `system_instruction` | String |  | Optional. Input only. Immutable. Developer set system instruction. Currently, text only |
| `contents` | Vec<String> |  | Optional. Input only. Immutable. The content to cache |
| `ttl` | String |  | Input only. The TTL for this resource. The expiration time is computed: now + TTL. |
| `expire_time` | String |  | Timestamp of when this resource is considered expired. This is *always* provided on output, regardless of what was sent on input. |
| `update_time` | String |  | Output only. When the cache entry was last updated in UTC time. |
| `usage_metadata` | String |  | Output only. Metadata on the usage of the cached content. |
| `parent` | String | ✅ | Required. The parent resource where the cached content will be created |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model` | String | Immutable. The name of the `Model` to use for cached content. Currently, only the published Gemini base models are supported, in form of projects/{PROJECT}/locations/{LOCATION}/publishers/google/models/{MODEL} |
| `display_name` | String | Optional. Immutable. The user-generated meaningful display name of the cached content. |
| `create_time` | String | Output only. Creation time of the cache entry. |
| `encryption_spec` | String | Input only. Immutable. Customer-managed encryption key spec for a `CachedContent`. If set, this `CachedContent` and all its sub-resources will be secured by this key. |
| `tool_config` | String | Optional. Input only. Immutable. Tool config. This config is shared for all tools |
| `tools` | Vec<String> | Optional. Input only. Immutable. A list of `Tools` the model may use to generate the next response |
| `name` | String | Immutable. Identifier. The server-generated resource name of the cached content Format: projects/{project}/locations/{location}/cachedContents/{cached_content} |
| `system_instruction` | String | Optional. Input only. Immutable. Developer set system instruction. Currently, text only |
| `contents` | Vec<String> | Optional. Input only. Immutable. The content to cache |
| `ttl` | String | Input only. The TTL for this resource. The expiration time is computed: now + TTL. |
| `expire_time` | String | Timestamp of when this resource is considered expired. This is *always* provided on output, regardless of what was sent on input. |
| `update_time` | String | Output only. When the cache entry was last updated in UTC time. |
| `usage_metadata` | String | Output only. Metadata on the usage of the cached content. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cached_content
cached_content = provider.aiplatform_api.Cached_content {
    parent = "value"  # Required. The parent resource where the cached content will be created
}

# Access cached_content outputs
cached_content_id = cached_content.id
cached_content_model = cached_content.model
cached_content_display_name = cached_content.display_name
cached_content_create_time = cached_content.create_time
cached_content_encryption_spec = cached_content.encryption_spec
cached_content_tool_config = cached_content.tool_config
cached_content_tools = cached_content.tools
cached_content_name = cached_content.name
cached_content_system_instruction = cached_content.system_instruction
cached_content_contents = cached_content.contents
cached_content_ttl = cached_content.ttl
cached_content_expire_time = cached_content.expire_time
cached_content_update_time = cached_content.update_time
cached_content_usage_metadata = cached_content.usage_metadata
```

---


### Batch_prediction_job

Creates a BatchPredictionJob. A BatchPredictionJob once created will right away be attempted to start.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disable_container_logging` | bool |  | For custom-trained Models and AutoML Tabular Models, the container of the DeployedModel instances will send `stderr` and `stdout` streams to Cloud Logging by default. Please note that the logs incur cost, which are subject to [Cloud Logging pricing](https://cloud.google.com/logging/pricing). User can disable container logging by setting this flag to true. |
| `model_parameters` | String |  | The parameters that govern the predictions. The schema of the parameters may be specified via the Model's PredictSchemata's parameters_schema_uri. |
| `name` | String |  | Output only. Resource name of the BatchPredictionJob. |
| `explanation_spec` | String |  | Explanation configuration for this BatchPredictionJob. Can be specified only if generate_explanation is set to `true`. This value overrides the value of Model.explanation_spec. All fields of explanation_spec are optional in the request. If a field of the explanation_spec object is not populated, the corresponding field of the Model.explanation_spec object is inherited. |
| `encryption_spec` | String |  | Customer-managed encryption key options for a BatchPredictionJob. If this is set, then all resources created by the BatchPredictionJob will be encrypted with the provided encryption key. |
| `output_info` | String |  | Output only. Information further describing the output of this job. |
| `partial_failures` | Vec<String> |  | Output only. Partial failures encountered. For example, single files that can't be read. This field never exceeds 20 entries. Status details fields contain standard Google Cloud error details. |
| `create_time` | String |  | Output only. Time when the BatchPredictionJob was created. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize BatchPredictionJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `output_config` | String |  | Required. The Configuration specifying where output predictions should be written. The schema of any single prediction may be specified as a concatenation of Model's PredictSchemata's instance_schema_uri and prediction_schema_uri. |
| `model_version_id` | String |  | Output only. The version ID of the Model that produces the predictions via this job. |
| `manual_batch_tuning_parameters` | String |  | Immutable. Parameters configuring the batch behavior. Currently only applicable when dedicated_resources are used (in other cases Vertex AI does the tuning itself). |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `start_time` | String |  | Output only. Time when the BatchPredictionJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `update_time` | String |  | Output only. Time when the BatchPredictionJob was most recently updated. |
| `model` | String |  | The name of the Model resource that produces the predictions via this job, must share the same ancestor Location. Starting this job has no impact on any existing deployments of the Model and their resources. Exactly one of model and unmanaged_container_model must be set. The model resource name may contain version id or version alias to specify the version. Example: `projects/{project}/locations/{location}/models/{model}@2` or `projects/{project}/locations/{location}/models/{model}@golden` if no version is specified, the default version will be deployed. The model resource could also be a publisher model. Example: `publishers/{publisher}/models/{model}` or `projects/{project}/locations/{location}/publishers/{publisher}/models/{model}` |
| `generate_explanation` | bool |  | Generate explanation with the batch prediction results. When set to `true`, the batch prediction output changes based on the `predictions_format` field of the BatchPredictionJob.output_config object: * `bigquery`: output includes a column named `explanation`. The value is a struct that conforms to the Explanation object. * `jsonl`: The JSON objects on each line include an additional entry keyed `explanation`. The value of the entry is a JSON object that conforms to the Explanation object. * `csv`: Generating explanations for CSV format is not supported. If this field is set to true, either the Model.explanation_spec or explanation_spec must be populated. |
| `completion_stats` | String |  | Output only. Statistics on completed and failed prediction instances. |
| `display_name` | String |  | Required. The user-defined name of this BatchPredictionJob. |
| `resources_consumed` | String |  | Output only. Information about resources that had been consumed by this job. Provided in real time at best effort basis, as well as a final value once the job completes. Note: This field currently may be not populated for batch predictions that use AutoML Models. |
| `state` | String |  | Output only. The detailed state of the job. |
| `unmanaged_container_model` | String |  | Contains model information necessary to perform batch prediction without requiring uploading to model registry. Exactly one of model and unmanaged_container_model must be set. |
| `instance_config` | String |  | Configuration for how to convert batch prediction input instances to the prediction instances that are sent to the Model. |
| `service_account` | String |  | The service account that the DeployedModel's container runs as. If not specified, a system generated one will be used, which has minimal permissions and the custom container, if used, may not have enough permission to access other Google Cloud resources. Users deploying the Model must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `end_time` | String |  | Output only. Time when the BatchPredictionJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `dedicated_resources` | String |  | The config of resources used by the Model during the batch prediction. If the Model supports DEDICATED_RESOURCES this config may be provided (and the job will use these resources), if the Model doesn't support AUTOMATIC_RESOURCES, this config must be provided. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `error` | String |  | Output only. Only populated when the job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `input_config` | String |  | Required. Input configuration of the instances on which predictions are performed. The schema of any single instance may be specified via the Model's PredictSchemata's instance_schema_uri. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disable_container_logging` | bool | For custom-trained Models and AutoML Tabular Models, the container of the DeployedModel instances will send `stderr` and `stdout` streams to Cloud Logging by default. Please note that the logs incur cost, which are subject to [Cloud Logging pricing](https://cloud.google.com/logging/pricing). User can disable container logging by setting this flag to true. |
| `model_parameters` | String | The parameters that govern the predictions. The schema of the parameters may be specified via the Model's PredictSchemata's parameters_schema_uri. |
| `name` | String | Output only. Resource name of the BatchPredictionJob. |
| `explanation_spec` | String | Explanation configuration for this BatchPredictionJob. Can be specified only if generate_explanation is set to `true`. This value overrides the value of Model.explanation_spec. All fields of explanation_spec are optional in the request. If a field of the explanation_spec object is not populated, the corresponding field of the Model.explanation_spec object is inherited. |
| `encryption_spec` | String | Customer-managed encryption key options for a BatchPredictionJob. If this is set, then all resources created by the BatchPredictionJob will be encrypted with the provided encryption key. |
| `output_info` | String | Output only. Information further describing the output of this job. |
| `partial_failures` | Vec<String> | Output only. Partial failures encountered. For example, single files that can't be read. This field never exceeds 20 entries. Status details fields contain standard Google Cloud error details. |
| `create_time` | String | Output only. Time when the BatchPredictionJob was created. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize BatchPredictionJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `output_config` | String | Required. The Configuration specifying where output predictions should be written. The schema of any single prediction may be specified as a concatenation of Model's PredictSchemata's instance_schema_uri and prediction_schema_uri. |
| `model_version_id` | String | Output only. The version ID of the Model that produces the predictions via this job. |
| `manual_batch_tuning_parameters` | String | Immutable. Parameters configuring the batch behavior. Currently only applicable when dedicated_resources are used (in other cases Vertex AI does the tuning itself). |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `start_time` | String | Output only. Time when the BatchPredictionJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `update_time` | String | Output only. Time when the BatchPredictionJob was most recently updated. |
| `model` | String | The name of the Model resource that produces the predictions via this job, must share the same ancestor Location. Starting this job has no impact on any existing deployments of the Model and their resources. Exactly one of model and unmanaged_container_model must be set. The model resource name may contain version id or version alias to specify the version. Example: `projects/{project}/locations/{location}/models/{model}@2` or `projects/{project}/locations/{location}/models/{model}@golden` if no version is specified, the default version will be deployed. The model resource could also be a publisher model. Example: `publishers/{publisher}/models/{model}` or `projects/{project}/locations/{location}/publishers/{publisher}/models/{model}` |
| `generate_explanation` | bool | Generate explanation with the batch prediction results. When set to `true`, the batch prediction output changes based on the `predictions_format` field of the BatchPredictionJob.output_config object: * `bigquery`: output includes a column named `explanation`. The value is a struct that conforms to the Explanation object. * `jsonl`: The JSON objects on each line include an additional entry keyed `explanation`. The value of the entry is a JSON object that conforms to the Explanation object. * `csv`: Generating explanations for CSV format is not supported. If this field is set to true, either the Model.explanation_spec or explanation_spec must be populated. |
| `completion_stats` | String | Output only. Statistics on completed and failed prediction instances. |
| `display_name` | String | Required. The user-defined name of this BatchPredictionJob. |
| `resources_consumed` | String | Output only. Information about resources that had been consumed by this job. Provided in real time at best effort basis, as well as a final value once the job completes. Note: This field currently may be not populated for batch predictions that use AutoML Models. |
| `state` | String | Output only. The detailed state of the job. |
| `unmanaged_container_model` | String | Contains model information necessary to perform batch prediction without requiring uploading to model registry. Exactly one of model and unmanaged_container_model must be set. |
| `instance_config` | String | Configuration for how to convert batch prediction input instances to the prediction instances that are sent to the Model. |
| `service_account` | String | The service account that the DeployedModel's container runs as. If not specified, a system generated one will be used, which has minimal permissions and the custom container, if used, may not have enough permission to access other Google Cloud resources. Users deploying the Model must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `end_time` | String | Output only. Time when the BatchPredictionJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `dedicated_resources` | String | The config of resources used by the Model during the batch prediction. If the Model supports DEDICATED_RESOURCES this config may be provided (and the job will use these resources), if the Model doesn't support AUTOMATIC_RESOURCES, this config must be provided. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `error` | String | Output only. Only populated when the job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `input_config` | String | Required. Input configuration of the instances on which predictions are performed. The schema of any single instance may be specified via the Model's PredictSchemata's instance_schema_uri. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create batch_prediction_job
batch_prediction_job = provider.aiplatform_api.Batch_prediction_job {
}

# Access batch_prediction_job outputs
batch_prediction_job_id = batch_prediction_job.id
batch_prediction_job_disable_container_logging = batch_prediction_job.disable_container_logging
batch_prediction_job_model_parameters = batch_prediction_job.model_parameters
batch_prediction_job_name = batch_prediction_job.name
batch_prediction_job_explanation_spec = batch_prediction_job.explanation_spec
batch_prediction_job_encryption_spec = batch_prediction_job.encryption_spec
batch_prediction_job_output_info = batch_prediction_job.output_info
batch_prediction_job_partial_failures = batch_prediction_job.partial_failures
batch_prediction_job_create_time = batch_prediction_job.create_time
batch_prediction_job_labels = batch_prediction_job.labels
batch_prediction_job_output_config = batch_prediction_job.output_config
batch_prediction_job_model_version_id = batch_prediction_job.model_version_id
batch_prediction_job_manual_batch_tuning_parameters = batch_prediction_job.manual_batch_tuning_parameters
batch_prediction_job_satisfies_pzi = batch_prediction_job.satisfies_pzi
batch_prediction_job_start_time = batch_prediction_job.start_time
batch_prediction_job_update_time = batch_prediction_job.update_time
batch_prediction_job_model = batch_prediction_job.model
batch_prediction_job_generate_explanation = batch_prediction_job.generate_explanation
batch_prediction_job_completion_stats = batch_prediction_job.completion_stats
batch_prediction_job_display_name = batch_prediction_job.display_name
batch_prediction_job_resources_consumed = batch_prediction_job.resources_consumed
batch_prediction_job_state = batch_prediction_job.state
batch_prediction_job_unmanaged_container_model = batch_prediction_job.unmanaged_container_model
batch_prediction_job_instance_config = batch_prediction_job.instance_config
batch_prediction_job_service_account = batch_prediction_job.service_account
batch_prediction_job_end_time = batch_prediction_job.end_time
batch_prediction_job_dedicated_resources = batch_prediction_job.dedicated_resources
batch_prediction_job_satisfies_pzs = batch_prediction_job.satisfies_pzs
batch_prediction_job_error = batch_prediction_job.error
batch_prediction_job_input_config = batch_prediction_job.input_config
```

---


### Feature_online_store

Creates a new FeatureOnlineStore in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your FeatureOnlineStore. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureOnlineStore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this FeatureOnlineStore was last updated. |
| `name` | String |  | Identifier. Name of the FeatureOnlineStore. Format: `projects/{project}/locations/{location}/featureOnlineStores/{featureOnlineStore}` |
| `state` | String |  | Output only. State of the featureOnlineStore. |
| `optimized` | String |  | Contains settings for the Optimized store that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore. When choose Optimized storage type, need to set PrivateServiceConnectConfig.enable_private_service_connect to use private endpoint. Otherwise will use public endpoint by default. |
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `bigtable` | String |  | Contains settings for the Cloud Bigtable instance that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore. |
| `create_time` | String |  | Output only. Timestamp when this FeatureOnlineStore was created. |
| `dedicated_serving_endpoint` | String |  | Optional. The dedicated serving endpoint for this FeatureOnlineStore, which is different from common Vertex service endpoint. |
| `encryption_spec` | String |  | Optional. Customer-managed encryption key spec for data storage. If set, online store will be secured by this key. |
| `parent` | String | ✅ | Required. The resource name of the Location to create FeatureOnlineStores. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your FeatureOnlineStore. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureOnlineStore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this FeatureOnlineStore was last updated. |
| `name` | String | Identifier. Name of the FeatureOnlineStore. Format: `projects/{project}/locations/{location}/featureOnlineStores/{featureOnlineStore}` |
| `state` | String | Output only. State of the featureOnlineStore. |
| `optimized` | String | Contains settings for the Optimized store that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore. When choose Optimized storage type, need to set PrivateServiceConnectConfig.enable_private_service_connect to use private endpoint. Otherwise will use public endpoint by default. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `bigtable` | String | Contains settings for the Cloud Bigtable instance that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore. |
| `create_time` | String | Output only. Timestamp when this FeatureOnlineStore was created. |
| `dedicated_serving_endpoint` | String | Optional. The dedicated serving endpoint for this FeatureOnlineStore, which is different from common Vertex service endpoint. |
| `encryption_spec` | String | Optional. Customer-managed encryption key spec for data storage. If set, online store will be secured by this key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature_online_store
feature_online_store = provider.aiplatform_api.Feature_online_store {
    parent = "value"  # Required. The resource name of the Location to create FeatureOnlineStores. Format: `projects/{project}/locations/{location}`
}

# Access feature_online_store outputs
feature_online_store_id = feature_online_store.id
feature_online_store_labels = feature_online_store.labels
feature_online_store_satisfies_pzi = feature_online_store.satisfies_pzi
feature_online_store_satisfies_pzs = feature_online_store.satisfies_pzs
feature_online_store_update_time = feature_online_store.update_time
feature_online_store_name = feature_online_store.name
feature_online_store_state = feature_online_store.state
feature_online_store_optimized = feature_online_store.optimized
feature_online_store_etag = feature_online_store.etag
feature_online_store_bigtable = feature_online_store.bigtable
feature_online_store_create_time = feature_online_store.create_time
feature_online_store_dedicated_serving_endpoint = feature_online_store.dedicated_serving_endpoint
feature_online_store_encryption_spec = feature_online_store.encryption_spec
```

---


### Reasoning_engine

Creates a reasoning engine.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The display name of the ReasoningEngine. |
| `create_time` | String |  | Output only. Timestamp when this ReasoningEngine was created. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a ReasoningEngine. If set, this ReasoningEngine and all sub-resources of this ReasoningEngine will be secured by this key. |
| `spec` | String |  | Optional. Configurations of the ReasoningEngine |
| `update_time` | String |  | Output only. Timestamp when this ReasoningEngine was most recently updated. |
| `name` | String |  | Identifier. The resource name of the ReasoningEngine. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}` |
| `description` | String |  | Optional. The description of the ReasoningEngine. |
| `labels` | HashMap<String, String> |  | Labels for the ReasoningEngine. |
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the ReasoningEngine. |
| `create_time` | String | Output only. Timestamp when this ReasoningEngine was created. |
| `encryption_spec` | String | Customer-managed encryption key spec for a ReasoningEngine. If set, this ReasoningEngine and all sub-resources of this ReasoningEngine will be secured by this key. |
| `spec` | String | Optional. Configurations of the ReasoningEngine |
| `update_time` | String | Output only. Timestamp when this ReasoningEngine was most recently updated. |
| `name` | String | Identifier. The resource name of the ReasoningEngine. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}` |
| `description` | String | Optional. The description of the ReasoningEngine. |
| `labels` | HashMap<String, String> | Labels for the ReasoningEngine. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reasoning_engine
reasoning_engine = provider.aiplatform_api.Reasoning_engine {
}

# Access reasoning_engine outputs
reasoning_engine_id = reasoning_engine.id
reasoning_engine_display_name = reasoning_engine.display_name
reasoning_engine_create_time = reasoning_engine.create_time
reasoning_engine_encryption_spec = reasoning_engine.encryption_spec
reasoning_engine_spec = reasoning_engine.spec
reasoning_engine_update_time = reasoning_engine.update_time
reasoning_engine_name = reasoning_engine.name
reasoning_engine_description = reasoning_engine.description
reasoning_engine_labels = reasoning_engine.labels
reasoning_engine_etag = reasoning_engine.etag
```

---


### Context

Creates a Context associated with a MetadataStore.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | User provided display name of the Context. May be up to 128 Unicode characters. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Contexts. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Context (System labels are excluded). |
| `etag` | String |  | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `metadata` | HashMap<String, String> |  | Properties of the Context. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |
| `parent_contexts` | Vec<String> |  | Output only. A list of resource names of Contexts that are parents of this Context. A Context may have at most 10 parent_contexts. |
| `create_time` | String |  | Output only. Timestamp when this Context was created. |
| `schema_title` | String |  | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `description` | String |  | Description of the Context |
| `schema_version` | String |  | The version of the schema in schema_name to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `update_time` | String |  | Output only. Timestamp when this Context was last updated. |
| `name` | String |  | Immutable. The resource name of the Context. |
| `parent` | String | ✅ | Required. The resource name of the MetadataStore where the Context should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | User provided display name of the Context. May be up to 128 Unicode characters. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Contexts. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Context (System labels are excluded). |
| `etag` | String | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `metadata` | HashMap<String, String> | Properties of the Context. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |
| `parent_contexts` | Vec<String> | Output only. A list of resource names of Contexts that are parents of this Context. A Context may have at most 10 parent_contexts. |
| `create_time` | String | Output only. Timestamp when this Context was created. |
| `schema_title` | String | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `description` | String | Description of the Context |
| `schema_version` | String | The version of the schema in schema_name to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `update_time` | String | Output only. Timestamp when this Context was last updated. |
| `name` | String | Immutable. The resource name of the Context. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create context
context = provider.aiplatform_api.Context {
    parent = "value"  # Required. The resource name of the MetadataStore where the Context should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}`
}

# Access context outputs
context_id = context.id
context_display_name = context.display_name
context_labels = context.labels
context_etag = context.etag
context_metadata = context.metadata
context_parent_contexts = context.parent_contexts
context_create_time = context.create_time
context_schema_title = context.schema_title
context_description = context.description
context_schema_version = context.schema_version
context_update_time = context.update_time
context_name = context.name
```

---


### Location

Given an input text, it returns a score that evaluates the factuality of the text. It also extracts and returns claims from the text and provides supporting facts.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | String |  | Optional. Parameters that can be set to override default settings per request. |
| `facts` | Vec<String> |  | Optional. Facts used to generate the text can also be used to corroborate the text. |
| `content` | String |  | Optional. Input content to corroborate, only text format is supported for now. |
| `parent` | String | ✅ | Required. The resource name of the Location from which to corroborate text. The users must have permission to make a call in the project. Format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.aiplatform_api.Location {
    parent = "value"  # Required. The resource name of the Location from which to corroborate text. The users must have permission to make a call in the project. Format: `projects/{project}/locations/{location}`.
}

# Access location outputs
location_id = location.id
location_display_name = location.display_name
location_location_id = location.location_id
location_name = location.name
location_metadata = location.metadata
location_labels = location.labels
```

---


### Project

Gets a GenAI cache config.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Name of the cache config. Format: - `projects/{project}/cacheConfig`. |
| `disable_cache` | bool |  | If set to true, disables GenAI caching. Otherwise caching is enabled. |
| `name` | String | ✅ | Identifier. Name of the cache config. Format: - `projects/{project}/cacheConfig`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Name of the cache config. Format: - `projects/{project}/cacheConfig`. |
| `disable_cache` | bool | If set to true, disables GenAI caching. Otherwise caching is enabled. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access project outputs
project_id = project.id
project_name = project.name
project_disable_cache = project.disable_cache
```

---


### Time_serie

Creates a TensorboardTimeSeries.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Name of the TensorboardTimeSeries. |
| `create_time` | String |  | Output only. Timestamp when this TensorboardTimeSeries was created. |
| `plugin_data` | String |  | Data of the current plugin, with the size limited to 65KB. |
| `display_name` | String |  | Required. User provided name of this TensorboardTimeSeries. This value should be unique among all TensorboardTimeSeries resources belonging to the same TensorboardRun resource (parent resource). |
| `update_time` | String |  | Output only. Timestamp when this TensorboardTimeSeries was last updated. |
| `value_type` | String |  | Required. Immutable. Type of TensorboardTimeSeries value. |
| `etag` | String |  | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `description` | String |  | Description of this TensorboardTimeSeries. |
| `plugin_name` | String |  | Immutable. Name of the plugin this time series pertain to. Such as Scalar, Tensor, Blob |
| `metadata` | String |  | Output only. Scalar, Tensor, or Blob metadata for this TensorboardTimeSeries. |
| `parent` | String | ✅ | Required. The resource name of the TensorboardRun to create the TensorboardTimeSeries in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}/runs/{run}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Name of the TensorboardTimeSeries. |
| `create_time` | String | Output only. Timestamp when this TensorboardTimeSeries was created. |
| `plugin_data` | String | Data of the current plugin, with the size limited to 65KB. |
| `display_name` | String | Required. User provided name of this TensorboardTimeSeries. This value should be unique among all TensorboardTimeSeries resources belonging to the same TensorboardRun resource (parent resource). |
| `update_time` | String | Output only. Timestamp when this TensorboardTimeSeries was last updated. |
| `value_type` | String | Required. Immutable. Type of TensorboardTimeSeries value. |
| `etag` | String | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `description` | String | Description of this TensorboardTimeSeries. |
| `plugin_name` | String | Immutable. Name of the plugin this time series pertain to. Such as Scalar, Tensor, Blob |
| `metadata` | String | Output only. Scalar, Tensor, or Blob metadata for this TensorboardTimeSeries. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create time_serie
time_serie = provider.aiplatform_api.Time_serie {
    parent = "value"  # Required. The resource name of the TensorboardRun to create the TensorboardTimeSeries in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}/runs/{run}`
}

# Access time_serie outputs
time_serie_id = time_serie.id
time_serie_name = time_serie.name
time_serie_create_time = time_serie.create_time
time_serie_plugin_data = time_serie.plugin_data
time_serie_display_name = time_serie.display_name
time_serie_update_time = time_serie.update_time
time_serie_value_type = time_serie.value_type
time_serie_etag = time_serie.etag
time_serie_description = time_serie.description
time_serie_plugin_name = time_serie.plugin_name
time_serie_metadata = time_serie.metadata
```

---


### Tuning_job

Creates a TuningJob. A created TuningJob right away will be attempted to be run.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The detailed state of the job. |
| `service_account` | String |  | The service account that the tuningJob workload runs as. If not specified, the Vertex AI Secure Fine-Tuned Service Agent in the project will be used. See https://cloud.google.com/iam/docs/service-agents#vertex-ai-secure-fine-tuning-service-agent Users starting the pipeline must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `tuned_model_display_name` | String |  | Optional. The display name of the TunedModel. The name can be up to 128 characters long and can consist of any UTF-8 characters. For continuous tuning, tuned_model_display_name will by default use the same display name as the pre-tuned model. If a new display name is provided, the tuning job will create a new model instead of a new version. |
| `supervised_tuning_spec` | String |  | Tuning Spec for Supervised Fine Tuning. |
| `description` | String |  | Optional. The description of the TuningJob. |
| `start_time` | String |  | Output only. Time when the TuningJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `name` | String |  | Output only. Identifier. Resource name of a TuningJob. Format: `projects/{project}/locations/{location}/tuningJobs/{tuning_job}` |
| `base_model` | String |  | The base model that is being tuned. See [Supported models](https://cloud.google.com/vertex-ai/generative-ai/docs/model-reference/tuning#supported_models). |
| `error` | String |  | Output only. Only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `create_time` | String |  | Output only. Time when the TuningJob was created. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize TuningJob and generated resources such as Model and Endpoint. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `tuning_data_stats` | String |  | Output only. The tuning data statistics associated with this TuningJob. |
| `update_time` | String |  | Output only. Time when the TuningJob was most recently updated. |
| `encryption_spec` | String |  | Customer-managed encryption key options for a TuningJob. If this is set, then all resources created by the TuningJob will be encrypted with the provided encryption key. |
| `end_time` | String |  | Output only. Time when the TuningJob entered any of the following JobStates: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`, `JOB_STATE_EXPIRED`. |
| `tuned_model` | String |  | Output only. The tuned model resources associated with this TuningJob. |
| `experiment` | String |  | Output only. The Experiment associated with this TuningJob. |
| `pre_tuned_model` | String |  | The pre-tuned model for continuous tuning. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the TuningJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The detailed state of the job. |
| `service_account` | String | The service account that the tuningJob workload runs as. If not specified, the Vertex AI Secure Fine-Tuned Service Agent in the project will be used. See https://cloud.google.com/iam/docs/service-agents#vertex-ai-secure-fine-tuning-service-agent Users starting the pipeline must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `tuned_model_display_name` | String | Optional. The display name of the TunedModel. The name can be up to 128 characters long and can consist of any UTF-8 characters. For continuous tuning, tuned_model_display_name will by default use the same display name as the pre-tuned model. If a new display name is provided, the tuning job will create a new model instead of a new version. |
| `supervised_tuning_spec` | String | Tuning Spec for Supervised Fine Tuning. |
| `description` | String | Optional. The description of the TuningJob. |
| `start_time` | String | Output only. Time when the TuningJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `name` | String | Output only. Identifier. Resource name of a TuningJob. Format: `projects/{project}/locations/{location}/tuningJobs/{tuning_job}` |
| `base_model` | String | The base model that is being tuned. See [Supported models](https://cloud.google.com/vertex-ai/generative-ai/docs/model-reference/tuning#supported_models). |
| `error` | String | Output only. Only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `create_time` | String | Output only. Time when the TuningJob was created. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize TuningJob and generated resources such as Model and Endpoint. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `tuning_data_stats` | String | Output only. The tuning data statistics associated with this TuningJob. |
| `update_time` | String | Output only. Time when the TuningJob was most recently updated. |
| `encryption_spec` | String | Customer-managed encryption key options for a TuningJob. If this is set, then all resources created by the TuningJob will be encrypted with the provided encryption key. |
| `end_time` | String | Output only. Time when the TuningJob entered any of the following JobStates: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`, `JOB_STATE_EXPIRED`. |
| `tuned_model` | String | Output only. The tuned model resources associated with this TuningJob. |
| `experiment` | String | Output only. The Experiment associated with this TuningJob. |
| `pre_tuned_model` | String | The pre-tuned model for continuous tuning. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tuning_job
tuning_job = provider.aiplatform_api.Tuning_job {
    parent = "value"  # Required. The resource name of the Location to create the TuningJob in. Format: `projects/{project}/locations/{location}`
}

# Access tuning_job outputs
tuning_job_id = tuning_job.id
tuning_job_state = tuning_job.state
tuning_job_service_account = tuning_job.service_account
tuning_job_tuned_model_display_name = tuning_job.tuned_model_display_name
tuning_job_supervised_tuning_spec = tuning_job.supervised_tuning_spec
tuning_job_description = tuning_job.description
tuning_job_start_time = tuning_job.start_time
tuning_job_name = tuning_job.name
tuning_job_base_model = tuning_job.base_model
tuning_job_error = tuning_job.error
tuning_job_create_time = tuning_job.create_time
tuning_job_labels = tuning_job.labels
tuning_job_tuning_data_stats = tuning_job.tuning_data_stats
tuning_job_update_time = tuning_job.update_time
tuning_job_encryption_spec = tuning_job.encryption_spec
tuning_job_end_time = tuning_job.end_time
tuning_job_tuned_model = tuning_job.tuned_model
tuning_job_experiment = tuning_job.experiment
tuning_job_pre_tuned_model = tuning_job.pre_tuned_model
```

---


### Model_deployment_monitoring_job

Creates a ModelDeploymentMonitoringJob. It will run periodically on a configured interval.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `logging_sampling_strategy` | String |  | Required. Sample Strategy for logging. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a ModelDeploymentMonitoringJob. If set, this ModelDeploymentMonitoringJob and all sub-resources of this ModelDeploymentMonitoringJob will be secured by this key. |
| `stats_anomalies_base_directory` | String |  | Stats anomalies base folder path. |
| `endpoint` | String |  | Required. Endpoint resource name. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}` |
| `error` | String |  | Output only. Only populated when the job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `model_monitoring_alert_config` | String |  | Alert config for model monitoring. |
| `enable_monitoring_pipeline_logs` | bool |  | If true, the scheduled monitoring pipeline logs are sent to Google Cloud Logging, including pipeline status and anomalies detected. Please note the logs incur cost, which are subject to [Cloud Logging pricing](https://cloud.google.com/logging#pricing). |
| `sample_predict_instance` | String |  | Sample Predict instance, same format as PredictRequest.instances, this can be set as a replacement of ModelDeploymentMonitoringJob.predict_instance_schema_uri. If not set, we will generate predict schema from collected predict requests. |
| `name` | String |  | Output only. Resource name of a ModelDeploymentMonitoringJob. |
| `latest_monitoring_pipeline_metadata` | String |  | Output only. Latest triggered monitoring pipeline metadata. |
| `bigquery_tables` | Vec<String> |  | Output only. The created bigquery tables for the job under customer project. Customer could do their own query & analysis. There could be 4 log tables in maximum: 1. Training data logging predict request/response 2. Serving data logging predict request/response |
| `next_schedule_time` | String |  | Output only. Timestamp when this monitoring pipeline will be scheduled to run for the next round. |
| `predict_instance_schema_uri` | String |  | YAML schema file uri describing the format of a single instance, which are given to format this Endpoint's prediction (and explanation). If not set, we will generate predict schema from collected predict requests. |
| `state` | String |  | Output only. The detailed state of the monitoring job. When the job is still creating, the state will be 'PENDING'. Once the job is successfully created, the state will be 'RUNNING'. Pause the job, the state will be 'PAUSED'. Resume the job, the state will return to 'RUNNING'. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `log_ttl` | String |  | The TTL of BigQuery tables in user projects which stores logs. A day is the basic unit of the TTL and we take the ceil of TTL/86400(a day). e.g. { second: 3600} indicates ttl = 1 day. |
| `create_time` | String |  | Output only. Timestamp when this ModelDeploymentMonitoringJob was created. |
| `model_deployment_monitoring_objective_configs` | Vec<String> |  | Required. The config for monitoring objectives. This is a per DeployedModel config. Each DeployedModel needs to be configured separately. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this ModelDeploymentMonitoringJob was updated most recently. |
| `schedule_state` | String |  | Output only. Schedule state when the monitoring job is in Running state. |
| `model_deployment_monitoring_schedule_config` | String |  | Required. Schedule config for running the monitoring job. |
| `analysis_instance_schema_uri` | String |  | YAML schema file uri describing the format of a single instance that you want Tensorflow Data Validation (TFDV) to analyze. If this field is empty, all the feature data types are inferred from predict_instance_schema_uri, meaning that TFDV will use the data in the exact format(data type) as prediction request/response. If there are any data type differences between predict instance and TFDV instance, this field can be used to override the schema. For models trained with Vertex AI, this field must be set as all the fields in predict instance formatted as string. |
| `display_name` | String |  | Required. The user-defined name of the ModelDeploymentMonitoringJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. Display name of a ModelDeploymentMonitoringJob. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your ModelDeploymentMonitoringJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `parent` | String | ✅ | Required. The parent of the ModelDeploymentMonitoringJob. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `logging_sampling_strategy` | String | Required. Sample Strategy for logging. |
| `encryption_spec` | String | Customer-managed encryption key spec for a ModelDeploymentMonitoringJob. If set, this ModelDeploymentMonitoringJob and all sub-resources of this ModelDeploymentMonitoringJob will be secured by this key. |
| `stats_anomalies_base_directory` | String | Stats anomalies base folder path. |
| `endpoint` | String | Required. Endpoint resource name. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}` |
| `error` | String | Output only. Only populated when the job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `model_monitoring_alert_config` | String | Alert config for model monitoring. |
| `enable_monitoring_pipeline_logs` | bool | If true, the scheduled monitoring pipeline logs are sent to Google Cloud Logging, including pipeline status and anomalies detected. Please note the logs incur cost, which are subject to [Cloud Logging pricing](https://cloud.google.com/logging#pricing). |
| `sample_predict_instance` | String | Sample Predict instance, same format as PredictRequest.instances, this can be set as a replacement of ModelDeploymentMonitoringJob.predict_instance_schema_uri. If not set, we will generate predict schema from collected predict requests. |
| `name` | String | Output only. Resource name of a ModelDeploymentMonitoringJob. |
| `latest_monitoring_pipeline_metadata` | String | Output only. Latest triggered monitoring pipeline metadata. |
| `bigquery_tables` | Vec<String> | Output only. The created bigquery tables for the job under customer project. Customer could do their own query & analysis. There could be 4 log tables in maximum: 1. Training data logging predict request/response 2. Serving data logging predict request/response |
| `next_schedule_time` | String | Output only. Timestamp when this monitoring pipeline will be scheduled to run for the next round. |
| `predict_instance_schema_uri` | String | YAML schema file uri describing the format of a single instance, which are given to format this Endpoint's prediction (and explanation). If not set, we will generate predict schema from collected predict requests. |
| `state` | String | Output only. The detailed state of the monitoring job. When the job is still creating, the state will be 'PENDING'. Once the job is successfully created, the state will be 'RUNNING'. Pause the job, the state will be 'PAUSED'. Resume the job, the state will return to 'RUNNING'. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `log_ttl` | String | The TTL of BigQuery tables in user projects which stores logs. A day is the basic unit of the TTL and we take the ceil of TTL/86400(a day). e.g. { second: 3600} indicates ttl = 1 day. |
| `create_time` | String | Output only. Timestamp when this ModelDeploymentMonitoringJob was created. |
| `model_deployment_monitoring_objective_configs` | Vec<String> | Required. The config for monitoring objectives. This is a per DeployedModel config. Each DeployedModel needs to be configured separately. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this ModelDeploymentMonitoringJob was updated most recently. |
| `schedule_state` | String | Output only. Schedule state when the monitoring job is in Running state. |
| `model_deployment_monitoring_schedule_config` | String | Required. Schedule config for running the monitoring job. |
| `analysis_instance_schema_uri` | String | YAML schema file uri describing the format of a single instance that you want Tensorflow Data Validation (TFDV) to analyze. If this field is empty, all the feature data types are inferred from predict_instance_schema_uri, meaning that TFDV will use the data in the exact format(data type) as prediction request/response. If there are any data type differences between predict instance and TFDV instance, this field can be used to override the schema. For models trained with Vertex AI, this field must be set as all the fields in predict instance formatted as string. |
| `display_name` | String | Required. The user-defined name of the ModelDeploymentMonitoringJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. Display name of a ModelDeploymentMonitoringJob. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your ModelDeploymentMonitoringJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model_deployment_monitoring_job
model_deployment_monitoring_job = provider.aiplatform_api.Model_deployment_monitoring_job {
    parent = "value"  # Required. The parent of the ModelDeploymentMonitoringJob. Format: `projects/{project}/locations/{location}`
}

# Access model_deployment_monitoring_job outputs
model_deployment_monitoring_job_id = model_deployment_monitoring_job.id
model_deployment_monitoring_job_logging_sampling_strategy = model_deployment_monitoring_job.logging_sampling_strategy
model_deployment_monitoring_job_encryption_spec = model_deployment_monitoring_job.encryption_spec
model_deployment_monitoring_job_stats_anomalies_base_directory = model_deployment_monitoring_job.stats_anomalies_base_directory
model_deployment_monitoring_job_endpoint = model_deployment_monitoring_job.endpoint
model_deployment_monitoring_job_error = model_deployment_monitoring_job.error
model_deployment_monitoring_job_model_monitoring_alert_config = model_deployment_monitoring_job.model_monitoring_alert_config
model_deployment_monitoring_job_enable_monitoring_pipeline_logs = model_deployment_monitoring_job.enable_monitoring_pipeline_logs
model_deployment_monitoring_job_sample_predict_instance = model_deployment_monitoring_job.sample_predict_instance
model_deployment_monitoring_job_name = model_deployment_monitoring_job.name
model_deployment_monitoring_job_latest_monitoring_pipeline_metadata = model_deployment_monitoring_job.latest_monitoring_pipeline_metadata
model_deployment_monitoring_job_bigquery_tables = model_deployment_monitoring_job.bigquery_tables
model_deployment_monitoring_job_next_schedule_time = model_deployment_monitoring_job.next_schedule_time
model_deployment_monitoring_job_predict_instance_schema_uri = model_deployment_monitoring_job.predict_instance_schema_uri
model_deployment_monitoring_job_state = model_deployment_monitoring_job.state
model_deployment_monitoring_job_satisfies_pzi = model_deployment_monitoring_job.satisfies_pzi
model_deployment_monitoring_job_log_ttl = model_deployment_monitoring_job.log_ttl
model_deployment_monitoring_job_create_time = model_deployment_monitoring_job.create_time
model_deployment_monitoring_job_model_deployment_monitoring_objective_configs = model_deployment_monitoring_job.model_deployment_monitoring_objective_configs
model_deployment_monitoring_job_satisfies_pzs = model_deployment_monitoring_job.satisfies_pzs
model_deployment_monitoring_job_update_time = model_deployment_monitoring_job.update_time
model_deployment_monitoring_job_schedule_state = model_deployment_monitoring_job.schedule_state
model_deployment_monitoring_job_model_deployment_monitoring_schedule_config = model_deployment_monitoring_job.model_deployment_monitoring_schedule_config
model_deployment_monitoring_job_analysis_instance_schema_uri = model_deployment_monitoring_job.analysis_instance_schema_uri
model_deployment_monitoring_job_display_name = model_deployment_monitoring_job.display_name
model_deployment_monitoring_job_labels = model_deployment_monitoring_job.labels
```

---


### Metadata_store

Initializes a MetadataStore, including allocation of resources.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when this MetadataStore was created. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a Metadata Store. If set, this Metadata Store and all sub-resources of this Metadata Store are secured using this key. |
| `dataplex_config` | String |  | Optional. Dataplex integration settings. |
| `description` | String |  | Description of the MetadataStore. |
| `name` | String |  | Output only. The resource name of the MetadataStore instance. |
| `state` | String |  | Output only. State information of the MetadataStore. |
| `update_time` | String |  | Output only. Timestamp when this MetadataStore was last updated. |
| `parent` | String | ✅ | Required. The resource name of the Location where the MetadataStore should be created. Format: `projects/{project}/locations/{location}/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when this MetadataStore was created. |
| `encryption_spec` | String | Customer-managed encryption key spec for a Metadata Store. If set, this Metadata Store and all sub-resources of this Metadata Store are secured using this key. |
| `dataplex_config` | String | Optional. Dataplex integration settings. |
| `description` | String | Description of the MetadataStore. |
| `name` | String | Output only. The resource name of the MetadataStore instance. |
| `state` | String | Output only. State information of the MetadataStore. |
| `update_time` | String | Output only. Timestamp when this MetadataStore was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create metadata_store
metadata_store = provider.aiplatform_api.Metadata_store {
    parent = "value"  # Required. The resource name of the Location where the MetadataStore should be created. Format: `projects/{project}/locations/{location}/`
}

# Access metadata_store outputs
metadata_store_id = metadata_store.id
metadata_store_create_time = metadata_store.create_time
metadata_store_encryption_spec = metadata_store.encryption_spec
metadata_store_dataplex_config = metadata_store.dataplex_config
metadata_store_description = metadata_store.description
metadata_store_name = metadata_store.name
metadata_store_state = metadata_store.state
metadata_store_update_time = metadata_store.update_time
```

---


### Evaluation_item

Creates an Evaluation Item.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_uri` | String |  | The Cloud Storage object where the request or response is stored. |
| `evaluation_item_type` | String |  | Required. The type of the EvaluationItem. |
| `evaluation_request` | String |  | The request to evaluate. |
| `evaluation_response` | String |  | Output only. The response from evaluation. |
| `metadata` | String |  | Optional. Metadata for the EvaluationItem. |
| `name` | String |  | Identifier. The resource name of the EvaluationItem. Format: `projects/{project}/locations/{location}/evaluationItems/{evaluation_item}` |
| `error` | String |  | Output only. Error for the evaluation item. |
| `display_name` | String |  | Required. The display name of the EvaluationItem. |
| `labels` | HashMap<String, String> |  | Optional. Labels for the EvaluationItem. |
| `create_time` | String |  | Output only. Timestamp when this item was created. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Evaluation Item in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gcs_uri` | String | The Cloud Storage object where the request or response is stored. |
| `evaluation_item_type` | String | Required. The type of the EvaluationItem. |
| `evaluation_request` | String | The request to evaluate. |
| `evaluation_response` | String | Output only. The response from evaluation. |
| `metadata` | String | Optional. Metadata for the EvaluationItem. |
| `name` | String | Identifier. The resource name of the EvaluationItem. Format: `projects/{project}/locations/{location}/evaluationItems/{evaluation_item}` |
| `error` | String | Output only. Error for the evaluation item. |
| `display_name` | String | Required. The display name of the EvaluationItem. |
| `labels` | HashMap<String, String> | Optional. Labels for the EvaluationItem. |
| `create_time` | String | Output only. Timestamp when this item was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation_item
evaluation_item = provider.aiplatform_api.Evaluation_item {
    parent = "value"  # Required. The resource name of the Location to create the Evaluation Item in. Format: `projects/{project}/locations/{location}`
}

# Access evaluation_item outputs
evaluation_item_id = evaluation_item.id
evaluation_item_gcs_uri = evaluation_item.gcs_uri
evaluation_item_evaluation_item_type = evaluation_item.evaluation_item_type
evaluation_item_evaluation_request = evaluation_item.evaluation_request
evaluation_item_evaluation_response = evaluation_item.evaluation_response
evaluation_item_metadata = evaluation_item.metadata
evaluation_item_name = evaluation_item.name
evaluation_item_error = evaluation_item.error
evaluation_item_display_name = evaluation_item.display_name
evaluation_item_labels = evaluation_item.labels
evaluation_item_create_time = evaluation_item.create_time
```

---


### Event

Lists Events in a given session.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `session_events` | Vec<String> | A list of events matching the request. Ordered by timestamp in ascending order. |
| `next_page_token` | String | A token, which can be sent as ListEventsRequest.page_token to retrieve the next page. Absence of this field indicates there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access event outputs
event_id = event.id
event_session_events = event.session_events
event_next_page_token = event.next_page_token
```

---


### Nas_job

Creates a NasJob

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `end_time` | String |  | Output only. Time when the NasJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `nas_job_spec` | String |  | Required. The specification of a NasJob. |
| `enable_restricted_image_training` | bool |  | Optional. Enable a separation of Custom model training and restricted image training for tenant project. |
| `error` | String |  | Output only. Only populated when job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `encryption_spec` | String |  | Customer-managed encryption key options for a NasJob. If this is set, then all resources created by the NasJob will be encrypted with the provided encryption key. |
| `display_name` | String |  | Required. The display name of the NasJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `start_time` | String |  | Output only. Time when the NasJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `state` | String |  | Output only. The detailed state of the job. |
| `create_time` | String |  | Output only. Time when the NasJob was created. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize NasJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `update_time` | String |  | Output only. Time when the NasJob was most recently updated. |
| `nas_job_output` | String |  | Output only. Output of the NasJob. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `name` | String |  | Output only. Resource name of the NasJob. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the NasJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. Time when the NasJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `nas_job_spec` | String | Required. The specification of a NasJob. |
| `enable_restricted_image_training` | bool | Optional. Enable a separation of Custom model training and restricted image training for tenant project. |
| `error` | String | Output only. Only populated when job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `encryption_spec` | String | Customer-managed encryption key options for a NasJob. If this is set, then all resources created by the NasJob will be encrypted with the provided encryption key. |
| `display_name` | String | Required. The display name of the NasJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `start_time` | String | Output only. Time when the NasJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `state` | String | Output only. The detailed state of the job. |
| `create_time` | String | Output only. Time when the NasJob was created. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize NasJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `update_time` | String | Output only. Time when the NasJob was most recently updated. |
| `nas_job_output` | String | Output only. Output of the NasJob. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `name` | String | Output only. Resource name of the NasJob. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create nas_job
nas_job = provider.aiplatform_api.Nas_job {
    parent = "value"  # Required. The resource name of the Location to create the NasJob in. Format: `projects/{project}/locations/{location}`
}

# Access nas_job outputs
nas_job_id = nas_job.id
nas_job_end_time = nas_job.end_time
nas_job_nas_job_spec = nas_job.nas_job_spec
nas_job_enable_restricted_image_training = nas_job.enable_restricted_image_training
nas_job_error = nas_job.error
nas_job_encryption_spec = nas_job.encryption_spec
nas_job_display_name = nas_job.display_name
nas_job_start_time = nas_job.start_time
nas_job_satisfies_pzs = nas_job.satisfies_pzs
nas_job_state = nas_job.state
nas_job_create_time = nas_job.create_time
nas_job_labels = nas_job.labels
nas_job_update_time = nas_job.update_time
nas_job_nas_job_output = nas_job.nas_job_output
nas_job_satisfies_pzi = nas_job.satisfies_pzi
nas_job_name = nas_job.name
```

---


### Cached_content

Creates cached content, this call will initialize the cached content in the data storage, and users need to pay for the cache data storage.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. When the cache entry was last updated in UTC time. |
| `contents` | Vec<String> |  | Optional. Input only. Immutable. The content to cache |
| `tool_config` | String |  | Optional. Input only. Immutable. Tool config. This config is shared for all tools |
| `encryption_spec` | String |  | Input only. Immutable. Customer-managed encryption key spec for a `CachedContent`. If set, this `CachedContent` and all its sub-resources will be secured by this key. |
| `tools` | Vec<String> |  | Optional. Input only. Immutable. A list of `Tools` the model may use to generate the next response |
| `create_time` | String |  | Output only. Creation time of the cache entry. |
| `display_name` | String |  | Optional. Immutable. The user-generated meaningful display name of the cached content. |
| `name` | String |  | Immutable. Identifier. The server-generated resource name of the cached content Format: projects/{project}/locations/{location}/cachedContents/{cached_content} |
| `model` | String |  | Immutable. The name of the `Model` to use for cached content. Currently, only the published Gemini base models are supported, in form of projects/{PROJECT}/locations/{LOCATION}/publishers/google/models/{MODEL} |
| `system_instruction` | String |  | Optional. Input only. Immutable. Developer set system instruction. Currently, text only |
| `ttl` | String |  | Input only. The TTL for this resource. The expiration time is computed: now + TTL. |
| `expire_time` | String |  | Timestamp of when this resource is considered expired. This is *always* provided on output, regardless of what was sent on input. |
| `usage_metadata` | String |  | Output only. Metadata on the usage of the cached content. |
| `parent` | String | ✅ | Required. The parent resource where the cached content will be created |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. When the cache entry was last updated in UTC time. |
| `contents` | Vec<String> | Optional. Input only. Immutable. The content to cache |
| `tool_config` | String | Optional. Input only. Immutable. Tool config. This config is shared for all tools |
| `encryption_spec` | String | Input only. Immutable. Customer-managed encryption key spec for a `CachedContent`. If set, this `CachedContent` and all its sub-resources will be secured by this key. |
| `tools` | Vec<String> | Optional. Input only. Immutable. A list of `Tools` the model may use to generate the next response |
| `create_time` | String | Output only. Creation time of the cache entry. |
| `display_name` | String | Optional. Immutable. The user-generated meaningful display name of the cached content. |
| `name` | String | Immutable. Identifier. The server-generated resource name of the cached content Format: projects/{project}/locations/{location}/cachedContents/{cached_content} |
| `model` | String | Immutable. The name of the `Model` to use for cached content. Currently, only the published Gemini base models are supported, in form of projects/{PROJECT}/locations/{LOCATION}/publishers/google/models/{MODEL} |
| `system_instruction` | String | Optional. Input only. Immutable. Developer set system instruction. Currently, text only |
| `ttl` | String | Input only. The TTL for this resource. The expiration time is computed: now + TTL. |
| `expire_time` | String | Timestamp of when this resource is considered expired. This is *always* provided on output, regardless of what was sent on input. |
| `usage_metadata` | String | Output only. Metadata on the usage of the cached content. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cached_content
cached_content = provider.aiplatform_api.Cached_content {
    parent = "value"  # Required. The parent resource where the cached content will be created
}

# Access cached_content outputs
cached_content_id = cached_content.id
cached_content_update_time = cached_content.update_time
cached_content_contents = cached_content.contents
cached_content_tool_config = cached_content.tool_config
cached_content_encryption_spec = cached_content.encryption_spec
cached_content_tools = cached_content.tools
cached_content_create_time = cached_content.create_time
cached_content_display_name = cached_content.display_name
cached_content_name = cached_content.name
cached_content_model = cached_content.model
cached_content_system_instruction = cached_content.system_instruction
cached_content_ttl = cached_content.ttl
cached_content_expire_time = cached_content.expire_time
cached_content_usage_metadata = cached_content.usage_metadata
```

---


### Hyperparameter_tuning_job

Creates a HyperparameterTuningJob

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time when the HyperparameterTuningJob was created. |
| `parallel_trial_count` | i64 |  | Required. The desired number of Trials to run in parallel. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize HyperparameterTuningJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `error` | String |  | Output only. Only populated when job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `state` | String |  | Output only. The detailed state of the job. |
| `name` | String |  | Output only. Resource name of the HyperparameterTuningJob. |
| `encryption_spec` | String |  | Customer-managed encryption key options for a HyperparameterTuningJob. If this is set, then all resources created by the HyperparameterTuningJob will be encrypted with the provided encryption key. |
| `trials` | Vec<String> |  | Output only. Trials of the HyperparameterTuningJob. |
| `update_time` | String |  | Output only. Time when the HyperparameterTuningJob was most recently updated. |
| `study_spec` | String |  | Required. Study configuration of the HyperparameterTuningJob. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `end_time` | String |  | Output only. Time when the HyperparameterTuningJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `start_time` | String |  | Output only. Time when the HyperparameterTuningJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `max_failed_trial_count` | i64 |  | The number of failed Trials that need to be seen before failing the HyperparameterTuningJob. If set to 0, Vertex AI decides how many Trials must fail before the whole job fails. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `trial_job_spec` | String |  | Required. The spec of a trial job. The same spec applies to the CustomJobs created in all the trials. |
| `display_name` | String |  | Required. The display name of the HyperparameterTuningJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `max_trial_count` | i64 |  | Required. The desired total number of Trials. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the HyperparameterTuningJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time when the HyperparameterTuningJob was created. |
| `parallel_trial_count` | i64 | Required. The desired number of Trials to run in parallel. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize HyperparameterTuningJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `error` | String | Output only. Only populated when job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `state` | String | Output only. The detailed state of the job. |
| `name` | String | Output only. Resource name of the HyperparameterTuningJob. |
| `encryption_spec` | String | Customer-managed encryption key options for a HyperparameterTuningJob. If this is set, then all resources created by the HyperparameterTuningJob will be encrypted with the provided encryption key. |
| `trials` | Vec<String> | Output only. Trials of the HyperparameterTuningJob. |
| `update_time` | String | Output only. Time when the HyperparameterTuningJob was most recently updated. |
| `study_spec` | String | Required. Study configuration of the HyperparameterTuningJob. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `end_time` | String | Output only. Time when the HyperparameterTuningJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `start_time` | String | Output only. Time when the HyperparameterTuningJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `max_failed_trial_count` | i64 | The number of failed Trials that need to be seen before failing the HyperparameterTuningJob. If set to 0, Vertex AI decides how many Trials must fail before the whole job fails. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `trial_job_spec` | String | Required. The spec of a trial job. The same spec applies to the CustomJobs created in all the trials. |
| `display_name` | String | Required. The display name of the HyperparameterTuningJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `max_trial_count` | i64 | Required. The desired total number of Trials. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create hyperparameter_tuning_job
hyperparameter_tuning_job = provider.aiplatform_api.Hyperparameter_tuning_job {
    parent = "value"  # Required. The resource name of the Location to create the HyperparameterTuningJob in. Format: `projects/{project}/locations/{location}`
}

# Access hyperparameter_tuning_job outputs
hyperparameter_tuning_job_id = hyperparameter_tuning_job.id
hyperparameter_tuning_job_create_time = hyperparameter_tuning_job.create_time
hyperparameter_tuning_job_parallel_trial_count = hyperparameter_tuning_job.parallel_trial_count
hyperparameter_tuning_job_labels = hyperparameter_tuning_job.labels
hyperparameter_tuning_job_error = hyperparameter_tuning_job.error
hyperparameter_tuning_job_state = hyperparameter_tuning_job.state
hyperparameter_tuning_job_name = hyperparameter_tuning_job.name
hyperparameter_tuning_job_encryption_spec = hyperparameter_tuning_job.encryption_spec
hyperparameter_tuning_job_trials = hyperparameter_tuning_job.trials
hyperparameter_tuning_job_update_time = hyperparameter_tuning_job.update_time
hyperparameter_tuning_job_study_spec = hyperparameter_tuning_job.study_spec
hyperparameter_tuning_job_satisfies_pzi = hyperparameter_tuning_job.satisfies_pzi
hyperparameter_tuning_job_end_time = hyperparameter_tuning_job.end_time
hyperparameter_tuning_job_start_time = hyperparameter_tuning_job.start_time
hyperparameter_tuning_job_max_failed_trial_count = hyperparameter_tuning_job.max_failed_trial_count
hyperparameter_tuning_job_satisfies_pzs = hyperparameter_tuning_job.satisfies_pzs
hyperparameter_tuning_job_trial_job_spec = hyperparameter_tuning_job.trial_job_spec
hyperparameter_tuning_job_display_name = hyperparameter_tuning_job.display_name
hyperparameter_tuning_job_max_trial_count = hyperparameter_tuning_job.max_trial_count
```

---


### Metadata_schema

Creates a MetadataSchema.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when this MetadataSchema was created. |
| `schema_type` | String |  | The type of the MetadataSchema. This is a property that identifies which metadata types will use the MetadataSchema. |
| `schema` | String |  | Required. The raw YAML string representation of the MetadataSchema. The combination of [MetadataSchema.version] and the schema name given by `title` in [MetadataSchema.schema] must be unique within a MetadataStore. The schema is defined as an OpenAPI 3.0.2 [MetadataSchema Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.2.md#schemaObject) |
| `schema_version` | String |  | The version of the MetadataSchema. The version's format must match the following regular expression: `^[0-9]+.+.+$`, which would allow to order/compare different versions. Example: 1.0.0, 1.0.1, etc. |
| `description` | String |  | Description of the Metadata Schema |
| `name` | String |  | Output only. The resource name of the MetadataSchema. |
| `parent` | String | ✅ | Required. The resource name of the MetadataStore where the MetadataSchema should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when this MetadataSchema was created. |
| `schema_type` | String | The type of the MetadataSchema. This is a property that identifies which metadata types will use the MetadataSchema. |
| `schema` | String | Required. The raw YAML string representation of the MetadataSchema. The combination of [MetadataSchema.version] and the schema name given by `title` in [MetadataSchema.schema] must be unique within a MetadataStore. The schema is defined as an OpenAPI 3.0.2 [MetadataSchema Object](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.2.md#schemaObject) |
| `schema_version` | String | The version of the MetadataSchema. The version's format must match the following regular expression: `^[0-9]+.+.+$`, which would allow to order/compare different versions. Example: 1.0.0, 1.0.1, etc. |
| `description` | String | Description of the Metadata Schema |
| `name` | String | Output only. The resource name of the MetadataSchema. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create metadata_schema
metadata_schema = provider.aiplatform_api.Metadata_schema {
    parent = "value"  # Required. The resource name of the MetadataStore where the MetadataSchema should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}`
}

# Access metadata_schema outputs
metadata_schema_id = metadata_schema.id
metadata_schema_create_time = metadata_schema.create_time
metadata_schema_schema_type = metadata_schema.schema_type
metadata_schema_schema = metadata_schema.schema
metadata_schema_schema_version = metadata_schema.schema_version
metadata_schema_description = metadata_schema.description
metadata_schema_name = metadata_schema.name
```

---


### Feature_online_store

Creates a new FeatureOnlineStore in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when this FeatureOnlineStore was created. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `bigtable` | String |  | Contains settings for the Cloud Bigtable instance that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore. |
| `name` | String |  | Identifier. Name of the FeatureOnlineStore. Format: `projects/{project}/locations/{location}/featureOnlineStores/{featureOnlineStore}` |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your FeatureOnlineStore. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureOnlineStore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `dedicated_serving_endpoint` | String |  | Optional. The dedicated serving endpoint for this FeatureOnlineStore, which is different from common Vertex service endpoint. |
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `optimized` | String |  | Contains settings for the Optimized store that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore. When choose Optimized storage type, need to set PrivateServiceConnectConfig.enable_private_service_connect to use private endpoint. Otherwise will use public endpoint by default. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this FeatureOnlineStore was last updated. |
| `embedding_management` | String |  | Optional. Deprecated: This field is no longer needed anymore and embedding management is automatically enabled when specifying Optimized storage type. |
| `encryption_spec` | String |  | Optional. Customer-managed encryption key spec for data storage. If set, online store will be secured by this key. |
| `state` | String |  | Output only. State of the featureOnlineStore. |
| `parent` | String | ✅ | Required. The resource name of the Location to create FeatureOnlineStores. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when this FeatureOnlineStore was created. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `bigtable` | String | Contains settings for the Cloud Bigtable instance that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore. |
| `name` | String | Identifier. Name of the FeatureOnlineStore. Format: `projects/{project}/locations/{location}/featureOnlineStores/{featureOnlineStore}` |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your FeatureOnlineStore. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureOnlineStore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `dedicated_serving_endpoint` | String | Optional. The dedicated serving endpoint for this FeatureOnlineStore, which is different from common Vertex service endpoint. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `optimized` | String | Contains settings for the Optimized store that will be created to serve featureValues for all FeatureViews under this FeatureOnlineStore. When choose Optimized storage type, need to set PrivateServiceConnectConfig.enable_private_service_connect to use private endpoint. Otherwise will use public endpoint by default. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this FeatureOnlineStore was last updated. |
| `embedding_management` | String | Optional. Deprecated: This field is no longer needed anymore and embedding management is automatically enabled when specifying Optimized storage type. |
| `encryption_spec` | String | Optional. Customer-managed encryption key spec for data storage. If set, online store will be secured by this key. |
| `state` | String | Output only. State of the featureOnlineStore. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature_online_store
feature_online_store = provider.aiplatform_api.Feature_online_store {
    parent = "value"  # Required. The resource name of the Location to create FeatureOnlineStores. Format: `projects/{project}/locations/{location}`
}

# Access feature_online_store outputs
feature_online_store_id = feature_online_store.id
feature_online_store_create_time = feature_online_store.create_time
feature_online_store_satisfies_pzs = feature_online_store.satisfies_pzs
feature_online_store_bigtable = feature_online_store.bigtable
feature_online_store_name = feature_online_store.name
feature_online_store_labels = feature_online_store.labels
feature_online_store_dedicated_serving_endpoint = feature_online_store.dedicated_serving_endpoint
feature_online_store_etag = feature_online_store.etag
feature_online_store_optimized = feature_online_store.optimized
feature_online_store_satisfies_pzi = feature_online_store.satisfies_pzi
feature_online_store_update_time = feature_online_store.update_time
feature_online_store_embedding_management = feature_online_store.embedding_management
feature_online_store_encryption_spec = feature_online_store.encryption_spec
feature_online_store_state = feature_online_store.state
```

---


### Trial

Adds a user provided Trial to a Study.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of the Trial assigned by the service. |
| `client_id` | String |  | Output only. The identifier of the client that originally requested this Trial. Each client is identified by a unique client_id. When a client asks for a suggestion, Vertex AI Vizier will assign it a Trial. The client should evaluate the Trial, complete it, and report back to Vertex AI Vizier. If suggestion is asked again by same client_id before the Trial is completed, the same Trial will be returned. Multiple clients with different client_ids can ask for suggestions simultaneously, each of them will get their own Trial. |
| `measurements` | Vec<String> |  | Output only. A list of measurements that are strictly lexicographically ordered by their induced tuples (steps, elapsed_duration). These are used for early stopping computations. |
| `end_time` | String |  | Output only. Time when the Trial's status changed to `SUCCEEDED` or `INFEASIBLE`. |
| `parameters` | Vec<String> |  | Output only. The parameters of the Trial. |
| `infeasible_reason` | String |  | Output only. A human readable string describing why the Trial is infeasible. This is set only if Trial state is `INFEASIBLE`. |
| `web_access_uris` | HashMap<String, String> |  | Output only. URIs for accessing [interactive shells](https://cloud.google.com/vertex-ai/docs/training/monitor-debug-interactive-shell) (one URI for each training node). Only available if this trial is part of a HyperparameterTuningJob and the job's trial_job_spec.enable_web_access field is `true`. The keys are names of each node used for the trial; for example, `workerpool0-0` for the primary node, `workerpool1-0` for the first node in the second worker pool, and `workerpool1-1` for the second node in the second worker pool. The values are the URIs for each node's interactive shell. |
| `final_measurement` | String |  | Output only. The final measurement containing the objective value. |
| `start_time` | String |  | Output only. Time when the Trial was started. |
| `state` | String |  | Output only. The detailed state of the Trial. |
| `id` | String |  | Output only. The identifier of the Trial assigned by the service. |
| `custom_job` | String |  | Output only. The CustomJob name linked to the Trial. It's set for a HyperparameterTuningJob's Trial. |
| `parent` | String | ✅ | Required. The resource name of the Study to create the Trial in. Format: `projects/{project}/locations/{location}/studies/{study}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the Trial assigned by the service. |
| `client_id` | String | Output only. The identifier of the client that originally requested this Trial. Each client is identified by a unique client_id. When a client asks for a suggestion, Vertex AI Vizier will assign it a Trial. The client should evaluate the Trial, complete it, and report back to Vertex AI Vizier. If suggestion is asked again by same client_id before the Trial is completed, the same Trial will be returned. Multiple clients with different client_ids can ask for suggestions simultaneously, each of them will get their own Trial. |
| `measurements` | Vec<String> | Output only. A list of measurements that are strictly lexicographically ordered by their induced tuples (steps, elapsed_duration). These are used for early stopping computations. |
| `end_time` | String | Output only. Time when the Trial's status changed to `SUCCEEDED` or `INFEASIBLE`. |
| `parameters` | Vec<String> | Output only. The parameters of the Trial. |
| `infeasible_reason` | String | Output only. A human readable string describing why the Trial is infeasible. This is set only if Trial state is `INFEASIBLE`. |
| `web_access_uris` | HashMap<String, String> | Output only. URIs for accessing [interactive shells](https://cloud.google.com/vertex-ai/docs/training/monitor-debug-interactive-shell) (one URI for each training node). Only available if this trial is part of a HyperparameterTuningJob and the job's trial_job_spec.enable_web_access field is `true`. The keys are names of each node used for the trial; for example, `workerpool0-0` for the primary node, `workerpool1-0` for the first node in the second worker pool, and `workerpool1-1` for the second node in the second worker pool. The values are the URIs for each node's interactive shell. |
| `final_measurement` | String | Output only. The final measurement containing the objective value. |
| `start_time` | String | Output only. Time when the Trial was started. |
| `state` | String | Output only. The detailed state of the Trial. |
| `id` | String | Output only. The identifier of the Trial assigned by the service. |
| `custom_job` | String | Output only. The CustomJob name linked to the Trial. It's set for a HyperparameterTuningJob's Trial. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trial
trial = provider.aiplatform_api.Trial {
    parent = "value"  # Required. The resource name of the Study to create the Trial in. Format: `projects/{project}/locations/{location}/studies/{study}`
}

# Access trial outputs
trial_id = trial.id
trial_name = trial.name
trial_client_id = trial.client_id
trial_measurements = trial.measurements
trial_end_time = trial.end_time
trial_parameters = trial.parameters
trial_infeasible_reason = trial.infeasible_reason
trial_web_access_uris = trial.web_access_uris
trial_final_measurement = trial.final_measurement
trial_start_time = trial.start_time
trial_state = trial.state
trial_id = trial.id
trial_custom_job = trial.custom_job
```

---


### Extension

Imports an Extension.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `tool_use_examples` | Vec<String> |  | Optional. Examples to illustrate the usage of the extension as a tool. |
| `update_time` | String |  | Output only. Timestamp when this Extension was most recently updated. |
| `extension_operations` | Vec<String> |  | Output only. Supported operations. |
| `name` | String |  | Identifier. The resource name of the Extension. |
| `private_service_connect_config` | String |  | Optional. The PrivateServiceConnect config for the extension. If specified, the service endpoints associated with the Extension should be [registered with private network access in the provided Service Directory](https://cloud.google.com/service-directory/docs/configuring-private-network-access). If the service contains more than one endpoint with a network, the service will arbitrarilty choose one of the endpoints to use for extension execution. |
| `display_name` | String |  | Required. The display name of the Extension. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `create_time` | String |  | Output only. Timestamp when this Extension was created. |
| `runtime_config` | String |  | Optional. Runtime config controlling the runtime behavior of this Extension. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `description` | String |  | Optional. The description of the Extension. |
| `manifest` | String |  | Required. Manifest of the Extension. |
| `parent` | String | ✅ | Required. The resource name of the Location to import the Extension in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `tool_use_examples` | Vec<String> | Optional. Examples to illustrate the usage of the extension as a tool. |
| `update_time` | String | Output only. Timestamp when this Extension was most recently updated. |
| `extension_operations` | Vec<String> | Output only. Supported operations. |
| `name` | String | Identifier. The resource name of the Extension. |
| `private_service_connect_config` | String | Optional. The PrivateServiceConnect config for the extension. If specified, the service endpoints associated with the Extension should be [registered with private network access in the provided Service Directory](https://cloud.google.com/service-directory/docs/configuring-private-network-access). If the service contains more than one endpoint with a network, the service will arbitrarilty choose one of the endpoints to use for extension execution. |
| `display_name` | String | Required. The display name of the Extension. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `create_time` | String | Output only. Timestamp when this Extension was created. |
| `runtime_config` | String | Optional. Runtime config controlling the runtime behavior of this Extension. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `description` | String | Optional. The description of the Extension. |
| `manifest` | String | Required. Manifest of the Extension. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create extension
extension = provider.aiplatform_api.Extension {
    parent = "value"  # Required. The resource name of the Location to import the Extension in. Format: `projects/{project}/locations/{location}`
}

# Access extension outputs
extension_id = extension.id
extension_etag = extension.etag
extension_satisfies_pzi = extension.satisfies_pzi
extension_tool_use_examples = extension.tool_use_examples
extension_update_time = extension.update_time
extension_extension_operations = extension.extension_operations
extension_name = extension.name
extension_private_service_connect_config = extension.private_service_connect_config
extension_display_name = extension.display_name
extension_create_time = extension.create_time
extension_runtime_config = extension.runtime_config
extension_satisfies_pzs = extension.satisfies_pzs
extension_description = extension.description
extension_manifest = extension.manifest
```

---


### Tuning_job

Creates a TuningJob. A created TuningJob right away will be attempted to be run.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `partner_model_tuning_spec` | String |  | Tuning Spec for open sourced and third party Partner models. |
| `output_uri` | String |  | Optional. Cloud Storage path to the directory where tuning job outputs are written to. This field is only available and required for open source models. |
| `veo_tuning_spec` | String |  | Tuning Spec for Veo Tuning. |
| `name` | String |  | Output only. Identifier. Resource name of a TuningJob. Format: `projects/{project}/locations/{location}/tuningJobs/{tuning_job}` |
| `distillation_spec` | String |  | Tuning Spec for Distillation. |
| `tuned_model_display_name` | String |  | Optional. The display name of the TunedModel. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `encryption_spec` | String |  | Customer-managed encryption key options for a TuningJob. If this is set, then all resources created by the TuningJob will be encrypted with the provided encryption key. |
| `description` | String |  | Optional. The description of the TuningJob. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize TuningJob and generated resources such as Model and Endpoint. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `service_account` | String |  | The service account that the tuningJob workload runs as. If not specified, the Vertex AI Secure Fine-Tuned Service Agent in the project will be used. See https://cloud.google.com/iam/docs/service-agents#vertex-ai-secure-fine-tuning-service-agent Users starting the pipeline must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `start_time` | String |  | Output only. Time when the TuningJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `tuned_model` | String |  | Output only. The tuned model resources associated with this TuningJob. |
| `create_time` | String |  | Output only. Time when the TuningJob was created. |
| `base_model` | String |  | The base model that is being tuned. See [Supported models](https://cloud.google.com/vertex-ai/generative-ai/docs/model-reference/tuning#supported_models). |
| `error` | String |  | Output only. Only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `end_time` | String |  | Output only. Time when the TuningJob entered any of the following JobStates: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`, `JOB_STATE_EXPIRED`. |
| `tuning_job_state` | String |  | Output only. The detail state of the tuning job (while the overall `JobState` is running). |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `custom_base_model` | String |  | Optional. The user-provided path to custom model weights. Set this field to tune a custom model. The path must be a Cloud Storage directory that contains the model weights in .safetensors format along with associated model metadata files. If this field is set, the base_model field must still be set to indicate which base model the custom model is derived from. This feature is only available for open source models. |
| `supervised_tuning_spec` | String |  | Tuning Spec for Supervised Fine Tuning. |
| `preference_optimization_spec` | String |  | Tuning Spec for Preference Optimization. |
| `state` | String |  | Output only. The detailed state of the job. |
| `experiment` | String |  | Output only. The Experiment associated with this TuningJob. |
| `pre_tuned_model` | String |  | The pre-tuned model for continuous tuning. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `evaluate_dataset_runs` | Vec<String> |  | Output only. Evaluation runs for the Tuning Job. |
| `update_time` | String |  | Output only. Time when the TuningJob was most recently updated. |
| `pipeline_job` | String |  | Output only. The resource name of the PipelineJob associated with the TuningJob. Format: `projects/{project}/locations/{location}/pipelineJobs/{pipeline_job}`. |
| `tuning_data_stats` | String |  | Output only. The tuning data statistics associated with this TuningJob. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the TuningJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `partner_model_tuning_spec` | String | Tuning Spec for open sourced and third party Partner models. |
| `output_uri` | String | Optional. Cloud Storage path to the directory where tuning job outputs are written to. This field is only available and required for open source models. |
| `veo_tuning_spec` | String | Tuning Spec for Veo Tuning. |
| `name` | String | Output only. Identifier. Resource name of a TuningJob. Format: `projects/{project}/locations/{location}/tuningJobs/{tuning_job}` |
| `distillation_spec` | String | Tuning Spec for Distillation. |
| `tuned_model_display_name` | String | Optional. The display name of the TunedModel. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `encryption_spec` | String | Customer-managed encryption key options for a TuningJob. If this is set, then all resources created by the TuningJob will be encrypted with the provided encryption key. |
| `description` | String | Optional. The description of the TuningJob. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize TuningJob and generated resources such as Model and Endpoint. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `service_account` | String | The service account that the tuningJob workload runs as. If not specified, the Vertex AI Secure Fine-Tuned Service Agent in the project will be used. See https://cloud.google.com/iam/docs/service-agents#vertex-ai-secure-fine-tuning-service-agent Users starting the pipeline must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `start_time` | String | Output only. Time when the TuningJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `tuned_model` | String | Output only. The tuned model resources associated with this TuningJob. |
| `create_time` | String | Output only. Time when the TuningJob was created. |
| `base_model` | String | The base model that is being tuned. See [Supported models](https://cloud.google.com/vertex-ai/generative-ai/docs/model-reference/tuning#supported_models). |
| `error` | String | Output only. Only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `end_time` | String | Output only. Time when the TuningJob entered any of the following JobStates: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`, `JOB_STATE_EXPIRED`. |
| `tuning_job_state` | String | Output only. The detail state of the tuning job (while the overall `JobState` is running). |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `custom_base_model` | String | Optional. The user-provided path to custom model weights. Set this field to tune a custom model. The path must be a Cloud Storage directory that contains the model weights in .safetensors format along with associated model metadata files. If this field is set, the base_model field must still be set to indicate which base model the custom model is derived from. This feature is only available for open source models. |
| `supervised_tuning_spec` | String | Tuning Spec for Supervised Fine Tuning. |
| `preference_optimization_spec` | String | Tuning Spec for Preference Optimization. |
| `state` | String | Output only. The detailed state of the job. |
| `experiment` | String | Output only. The Experiment associated with this TuningJob. |
| `pre_tuned_model` | String | The pre-tuned model for continuous tuning. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `evaluate_dataset_runs` | Vec<String> | Output only. Evaluation runs for the Tuning Job. |
| `update_time` | String | Output only. Time when the TuningJob was most recently updated. |
| `pipeline_job` | String | Output only. The resource name of the PipelineJob associated with the TuningJob. Format: `projects/{project}/locations/{location}/pipelineJobs/{pipeline_job}`. |
| `tuning_data_stats` | String | Output only. The tuning data statistics associated with this TuningJob. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tuning_job
tuning_job = provider.aiplatform_api.Tuning_job {
    parent = "value"  # Required. The resource name of the Location to create the TuningJob in. Format: `projects/{project}/locations/{location}`
}

# Access tuning_job outputs
tuning_job_id = tuning_job.id
tuning_job_partner_model_tuning_spec = tuning_job.partner_model_tuning_spec
tuning_job_output_uri = tuning_job.output_uri
tuning_job_veo_tuning_spec = tuning_job.veo_tuning_spec
tuning_job_name = tuning_job.name
tuning_job_distillation_spec = tuning_job.distillation_spec
tuning_job_tuned_model_display_name = tuning_job.tuned_model_display_name
tuning_job_encryption_spec = tuning_job.encryption_spec
tuning_job_description = tuning_job.description
tuning_job_labels = tuning_job.labels
tuning_job_service_account = tuning_job.service_account
tuning_job_start_time = tuning_job.start_time
tuning_job_tuned_model = tuning_job.tuned_model
tuning_job_create_time = tuning_job.create_time
tuning_job_base_model = tuning_job.base_model
tuning_job_error = tuning_job.error
tuning_job_end_time = tuning_job.end_time
tuning_job_tuning_job_state = tuning_job.tuning_job_state
tuning_job_satisfies_pzs = tuning_job.satisfies_pzs
tuning_job_custom_base_model = tuning_job.custom_base_model
tuning_job_supervised_tuning_spec = tuning_job.supervised_tuning_spec
tuning_job_preference_optimization_spec = tuning_job.preference_optimization_spec
tuning_job_state = tuning_job.state
tuning_job_experiment = tuning_job.experiment
tuning_job_pre_tuned_model = tuning_job.pre_tuned_model
tuning_job_satisfies_pzi = tuning_job.satisfies_pzi
tuning_job_evaluate_dataset_runs = tuning_job.evaluate_dataset_runs
tuning_job_update_time = tuning_job.update_time
tuning_job_pipeline_job = tuning_job.pipeline_job
tuning_job_tuning_data_stats = tuning_job.tuning_data_stats
```

---


### Feature_monitor

Creates a new FeatureMonitor in a given project, location and FeatureGroup.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when this FeatureMonitor was created. |
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your FeatureMonitor. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureMonitor(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `schedule_config` | String |  | Required. Schedule config for the FeatureMonitor. |
| `feature_selection_config` | String |  | Required. Feature selection config for the FeatureMonitor. |
| `update_time` | String |  | Output only. Timestamp when this FeatureMonitor was last updated. |
| `description` | String |  | Optional. Description of the FeatureMonitor. |
| `name` | String |  | Identifier. Name of the FeatureMonitor. Format: `projects/{project}/locations/{location}/featureGroups/{featureGroup}/featureMonitors/{featureMonitor}` |
| `parent` | String | ✅ | Required. The resource name of FeatureGroup to create FeatureMonitor. Format: `projects/{project}/locations/{location}/featureGroups/{featuregroup}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when this FeatureMonitor was created. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your FeatureMonitor. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureMonitor(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `schedule_config` | String | Required. Schedule config for the FeatureMonitor. |
| `feature_selection_config` | String | Required. Feature selection config for the FeatureMonitor. |
| `update_time` | String | Output only. Timestamp when this FeatureMonitor was last updated. |
| `description` | String | Optional. Description of the FeatureMonitor. |
| `name` | String | Identifier. Name of the FeatureMonitor. Format: `projects/{project}/locations/{location}/featureGroups/{featureGroup}/featureMonitors/{featureMonitor}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature_monitor
feature_monitor = provider.aiplatform_api.Feature_monitor {
    parent = "value"  # Required. The resource name of FeatureGroup to create FeatureMonitor. Format: `projects/{project}/locations/{location}/featureGroups/{featuregroup}`
}

# Access feature_monitor outputs
feature_monitor_id = feature_monitor.id
feature_monitor_create_time = feature_monitor.create_time
feature_monitor_etag = feature_monitor.etag
feature_monitor_labels = feature_monitor.labels
feature_monitor_schedule_config = feature_monitor.schedule_config
feature_monitor_feature_selection_config = feature_monitor.feature_selection_config
feature_monitor_update_time = feature_monitor.update_time
feature_monitor_description = feature_monitor.description
feature_monitor_name = feature_monitor.name
```

---


### Pipeline_job

Creates a PipelineJob. A PipelineJob will run immediately when created.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this PipelineJob was most recently updated. |
| `network` | String |  | The full name of the Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to which the Pipeline Job's workload should be peered. For example, `projects/12345/global/networks/myVPC`. [Format](/compute/docs/reference/rest/v1/networks/insert) is of the form `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in `12345`, and {network} is a network name. Private services access must already be configured for the network. Pipeline job will apply the network configuration to the Google Cloud resources being launched, if applied, such as Vertex AI Training or Dataflow job. If left unspecified, the workload is not peered with any network. |
| `job_detail` | String |  | Output only. The details of pipeline run. Not available in the list view. |
| `reserved_ip_ranges` | Vec<String> |  | A list of names for the reserved ip ranges under the VPC network that can be used for this Pipeline Job's workload. If set, we will deploy the Pipeline Job's workload within the provided ip ranges. Otherwise, the job will be deployed to any ip ranges under the provided VPC network. Example: ['vertex-ai-ip-range']. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `schedule_name` | String |  | Output only. The schedule resource name. Only returned if the Pipeline is created by Schedule API. |
| `name` | String |  | Output only. The resource name of the PipelineJob. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize PipelineJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. Note there is some reserved label key for Vertex AI Pipelines. - `vertex-ai-pipelines-run-billing-id`, user set value will get overrided. |
| `preflight_validations` | bool |  | Optional. Whether to do component level validations before job creation. |
| `template_uri` | String |  | A template uri from where the PipelineJob.pipeline_spec, if empty, will be downloaded. Currently, only uri from Vertex Template Registry & Gallery is supported. Reference to https://cloud.google.com/vertex-ai/docs/pipelines/create-pipeline-template. |
| `original_pipeline_job_id` | String |  | Optional. The original pipeline job id if this pipeline job is a rerun of a previous pipeline job. |
| `pipeline_spec` | HashMap<String, String> |  | The spec of the pipeline. |
| `psc_interface_config` | String |  | Optional. Configuration for PSC-I for PipelineJob. |
| `create_time` | String |  | Output only. Pipeline creation time. |
| `end_time` | String |  | Output only. Pipeline end time. |
| `error` | String |  | Output only. The error that occurred during pipeline execution. Only populated when the pipeline's state is FAILED or CANCELLED. |
| `service_account` | String |  | The service account that the pipeline workload runs as. If not specified, the Compute Engine default service account in the project will be used. See https://cloud.google.com/compute/docs/access/service-accounts#default_service_account Users starting the pipeline must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `template_metadata` | String |  | Output only. Pipeline template metadata. Will fill up fields if PipelineJob.template_uri is from supported template registry. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a pipelineJob. If set, this PipelineJob and all of its sub-resources will be secured by this key. |
| `display_name` | String |  | The display name of the Pipeline. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `pipeline_task_rerun_configs` | Vec<String> |  | Optional. The rerun configs for each task in the pipeline job. By default, the rerun will: 1. Use the same input artifacts as the original run. 2. Use the same input parameters as the original run. 3. Skip all the tasks that are already succeeded in the original run. 4. Rerun all the tasks that are not succeeded in the original run. By providing this field, users can override the default behavior and specify the rerun config for each task. |
| `start_time` | String |  | Output only. Pipeline start time. |
| `runtime_config` | String |  | Runtime config of the pipeline. |
| `state` | String |  | Output only. The detailed state of the job. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the PipelineJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this PipelineJob was most recently updated. |
| `network` | String | The full name of the Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to which the Pipeline Job's workload should be peered. For example, `projects/12345/global/networks/myVPC`. [Format](/compute/docs/reference/rest/v1/networks/insert) is of the form `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in `12345`, and {network} is a network name. Private services access must already be configured for the network. Pipeline job will apply the network configuration to the Google Cloud resources being launched, if applied, such as Vertex AI Training or Dataflow job. If left unspecified, the workload is not peered with any network. |
| `job_detail` | String | Output only. The details of pipeline run. Not available in the list view. |
| `reserved_ip_ranges` | Vec<String> | A list of names for the reserved ip ranges under the VPC network that can be used for this Pipeline Job's workload. If set, we will deploy the Pipeline Job's workload within the provided ip ranges. Otherwise, the job will be deployed to any ip ranges under the provided VPC network. Example: ['vertex-ai-ip-range']. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `schedule_name` | String | Output only. The schedule resource name. Only returned if the Pipeline is created by Schedule API. |
| `name` | String | Output only. The resource name of the PipelineJob. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize PipelineJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. Note there is some reserved label key for Vertex AI Pipelines. - `vertex-ai-pipelines-run-billing-id`, user set value will get overrided. |
| `preflight_validations` | bool | Optional. Whether to do component level validations before job creation. |
| `template_uri` | String | A template uri from where the PipelineJob.pipeline_spec, if empty, will be downloaded. Currently, only uri from Vertex Template Registry & Gallery is supported. Reference to https://cloud.google.com/vertex-ai/docs/pipelines/create-pipeline-template. |
| `original_pipeline_job_id` | String | Optional. The original pipeline job id if this pipeline job is a rerun of a previous pipeline job. |
| `pipeline_spec` | HashMap<String, String> | The spec of the pipeline. |
| `psc_interface_config` | String | Optional. Configuration for PSC-I for PipelineJob. |
| `create_time` | String | Output only. Pipeline creation time. |
| `end_time` | String | Output only. Pipeline end time. |
| `error` | String | Output only. The error that occurred during pipeline execution. Only populated when the pipeline's state is FAILED or CANCELLED. |
| `service_account` | String | The service account that the pipeline workload runs as. If not specified, the Compute Engine default service account in the project will be used. See https://cloud.google.com/compute/docs/access/service-accounts#default_service_account Users starting the pipeline must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `template_metadata` | String | Output only. Pipeline template metadata. Will fill up fields if PipelineJob.template_uri is from supported template registry. |
| `encryption_spec` | String | Customer-managed encryption key spec for a pipelineJob. If set, this PipelineJob and all of its sub-resources will be secured by this key. |
| `display_name` | String | The display name of the Pipeline. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `pipeline_task_rerun_configs` | Vec<String> | Optional. The rerun configs for each task in the pipeline job. By default, the rerun will: 1. Use the same input artifacts as the original run. 2. Use the same input parameters as the original run. 3. Skip all the tasks that are already succeeded in the original run. 4. Rerun all the tasks that are not succeeded in the original run. By providing this field, users can override the default behavior and specify the rerun config for each task. |
| `start_time` | String | Output only. Pipeline start time. |
| `runtime_config` | String | Runtime config of the pipeline. |
| `state` | String | Output only. The detailed state of the job. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create pipeline_job
pipeline_job = provider.aiplatform_api.Pipeline_job {
    parent = "value"  # Required. The resource name of the Location to create the PipelineJob in. Format: `projects/{project}/locations/{location}`
}

# Access pipeline_job outputs
pipeline_job_id = pipeline_job.id
pipeline_job_satisfies_pzi = pipeline_job.satisfies_pzi
pipeline_job_update_time = pipeline_job.update_time
pipeline_job_network = pipeline_job.network
pipeline_job_job_detail = pipeline_job.job_detail
pipeline_job_reserved_ip_ranges = pipeline_job.reserved_ip_ranges
pipeline_job_satisfies_pzs = pipeline_job.satisfies_pzs
pipeline_job_schedule_name = pipeline_job.schedule_name
pipeline_job_name = pipeline_job.name
pipeline_job_labels = pipeline_job.labels
pipeline_job_preflight_validations = pipeline_job.preflight_validations
pipeline_job_template_uri = pipeline_job.template_uri
pipeline_job_original_pipeline_job_id = pipeline_job.original_pipeline_job_id
pipeline_job_pipeline_spec = pipeline_job.pipeline_spec
pipeline_job_psc_interface_config = pipeline_job.psc_interface_config
pipeline_job_create_time = pipeline_job.create_time
pipeline_job_end_time = pipeline_job.end_time
pipeline_job_error = pipeline_job.error
pipeline_job_service_account = pipeline_job.service_account
pipeline_job_template_metadata = pipeline_job.template_metadata
pipeline_job_encryption_spec = pipeline_job.encryption_spec
pipeline_job_display_name = pipeline_job.display_name
pipeline_job_pipeline_task_rerun_configs = pipeline_job.pipeline_task_rerun_configs
pipeline_job_start_time = pipeline_job.start_time
pipeline_job_runtime_config = pipeline_job.runtime_config
pipeline_job_state = pipeline_job.state
```

---


### Feature_view

Creates a new FeatureView in a given FeatureOnlineStore.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `index_config` | String |  | Optional. Configuration for index preparation for vector search. It contains the required configurations to create an index from source data, so that approximate nearest neighbor (a.k.a ANN) algorithms search can be performed during online serving. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your FeatureViews. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureOnlineStore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `vertex_rag_source` | String |  | Optional. The Vertex RAG Source that the FeatureView is linked to. |
| `optimized_config` | String |  | Optional. Configuration for FeatureView created under Optimized FeatureOnlineStore. |
| `big_query_source` | String |  | Optional. Configures how data is supposed to be extracted from a BigQuery source to be loaded onto the FeatureOnlineStore. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `sync_config` | String |  | Configures when data is to be synced/updated for this FeatureView. At the end of the sync the latest featureValues for each entityId of this FeatureView are made ready for online serving. |
| `feature_registry_source` | String |  | Optional. Configures the features from a Feature Registry source that need to be loaded onto the FeatureOnlineStore. |
| `service_account_email` | String |  | Output only. A Service Account unique to this FeatureView. The role bigquery.dataViewer should be granted to this service account to allow Vertex AI Feature Store to sync data to the online store. |
| `create_time` | String |  | Output only. Timestamp when this FeatureView was created. |
| `name` | String |  | Identifier. Name of the FeatureView. Format: `projects/{project}/locations/{location}/featureOnlineStores/{feature_online_store}/featureViews/{feature_view}` |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `vector_search_config` | String |  | Optional. Deprecated: please use FeatureView.index_config instead. |
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `update_time` | String |  | Output only. Timestamp when this FeatureView was last updated. |
| `service_agent_type` | String |  | Optional. Service agent type used during data sync. By default, the Vertex AI Service Agent is used. When using an IAM Policy to isolate this FeatureView within a project, a separate service account should be provisioned by setting this field to `SERVICE_AGENT_TYPE_FEATURE_VIEW`. This will generate a separate service account to access the BigQuery source table. |
| `parent` | String | ✅ | Required. The resource name of the FeatureOnlineStore to create FeatureViews. Format: `projects/{project}/locations/{location}/featureOnlineStores/{feature_online_store}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `index_config` | String | Optional. Configuration for index preparation for vector search. It contains the required configurations to create an index from source data, so that approximate nearest neighbor (a.k.a ANN) algorithms search can be performed during online serving. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your FeatureViews. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureOnlineStore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `vertex_rag_source` | String | Optional. The Vertex RAG Source that the FeatureView is linked to. |
| `optimized_config` | String | Optional. Configuration for FeatureView created under Optimized FeatureOnlineStore. |
| `big_query_source` | String | Optional. Configures how data is supposed to be extracted from a BigQuery source to be loaded onto the FeatureOnlineStore. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `sync_config` | String | Configures when data is to be synced/updated for this FeatureView. At the end of the sync the latest featureValues for each entityId of this FeatureView are made ready for online serving. |
| `feature_registry_source` | String | Optional. Configures the features from a Feature Registry source that need to be loaded onto the FeatureOnlineStore. |
| `service_account_email` | String | Output only. A Service Account unique to this FeatureView. The role bigquery.dataViewer should be granted to this service account to allow Vertex AI Feature Store to sync data to the online store. |
| `create_time` | String | Output only. Timestamp when this FeatureView was created. |
| `name` | String | Identifier. Name of the FeatureView. Format: `projects/{project}/locations/{location}/featureOnlineStores/{feature_online_store}/featureViews/{feature_view}` |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `vector_search_config` | String | Optional. Deprecated: please use FeatureView.index_config instead. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `update_time` | String | Output only. Timestamp when this FeatureView was last updated. |
| `service_agent_type` | String | Optional. Service agent type used during data sync. By default, the Vertex AI Service Agent is used. When using an IAM Policy to isolate this FeatureView within a project, a separate service account should be provisioned by setting this field to `SERVICE_AGENT_TYPE_FEATURE_VIEW`. This will generate a separate service account to access the BigQuery source table. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature_view
feature_view = provider.aiplatform_api.Feature_view {
    parent = "value"  # Required. The resource name of the FeatureOnlineStore to create FeatureViews. Format: `projects/{project}/locations/{location}/featureOnlineStores/{feature_online_store}`
}

# Access feature_view outputs
feature_view_id = feature_view.id
feature_view_index_config = feature_view.index_config
feature_view_labels = feature_view.labels
feature_view_vertex_rag_source = feature_view.vertex_rag_source
feature_view_optimized_config = feature_view.optimized_config
feature_view_big_query_source = feature_view.big_query_source
feature_view_satisfies_pzi = feature_view.satisfies_pzi
feature_view_sync_config = feature_view.sync_config
feature_view_feature_registry_source = feature_view.feature_registry_source
feature_view_service_account_email = feature_view.service_account_email
feature_view_create_time = feature_view.create_time
feature_view_name = feature_view.name
feature_view_satisfies_pzs = feature_view.satisfies_pzs
feature_view_vector_search_config = feature_view.vector_search_config
feature_view_etag = feature_view.etag
feature_view_update_time = feature_view.update_time
feature_view_service_agent_type = feature_view.service_agent_type
```

---


### Sandbox_environment

Creates a SandboxEnvironment in a given reasoning engine.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when this SandboxEnvironment was created. |
| `display_name` | String |  | Required. The display name of the SandboxEnvironment. |
| `spec` | String |  | Optional. The configuration of the SandboxEnvironment. |
| `state` | String |  | Output only. The runtime state of the SandboxEnvironment. |
| `update_time` | String |  | Output only. The timestamp when this SandboxEnvironment was most recently updated. |
| `name` | String |  | Identifier. The name of the SandboxEnvironment. |
| `parent` | String | ✅ | Required. The resource name of the reasoning engine to create the SandboxEnvironment in. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when this SandboxEnvironment was created. |
| `display_name` | String | Required. The display name of the SandboxEnvironment. |
| `spec` | String | Optional. The configuration of the SandboxEnvironment. |
| `state` | String | Output only. The runtime state of the SandboxEnvironment. |
| `update_time` | String | Output only. The timestamp when this SandboxEnvironment was most recently updated. |
| `name` | String | Identifier. The name of the SandboxEnvironment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sandbox_environment
sandbox_environment = provider.aiplatform_api.Sandbox_environment {
    parent = "value"  # Required. The resource name of the reasoning engine to create the SandboxEnvironment in. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}`.
}

# Access sandbox_environment outputs
sandbox_environment_id = sandbox_environment.id
sandbox_environment_create_time = sandbox_environment.create_time
sandbox_environment_display_name = sandbox_environment.display_name
sandbox_environment_spec = sandbox_environment.spec
sandbox_environment_state = sandbox_environment.state
sandbox_environment_update_time = sandbox_environment.update_time
sandbox_environment_name = sandbox_environment.name
```

---


### Time_serie

Creates a TensorboardTimeSeries.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metadata` | String |  | Output only. Scalar, Tensor, or Blob metadata for this TensorboardTimeSeries. |
| `value_type` | String |  | Required. Immutable. Type of TensorboardTimeSeries value. |
| `name` | String |  | Output only. Name of the TensorboardTimeSeries. |
| `create_time` | String |  | Output only. Timestamp when this TensorboardTimeSeries was created. |
| `etag` | String |  | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `description` | String |  | Description of this TensorboardTimeSeries. |
| `plugin_data` | String |  | Data of the current plugin, with the size limited to 65KB. |
| `update_time` | String |  | Output only. Timestamp when this TensorboardTimeSeries was last updated. |
| `plugin_name` | String |  | Immutable. Name of the plugin this time series pertain to. Such as Scalar, Tensor, Blob |
| `display_name` | String |  | Required. User provided name of this TensorboardTimeSeries. This value should be unique among all TensorboardTimeSeries resources belonging to the same TensorboardRun resource (parent resource). |
| `parent` | String | ✅ | Required. The resource name of the TensorboardRun to create the TensorboardTimeSeries in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}/runs/{run}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | Output only. Scalar, Tensor, or Blob metadata for this TensorboardTimeSeries. |
| `value_type` | String | Required. Immutable. Type of TensorboardTimeSeries value. |
| `name` | String | Output only. Name of the TensorboardTimeSeries. |
| `create_time` | String | Output only. Timestamp when this TensorboardTimeSeries was created. |
| `etag` | String | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `description` | String | Description of this TensorboardTimeSeries. |
| `plugin_data` | String | Data of the current plugin, with the size limited to 65KB. |
| `update_time` | String | Output only. Timestamp when this TensorboardTimeSeries was last updated. |
| `plugin_name` | String | Immutable. Name of the plugin this time series pertain to. Such as Scalar, Tensor, Blob |
| `display_name` | String | Required. User provided name of this TensorboardTimeSeries. This value should be unique among all TensorboardTimeSeries resources belonging to the same TensorboardRun resource (parent resource). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create time_serie
time_serie = provider.aiplatform_api.Time_serie {
    parent = "value"  # Required. The resource name of the TensorboardRun to create the TensorboardTimeSeries in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}/runs/{run}`
}

# Access time_serie outputs
time_serie_id = time_serie.id
time_serie_metadata = time_serie.metadata
time_serie_value_type = time_serie.value_type
time_serie_name = time_serie.name
time_serie_create_time = time_serie.create_time
time_serie_etag = time_serie.etag
time_serie_description = time_serie.description
time_serie_plugin_data = time_serie.plugin_data
time_serie_update_time = time_serie.update_time
time_serie_plugin_name = time_serie.plugin_name
time_serie_display_name = time_serie.display_name
```

---


### Evaluation_item

Creates an Evaluation Item.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The display name of the EvaluationItem. |
| `evaluation_response` | String |  | Output only. The response from evaluation. |
| `evaluation_request` | String |  | The request to evaluate. |
| `labels` | HashMap<String, String> |  | Optional. Labels for the EvaluationItem. |
| `name` | String |  | Identifier. The resource name of the EvaluationItem. Format: `projects/{project}/locations/{location}/evaluationItems/{evaluation_item}` |
| `error` | String |  | Output only. Error for the evaluation item. |
| `evaluation_item_type` | String |  | Required. The type of the EvaluationItem. |
| `gcs_uri` | String |  | The GCS object where the request or response is stored. |
| `metadata` | String |  | Optional. Metadata for the EvaluationItem. |
| `create_time` | String |  | Output only. Timestamp when this item was created. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Evaluation Item in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the EvaluationItem. |
| `evaluation_response` | String | Output only. The response from evaluation. |
| `evaluation_request` | String | The request to evaluate. |
| `labels` | HashMap<String, String> | Optional. Labels for the EvaluationItem. |
| `name` | String | Identifier. The resource name of the EvaluationItem. Format: `projects/{project}/locations/{location}/evaluationItems/{evaluation_item}` |
| `error` | String | Output only. Error for the evaluation item. |
| `evaluation_item_type` | String | Required. The type of the EvaluationItem. |
| `gcs_uri` | String | The GCS object where the request or response is stored. |
| `metadata` | String | Optional. Metadata for the EvaluationItem. |
| `create_time` | String | Output only. Timestamp when this item was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation_item
evaluation_item = provider.aiplatform_api.Evaluation_item {
    parent = "value"  # Required. The resource name of the Location to create the Evaluation Item in. Format: `projects/{project}/locations/{location}`
}

# Access evaluation_item outputs
evaluation_item_id = evaluation_item.id
evaluation_item_display_name = evaluation_item.display_name
evaluation_item_evaluation_response = evaluation_item.evaluation_response
evaluation_item_evaluation_request = evaluation_item.evaluation_request
evaluation_item_labels = evaluation_item.labels
evaluation_item_name = evaluation_item.name
evaluation_item_error = evaluation_item.error
evaluation_item_evaluation_item_type = evaluation_item.evaluation_item_type
evaluation_item_gcs_uri = evaluation_item.gcs_uri
evaluation_item_metadata = evaluation_item.metadata
evaluation_item_create_time = evaluation_item.create_time
```

---


### Media

Upload a file into a RagCorpus.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rag_file` | String |  | Required. The RagFile to upload. |
| `upload_rag_file_config` | String |  | Required. The config for the RagFiles to be uploaded into the RagCorpus. VertexRagDataService.UploadRagFile. |
| `parent` | String | ✅ | Required. The name of the RagCorpus resource into which to upload the file. Format: `projects/{project}/locations/{location}/ragCorpora/{rag_corpus}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.aiplatform_api.Media {
    parent = "value"  # Required. The name of the RagCorpus resource into which to upload the file. Format: `projects/{project}/locations/{location}/ragCorpora/{rag_corpus}`
}

```

---


### Model_deployment_monitoring_job

Creates a ModelDeploymentMonitoringJob. It will run periodically on a configured interval.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bigquery_tables` | Vec<String> |  | Output only. The created bigquery tables for the job under customer project. Customer could do their own query & analysis. There could be 4 log tables in maximum: 1. Training data logging predict request/response 2. Serving data logging predict request/response |
| `log_ttl` | String |  | The TTL of BigQuery tables in user projects which stores logs. A day is the basic unit of the TTL and we take the ceil of TTL/86400(a day). e.g. { second: 3600} indicates ttl = 1 day. |
| `name` | String |  | Output only. Resource name of a ModelDeploymentMonitoringJob. |
| `next_schedule_time` | String |  | Output only. Timestamp when this monitoring pipeline will be scheduled to run for the next round. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your ModelDeploymentMonitoringJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `error` | String |  | Output only. Only populated when the job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `sample_predict_instance` | String |  | Sample Predict instance, same format as PredictRequest.instances, this can be set as a replacement of ModelDeploymentMonitoringJob.predict_instance_schema_uri. If not set, we will generate predict schema from collected predict requests. |
| `enable_monitoring_pipeline_logs` | bool |  | If true, the scheduled monitoring pipeline logs are sent to Google Cloud Logging, including pipeline status and anomalies detected. Please note the logs incur cost, which are subject to [Cloud Logging pricing](https://cloud.google.com/logging#pricing). |
| `stats_anomalies_base_directory` | String |  | Stats anomalies base folder path. |
| `analysis_instance_schema_uri` | String |  | YAML schema file uri describing the format of a single instance that you want Tensorflow Data Validation (TFDV) to analyze. If this field is empty, all the feature data types are inferred from predict_instance_schema_uri, meaning that TFDV will use the data in the exact format(data type) as prediction request/response. If there are any data type differences between predict instance and TFDV instance, this field can be used to override the schema. For models trained with Vertex AI, this field must be set as all the fields in predict instance formatted as string. |
| `model_deployment_monitoring_schedule_config` | String |  | Required. Schedule config for running the monitoring job. |
| `schedule_state` | String |  | Output only. Schedule state when the monitoring job is in Running state. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a ModelDeploymentMonitoringJob. If set, this ModelDeploymentMonitoringJob and all sub-resources of this ModelDeploymentMonitoringJob will be secured by this key. |
| `latest_monitoring_pipeline_metadata` | String |  | Output only. Latest triggered monitoring pipeline metadata. |
| `endpoint` | String |  | Required. Endpoint resource name. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}` |
| `display_name` | String |  | Required. The user-defined name of the ModelDeploymentMonitoringJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. Display name of a ModelDeploymentMonitoringJob. |
| `logging_sampling_strategy` | String |  | Required. Sample Strategy for logging. |
| `model_deployment_monitoring_objective_configs` | Vec<String> |  | Required. The config for monitoring objectives. This is a per DeployedModel config. Each DeployedModel needs to be configured separately. |
| `model_monitoring_alert_config` | String |  | Alert config for model monitoring. |
| `predict_instance_schema_uri` | String |  | YAML schema file uri describing the format of a single instance, which are given to format this Endpoint's prediction (and explanation). If not set, we will generate predict schema from collected predict requests. |
| `create_time` | String |  | Output only. Timestamp when this ModelDeploymentMonitoringJob was created. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `state` | String |  | Output only. The detailed state of the monitoring job. When the job is still creating, the state will be 'PENDING'. Once the job is successfully created, the state will be 'RUNNING'. Pause the job, the state will be 'PAUSED'. Resume the job, the state will return to 'RUNNING'. |
| `update_time` | String |  | Output only. Timestamp when this ModelDeploymentMonitoringJob was updated most recently. |
| `parent` | String | ✅ | Required. The parent of the ModelDeploymentMonitoringJob. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bigquery_tables` | Vec<String> | Output only. The created bigquery tables for the job under customer project. Customer could do their own query & analysis. There could be 4 log tables in maximum: 1. Training data logging predict request/response 2. Serving data logging predict request/response |
| `log_ttl` | String | The TTL of BigQuery tables in user projects which stores logs. A day is the basic unit of the TTL and we take the ceil of TTL/86400(a day). e.g. { second: 3600} indicates ttl = 1 day. |
| `name` | String | Output only. Resource name of a ModelDeploymentMonitoringJob. |
| `next_schedule_time` | String | Output only. Timestamp when this monitoring pipeline will be scheduled to run for the next round. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your ModelDeploymentMonitoringJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `error` | String | Output only. Only populated when the job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `sample_predict_instance` | String | Sample Predict instance, same format as PredictRequest.instances, this can be set as a replacement of ModelDeploymentMonitoringJob.predict_instance_schema_uri. If not set, we will generate predict schema from collected predict requests. |
| `enable_monitoring_pipeline_logs` | bool | If true, the scheduled monitoring pipeline logs are sent to Google Cloud Logging, including pipeline status and anomalies detected. Please note the logs incur cost, which are subject to [Cloud Logging pricing](https://cloud.google.com/logging#pricing). |
| `stats_anomalies_base_directory` | String | Stats anomalies base folder path. |
| `analysis_instance_schema_uri` | String | YAML schema file uri describing the format of a single instance that you want Tensorflow Data Validation (TFDV) to analyze. If this field is empty, all the feature data types are inferred from predict_instance_schema_uri, meaning that TFDV will use the data in the exact format(data type) as prediction request/response. If there are any data type differences between predict instance and TFDV instance, this field can be used to override the schema. For models trained with Vertex AI, this field must be set as all the fields in predict instance formatted as string. |
| `model_deployment_monitoring_schedule_config` | String | Required. Schedule config for running the monitoring job. |
| `schedule_state` | String | Output only. Schedule state when the monitoring job is in Running state. |
| `encryption_spec` | String | Customer-managed encryption key spec for a ModelDeploymentMonitoringJob. If set, this ModelDeploymentMonitoringJob and all sub-resources of this ModelDeploymentMonitoringJob will be secured by this key. |
| `latest_monitoring_pipeline_metadata` | String | Output only. Latest triggered monitoring pipeline metadata. |
| `endpoint` | String | Required. Endpoint resource name. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}` |
| `display_name` | String | Required. The user-defined name of the ModelDeploymentMonitoringJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. Display name of a ModelDeploymentMonitoringJob. |
| `logging_sampling_strategy` | String | Required. Sample Strategy for logging. |
| `model_deployment_monitoring_objective_configs` | Vec<String> | Required. The config for monitoring objectives. This is a per DeployedModel config. Each DeployedModel needs to be configured separately. |
| `model_monitoring_alert_config` | String | Alert config for model monitoring. |
| `predict_instance_schema_uri` | String | YAML schema file uri describing the format of a single instance, which are given to format this Endpoint's prediction (and explanation). If not set, we will generate predict schema from collected predict requests. |
| `create_time` | String | Output only. Timestamp when this ModelDeploymentMonitoringJob was created. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `state` | String | Output only. The detailed state of the monitoring job. When the job is still creating, the state will be 'PENDING'. Once the job is successfully created, the state will be 'RUNNING'. Pause the job, the state will be 'PAUSED'. Resume the job, the state will return to 'RUNNING'. |
| `update_time` | String | Output only. Timestamp when this ModelDeploymentMonitoringJob was updated most recently. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model_deployment_monitoring_job
model_deployment_monitoring_job = provider.aiplatform_api.Model_deployment_monitoring_job {
    parent = "value"  # Required. The parent of the ModelDeploymentMonitoringJob. Format: `projects/{project}/locations/{location}`
}

# Access model_deployment_monitoring_job outputs
model_deployment_monitoring_job_id = model_deployment_monitoring_job.id
model_deployment_monitoring_job_bigquery_tables = model_deployment_monitoring_job.bigquery_tables
model_deployment_monitoring_job_log_ttl = model_deployment_monitoring_job.log_ttl
model_deployment_monitoring_job_name = model_deployment_monitoring_job.name
model_deployment_monitoring_job_next_schedule_time = model_deployment_monitoring_job.next_schedule_time
model_deployment_monitoring_job_labels = model_deployment_monitoring_job.labels
model_deployment_monitoring_job_error = model_deployment_monitoring_job.error
model_deployment_monitoring_job_sample_predict_instance = model_deployment_monitoring_job.sample_predict_instance
model_deployment_monitoring_job_enable_monitoring_pipeline_logs = model_deployment_monitoring_job.enable_monitoring_pipeline_logs
model_deployment_monitoring_job_stats_anomalies_base_directory = model_deployment_monitoring_job.stats_anomalies_base_directory
model_deployment_monitoring_job_analysis_instance_schema_uri = model_deployment_monitoring_job.analysis_instance_schema_uri
model_deployment_monitoring_job_model_deployment_monitoring_schedule_config = model_deployment_monitoring_job.model_deployment_monitoring_schedule_config
model_deployment_monitoring_job_schedule_state = model_deployment_monitoring_job.schedule_state
model_deployment_monitoring_job_encryption_spec = model_deployment_monitoring_job.encryption_spec
model_deployment_monitoring_job_latest_monitoring_pipeline_metadata = model_deployment_monitoring_job.latest_monitoring_pipeline_metadata
model_deployment_monitoring_job_endpoint = model_deployment_monitoring_job.endpoint
model_deployment_monitoring_job_display_name = model_deployment_monitoring_job.display_name
model_deployment_monitoring_job_logging_sampling_strategy = model_deployment_monitoring_job.logging_sampling_strategy
model_deployment_monitoring_job_model_deployment_monitoring_objective_configs = model_deployment_monitoring_job.model_deployment_monitoring_objective_configs
model_deployment_monitoring_job_model_monitoring_alert_config = model_deployment_monitoring_job.model_monitoring_alert_config
model_deployment_monitoring_job_predict_instance_schema_uri = model_deployment_monitoring_job.predict_instance_schema_uri
model_deployment_monitoring_job_create_time = model_deployment_monitoring_job.create_time
model_deployment_monitoring_job_satisfies_pzs = model_deployment_monitoring_job.satisfies_pzs
model_deployment_monitoring_job_satisfies_pzi = model_deployment_monitoring_job.satisfies_pzi
model_deployment_monitoring_job_state = model_deployment_monitoring_job.state
model_deployment_monitoring_job_update_time = model_deployment_monitoring_job.update_time
```

---


### Dataset_version

Create a version from a Dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The user-defined name of the DatasetVersion. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `create_time` | String |  | Output only. Timestamp when this DatasetVersion was created. |
| `big_query_dataset_name` | String |  | Output only. Name of the associated BigQuery dataset. |
| `metadata` | String |  | Required. Output only. Additional information about the DatasetVersion. |
| `name` | String |  | Output only. Identifier. The resource name of the DatasetVersion. Format: `projects/{project}/locations/{location}/datasets/{dataset}/datasetVersions/{dataset_version}` |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this DatasetVersion was last updated. |
| `model_reference` | String |  | Output only. Reference to the public base model last used by the dataset version. Only set for prompt dataset versions. |
| `parent` | String | ✅ | Required. The name of the Dataset resource. Format: `projects/{project}/locations/{location}/datasets/{dataset}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The user-defined name of the DatasetVersion. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `create_time` | String | Output only. Timestamp when this DatasetVersion was created. |
| `big_query_dataset_name` | String | Output only. Name of the associated BigQuery dataset. |
| `metadata` | String | Required. Output only. Additional information about the DatasetVersion. |
| `name` | String | Output only. Identifier. The resource name of the DatasetVersion. Format: `projects/{project}/locations/{location}/datasets/{dataset}/datasetVersions/{dataset_version}` |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this DatasetVersion was last updated. |
| `model_reference` | String | Output only. Reference to the public base model last used by the dataset version. Only set for prompt dataset versions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset_version
dataset_version = provider.aiplatform_api.Dataset_version {
    parent = "value"  # Required. The name of the Dataset resource. Format: `projects/{project}/locations/{location}/datasets/{dataset}`
}

# Access dataset_version outputs
dataset_version_id = dataset_version.id
dataset_version_display_name = dataset_version.display_name
dataset_version_create_time = dataset_version.create_time
dataset_version_big_query_dataset_name = dataset_version.big_query_dataset_name
dataset_version_metadata = dataset_version.metadata
dataset_version_name = dataset_version.name
dataset_version_etag = dataset_version.etag
dataset_version_satisfies_pzi = dataset_version.satisfies_pzi
dataset_version_satisfies_pzs = dataset_version.satisfies_pzs
dataset_version_update_time = dataset_version.update_time
dataset_version_model_reference = dataset_version.model_reference
```

---


### Evaluation_set

Creates an Evaluation Set.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `evaluation_items` | Vec<String> |  | Required. The EvaluationItems that are part of this dataset. |
| `display_name` | String |  | Required. The display name of the EvaluationSet. |
| `name` | String |  | Identifier. The resource name of the EvaluationSet. Format: `projects/{project}/locations/{location}/evaluationSets/{evaluation_set}` |
| `update_time` | String |  | Output only. Timestamp when this item was last updated. |
| `create_time` | String |  | Output only. Timestamp when this item was created. |
| `metadata` | String |  | Optional. Metadata for the EvaluationSet. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Evaluation Set in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evaluation_items` | Vec<String> | Required. The EvaluationItems that are part of this dataset. |
| `display_name` | String | Required. The display name of the EvaluationSet. |
| `name` | String | Identifier. The resource name of the EvaluationSet. Format: `projects/{project}/locations/{location}/evaluationSets/{evaluation_set}` |
| `update_time` | String | Output only. Timestamp when this item was last updated. |
| `create_time` | String | Output only. Timestamp when this item was created. |
| `metadata` | String | Optional. Metadata for the EvaluationSet. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation_set
evaluation_set = provider.aiplatform_api.Evaluation_set {
    parent = "value"  # Required. The resource name of the Location to create the Evaluation Set in. Format: `projects/{project}/locations/{location}`
}

# Access evaluation_set outputs
evaluation_set_id = evaluation_set.id
evaluation_set_evaluation_items = evaluation_set.evaluation_items
evaluation_set_display_name = evaluation_set.display_name
evaluation_set_name = evaluation_set.name
evaluation_set_update_time = evaluation_set.update_time
evaluation_set_create_time = evaluation_set.create_time
evaluation_set_metadata = evaluation_set.metadata
```

---


### Chat

Exposes an OpenAI-compatible endpoint for chat completions.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extensions` | Vec<HashMap<String, String>> |  | Application specific response metadata. Must be set in the first response for streaming APIs. |
| `content_type` | String |  | The HTTP Content-Type header value specifying the content type of the body. |
| `data` | String |  | The HTTP request/response body as raw binary. |
| `endpoint` | String | ✅ | Required. The name of the endpoint requested to serve the prediction. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create chat
chat = provider.aiplatform_api.Chat {
    endpoint = "value"  # Required. The name of the endpoint requested to serve the prediction. Format: `projects/{project}/locations/{location}/endpoints/{endpoint}`
}

```

---


### Reasoning_engine

Creates a reasoning engine.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Timestamp when this ReasoningEngine was most recently updated. |
| `context_spec` | String |  | Optional. Configuration for how Agent Engine sub-resources should manage context. |
| `display_name` | String |  | Required. The display name of the ReasoningEngine. |
| `create_time` | String |  | Output only. Timestamp when this ReasoningEngine was created. |
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a ReasoningEngine. If set, this ReasoningEngine and all sub-resources of this ReasoningEngine will be secured by this key. |
| `labels` | HashMap<String, String> |  | Labels for the ReasoningEngine. |
| `name` | String |  | Identifier. The resource name of the ReasoningEngine. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}` |
| `description` | String |  | Optional. The description of the ReasoningEngine. |
| `spec` | String |  | Optional. Configurations of the ReasoningEngine |
| `parent` | String | ✅ | Required. The resource name of the Location to create the ReasoningEngine in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Timestamp when this ReasoningEngine was most recently updated. |
| `context_spec` | String | Optional. Configuration for how Agent Engine sub-resources should manage context. |
| `display_name` | String | Required. The display name of the ReasoningEngine. |
| `create_time` | String | Output only. Timestamp when this ReasoningEngine was created. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `encryption_spec` | String | Customer-managed encryption key spec for a ReasoningEngine. If set, this ReasoningEngine and all sub-resources of this ReasoningEngine will be secured by this key. |
| `labels` | HashMap<String, String> | Labels for the ReasoningEngine. |
| `name` | String | Identifier. The resource name of the ReasoningEngine. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}` |
| `description` | String | Optional. The description of the ReasoningEngine. |
| `spec` | String | Optional. Configurations of the ReasoningEngine |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reasoning_engine
reasoning_engine = provider.aiplatform_api.Reasoning_engine {
    parent = "value"  # Required. The resource name of the Location to create the ReasoningEngine in. Format: `projects/{project}/locations/{location}`
}

# Access reasoning_engine outputs
reasoning_engine_id = reasoning_engine.id
reasoning_engine_update_time = reasoning_engine.update_time
reasoning_engine_context_spec = reasoning_engine.context_spec
reasoning_engine_display_name = reasoning_engine.display_name
reasoning_engine_create_time = reasoning_engine.create_time
reasoning_engine_etag = reasoning_engine.etag
reasoning_engine_encryption_spec = reasoning_engine.encryption_spec
reasoning_engine_labels = reasoning_engine.labels
reasoning_engine_name = reasoning_engine.name
reasoning_engine_description = reasoning_engine.description
reasoning_engine_spec = reasoning_engine.spec
```

---


### Migratable_resource

Batch migrates resources from ml.googleapis.com, automl.googleapis.com, and datalabeling.googleapis.com to Vertex AI.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `migrate_resource_requests` | Vec<String> |  | Required. The request messages specifying the resources to migrate. They must be in the same location as the destination. Up to 50 resources can be migrated in one batch. |
| `parent` | String | ✅ | Required. The location of the migrated resource will live in. Format: `projects/{project}/locations/{location}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create migratable_resource
migratable_resource = provider.aiplatform_api.Migratable_resource {
    parent = "value"  # Required. The location of the migrated resource will live in. Format: `projects/{project}/locations/{location}`
}

```

---


### Schedule

Creates a Schedule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_model_monitoring_job_request` | String |  | Request for ModelMonitoringService.CreateModelMonitoringJob. |
| `next_run_time` | String |  | Output only. Timestamp when this Schedule should schedule the next run. Having a next_run_time in the past means the runs are being started behind schedule. |
| `update_time` | String |  | Output only. Timestamp when this Schedule was updated. |
| `create_notebook_execution_job_request` | String |  | Request for NotebookService.CreateNotebookExecutionJob. |
| `allow_queueing` | bool |  | Optional. Whether new scheduled runs can be queued when max_concurrent_runs limit is reached. If set to true, new runs will be queued instead of skipped. Default to false. |
| `last_scheduled_run_response` | String |  | Output only. Response of the last scheduled run. This is the response for starting the scheduled requests and not the execution of the operations/jobs created by the requests (if applicable). Unset if no run has been scheduled yet. |
| `create_pipeline_job_request` | String |  | Request for PipelineService.CreatePipelineJob. CreatePipelineJobRequest.parent field is required (format: projects/{project}/locations/{location}). |
| `end_time` | String |  | Optional. Timestamp after which no new runs can be scheduled. If specified, The schedule will be completed when either end_time is reached or when scheduled_run_count >= max_run_count. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified. |
| `create_time` | String |  | Output only. Timestamp when this Schedule was created. |
| `max_concurrent_run_count` | String |  | Required. Maximum number of runs that can be started concurrently for this Schedule. This is the limit for starting the scheduled requests and not the execution of the operations/jobs created by the requests (if applicable). |
| `last_pause_time` | String |  | Output only. Timestamp when this Schedule was last paused. Unset if never paused. |
| `display_name` | String |  | Required. User provided name of the Schedule. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `start_time` | String |  | Optional. Timestamp after which the first run can be scheduled. Default to Schedule create time if not specified. |
| `name` | String |  | Immutable. The resource name of the Schedule. |
| `state` | String |  | Output only. The state of this Schedule. |
| `max_run_count` | String |  | Optional. Maximum run count of the schedule. If specified, The schedule will be completed when either started_run_count >= max_run_count or when end_time is reached. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified. |
| `catch_up` | bool |  | Output only. Whether to backfill missed runs when the schedule is resumed from PAUSED state. If set to true, all missed runs will be scheduled. New runs will be scheduled after the backfill is complete. Default to false. |
| `last_resume_time` | String |  | Output only. Timestamp when this Schedule was last resumed. Unset if never resumed from pause. |
| `cron` | String |  | Cron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or "TZ=${IANA_TIME_ZONE}". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, "CRON_TZ=America/New_York 1 * * * *", or "TZ=America/New_York 1 * * * *". |
| `started_run_count` | String |  | Output only. The number of runs started by this schedule. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Schedule in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_model_monitoring_job_request` | String | Request for ModelMonitoringService.CreateModelMonitoringJob. |
| `next_run_time` | String | Output only. Timestamp when this Schedule should schedule the next run. Having a next_run_time in the past means the runs are being started behind schedule. |
| `update_time` | String | Output only. Timestamp when this Schedule was updated. |
| `create_notebook_execution_job_request` | String | Request for NotebookService.CreateNotebookExecutionJob. |
| `allow_queueing` | bool | Optional. Whether new scheduled runs can be queued when max_concurrent_runs limit is reached. If set to true, new runs will be queued instead of skipped. Default to false. |
| `last_scheduled_run_response` | String | Output only. Response of the last scheduled run. This is the response for starting the scheduled requests and not the execution of the operations/jobs created by the requests (if applicable). Unset if no run has been scheduled yet. |
| `create_pipeline_job_request` | String | Request for PipelineService.CreatePipelineJob. CreatePipelineJobRequest.parent field is required (format: projects/{project}/locations/{location}). |
| `end_time` | String | Optional. Timestamp after which no new runs can be scheduled. If specified, The schedule will be completed when either end_time is reached or when scheduled_run_count >= max_run_count. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified. |
| `create_time` | String | Output only. Timestamp when this Schedule was created. |
| `max_concurrent_run_count` | String | Required. Maximum number of runs that can be started concurrently for this Schedule. This is the limit for starting the scheduled requests and not the execution of the operations/jobs created by the requests (if applicable). |
| `last_pause_time` | String | Output only. Timestamp when this Schedule was last paused. Unset if never paused. |
| `display_name` | String | Required. User provided name of the Schedule. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `start_time` | String | Optional. Timestamp after which the first run can be scheduled. Default to Schedule create time if not specified. |
| `name` | String | Immutable. The resource name of the Schedule. |
| `state` | String | Output only. The state of this Schedule. |
| `max_run_count` | String | Optional. Maximum run count of the schedule. If specified, The schedule will be completed when either started_run_count >= max_run_count or when end_time is reached. If not specified, new runs will keep getting scheduled until this Schedule is paused or deleted. Already scheduled runs will be allowed to complete. Unset if not specified. |
| `catch_up` | bool | Output only. Whether to backfill missed runs when the schedule is resumed from PAUSED state. If set to true, all missed runs will be scheduled. New runs will be scheduled after the backfill is complete. Default to false. |
| `last_resume_time` | String | Output only. Timestamp when this Schedule was last resumed. Unset if never resumed from pause. |
| `cron` | String | Cron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: "CRON_TZ=${IANA_TIME_ZONE}" or "TZ=${IANA_TIME_ZONE}". The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, "CRON_TZ=America/New_York 1 * * * *", or "TZ=America/New_York 1 * * * *". |
| `started_run_count` | String | Output only. The number of runs started by this schedule. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schedule
schedule = provider.aiplatform_api.Schedule {
    parent = "value"  # Required. The resource name of the Location to create the Schedule in. Format: `projects/{project}/locations/{location}`
}

# Access schedule outputs
schedule_id = schedule.id
schedule_create_model_monitoring_job_request = schedule.create_model_monitoring_job_request
schedule_next_run_time = schedule.next_run_time
schedule_update_time = schedule.update_time
schedule_create_notebook_execution_job_request = schedule.create_notebook_execution_job_request
schedule_allow_queueing = schedule.allow_queueing
schedule_last_scheduled_run_response = schedule.last_scheduled_run_response
schedule_create_pipeline_job_request = schedule.create_pipeline_job_request
schedule_end_time = schedule.end_time
schedule_create_time = schedule.create_time
schedule_max_concurrent_run_count = schedule.max_concurrent_run_count
schedule_last_pause_time = schedule.last_pause_time
schedule_display_name = schedule.display_name
schedule_start_time = schedule.start_time
schedule_name = schedule.name
schedule_state = schedule.state
schedule_max_run_count = schedule.max_run_count
schedule_catch_up = schedule.catch_up
schedule_last_resume_time = schedule.last_resume_time
schedule_cron = schedule.cron
schedule_started_run_count = schedule.started_run_count
```

---


### Artifact

Creates an Artifact associated with a MetadataStore.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | User provided display name of the Artifact. May be up to 128 Unicode characters. |
| `description` | String |  | Description of the Artifact |
| `schema_version` | String |  | The version of the schema in schema_name to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `state` | String |  | The state of this Artifact. This is a property of the Artifact, and does not imply or capture any ongoing process. This property is managed by clients (such as Vertex AI Pipelines), and the system does not prescribe or check the validity of state transitions. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Artifacts. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Artifact (System labels are excluded). |
| `create_time` | String |  | Output only. Timestamp when this Artifact was created. |
| `uri` | String |  | The uniform resource identifier of the artifact file. May be empty if there is no actual artifact file. |
| `etag` | String |  | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String |  | Output only. The resource name of the Artifact. |
| `schema_title` | String |  | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `update_time` | String |  | Output only. Timestamp when this Artifact was last updated. |
| `metadata` | HashMap<String, String> |  | Properties of the Artifact. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |
| `parent` | String | ✅ | Required. The resource name of the MetadataStore where the Artifact should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | User provided display name of the Artifact. May be up to 128 Unicode characters. |
| `description` | String | Description of the Artifact |
| `schema_version` | String | The version of the schema in schema_name to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `state` | String | The state of this Artifact. This is a property of the Artifact, and does not imply or capture any ongoing process. This property is managed by clients (such as Vertex AI Pipelines), and the system does not prescribe or check the validity of state transitions. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Artifacts. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Artifact (System labels are excluded). |
| `create_time` | String | Output only. Timestamp when this Artifact was created. |
| `uri` | String | The uniform resource identifier of the artifact file. May be empty if there is no actual artifact file. |
| `etag` | String | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String | Output only. The resource name of the Artifact. |
| `schema_title` | String | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `update_time` | String | Output only. Timestamp when this Artifact was last updated. |
| `metadata` | HashMap<String, String> | Properties of the Artifact. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create artifact
artifact = provider.aiplatform_api.Artifact {
    parent = "value"  # Required. The resource name of the MetadataStore where the Artifact should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}`
}

# Access artifact outputs
artifact_id = artifact.id
artifact_display_name = artifact.display_name
artifact_description = artifact.description
artifact_schema_version = artifact.schema_version
artifact_state = artifact.state
artifact_labels = artifact.labels
artifact_create_time = artifact.create_time
artifact_uri = artifact.uri
artifact_etag = artifact.etag
artifact_name = artifact.name
artifact_schema_title = artifact.schema_title
artifact_update_time = artifact.update_time
artifact_metadata = artifact.metadata
```

---


### Tensorboard

Creates a Tensorboard.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when this Tensorboard was created. |
| `run_count` | i64 |  | Output only. The number of Runs stored in this Tensorboard. |
| `name` | String |  | Output only. Name of the Tensorboard. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}` |
| `etag` | String |  | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Tensorboards. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Tensorboard (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `display_name` | String |  | Required. User provided name of this Tensorboard. |
| `blob_storage_path_prefix` | String |  | Output only. Consumer project Cloud Storage path prefix used to store blob data, which can either be a bucket or directory. Does not end with a '/'. |
| `description` | String |  | Description of this Tensorboard. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this Tensorboard was last updated. |
| `is_default` | bool |  | Used to indicate if the TensorBoard instance is the default one. Each project & region can have at most one default TensorBoard instance. Creation of a default TensorBoard instance and updating an existing TensorBoard instance to be default will mark all other TensorBoard instances (if any) as non default. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a Tensorboard. If set, this Tensorboard and all sub-resources of this Tensorboard will be secured by this key. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Tensorboard in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when this Tensorboard was created. |
| `run_count` | i64 | Output only. The number of Runs stored in this Tensorboard. |
| `name` | String | Output only. Name of the Tensorboard. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}` |
| `etag` | String | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Tensorboards. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Tensorboard (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `display_name` | String | Required. User provided name of this Tensorboard. |
| `blob_storage_path_prefix` | String | Output only. Consumer project Cloud Storage path prefix used to store blob data, which can either be a bucket or directory. Does not end with a '/'. |
| `description` | String | Description of this Tensorboard. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this Tensorboard was last updated. |
| `is_default` | bool | Used to indicate if the TensorBoard instance is the default one. Each project & region can have at most one default TensorBoard instance. Creation of a default TensorBoard instance and updating an existing TensorBoard instance to be default will mark all other TensorBoard instances (if any) as non default. |
| `encryption_spec` | String | Customer-managed encryption key spec for a Tensorboard. If set, this Tensorboard and all sub-resources of this Tensorboard will be secured by this key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tensorboard
tensorboard = provider.aiplatform_api.Tensorboard {
    parent = "value"  # Required. The resource name of the Location to create the Tensorboard in. Format: `projects/{project}/locations/{location}`
}

# Access tensorboard outputs
tensorboard_id = tensorboard.id
tensorboard_create_time = tensorboard.create_time
tensorboard_run_count = tensorboard.run_count
tensorboard_name = tensorboard.name
tensorboard_etag = tensorboard.etag
tensorboard_satisfies_pzs = tensorboard.satisfies_pzs
tensorboard_labels = tensorboard.labels
tensorboard_display_name = tensorboard.display_name
tensorboard_blob_storage_path_prefix = tensorboard.blob_storage_path_prefix
tensorboard_description = tensorboard.description
tensorboard_satisfies_pzi = tensorboard.satisfies_pzi
tensorboard_update_time = tensorboard.update_time
tensorboard_is_default = tensorboard.is_default
tensorboard_encryption_spec = tensorboard.encryption_spec
```

---


### Feature_monitor_job

Creates a new feature monitor job.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. Description of the FeatureMonitor. |
| `drift_base_snapshot_time` | String |  | Output only. Data snapshot time comparing to which the drift is calculated. |
| `feature_selection_config` | String |  | Output only. Feature selection config used when creating FeatureMonitorJob. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your FeatureMonitorJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureMonitor(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `job_summary` | String |  | Output only. Summary from the FeatureMonitorJob. |
| `trigger_type` | String |  | Output only. Trigger type of the Feature Monitor Job. |
| `create_time` | String |  | Output only. Timestamp when this FeatureMonitorJob was created. Creation of a FeatureMonitorJob means that the job is pending / waiting for sufficient resources but may not have started running yet. |
| `final_status` | String |  | Output only. Final status of the FeatureMonitorJob. |
| `name` | String |  | Identifier. Name of the FeatureMonitorJob. Format: `projects/{project}/locations/{location}/featureGroups/{feature_group}/featureMonitors/{feature_monitor}/featureMonitorJobs/{feature_monitor_job}`. |
| `drift_base_feature_monitor_job_id` | String |  | Output only. FeatureMonitorJob ID comparing to which the drift is calculated. |
| `parent` | String | ✅ | Required. The resource name of FeatureMonitor to create FeatureMonitorJob. Format: `projects/{project}/locations/{location}/featureGroups/{feature_group}/featureMonitors/{feature_monitor}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. Description of the FeatureMonitor. |
| `drift_base_snapshot_time` | String | Output only. Data snapshot time comparing to which the drift is calculated. |
| `feature_selection_config` | String | Output only. Feature selection config used when creating FeatureMonitorJob. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your FeatureMonitorJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureMonitor(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `job_summary` | String | Output only. Summary from the FeatureMonitorJob. |
| `trigger_type` | String | Output only. Trigger type of the Feature Monitor Job. |
| `create_time` | String | Output only. Timestamp when this FeatureMonitorJob was created. Creation of a FeatureMonitorJob means that the job is pending / waiting for sufficient resources but may not have started running yet. |
| `final_status` | String | Output only. Final status of the FeatureMonitorJob. |
| `name` | String | Identifier. Name of the FeatureMonitorJob. Format: `projects/{project}/locations/{location}/featureGroups/{feature_group}/featureMonitors/{feature_monitor}/featureMonitorJobs/{feature_monitor_job}`. |
| `drift_base_feature_monitor_job_id` | String | Output only. FeatureMonitorJob ID comparing to which the drift is calculated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature_monitor_job
feature_monitor_job = provider.aiplatform_api.Feature_monitor_job {
    parent = "value"  # Required. The resource name of FeatureMonitor to create FeatureMonitorJob. Format: `projects/{project}/locations/{location}/featureGroups/{feature_group}/featureMonitors/{feature_monitor}`
}

# Access feature_monitor_job outputs
feature_monitor_job_id = feature_monitor_job.id
feature_monitor_job_description = feature_monitor_job.description
feature_monitor_job_drift_base_snapshot_time = feature_monitor_job.drift_base_snapshot_time
feature_monitor_job_feature_selection_config = feature_monitor_job.feature_selection_config
feature_monitor_job_labels = feature_monitor_job.labels
feature_monitor_job_job_summary = feature_monitor_job.job_summary
feature_monitor_job_trigger_type = feature_monitor_job.trigger_type
feature_monitor_job_create_time = feature_monitor_job.create_time
feature_monitor_job_final_status = feature_monitor_job.final_status
feature_monitor_job_name = feature_monitor_job.name
feature_monitor_job_drift_base_feature_monitor_job_id = feature_monitor_job.drift_base_feature_monitor_job_id
```

---


### Notebook_execution_job

Creates a NotebookExecutionJob.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `execution_user` | String |  | The user email to run the execution as. Only supported by Colab runtimes. |
| `direct_notebook_source` | String |  | The contents of an input notebook file. |
| `workbench_runtime` | String |  | The Workbench runtime configuration to use for the notebook execution. |
| `gcs_notebook_source` | String |  | The Cloud Storage url pointing to the ipynb file. Format: `gs://bucket/notebook_file.ipynb` |
| `status` | String |  | Output only. Populated when the NotebookExecutionJob is completed. When there is an error during notebook execution, the error details are populated. |
| `notebook_runtime_template_resource_name` | String |  | The NotebookRuntimeTemplate to source compute configuration from. |
| `custom_environment_spec` | String |  | The custom compute configuration for an execution job. |
| `kernel_name` | String |  | The name of the kernel to use during notebook execution. If unset, the default kernel is used. |
| `name` | String |  | Output only. The resource name of this NotebookExecutionJob. Format: `projects/{project_id}/locations/{location}/notebookExecutionJobs/{job_id}` |
| `encryption_spec` | String |  | Customer-managed encryption key spec for the notebook execution job. This field is auto-populated if the NotebookRuntimeTemplate has an encryption spec. |
| `gcs_output_uri` | String |  | The Cloud Storage location to upload the result to. Format: `gs://bucket-name` |
| `schedule_resource_name` | String |  | The Schedule resource name if this job is triggered by one. Format: `projects/{project_id}/locations/{location}/schedules/{schedule_id}` |
| `update_time` | String |  | Output only. Timestamp when this NotebookExecutionJob was most recently updated. |
| `job_state` | String |  | Output only. The state of the NotebookExecutionJob. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize NotebookExecutionJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `service_account` | String |  | The service account to run the execution as. |
| `execution_timeout` | String |  | Max running time of the execution job in seconds (default 86400s / 24 hrs). |
| `display_name` | String |  | The display name of the NotebookExecutionJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `create_time` | String |  | Output only. Timestamp when this NotebookExecutionJob was created. |
| `dataform_repository_source` | String |  | The Dataform Repository pointing to a single file notebook repository. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the NotebookExecutionJob. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `execution_user` | String | The user email to run the execution as. Only supported by Colab runtimes. |
| `direct_notebook_source` | String | The contents of an input notebook file. |
| `workbench_runtime` | String | The Workbench runtime configuration to use for the notebook execution. |
| `gcs_notebook_source` | String | The Cloud Storage url pointing to the ipynb file. Format: `gs://bucket/notebook_file.ipynb` |
| `status` | String | Output only. Populated when the NotebookExecutionJob is completed. When there is an error during notebook execution, the error details are populated. |
| `notebook_runtime_template_resource_name` | String | The NotebookRuntimeTemplate to source compute configuration from. |
| `custom_environment_spec` | String | The custom compute configuration for an execution job. |
| `kernel_name` | String | The name of the kernel to use during notebook execution. If unset, the default kernel is used. |
| `name` | String | Output only. The resource name of this NotebookExecutionJob. Format: `projects/{project_id}/locations/{location}/notebookExecutionJobs/{job_id}` |
| `encryption_spec` | String | Customer-managed encryption key spec for the notebook execution job. This field is auto-populated if the NotebookRuntimeTemplate has an encryption spec. |
| `gcs_output_uri` | String | The Cloud Storage location to upload the result to. Format: `gs://bucket-name` |
| `schedule_resource_name` | String | The Schedule resource name if this job is triggered by one. Format: `projects/{project_id}/locations/{location}/schedules/{schedule_id}` |
| `update_time` | String | Output only. Timestamp when this NotebookExecutionJob was most recently updated. |
| `job_state` | String | Output only. The state of the NotebookExecutionJob. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize NotebookExecutionJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `service_account` | String | The service account to run the execution as. |
| `execution_timeout` | String | Max running time of the execution job in seconds (default 86400s / 24 hrs). |
| `display_name` | String | The display name of the NotebookExecutionJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `create_time` | String | Output only. Timestamp when this NotebookExecutionJob was created. |
| `dataform_repository_source` | String | The Dataform Repository pointing to a single file notebook repository. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notebook_execution_job
notebook_execution_job = provider.aiplatform_api.Notebook_execution_job {
    parent = "value"  # Required. The resource name of the Location to create the NotebookExecutionJob. Format: `projects/{project}/locations/{location}`
}

# Access notebook_execution_job outputs
notebook_execution_job_id = notebook_execution_job.id
notebook_execution_job_execution_user = notebook_execution_job.execution_user
notebook_execution_job_direct_notebook_source = notebook_execution_job.direct_notebook_source
notebook_execution_job_workbench_runtime = notebook_execution_job.workbench_runtime
notebook_execution_job_gcs_notebook_source = notebook_execution_job.gcs_notebook_source
notebook_execution_job_status = notebook_execution_job.status
notebook_execution_job_notebook_runtime_template_resource_name = notebook_execution_job.notebook_runtime_template_resource_name
notebook_execution_job_custom_environment_spec = notebook_execution_job.custom_environment_spec
notebook_execution_job_kernel_name = notebook_execution_job.kernel_name
notebook_execution_job_name = notebook_execution_job.name
notebook_execution_job_encryption_spec = notebook_execution_job.encryption_spec
notebook_execution_job_gcs_output_uri = notebook_execution_job.gcs_output_uri
notebook_execution_job_schedule_resource_name = notebook_execution_job.schedule_resource_name
notebook_execution_job_update_time = notebook_execution_job.update_time
notebook_execution_job_job_state = notebook_execution_job.job_state
notebook_execution_job_labels = notebook_execution_job.labels
notebook_execution_job_service_account = notebook_execution_job.service_account
notebook_execution_job_execution_timeout = notebook_execution_job.execution_timeout
notebook_execution_job_display_name = notebook_execution_job.display_name
notebook_execution_job_create_time = notebook_execution_job.create_time
notebook_execution_job_dataform_repository_source = notebook_execution_job.dataform_repository_source
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.aiplatform_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Model

Generate content with multimodal inputs with streaming support.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cached_content` | String |  | Optional. The name of the cached content used as context to serve the prediction. Note: only used in explicit caching, where users can have control over caching (e.g. what content to cache) and enjoy guaranteed cost savings. Format: `projects/{project}/locations/{location}/cachedContents/{cachedContent}` |
| `generation_config` | String |  | Optional. Generation config. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. It is used for billing and reporting only. Label keys and values can be no longer than 63 characters (Unicode codepoints) and can only contain lowercase letters, numeric characters, underscores, and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. |
| `tool_config` | String |  | Optional. Tool config. This config is shared for all tools provided in the request. |
| `tools` | Vec<String> |  | Optional. A list of `Tools` the model may use to generate the next response. A `Tool` is a piece of code that enables the system to interact with external systems to perform an action, or set of actions, outside of knowledge and scope of the model. |
| `safety_settings` | Vec<String> |  | Optional. Per request settings for blocking unsafe content. Enforced on GenerateContentResponse.candidates. |
| `system_instruction` | String |  | Optional. The user provided system instructions for the model. Note: only text should be used in parts and content in each part will be in a separate paragraph. |
| `contents` | Vec<String> |  | Required. The content of the current conversation with the model. For single-turn queries, this is a single instance. For multi-turn queries, this is a repeated field that contains conversation history + latest request. |
| `model_armor_config` | String |  | Optional. Settings for prompt and response sanitization using the Model Armor service. If supplied, safety_settings must not be supplied. |
| `model` | String | ✅ | Required. The fully qualified name of the publisher model or tuned model endpoint to use. Publisher model format: `projects/{project}/locations/{location}/publishers/*/models/*` Tuned model endpoint format: `projects/{project}/locations/{location}/endpoints/{endpoint}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `launch_stage` | String | Optional. Indicates the launch stage of the model. |
| `version_state` | String | Optional. Indicates the state of the model version. |
| `supported_actions` | String | Optional. Supported call-to-action options. |
| `predict_schemata` | String | Optional. The schemata that describes formats of the PublisherModel's predictions and explanations as given and returned via PredictionService.Predict. |
| `name` | String | Output only. The resource name of the PublisherModel. |
| `open_source_category` | String | Required. Indicates the open source category of the publisher model. |
| `publisher_model_template` | String | Optional. Output only. Immutable. Used to indicate this model has a publisher model and provide the template of the publisher model resource name. |
| `version_id` | String | Output only. Immutable. The version ID of the PublisherModel. A new version is committed when a new model version is uploaded under an existing model id. It is an auto-incrementing decimal number in string representation. |
| `frameworks` | Vec<String> | Optional. Additional information about the model's Frameworks. |
| `parent` | String | Optional. The parent that this model was customized from. E.g., Vision API, Natural Language API, LaMDA, T5, etc. Foundation models don't have parents. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model
model = provider.aiplatform_api.Model {
    model = "value"  # Required. The fully qualified name of the publisher model or tuned model endpoint to use. Publisher model format: `projects/{project}/locations/{location}/publishers/*/models/*` Tuned model endpoint format: `projects/{project}/locations/{location}/endpoints/{endpoint}`
}

# Access model outputs
model_id = model.id
model_launch_stage = model.launch_stage
model_version_state = model.version_state
model_supported_actions = model.supported_actions
model_predict_schemata = model.predict_schemata
model_name = model.name
model_open_source_category = model.open_source_category
model_publisher_model_template = model.publisher_model_template
model_version_id = model.version_id
model_frameworks = model.frameworks
model_parent = model.parent
```

---


### Training_pipeline

Creates a TrainingPipeline. A created TrainingPipeline right away will be attempted to be run.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encryption_spec` | String |  | Customer-managed encryption key spec for a TrainingPipeline. If set, this TrainingPipeline will be secured by this key. Note: Model trained by this TrainingPipeline is also secured by this key if model_to_upload is not set separately. |
| `training_task_inputs` | String |  | Required. The training task's parameter(s), as specified in the training_task_definition's `inputs`. |
| `update_time` | String |  | Output only. Time when the TrainingPipeline was most recently updated. |
| `name` | String |  | Output only. Resource name of the TrainingPipeline. |
| `display_name` | String |  | Required. The user-defined name of this TrainingPipeline. |
| `input_data_config` | String |  | Specifies Vertex AI owned input data that may be used for training the Model. The TrainingPipeline's training_task_definition should make clear whether this config is used and if there are any special requirements on how it should be filled. If nothing about this config is mentioned in the training_task_definition, then it should be assumed that the TrainingPipeline does not depend on this configuration. |
| `model_id` | String |  | Optional. The ID to use for the uploaded Model, which will become the final component of the model resource name. This value may be up to 63 characters, and valid characters are `[a-z0-9_-]`. The first character cannot be a number or hyphen. |
| `end_time` | String |  | Output only. Time when the TrainingPipeline entered any of the following states: `PIPELINE_STATE_SUCCEEDED`, `PIPELINE_STATE_FAILED`, `PIPELINE_STATE_CANCELLED`. |
| `training_task_definition` | String |  | Required. A Google Cloud Storage path to the YAML file that defines the training task which is responsible for producing the model artifact, and may also include additional auxiliary work. The definition files that can be used here are found in gs://google-cloud-aiplatform/schema/trainingjob/definition/. Note: The URI given on output will be immutable and probably different, including the URI scheme, than the one given on input. The output URI will point to a location where the user only has a read access. |
| `training_task_metadata` | String |  | Output only. The metadata information as specified in the training_task_definition's `metadata`. This metadata is an auxiliary runtime and final information about the training task. While the pipeline is running this information is populated only at a best effort basis. Only present if the pipeline's training_task_definition contains `metadata` object. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize TrainingPipelines. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `parent_model` | String |  | Optional. When specify this field, the `model_to_upload` will not be uploaded as a new model, instead, it will become a new version of this `parent_model`. |
| `create_time` | String |  | Output only. Time when the TrainingPipeline was created. |
| `model_to_upload` | String |  | Describes the Model that may be uploaded (via ModelService.UploadModel) by this TrainingPipeline. The TrainingPipeline's training_task_definition should make clear whether this Model description should be populated, and if there are any special requirements regarding how it should be filled. If nothing is mentioned in the training_task_definition, then it should be assumed that this field should not be filled and the training task either uploads the Model without a need of this information, or that training task does not support uploading a Model as part of the pipeline. When the Pipeline's state becomes `PIPELINE_STATE_SUCCEEDED` and the trained Model had been uploaded into Vertex AI, then the model_to_upload's resource name is populated. The Model is always uploaded into the Project and Location in which this pipeline is. |
| `state` | String |  | Output only. The detailed state of the pipeline. |
| `error` | String |  | Output only. Only populated when the pipeline's state is `PIPELINE_STATE_FAILED` or `PIPELINE_STATE_CANCELLED`. |
| `start_time` | String |  | Output only. Time when the TrainingPipeline for the first time entered the `PIPELINE_STATE_RUNNING` state. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the TrainingPipeline in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encryption_spec` | String | Customer-managed encryption key spec for a TrainingPipeline. If set, this TrainingPipeline will be secured by this key. Note: Model trained by this TrainingPipeline is also secured by this key if model_to_upload is not set separately. |
| `training_task_inputs` | String | Required. The training task's parameter(s), as specified in the training_task_definition's `inputs`. |
| `update_time` | String | Output only. Time when the TrainingPipeline was most recently updated. |
| `name` | String | Output only. Resource name of the TrainingPipeline. |
| `display_name` | String | Required. The user-defined name of this TrainingPipeline. |
| `input_data_config` | String | Specifies Vertex AI owned input data that may be used for training the Model. The TrainingPipeline's training_task_definition should make clear whether this config is used and if there are any special requirements on how it should be filled. If nothing about this config is mentioned in the training_task_definition, then it should be assumed that the TrainingPipeline does not depend on this configuration. |
| `model_id` | String | Optional. The ID to use for the uploaded Model, which will become the final component of the model resource name. This value may be up to 63 characters, and valid characters are `[a-z0-9_-]`. The first character cannot be a number or hyphen. |
| `end_time` | String | Output only. Time when the TrainingPipeline entered any of the following states: `PIPELINE_STATE_SUCCEEDED`, `PIPELINE_STATE_FAILED`, `PIPELINE_STATE_CANCELLED`. |
| `training_task_definition` | String | Required. A Google Cloud Storage path to the YAML file that defines the training task which is responsible for producing the model artifact, and may also include additional auxiliary work. The definition files that can be used here are found in gs://google-cloud-aiplatform/schema/trainingjob/definition/. Note: The URI given on output will be immutable and probably different, including the URI scheme, than the one given on input. The output URI will point to a location where the user only has a read access. |
| `training_task_metadata` | String | Output only. The metadata information as specified in the training_task_definition's `metadata`. This metadata is an auxiliary runtime and final information about the training task. While the pipeline is running this information is populated only at a best effort basis. Only present if the pipeline's training_task_definition contains `metadata` object. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize TrainingPipelines. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `parent_model` | String | Optional. When specify this field, the `model_to_upload` will not be uploaded as a new model, instead, it will become a new version of this `parent_model`. |
| `create_time` | String | Output only. Time when the TrainingPipeline was created. |
| `model_to_upload` | String | Describes the Model that may be uploaded (via ModelService.UploadModel) by this TrainingPipeline. The TrainingPipeline's training_task_definition should make clear whether this Model description should be populated, and if there are any special requirements regarding how it should be filled. If nothing is mentioned in the training_task_definition, then it should be assumed that this field should not be filled and the training task either uploads the Model without a need of this information, or that training task does not support uploading a Model as part of the pipeline. When the Pipeline's state becomes `PIPELINE_STATE_SUCCEEDED` and the trained Model had been uploaded into Vertex AI, then the model_to_upload's resource name is populated. The Model is always uploaded into the Project and Location in which this pipeline is. |
| `state` | String | Output only. The detailed state of the pipeline. |
| `error` | String | Output only. Only populated when the pipeline's state is `PIPELINE_STATE_FAILED` or `PIPELINE_STATE_CANCELLED`. |
| `start_time` | String | Output only. Time when the TrainingPipeline for the first time entered the `PIPELINE_STATE_RUNNING` state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create training_pipeline
training_pipeline = provider.aiplatform_api.Training_pipeline {
    parent = "value"  # Required. The resource name of the Location to create the TrainingPipeline in. Format: `projects/{project}/locations/{location}`
}

# Access training_pipeline outputs
training_pipeline_id = training_pipeline.id
training_pipeline_encryption_spec = training_pipeline.encryption_spec
training_pipeline_training_task_inputs = training_pipeline.training_task_inputs
training_pipeline_update_time = training_pipeline.update_time
training_pipeline_name = training_pipeline.name
training_pipeline_display_name = training_pipeline.display_name
training_pipeline_input_data_config = training_pipeline.input_data_config
training_pipeline_model_id = training_pipeline.model_id
training_pipeline_end_time = training_pipeline.end_time
training_pipeline_training_task_definition = training_pipeline.training_task_definition
training_pipeline_training_task_metadata = training_pipeline.training_task_metadata
training_pipeline_labels = training_pipeline.labels
training_pipeline_parent_model = training_pipeline.parent_model
training_pipeline_create_time = training_pipeline.create_time
training_pipeline_model_to_upload = training_pipeline.model_to_upload
training_pipeline_state = training_pipeline.state
training_pipeline_error = training_pipeline.error
training_pipeline_start_time = training_pipeline.start_time
```

---


### Model_monitor

Creates a ModelMonitor.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_spec` | String |  | Optional default notification spec, it can be overridden in the ModelMonitoringJob notification spec. |
| `model_monitoring_schema` | String |  | Monitoring Schema is to specify the model's features, prediction outputs and ground truth properties. It is used to extract pertinent data from the dataset and to process features based on their properties. Make sure that the schema aligns with your dataset, if it does not, we will be unable to extract data from the dataset. It is required for most models, but optional for Vertex AI AutoML Tables unless the schem information is not available. |
| `model_monitoring_target` | String |  | The entity that is subject to analysis. Currently only models in Vertex AI Model Registry are supported. If you want to analyze the model which is outside the Vertex AI, you could register a model in Vertex AI Model Registry using just a display name. |
| `create_time` | String |  | Output only. Timestamp when this ModelMonitor was created. |
| `name` | String |  | Immutable. Resource name of the ModelMonitor. Format: `projects/{project}/locations/{location}/modelMonitors/{model_monitor}`. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `display_name` | String |  | The display name of the ModelMonitor. The name can be up to 128 characters long and can consist of any UTF-8. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a ModelMonitor. If set, this ModelMonitor and all sub-resources of this ModelMonitor will be secured by this key. |
| `explanation_spec` | String |  | Optional model explanation spec. It is used for feature attribution monitoring. |
| `tabular_objective` | String |  | Optional default tabular model monitoring objective. |
| `output_spec` | String |  | Optional default monitoring metrics/logs export spec, it can be overridden in the ModelMonitoringJob output spec. If not specified, a default Google Cloud Storage bucket will be created under your project. |
| `training_dataset` | String |  | Optional training dataset used to train the model. It can serve as a reference dataset to identify changes in production. |
| `update_time` | String |  | Output only. Timestamp when this ModelMonitor was updated most recently. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the ModelMonitor in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notification_spec` | String | Optional default notification spec, it can be overridden in the ModelMonitoringJob notification spec. |
| `model_monitoring_schema` | String | Monitoring Schema is to specify the model's features, prediction outputs and ground truth properties. It is used to extract pertinent data from the dataset and to process features based on their properties. Make sure that the schema aligns with your dataset, if it does not, we will be unable to extract data from the dataset. It is required for most models, but optional for Vertex AI AutoML Tables unless the schem information is not available. |
| `model_monitoring_target` | String | The entity that is subject to analysis. Currently only models in Vertex AI Model Registry are supported. If you want to analyze the model which is outside the Vertex AI, you could register a model in Vertex AI Model Registry using just a display name. |
| `create_time` | String | Output only. Timestamp when this ModelMonitor was created. |
| `name` | String | Immutable. Resource name of the ModelMonitor. Format: `projects/{project}/locations/{location}/modelMonitors/{model_monitor}`. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `display_name` | String | The display name of the ModelMonitor. The name can be up to 128 characters long and can consist of any UTF-8. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `encryption_spec` | String | Customer-managed encryption key spec for a ModelMonitor. If set, this ModelMonitor and all sub-resources of this ModelMonitor will be secured by this key. |
| `explanation_spec` | String | Optional model explanation spec. It is used for feature attribution monitoring. |
| `tabular_objective` | String | Optional default tabular model monitoring objective. |
| `output_spec` | String | Optional default monitoring metrics/logs export spec, it can be overridden in the ModelMonitoringJob output spec. If not specified, a default Google Cloud Storage bucket will be created under your project. |
| `training_dataset` | String | Optional training dataset used to train the model. It can serve as a reference dataset to identify changes in production. |
| `update_time` | String | Output only. Timestamp when this ModelMonitor was updated most recently. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model_monitor
model_monitor = provider.aiplatform_api.Model_monitor {
    parent = "value"  # Required. The resource name of the Location to create the ModelMonitor in. Format: `projects/{project}/locations/{location}`
}

# Access model_monitor outputs
model_monitor_id = model_monitor.id
model_monitor_notification_spec = model_monitor.notification_spec
model_monitor_model_monitoring_schema = model_monitor.model_monitoring_schema
model_monitor_model_monitoring_target = model_monitor.model_monitoring_target
model_monitor_create_time = model_monitor.create_time
model_monitor_name = model_monitor.name
model_monitor_satisfies_pzi = model_monitor.satisfies_pzi
model_monitor_display_name = model_monitor.display_name
model_monitor_satisfies_pzs = model_monitor.satisfies_pzs
model_monitor_encryption_spec = model_monitor.encryption_spec
model_monitor_explanation_spec = model_monitor.explanation_spec
model_monitor_tabular_objective = model_monitor.tabular_objective
model_monitor_output_spec = model_monitor.output_spec
model_monitor_training_dataset = model_monitor.training_dataset
model_monitor_update_time = model_monitor.update_time
```

---


### Feature_view_sync

Gets details of a single FeatureViewSync.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `create_time` | String | Output only. Time when this FeatureViewSync is created. Creation of a FeatureViewSync means that the job is pending / waiting for sufficient resources but may not have started the actual data transfer yet. |
| `run_time` | String | Output only. Time when this FeatureViewSync is finished. |
| `final_status` | String | Output only. Final status of the FeatureViewSync. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `sync_summary` | String | Output only. Summary of the sync job. |
| `name` | String | Identifier. Name of the FeatureViewSync. Format: `projects/{project}/locations/{location}/featureOnlineStores/{feature_online_store}/featureViews/{feature_view}/featureViewSyncs/{feature_view_sync}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access feature_view_sync outputs
feature_view_sync_id = feature_view_sync.id
feature_view_sync_satisfies_pzi = feature_view_sync.satisfies_pzi
feature_view_sync_create_time = feature_view_sync.create_time
feature_view_sync_run_time = feature_view_sync.run_time
feature_view_sync_final_status = feature_view_sync.final_status
feature_view_sync_satisfies_pzs = feature_view_sync.satisfies_pzs
feature_view_sync_sync_summary = feature_view_sync.sync_summary
feature_view_sync_name = feature_view_sync.name
```

---


### Context

Creates a Context associated with a MetadataStore.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Description of the Context |
| `parent_contexts` | Vec<String> |  | Output only. A list of resource names of Contexts that are parents of this Context. A Context may have at most 10 parent_contexts. |
| `schema_title` | String |  | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `update_time` | String |  | Output only. Timestamp when this Context was last updated. |
| `etag` | String |  | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Contexts. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Context (System labels are excluded). |
| `schema_version` | String |  | The version of the schema in schema_name to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `create_time` | String |  | Output only. Timestamp when this Context was created. |
| `metadata` | HashMap<String, String> |  | Properties of the Context. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |
| `name` | String |  | Immutable. The resource name of the Context. |
| `display_name` | String |  | User provided display name of the Context. May be up to 128 Unicode characters. |
| `parent` | String | ✅ | Required. The resource name of the MetadataStore where the Context should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Description of the Context |
| `parent_contexts` | Vec<String> | Output only. A list of resource names of Contexts that are parents of this Context. A Context may have at most 10 parent_contexts. |
| `schema_title` | String | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `update_time` | String | Output only. Timestamp when this Context was last updated. |
| `etag` | String | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Contexts. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Context (System labels are excluded). |
| `schema_version` | String | The version of the schema in schema_name to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `create_time` | String | Output only. Timestamp when this Context was created. |
| `metadata` | HashMap<String, String> | Properties of the Context. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |
| `name` | String | Immutable. The resource name of the Context. |
| `display_name` | String | User provided display name of the Context. May be up to 128 Unicode characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create context
context = provider.aiplatform_api.Context {
    parent = "value"  # Required. The resource name of the MetadataStore where the Context should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}`
}

# Access context outputs
context_id = context.id
context_description = context.description
context_parent_contexts = context.parent_contexts
context_schema_title = context.schema_title
context_update_time = context.update_time
context_etag = context.etag
context_labels = context.labels
context_schema_version = context.schema_version
context_create_time = context.create_time
context_metadata = context.metadata
context_name = context.name
context_display_name = context.display_name
```

---


### Location

Gets a Model's spec recommendations. This API is called by UI, SDK, and internal.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `check_user_quota` | bool |  | Optional. If true, check user quota for the recommended regions. Returns all the machine spec in regions they are available, and also the user quota state for each machine type in each region. |
| `gcs_uri` | String |  | Required. The Google Cloud Storage URI of the custom model, storing weights and config files (which can be used to infer the base model). |
| `check_machine_availability` | bool |  | Optional. If true, check machine availability for the recommended regions. Only return the machine spec in regions where the machine is available. |
| `parent` | String | ✅ | Required. The resource name of the Location from which to recommend specs. The users must have permission to make a call in the project. Format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location = provider.aiplatform_api.Location {
    parent = "value"  # Required. The resource name of the Location from which to recommend specs. The users must have permission to make a call in the project. Format: `projects/{project}/locations/{location}`.
}

# Access location outputs
location_id = location.id
location_display_name = location.display_name
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
```

---


### Session

Creates a new Session.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ttl` | String |  | Optional. Input only. The TTL for this session. |
| `expire_time` | String |  | Optional. Timestamp of when this session is considered expired. This is *always* provided on output, regardless of what was sent on input. |
| `user_id` | String |  | Required. Immutable. String id provided by the user |
| `session_state` | HashMap<String, String> |  | Optional. Session specific memory which stores key conversation points. |
| `display_name` | String |  | Optional. The display name of the session. |
| `update_time` | String |  | Output only. Timestamp when the session was updated. |
| `name` | String |  | Identifier. The resource name of the session. Format: 'projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}/sessions/{session}'. |
| `create_time` | String |  | Output only. Timestamp when the session was created. |
| `parent` | String | ✅ | Required. The resource name of the location to create the session in. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ttl` | String | Optional. Input only. The TTL for this session. |
| `expire_time` | String | Optional. Timestamp of when this session is considered expired. This is *always* provided on output, regardless of what was sent on input. |
| `user_id` | String | Required. Immutable. String id provided by the user |
| `session_state` | HashMap<String, String> | Optional. Session specific memory which stores key conversation points. |
| `display_name` | String | Optional. The display name of the session. |
| `update_time` | String | Output only. Timestamp when the session was updated. |
| `name` | String | Identifier. The resource name of the session. Format: 'projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}/sessions/{session}'. |
| `create_time` | String | Output only. Timestamp when the session was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create session
session = provider.aiplatform_api.Session {
    parent = "value"  # Required. The resource name of the location to create the session in. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}`
}

# Access session outputs
session_id = session.id
session_ttl = session.ttl
session_expire_time = session.expire_time
session_user_id = session.user_id
session_session_state = session.session_state
session_display_name = session.display_name
session_update_time = session.update_time
session_name = session.name
session_create_time = session.create_time
```

---


### Feature_group

Creates a new FeatureGroup in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_agent_type` | String |  | Optional. Service agent type used during jobs under a FeatureGroup. By default, the Vertex AI Service Agent is used. When using an IAM Policy to isolate this FeatureGroup within a project, a separate service account should be provisioned by setting this field to `SERVICE_AGENT_TYPE_FEATURE_GROUP`. This will generate a separate service account to access the BigQuery source table. |
| `big_query` | String |  | Indicates that features for this group come from BigQuery Table/View. By default treats the source as a sparse time series source. The BigQuery source table or view must have at least one entity ID column and a column named `feature_timestamp`. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your FeatureGroup. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureGroup(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `create_time` | String |  | Output only. Timestamp when this FeatureGroup was created. |
| `description` | String |  | Optional. Description of the FeatureGroup. |
| `update_time` | String |  | Output only. Timestamp when this FeatureGroup was last updated. |
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String |  | Identifier. Name of the FeatureGroup. Format: `projects/{project}/locations/{location}/featureGroups/{featureGroup}` |
| `service_account_email` | String |  | Output only. A Service Account unique to this FeatureGroup. The role bigquery.dataViewer should be granted to this service account to allow Vertex AI Feature Store to access source data while running jobs under this FeatureGroup. |
| `parent` | String | ✅ | Required. The resource name of the Location to create FeatureGroups. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_agent_type` | String | Optional. Service agent type used during jobs under a FeatureGroup. By default, the Vertex AI Service Agent is used. When using an IAM Policy to isolate this FeatureGroup within a project, a separate service account should be provisioned by setting this field to `SERVICE_AGENT_TYPE_FEATURE_GROUP`. This will generate a separate service account to access the BigQuery source table. |
| `big_query` | String | Indicates that features for this group come from BigQuery Table/View. By default treats the source as a sparse time series source. The BigQuery source table or view must have at least one entity ID column and a column named `feature_timestamp`. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your FeatureGroup. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one FeatureGroup(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `create_time` | String | Output only. Timestamp when this FeatureGroup was created. |
| `description` | String | Optional. Description of the FeatureGroup. |
| `update_time` | String | Output only. Timestamp when this FeatureGroup was last updated. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `name` | String | Identifier. Name of the FeatureGroup. Format: `projects/{project}/locations/{location}/featureGroups/{featureGroup}` |
| `service_account_email` | String | Output only. A Service Account unique to this FeatureGroup. The role bigquery.dataViewer should be granted to this service account to allow Vertex AI Feature Store to access source data while running jobs under this FeatureGroup. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature_group
feature_group = provider.aiplatform_api.Feature_group {
    parent = "value"  # Required. The resource name of the Location to create FeatureGroups. Format: `projects/{project}/locations/{location}`
}

# Access feature_group outputs
feature_group_id = feature_group.id
feature_group_service_agent_type = feature_group.service_agent_type
feature_group_big_query = feature_group.big_query
feature_group_labels = feature_group.labels
feature_group_create_time = feature_group.create_time
feature_group_description = feature_group.description
feature_group_update_time = feature_group.update_time
feature_group_etag = feature_group.etag
feature_group_name = feature_group.name
feature_group_service_account_email = feature_group.service_account_email
```

---


### Model_garden_eula

Checks the EULA acceptance status of a publisher model.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `publisher_model` | String |  | Required. The name of the PublisherModel resource. Format: `publishers/{publisher}/models/{publisher_model}`, or `publishers/hf-{hugging-face-author}/models/{hugging-face-model-name}` |
| `parent` | String | ✅ | Required. The project requesting access for named model. The format is `projects/{project}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model_garden_eula
model_garden_eula = provider.aiplatform_api.Model_garden_eula {
    parent = "value"  # Required. The project requesting access for named model. The format is `projects/{project}`.
}

```

---


### Deployment_resource_pool

Create a DeploymentResourcePool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deployment_resource_pool` | String |  | Required. The DeploymentResourcePool to create. |
| `deployment_resource_pool_id` | String |  | Required. The ID to use for the DeploymentResourcePool, which will become the final component of the DeploymentResourcePool's resource name. The maximum length is 63 characters, and valid characters are `/^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$/`. |
| `parent` | String | ✅ | Required. The parent location resource where this DeploymentResourcePool will be created. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disable_container_logging` | bool | If the DeploymentResourcePool is deployed with custom-trained Models or AutoML Tabular Models, the container(s) of the DeploymentResourcePool will send `stderr` and `stdout` streams to Cloud Logging by default. Please note that the logs incur cost, which are subject to [Cloud Logging pricing](https://cloud.google.com/logging/pricing). User can disable container logging by setting this flag to true. |
| `dedicated_resources` | String | Required. The underlying DedicatedResources that the DeploymentResourcePool uses. |
| `encryption_spec` | String | Customer-managed encryption key spec for a DeploymentResourcePool. If set, this DeploymentResourcePool will be secured by this key. Endpoints and the DeploymentResourcePool they deploy in need to have the same EncryptionSpec. |
| `name` | String | Immutable. The resource name of the DeploymentResourcePool. Format: `projects/{project}/locations/{location}/deploymentResourcePools/{deployment_resource_pool}` |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `create_time` | String | Output only. Timestamp when this DeploymentResourcePool was created. |
| `service_account` | String | The service account that the DeploymentResourcePool's container(s) run as. Specify the email address of the service account. If this service account is not specified, the container(s) run as a service account that doesn't have access to the resource project. Users deploying the Models to this DeploymentResourcePool must have the `iam.serviceAccounts.actAs` permission on this service account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deployment_resource_pool
deployment_resource_pool = provider.aiplatform_api.Deployment_resource_pool {
    parent = "value"  # Required. The parent location resource where this DeploymentResourcePool will be created. Format: `projects/{project}/locations/{location}`
}

# Access deployment_resource_pool outputs
deployment_resource_pool_id = deployment_resource_pool.id
deployment_resource_pool_disable_container_logging = deployment_resource_pool.disable_container_logging
deployment_resource_pool_dedicated_resources = deployment_resource_pool.dedicated_resources
deployment_resource_pool_encryption_spec = deployment_resource_pool.encryption_spec
deployment_resource_pool_name = deployment_resource_pool.name
deployment_resource_pool_satisfies_pzi = deployment_resource_pool.satisfies_pzi
deployment_resource_pool_satisfies_pzs = deployment_resource_pool.satisfies_pzs
deployment_resource_pool_create_time = deployment_resource_pool.create_time
deployment_resource_pool_service_account = deployment_resource_pool.service_account
```

---


### Memorie

Create a Memory.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fact` | String |  | Required. Semantic knowledge extracted from the source content. |
| `update_time` | String |  | Output only. Timestamp when this Memory was most recently updated. |
| `scope` | HashMap<String, String> |  | Required. Immutable. The scope of the Memory. Memories are isolated within their scope. The scope is defined when creating or generating memories. Scope values cannot contain the wildcard character '*'. |
| `description` | String |  | Optional. Description of the Memory. |
| `display_name` | String |  | Optional. Display name of the Memory. |
| `create_time` | String |  | Output only. Timestamp when this Memory was created. |
| `expire_time` | String |  | Optional. Timestamp of when this resource is considered expired. This is *always* provided on output when `expiration` is set on input, regardless of whether `expire_time` or `ttl` was provided. |
| `ttl` | String |  | Optional. Input only. The TTL for this resource. The expiration time is computed: now + TTL. |
| `name` | String |  | Identifier. The resource name of the Memory. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}/memories/{memory}` |
| `parent` | String | ✅ | Required. The resource name of the ReasoningEngine to create the Memory under. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fact` | String | Required. Semantic knowledge extracted from the source content. |
| `update_time` | String | Output only. Timestamp when this Memory was most recently updated. |
| `scope` | HashMap<String, String> | Required. Immutable. The scope of the Memory. Memories are isolated within their scope. The scope is defined when creating or generating memories. Scope values cannot contain the wildcard character '*'. |
| `description` | String | Optional. Description of the Memory. |
| `display_name` | String | Optional. Display name of the Memory. |
| `create_time` | String | Output only. Timestamp when this Memory was created. |
| `expire_time` | String | Optional. Timestamp of when this resource is considered expired. This is *always* provided on output when `expiration` is set on input, regardless of whether `expire_time` or `ttl` was provided. |
| `ttl` | String | Optional. Input only. The TTL for this resource. The expiration time is computed: now + TTL. |
| `name` | String | Identifier. The resource name of the Memory. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}/memories/{memory}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create memorie
memorie = provider.aiplatform_api.Memorie {
    parent = "value"  # Required. The resource name of the ReasoningEngine to create the Memory under. Format: `projects/{project}/locations/{location}/reasoningEngines/{reasoning_engine}`
}

# Access memorie outputs
memorie_id = memorie.id
memorie_fact = memorie.fact
memorie_update_time = memorie.update_time
memorie_scope = memorie.scope
memorie_description = memorie.description
memorie_display_name = memorie.display_name
memorie_create_time = memorie.create_time
memorie_expire_time = memorie.expire_time
memorie_ttl = memorie.ttl
memorie_name = memorie.name
```

---


### Featurestore

Creates a new Featurestore in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the featurestore. |
| `update_time` | String |  | Output only. Timestamp when this Featurestore was last updated. |
| `name` | String |  | Output only. Name of the Featurestore. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}` |
| `online_serving_config` | String |  | Optional. Config for online storage resources. The field should not co-exist with the field of `OnlineStoreReplicationConfig`. If both of it and OnlineStoreReplicationConfig are unset, the feature store will not have an online store and cannot be used for online serving. |
| `create_time` | String |  | Output only. Timestamp when this Featurestore was created. |
| `etag` | String |  | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your Featurestore. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one Featurestore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `online_storage_ttl_days` | i64 |  | Optional. TTL in days for feature values that will be stored in online serving storage. The Feature Store online storage periodically removes obsolete feature values older than `online_storage_ttl_days` since the feature generation time. Note that `online_storage_ttl_days` should be less than or equal to `offline_storage_ttl_days` for each EntityType under a featurestore. If not set, default to 4000 days |
| `encryption_spec` | String |  | Optional. Customer-managed encryption key spec for data storage. If set, both of the online and offline data storage will be secured by this key. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The resource name of the Location to create Featurestores. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the featurestore. |
| `update_time` | String | Output only. Timestamp when this Featurestore was last updated. |
| `name` | String | Output only. Name of the Featurestore. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}` |
| `online_serving_config` | String | Optional. Config for online storage resources. The field should not co-exist with the field of `OnlineStoreReplicationConfig`. If both of it and OnlineStoreReplicationConfig are unset, the feature store will not have an online store and cannot be used for online serving. |
| `create_time` | String | Output only. Timestamp when this Featurestore was created. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your Featurestore. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one Featurestore(System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `online_storage_ttl_days` | i64 | Optional. TTL in days for feature values that will be stored in online serving storage. The Feature Store online storage periodically removes obsolete feature values older than `online_storage_ttl_days` since the feature generation time. Note that `online_storage_ttl_days` should be less than or equal to `offline_storage_ttl_days` for each EntityType under a featurestore. If not set, default to 4000 days |
| `encryption_spec` | String | Optional. Customer-managed encryption key spec for data storage. If set, both of the online and offline data storage will be secured by this key. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create featurestore
featurestore = provider.aiplatform_api.Featurestore {
    parent = "value"  # Required. The resource name of the Location to create Featurestores. Format: `projects/{project}/locations/{location}`
}

# Access featurestore outputs
featurestore_id = featurestore.id
featurestore_state = featurestore.state
featurestore_update_time = featurestore.update_time
featurestore_name = featurestore.name
featurestore_online_serving_config = featurestore.online_serving_config
featurestore_create_time = featurestore.create_time
featurestore_etag = featurestore.etag
featurestore_labels = featurestore.labels
featurestore_online_storage_ttl_days = featurestore.online_storage_ttl_days
featurestore_encryption_spec = featurestore.encryption_spec
featurestore_satisfies_pzi = featurestore.satisfies_pzi
featurestore_satisfies_pzs = featurestore.satisfies_pzs
```

---


### Endpoint

Creates an Endpoint.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gen_ai_advanced_features_config` | String |  | Optional. Configuration for GenAiAdvancedFeatures. If the endpoint is serving GenAI models, advanced features like native RAG integration can be configured. Currently, only Model Garden models are supported. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `network` | String |  | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where `{project}` is a project number, as in `12345`, and `{network}` is network name. |
| `model_deployment_monitoring_job` | String |  | Output only. Resource name of the Model Monitoring job associated with this Endpoint if monitoring is enabled by JobService.CreateModelDeploymentMonitoringJob. Format: `projects/{project}/locations/{location}/modelDeploymentMonitoringJobs/{model_deployment_monitoring_job}` |
| `encryption_spec` | String |  | Customer-managed encryption key spec for an Endpoint. If set, this Endpoint and all sub-resources of this Endpoint will be secured by this key. |
| `enable_private_service_connect` | bool |  | Deprecated: If true, expose the Endpoint via private service connect. Only one of the fields, network or enable_private_service_connect, can be set. |
| `client_connection_config` | String |  | Configurations that are applied to the endpoint for online prediction. |
| `gdc_config` | String |  | Configures the Google Distributed Cloud (GDC) environment for online prediction. Only set this field when the Endpoint is to be deployed in a GDC environment. |
| `deployed_models` | Vec<String> |  | Output only. The models deployed in this Endpoint. To add or remove DeployedModels use EndpointService.DeployModel and EndpointService.UndeployModel respectively. |
| `description` | String |  | The description of the Endpoint. |
| `display_name` | String |  | Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `dedicated_endpoint_dns` | String |  | Output only. DNS of the dedicated endpoint. Will only be populated if dedicated_endpoint_enabled is true. Depending on the features enabled, uid might be a random number or a string. For example, if fast_tryout is enabled, uid will be fasttryout. Format: `https://{endpoint_id}.{region}-{uid}.prediction.vertexai.goog`. |
| `create_time` | String |  | Output only. Timestamp when this Endpoint was created. |
| `name` | String |  | Output only. The resource name of the Endpoint. |
| `private_service_connect_config` | String |  | Optional. Configuration for private service connect. network and private_service_connect_config are mutually exclusive. |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `dedicated_endpoint_enabled` | bool |  | If true, the endpoint will be exposed through a dedicated DNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS will be isolated from other users' traffic and will have better performance and reliability. Note: Once you enabled dedicated endpoint, you won't be able to send request to the shared DNS {region}-aiplatform.googleapis.com. The limitation will be removed soon. |
| `traffic_split` | HashMap<String, i64> |  | A map from a DeployedModel's ID to the percentage of this Endpoint's traffic that should be forwarded to that DeployedModel. If a DeployedModel's ID is not listed in this map, then it receives no traffic. The traffic percentage values must add up to 100, or map must be empty if the Endpoint is to not accept any traffic at a moment. |
| `predict_request_response_logging_config` | String |  | Configures the request-response logging for online prediction. |
| `update_time` | String |  | Output only. Timestamp when this Endpoint was last updated. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Endpoint in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gen_ai_advanced_features_config` | String | Optional. Configuration for GenAiAdvancedFeatures. If the endpoint is serving GenAI models, advanced features like native RAG integration can be configured. Currently, only Model Garden models are supported. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `network` | String | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the Endpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. Only one of the fields, network or enable_private_service_connect, can be set. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where `{project}` is a project number, as in `12345`, and `{network}` is network name. |
| `model_deployment_monitoring_job` | String | Output only. Resource name of the Model Monitoring job associated with this Endpoint if monitoring is enabled by JobService.CreateModelDeploymentMonitoringJob. Format: `projects/{project}/locations/{location}/modelDeploymentMonitoringJobs/{model_deployment_monitoring_job}` |
| `encryption_spec` | String | Customer-managed encryption key spec for an Endpoint. If set, this Endpoint and all sub-resources of this Endpoint will be secured by this key. |
| `enable_private_service_connect` | bool | Deprecated: If true, expose the Endpoint via private service connect. Only one of the fields, network or enable_private_service_connect, can be set. |
| `client_connection_config` | String | Configurations that are applied to the endpoint for online prediction. |
| `gdc_config` | String | Configures the Google Distributed Cloud (GDC) environment for online prediction. Only set this field when the Endpoint is to be deployed in a GDC environment. |
| `deployed_models` | Vec<String> | Output only. The models deployed in this Endpoint. To add or remove DeployedModels use EndpointService.DeployModel and EndpointService.UndeployModel respectively. |
| `description` | String | The description of the Endpoint. |
| `display_name` | String | Required. The display name of the Endpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `dedicated_endpoint_dns` | String | Output only. DNS of the dedicated endpoint. Will only be populated if dedicated_endpoint_enabled is true. Depending on the features enabled, uid might be a random number or a string. For example, if fast_tryout is enabled, uid will be fasttryout. Format: `https://{endpoint_id}.{region}-{uid}.prediction.vertexai.goog`. |
| `create_time` | String | Output only. Timestamp when this Endpoint was created. |
| `name` | String | Output only. The resource name of the Endpoint. |
| `private_service_connect_config` | String | Optional. Configuration for private service connect. network and private_service_connect_config are mutually exclusive. |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Endpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `dedicated_endpoint_enabled` | bool | If true, the endpoint will be exposed through a dedicated DNS [Endpoint.dedicated_endpoint_dns]. Your request to the dedicated DNS will be isolated from other users' traffic and will have better performance and reliability. Note: Once you enabled dedicated endpoint, you won't be able to send request to the shared DNS {region}-aiplatform.googleapis.com. The limitation will be removed soon. |
| `traffic_split` | HashMap<String, i64> | A map from a DeployedModel's ID to the percentage of this Endpoint's traffic that should be forwarded to that DeployedModel. If a DeployedModel's ID is not listed in this map, then it receives no traffic. The traffic percentage values must add up to 100, or map must be empty if the Endpoint is to not accept any traffic at a moment. |
| `predict_request_response_logging_config` | String | Configures the request-response logging for online prediction. |
| `update_time` | String | Output only. Timestamp when this Endpoint was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create endpoint
endpoint = provider.aiplatform_api.Endpoint {
    parent = "value"  # Required. The resource name of the Location to create the Endpoint in. Format: `projects/{project}/locations/{location}`
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_gen_ai_advanced_features_config = endpoint.gen_ai_advanced_features_config
endpoint_satisfies_pzs = endpoint.satisfies_pzs
endpoint_network = endpoint.network
endpoint_model_deployment_monitoring_job = endpoint.model_deployment_monitoring_job
endpoint_encryption_spec = endpoint.encryption_spec
endpoint_enable_private_service_connect = endpoint.enable_private_service_connect
endpoint_client_connection_config = endpoint.client_connection_config
endpoint_gdc_config = endpoint.gdc_config
endpoint_deployed_models = endpoint.deployed_models
endpoint_description = endpoint.description
endpoint_display_name = endpoint.display_name
endpoint_dedicated_endpoint_dns = endpoint.dedicated_endpoint_dns
endpoint_create_time = endpoint.create_time
endpoint_name = endpoint.name
endpoint_private_service_connect_config = endpoint.private_service_connect_config
endpoint_etag = endpoint.etag
endpoint_labels = endpoint.labels
endpoint_satisfies_pzi = endpoint.satisfies_pzi
endpoint_dedicated_endpoint_enabled = endpoint.dedicated_endpoint_enabled
endpoint_traffic_split = endpoint.traffic_split
endpoint_predict_request_response_logging_config = endpoint.predict_request_response_logging_config
endpoint_update_time = endpoint.update_time
```

---


### Experiment

Creates a TensorboardExperiment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Name of the TensorboardExperiment. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}` |
| `source` | String |  | Immutable. Source of the TensorboardExperiment. Example: a custom training job. |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `create_time` | String |  | Output only. Timestamp when this TensorboardExperiment was created. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your TensorboardExperiment. Label keys and values cannot be longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Dataset (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `aiplatform.googleapis.com/` and are immutable. The following system labels exist for each Dataset: * `aiplatform.googleapis.com/dataset_metadata_schema`: output only. Its value is the metadata_schema's title. |
| `description` | String |  | Description of this TensorboardExperiment. |
| `display_name` | String |  | User provided name of this TensorboardExperiment. |
| `update_time` | String |  | Output only. Timestamp when this TensorboardExperiment was last updated. |
| `parent` | String | ✅ | Required. The resource name of the Tensorboard to create the TensorboardExperiment in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Name of the TensorboardExperiment. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}` |
| `source` | String | Immutable. Source of the TensorboardExperiment. Example: a custom training job. |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `create_time` | String | Output only. Timestamp when this TensorboardExperiment was created. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your TensorboardExperiment. Label keys and values cannot be longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Dataset (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `aiplatform.googleapis.com/` and are immutable. The following system labels exist for each Dataset: * `aiplatform.googleapis.com/dataset_metadata_schema`: output only. Its value is the metadata_schema's title. |
| `description` | String | Description of this TensorboardExperiment. |
| `display_name` | String | User provided name of this TensorboardExperiment. |
| `update_time` | String | Output only. Timestamp when this TensorboardExperiment was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create experiment
experiment = provider.aiplatform_api.Experiment {
    parent = "value"  # Required. The resource name of the Tensorboard to create the TensorboardExperiment in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}`
}

# Access experiment outputs
experiment_id = experiment.id
experiment_name = experiment.name
experiment_source = experiment.source
experiment_etag = experiment.etag
experiment_create_time = experiment.create_time
experiment_labels = experiment.labels
experiment_description = experiment.description
experiment_display_name = experiment.display_name
experiment_update_time = experiment.update_time
```

---


### Persistent_resource

Creates a PersistentResource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network` | String |  | Optional. The full name of the Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to peered with Vertex AI to host the persistent resources. For example, `projects/12345/global/networks/myVPC`. [Format](/compute/docs/reference/rest/v1/networks/insert) is of the form `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in `12345`, and {network} is a network name. To specify this field, you must have already [configured VPC Network Peering for Vertex AI](https://cloud.google.com/vertex-ai/docs/general/vpc-peering). If this field is left unspecified, the resources aren't peered with any network. |
| `resource_runtime_spec` | String |  | Optional. Persistent Resource runtime spec. For example, used for Ray cluster configuration. |
| `reserved_ip_ranges` | Vec<String> |  | Optional. A list of names for the reserved IP ranges under the VPC network that can be used for this persistent resource. If set, we will deploy the persistent resource within the provided IP ranges. Otherwise, the persistent resource is deployed to any IP ranges under the provided VPC network. Example: ['vertex-ai-ip-range']. |
| `resource_runtime` | String |  | Output only. Runtime information of the Persistent Resource. |
| `create_time` | String |  | Output only. Time when the PersistentResource was created. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize PersistentResource. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `name` | String |  | Immutable. Resource name of a PersistentResource. |
| `resource_pools` | Vec<String> |  | Required. The spec of the pools of different resources. |
| `start_time` | String |  | Output only. Time when the PersistentResource for the first time entered the `RUNNING` state. |
| `encryption_spec` | String |  | Optional. Customer-managed encryption key spec for a PersistentResource. If set, this PersistentResource and all sub-resources of this PersistentResource will be secured by this key. |
| `psc_interface_config` | String |  | Optional. Configuration for PSC-I for PersistentResource. |
| `state` | String |  | Output only. The detailed state of a Study. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `error` | String |  | Output only. Only populated when persistent resource's state is `STOPPING` or `ERROR`. |
| `display_name` | String |  | Optional. The display name of the PersistentResource. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `update_time` | String |  | Output only. Time when the PersistentResource was most recently updated. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the PersistentResource in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network` | String | Optional. The full name of the Compute Engine [network](/compute/docs/networks-and-firewalls#networks) to peered with Vertex AI to host the persistent resources. For example, `projects/12345/global/networks/myVPC`. [Format](/compute/docs/reference/rest/v1/networks/insert) is of the form `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in `12345`, and {network} is a network name. To specify this field, you must have already [configured VPC Network Peering for Vertex AI](https://cloud.google.com/vertex-ai/docs/general/vpc-peering). If this field is left unspecified, the resources aren't peered with any network. |
| `resource_runtime_spec` | String | Optional. Persistent Resource runtime spec. For example, used for Ray cluster configuration. |
| `reserved_ip_ranges` | Vec<String> | Optional. A list of names for the reserved IP ranges under the VPC network that can be used for this persistent resource. If set, we will deploy the persistent resource within the provided IP ranges. Otherwise, the persistent resource is deployed to any IP ranges under the provided VPC network. Example: ['vertex-ai-ip-range']. |
| `resource_runtime` | String | Output only. Runtime information of the Persistent Resource. |
| `create_time` | String | Output only. Time when the PersistentResource was created. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize PersistentResource. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `name` | String | Immutable. Resource name of a PersistentResource. |
| `resource_pools` | Vec<String> | Required. The spec of the pools of different resources. |
| `start_time` | String | Output only. Time when the PersistentResource for the first time entered the `RUNNING` state. |
| `encryption_spec` | String | Optional. Customer-managed encryption key spec for a PersistentResource. If set, this PersistentResource and all sub-resources of this PersistentResource will be secured by this key. |
| `psc_interface_config` | String | Optional. Configuration for PSC-I for PersistentResource. |
| `state` | String | Output only. The detailed state of a Study. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `error` | String | Output only. Only populated when persistent resource's state is `STOPPING` or `ERROR`. |
| `display_name` | String | Optional. The display name of the PersistentResource. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `update_time` | String | Output only. Time when the PersistentResource was most recently updated. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create persistent_resource
persistent_resource = provider.aiplatform_api.Persistent_resource {
    parent = "value"  # Required. The resource name of the Location to create the PersistentResource in. Format: `projects/{project}/locations/{location}`
}

# Access persistent_resource outputs
persistent_resource_id = persistent_resource.id
persistent_resource_network = persistent_resource.network
persistent_resource_resource_runtime_spec = persistent_resource.resource_runtime_spec
persistent_resource_reserved_ip_ranges = persistent_resource.reserved_ip_ranges
persistent_resource_resource_runtime = persistent_resource.resource_runtime
persistent_resource_create_time = persistent_resource.create_time
persistent_resource_labels = persistent_resource.labels
persistent_resource_name = persistent_resource.name
persistent_resource_resource_pools = persistent_resource.resource_pools
persistent_resource_start_time = persistent_resource.start_time
persistent_resource_encryption_spec = persistent_resource.encryption_spec
persistent_resource_psc_interface_config = persistent_resource.psc_interface_config
persistent_resource_state = persistent_resource.state
persistent_resource_satisfies_pzi = persistent_resource.satisfies_pzi
persistent_resource_error = persistent_resource.error
persistent_resource_display_name = persistent_resource.display_name
persistent_resource_update_time = persistent_resource.update_time
persistent_resource_satisfies_pzs = persistent_resource.satisfies_pzs
```

---


### Custom_job

Creates a CustomJob. A created CustomJob right away will be attempted to be run.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `start_time` | String |  | Output only. Time when the CustomJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Time when the CustomJob was most recently updated. |
| `web_access_uris` | HashMap<String, String> |  | Output only. URIs for accessing [interactive shells](https://cloud.google.com/vertex-ai/docs/training/monitor-debug-interactive-shell) (one URI for each training node). Only available if job_spec.enable_web_access is `true`. The keys are names of each node in the training job; for example, `workerpool0-0` for the primary node, `workerpool1-0` for the first node in the second worker pool, and `workerpool1-1` for the second node in the second worker pool. The values are the URIs for each node's interactive shell. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize CustomJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `end_time` | String |  | Output only. Time when the CustomJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `state` | String |  | Output only. The detailed state of the job. |
| `create_time` | String |  | Output only. Time when the CustomJob was created. |
| `display_name` | String |  | Required. The display name of the CustomJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `job_spec` | String |  | Required. Job spec. |
| `encryption_spec` | String |  | Customer-managed encryption key options for a CustomJob. If this is set, then all resources created by the CustomJob will be encrypted with the provided encryption key. |
| `error` | String |  | Output only. Only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `name` | String |  | Output only. Resource name of a CustomJob. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the CustomJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `start_time` | String | Output only. Time when the CustomJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Time when the CustomJob was most recently updated. |
| `web_access_uris` | HashMap<String, String> | Output only. URIs for accessing [interactive shells](https://cloud.google.com/vertex-ai/docs/training/monitor-debug-interactive-shell) (one URI for each training node). Only available if job_spec.enable_web_access is `true`. The keys are names of each node in the training job; for example, `workerpool0-0` for the primary node, `workerpool1-0` for the first node in the second worker pool, and `workerpool1-1` for the second node in the second worker pool. The values are the URIs for each node's interactive shell. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize CustomJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `end_time` | String | Output only. Time when the CustomJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `state` | String | Output only. The detailed state of the job. |
| `create_time` | String | Output only. Time when the CustomJob was created. |
| `display_name` | String | Required. The display name of the CustomJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `job_spec` | String | Required. Job spec. |
| `encryption_spec` | String | Customer-managed encryption key options for a CustomJob. If this is set, then all resources created by the CustomJob will be encrypted with the provided encryption key. |
| `error` | String | Output only. Only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `name` | String | Output only. Resource name of a CustomJob. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_job
custom_job = provider.aiplatform_api.Custom_job {
    parent = "value"  # Required. The resource name of the Location to create the CustomJob in. Format: `projects/{project}/locations/{location}`
}

# Access custom_job outputs
custom_job_id = custom_job.id
custom_job_satisfies_pzs = custom_job.satisfies_pzs
custom_job_start_time = custom_job.start_time
custom_job_satisfies_pzi = custom_job.satisfies_pzi
custom_job_update_time = custom_job.update_time
custom_job_web_access_uris = custom_job.web_access_uris
custom_job_labels = custom_job.labels
custom_job_end_time = custom_job.end_time
custom_job_state = custom_job.state
custom_job_create_time = custom_job.create_time
custom_job_display_name = custom_job.display_name
custom_job_job_spec = custom_job.job_spec
custom_job_encryption_spec = custom_job.encryption_spec
custom_job_error = custom_job.error
custom_job_name = custom_job.name
```

---


### Saved_querie

Lists SavedQueries in a Dataset.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `saved_queries` | Vec<String> | A list of SavedQueries that match the specified filter in the request. |
| `next_page_token` | String | The standard List next-page token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access saved_querie outputs
saved_querie_id = saved_querie.id
saved_querie_saved_queries = saved_querie.saved_queries
saved_querie_next_page_token = saved_querie.next_page_token
```

---


### Evaluation_run

Creates an Evaluation Run.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `evaluation_results` | String |  | Output only. The results of the evaluation run. Only populated when the evaluation run's state is SUCCEEDED. |
| `display_name` | String |  | Required. The display name of the Evaluation Run. |
| `name` | String |  | Identifier. The resource name of the EvaluationRun. This is a unique identifier. Format: `projects/{project}/locations/{location}/evaluationRuns/{evaluation_run}` |
| `labels` | HashMap<String, String> |  | Optional. Labels for the evaluation run. |
| `evaluation_set_snapshot` | String |  | Output only. The specific evaluation set of the evaluation run. For runs with an evaluation set input, this will be that same set. For runs with BigQuery input, it's the sampled BigQuery dataset. |
| `metadata` | String |  | Optional. Metadata about the evaluation run, can be used by the caller to store additional tracking information about the evaluation run. |
| `completion_time` | String |  | Output only. Time when the evaluation run was completed. |
| `state` | String |  | Output only. The state of the evaluation run. |
| `evaluation_config` | String |  | Required. The configuration used for the evaluation. |
| `create_time` | String |  | Output only. Time when the evaluation run was created. |
| `data_source` | String |  | Required. The data source for the evaluation run. |
| `error` | String |  | Output only. Only populated when the evaluation run's state is FAILED or CANCELLED. |
| `inference_configs` | HashMap<String, String> |  | Optional. The candidate to inference config map for the evaluation run. The candidate can be up to 128 characters long and can consist of any UTF-8 characters. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Evaluation Run in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evaluation_results` | String | Output only. The results of the evaluation run. Only populated when the evaluation run's state is SUCCEEDED. |
| `display_name` | String | Required. The display name of the Evaluation Run. |
| `name` | String | Identifier. The resource name of the EvaluationRun. This is a unique identifier. Format: `projects/{project}/locations/{location}/evaluationRuns/{evaluation_run}` |
| `labels` | HashMap<String, String> | Optional. Labels for the evaluation run. |
| `evaluation_set_snapshot` | String | Output only. The specific evaluation set of the evaluation run. For runs with an evaluation set input, this will be that same set. For runs with BigQuery input, it's the sampled BigQuery dataset. |
| `metadata` | String | Optional. Metadata about the evaluation run, can be used by the caller to store additional tracking information about the evaluation run. |
| `completion_time` | String | Output only. Time when the evaluation run was completed. |
| `state` | String | Output only. The state of the evaluation run. |
| `evaluation_config` | String | Required. The configuration used for the evaluation. |
| `create_time` | String | Output only. Time when the evaluation run was created. |
| `data_source` | String | Required. The data source for the evaluation run. |
| `error` | String | Output only. Only populated when the evaluation run's state is FAILED or CANCELLED. |
| `inference_configs` | HashMap<String, String> | Optional. The candidate to inference config map for the evaluation run. The candidate can be up to 128 characters long and can consist of any UTF-8 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation_run
evaluation_run = provider.aiplatform_api.Evaluation_run {
    parent = "value"  # Required. The resource name of the Location to create the Evaluation Run in. Format: `projects/{project}/locations/{location}`
}

# Access evaluation_run outputs
evaluation_run_id = evaluation_run.id
evaluation_run_evaluation_results = evaluation_run.evaluation_results
evaluation_run_display_name = evaluation_run.display_name
evaluation_run_name = evaluation_run.name
evaluation_run_labels = evaluation_run.labels
evaluation_run_evaluation_set_snapshot = evaluation_run.evaluation_set_snapshot
evaluation_run_metadata = evaluation_run.metadata
evaluation_run_completion_time = evaluation_run.completion_time
evaluation_run_state = evaluation_run.state
evaluation_run_evaluation_config = evaluation_run.evaluation_config
evaluation_run_create_time = evaluation_run.create_time
evaluation_run_data_source = evaluation_run.data_source
evaluation_run_error = evaluation_run.error
evaluation_run_inference_configs = evaluation_run.inference_configs
```

---


### Index_endpoint

Creates an IndexEndpoint.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deployed_indexes` | Vec<String> |  | Output only. The indexes deployed in this endpoint. |
| `private_service_connect_config` | String |  | Optional. Configuration for private service connect. network and private_service_connect_config are mutually exclusive. |
| `encryption_spec` | String |  | Immutable. Customer-managed encryption key spec for an IndexEndpoint. If set, this IndexEndpoint and all sub-resources of this IndexEndpoint will be secured by this key. |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `public_endpoint_domain_name` | String |  | Output only. If public_endpoint_enabled is true, this field will be populated with the domain name to use for this index endpoint. |
| `enable_private_service_connect` | bool |  | Optional. Deprecated: If true, expose the IndexEndpoint via private service connect. Only one of the fields, network or enable_private_service_connect, can be set. |
| `public_endpoint_enabled` | bool |  | Optional. If true, the deployed index will be accessible through public endpoint. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your IndexEndpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `display_name` | String |  | Required. The display name of the IndexEndpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `update_time` | String |  | Output only. Timestamp when this IndexEndpoint was last updated. This timestamp is not updated when the endpoint's DeployedIndexes are updated, e.g. due to updates of the original Indexes they are the deployments of. |
| `network` | String |  | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the IndexEndpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. network and private_service_connect_config are mutually exclusive. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in '12345', and {network} is network name. |
| `create_time` | String |  | Output only. Timestamp when this IndexEndpoint was created. |
| `name` | String |  | Output only. The resource name of the IndexEndpoint. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `description` | String |  | The description of the IndexEndpoint. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the IndexEndpoint in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployed_indexes` | Vec<String> | Output only. The indexes deployed in this endpoint. |
| `private_service_connect_config` | String | Optional. Configuration for private service connect. network and private_service_connect_config are mutually exclusive. |
| `encryption_spec` | String | Immutable. Customer-managed encryption key spec for an IndexEndpoint. If set, this IndexEndpoint and all sub-resources of this IndexEndpoint will be secured by this key. |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `public_endpoint_domain_name` | String | Output only. If public_endpoint_enabled is true, this field will be populated with the domain name to use for this index endpoint. |
| `enable_private_service_connect` | bool | Optional. Deprecated: If true, expose the IndexEndpoint via private service connect. Only one of the fields, network or enable_private_service_connect, can be set. |
| `public_endpoint_enabled` | bool | Optional. If true, the deployed index will be accessible through public endpoint. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your IndexEndpoints. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `display_name` | String | Required. The display name of the IndexEndpoint. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `update_time` | String | Output only. Timestamp when this IndexEndpoint was last updated. This timestamp is not updated when the endpoint's DeployedIndexes are updated, e.g. due to updates of the original Indexes they are the deployments of. |
| `network` | String | Optional. The full name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the IndexEndpoint should be peered. Private services access must already be configured for the network. If left unspecified, the Endpoint is not peered with any network. network and private_service_connect_config are mutually exclusive. [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): `projects/{project}/global/networks/{network}`. Where {project} is a project number, as in '12345', and {network} is network name. |
| `create_time` | String | Output only. Timestamp when this IndexEndpoint was created. |
| `name` | String | Output only. The resource name of the IndexEndpoint. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `description` | String | The description of the IndexEndpoint. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create index_endpoint
index_endpoint = provider.aiplatform_api.Index_endpoint {
    parent = "value"  # Required. The resource name of the Location to create the IndexEndpoint in. Format: `projects/{project}/locations/{location}`
}

# Access index_endpoint outputs
index_endpoint_id = index_endpoint.id
index_endpoint_deployed_indexes = index_endpoint.deployed_indexes
index_endpoint_private_service_connect_config = index_endpoint.private_service_connect_config
index_endpoint_encryption_spec = index_endpoint.encryption_spec
index_endpoint_etag = index_endpoint.etag
index_endpoint_public_endpoint_domain_name = index_endpoint.public_endpoint_domain_name
index_endpoint_enable_private_service_connect = index_endpoint.enable_private_service_connect
index_endpoint_public_endpoint_enabled = index_endpoint.public_endpoint_enabled
index_endpoint_labels = index_endpoint.labels
index_endpoint_display_name = index_endpoint.display_name
index_endpoint_update_time = index_endpoint.update_time
index_endpoint_network = index_endpoint.network
index_endpoint_create_time = index_endpoint.create_time
index_endpoint_name = index_endpoint.name
index_endpoint_satisfies_pzs = index_endpoint.satisfies_pzs
index_endpoint_description = index_endpoint.description
index_endpoint_satisfies_pzi = index_endpoint.satisfies_pzi
```

---


### Notebook_runtime_template

Creates a NotebookRuntimeTemplate.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_persistent_disk_spec` | String |  | Optional. The specification of persistent disk attached to the runtime as data disk storage. |
| `description` | String |  | The description of the NotebookRuntimeTemplate. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for the notebook runtime. |
| `reservation_affinity` | String |  | Optional. Reservation Affinity of the notebook runtime template. |
| `software_config` | String |  | Optional. The notebook software configuration of the notebook runtime. |
| `update_time` | String |  | Output only. Timestamp when this NotebookRuntimeTemplate was most recently updated. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize the NotebookRuntimeTemplates. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `service_account` | String |  | Deprecated: This field is ignored and the "Vertex AI Notebook Service Account" (service-PROJECT_NUMBER@gcp-sa-aiplatform-vm.iam.gserviceaccount.com) is used for the runtime workload identity. See https://cloud.google.com/iam/docs/service-agents#vertex-ai-notebook-service-account for more details. For NotebookExecutionJob, use NotebookExecutionJob.service_account instead. The service account that the runtime workload runs as. You can use any service account within the same project, but you must have the service account user permission to use the instance. If not specified, the [Compute Engine default service account](https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used. |
| `shielded_vm_config` | String |  | Optional. Immutable. Runtime Shielded VM spec. |
| `network_tags` | Vec<String> |  | Optional. The Compute Engine tags to add to runtime (see [Tagging instances](https://cloud.google.com/vpc/docs/add-remove-network-tags)). |
| `notebook_runtime_type` | String |  | Optional. Immutable. The type of the notebook runtime template. |
| `create_time` | String |  | Output only. Timestamp when this NotebookRuntimeTemplate was created. |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `euc_config` | String |  | EUC configuration of the NotebookRuntimeTemplate. |
| `network_spec` | String |  | Optional. Network spec. |
| `idle_shutdown_config` | String |  | The idle shutdown configuration of NotebookRuntimeTemplate. This config will only be set when idle shutdown is enabled. |
| `display_name` | String |  | Required. The display name of the NotebookRuntimeTemplate. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `is_default` | bool |  | Output only. Deprecated: This field has no behavior. Use notebook_runtime_type = 'ONE_CLICK' instead. The default template to use if not specified. |
| `name` | String |  | The resource name of the NotebookRuntimeTemplate. |
| `machine_spec` | String |  | Optional. Immutable. The specification of a single machine for the template. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the NotebookRuntimeTemplate. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_persistent_disk_spec` | String | Optional. The specification of persistent disk attached to the runtime as data disk storage. |
| `description` | String | The description of the NotebookRuntimeTemplate. |
| `encryption_spec` | String | Customer-managed encryption key spec for the notebook runtime. |
| `reservation_affinity` | String | Optional. Reservation Affinity of the notebook runtime template. |
| `software_config` | String | Optional. The notebook software configuration of the notebook runtime. |
| `update_time` | String | Output only. Timestamp when this NotebookRuntimeTemplate was most recently updated. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize the NotebookRuntimeTemplates. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `service_account` | String | Deprecated: This field is ignored and the "Vertex AI Notebook Service Account" (service-PROJECT_NUMBER@gcp-sa-aiplatform-vm.iam.gserviceaccount.com) is used for the runtime workload identity. See https://cloud.google.com/iam/docs/service-agents#vertex-ai-notebook-service-account for more details. For NotebookExecutionJob, use NotebookExecutionJob.service_account instead. The service account that the runtime workload runs as. You can use any service account within the same project, but you must have the service account user permission to use the instance. If not specified, the [Compute Engine default service account](https://cloud.google.com/compute/docs/access/service-accounts#default_service_account) is used. |
| `shielded_vm_config` | String | Optional. Immutable. Runtime Shielded VM spec. |
| `network_tags` | Vec<String> | Optional. The Compute Engine tags to add to runtime (see [Tagging instances](https://cloud.google.com/vpc/docs/add-remove-network-tags)). |
| `notebook_runtime_type` | String | Optional. Immutable. The type of the notebook runtime template. |
| `create_time` | String | Output only. Timestamp when this NotebookRuntimeTemplate was created. |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `euc_config` | String | EUC configuration of the NotebookRuntimeTemplate. |
| `network_spec` | String | Optional. Network spec. |
| `idle_shutdown_config` | String | The idle shutdown configuration of NotebookRuntimeTemplate. This config will only be set when idle shutdown is enabled. |
| `display_name` | String | Required. The display name of the NotebookRuntimeTemplate. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `is_default` | bool | Output only. Deprecated: This field has no behavior. Use notebook_runtime_type = 'ONE_CLICK' instead. The default template to use if not specified. |
| `name` | String | The resource name of the NotebookRuntimeTemplate. |
| `machine_spec` | String | Optional. Immutable. The specification of a single machine for the template. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notebook_runtime_template
notebook_runtime_template = provider.aiplatform_api.Notebook_runtime_template {
    parent = "value"  # Required. The resource name of the Location to create the NotebookRuntimeTemplate. Format: `projects/{project}/locations/{location}`
}

# Access notebook_runtime_template outputs
notebook_runtime_template_id = notebook_runtime_template.id
notebook_runtime_template_data_persistent_disk_spec = notebook_runtime_template.data_persistent_disk_spec
notebook_runtime_template_description = notebook_runtime_template.description
notebook_runtime_template_encryption_spec = notebook_runtime_template.encryption_spec
notebook_runtime_template_reservation_affinity = notebook_runtime_template.reservation_affinity
notebook_runtime_template_software_config = notebook_runtime_template.software_config
notebook_runtime_template_update_time = notebook_runtime_template.update_time
notebook_runtime_template_labels = notebook_runtime_template.labels
notebook_runtime_template_service_account = notebook_runtime_template.service_account
notebook_runtime_template_shielded_vm_config = notebook_runtime_template.shielded_vm_config
notebook_runtime_template_network_tags = notebook_runtime_template.network_tags
notebook_runtime_template_notebook_runtime_type = notebook_runtime_template.notebook_runtime_type
notebook_runtime_template_create_time = notebook_runtime_template.create_time
notebook_runtime_template_etag = notebook_runtime_template.etag
notebook_runtime_template_euc_config = notebook_runtime_template.euc_config
notebook_runtime_template_network_spec = notebook_runtime_template.network_spec
notebook_runtime_template_idle_shutdown_config = notebook_runtime_template.idle_shutdown_config
notebook_runtime_template_display_name = notebook_runtime_template.display_name
notebook_runtime_template_is_default = notebook_runtime_template.is_default
notebook_runtime_template_name = notebook_runtime_template.name
notebook_runtime_template_machine_spec = notebook_runtime_template.machine_spec
```

---


### Dataset

Creates a Dataset.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Identifier. The resource name of the Dataset. Format: `projects/{project}/locations/{location}/datasets/{dataset}` |
| `saved_queries` | Vec<String> |  | All SavedQueries belong to the Dataset will be returned in List/Get Dataset response. The annotation_specs field will not be populated except for UI cases which will only use annotation_spec_count. In CreateDataset request, a SavedQuery is created together if this field is set, up to one SavedQuery can be set in CreateDatasetRequest. The SavedQuery should not contain any AnnotationSpec. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this Dataset was last updated. |
| `data_item_count` | String |  | Output only. The number of DataItems in this Dataset. Only apply for non-structured Dataset. |
| `display_name` | String |  | Required. The user-defined name of the Dataset. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a Dataset. If set, this Dataset and all sub-resources of this Dataset will be secured by this key. |
| `metadata_artifact` | String |  | Output only. The resource name of the Artifact that was created in MetadataStore when creating the Dataset. The Artifact resource name pattern is `projects/{project}/locations/{location}/metadataStores/{metadata_store}/artifacts/{artifact}`. |
| `metadata` | String |  | Required. Additional information about the Dataset. |
| `description` | String |  | The description of the Dataset. |
| `metadata_schema_uri` | String |  | Required. Points to a YAML file stored on Google Cloud Storage describing additional information about the Dataset. The schema is defined as an OpenAPI 3.0.2 Schema Object. The schema files that can be used here are found in gs://google-cloud-aiplatform/schema/dataset/metadata/. |
| `model_reference` | String |  | Optional. Reference to the public base model last used by the dataset. Only set for prompt datasets. |
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Datasets. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Dataset (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. Following system labels exist for each Dataset: * "aiplatform.googleapis.com/dataset_metadata_schema": output only, its value is the metadata_schema's title. |
| `create_time` | String |  | Output only. Timestamp when this Dataset was created. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Dataset in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Identifier. The resource name of the Dataset. Format: `projects/{project}/locations/{location}/datasets/{dataset}` |
| `saved_queries` | Vec<String> | All SavedQueries belong to the Dataset will be returned in List/Get Dataset response. The annotation_specs field will not be populated except for UI cases which will only use annotation_spec_count. In CreateDataset request, a SavedQuery is created together if this field is set, up to one SavedQuery can be set in CreateDatasetRequest. The SavedQuery should not contain any AnnotationSpec. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this Dataset was last updated. |
| `data_item_count` | String | Output only. The number of DataItems in this Dataset. Only apply for non-structured Dataset. |
| `display_name` | String | Required. The user-defined name of the Dataset. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `encryption_spec` | String | Customer-managed encryption key spec for a Dataset. If set, this Dataset and all sub-resources of this Dataset will be secured by this key. |
| `metadata_artifact` | String | Output only. The resource name of the Artifact that was created in MetadataStore when creating the Dataset. The Artifact resource name pattern is `projects/{project}/locations/{location}/metadataStores/{metadata_store}/artifacts/{artifact}`. |
| `metadata` | String | Required. Additional information about the Dataset. |
| `description` | String | The description of the Dataset. |
| `metadata_schema_uri` | String | Required. Points to a YAML file stored on Google Cloud Storage describing additional information about the Dataset. The schema is defined as an OpenAPI 3.0.2 Schema Object. The schema files that can be used here are found in gs://google-cloud-aiplatform/schema/dataset/metadata/. |
| `model_reference` | String | Optional. Reference to the public base model last used by the dataset. Only set for prompt datasets. |
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Datasets. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Dataset (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. Following system labels exist for each Dataset: * "aiplatform.googleapis.com/dataset_metadata_schema": output only, its value is the metadata_schema's title. |
| `create_time` | String | Output only. Timestamp when this Dataset was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset
dataset = provider.aiplatform_api.Dataset {
    parent = "value"  # Required. The resource name of the Location to create the Dataset in. Format: `projects/{project}/locations/{location}`
}

# Access dataset outputs
dataset_id = dataset.id
dataset_name = dataset.name
dataset_saved_queries = dataset.saved_queries
dataset_satisfies_pzi = dataset.satisfies_pzi
dataset_update_time = dataset.update_time
dataset_data_item_count = dataset.data_item_count
dataset_display_name = dataset.display_name
dataset_satisfies_pzs = dataset.satisfies_pzs
dataset_encryption_spec = dataset.encryption_spec
dataset_metadata_artifact = dataset.metadata_artifact
dataset_metadata = dataset.metadata
dataset_description = dataset.description
dataset_metadata_schema_uri = dataset.metadata_schema_uri
dataset_model_reference = dataset.model_reference
dataset_etag = dataset.etag
dataset_labels = dataset.labels
dataset_create_time = dataset.create_time
```

---


### Notebook_runtime

Stops a NotebookRuntime.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the NotebookRuntime resource to be stopped. Instead of checking whether the name is in valid NotebookRuntime resource name format, directly throw NotFound exception if there is no such NotebookRuntime in spanner. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network_spec` | String | Output only. Network spec of the notebook runtime. |
| `notebook_runtime_type` | String | Output only. The type of the notebook runtime. |
| `software_config` | String | Output only. Software config of the notebook runtime. |
| `encryption_spec` | String | Output only. Customer-managed encryption key spec for the notebook runtime. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `display_name` | String | Required. The display name of the NotebookRuntime. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `shielded_vm_config` | String | Output only. Runtime Shielded VM spec. |
| `version` | String | Output only. The VM os image version of NotebookRuntime. |
| `health_state` | String | Output only. The health state of the NotebookRuntime. |
| `reservation_affinity` | String | Output only. Reservation Affinity of the notebook runtime. |
| `update_time` | String | Output only. Timestamp when this NotebookRuntime was most recently updated. |
| `data_persistent_disk_spec` | String | Output only. The specification of persistent disk attached to the notebook runtime as data disk storage. |
| `idle_shutdown_config` | String | Output only. The idle shutdown configuration of the notebook runtime. |
| `machine_spec` | String | Output only. The specification of a single machine used by the notebook runtime. |
| `description` | String | The description of the NotebookRuntime. |
| `runtime_state` | String | Output only. The runtime (instance) state of the NotebookRuntime. |
| `runtime_user` | String | Required. The user email of the NotebookRuntime. |
| `is_upgradable` | bool | Output only. Whether NotebookRuntime is upgradable. |
| `proxy_uri` | String | Output only. The proxy endpoint used to access the NotebookRuntime. |
| `euc_config` | String | Output only. EUC configuration of the notebook runtime. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `notebook_runtime_template_ref` | String | Output only. The pointer to NotebookRuntimeTemplate this NotebookRuntime is created from. |
| `service_account` | String | Output only. Deprecated: This field is no longer used and the "Vertex AI Notebook Service Account" (service-PROJECT_NUMBER@gcp-sa-aiplatform-vm.iam.gserviceaccount.com) is used for the runtime workload identity. See https://cloud.google.com/iam/docs/service-agents#vertex-ai-notebook-service-account for more details. The service account that the NotebookRuntime workload runs as. |
| `create_time` | String | Output only. Timestamp when this NotebookRuntime was created. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your NotebookRuntime. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one NotebookRuntime (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. Following system labels exist for NotebookRuntime: * "aiplatform.googleapis.com/notebook_runtime_gce_instance_id": output only, its value is the Compute Engine instance id. * "aiplatform.googleapis.com/colab_enterprise_entry_service": its value is either "bigquery" or "vertex"; if absent, it should be "vertex". This is to describe the entry service, either BigQuery or Vertex. |
| `network_tags` | Vec<String> | Optional. The Compute Engine tags to add to runtime (see [Tagging instances](https://cloud.google.com/vpc/docs/add-remove-network-tags)). |
| `expiration_time` | String | Output only. Timestamp when this NotebookRuntime will be expired: 1. System Predefined NotebookRuntime: 24 hours after creation. After expiration, system predifined runtime will be deleted. 2. User created NotebookRuntime: 6 months after last upgrade. After expiration, user created runtime will be stopped and allowed for upgrade. |
| `name` | String | Output only. The resource name of the NotebookRuntime. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create notebook_runtime
notebook_runtime = provider.aiplatform_api.Notebook_runtime {
    name = "value"  # Required. The name of the NotebookRuntime resource to be stopped. Instead of checking whether the name is in valid NotebookRuntime resource name format, directly throw NotFound exception if there is no such NotebookRuntime in spanner.
}

# Access notebook_runtime outputs
notebook_runtime_id = notebook_runtime.id
notebook_runtime_network_spec = notebook_runtime.network_spec
notebook_runtime_notebook_runtime_type = notebook_runtime.notebook_runtime_type
notebook_runtime_software_config = notebook_runtime.software_config
notebook_runtime_encryption_spec = notebook_runtime.encryption_spec
notebook_runtime_satisfies_pzs = notebook_runtime.satisfies_pzs
notebook_runtime_display_name = notebook_runtime.display_name
notebook_runtime_shielded_vm_config = notebook_runtime.shielded_vm_config
notebook_runtime_version = notebook_runtime.version
notebook_runtime_health_state = notebook_runtime.health_state
notebook_runtime_reservation_affinity = notebook_runtime.reservation_affinity
notebook_runtime_update_time = notebook_runtime.update_time
notebook_runtime_data_persistent_disk_spec = notebook_runtime.data_persistent_disk_spec
notebook_runtime_idle_shutdown_config = notebook_runtime.idle_shutdown_config
notebook_runtime_machine_spec = notebook_runtime.machine_spec
notebook_runtime_description = notebook_runtime.description
notebook_runtime_runtime_state = notebook_runtime.runtime_state
notebook_runtime_runtime_user = notebook_runtime.runtime_user
notebook_runtime_is_upgradable = notebook_runtime.is_upgradable
notebook_runtime_proxy_uri = notebook_runtime.proxy_uri
notebook_runtime_euc_config = notebook_runtime.euc_config
notebook_runtime_satisfies_pzi = notebook_runtime.satisfies_pzi
notebook_runtime_notebook_runtime_template_ref = notebook_runtime.notebook_runtime_template_ref
notebook_runtime_service_account = notebook_runtime.service_account
notebook_runtime_create_time = notebook_runtime.create_time
notebook_runtime_labels = notebook_runtime.labels
notebook_runtime_network_tags = notebook_runtime.network_tags
notebook_runtime_expiration_time = notebook_runtime.expiration_time
notebook_runtime_name = notebook_runtime.name
```

---


### Project

Sets (creates or updates) configs of publisher models. For example, sets the request/response logging config.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `publisher_model_config` | String |  | Required. The publisher model config. |
| `name` | String | ✅ | Required. The name of the publisher model, in the format of `projects/{project}/locations/{location}/publishers/{publisher}/models/{model}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `logging_config` | String | The prediction request/response logging config. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.aiplatform_api.Project {
    name = "value"  # Required. The name of the publisher model, in the format of `projects/{project}/locations/{location}/publishers/{publisher}/models/{model}`.
}

# Access project outputs
project_id = project.id
project_logging_config = project.logging_config
```

---


### Rag_corpora

Creates a RagCorpus.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `corpus_type_config` | String |  | Optional. The corpus type config of the RagCorpus. |
| `vector_db_config` | String |  | Optional. Immutable. The config for the Vector DBs. |
| `update_time` | String |  | Output only. Timestamp when this RagCorpus was last updated. |
| `corpus_status` | String |  | Output only. RagCorpus state. |
| `description` | String |  | Optional. The description of the RagCorpus. |
| `rag_files_count` | i64 |  | Output only. Number of RagFiles in the RagCorpus. NOTE: This field is not populated in the response of VertexRagDataService.ListRagCorpora. |
| `create_time` | String |  | Output only. Timestamp when this RagCorpus was created. |
| `encryption_spec` | String |  | Optional. Immutable. The CMEK key name used to encrypt at-rest data related to this Corpus. Only applicable to RagManagedDb option for Vector DB. This field can only be set at corpus creation time, and cannot be updated or deleted. |
| `name` | String |  | Output only. The resource name of the RagCorpus. |
| `rag_embedding_model_config` | String |  | Optional. Immutable. The embedding model config of the RagCorpus. |
| `display_name` | String |  | Required. The display name of the RagCorpus. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `rag_vector_db_config` | String |  | Optional. Immutable. The Vector DB config of the RagCorpus. |
| `vertex_ai_search_config` | String |  | Optional. Immutable. The config for the Vertex AI Search. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the RagCorpus in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `corpus_type_config` | String | Optional. The corpus type config of the RagCorpus. |
| `vector_db_config` | String | Optional. Immutable. The config for the Vector DBs. |
| `update_time` | String | Output only. Timestamp when this RagCorpus was last updated. |
| `corpus_status` | String | Output only. RagCorpus state. |
| `description` | String | Optional. The description of the RagCorpus. |
| `rag_files_count` | i64 | Output only. Number of RagFiles in the RagCorpus. NOTE: This field is not populated in the response of VertexRagDataService.ListRagCorpora. |
| `create_time` | String | Output only. Timestamp when this RagCorpus was created. |
| `encryption_spec` | String | Optional. Immutable. The CMEK key name used to encrypt at-rest data related to this Corpus. Only applicable to RagManagedDb option for Vector DB. This field can only be set at corpus creation time, and cannot be updated or deleted. |
| `name` | String | Output only. The resource name of the RagCorpus. |
| `rag_embedding_model_config` | String | Optional. Immutable. The embedding model config of the RagCorpus. |
| `display_name` | String | Required. The display name of the RagCorpus. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `rag_vector_db_config` | String | Optional. Immutable. The Vector DB config of the RagCorpus. |
| `vertex_ai_search_config` | String | Optional. Immutable. The config for the Vertex AI Search. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rag_corpora
rag_corpora = provider.aiplatform_api.Rag_corpora {
    parent = "value"  # Required. The resource name of the Location to create the RagCorpus in. Format: `projects/{project}/locations/{location}`
}

# Access rag_corpora outputs
rag_corpora_id = rag_corpora.id
rag_corpora_corpus_type_config = rag_corpora.corpus_type_config
rag_corpora_vector_db_config = rag_corpora.vector_db_config
rag_corpora_update_time = rag_corpora.update_time
rag_corpora_corpus_status = rag_corpora.corpus_status
rag_corpora_description = rag_corpora.description
rag_corpora_rag_files_count = rag_corpora.rag_files_count
rag_corpora_create_time = rag_corpora.create_time
rag_corpora_encryption_spec = rag_corpora.encryption_spec
rag_corpora_name = rag_corpora.name
rag_corpora_rag_embedding_model_config = rag_corpora.rag_embedding_model_config
rag_corpora_display_name = rag_corpora.display_name
rag_corpora_rag_vector_db_config = rag_corpora.rag_vector_db_config
rag_corpora_vertex_ai_search_config = rag_corpora.vertex_ai_search_config
```

---


### Indexe

Creates an Index.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `display_name` | String |  | Required. The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `encryption_spec` | String |  | Immutable. Customer-managed encryption key spec for an Index. If set, this Index and all sub-resources of this Index will be secured by this key. |
| `update_time` | String |  | Output only. Timestamp when this Index was most recently updated. This also includes any update to the contents of the Index. Note that Operations working on this Index may have their Operations.metadata.generic_metadata.update_time a little after the value of this timestamp, yet that does not mean their results are not already reflected in the Index. Result of any successfully completed Operation on the Index is reflected in it. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Indexes. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `metadata_schema_uri` | String |  | Immutable. Points to a YAML file stored on Google Cloud Storage describing additional information about the Index, that is specific to it. Unset if the Index does not have any additional information. The schema is defined as an OpenAPI 3.0.2 [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.2.md#schemaObject). Note: The URI given on output will be immutable and probably different, including the URI scheme, than the one given on input. The output URI will point to a location where the user only has a read access. |
| `deployed_indexes` | Vec<String> |  | Output only. The pointers to DeployedIndexes created from this Index. An Index can be only deleted if all its DeployedIndexes had been undeployed first. |
| `description` | String |  | The description of the Index. |
| `create_time` | String |  | Output only. Timestamp when this Index was created. |
| `index_stats` | String |  | Output only. Stats of the index resource. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `index_update_method` | String |  | Immutable. The update method to use with this Index. If not set, BATCH_UPDATE will be used by default. |
| `metadata` | String |  | An additional information about the Index; the schema of the metadata can be found in metadata_schema. |
| `name` | String |  | Output only. The resource name of the Index. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the Index in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `display_name` | String | Required. The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `encryption_spec` | String | Immutable. Customer-managed encryption key spec for an Index. If set, this Index and all sub-resources of this Index will be secured by this key. |
| `update_time` | String | Output only. Timestamp when this Index was most recently updated. This also includes any update to the contents of the Index. Note that Operations working on this Index may have their Operations.metadata.generic_metadata.update_time a little after the value of this timestamp, yet that does not mean their results are not already reflected in the Index. Result of any successfully completed Operation on the Index is reflected in it. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Indexes. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `metadata_schema_uri` | String | Immutable. Points to a YAML file stored on Google Cloud Storage describing additional information about the Index, that is specific to it. Unset if the Index does not have any additional information. The schema is defined as an OpenAPI 3.0.2 [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.2.md#schemaObject). Note: The URI given on output will be immutable and probably different, including the URI scheme, than the one given on input. The output URI will point to a location where the user only has a read access. |
| `deployed_indexes` | Vec<String> | Output only. The pointers to DeployedIndexes created from this Index. An Index can be only deleted if all its DeployedIndexes had been undeployed first. |
| `description` | String | The description of the Index. |
| `create_time` | String | Output only. Timestamp when this Index was created. |
| `index_stats` | String | Output only. Stats of the index resource. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `index_update_method` | String | Immutable. The update method to use with this Index. If not set, BATCH_UPDATE will be used by default. |
| `metadata` | String | An additional information about the Index; the schema of the metadata can be found in metadata_schema. |
| `name` | String | Output only. The resource name of the Index. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create indexe
indexe = provider.aiplatform_api.Indexe {
    parent = "value"  # Required. The resource name of the Location to create the Index in. Format: `projects/{project}/locations/{location}`
}

# Access indexe outputs
indexe_id = indexe.id
indexe_etag = indexe.etag
indexe_display_name = indexe.display_name
indexe_encryption_spec = indexe.encryption_spec
indexe_update_time = indexe.update_time
indexe_satisfies_pzs = indexe.satisfies_pzs
indexe_labels = indexe.labels
indexe_metadata_schema_uri = indexe.metadata_schema_uri
indexe_deployed_indexes = indexe.deployed_indexes
indexe_description = indexe.description
indexe_create_time = indexe.create_time
indexe_index_stats = indexe.index_stats
indexe_satisfies_pzi = indexe.satisfies_pzi
indexe_index_update_method = indexe.index_update_method
indexe_metadata = indexe.metadata
indexe_name = indexe.name
```

---


### Example_store

Create an ExampleStore.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | Required. The resource name of the Location to create the ExampleStore in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `example_store_config` | String | Required. Example Store config. |
| `create_time` | String | Output only. Timestamp when this ExampleStore was created. |
| `description` | String | Optional. Description of the ExampleStore. |
| `update_time` | String | Output only. Timestamp when this ExampleStore was most recently updated. |
| `display_name` | String | Required. Display name of the ExampleStore. |
| `name` | String | Identifier. The resource name of the ExampleStore. This is a unique identifier. Format: projects/{project}/locations/{location}/exampleStores/{example_store} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create example_store
example_store = provider.aiplatform_api.Example_store {
    parent = "value"  # Required. The resource name of the Location to create the ExampleStore in. Format: `projects/{project}/locations/{location}`
}

# Access example_store outputs
example_store_id = example_store.id
example_store_example_store_config = example_store.example_store_config
example_store_create_time = example_store.create_time
example_store_description = example_store.description
example_store_update_time = example_store.update_time
example_store_display_name = example_store.display_name
example_store_name = example_store.name
```

---


### Data_item

Lists DataItems in a Dataset.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The standard List next-page token. |
| `data_items` | Vec<String> | A list of DataItems that matches the specified filter in the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access data_item outputs
data_item_id = data_item.id
data_item_next_page_token = data_item.next_page_token
data_item_data_items = data_item.data_items
```

---


### Annotation

Lists Annotations belongs to a dataitem.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | Vec<String> | A list of Annotations that matches the specified filter in the request. |
| `next_page_token` | String | The standard List next-page token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access annotation outputs
annotation_id = annotation.id
annotation_annotations = annotation.annotations
annotation_next_page_token = annotation.next_page_token
```

---


### Model_monitoring_job

Creates a ModelMonitoringJob.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job_execution_detail` | String |  | Output only. Execution results for all the monitoring objectives. |
| `schedule` | String |  | Output only. Schedule resource name. It will only appear when this job is triggered by a schedule. |
| `create_time` | String |  | Output only. Timestamp when this ModelMonitoringJob was created. |
| `schedule_time` | String |  | Output only. Timestamp when this ModelMonitoringJob was scheduled. It will only appear when this job is triggered by a schedule. |
| `display_name` | String |  | The display name of the ModelMonitoringJob. The name can be up to 128 characters long and can consist of any UTF-8. |
| `state` | String |  | Output only. The state of the monitoring job. * When the job is still creating, the state will be 'JOB_STATE_PENDING'. * Once the job is successfully created, the state will be 'JOB_STATE_RUNNING'. * Once the job is finished, the state will be one of 'JOB_STATE_FAILED', 'JOB_STATE_SUCCEEDED', 'JOB_STATE_PARTIALLY_SUCCEEDED'. |
| `update_time` | String |  | Output only. Timestamp when this ModelMonitoringJob was updated most recently. |
| `model_monitoring_spec` | String |  | Monitoring monitoring job spec. It outlines the specifications for monitoring objectives, notifications, and result exports. If left blank, the default monitoring specifications from the top-level resource 'ModelMonitor' will be applied. If provided, we will use the specification defined here rather than the default one. |
| `name` | String |  | Output only. Resource name of a ModelMonitoringJob. Format: `projects/{project_id}/locations/{location_id}/modelMonitors/{model_monitor_id}/modelMonitoringJobs/{model_monitoring_job_id}` |
| `parent` | String | ✅ | Required. The parent of the ModelMonitoringJob. Format: `projects/{project}/locations/{location}/modelMoniitors/{model_monitor}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `job_execution_detail` | String | Output only. Execution results for all the monitoring objectives. |
| `schedule` | String | Output only. Schedule resource name. It will only appear when this job is triggered by a schedule. |
| `create_time` | String | Output only. Timestamp when this ModelMonitoringJob was created. |
| `schedule_time` | String | Output only. Timestamp when this ModelMonitoringJob was scheduled. It will only appear when this job is triggered by a schedule. |
| `display_name` | String | The display name of the ModelMonitoringJob. The name can be up to 128 characters long and can consist of any UTF-8. |
| `state` | String | Output only. The state of the monitoring job. * When the job is still creating, the state will be 'JOB_STATE_PENDING'. * Once the job is successfully created, the state will be 'JOB_STATE_RUNNING'. * Once the job is finished, the state will be one of 'JOB_STATE_FAILED', 'JOB_STATE_SUCCEEDED', 'JOB_STATE_PARTIALLY_SUCCEEDED'. |
| `update_time` | String | Output only. Timestamp when this ModelMonitoringJob was updated most recently. |
| `model_monitoring_spec` | String | Monitoring monitoring job spec. It outlines the specifications for monitoring objectives, notifications, and result exports. If left blank, the default monitoring specifications from the top-level resource 'ModelMonitor' will be applied. If provided, we will use the specification defined here rather than the default one. |
| `name` | String | Output only. Resource name of a ModelMonitoringJob. Format: `projects/{project_id}/locations/{location_id}/modelMonitors/{model_monitor_id}/modelMonitoringJobs/{model_monitoring_job_id}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model_monitoring_job
model_monitoring_job = provider.aiplatform_api.Model_monitoring_job {
    parent = "value"  # Required. The parent of the ModelMonitoringJob. Format: `projects/{project}/locations/{location}/modelMoniitors/{model_monitor}`
}

# Access model_monitoring_job outputs
model_monitoring_job_id = model_monitoring_job.id
model_monitoring_job_job_execution_detail = model_monitoring_job.job_execution_detail
model_monitoring_job_schedule = model_monitoring_job.schedule
model_monitoring_job_create_time = model_monitoring_job.create_time
model_monitoring_job_schedule_time = model_monitoring_job.schedule_time
model_monitoring_job_display_name = model_monitoring_job.display_name
model_monitoring_job_state = model_monitoring_job.state
model_monitoring_job_update_time = model_monitoring_job.update_time
model_monitoring_job_model_monitoring_spec = model_monitoring_job.model_monitoring_spec
model_monitoring_job_name = model_monitoring_job.name
```

---


### Execution

Creates an Execution associated with a MetadataStore.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_title` | String |  | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `metadata` | HashMap<String, String> |  | Properties of the Execution. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |
| `create_time` | String |  | Output only. Timestamp when this Execution was created. |
| `schema_version` | String |  | The version of the schema in `schema_title` to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `update_time` | String |  | Output only. Timestamp when this Execution was last updated. |
| `state` | String |  | The state of this Execution. This is a property of the Execution, and does not imply or capture any ongoing process. This property is managed by clients (such as Vertex AI Pipelines) and the system does not prescribe or check the validity of state transitions. |
| `display_name` | String |  | User provided display name of the Execution. May be up to 128 Unicode characters. |
| `name` | String |  | Output only. The resource name of the Execution. |
| `etag` | String |  | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `description` | String |  | Description of the Execution |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your Executions. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Execution (System labels are excluded). |
| `parent` | String | ✅ | Required. The resource name of the MetadataStore where the Execution should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema_title` | String | The title of the schema describing the metadata. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `metadata` | HashMap<String, String> | Properties of the Execution. Top level metadata keys' heading and trailing spaces will be trimmed. The size of this field should not exceed 200KB. |
| `create_time` | String | Output only. Timestamp when this Execution was created. |
| `schema_version` | String | The version of the schema in `schema_title` to use. Schema title and version is expected to be registered in earlier Create Schema calls. And both are used together as unique identifiers to identify schemas within the local metadata store. |
| `update_time` | String | Output only. Timestamp when this Execution was last updated. |
| `state` | String | The state of this Execution. This is a property of the Execution, and does not imply or capture any ongoing process. This property is managed by clients (such as Vertex AI Pipelines) and the system does not prescribe or check the validity of state transitions. |
| `display_name` | String | User provided display name of the Execution. May be up to 128 Unicode characters. |
| `name` | String | Output only. The resource name of the Execution. |
| `etag` | String | An eTag used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `description` | String | Description of the Execution |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your Executions. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one Execution (System labels are excluded). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create execution
execution = provider.aiplatform_api.Execution {
    parent = "value"  # Required. The resource name of the MetadataStore where the Execution should be created. Format: `projects/{project}/locations/{location}/metadataStores/{metadatastore}`
}

# Access execution outputs
execution_id = execution.id
execution_schema_title = execution.schema_title
execution_metadata = execution.metadata
execution_create_time = execution.create_time
execution_schema_version = execution.schema_version
execution_update_time = execution.update_time
execution_state = execution.state
execution_display_name = execution.display_name
execution_name = execution.name
execution_etag = execution.etag
execution_description = execution.description
execution_labels = execution.labels
```

---


### Batch_prediction_job

Creates a BatchPredictionJob. A BatchPredictionJob once created will right away be attempted to start.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disable_container_logging` | bool |  | For custom-trained Models and AutoML Tabular Models, the container of the DeployedModel instances will send `stderr` and `stdout` streams to Cloud Logging by default. Please note that the logs incur cost, which are subject to [Cloud Logging pricing](https://cloud.google.com/logging/pricing). User can disable container logging by setting this flag to true. |
| `unmanaged_container_model` | String |  | Contains model information necessary to perform batch prediction without requiring uploading to model registry. Exactly one of model and unmanaged_container_model must be set. |
| `instance_config` | String |  | Configuration for how to convert batch prediction input instances to the prediction instances that are sent to the Model. |
| `display_name` | String |  | Required. The user-defined name of this BatchPredictionJob. |
| `model_monitoring_status` | String |  | Output only. The running status of the model monitoring pipeline. |
| `manual_batch_tuning_parameters` | String |  | Immutable. Parameters configuring the batch behavior. Currently only applicable when dedicated_resources are used (in other cases Vertex AI does the tuning itself). |
| `encryption_spec` | String |  | Customer-managed encryption key options for a BatchPredictionJob. If this is set, then all resources created by the BatchPredictionJob will be encrypted with the provided encryption key. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `model` | String |  | The name of the Model resource that produces the predictions via this job, must share the same ancestor Location. Starting this job has no impact on any existing deployments of the Model and their resources. Exactly one of model and unmanaged_container_model must be set. The model resource name may contain version id or version alias to specify the version. Example: `projects/{project}/locations/{location}/models/{model}@2` or `projects/{project}/locations/{location}/models/{model}@golden` if no version is specified, the default version will be deployed. The model resource could also be a publisher model. Example: `publishers/{publisher}/models/{model}` or `projects/{project}/locations/{location}/publishers/{publisher}/models/{model}` |
| `state` | String |  | Output only. The detailed state of the job. |
| `output_info` | String |  | Output only. Information further describing the output of this job. |
| `end_time` | String |  | Output only. Time when the BatchPredictionJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `name` | String |  | Output only. Resource name of the BatchPredictionJob. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `model_parameters` | String |  | The parameters that govern the predictions. The schema of the parameters may be specified via the Model's PredictSchemata's parameters_schema_uri. |
| `error` | String |  | Output only. Only populated when the job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `model_monitoring_config` | String |  | Model monitoring config will be used for analysis model behaviors, based on the input and output to the batch prediction job, as well as the provided training dataset. |
| `generate_explanation` | bool |  | Generate explanation with the batch prediction results. When set to `true`, the batch prediction output changes based on the `predictions_format` field of the BatchPredictionJob.output_config object: * `bigquery`: output includes a column named `explanation`. The value is a struct that conforms to the Explanation object. * `jsonl`: The JSON objects on each line include an additional entry keyed `explanation`. The value of the entry is a JSON object that conforms to the Explanation object. * `csv`: Generating explanations for CSV format is not supported. If this field is set to true, either the Model.explanation_spec or explanation_spec must be populated. |
| `dedicated_resources` | String |  | The config of resources used by the Model during the batch prediction. If the Model supports DEDICATED_RESOURCES this config may be provided (and the job will use these resources), if the Model doesn't support AUTOMATIC_RESOURCES, this config must be provided. |
| `completion_stats` | String |  | Output only. Statistics on completed and failed prediction instances. |
| `output_config` | String |  | Required. The Configuration specifying where output predictions should be written. The schema of any single prediction may be specified as a concatenation of Model's PredictSchemata's instance_schema_uri and prediction_schema_uri. |
| `resources_consumed` | String |  | Output only. Information about resources that had been consumed by this job. Provided in real time at best effort basis, as well as a final value once the job completes. Note: This field currently may be not populated for batch predictions that use AutoML Models. |
| `start_time` | String |  | Output only. Time when the BatchPredictionJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `model_monitoring_stats_anomalies` | Vec<String> |  | Get batch prediction job monitoring statistics. |
| `partial_failures` | Vec<String> |  | Output only. Partial failures encountered. For example, single files that can't be read. This field never exceeds 20 entries. Status details fields contain standard Google Cloud error details. |
| `create_time` | String |  | Output only. Time when the BatchPredictionJob was created. |
| `model_version_id` | String |  | Output only. The version ID of the Model that produces the predictions via this job. |
| `update_time` | String |  | Output only. Time when the BatchPredictionJob was most recently updated. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize BatchPredictionJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `service_account` | String |  | The service account that the DeployedModel's container runs as. If not specified, a system generated one will be used, which has minimal permissions and the custom container, if used, may not have enough permission to access other Google Cloud resources. Users deploying the Model must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `input_config` | String |  | Required. Input configuration of the instances on which predictions are performed. The schema of any single instance may be specified via the Model's PredictSchemata's instance_schema_uri. |
| `explanation_spec` | String |  | Explanation configuration for this BatchPredictionJob. Can be specified only if generate_explanation is set to `true`. This value overrides the value of Model.explanation_spec. All fields of explanation_spec are optional in the request. If a field of the explanation_spec object is not populated, the corresponding field of the Model.explanation_spec object is inherited. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disable_container_logging` | bool | For custom-trained Models and AutoML Tabular Models, the container of the DeployedModel instances will send `stderr` and `stdout` streams to Cloud Logging by default. Please note that the logs incur cost, which are subject to [Cloud Logging pricing](https://cloud.google.com/logging/pricing). User can disable container logging by setting this flag to true. |
| `unmanaged_container_model` | String | Contains model information necessary to perform batch prediction without requiring uploading to model registry. Exactly one of model and unmanaged_container_model must be set. |
| `instance_config` | String | Configuration for how to convert batch prediction input instances to the prediction instances that are sent to the Model. |
| `display_name` | String | Required. The user-defined name of this BatchPredictionJob. |
| `model_monitoring_status` | String | Output only. The running status of the model monitoring pipeline. |
| `manual_batch_tuning_parameters` | String | Immutable. Parameters configuring the batch behavior. Currently only applicable when dedicated_resources are used (in other cases Vertex AI does the tuning itself). |
| `encryption_spec` | String | Customer-managed encryption key options for a BatchPredictionJob. If this is set, then all resources created by the BatchPredictionJob will be encrypted with the provided encryption key. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `model` | String | The name of the Model resource that produces the predictions via this job, must share the same ancestor Location. Starting this job has no impact on any existing deployments of the Model and their resources. Exactly one of model and unmanaged_container_model must be set. The model resource name may contain version id or version alias to specify the version. Example: `projects/{project}/locations/{location}/models/{model}@2` or `projects/{project}/locations/{location}/models/{model}@golden` if no version is specified, the default version will be deployed. The model resource could also be a publisher model. Example: `publishers/{publisher}/models/{model}` or `projects/{project}/locations/{location}/publishers/{publisher}/models/{model}` |
| `state` | String | Output only. The detailed state of the job. |
| `output_info` | String | Output only. Information further describing the output of this job. |
| `end_time` | String | Output only. Time when the BatchPredictionJob entered any of the following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`. |
| `name` | String | Output only. Resource name of the BatchPredictionJob. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `model_parameters` | String | The parameters that govern the predictions. The schema of the parameters may be specified via the Model's PredictSchemata's parameters_schema_uri. |
| `error` | String | Output only. Only populated when the job's state is JOB_STATE_FAILED or JOB_STATE_CANCELLED. |
| `model_monitoring_config` | String | Model monitoring config will be used for analysis model behaviors, based on the input and output to the batch prediction job, as well as the provided training dataset. |
| `generate_explanation` | bool | Generate explanation with the batch prediction results. When set to `true`, the batch prediction output changes based on the `predictions_format` field of the BatchPredictionJob.output_config object: * `bigquery`: output includes a column named `explanation`. The value is a struct that conforms to the Explanation object. * `jsonl`: The JSON objects on each line include an additional entry keyed `explanation`. The value of the entry is a JSON object that conforms to the Explanation object. * `csv`: Generating explanations for CSV format is not supported. If this field is set to true, either the Model.explanation_spec or explanation_spec must be populated. |
| `dedicated_resources` | String | The config of resources used by the Model during the batch prediction. If the Model supports DEDICATED_RESOURCES this config may be provided (and the job will use these resources), if the Model doesn't support AUTOMATIC_RESOURCES, this config must be provided. |
| `completion_stats` | String | Output only. Statistics on completed and failed prediction instances. |
| `output_config` | String | Required. The Configuration specifying where output predictions should be written. The schema of any single prediction may be specified as a concatenation of Model's PredictSchemata's instance_schema_uri and prediction_schema_uri. |
| `resources_consumed` | String | Output only. Information about resources that had been consumed by this job. Provided in real time at best effort basis, as well as a final value once the job completes. Note: This field currently may be not populated for batch predictions that use AutoML Models. |
| `start_time` | String | Output only. Time when the BatchPredictionJob for the first time entered the `JOB_STATE_RUNNING` state. |
| `model_monitoring_stats_anomalies` | Vec<String> | Get batch prediction job monitoring statistics. |
| `partial_failures` | Vec<String> | Output only. Partial failures encountered. For example, single files that can't be read. This field never exceeds 20 entries. Status details fields contain standard Google Cloud error details. |
| `create_time` | String | Output only. Time when the BatchPredictionJob was created. |
| `model_version_id` | String | Output only. The version ID of the Model that produces the predictions via this job. |
| `update_time` | String | Output only. Time when the BatchPredictionJob was most recently updated. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize BatchPredictionJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. |
| `service_account` | String | The service account that the DeployedModel's container runs as. If not specified, a system generated one will be used, which has minimal permissions and the custom container, if used, may not have enough permission to access other Google Cloud resources. Users deploying the Model must have the `iam.serviceAccounts.actAs` permission on this service account. |
| `input_config` | String | Required. Input configuration of the instances on which predictions are performed. The schema of any single instance may be specified via the Model's PredictSchemata's instance_schema_uri. |
| `explanation_spec` | String | Explanation configuration for this BatchPredictionJob. Can be specified only if generate_explanation is set to `true`. This value overrides the value of Model.explanation_spec. All fields of explanation_spec are optional in the request. If a field of the explanation_spec object is not populated, the corresponding field of the Model.explanation_spec object is inherited. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create batch_prediction_job
batch_prediction_job = provider.aiplatform_api.Batch_prediction_job {
}

# Access batch_prediction_job outputs
batch_prediction_job_id = batch_prediction_job.id
batch_prediction_job_disable_container_logging = batch_prediction_job.disable_container_logging
batch_prediction_job_unmanaged_container_model = batch_prediction_job.unmanaged_container_model
batch_prediction_job_instance_config = batch_prediction_job.instance_config
batch_prediction_job_display_name = batch_prediction_job.display_name
batch_prediction_job_model_monitoring_status = batch_prediction_job.model_monitoring_status
batch_prediction_job_manual_batch_tuning_parameters = batch_prediction_job.manual_batch_tuning_parameters
batch_prediction_job_encryption_spec = batch_prediction_job.encryption_spec
batch_prediction_job_satisfies_pzs = batch_prediction_job.satisfies_pzs
batch_prediction_job_model = batch_prediction_job.model
batch_prediction_job_state = batch_prediction_job.state
batch_prediction_job_output_info = batch_prediction_job.output_info
batch_prediction_job_end_time = batch_prediction_job.end_time
batch_prediction_job_name = batch_prediction_job.name
batch_prediction_job_satisfies_pzi = batch_prediction_job.satisfies_pzi
batch_prediction_job_model_parameters = batch_prediction_job.model_parameters
batch_prediction_job_error = batch_prediction_job.error
batch_prediction_job_model_monitoring_config = batch_prediction_job.model_monitoring_config
batch_prediction_job_generate_explanation = batch_prediction_job.generate_explanation
batch_prediction_job_dedicated_resources = batch_prediction_job.dedicated_resources
batch_prediction_job_completion_stats = batch_prediction_job.completion_stats
batch_prediction_job_output_config = batch_prediction_job.output_config
batch_prediction_job_resources_consumed = batch_prediction_job.resources_consumed
batch_prediction_job_start_time = batch_prediction_job.start_time
batch_prediction_job_model_monitoring_stats_anomalies = batch_prediction_job.model_monitoring_stats_anomalies
batch_prediction_job_partial_failures = batch_prediction_job.partial_failures
batch_prediction_job_create_time = batch_prediction_job.create_time
batch_prediction_job_model_version_id = batch_prediction_job.model_version_id
batch_prediction_job_update_time = batch_prediction_job.update_time
batch_prediction_job_labels = batch_prediction_job.labels
batch_prediction_job_service_account = batch_prediction_job.service_account
batch_prediction_job_input_config = batch_prediction_job.input_config
batch_prediction_job_explanation_spec = batch_prediction_job.explanation_spec
```

---


### Run

Creates a TensorboardRun.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when this TensorboardRun was created. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your TensorboardRuns. This field will be used to filter and visualize Runs in the Tensorboard UI. For example, a Vertex AI training job can set a label aiplatform.googleapis.com/training_job_id=xxxxx to all the runs created within that job. An end user can set a label experiment_id=xxxxx for all the runs produced in a Jupyter notebook. These runs can be grouped by a label value and visualized together in the Tensorboard UI. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one TensorboardRun (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `name` | String |  | Output only. Name of the TensorboardRun. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}/runs/{run}` |
| `update_time` | String |  | Output only. Timestamp when this TensorboardRun was last updated. |
| `description` | String |  | Description of this TensorboardRun. |
| `etag` | String |  | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `display_name` | String |  | Required. User provided name of this TensorboardRun. This value must be unique among all TensorboardRuns belonging to the same parent TensorboardExperiment. |
| `parent` | String | ✅ | Required. The resource name of the TensorboardExperiment to create the TensorboardRun in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when this TensorboardRun was created. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your TensorboardRuns. This field will be used to filter and visualize Runs in the Tensorboard UI. For example, a Vertex AI training job can set a label aiplatform.googleapis.com/training_job_id=xxxxx to all the runs created within that job. An end user can set a label experiment_id=xxxxx for all the runs produced in a Jupyter notebook. These runs can be grouped by a label value and visualized together in the Tensorboard UI. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one TensorboardRun (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `name` | String | Output only. Name of the TensorboardRun. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}/runs/{run}` |
| `update_time` | String | Output only. Timestamp when this TensorboardRun was last updated. |
| `description` | String | Description of this TensorboardRun. |
| `etag` | String | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `display_name` | String | Required. User provided name of this TensorboardRun. This value must be unique among all TensorboardRuns belonging to the same parent TensorboardExperiment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create run
run = provider.aiplatform_api.Run {
    parent = "value"  # Required. The resource name of the TensorboardExperiment to create the TensorboardRun in. Format: `projects/{project}/locations/{location}/tensorboards/{tensorboard}/experiments/{experiment}`
}

# Access run outputs
run_id = run.id
run_create_time = run.create_time
run_labels = run.labels
run_name = run.name
run_update_time = run.update_time
run_description = run.description
run_etag = run.etag
run_display_name = run.display_name
```

---


### Slice

Imports a list of externally generated EvaluatedAnnotations.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `evaluated_annotations` | Vec<String> |  | Required. Evaluated annotations resource to be imported. |
| `parent` | String | ✅ | Required. The name of the parent ModelEvaluationSlice resource. Format: `projects/{project}/locations/{location}/models/{model}/evaluations/{evaluation}/slices/{slice}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metrics` | String | Output only. Sliced evaluation metrics of the Model. The schema of the metrics is stored in metrics_schema_uri |
| `model_explanation` | String | Output only. Aggregated explanation metrics for the Model's prediction output over the data this ModelEvaluation uses. This field is populated only if the Model is evaluated with explanations, and only for tabular Models. |
| `slice` | String | Output only. The slice of the test data that is used to evaluate the Model. |
| `metrics_schema_uri` | String | Output only. Points to a YAML file stored on Google Cloud Storage describing the metrics of this ModelEvaluationSlice. The schema is defined as an OpenAPI 3.0.2 [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.2.md#schemaObject). |
| `create_time` | String | Output only. Timestamp when this ModelEvaluationSlice was created. |
| `name` | String | Output only. The resource name of the ModelEvaluationSlice. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create slice
slice = provider.aiplatform_api.Slice {
    parent = "value"  # Required. The name of the parent ModelEvaluationSlice resource. Format: `projects/{project}/locations/{location}/models/{model}/evaluations/{evaluation}/slices/{slice}`
}

# Access slice outputs
slice_id = slice.id
slice_metrics = slice.metrics
slice_model_explanation = slice.model_explanation
slice_slice = slice.slice
slice_metrics_schema_uri = slice.metrics_schema_uri
slice_create_time = slice.create_time
slice_name = slice.name
```

---


### Nas_trial_detail

Gets a NasTrialDetail.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `search_trial` | String | The requested search NasTrial. |
| `train_trial` | String | The train NasTrial corresponding to search_trial. Only populated if search_trial is used for training. |
| `parameters` | String | The parameters for the NasJob NasTrial. |
| `name` | String | Output only. Resource name of the NasTrialDetail. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access nas_trial_detail outputs
nas_trial_detail_id = nas_trial_detail.id
nas_trial_detail_search_trial = nas_trial_detail.search_trial
nas_trial_detail_train_trial = nas_trial_detail.train_trial
nas_trial_detail_parameters = nas_trial_detail.parameters
nas_trial_detail_name = nas_trial_detail.name
```

---


### Specialist_pool

Creates a SpecialistPool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `specialist_worker_emails` | Vec<String> |  | The email addresses of workers in the SpecialistPool. |
| `pending_data_labeling_jobs` | Vec<String> |  | Output only. The resource name of the pending data labeling jobs. |
| `name` | String |  | Required. The resource name of the SpecialistPool. |
| `specialist_manager_emails` | Vec<String> |  | The email addresses of the managers in the SpecialistPool. |
| `display_name` | String |  | Required. The user-defined name of the SpecialistPool. The name can be up to 128 characters long and can consist of any UTF-8 characters. This field should be unique on project-level. |
| `specialist_managers_count` | i64 |  | Output only. The number of managers in this SpecialistPool. |
| `parent` | String | ✅ | Required. The parent Project name for the new SpecialistPool. The form is `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `specialist_worker_emails` | Vec<String> | The email addresses of workers in the SpecialistPool. |
| `pending_data_labeling_jobs` | Vec<String> | Output only. The resource name of the pending data labeling jobs. |
| `name` | String | Required. The resource name of the SpecialistPool. |
| `specialist_manager_emails` | Vec<String> | The email addresses of the managers in the SpecialistPool. |
| `display_name` | String | Required. The user-defined name of the SpecialistPool. The name can be up to 128 characters long and can consist of any UTF-8 characters. This field should be unique on project-level. |
| `specialist_managers_count` | i64 | Output only. The number of managers in this SpecialistPool. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create specialist_pool
specialist_pool = provider.aiplatform_api.Specialist_pool {
    parent = "value"  # Required. The parent Project name for the new SpecialistPool. The form is `projects/{project}/locations/{location}`.
}

# Access specialist_pool outputs
specialist_pool_id = specialist_pool.id
specialist_pool_specialist_worker_emails = specialist_pool.specialist_worker_emails
specialist_pool_pending_data_labeling_jobs = specialist_pool.pending_data_labeling_jobs
specialist_pool_name = specialist_pool.name
specialist_pool_specialist_manager_emails = specialist_pool.specialist_manager_emails
specialist_pool_display_name = specialist_pool.display_name
specialist_pool_specialist_managers_count = specialist_pool.specialist_managers_count
```

---


### Data_labeling_job

Creates a DataLabelingJob.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when this DataLabelingJob was created. |
| `labeling_progress` | i64 |  | Output only. Current labeling job progress percentage scaled in interval [0, 100], indicating the percentage of DataItems that has been finished. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a DataLabelingJob. If set, this DataLabelingJob will be secured by this key. Note: Annotations created in the DataLabelingJob are associated with the EncryptionSpec of the Dataset they are exported to. |
| `labels` | HashMap<String, String> |  | The labels with user-defined metadata to organize your DataLabelingJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. Following system labels exist for each DataLabelingJob: * "aiplatform.googleapis.com/schema": output only, its value is the inputs_schema's title. |
| `update_time` | String |  | Output only. Timestamp when this DataLabelingJob was updated most recently. |
| `error` | String |  | Output only. DataLabelingJob errors. It is only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `specialist_pools` | Vec<String> |  | The SpecialistPools' resource names associated with this job. |
| `inputs_schema_uri` | String |  | Required. Points to a YAML file stored on Google Cloud Storage describing the config for a specific type of DataLabelingJob. The schema files that can be used here are found in the https://storage.googleapis.com/google-cloud-aiplatform bucket in the /schema/datalabelingjob/inputs/ folder. |
| `display_name` | String |  | Required. The user-defined name of the DataLabelingJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. Display name of a DataLabelingJob. |
| `instruction_uri` | String |  | Required. The Google Cloud Storage location of the instruction pdf. This pdf is shared with labelers, and provides detailed description on how to label DataItems in Datasets. |
| `inputs` | String |  | Required. Input config parameters for the DataLabelingJob. |
| `annotation_labels` | HashMap<String, String> |  | Labels to assign to annotations generated by this DataLabelingJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `datasets` | Vec<String> |  | Required. Dataset resource names. Right now we only support labeling from a single Dataset. Format: `projects/{project}/locations/{location}/datasets/{dataset}` |
| `name` | String |  | Output only. Resource name of the DataLabelingJob. |
| `active_learning_config` | String |  | Parameters that configure the active learning pipeline. Active learning will label the data incrementally via several iterations. For every iteration, it will select a batch of data based on the sampling strategy. |
| `state` | String |  | Output only. The detailed state of the job. |
| `current_spend` | String |  | Output only. Estimated cost(in US dollars) that the DataLabelingJob has incurred to date. |
| `labeler_count` | i64 |  | Required. Number of labelers to work on each DataItem. |
| `parent` | String | ✅ | Required. The parent of the DataLabelingJob. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when this DataLabelingJob was created. |
| `labeling_progress` | i64 | Output only. Current labeling job progress percentage scaled in interval [0, 100], indicating the percentage of DataItems that has been finished. |
| `encryption_spec` | String | Customer-managed encryption key spec for a DataLabelingJob. If set, this DataLabelingJob will be secured by this key. Note: Annotations created in the DataLabelingJob are associated with the EncryptionSpec of the Dataset they are exported to. |
| `labels` | HashMap<String, String> | The labels with user-defined metadata to organize your DataLabelingJobs. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. Following system labels exist for each DataLabelingJob: * "aiplatform.googleapis.com/schema": output only, its value is the inputs_schema's title. |
| `update_time` | String | Output only. Timestamp when this DataLabelingJob was updated most recently. |
| `error` | String | Output only. DataLabelingJob errors. It is only populated when job's state is `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`. |
| `specialist_pools` | Vec<String> | The SpecialistPools' resource names associated with this job. |
| `inputs_schema_uri` | String | Required. Points to a YAML file stored on Google Cloud Storage describing the config for a specific type of DataLabelingJob. The schema files that can be used here are found in the https://storage.googleapis.com/google-cloud-aiplatform bucket in the /schema/datalabelingjob/inputs/ folder. |
| `display_name` | String | Required. The user-defined name of the DataLabelingJob. The name can be up to 128 characters long and can consist of any UTF-8 characters. Display name of a DataLabelingJob. |
| `instruction_uri` | String | Required. The Google Cloud Storage location of the instruction pdf. This pdf is shared with labelers, and provides detailed description on how to label DataItems in Datasets. |
| `inputs` | String | Required. Input config parameters for the DataLabelingJob. |
| `annotation_labels` | HashMap<String, String> | Labels to assign to annotations generated by this DataLabelingJob. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `datasets` | Vec<String> | Required. Dataset resource names. Right now we only support labeling from a single Dataset. Format: `projects/{project}/locations/{location}/datasets/{dataset}` |
| `name` | String | Output only. Resource name of the DataLabelingJob. |
| `active_learning_config` | String | Parameters that configure the active learning pipeline. Active learning will label the data incrementally via several iterations. For every iteration, it will select a batch of data based on the sampling strategy. |
| `state` | String | Output only. The detailed state of the job. |
| `current_spend` | String | Output only. Estimated cost(in US dollars) that the DataLabelingJob has incurred to date. |
| `labeler_count` | i64 | Required. Number of labelers to work on each DataItem. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_labeling_job
data_labeling_job = provider.aiplatform_api.Data_labeling_job {
    parent = "value"  # Required. The parent of the DataLabelingJob. Format: `projects/{project}/locations/{location}`
}

# Access data_labeling_job outputs
data_labeling_job_id = data_labeling_job.id
data_labeling_job_create_time = data_labeling_job.create_time
data_labeling_job_labeling_progress = data_labeling_job.labeling_progress
data_labeling_job_encryption_spec = data_labeling_job.encryption_spec
data_labeling_job_labels = data_labeling_job.labels
data_labeling_job_update_time = data_labeling_job.update_time
data_labeling_job_error = data_labeling_job.error
data_labeling_job_specialist_pools = data_labeling_job.specialist_pools
data_labeling_job_inputs_schema_uri = data_labeling_job.inputs_schema_uri
data_labeling_job_display_name = data_labeling_job.display_name
data_labeling_job_instruction_uri = data_labeling_job.instruction_uri
data_labeling_job_inputs = data_labeling_job.inputs
data_labeling_job_annotation_labels = data_labeling_job.annotation_labels
data_labeling_job_datasets = data_labeling_job.datasets
data_labeling_job_name = data_labeling_job.name
data_labeling_job_active_learning_config = data_labeling_job.active_learning_config
data_labeling_job_state = data_labeling_job.state
data_labeling_job_current_spend = data_labeling_job.current_spend
data_labeling_job_labeler_count = data_labeling_job.labeler_count
```

---


### Feature

Creates a new Feature in a given FeatureGroup.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `monitoring_stats_anomalies` | Vec<String> |  | Output only. Only applicable for Vertex AI Feature Store (Legacy). The list of historical stats and anomalies with specified objectives. |
| `name` | String |  | Immutable. Name of the Feature. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}/features/{feature}` `projects/{project}/locations/{location}/featureGroups/{feature_group}/features/{feature}` The last part feature is assigned by the client. The feature can be up to 64 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscore(_), and ASCII digits 0-9 starting with a letter. The value will be unique given an entity type. |
| `point_of_contact` | String |  | Entity responsible for maintaining this feature. Can be comma separated list of email addresses or URIs. |
| `update_time` | String |  | Output only. Only applicable for Vertex AI Feature Store (Legacy). Timestamp when this EntityType was most recently updated. |
| `value_type` | String |  | Immutable. Only applicable for Vertex AI Feature Store (Legacy). Type of Feature value. |
| `version_column_name` | String |  | Only applicable for Vertex AI Feature Store. The name of the BigQuery Table/View column hosting data for this version. If no value is provided, will use feature_id. |
| `feature_stats_and_anomaly` | Vec<String> |  | Output only. Only applicable for Vertex AI Feature Store. The list of historical stats and anomalies. |
| `description` | String |  | Description of the Feature. |
| `monitoring_config` | String |  | Optional. Only applicable for Vertex AI Feature Store (Legacy). Deprecated: The custom monitoring configuration for this Feature, if not set, use the monitoring_config defined for the EntityType this Feature belongs to. Only Features with type (Feature.ValueType) BOOL, STRING, DOUBLE or INT64 can enable monitoring. If this is populated with FeaturestoreMonitoringConfig.disabled = true, snapshot analysis monitoring is disabled; if FeaturestoreMonitoringConfig.monitoring_interval specified, snapshot analysis monitoring is enabled. Otherwise, snapshot analysis monitoring config is same as the EntityType's this Feature belongs to. |
| `monitoring_stats` | Vec<String> |  | Output only. Only applicable for Vertex AI Feature Store (Legacy). A list of historical SnapshotAnalysis stats requested by user, sorted by FeatureStatsAnomaly.start_time descending. |
| `disable_monitoring` | bool |  | Optional. Only applicable for Vertex AI Feature Store (Legacy). If not set, use the monitoring_config defined for the EntityType this Feature belongs to. Only Features with type (Feature.ValueType) BOOL, STRING, DOUBLE or INT64 can enable monitoring. If set to true, all types of data monitoring are disabled despite the config on EntityType. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your Features. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one Feature (System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `create_time` | String |  | Output only. Only applicable for Vertex AI Feature Store (Legacy). Timestamp when this EntityType was created. |
| `parent` | String | ✅ | Required. The resource name of the EntityType or FeatureGroup to create a Feature. Format for entity_type as parent: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}` Format for feature_group as parent: `projects/{project}/locations/{location}/featureGroups/{feature_group}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `monitoring_stats_anomalies` | Vec<String> | Output only. Only applicable for Vertex AI Feature Store (Legacy). The list of historical stats and anomalies with specified objectives. |
| `name` | String | Immutable. Name of the Feature. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}/features/{feature}` `projects/{project}/locations/{location}/featureGroups/{feature_group}/features/{feature}` The last part feature is assigned by the client. The feature can be up to 64 characters long and can consist only of ASCII Latin letters A-Z and a-z, underscore(_), and ASCII digits 0-9 starting with a letter. The value will be unique given an entity type. |
| `point_of_contact` | String | Entity responsible for maintaining this feature. Can be comma separated list of email addresses or URIs. |
| `update_time` | String | Output only. Only applicable for Vertex AI Feature Store (Legacy). Timestamp when this EntityType was most recently updated. |
| `value_type` | String | Immutable. Only applicable for Vertex AI Feature Store (Legacy). Type of Feature value. |
| `version_column_name` | String | Only applicable for Vertex AI Feature Store. The name of the BigQuery Table/View column hosting data for this version. If no value is provided, will use feature_id. |
| `feature_stats_and_anomaly` | Vec<String> | Output only. Only applicable for Vertex AI Feature Store. The list of historical stats and anomalies. |
| `description` | String | Description of the Feature. |
| `monitoring_config` | String | Optional. Only applicable for Vertex AI Feature Store (Legacy). Deprecated: The custom monitoring configuration for this Feature, if not set, use the monitoring_config defined for the EntityType this Feature belongs to. Only Features with type (Feature.ValueType) BOOL, STRING, DOUBLE or INT64 can enable monitoring. If this is populated with FeaturestoreMonitoringConfig.disabled = true, snapshot analysis monitoring is disabled; if FeaturestoreMonitoringConfig.monitoring_interval specified, snapshot analysis monitoring is enabled. Otherwise, snapshot analysis monitoring config is same as the EntityType's this Feature belongs to. |
| `monitoring_stats` | Vec<String> | Output only. Only applicable for Vertex AI Feature Store (Legacy). A list of historical SnapshotAnalysis stats requested by user, sorted by FeatureStatsAnomaly.start_time descending. |
| `disable_monitoring` | bool | Optional. Only applicable for Vertex AI Feature Store (Legacy). If not set, use the monitoring_config defined for the EntityType this Feature belongs to. Only Features with type (Feature.ValueType) BOOL, STRING, DOUBLE or INT64 can enable monitoring. If set to true, all types of data monitoring are disabled despite the config on EntityType. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your Features. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one Feature (System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `create_time` | String | Output only. Only applicable for Vertex AI Feature Store (Legacy). Timestamp when this EntityType was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature
feature = provider.aiplatform_api.Feature {
    parent = "value"  # Required. The resource name of the EntityType or FeatureGroup to create a Feature. Format for entity_type as parent: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}` Format for feature_group as parent: `projects/{project}/locations/{location}/featureGroups/{feature_group}`
}

# Access feature outputs
feature_id = feature.id
feature_etag = feature.etag
feature_monitoring_stats_anomalies = feature.monitoring_stats_anomalies
feature_name = feature.name
feature_point_of_contact = feature.point_of_contact
feature_update_time = feature.update_time
feature_value_type = feature.value_type
feature_version_column_name = feature.version_column_name
feature_feature_stats_and_anomaly = feature.feature_stats_and_anomaly
feature_description = feature.description
feature_monitoring_config = feature.monitoring_config
feature_monitoring_stats = feature.monitoring_stats
feature_disable_monitoring = feature.disable_monitoring
feature_labels = feature.labels
feature_create_time = feature.create_time
```

---


### Metadata_store

Initializes a MetadataStore, including allocation of resources.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The resource name of the MetadataStore instance. |
| `update_time` | String |  | Output only. Timestamp when this MetadataStore was last updated. |
| `description` | String |  | Description of the MetadataStore. |
| `encryption_spec` | String |  | Customer-managed encryption key spec for a Metadata Store. If set, this Metadata Store and all sub-resources of this Metadata Store are secured using this key. |
| `state` | String |  | Output only. State information of the MetadataStore. |
| `create_time` | String |  | Output only. Timestamp when this MetadataStore was created. |
| `dataplex_config` | String |  | Optional. Dataplex integration settings. |
| `parent` | String | ✅ | Required. The resource name of the Location where the MetadataStore should be created. Format: `projects/{project}/locations/{location}/` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name of the MetadataStore instance. |
| `update_time` | String | Output only. Timestamp when this MetadataStore was last updated. |
| `description` | String | Description of the MetadataStore. |
| `encryption_spec` | String | Customer-managed encryption key spec for a Metadata Store. If set, this Metadata Store and all sub-resources of this Metadata Store are secured using this key. |
| `state` | String | Output only. State information of the MetadataStore. |
| `create_time` | String | Output only. Timestamp when this MetadataStore was created. |
| `dataplex_config` | String | Optional. Dataplex integration settings. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create metadata_store
metadata_store = provider.aiplatform_api.Metadata_store {
    parent = "value"  # Required. The resource name of the Location where the MetadataStore should be created. Format: `projects/{project}/locations/{location}/`
}

# Access metadata_store outputs
metadata_store_id = metadata_store.id
metadata_store_name = metadata_store.name
metadata_store_update_time = metadata_store.update_time
metadata_store_description = metadata_store.description
metadata_store_encryption_spec = metadata_store.encryption_spec
metadata_store_state = metadata_store.state
metadata_store_create_time = metadata_store.create_time
metadata_store_dataplex_config = metadata_store.dataplex_config
```

---


### Studie

Creates a Study. A resource name will be generated after creation of the Study.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The name of a study. The study's globally unique identifier. Format: `projects/{project}/locations/{location}/studies/{study}` |
| `study_spec` | String |  | Required. Configuration of the Study. |
| `state` | String |  | Output only. The detailed state of a Study. |
| `create_time` | String |  | Output only. Time at which the study was created. |
| `display_name` | String |  | Required. Describes the Study, default value is empty string. |
| `inactive_reason` | String |  | Output only. A human readable reason why the Study is inactive. This should be empty if a study is ACTIVE or COMPLETED. |
| `parent` | String | ✅ | Required. The resource name of the Location to create the CustomJob in. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The name of a study. The study's globally unique identifier. Format: `projects/{project}/locations/{location}/studies/{study}` |
| `study_spec` | String | Required. Configuration of the Study. |
| `state` | String | Output only. The detailed state of a Study. |
| `create_time` | String | Output only. Time at which the study was created. |
| `display_name` | String | Required. Describes the Study, default value is empty string. |
| `inactive_reason` | String | Output only. A human readable reason why the Study is inactive. This should be empty if a study is ACTIVE or COMPLETED. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create studie
studie = provider.aiplatform_api.Studie {
    parent = "value"  # Required. The resource name of the Location to create the CustomJob in. Format: `projects/{project}/locations/{location}`
}

# Access studie outputs
studie_id = studie.id
studie_name = studie.name
studie_study_spec = studie.study_spec
studie_state = studie.state
studie_create_time = studie.create_time
studie_display_name = studie.display_name
studie_inactive_reason = studie.inactive_reason
```

---


### Entity_type

Creates a new EntityType in a given Featurestore.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `monitoring_config` | String |  | Optional. The default monitoring configuration for all Features with value type (Feature.ValueType) BOOL, STRING, DOUBLE or INT64 under this EntityType. If this is populated with [FeaturestoreMonitoringConfig.monitoring_interval] specified, snapshot analysis monitoring is enabled. Otherwise, snapshot analysis monitoring is disabled. |
| `create_time` | String |  | Output only. Timestamp when this EntityType was created. |
| `name` | String |  | Immutable. Name of the EntityType. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}` The last part entity_type is assigned by the client. The entity_type can be up to 64 characters long and can consist only of ASCII Latin letters A-Z and a-z and underscore(_), and ASCII digits 0-9 starting with a letter. The value will be unique given a featurestore. |
| `etag` | String |  | Optional. Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `description` | String |  | Optional. Description of the EntityType. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata to organize your EntityTypes. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one EntityType (System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `offline_storage_ttl_days` | i64 |  | Optional. Config for data retention policy in offline storage. TTL in days for feature values that will be stored in offline storage. The Feature Store offline storage periodically removes obsolete feature values older than `offline_storage_ttl_days` since the feature generation time. If unset (or explicitly set to 0), default to 4000 days TTL. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. Timestamp when this EntityType was most recently updated. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The resource name of the Featurestore to create EntityTypes. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `monitoring_config` | String | Optional. The default monitoring configuration for all Features with value type (Feature.ValueType) BOOL, STRING, DOUBLE or INT64 under this EntityType. If this is populated with [FeaturestoreMonitoringConfig.monitoring_interval] specified, snapshot analysis monitoring is enabled. Otherwise, snapshot analysis monitoring is disabled. |
| `create_time` | String | Output only. Timestamp when this EntityType was created. |
| `name` | String | Immutable. Name of the EntityType. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}/entityTypes/{entity_type}` The last part entity_type is assigned by the client. The entity_type can be up to 64 characters long and can consist only of ASCII Latin letters A-Z and a-z and underscore(_), and ASCII digits 0-9 starting with a letter. The value will be unique given a featurestore. |
| `etag` | String | Optional. Used to perform a consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `description` | String | Optional. Description of the EntityType. |
| `labels` | HashMap<String, String> | Optional. The labels with user-defined metadata to organize your EntityTypes. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. See https://goo.gl/xmQnxf for more information on and examples of labels. No more than 64 user labels can be associated with one EntityType (System labels are excluded)." System reserved label keys are prefixed with "aiplatform.googleapis.com/" and are immutable. |
| `offline_storage_ttl_days` | i64 | Optional. Config for data retention policy in offline storage. TTL in days for feature values that will be stored in offline storage. The Feature Store offline storage periodically removes obsolete feature values older than `offline_storage_ttl_days` since the feature generation time. If unset (or explicitly set to 0), default to 4000 days TTL. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. Timestamp when this EntityType was most recently updated. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entity_type
entity_type = provider.aiplatform_api.Entity_type {
    parent = "value"  # Required. The resource name of the Featurestore to create EntityTypes. Format: `projects/{project}/locations/{location}/featurestores/{featurestore}`
}

# Access entity_type outputs
entity_type_id = entity_type.id
entity_type_monitoring_config = entity_type.monitoring_config
entity_type_create_time = entity_type.create_time
entity_type_name = entity_type.name
entity_type_etag = entity_type.etag
entity_type_description = entity_type.description
entity_type_labels = entity_type.labels
entity_type_offline_storage_ttl_days = entity_type.offline_storage_ttl_days
entity_type_satisfies_pzi = entity_type.satisfies_pzi
entity_type_update_time = entity_type.update_time
entity_type_satisfies_pzs = entity_type.satisfies_pzs
```

---


### Annotation_spec

Gets an AnnotationSpec.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Timestamp when AnnotationSpec was last updated. |
| `create_time` | String | Output only. Timestamp when this AnnotationSpec was created. |
| `etag` | String | Optional. Used to perform consistent read-modify-write updates. If not set, a blind "overwrite" update happens. |
| `display_name` | String | Required. The user-defined name of the AnnotationSpec. The name can be up to 128 characters long and can consist of any UTF-8 characters. |
| `name` | String | Output only. Resource name of the AnnotationSpec. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access annotation_spec outputs
annotation_spec_id = annotation_spec.id
annotation_spec_update_time = annotation_spec.update_time
annotation_spec_create_time = annotation_spec.create_time
annotation_spec_etag = annotation_spec.etag
annotation_spec_display_name = annotation_spec.display_name
annotation_spec_name = annotation_spec.name
```

---


### Evaluation

Imports an externally generated ModelEvaluation.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_evaluation` | String |  | Required. Model evaluation resource to be imported. |
| `parent` | String | ✅ | Required. The name of the parent model resource. Format: `projects/{project}/locations/{location}/models/{model}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name of the ModelEvaluation. |
| `display_name` | String | The display name of the ModelEvaluation. |
| `metrics` | String | Evaluation metrics of the Model. The schema of the metrics is stored in metrics_schema_uri |
| `create_time` | String | Output only. Timestamp when this ModelEvaluation was created. |
| `metadata` | String | The metadata of the ModelEvaluation. For the ModelEvaluation uploaded from Managed Pipeline, metadata contains a structured value with keys of "pipeline_job_id", "evaluation_dataset_type", "evaluation_dataset_path", "row_based_metrics_path". |
| `slice_dimensions` | Vec<String> | All possible dimensions of ModelEvaluationSlices. The dimensions can be used as the filter of the ModelService.ListModelEvaluationSlices request, in the form of `slice.dimension = `. |
| `bias_configs` | String | Specify the configuration for bias detection. |
| `model_explanation` | String | Aggregated explanation metrics for the Model's prediction output over the data this ModelEvaluation uses. This field is populated only if the Model is evaluated with explanations, and only for AutoML tabular Models.  |
| `explanation_specs` | Vec<String> | Describes the values of ExplanationSpec that are used for explaining the predicted values on the evaluated data. |
| `metrics_schema_uri` | String | Points to a YAML file stored on Google Cloud Storage describing the metrics of this ModelEvaluation. The schema is defined as an OpenAPI 3.0.2 [Schema Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.0.2.md#schemaObject). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation
evaluation = provider.aiplatform_api.Evaluation {
    parent = "value"  # Required. The name of the parent model resource. Format: `projects/{project}/locations/{location}/models/{model}`
}

# Access evaluation outputs
evaluation_id = evaluation.id
evaluation_name = evaluation.name
evaluation_display_name = evaluation.display_name
evaluation_metrics = evaluation.metrics
evaluation_create_time = evaluation.create_time
evaluation_metadata = evaluation.metadata
evaluation_slice_dimensions = evaluation.slice_dimensions
evaluation_bias_configs = evaluation.bias_configs
evaluation_model_explanation = evaluation.model_explanation
evaluation_explanation_specs = evaluation.explanation_specs
evaluation_metrics_schema_uri = evaluation.metrics_schema_uri
```

---


### Rag_file

Import files from Google Cloud Storage or Google Drive into a RagCorpus.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `import_rag_files_config` | String |  | Required. The config for the RagFiles to be synced and imported into the RagCorpus. VertexRagDataService.ImportRagFiles. |
| `parent` | String | ✅ | Required. The name of the RagCorpus resource into which to import files. Format: `projects/{project}/locations/{location}/ragCorpora/{rag_corpus}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `google_drive_source` | String | Output only. Google Drive location. Supports importing individual files as well as Google Drive folders. |
| `direct_upload_source` | String | Output only. The RagFile is encapsulated and uploaded in the UploadRagFile request. |
| `slack_source` | String | The RagFile is imported from a Slack channel. |
| `gcs_source` | String | Output only. Google Cloud Storage location of the RagFile. It does not support wildcards in the Cloud Storage uri for now. |
| `user_metadata` | String | Output only. The metadata for metadata search. The user_metadata Needs to be in JSON format. |
| `create_time` | String | Output only. Timestamp when this RagFile was created. |
| `name` | String | Output only. The resource name of the RagFile. |
| `file_status` | String | Output only. State of the RagFile. |
| `size_bytes` | String | Output only. The size of the RagFile in bytes. |
| `update_time` | String | Output only. Timestamp when this RagFile was last updated. |
| `description` | String | Optional. The description of the RagFile. |
| `rag_file_type` | String | Output only. The type of the RagFile. |
| `jira_source` | String | The RagFile is imported from a Jira query. |
| `share_point_sources` | String | The RagFile is imported from a SharePoint source. |
| `display_name` | String | Required. The display name of the RagFile. The name can be up to 128 characters long and can consist of any UTF-8 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rag_file
rag_file = provider.aiplatform_api.Rag_file {
    parent = "value"  # Required. The name of the RagCorpus resource into which to import files. Format: `projects/{project}/locations/{location}/ragCorpora/{rag_corpus}`
}

# Access rag_file outputs
rag_file_id = rag_file.id
rag_file_google_drive_source = rag_file.google_drive_source
rag_file_direct_upload_source = rag_file.direct_upload_source
rag_file_slack_source = rag_file.slack_source
rag_file_gcs_source = rag_file.gcs_source
rag_file_user_metadata = rag_file.user_metadata
rag_file_create_time = rag_file.create_time
rag_file_name = rag_file.name
rag_file_file_status = rag_file.file_status
rag_file_size_bytes = rag_file.size_bytes
rag_file_update_time = rag_file.update_time
rag_file_description = rag_file.description
rag_file_rag_file_type = rag_file.rag_file_type
rag_file_jira_source = rag_file.jira_source
rag_file_share_point_sources = rag_file.share_point_sources
rag_file_display_name = rag_file.display_name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple model resources
model_0 = provider.aiplatform_api.Model {
    endpoint = "value-0"
}
model_1 = provider.aiplatform_api.Model {
    endpoint = "value-1"
}
model_2 = provider.aiplatform_api.Model {
    endpoint = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    model = provider.aiplatform_api.Model {
        endpoint = "production-value"
    }
```

---

## Related Documentation

- [GCP Aiplatform_api Documentation](https://cloud.google.com/aiplatform_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
