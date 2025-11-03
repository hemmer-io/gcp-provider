# Ml_api Service



**Resources**: 8

---

## Overview

The ml_api service provides access to 8 resource types:

- [Operation](#operation) [CR]
- [Job](#job) [CRU]
- [Trial](#trial) [CRD]
- [Location](#location) [R]
- [Studie](#studie) [CRD]
- [Project](#project) [CR]
- [Model](#model) [CRUD]
- [Version](#version) [CRUD]

---

## Resources


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read

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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.ml_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
```

---


### Job

Creates a training or a batch prediction job.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prediction_output` | String |  | The current prediction job result. |
| `end_time` | String |  | Output only. When the job processing was completed. |
| `start_time` | String |  | Output only. When the job processing was started. |
| `training_output` | String |  | The current training job result. |
| `etag` | String |  | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a job from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform job updates in order to avoid race conditions: An `etag` is returned in the response to `GetJob`, and systems are expected to put that etag in the request to `UpdateJob` to ensure that their change will be applied to the same version of the job. |
| `job_id` | String |  | Required. The user-specified id of the job. |
| `job_position` | String |  | Output only. It's only effect when the job is in QUEUED state. If it's positive, it indicates the job's position in the job scheduler. It's 0 when the job is already scheduled. |
| `create_time` | String |  | Output only. When the job was created. |
| `labels` | HashMap<String, String> |  | Optional. One or more labels that you can add, to organize your jobs. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels. |
| `error_message` | String |  | Output only. The details of a failure or a cancellation. |
| `prediction_input` | String |  | Input parameters to create a prediction job. |
| `state` | String |  | Output only. The detailed state of a job. |
| `training_input` | String |  | Input parameters to create a training job. |
| `parent` | String | ✅ | Required. The project name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `prediction_output` | String | The current prediction job result. |
| `end_time` | String | Output only. When the job processing was completed. |
| `start_time` | String | Output only. When the job processing was started. |
| `training_output` | String | The current training job result. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a job from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform job updates in order to avoid race conditions: An `etag` is returned in the response to `GetJob`, and systems are expected to put that etag in the request to `UpdateJob` to ensure that their change will be applied to the same version of the job. |
| `job_id` | String | Required. The user-specified id of the job. |
| `job_position` | String | Output only. It's only effect when the job is in QUEUED state. If it's positive, it indicates the job's position in the job scheduler. It's 0 when the job is already scheduled. |
| `create_time` | String | Output only. When the job was created. |
| `labels` | HashMap<String, String> | Optional. One or more labels that you can add, to organize your jobs. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels. |
| `error_message` | String | Output only. The details of a failure or a cancellation. |
| `prediction_input` | String | Input parameters to create a prediction job. |
| `state` | String | Output only. The detailed state of a job. |
| `training_input` | String | Input parameters to create a training job. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.ml_api.Job {
    parent = "value"  # Required. The project name.
}

# Access job outputs
job_id = job.id
job_prediction_output = job.prediction_output
job_end_time = job.end_time
job_start_time = job.start_time
job_training_output = job.training_output
job_etag = job.etag
job_job_id = job.job_id
job_job_position = job.job_position
job_create_time = job.create_time
job_labels = job.labels
job_error_message = job.error_message
job_prediction_input = job.prediction_input
job_state = job.state
job_training_input = job.training_input
```

---


### Trial

Adds a user provided trial to a study.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `trial_infeasible` | bool |  | Output only. If true, the parameters in this trial are not attempted again. |
| `measurements` | Vec<String> |  | A list of measurements that are strictly lexicographically ordered by their induced tuples (steps, elapsed_time). These are used for early stopping computations. |
| `start_time` | String |  | Output only. Time at which the trial was started. |
| `client_id` | String |  | Output only. The identifier of the client that originally requested this trial. |
| `parameters` | Vec<String> |  | The parameters of the trial. |
| `state` | String |  | The detailed state of a trial. |
| `name` | String |  | Output only. Name of the trial assigned by the service. |
| `infeasible_reason` | String |  | Output only. A human readable string describing why the trial is infeasible. This should only be set if trial_infeasible is true. |
| `final_measurement` | String |  | The final measurement containing the objective value. |
| `end_time` | String |  | Output only. Time at which the trial's status changed to COMPLETED. |
| `parent` | String | ✅ | Required. The name of the study that the trial belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `trial_infeasible` | bool | Output only. If true, the parameters in this trial are not attempted again. |
| `measurements` | Vec<String> | A list of measurements that are strictly lexicographically ordered by their induced tuples (steps, elapsed_time). These are used for early stopping computations. |
| `start_time` | String | Output only. Time at which the trial was started. |
| `client_id` | String | Output only. The identifier of the client that originally requested this trial. |
| `parameters` | Vec<String> | The parameters of the trial. |
| `state` | String | The detailed state of a trial. |
| `name` | String | Output only. Name of the trial assigned by the service. |
| `infeasible_reason` | String | Output only. A human readable string describing why the trial is infeasible. This should only be set if trial_infeasible is true. |
| `final_measurement` | String | The final measurement containing the objective value. |
| `end_time` | String | Output only. Time at which the trial's status changed to COMPLETED. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trial
trial = provider.ml_api.Trial {
    parent = "value"  # Required. The name of the study that the trial belongs to.
}

# Access trial outputs
trial_id = trial.id
trial_trial_infeasible = trial.trial_infeasible
trial_measurements = trial.measurements
trial_start_time = trial.start_time
trial_client_id = trial.client_id
trial_parameters = trial.parameters
trial_state = trial.state
trial_name = trial.name
trial_infeasible_reason = trial.infeasible_reason
trial_final_measurement = trial.final_measurement
trial_end_time = trial.end_time
```

---


### Location

Get the complete list of CMLE capabilities in a location, along with their location-specific properties.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String |  |
| `capabilities` | Vec<String> | Capabilities available in the location. |


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
location_name = location.name
location_capabilities = location.capabilities
```

---


### Studie

Creates a study.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The name of a study. |
| `inactive_reason` | String |  | Output only. A human readable reason why the Study is inactive. This should be empty if a study is ACTIVE or COMPLETED. |
| `state` | String |  | Output only. The detailed state of a study. |
| `study_config` | String |  | Required. Configuration of the study. |
| `create_time` | String |  | Output only. Time at which the study was created. |
| `parent` | String | ✅ | Required. The project and location that the study belongs to. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The name of a study. |
| `inactive_reason` | String | Output only. A human readable reason why the Study is inactive. This should be empty if a study is ACTIVE or COMPLETED. |
| `state` | String | Output only. The detailed state of a study. |
| `study_config` | String | Required. Configuration of the study. |
| `create_time` | String | Output only. Time at which the study was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create studie
studie = provider.ml_api.Studie {
    parent = "value"  # Required. The project and location that the study belongs to. Format: projects/{project}/locations/{location}
}

# Access studie outputs
studie_id = studie.id
studie_name = studie.name
studie_inactive_reason = studie.inactive_reason
studie_state = studie.state
studie_study_config = studie.study_config
studie_create_time = studie.create_time
```

---


### Project

Performs explanation on the data in the request. {% dynamic include "/ai-platform/includes/___explain-request" %} 

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `http_body` | String |  | Required. The explanation request body. |
| `name` | String | ✅ | Required. The resource name of a model or a version. Authorization: requires the `predict` permission on the specified resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_account_project` | String | The project number for `service_account`. |
| `service_account` | String | The service account Cloud ML uses to access resources in the project. |
| `config` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.ml_api.Project {
    name = "value"  # Required. The resource name of a model or a version. Authorization: requires the `predict` permission on the specified resource.
}

# Access project outputs
project_id = project.id
project_service_account_project = project.service_account_project
project_service_account = project.service_account
project_config = project.config
```

---


### Model

Creates a model which will later contain one or more versions. You must add at least one version before you can request predictions from the model. Add versions by calling projects.models.versions.create.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. One or more labels that you can add, to organize your models. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels. Note that this field is not updatable for mls1* models. |
| `etag` | String |  | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a model from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform model updates in order to avoid race conditions: An `etag` is returned in the response to `GetModel`, and systems are expected to put that etag in the request to `UpdateModel` to ensure that their change will be applied to the model as intended. |
| `regions` | Vec<String> |  | Optional. The list of regions where the model is going to be deployed. Only one region per model is supported. Defaults to 'us-central1' if nothing is set. See the available regions for AI Platform services. Note: * No matter where a model is deployed, it can always be accessed by users from anywhere, both for online and batch prediction. * The region for a batch prediction job is set by the region field when submitting the batch prediction job and does not take its value from this field. |
| `name` | String |  | Required. The name specified for the model when it was created. The model name must be unique within the project it is created in. |
| `online_prediction_logging` | bool |  | Optional. If true, online prediction access logs are sent to Cloud Logging. These logs are like standard server access logs, containing information like timestamp and latency for each request. Note that [logs may incur a cost](/stackdriver/pricing), especially if your project receives prediction requests at a high queries per second rate (QPS). Estimate your costs before enabling this option. Default is false. |
| `description` | String |  | Optional. The description specified for the model when it was created. |
| `online_prediction_console_logging` | bool |  | Optional. If true, online prediction nodes send `stderr` and `stdout` streams to Cloud Logging. These can be more verbose than the standard access logs (see `onlinePredictionLogging`) and can incur higher cost. However, they are helpful for debugging. Note that [logs may incur a cost](/stackdriver/pricing), especially if your project receives prediction requests at a high QPS. Estimate your costs before enabling this option. Default is false. |
| `default_version` | String |  | Output only. The default version of the model. This version will be used to handle prediction requests that do not specify a version. You can change the default version by calling projects.models.versions.setDefault. |
| `parent` | String | ✅ | Required. The project name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. One or more labels that you can add, to organize your models. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels. Note that this field is not updatable for mls1* models. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a model from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform model updates in order to avoid race conditions: An `etag` is returned in the response to `GetModel`, and systems are expected to put that etag in the request to `UpdateModel` to ensure that their change will be applied to the model as intended. |
| `regions` | Vec<String> | Optional. The list of regions where the model is going to be deployed. Only one region per model is supported. Defaults to 'us-central1' if nothing is set. See the available regions for AI Platform services. Note: * No matter where a model is deployed, it can always be accessed by users from anywhere, both for online and batch prediction. * The region for a batch prediction job is set by the region field when submitting the batch prediction job and does not take its value from this field. |
| `name` | String | Required. The name specified for the model when it was created. The model name must be unique within the project it is created in. |
| `online_prediction_logging` | bool | Optional. If true, online prediction access logs are sent to Cloud Logging. These logs are like standard server access logs, containing information like timestamp and latency for each request. Note that [logs may incur a cost](/stackdriver/pricing), especially if your project receives prediction requests at a high queries per second rate (QPS). Estimate your costs before enabling this option. Default is false. |
| `description` | String | Optional. The description specified for the model when it was created. |
| `online_prediction_console_logging` | bool | Optional. If true, online prediction nodes send `stderr` and `stdout` streams to Cloud Logging. These can be more verbose than the standard access logs (see `onlinePredictionLogging`) and can incur higher cost. However, they are helpful for debugging. Note that [logs may incur a cost](/stackdriver/pricing), especially if your project receives prediction requests at a high QPS. Estimate your costs before enabling this option. Default is false. |
| `default_version` | String | Output only. The default version of the model. This version will be used to handle prediction requests that do not specify a version. You can change the default version by calling projects.models.versions.setDefault. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model
model = provider.ml_api.Model {
    parent = "value"  # Required. The project name.
}

# Access model outputs
model_id = model.id
model_labels = model.labels
model_etag = model.etag
model_regions = model.regions
model_name = model.name
model_online_prediction_logging = model.online_prediction_logging
model_description = model.description
model_online_prediction_console_logging = model.online_prediction_console_logging
model_default_version = model.default_version
```

---


### Version

Creates a new version of a model from a trained TensorFlow model. If the version created in the cloud by this call is the first deployed version of the specified model, it will be made the default version of the model. When you add a version to a model that already has one or more versions, the default version does not automatically change. If you want a new version to be the default, you must call projects.models.versions.setDefault.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The name specified for the version when it was created. The version name must be unique within the model it is created in. |
| `container` | String |  | Optional. Specifies a custom container to use for serving predictions. If you specify this field, then `machineType` is required. If you specify this field, then `deploymentUri` is optional. If you specify this field, then you must not specify `runtimeVersion`, `packageUris`, `framework`, `pythonVersion`, or `predictionClass`. |
| `runtime_version` | String |  | Required. The AI Platform runtime version to use for this deployment. For more information, see the [runtime version list](/ml-engine/docs/runtime-version-list) and [how to manage runtime versions](/ml-engine/docs/versioning). |
| `service_account` | String |  | Optional. Specifies the service account for resource access control. If you specify this field, then you must also specify either the `containerSpec` or the `predictionClass` field. Learn more about [using a custom service account](/ai-platform/prediction/docs/custom-service-account). |
| `last_migration_model_id` | String |  | Output only. The [AI Platform (Unified) `Model`](https://cloud.google.com/ai-platform-unified/docs/reference/rest/v1beta1/projects.locations.models) ID for the last [model migration](https://cloud.google.com/ai-platform-unified/docs/start/migrating-to-ai-platform-unified). |
| `python_version` | String |  | Required. The version of Python used in prediction. The following Python versions are available: * Python '3.7' is available when `runtime_version` is set to '1.15' or later. * Python '3.5' is available when `runtime_version` is set to a version from '1.4' to '1.14'. * Python '2.7' is available when `runtime_version` is set to '1.15' or earlier. Read more about the Python versions available for [each runtime version](/ml-engine/docs/runtime-version-list). |
| `description` | String |  | Optional. The description specified for the version when it was created. |
| `explanation_config` | String |  | Optional. Configures explainability features on the model's version. Some explanation features require additional metadata to be loaded as part of the model payload. |
| `last_migration_time` | String |  | Output only. The last time this version was successfully [migrated to AI Platform (Unified)](https://cloud.google.com/ai-platform-unified/docs/start/migrating-to-ai-platform-unified). |
| `package_uris` | Vec<String> |  | Optional. Cloud Storage paths (`gs://…`) of packages for [custom prediction routines](/ml-engine/docs/tensorflow/custom-prediction-routines) or [scikit-learn pipelines with custom code](/ml-engine/docs/scikit/exporting-for-prediction#custom-pipeline-code). For a custom prediction routine, one of these packages must contain your Predictor class (see [`predictionClass`](#Version.FIELDS.prediction_class)). Additionally, include any dependencies used by your Predictor or scikit-learn pipeline uses that are not already included in your selected [runtime version](/ml-engine/docs/tensorflow/runtime-version-list). If you specify this field, you must also set [`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater. |
| `manual_scaling` | String |  | Manually select the number of nodes to use for serving the model. You should generally use `auto_scaling` with an appropriate `min_nodes` instead, but this option is available if you want more predictable billing. Beware that latency and error rates will increase if the traffic exceeds that capability of the system to serve it based on the selected number of nodes. |
| `error_message` | String |  | Output only. The details of a failure or a cancellation. |
| `create_time` | String |  | Output only. The time the version was created. |
| `is_default` | bool |  | Output only. If true, this version will be used to handle prediction requests that do not specify a version. You can change the default version by calling projects.methods.versions.setDefault. |
| `labels` | HashMap<String, String> |  | Optional. One or more labels that you can add, to organize your model versions. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels. Note that this field is not updatable for mls1* models. |
| `auto_scaling` | String |  | Automatically scale the number of nodes used to serve the model in response to increases and decreases in traffic. Care should be taken to ramp up traffic according to the model's ability to scale or you will start seeing increases in latency and 429 response codes. |
| `etag` | String |  | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a model from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform model updates in order to avoid race conditions: An `etag` is returned in the response to `GetVersion`, and systems are expected to put that etag in the request to `UpdateVersion` to ensure that their change will be applied to the model as intended. |
| `prediction_class` | String |  | Optional. The fully qualified name (module_name.class_name) of a class that implements the Predictor interface described in this reference field. The module containing this class should be included in a package provided to the [`packageUris` field](#Version.FIELDS.package_uris). Specify this field if and only if you are deploying a [custom prediction routine (beta)](/ml-engine/docs/tensorflow/custom-prediction-routines). If you specify this field, you must set [`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater and you must set `machineType` to a [legacy (MLS1) machine type](/ml-engine/docs/machine-types-online-prediction). The following code sample provides the Predictor interface: class Predictor(object): """Interface for constructing custom predictors.""" def predict(self, instances, **kwargs): """Performs custom prediction. Instances are the decoded values from the request. They have already been deserialized from JSON. Args: instances: A list of prediction input instances. **kwargs: A dictionary of keyword args provided as additional fields on the predict request body. Returns: A list of outputs containing the prediction results. This list must be JSON serializable. """ raise NotImplementedError() @classmethod def from_path(cls, model_dir): """Creates an instance of Predictor using the given path. Loading of the predictor should be done in this method. Args: model_dir: The local directory that contains the exported model file along with any additional files uploaded when creating the version resource. Returns: An instance implementing this Predictor class. """ raise NotImplementedError() Learn more about [the Predictor interface and custom prediction routines](/ml-engine/docs/tensorflow/custom-prediction-routines). |
| `deployment_uri` | String |  | The Cloud Storage URI of a directory containing trained model artifacts to be used to create the model version. See the [guide to deploying models](/ai-platform/prediction/docs/deploying-models) for more information. The total number of files under this directory must not exceed 1000. During projects.models.versions.create, AI Platform Prediction copies all files from the specified directory to a location managed by the service. From then on, AI Platform Prediction uses these copies of the model artifacts to serve predictions, not the original files in Cloud Storage, so this location is useful only as a historical record. If you specify container, then this field is optional. Otherwise, it is required. Learn [how to use this field with a custom container](/ai-platform/prediction/docs/custom-container-requirements#artifacts). |
| `routes` | String |  | Optional. Specifies paths on a custom container's HTTP server where AI Platform Prediction sends certain requests. If you specify this field, then you must also specify the `container` field. If you specify the `container` field and do not specify this field, it defaults to the following: ```json { "predict": "/v1/models/MODEL/versions/VERSION:predict", "health": "/v1/models/MODEL/versions/VERSION" } ``` See RouteMap for more details about these default values. |
| `state` | String |  | Output only. The state of a version. |
| `accelerator_config` | String |  | Optional. Accelerator config for using GPUs for online prediction (beta). Only specify this field if you have specified a Compute Engine (N1) machine type in the `machineType` field. Learn more about [using GPUs for online prediction](/ml-engine/docs/machine-types-online-prediction#gpus). |
| `last_use_time` | String |  | Output only. The time the version was last used for prediction. |
| `framework` | String |  | Optional. The machine learning framework AI Platform uses to train this version of the model. Valid values are `TENSORFLOW`, `SCIKIT_LEARN`, `XGBOOST`. If you do not specify a framework, AI Platform will analyze files in the deployment_uri to determine a framework. If you choose `SCIKIT_LEARN` or `XGBOOST`, you must also set the runtime version of the model to 1.4 or greater. Do **not** specify a framework if you're deploying a [custom prediction routine](/ai-platform/prediction/docs/custom-prediction-routines) or if you're using a [custom container](/ai-platform/prediction/docs/use-custom-container). |
| `machine_type` | String |  | Optional. The type of machine on which to serve the model. Currently only applies to online prediction service. To learn about valid values for this field, read [Choosing a machine type for online prediction](/ai-platform/prediction/docs/machine-types-online-prediction). If this field is not specified and you are using a [regional endpoint](/ai-platform/prediction/docs/regional-endpoints), then the machine type defaults to `n1-standard-2`. If this field is not specified and you are using the global endpoint (`ml.googleapis.com`), then the machine type defaults to `mls1-c1-m2`. |
| `request_logging_config` | String |  | Optional. *Only* specify this field in a projects.models.versions.patch request. Specifying it in a projects.models.versions.create request has no effect. Configures the request-response pair logging on predictions from this Version. |
| `parent` | String | ✅ | Required. The name of the model. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The name specified for the version when it was created. The version name must be unique within the model it is created in. |
| `container` | String | Optional. Specifies a custom container to use for serving predictions. If you specify this field, then `machineType` is required. If you specify this field, then `deploymentUri` is optional. If you specify this field, then you must not specify `runtimeVersion`, `packageUris`, `framework`, `pythonVersion`, or `predictionClass`. |
| `runtime_version` | String | Required. The AI Platform runtime version to use for this deployment. For more information, see the [runtime version list](/ml-engine/docs/runtime-version-list) and [how to manage runtime versions](/ml-engine/docs/versioning). |
| `service_account` | String | Optional. Specifies the service account for resource access control. If you specify this field, then you must also specify either the `containerSpec` or the `predictionClass` field. Learn more about [using a custom service account](/ai-platform/prediction/docs/custom-service-account). |
| `last_migration_model_id` | String | Output only. The [AI Platform (Unified) `Model`](https://cloud.google.com/ai-platform-unified/docs/reference/rest/v1beta1/projects.locations.models) ID for the last [model migration](https://cloud.google.com/ai-platform-unified/docs/start/migrating-to-ai-platform-unified). |
| `python_version` | String | Required. The version of Python used in prediction. The following Python versions are available: * Python '3.7' is available when `runtime_version` is set to '1.15' or later. * Python '3.5' is available when `runtime_version` is set to a version from '1.4' to '1.14'. * Python '2.7' is available when `runtime_version` is set to '1.15' or earlier. Read more about the Python versions available for [each runtime version](/ml-engine/docs/runtime-version-list). |
| `description` | String | Optional. The description specified for the version when it was created. |
| `explanation_config` | String | Optional. Configures explainability features on the model's version. Some explanation features require additional metadata to be loaded as part of the model payload. |
| `last_migration_time` | String | Output only. The last time this version was successfully [migrated to AI Platform (Unified)](https://cloud.google.com/ai-platform-unified/docs/start/migrating-to-ai-platform-unified). |
| `package_uris` | Vec<String> | Optional. Cloud Storage paths (`gs://…`) of packages for [custom prediction routines](/ml-engine/docs/tensorflow/custom-prediction-routines) or [scikit-learn pipelines with custom code](/ml-engine/docs/scikit/exporting-for-prediction#custom-pipeline-code). For a custom prediction routine, one of these packages must contain your Predictor class (see [`predictionClass`](#Version.FIELDS.prediction_class)). Additionally, include any dependencies used by your Predictor or scikit-learn pipeline uses that are not already included in your selected [runtime version](/ml-engine/docs/tensorflow/runtime-version-list). If you specify this field, you must also set [`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater. |
| `manual_scaling` | String | Manually select the number of nodes to use for serving the model. You should generally use `auto_scaling` with an appropriate `min_nodes` instead, but this option is available if you want more predictable billing. Beware that latency and error rates will increase if the traffic exceeds that capability of the system to serve it based on the selected number of nodes. |
| `error_message` | String | Output only. The details of a failure or a cancellation. |
| `create_time` | String | Output only. The time the version was created. |
| `is_default` | bool | Output only. If true, this version will be used to handle prediction requests that do not specify a version. You can change the default version by calling projects.methods.versions.setDefault. |
| `labels` | HashMap<String, String> | Optional. One or more labels that you can add, to organize your model versions. Each label is a key-value pair, where both the key and the value are arbitrary strings that you supply. For more information, see the documentation on using labels. Note that this field is not updatable for mls1* models. |
| `auto_scaling` | String | Automatically scale the number of nodes used to serve the model in response to increases and decreases in traffic. Care should be taken to ramp up traffic according to the model's ability to scale or you will start seeing increases in latency and 429 response codes. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a model from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform model updates in order to avoid race conditions: An `etag` is returned in the response to `GetVersion`, and systems are expected to put that etag in the request to `UpdateVersion` to ensure that their change will be applied to the model as intended. |
| `prediction_class` | String | Optional. The fully qualified name (module_name.class_name) of a class that implements the Predictor interface described in this reference field. The module containing this class should be included in a package provided to the [`packageUris` field](#Version.FIELDS.package_uris). Specify this field if and only if you are deploying a [custom prediction routine (beta)](/ml-engine/docs/tensorflow/custom-prediction-routines). If you specify this field, you must set [`runtimeVersion`](#Version.FIELDS.runtime_version) to 1.4 or greater and you must set `machineType` to a [legacy (MLS1) machine type](/ml-engine/docs/machine-types-online-prediction). The following code sample provides the Predictor interface: class Predictor(object): """Interface for constructing custom predictors.""" def predict(self, instances, **kwargs): """Performs custom prediction. Instances are the decoded values from the request. They have already been deserialized from JSON. Args: instances: A list of prediction input instances. **kwargs: A dictionary of keyword args provided as additional fields on the predict request body. Returns: A list of outputs containing the prediction results. This list must be JSON serializable. """ raise NotImplementedError() @classmethod def from_path(cls, model_dir): """Creates an instance of Predictor using the given path. Loading of the predictor should be done in this method. Args: model_dir: The local directory that contains the exported model file along with any additional files uploaded when creating the version resource. Returns: An instance implementing this Predictor class. """ raise NotImplementedError() Learn more about [the Predictor interface and custom prediction routines](/ml-engine/docs/tensorflow/custom-prediction-routines). |
| `deployment_uri` | String | The Cloud Storage URI of a directory containing trained model artifacts to be used to create the model version. See the [guide to deploying models](/ai-platform/prediction/docs/deploying-models) for more information. The total number of files under this directory must not exceed 1000. During projects.models.versions.create, AI Platform Prediction copies all files from the specified directory to a location managed by the service. From then on, AI Platform Prediction uses these copies of the model artifacts to serve predictions, not the original files in Cloud Storage, so this location is useful only as a historical record. If you specify container, then this field is optional. Otherwise, it is required. Learn [how to use this field with a custom container](/ai-platform/prediction/docs/custom-container-requirements#artifacts). |
| `routes` | String | Optional. Specifies paths on a custom container's HTTP server where AI Platform Prediction sends certain requests. If you specify this field, then you must also specify the `container` field. If you specify the `container` field and do not specify this field, it defaults to the following: ```json { "predict": "/v1/models/MODEL/versions/VERSION:predict", "health": "/v1/models/MODEL/versions/VERSION" } ``` See RouteMap for more details about these default values. |
| `state` | String | Output only. The state of a version. |
| `accelerator_config` | String | Optional. Accelerator config for using GPUs for online prediction (beta). Only specify this field if you have specified a Compute Engine (N1) machine type in the `machineType` field. Learn more about [using GPUs for online prediction](/ml-engine/docs/machine-types-online-prediction#gpus). |
| `last_use_time` | String | Output only. The time the version was last used for prediction. |
| `framework` | String | Optional. The machine learning framework AI Platform uses to train this version of the model. Valid values are `TENSORFLOW`, `SCIKIT_LEARN`, `XGBOOST`. If you do not specify a framework, AI Platform will analyze files in the deployment_uri to determine a framework. If you choose `SCIKIT_LEARN` or `XGBOOST`, you must also set the runtime version of the model to 1.4 or greater. Do **not** specify a framework if you're deploying a [custom prediction routine](/ai-platform/prediction/docs/custom-prediction-routines) or if you're using a [custom container](/ai-platform/prediction/docs/use-custom-container). |
| `machine_type` | String | Optional. The type of machine on which to serve the model. Currently only applies to online prediction service. To learn about valid values for this field, read [Choosing a machine type for online prediction](/ai-platform/prediction/docs/machine-types-online-prediction). If this field is not specified and you are using a [regional endpoint](/ai-platform/prediction/docs/regional-endpoints), then the machine type defaults to `n1-standard-2`. If this field is not specified and you are using the global endpoint (`ml.googleapis.com`), then the machine type defaults to `mls1-c1-m2`. |
| `request_logging_config` | String | Optional. *Only* specify this field in a projects.models.versions.patch request. Specifying it in a projects.models.versions.create request has no effect. Configures the request-response pair logging on predictions from this Version. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.ml_api.Version {
    parent = "value"  # Required. The name of the model.
}

# Access version outputs
version_id = version.id
version_name = version.name
version_container = version.container
version_runtime_version = version.runtime_version
version_service_account = version.service_account
version_last_migration_model_id = version.last_migration_model_id
version_python_version = version.python_version
version_description = version.description
version_explanation_config = version.explanation_config
version_last_migration_time = version.last_migration_time
version_package_uris = version.package_uris
version_manual_scaling = version.manual_scaling
version_error_message = version.error_message
version_create_time = version.create_time
version_is_default = version.is_default
version_labels = version.labels
version_auto_scaling = version.auto_scaling
version_etag = version.etag
version_prediction_class = version.prediction_class
version_deployment_uri = version.deployment_uri
version_routes = version.routes
version_state = version.state
version_accelerator_config = version.accelerator_config
version_last_use_time = version.last_use_time
version_framework = version.framework
version_machine_type = version.machine_type
version_request_logging_config = version.request_logging_config
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple operation resources
operation_0 = provider.ml_api.Operation {
    name = "value-0"
}
operation_1 = provider.ml_api.Operation {
    name = "value-1"
}
operation_2 = provider.ml_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.ml_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Ml_api Documentation](https://cloud.google.com/ml_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
