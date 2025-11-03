# Dataproc_api Service



**Resources**: 15

---

## Overview

The dataproc_api service provides access to 15 resource types:

- [Session_template](#session_template) [CRUD]
- [Operation](#operation) [CRD]
- [Batche](#batche) [CRD]
- [Cluster](#cluster) [CRUD]
- [Node_group](#node_group) [CR]
- [Job](#job) [CRUD]
- [Spark_application](#spark_application) [CR]
- [Workflow_template](#workflow_template) [CRUD]
- [Session](#session) [CRD]
- [Autoscaling_policie](#autoscaling_policie) [CRUD]
- [Cluster](#cluster) [CRUD]
- [Autoscaling_policie](#autoscaling_policie) [CRUD]
- [Workflow_template](#workflow_template) [CRUD]
- [Job](#job) [CRUD]
- [Operation](#operation) [CRD]

---

## Resources


### Session_template

Create a session template synchronously.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creator` | String |  | Output only. The email address of the user who created the template. |
| `update_time` | String |  | Output only. The time the template was last updated. |
| `environment_config` | String |  | Optional. Environment configuration for session execution. |
| `description` | String |  | Optional. Brief description of the template. |
| `labels` | HashMap<String, String> |  | Optional. Labels to associate with sessions created using this template. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values can be empty, but, if present, must contain 1 to 63 characters and conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a session. |
| `uuid` | String |  | Output only. A session template UUID (Unique Universal Identifier). The service generates this value when it creates the session template. |
| `create_time` | String |  | Output only. The time when the template was created. |
| `jupyter_session` | String |  | Optional. Jupyter session config. |
| `spark_connect_session` | String |  | Optional. Spark connect session config. |
| `name` | String |  | Required. Identifier. The resource name of the session template. |
| `runtime_config` | String |  | Optional. Runtime configuration for session execution. |
| `parent` | String | ✅ | Required. The parent resource where this session template will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creator` | String | Output only. The email address of the user who created the template. |
| `update_time` | String | Output only. The time the template was last updated. |
| `environment_config` | String | Optional. Environment configuration for session execution. |
| `description` | String | Optional. Brief description of the template. |
| `labels` | HashMap<String, String> | Optional. Labels to associate with sessions created using this template. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values can be empty, but, if present, must contain 1 to 63 characters and conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a session. |
| `uuid` | String | Output only. A session template UUID (Unique Universal Identifier). The service generates this value when it creates the session template. |
| `create_time` | String | Output only. The time when the template was created. |
| `jupyter_session` | String | Optional. Jupyter session config. |
| `spark_connect_session` | String | Optional. Spark connect session config. |
| `name` | String | Required. Identifier. The resource name of the session template. |
| `runtime_config` | String | Optional. Runtime configuration for session execution. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create session_template
session_template = provider.dataproc_api.Session_template {
    parent = "value"  # Required. The parent resource where this session template will be created.
}

# Access session_template outputs
session_template_id = session_template.id
session_template_creator = session_template.creator
session_template_update_time = session_template.update_time
session_template_environment_config = session_template.environment_config
session_template_description = session_template.description
session_template_labels = session_template.labels
session_template_uuid = session_template.uuid
session_template_create_time = session_template.create_time
session_template_jupyter_session = session_template.jupyter_session
session_template_spark_connect_session = session_template.spark_connect_session
session_template_name = session_template.name
session_template_runtime_config = session_template.runtime_config
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.dataproc_api.Operation {
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


### Batche

Creates a batch workload that executes asynchronously.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `operation` | String |  | Output only. The resource name of the operation associated with this batch. |
| `spark_sql_batch` | String |  | Optional. SparkSql batch config. |
| `spark_r_batch` | String |  | Optional. SparkR batch config. |
| `state` | String |  | Output only. The state of the batch. |
| `uuid` | String |  | Output only. A batch UUID (Unique Universal Identifier). The service generates this value when it creates the batch. |
| `create_time` | String |  | Output only. The time when the batch was created. |
| `environment_config` | String |  | Optional. Environment configuration for the batch execution. |
| `pyspark_batch` | String |  | Optional. PySpark batch config. |
| `runtime_config` | String |  | Optional. Runtime configuration for the batch execution. |
| `state_time` | String |  | Output only. The time when the batch entered a current state. |
| `name` | String |  | Output only. The resource name of the batch. |
| `state_message` | String |  | Output only. Batch state details, such as a failure description if the state is FAILED. |
| `spark_batch` | String |  | Optional. Spark batch config. |
| `creator` | String |  | Output only. The email address of the user who created the batch. |
| `state_history` | Vec<String> |  | Output only. Historical state information for the batch. |
| `labels` | HashMap<String, String> |  | Optional. The labels to associate with this batch. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a batch. |
| `runtime_info` | String |  | Output only. Runtime information about batch execution. |
| `parent` | String | ✅ | Required. The parent resource where this batch will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `operation` | String | Output only. The resource name of the operation associated with this batch. |
| `spark_sql_batch` | String | Optional. SparkSql batch config. |
| `spark_r_batch` | String | Optional. SparkR batch config. |
| `state` | String | Output only. The state of the batch. |
| `uuid` | String | Output only. A batch UUID (Unique Universal Identifier). The service generates this value when it creates the batch. |
| `create_time` | String | Output only. The time when the batch was created. |
| `environment_config` | String | Optional. Environment configuration for the batch execution. |
| `pyspark_batch` | String | Optional. PySpark batch config. |
| `runtime_config` | String | Optional. Runtime configuration for the batch execution. |
| `state_time` | String | Output only. The time when the batch entered a current state. |
| `name` | String | Output only. The resource name of the batch. |
| `state_message` | String | Output only. Batch state details, such as a failure description if the state is FAILED. |
| `spark_batch` | String | Optional. Spark batch config. |
| `creator` | String | Output only. The email address of the user who created the batch. |
| `state_history` | Vec<String> | Output only. Historical state information for the batch. |
| `labels` | HashMap<String, String> | Optional. The labels to associate with this batch. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a batch. |
| `runtime_info` | String | Output only. Runtime information about batch execution. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create batche
batche = provider.dataproc_api.Batche {
    parent = "value"  # Required. The parent resource where this batch will be created.
}

# Access batche outputs
batche_id = batche.id
batche_operation = batche.operation
batche_spark_sql_batch = batche.spark_sql_batch
batche_spark_r_batch = batche.spark_r_batch
batche_state = batche.state
batche_uuid = batche.uuid
batche_create_time = batche.create_time
batche_environment_config = batche.environment_config
batche_pyspark_batch = batche.pyspark_batch
batche_runtime_config = batche.runtime_config
batche_state_time = batche.state_time
batche_name = batche.name
batche_state_message = batche.state_message
batche_spark_batch = batche.spark_batch
batche_creator = batche.creator
batche_state_history = batche.state_history
batche_labels = batche.labels
batche_runtime_info = batche.runtime_info
```

---


### Cluster

Creates a cluster in a project. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#clusteroperationmetadata).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. The labels to associate with this cluster. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a cluster. |
| `status_history` | Vec<String> |  | Output only. The previous cluster status. |
| `cluster_name` | String |  | Required. The cluster name, which must be unique within a project. The name must start with a lowercase letter, and can contain up to 51 lowercase letters, numbers, and hyphens. It cannot end with a hyphen. The name of a deleted cluster can be reused. |
| `project_id` | String |  | Required. The Google Cloud Platform project ID that the cluster belongs to. |
| `cluster_uuid` | String |  | Output only. A cluster UUID (Unique Universal Identifier). Dataproc generates this value when it creates the cluster. |
| `virtual_cluster_config` | String |  | Optional. The virtual cluster config is used when creating a Dataproc cluster that does not directly control the underlying compute resources, for example, when creating a Dataproc-on-GKE cluster (https://cloud.google.com/dataproc/docs/guides/dpgke/dataproc-gke-overview). Dataproc may set default values, and values may change when clusters are updated. Exactly one of config or virtual_cluster_config must be specified. |
| `metrics` | String |  | Output only. Contains cluster daemon metrics such as HDFS and YARN stats.Beta Feature: This report is available for testing purposes only. It may be changed before final release. |
| `config` | String |  | Optional. The cluster config for a cluster of Compute Engine Instances. Note that Dataproc may set default values, and values may change when clusters are updated.Exactly one of ClusterConfig or VirtualClusterConfig must be specified. |
| `status` | String |  | Output only. Cluster status. |
| `project_id` | String | ✅ | Required. The ID of the Google Cloud Platform project that the cluster belongs to. |
| `region` | String | ✅ | Required. The Dataproc region in which to handle the request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. The labels to associate with this cluster. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a cluster. |
| `status_history` | Vec<String> | Output only. The previous cluster status. |
| `cluster_name` | String | Required. The cluster name, which must be unique within a project. The name must start with a lowercase letter, and can contain up to 51 lowercase letters, numbers, and hyphens. It cannot end with a hyphen. The name of a deleted cluster can be reused. |
| `project_id` | String | Required. The Google Cloud Platform project ID that the cluster belongs to. |
| `cluster_uuid` | String | Output only. A cluster UUID (Unique Universal Identifier). Dataproc generates this value when it creates the cluster. |
| `virtual_cluster_config` | String | Optional. The virtual cluster config is used when creating a Dataproc cluster that does not directly control the underlying compute resources, for example, when creating a Dataproc-on-GKE cluster (https://cloud.google.com/dataproc/docs/guides/dpgke/dataproc-gke-overview). Dataproc may set default values, and values may change when clusters are updated. Exactly one of config or virtual_cluster_config must be specified. |
| `metrics` | String | Output only. Contains cluster daemon metrics such as HDFS and YARN stats.Beta Feature: This report is available for testing purposes only. It may be changed before final release. |
| `config` | String | Optional. The cluster config for a cluster of Compute Engine Instances. Note that Dataproc may set default values, and values may change when clusters are updated.Exactly one of ClusterConfig or VirtualClusterConfig must be specified. |
| `status` | String | Output only. Cluster status. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.dataproc_api.Cluster {
    project_id = "value"  # Required. The ID of the Google Cloud Platform project that the cluster belongs to.
    region = "value"  # Required. The Dataproc region in which to handle the request.
}

# Access cluster outputs
cluster_id = cluster.id
cluster_labels = cluster.labels
cluster_status_history = cluster.status_history
cluster_cluster_name = cluster.cluster_name
cluster_project_id = cluster.project_id
cluster_cluster_uuid = cluster.cluster_uuid
cluster_virtual_cluster_config = cluster.virtual_cluster_config
cluster_metrics = cluster.metrics
cluster_config = cluster.config
cluster_status = cluster.status
```

---


### Node_group

Creates a node group in a cluster. The returned Operation.metadata is NodeGroupOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#nodegroupoperationmetadata).

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The Node group resource name (https://aip.dev/122). |
| `roles` | Vec<String> |  | Required. Node group roles. |
| `node_group_config` | String |  | Optional. The node group instance group configuration. |
| `labels` | HashMap<String, String> |  | Optional. Node group labels. Label keys must consist of from 1 to 63 characters and conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values can be empty. If specified, they must consist of from 1 to 63 characters and conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). The node group must have no more than 32 labels. |
| `parent` | String | ✅ | Required. The parent resource where this node group will be created. Format: projects/{project}/regions/{region}/clusters/{cluster} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The Node group resource name (https://aip.dev/122). |
| `roles` | Vec<String> | Required. Node group roles. |
| `node_group_config` | String | Optional. The node group instance group configuration. |
| `labels` | HashMap<String, String> | Optional. Node group labels. Label keys must consist of from 1 to 63 characters and conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values can be empty. If specified, they must consist of from 1 to 63 characters and conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). The node group must have no more than 32 labels. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create node_group
node_group = provider.dataproc_api.Node_group {
    parent = "value"  # Required. The parent resource where this node group will be created. Format: projects/{project}/regions/{region}/clusters/{cluster}
}

# Access node_group outputs
node_group_id = node_group.id
node_group_name = node_group.name
node_group_roles = node_group.roles
node_group_node_group_config = node_group.node_group_config
node_group_labels = node_group.labels
```

---


### Job

Submits job to a cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_id` | String |  | Optional. A unique id used to identify the request. If the server receives two SubmitJobRequest (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1#google.cloud.dataproc.v1.SubmitJobRequest)s with the same id, then the second request will be ignored and the first Job created and stored in the backend is returned.It is recommended to always set this value to a UUID (https://en.wikipedia.org/wiki/Universally_unique_identifier).The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). The maximum length is 40 characters. |
| `job` | String |  | Required. The job resource. |
| `region` | String | ✅ | Required. The Dataproc region in which to handle the request. |
| `project_id` | String | ✅ | Required. The ID of the Google Cloud Platform project that the job belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_history` | Vec<String> | Output only. The previous job status. |
| `placement` | String | Required. Job information, including how, when, and where to run the job. |
| `presto_job` | String | Optional. Job is a Presto job. |
| `done` | bool | Output only. Indicates whether the job is completed. If the value is false, the job is still in progress. If true, the job is completed, and status.state field will indicate if it was successful, failed, or cancelled. |
| `spark_job` | String | Optional. Job is a Spark job. |
| `spark_sql_job` | String | Optional. Job is a SparkSql job. |
| `yarn_applications` | Vec<String> | Output only. The collection of YARN applications spun up by this job.Beta Feature: This report is available for testing purposes only. It might be changed before final release. |
| `status` | String | Output only. The job status. Additional application-specific status information might be contained in the type_job and yarn_applications fields. |
| `hive_job` | String | Optional. Job is a Hive job. |
| `job_uuid` | String | Output only. A UUID that uniquely identifies a job within the project over time. This is in contrast to a user-settable reference.job_id that might be reused over time. |
| `scheduling` | String | Optional. Job scheduling configuration. |
| `driver_control_files_uri` | String | Output only. If present, the location of miscellaneous control files which can be used as part of job setup and handling. If not present, control files might be placed in the same location as driver_output_uri. |
| `driver_output_resource_uri` | String | Output only. A URI pointing to the location of the stdout of the job's driver program. |
| `labels` | HashMap<String, String> | Optional. The labels to associate with this job. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values can be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a job. |
| `pyspark_job` | String | Optional. Job is a PySpark job. |
| `reference` | String | Optional. The fully qualified reference to the job, which can be used to obtain the equivalent REST path of the job resource. If this property is not specified when a job is created, the server generates a job_id. |
| `pig_job` | String | Optional. Job is a Pig job. |
| `driver_scheduling_config` | String | Optional. Driver scheduling configuration. |
| `spark_r_job` | String | Optional. Job is a SparkR job. |
| `trino_job` | String | Optional. Job is a Trino job. |
| `hadoop_job` | String | Optional. Job is a Hadoop job. |
| `flink_job` | String | Optional. Job is a Flink job. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.dataproc_api.Job {
    region = "value"  # Required. The Dataproc region in which to handle the request.
    project_id = "value"  # Required. The ID of the Google Cloud Platform project that the job belongs to.
}

# Access job outputs
job_id = job.id
job_status_history = job.status_history
job_placement = job.placement
job_presto_job = job.presto_job
job_done = job.done
job_spark_job = job.spark_job
job_spark_sql_job = job.spark_sql_job
job_yarn_applications = job.yarn_applications
job_status = job.status
job_hive_job = job.hive_job
job_job_uuid = job.job_uuid
job_scheduling = job.scheduling
job_driver_control_files_uri = job.driver_control_files_uri
job_driver_output_resource_uri = job.driver_output_resource_uri
job_labels = job.labels
job_pyspark_job = job.pyspark_job
job_reference = job.reference
job_pig_job = job.pig_job
job_driver_scheduling_config = job.driver_scheduling_config
job_spark_r_job = job.spark_r_job
job_trino_job = job.trino_job
job_hadoop_job = job.hadoop_job
job_flink_job = job.flink_job
```

---


### Spark_application

Write wrapper objects from dataplane to spanner

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String |  | Required. Parent (Batch) resource reference. |
| `spark_wrapper_objects` | Vec<String> |  | Required. The batch of spark application context objects sent for ingestion. |
| `name` | String | ✅ | Required. The fully qualified name of the spark application to write data about in the format "projects/PROJECT_ID/locations/DATAPROC_REGION/sessions/SESSION_ID/sparkApplications/APPLICATION_ID" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | This token is included in the response if there are more results to fetch. To fetch additional results, provide this value as the page_token in a subsequent SearchSessionSparkApplicationStages. |
| `spark_application_stages` | Vec<String> | Output only. Data corresponding to a stage. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create spark_application
spark_application = provider.dataproc_api.Spark_application {
    name = "value"  # Required. The fully qualified name of the spark application to write data about in the format "projects/PROJECT_ID/locations/DATAPROC_REGION/sessions/SESSION_ID/sparkApplications/APPLICATION_ID"
}

# Access spark_application outputs
spark_application_id = spark_application.id
spark_application_next_page_token = spark_application.next_page_token
spark_application_spark_application_stages = spark_application.spark_application_stages
```

---


### Workflow_template

Creates new workflow template.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `placement` | String |  | Required. WorkflowTemplate scheduling information. |
| `version` | i64 |  | Optional. Used to perform a consistent read-modify-write.This field should be left blank for a CreateWorkflowTemplate request. It is required for an UpdateWorkflowTemplate request, and must match the current server version. A typical update template flow would fetch the current template with a GetWorkflowTemplate request, which will return the current template with the version field filled in with the current server version. The user updates other fields in the template, then returns it as part of the UpdateWorkflowTemplate request. |
| `dag_timeout` | String |  | Optional. Timeout duration for the DAG of jobs, expressed in seconds (see JSON representation of duration (https://developers.google.com/protocol-buffers/docs/proto3#json)). The timeout duration must be from 10 minutes ("600s") to 24 hours ("86400s"). The timer begins when the first job is submitted. If the workflow is running at the end of the timeout period, any remaining jobs are cancelled, the workflow is ended, and if the workflow was running on a managed cluster, the cluster is deleted. |
| `create_time` | String |  | Output only. The time template was created. |
| `id` | String |  |  |
| `update_time` | String |  | Output only. The time template was last updated. |
| `labels` | HashMap<String, String> |  | Optional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created by the workflow instance.Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).No more than 32 labels can be associated with a template. |
| `name` | String |  | Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id} |
| `jobs` | Vec<String> |  | Required. The Directed Acyclic Graph of Jobs to submit. |
| `parameters` | Vec<String> |  | Optional. Template parameters whose values are substituted into the template. Values for parameters must be provided when the template is instantiated. |
| `encryption_config` | String |  | Optional. Encryption settings for encrypting workflow template job arguments. |
| `parent` | String | ✅ | Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.create, the resource name of the location has the following format: projects/{project_id}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `placement` | String | Required. WorkflowTemplate scheduling information. |
| `version` | i64 | Optional. Used to perform a consistent read-modify-write.This field should be left blank for a CreateWorkflowTemplate request. It is required for an UpdateWorkflowTemplate request, and must match the current server version. A typical update template flow would fetch the current template with a GetWorkflowTemplate request, which will return the current template with the version field filled in with the current server version. The user updates other fields in the template, then returns it as part of the UpdateWorkflowTemplate request. |
| `dag_timeout` | String | Optional. Timeout duration for the DAG of jobs, expressed in seconds (see JSON representation of duration (https://developers.google.com/protocol-buffers/docs/proto3#json)). The timeout duration must be from 10 minutes ("600s") to 24 hours ("86400s"). The timer begins when the first job is submitted. If the workflow is running at the end of the timeout period, any remaining jobs are cancelled, the workflow is ended, and if the workflow was running on a managed cluster, the cluster is deleted. |
| `create_time` | String | Output only. The time template was created. |
| `id` | String |  |
| `update_time` | String | Output only. The time template was last updated. |
| `labels` | HashMap<String, String> | Optional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created by the workflow instance.Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).No more than 32 labels can be associated with a template. |
| `name` | String | Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id} |
| `jobs` | Vec<String> | Required. The Directed Acyclic Graph of Jobs to submit. |
| `parameters` | Vec<String> | Optional. Template parameters whose values are substituted into the template. Values for parameters must be provided when the template is instantiated. |
| `encryption_config` | String | Optional. Encryption settings for encrypting workflow template job arguments. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workflow_template
workflow_template = provider.dataproc_api.Workflow_template {
    parent = "value"  # Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.create, the resource name of the location has the following format: projects/{project_id}/locations/{location}
}

# Access workflow_template outputs
workflow_template_id = workflow_template.id
workflow_template_placement = workflow_template.placement
workflow_template_version = workflow_template.version
workflow_template_dag_timeout = workflow_template.dag_timeout
workflow_template_create_time = workflow_template.create_time
workflow_template_id = workflow_template.id
workflow_template_update_time = workflow_template.update_time
workflow_template_labels = workflow_template.labels
workflow_template_name = workflow_template.name
workflow_template_jobs = workflow_template.jobs
workflow_template_parameters = workflow_template.parameters
workflow_template_encryption_config = workflow_template.encryption_config
```

---


### Session

Create an interactive session asynchronously.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state_time` | String |  | Output only. The time when the session entered the current state. |
| `runtime_config` | String |  | Optional. Runtime configuration for the session execution. |
| `name` | String |  | Identifier. The resource name of the session. |
| `uuid` | String |  | Output only. A session UUID (Unique Universal Identifier). The service generates this value when it creates the session. |
| `runtime_info` | String |  | Output only. Runtime information about session execution. |
| `create_time` | String |  | Output only. The time when the session was created. |
| `session_template` | String |  | Optional. The session template used by the session.Only resource names, including project ID and location, are valid.Example: * https://www.googleapis.com/compute/v1/projects/[project_id]/locations/[dataproc_region]/sessionTemplates/[template_id] * projects/[project_id]/locations/[dataproc_region]/sessionTemplates/[template_id]The template must be in the same project and Dataproc region as the session. |
| `state` | String |  | Output only. A state of the session. |
| `state_message` | String |  | Output only. Session state details, such as the failure description if the state is FAILED. |
| `spark_connect_session` | String |  | Optional. Spark connect session config. |
| `jupyter_session` | String |  | Optional. Jupyter session config. |
| `user` | String |  | Optional. The email address of the user who owns the session. |
| `creator` | String |  | Output only. The email address of the user who created the session. |
| `state_history` | Vec<String> |  | Output only. Historical state information for the session. |
| `labels` | HashMap<String, String> |  | Optional. The labels to associate with the session. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a session. |
| `environment_config` | String |  | Optional. Environment configuration for the session execution. |
| `parent` | String | ✅ | Required. The parent resource where this session will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state_time` | String | Output only. The time when the session entered the current state. |
| `runtime_config` | String | Optional. Runtime configuration for the session execution. |
| `name` | String | Identifier. The resource name of the session. |
| `uuid` | String | Output only. A session UUID (Unique Universal Identifier). The service generates this value when it creates the session. |
| `runtime_info` | String | Output only. Runtime information about session execution. |
| `create_time` | String | Output only. The time when the session was created. |
| `session_template` | String | Optional. The session template used by the session.Only resource names, including project ID and location, are valid.Example: * https://www.googleapis.com/compute/v1/projects/[project_id]/locations/[dataproc_region]/sessionTemplates/[template_id] * projects/[project_id]/locations/[dataproc_region]/sessionTemplates/[template_id]The template must be in the same project and Dataproc region as the session. |
| `state` | String | Output only. A state of the session. |
| `state_message` | String | Output only. Session state details, such as the failure description if the state is FAILED. |
| `spark_connect_session` | String | Optional. Spark connect session config. |
| `jupyter_session` | String | Optional. Jupyter session config. |
| `user` | String | Optional. The email address of the user who owns the session. |
| `creator` | String | Output only. The email address of the user who created the session. |
| `state_history` | Vec<String> | Output only. Historical state information for the session. |
| `labels` | HashMap<String, String> | Optional. The labels to associate with the session. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a session. |
| `environment_config` | String | Optional. Environment configuration for the session execution. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create session
session = provider.dataproc_api.Session {
    parent = "value"  # Required. The parent resource where this session will be created.
}

# Access session outputs
session_id = session.id
session_state_time = session.state_time
session_runtime_config = session.runtime_config
session_name = session.name
session_uuid = session.uuid
session_runtime_info = session.runtime_info
session_create_time = session.create_time
session_session_template = session.session_template
session_state = session.state
session_state_message = session.state_message
session_spark_connect_session = session.spark_connect_session
session_jupyter_session = session.jupyter_session
session_user = session.user
session_creator = session.creator
session_state_history = session.state_history
session_labels = session.labels
session_environment_config = session.environment_config
```

---


### Autoscaling_policie

Creates new autoscaling policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `basic_algorithm` | String |  |  |
| `worker_config` | String |  | Required. Describes how the autoscaler will operate for primary workers. |
| `secondary_worker_config` | String |  | Optional. Describes how the autoscaler will operate for secondary workers. |
| `id` | String |  | Required. The policy id.The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters. |
| `cluster_type` | String |  | Optional. The type of the clusters for which this autoscaling policy is to be configured. |
| `name` | String |  | Output only. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id} |
| `labels` | HashMap<String, String> |  | Optional. The labels to associate with this autoscaling policy. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with an autoscaling policy. |
| `parent` | String | ✅ | Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.create, the resource name of the location has the following format: projects/{project_id}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `basic_algorithm` | String |  |
| `worker_config` | String | Required. Describes how the autoscaler will operate for primary workers. |
| `secondary_worker_config` | String | Optional. Describes how the autoscaler will operate for secondary workers. |
| `id` | String | Required. The policy id.The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters. |
| `cluster_type` | String | Optional. The type of the clusters for which this autoscaling policy is to be configured. |
| `name` | String | Output only. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id} |
| `labels` | HashMap<String, String> | Optional. The labels to associate with this autoscaling policy. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with an autoscaling policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create autoscaling_policie
autoscaling_policie = provider.dataproc_api.Autoscaling_policie {
    parent = "value"  # Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.create, the resource name of the location has the following format: projects/{project_id}/locations/{location}
}

# Access autoscaling_policie outputs
autoscaling_policie_id = autoscaling_policie.id
autoscaling_policie_basic_algorithm = autoscaling_policie.basic_algorithm
autoscaling_policie_worker_config = autoscaling_policie.worker_config
autoscaling_policie_secondary_worker_config = autoscaling_policie.secondary_worker_config
autoscaling_policie_id = autoscaling_policie.id
autoscaling_policie_cluster_type = autoscaling_policie.cluster_type
autoscaling_policie_name = autoscaling_policie.name
autoscaling_policie_labels = autoscaling_policie.labels
```

---


### Cluster

Creates a cluster in a project. The returned Operation.metadata will be ClusterOperationMetadata (https://cloud.google.com/dataproc/docs/reference/rpc/google.cloud.dataproc.v1beta2#clusteroperationmetadata).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metrics` | String |  | Output only. Contains cluster daemon metrics such as HDFS and YARN stats.Beta Feature: This report is available for testing purposes only. It may be changed before final release. |
| `labels` | HashMap<String, String> |  | Optional. The labels to associate with this cluster. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a cluster. |
| `project_id` | String |  | Required. The Google Cloud Platform project ID that the cluster belongs to. |
| `cluster_uuid` | String |  | Output only. A cluster UUID (Unique Universal Identifier). Dataproc generates this value when it creates the cluster. |
| `config` | String |  | Required. The cluster config. Note that Dataproc may set default values, and values may change when clusters are updated. |
| `status` | String |  | Output only. Cluster status. |
| `cluster_name` | String |  | Required. The cluster name. Cluster names within a project must be unique. Names of deleted clusters can be reused. |
| `status_history` | Vec<String> |  | Output only. The previous cluster status. |
| `region` | String | ✅ | Required. The Dataproc region in which to handle the request. |
| `project_id` | String | ✅ | Required. The ID of the Google Cloud Platform project that the cluster belongs to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metrics` | String | Output only. Contains cluster daemon metrics such as HDFS and YARN stats.Beta Feature: This report is available for testing purposes only. It may be changed before final release. |
| `labels` | HashMap<String, String> | Optional. The labels to associate with this cluster. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a cluster. |
| `project_id` | String | Required. The Google Cloud Platform project ID that the cluster belongs to. |
| `cluster_uuid` | String | Output only. A cluster UUID (Unique Universal Identifier). Dataproc generates this value when it creates the cluster. |
| `config` | String | Required. The cluster config. Note that Dataproc may set default values, and values may change when clusters are updated. |
| `status` | String | Output only. Cluster status. |
| `cluster_name` | String | Required. The cluster name. Cluster names within a project must be unique. Names of deleted clusters can be reused. |
| `status_history` | Vec<String> | Output only. The previous cluster status. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cluster
cluster = provider.dataproc_api.Cluster {
    region = "value"  # Required. The Dataproc region in which to handle the request.
    project_id = "value"  # Required. The ID of the Google Cloud Platform project that the cluster belongs to.
}

# Access cluster outputs
cluster_id = cluster.id
cluster_metrics = cluster.metrics
cluster_labels = cluster.labels
cluster_project_id = cluster.project_id
cluster_cluster_uuid = cluster.cluster_uuid
cluster_config = cluster.config
cluster_status = cluster.status
cluster_cluster_name = cluster.cluster_name
cluster_status_history = cluster.status_history
```

---


### Autoscaling_policie

Creates new autoscaling policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `worker_config` | String |  | Required. Describes how the autoscaler will operate for primary workers. |
| `name` | String |  | Output only. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id} |
| `secondary_worker_config` | String |  | Optional. Describes how the autoscaler will operate for secondary workers. |
| `id` | String |  | Required. The policy id.The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters. |
| `basic_algorithm` | String |  |  |
| `parent` | String | ✅ | Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.create, the resource name has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.create, the resource name has the following format: projects/{project_id}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `worker_config` | String | Required. Describes how the autoscaler will operate for primary workers. |
| `name` | String | Output only. The "resource name" of the autoscaling policy, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/regions/{region}/autoscalingPolicies/{policy_id} For projects.locations.autoscalingPolicies, the resource name of the policy has the following format: projects/{project_id}/locations/{location}/autoscalingPolicies/{policy_id} |
| `secondary_worker_config` | String | Optional. Describes how the autoscaler will operate for secondary workers. |
| `id` | String | Required. The policy id.The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters. |
| `basic_algorithm` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create autoscaling_policie
autoscaling_policie = provider.dataproc_api.Autoscaling_policie {
    parent = "value"  # Required. The "resource name" of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.autoscalingPolicies.create, the resource name has the following format: projects/{project_id}/regions/{region} For projects.locations.autoscalingPolicies.create, the resource name has the following format: projects/{project_id}/locations/{location}
}

# Access autoscaling_policie outputs
autoscaling_policie_id = autoscaling_policie.id
autoscaling_policie_worker_config = autoscaling_policie.worker_config
autoscaling_policie_name = autoscaling_policie.name
autoscaling_policie_secondary_worker_config = autoscaling_policie.secondary_worker_config
autoscaling_policie_id = autoscaling_policie.id
autoscaling_policie_basic_algorithm = autoscaling_policie.basic_algorithm
```

---


### Workflow_template

Creates new workflow template.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time template was created. |
| `placement` | String |  | Required. WorkflowTemplate scheduling information. |
| `id` | String |  | Required. The template id.The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters.. |
| `update_time` | String |  | Output only. The time template was last updated. |
| `labels` | HashMap<String, String> |  | Optional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created by the workflow instance.Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).No more than 32 labels can be associated with a template. |
| `version` | i64 |  | Optional. Used to perform a consistent read-modify-write.This field should be left blank for a CreateWorkflowTemplate request. It is required for an UpdateWorkflowTemplate request, and must match the current server version. A typical update template flow would fetch the current template with a GetWorkflowTemplate request, which will return the current template with the version field filled in with the current server version. The user updates other fields in the template, then returns it as part of the UpdateWorkflowTemplate request. |
| `parameters` | Vec<String> |  | Optional. Template parameters whose values are substituted into the template. Values for parameters must be provided when the template is instantiated. |
| `jobs` | Vec<String> |  | Required. The Directed Acyclic Graph of Jobs to submit. |
| `name` | String |  | Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id} |
| `dag_timeout` | String |  | Optional. Timeout duration for the DAG of jobs, expressed in seconds (see JSON representation of duration (https://developers.google.com/protocol-buffers/docs/proto3#json)). The timeout duration must be from 10 minutes ("600s") to 24 hours ("86400s"). The timer begins when the first job is submitted. If the workflow is running at the end of the timeout period, any remaining jobs are cancelled, the workflow is ended, and if the workflow was running on a managed cluster, the cluster is deleted. |
| `parent` | String | ✅ | Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates,create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.create, the resource name of the location has the following format: projects/{project_id}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time template was created. |
| `placement` | String | Required. WorkflowTemplate scheduling information. |
| `id` | String | Required. The template id.The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_), and hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between 3 and 50 characters.. |
| `update_time` | String | Output only. The time template was last updated. |
| `labels` | HashMap<String, String> | Optional. The labels to associate with this template. These labels will be propagated to all jobs and clusters created by the workflow instance.Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt).No more than 32 labels can be associated with a template. |
| `version` | i64 | Optional. Used to perform a consistent read-modify-write.This field should be left blank for a CreateWorkflowTemplate request. It is required for an UpdateWorkflowTemplate request, and must match the current server version. A typical update template flow would fetch the current template with a GetWorkflowTemplate request, which will return the current template with the version field filled in with the current server version. The user updates other fields in the template, then returns it as part of the UpdateWorkflowTemplate request. |
| `parameters` | Vec<String> | Optional. Template parameters whose values are substituted into the template. Values for parameters must be provided when the template is instantiated. |
| `jobs` | Vec<String> | Required. The Directed Acyclic Graph of Jobs to submit. |
| `name` | String | Output only. The resource name of the workflow template, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/regions/{region}/workflowTemplates/{template_id} For projects.locations.workflowTemplates, the resource name of the template has the following format: projects/{project_id}/locations/{location}/workflowTemplates/{template_id} |
| `dag_timeout` | String | Optional. Timeout duration for the DAG of jobs, expressed in seconds (see JSON representation of duration (https://developers.google.com/protocol-buffers/docs/proto3#json)). The timeout duration must be from 10 minutes ("600s") to 24 hours ("86400s"). The timer begins when the first job is submitted. If the workflow is running at the end of the timeout period, any remaining jobs are cancelled, the workflow is ended, and if the workflow was running on a managed cluster, the cluster is deleted. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workflow_template
workflow_template = provider.dataproc_api.Workflow_template {
    parent = "value"  # Required. The resource name of the region or location, as described in https://cloud.google.com/apis/design/resource_names. For projects.regions.workflowTemplates,create, the resource name of the region has the following format: projects/{project_id}/regions/{region} For projects.locations.workflowTemplates.create, the resource name of the location has the following format: projects/{project_id}/locations/{location}
}

# Access workflow_template outputs
workflow_template_id = workflow_template.id
workflow_template_create_time = workflow_template.create_time
workflow_template_placement = workflow_template.placement
workflow_template_id = workflow_template.id
workflow_template_update_time = workflow_template.update_time
workflow_template_labels = workflow_template.labels
workflow_template_version = workflow_template.version
workflow_template_parameters = workflow_template.parameters
workflow_template_jobs = workflow_template.jobs
workflow_template_name = workflow_template.name
workflow_template_dag_timeout = workflow_template.dag_timeout
```

---


### Job

Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the resource. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pyspark_job` | String | Optional. Job is a PySpark job. |
| `yarn_applications` | Vec<String> | Output only. The collection of YARN applications spun up by this job.Beta Feature: This report is available for testing purposes only. It may be changed before final release. |
| `reference` | String | Optional. The fully qualified reference to the job, which can be used to obtain the equivalent REST path of the job resource. If this property is not specified when a job is created, the server generates a job_id. |
| `spark_sql_job` | String | Optional. Job is a SparkSql job. |
| `status_history` | Vec<String> | Output only. The previous job status. |
| `spark_r_job` | String | Optional. Job is a SparkR job. |
| `presto_job` | String | Optional. Job is a Presto job. |
| `scheduling` | String | Optional. Job scheduling configuration. |
| `spark_job` | String | Optional. Job is a Spark job. |
| `status` | String | Output only. The job status. Additional application-specific status information may be contained in the type_job and yarn_applications fields. |
| `driver_output_resource_uri` | String | Output only. A URI pointing to the location of the stdout of the job's driver program. |
| `hadoop_job` | String | Optional. Job is a Hadoop job. |
| `job_uuid` | String | Output only. A UUID that uniquely identifies a job within the project over time. This is in contrast to a user-settable reference.job_id that may be reused over time. |
| `labels` | HashMap<String, String> | Optional. The labels to associate with this job. Label keys must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). Label values may be empty, but, if present, must contain 1 to 63 characters, and must conform to RFC 1035 (https://www.ietf.org/rfc/rfc1035.txt). No more than 32 labels can be associated with a job. |
| `done` | bool | Output only. Indicates whether the job is completed. If the value is false, the job is still in progress. If true, the job is completed, and status.state field will indicate if it was successful, failed, or cancelled. |
| `hive_job` | String | Optional. Job is a Hive job. |
| `placement` | String | Required. Job information, including how, when, and where to run the job. |
| `driver_control_files_uri` | String | Output only. If present, the location of miscellaneous control files which may be used as part of job setup and handling. If not present, control files may be placed in the same location as driver_output_uri. |
| `submitted_by` | String | Output only. The email address of the user submitting the job. For jobs submitted on the cluster, the address is username@hostname. |
| `pig_job` | String | Optional. Job is a Pig job. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.dataproc_api.Job {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field.
}

# Access job outputs
job_id = job.id
job_pyspark_job = job.pyspark_job
job_yarn_applications = job.yarn_applications
job_reference = job.reference
job_spark_sql_job = job.spark_sql_job
job_status_history = job.status_history
job_spark_r_job = job.spark_r_job
job_presto_job = job.presto_job
job_scheduling = job.scheduling
job_spark_job = job.spark_job
job_status = job.status
job_driver_output_resource_uri = job.driver_output_resource_uri
job_hadoop_job = job.hadoop_job
job_job_uuid = job.job_uuid
job_labels = job.labels
job_done = job.done
job_hive_job = job.hive_job
job_placement = job.placement
job_driver_control_files_uri = job.driver_control_files_uri
job_submitted_by = job.submitted_by
job_pig_job = job.pig_job
```

---


### Operation

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the resource. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see IAM Overview (https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.dataproc_api.Operation {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See the operation documentation for the appropriate value for this field.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple session_template resources
session_template_0 = provider.dataproc_api.Session_template {
    parent = "value-0"
}
session_template_1 = provider.dataproc_api.Session_template {
    parent = "value-1"
}
session_template_2 = provider.dataproc_api.Session_template {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    session_template = provider.dataproc_api.Session_template {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Dataproc_api Documentation](https://cloud.google.com/dataproc_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
