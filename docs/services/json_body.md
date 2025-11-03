# Json_body Service



**Resources**: 6

---

## Overview

The json_body service provides access to 6 resource types:

- [Model](#model) [CRUD]
- [Version](#version) [CRUD]
- [Job](#job) [CRU]
- [Project](#project) [CR]
- [Operation](#operation) [CRD]
- [Location](#location) [R]

---

## Resources


### Model

Creates a model which will later contain one or more versions.

You must add at least one version before you can request predictions from
the model. Add versions by calling
[projects.models.versions.create](/ml-engine/reference/rest/v1/projects.models.versions/create).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_version` | String |  | Output only. The default version of the model. This version will be used to
handle prediction requests that do not specify a version.

You can change the default version by calling
[projects.methods.versions.setDefault](/ml-engine/reference/rest/v1/projects.models.versions/setDefault). |
| `name` | String |  | Required. The name specified for the model when it was created.

The model name must be unique within the project it is created in. |
| `labels` | HashMap<String, String> |  | Optional. One or more labels that you can add, to organize your models.
Each label is a key-value pair, where both the key and the value are
arbitrary strings that you supply.
For more information, see the documentation on
<a href="/ml-engine/docs/tensorflow/resource-labels">using labels</a>. |
| `description` | String |  | Optional. The description specified for the model when it was created. |
| `online_prediction_logging` | bool |  | Optional. If true, enables StackDriver Logging for online prediction.
Default is false. |
| `regions` | Vec<String> |  | Optional. The list of regions where the model is going to be deployed.
Currently only one region per model is supported.
Defaults to 'us-central1' if nothing is set.
See the <a href="/ml-engine/docs/tensorflow/regions">available regions</a>
for ML Engine services.
Note:
*   No matter where a model is deployed, it can always be accessed by
    users from anywhere, both for online and batch prediction.
*   The region for a batch prediction job is set by the region field when
    submitting the batch prediction job and does not take its value from
    this field. |
| `etag` | String |  | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a model from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform model updates in order to avoid race
conditions: An `etag` is returned in the response to `GetModel`, and
systems are expected to put that etag in the request to `UpdateModel` to
ensure that their change will be applied to the model as intended. |
| `parent` | String | ✅ | Required. The project name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_version` | String | Output only. The default version of the model. This version will be used to
handle prediction requests that do not specify a version.

You can change the default version by calling
[projects.methods.versions.setDefault](/ml-engine/reference/rest/v1/projects.models.versions/setDefault). |
| `name` | String | Required. The name specified for the model when it was created.

The model name must be unique within the project it is created in. |
| `labels` | HashMap<String, String> | Optional. One or more labels that you can add, to organize your models.
Each label is a key-value pair, where both the key and the value are
arbitrary strings that you supply.
For more information, see the documentation on
<a href="/ml-engine/docs/tensorflow/resource-labels">using labels</a>. |
| `description` | String | Optional. The description specified for the model when it was created. |
| `online_prediction_logging` | bool | Optional. If true, enables StackDriver Logging for online prediction.
Default is false. |
| `regions` | Vec<String> | Optional. The list of regions where the model is going to be deployed.
Currently only one region per model is supported.
Defaults to 'us-central1' if nothing is set.
See the <a href="/ml-engine/docs/tensorflow/regions">available regions</a>
for ML Engine services.
Note:
*   No matter where a model is deployed, it can always be accessed by
    users from anywhere, both for online and batch prediction.
*   The region for a batch prediction job is set by the region field when
    submitting the batch prediction job and does not take its value from
    this field. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a model from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform model updates in order to avoid race
conditions: An `etag` is returned in the response to `GetModel`, and
systems are expected to put that etag in the request to `UpdateModel` to
ensure that their change will be applied to the model as intended. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model
model = provider.json_body.Model {
    parent = "value"  # Required. The project name.
}

# Access model outputs
model_id = model.id
model_default_version = model.default_version
model_name = model.name
model_labels = model.labels
model_description = model.description
model_online_prediction_logging = model.online_prediction_logging
model_regions = model.regions
model_etag = model.etag
```

---


### Version

Creates a new version of a model from a trained TensorFlow model.

If the version created in the cloud by this call is the first deployed
version of the specified model, it will be made the default version of the
model. When you add a version to a model that already has one or more
versions, the default version does not automatically change. If you want a
new version to be the default, you must call
[projects.models.versions.setDefault](/ml-engine/reference/rest/v1/projects.models.versions/setDefault).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time the version was created. |
| `labels` | HashMap<String, String> |  | Optional. One or more labels that you can add, to organize your model
versions. Each label is a key-value pair, where both the key and the value
are arbitrary strings that you supply.
For more information, see the documentation on
<a href="/ml-engine/docs/tensorflow/resource-labels">using labels</a>. |
| `python_version` | String |  | Optional. The version of Python used in prediction. If not set, the default
version is '2.7'. Python '3.5' is available when `runtime_version` is set
to '1.4' and above. Python '2.7' works with all supported runtime versions. |
| `manual_scaling` | String |  | Manually select the number of nodes to use for serving the
model. You should generally use `auto_scaling` with an appropriate
`min_nodes` instead, but this option is available if you want more
predictable billing. Beware that latency and error rates will increase
if the traffic exceeds that capability of the system to serve it based
on the selected number of nodes. |
| `name` | String |  | Required.The name specified for the version when it was created.

The version name must be unique within the model it is created in. |
| `error_message` | String |  | Output only. The details of a failure or a cancellation. |
| `etag` | String |  | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a model from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform model updates in order to avoid race
conditions: An `etag` is returned in the response to `GetVersion`, and
systems are expected to put that etag in the request to `UpdateVersion` to
ensure that their change will be applied to the model as intended. |
| `framework` | String |  | Optional. The machine learning framework Cloud ML Engine uses to train
this version of the model. Valid values are `TENSORFLOW`, `SCIKIT_LEARN`,
`XGBOOST`. If you do not specify a framework, Cloud ML Engine
will analyze files in the deployment_uri to determine a framework. If you
choose `SCIKIT_LEARN` or `XGBOOST`, you must also set the runtime version
of the model to 1.4 or greater. |
| `runtime_version` | String |  | Optional. The Google Cloud ML runtime version to use for this deployment.
If not set, Google Cloud ML will choose a version. |
| `is_default` | bool |  | Output only. If true, this version will be used to handle prediction
requests that do not specify a version.

You can change the default version by calling
[projects.methods.versions.setDefault](/ml-engine/reference/rest/v1/projects.models.versions/setDefault). |
| `auto_scaling` | String |  | Automatically scale the number of nodes used to serve the model in
response to increases and decreases in traffic. Care should be
taken to ramp up traffic according to the model's ability to scale
or you will start seeing increases in latency and 429 response codes. |
| `description` | String |  | Optional. The description specified for the version when it was created. |
| `state` | String |  | Output only. The state of a version. |
| `deployment_uri` | String |  | Required. The Google Cloud Storage location of the trained model used to
create the version. See the
[guide to model
deployment](/ml-engine/docs/tensorflow/deploying-models) for more
information.

When passing Version to
[projects.models.versions.create](/ml-engine/reference/rest/v1/projects.models.versions/create)
the model service uses the specified location as the source of the model.
Once deployed, the model version is hosted by the prediction service, so
this location is useful only as a historical record.
The total number of model files can't exceed 1000. |
| `machine_type` | String |  | Optional. The type of machine on which to serve the model. Currently only
applies to online prediction service.
The following are currently supported and will be deprecated in
Beta release.
  mls1-highmem-1    1 core    2 Gb RAM
  mls1-highcpu-4    4 core    2 Gb RAM
The following are available in Beta:
  mls1-c1-m2        1 core    2 Gb RAM   Default
  mls1-c4-m2        4 core    2 Gb RAM |
| `last_use_time` | String |  | Output only. The time the version was last used for prediction. |
| `parent` | String | ✅ | Required. The name of the model. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time the version was created. |
| `labels` | HashMap<String, String> | Optional. One or more labels that you can add, to organize your model
versions. Each label is a key-value pair, where both the key and the value
are arbitrary strings that you supply.
For more information, see the documentation on
<a href="/ml-engine/docs/tensorflow/resource-labels">using labels</a>. |
| `python_version` | String | Optional. The version of Python used in prediction. If not set, the default
version is '2.7'. Python '3.5' is available when `runtime_version` is set
to '1.4' and above. Python '2.7' works with all supported runtime versions. |
| `manual_scaling` | String | Manually select the number of nodes to use for serving the
model. You should generally use `auto_scaling` with an appropriate
`min_nodes` instead, but this option is available if you want more
predictable billing. Beware that latency and error rates will increase
if the traffic exceeds that capability of the system to serve it based
on the selected number of nodes. |
| `name` | String | Required.The name specified for the version when it was created.

The version name must be unique within the model it is created in. |
| `error_message` | String | Output only. The details of a failure or a cancellation. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a model from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform model updates in order to avoid race
conditions: An `etag` is returned in the response to `GetVersion`, and
systems are expected to put that etag in the request to `UpdateVersion` to
ensure that their change will be applied to the model as intended. |
| `framework` | String | Optional. The machine learning framework Cloud ML Engine uses to train
this version of the model. Valid values are `TENSORFLOW`, `SCIKIT_LEARN`,
`XGBOOST`. If you do not specify a framework, Cloud ML Engine
will analyze files in the deployment_uri to determine a framework. If you
choose `SCIKIT_LEARN` or `XGBOOST`, you must also set the runtime version
of the model to 1.4 or greater. |
| `runtime_version` | String | Optional. The Google Cloud ML runtime version to use for this deployment.
If not set, Google Cloud ML will choose a version. |
| `is_default` | bool | Output only. If true, this version will be used to handle prediction
requests that do not specify a version.

You can change the default version by calling
[projects.methods.versions.setDefault](/ml-engine/reference/rest/v1/projects.models.versions/setDefault). |
| `auto_scaling` | String | Automatically scale the number of nodes used to serve the model in
response to increases and decreases in traffic. Care should be
taken to ramp up traffic according to the model's ability to scale
or you will start seeing increases in latency and 429 response codes. |
| `description` | String | Optional. The description specified for the version when it was created. |
| `state` | String | Output only. The state of a version. |
| `deployment_uri` | String | Required. The Google Cloud Storage location of the trained model used to
create the version. See the
[guide to model
deployment](/ml-engine/docs/tensorflow/deploying-models) for more
information.

When passing Version to
[projects.models.versions.create](/ml-engine/reference/rest/v1/projects.models.versions/create)
the model service uses the specified location as the source of the model.
Once deployed, the model version is hosted by the prediction service, so
this location is useful only as a historical record.
The total number of model files can't exceed 1000. |
| `machine_type` | String | Optional. The type of machine on which to serve the model. Currently only
applies to online prediction service.
The following are currently supported and will be deprecated in
Beta release.
  mls1-highmem-1    1 core    2 Gb RAM
  mls1-highcpu-4    4 core    2 Gb RAM
The following are available in Beta:
  mls1-c1-m2        1 core    2 Gb RAM   Default
  mls1-c4-m2        4 core    2 Gb RAM |
| `last_use_time` | String | Output only. The time the version was last used for prediction. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.json_body.Version {
    parent = "value"  # Required. The name of the model.
}

# Access version outputs
version_id = version.id
version_create_time = version.create_time
version_labels = version.labels
version_python_version = version.python_version
version_manual_scaling = version.manual_scaling
version_name = version.name
version_error_message = version.error_message
version_etag = version.etag
version_framework = version.framework
version_runtime_version = version.runtime_version
version_is_default = version.is_default
version_auto_scaling = version.auto_scaling
version_description = version.description
version_state = version.state
version_deployment_uri = version.deployment_uri
version_machine_type = version.machine_type
version_last_use_time = version.last_use_time
```

---


### Job

Creates a training or a batch prediction job.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `training_input` | String |  | Input parameters to create a training job. |
| `training_output` | String |  | The current training job result. |
| `prediction_output` | String |  | The current prediction job result. |
| `job_id` | String |  | Required. The user-specified id of the job. |
| `error_message` | String |  | Output only. The details of a failure or a cancellation. |
| `end_time` | String |  | Output only. When the job processing was completed. |
| `create_time` | String |  | Output only. When the job was created. |
| `etag` | String |  | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a job from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform job updates in order to avoid race
conditions: An `etag` is returned in the response to `GetJob`, and
systems are expected to put that etag in the request to `UpdateJob` to
ensure that their change will be applied to the same version of the job. |
| `prediction_input` | String |  | Input parameters to create a prediction job. |
| `labels` | HashMap<String, String> |  | Optional. One or more labels that you can add, to organize your jobs.
Each label is a key-value pair, where both the key and the value are
arbitrary strings that you supply.
For more information, see the documentation on
<a href="/ml-engine/docs/tensorflow/resource-labels">using labels</a>. |
| `state` | String |  | Output only. The detailed state of a job. |
| `start_time` | String |  | Output only. When the job processing was started. |
| `parent` | String | ✅ | Required. The project name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `training_input` | String | Input parameters to create a training job. |
| `training_output` | String | The current training job result. |
| `prediction_output` | String | The current prediction job result. |
| `job_id` | String | Required. The user-specified id of the job. |
| `error_message` | String | Output only. The details of a failure or a cancellation. |
| `end_time` | String | Output only. When the job processing was completed. |
| `create_time` | String | Output only. When the job was created. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a job from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform job updates in order to avoid race
conditions: An `etag` is returned in the response to `GetJob`, and
systems are expected to put that etag in the request to `UpdateJob` to
ensure that their change will be applied to the same version of the job. |
| `prediction_input` | String | Input parameters to create a prediction job. |
| `labels` | HashMap<String, String> | Optional. One or more labels that you can add, to organize your jobs.
Each label is a key-value pair, where both the key and the value are
arbitrary strings that you supply.
For more information, see the documentation on
<a href="/ml-engine/docs/tensorflow/resource-labels">using labels</a>. |
| `state` | String | Output only. The detailed state of a job. |
| `start_time` | String | Output only. When the job processing was started. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.json_body.Job {
    parent = "value"  # Required. The project name.
}

# Access job outputs
job_id = job.id
job_training_input = job.training_input
job_training_output = job.training_output
job_prediction_output = job.prediction_output
job_job_id = job.job_id
job_error_message = job.error_message
job_end_time = job.end_time
job_create_time = job.create_time
job_etag = job.etag
job_prediction_input = job.prediction_input
job_labels = job.labels
job_state = job.state
job_start_time = job.start_time
```

---


### Project

Performs prediction on the data in the request.
Cloud ML Engine implements a custom `predict` verb on top of an HTTP POST
method. <p>For details of the request and response format, see the **guide
to the [predict request format](/ml-engine/docs/v1/predict-request)**.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `http_body` | String |  | 
Required. The prediction request body. |
| `name` | String | ✅ | Required. The resource name of a model or a version.

Authorization: requires the `predict` permission on the specified resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_account` | String | The service account Cloud ML uses to access resources in the project. |
| `config` | String |  |
| `service_account_project` | String | The project number for `service_account`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.json_body.Project {
    name = "value"  # Required. The resource name of a model or a version.

Authorization: requires the `predict` permission on the specified resource.
}

# Access project outputs
project_id = project.id
project_service_account = project.service_account
project_config = project.config
project_service_account_project = project.service_account_project
```

---


### Operation

Starts asynchronous cancellation on a long-running operation.  The server
makes a best effort to cancel the operation, but success is not
guaranteed.  If the server doesn't support this method, it returns
`google.rpc.Code.UNIMPLEMENTED`.  Clients can use
Operations.GetOperation or
other methods to check whether the cancellation succeeded or whether the
operation completed despite cancellation. On successful cancellation,
the operation is not deleted; instead, it becomes an operation with
an Operation.error value with a google.rpc.Status.code of 1,
corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress.
If `true`, the operation is completed, and either `error` or `response` is
available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success.  If the original
method returns no data on success, such as `Delete`, the response is
`google.protobuf.Empty`.  If the original method is standard
`Get`/`Create`/`Update`, the response should be the resource.  For other
methods, the response should have the type `XxxResponse`, where `Xxx`
is the original method name.  For example, if the original method name
is `TakeSnapshot()`, the inferred response type is
`TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation.  It typically
contains progress information and common metadata such as create time.
Some services might not provide such metadata.  Any method that returns a
long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that
originally returns it. If you use the default HTTP mapping, the
`name` should have the format of `operations/some/unique/name`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.json_body.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Location

Get the complete list of CMLE capabilities in a location, along with their
location-specific properties.

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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple model resources
model_0 = provider.json_body.Model {
    parent = "value-0"
}
model_1 = provider.json_body.Model {
    parent = "value-1"
}
model_2 = provider.json_body.Model {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    model = provider.json_body.Model {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Json_body Documentation](https://cloud.google.com/json_body/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
