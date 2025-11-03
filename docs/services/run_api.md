# Run_api Service



**Resources**: 23

---

## Overview

The run_api service provides access to 23 resource types:

- [Operation](#operation) [CRD]
- [Domainmapping](#domainmapping) [CRD]
- [Authorizeddomain](#authorizeddomain) [R]
- [Route](#route) [R]
- [Execution](#execution) [CRD]
- [Job](#job) [CRUD]
- [Service](#service) [CRUD]
- [Configuration](#configuration) [R]
- [Workerpool](#workerpool) [CRUD]
- [Task](#task) [R]
- [Revision](#revision) [RD]
- [Location](#location) [R]
- [Worker_pool](#worker_pool) [CRUD]
- [Revision](#revision) [RD]
- [Operation](#operation) [CRD]
- [Task](#task) [R]
- [Build](#build) [C]
- [Location](#location) [CR]
- [Job](#job) [CRUD]
- [Execution](#execution) [CRD]
- [Service](#service) [CRUD]
- [Customresourcedefinition](#customresourcedefinition) [R]
- [Job](#job) [CRD]

---

## Resources


### Operation

Waits until the specified long-running operation is done or reaches at most a specified timeout, returning the latest state. If the operation is already done, the latest state is immediately returned. If the timeout specified is greater than the default HTTP/RPC timeout, the HTTP/RPC timeout is used. If the server does not support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Note that this method is on a best-effort basis. It may return the latest state before the specified timeout (including immediately), meaning even an immediate response is no guarantee that the operation is done.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `timeout` | String |  | The maximum duration to wait before timing out. If left blank, the wait will be at most the time permitted by the underlying HTTP/RPC protocol. If RPC context deadline is also specified, the shorter one will be used. |
| `name` | String | ✅ | The name of the operation resource to wait on. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.run_api.Operation {
    name = "value"  # The name of the operation resource to wait on.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
operation_name = operation.name
```

---


### Domainmapping

Create a new domain mapping.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_version` | String |  | The API version for this call such as "domains.cloudrun.com/v1". |
| `status` | String |  | The current status of the DomainMapping. |
| `metadata` | String |  | Metadata associated with this BuildTemplate. |
| `spec` | String |  | The spec for this DomainMapping. |
| `kind` | String |  | The kind of resource, in this case "DomainMapping". |
| `parent` | String | ✅ | Required. The namespace in which the domain mapping should be created. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_version` | String | The API version for this call such as "domains.cloudrun.com/v1". |
| `status` | String | The current status of the DomainMapping. |
| `metadata` | String | Metadata associated with this BuildTemplate. |
| `spec` | String | The spec for this DomainMapping. |
| `kind` | String | The kind of resource, in this case "DomainMapping". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domainmapping
domainmapping = provider.run_api.Domainmapping {
    parent = "value"  # Required. The namespace in which the domain mapping should be created. For Cloud Run (fully managed), replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
}

# Access domainmapping outputs
domainmapping_id = domainmapping.id
domainmapping_api_version = domainmapping.api_version
domainmapping_status = domainmapping.status
domainmapping_metadata = domainmapping.metadata
domainmapping_spec = domainmapping.spec
domainmapping_kind = domainmapping.kind
```

---


### Authorizeddomain

List authorized domains.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Continuation token for fetching the next page of results. |
| `domains` | Vec<String> | The authorized domains belonging to the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access authorizeddomain outputs
authorizeddomain_id = authorizeddomain.id
authorizeddomain_next_page_token = authorizeddomain.next_page_token
authorizeddomain_domains = authorizeddomain.domains
```

---


### Route

Get information about a route.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_version` | String | The API version for this call such as "serving.knative.dev/v1". |
| `metadata` | String | Metadata associated with this Route, including name, namespace, labels, and annotations. |
| `spec` | String | Spec holds the desired state of the Route (from the client). |
| `kind` | String | The kind of this resource, in this case always "Route". |
| `status` | String | Status communicates the observed state of the Route (from the controller). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access route outputs
route_id = route.id
route_api_version = route.api_version
route_metadata = route.metadata
route_spec = route.spec
route_kind = route.kind
route_status = route.status
```

---


### Execution

Cancel an execution.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the execution to cancel. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_version` | String | Optional. APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. |
| `metadata` | String | Optional. Standard object's metadata. |
| `status` | String | Output only. Current status of an execution. |
| `kind` | String | Optional. Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. |
| `spec` | String | Optional. Specification of the desired behavior of an execution. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create execution
execution = provider.run_api.Execution {
    name = "value"  # Required. The name of the execution to cancel. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
}

# Access execution outputs
execution_id = execution.id
execution_api_version = execution.api_version
execution_metadata = execution.metadata
execution_status = execution.status
execution_kind = execution.kind
execution_spec = execution.spec
```

---


### Job

Create a job.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String |  | Output only. Current status of a job. |
| `metadata` | String |  | Optional. Standard object's metadata. |
| `kind` | String |  | Optional. Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. |
| `spec` | String |  | Optional. Specification of the desired behavior of a job. |
| `api_version` | String |  | Optional. APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. |
| `parent` | String | ✅ | Required. The namespace in which the job should be created. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | Output only. Current status of a job. |
| `metadata` | String | Optional. Standard object's metadata. |
| `kind` | String | Optional. Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. |
| `spec` | String | Optional. Specification of the desired behavior of a job. |
| `api_version` | String | Optional. APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.run_api.Job {
    parent = "value"  # Required. The namespace in which the job should be created. Replace {namespace} with the project ID or number. It takes the form namespaces/{namespace}. For example: namespaces/PROJECT_ID
}

# Access job outputs
job_id = job.id
job_status = job.status
job_metadata = job.metadata
job_kind = job.kind
job_spec = job.spec
job_api_version = job.api_version
```

---


### Service

Creates a new Service. Service creation will trigger a new deployment. Use GetService, and check service.status to determine if the Service is ready.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `spec` | String |  | Holds the desired state of the Service (from the client). |
| `metadata` | String |  | Metadata associated with this Service, including name, namespace, labels, and annotations. In Cloud Run, annotations with 'run.googleapis.com/' and 'autoscaling.knative.dev' are restricted, and the accepted annotations will be different depending on the resource type. The following Cloud Run-specific annotations are accepted in Service.metadata.annotations. * `run.googleapis.com/binary-authorization-breakglass` * `run.googleapis.com/binary-authorization` * `run.googleapis.com/client-name` * `run.googleapis.com/custom-audiences` * `run.googleapis.com/default-url-disabled` * `run.googleapis.com/description` * `run.googleapis.com/gc-traffic-tags` * `run.googleapis.com/ingress` * `run.googleapis.com/ingress` sets the ingress settings for the Service. See [the ingress settings documentation](/run/docs/securing/ingress) for details on configuring ingress settings. * `run.googleapis.com/ingress-status` is output-only and contains the currently active ingress settings for the Service. `run.googleapis.com/ingress-status` may differ from `run.googleapis.com/ingress` while the system is processing a change to `run.googleapis.com/ingress` or if the system failed to process a change to `run.googleapis.com/ingress`. When the system has processed all changes successfully `run.googleapis.com/ingress-status` and `run.googleapis.com/ingress` are equal. |
| `kind` | String |  | The kind of resource. It must be "Service". |
| `status` | String |  | Communicates the system-controlled state of the Service. |
| `api_version` | String |  | The API version for this call. It must be "serving.knative.dev/v1". |
| `parent` | String | ✅ | Required. The resource's parent. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/services` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spec` | String | Holds the desired state of the Service (from the client). |
| `metadata` | String | Metadata associated with this Service, including name, namespace, labels, and annotations. In Cloud Run, annotations with 'run.googleapis.com/' and 'autoscaling.knative.dev' are restricted, and the accepted annotations will be different depending on the resource type. The following Cloud Run-specific annotations are accepted in Service.metadata.annotations. * `run.googleapis.com/binary-authorization-breakglass` * `run.googleapis.com/binary-authorization` * `run.googleapis.com/client-name` * `run.googleapis.com/custom-audiences` * `run.googleapis.com/default-url-disabled` * `run.googleapis.com/description` * `run.googleapis.com/gc-traffic-tags` * `run.googleapis.com/ingress` * `run.googleapis.com/ingress` sets the ingress settings for the Service. See [the ingress settings documentation](/run/docs/securing/ingress) for details on configuring ingress settings. * `run.googleapis.com/ingress-status` is output-only and contains the currently active ingress settings for the Service. `run.googleapis.com/ingress-status` may differ from `run.googleapis.com/ingress` while the system is processing a change to `run.googleapis.com/ingress` or if the system failed to process a change to `run.googleapis.com/ingress`. When the system has processed all changes successfully `run.googleapis.com/ingress-status` and `run.googleapis.com/ingress` are equal. |
| `kind` | String | The kind of resource. It must be "Service". |
| `status` | String | Communicates the system-controlled state of the Service. |
| `api_version` | String | The API version for this call. It must be "serving.knative.dev/v1". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.run_api.Service {
    parent = "value"  # Required. The resource's parent. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/services` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}`
}

# Access service outputs
service_id = service.id
service_spec = service.spec
service_metadata = service.metadata
service_kind = service.kind
service_status = service.status
service_api_version = service.api_version
```

---


### Configuration

Get information about a configuration.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | Status communicates the observed state of the Configuration (from the controller). |
| `kind` | String | The kind of resource, in this case always "Configuration". |
| `api_version` | String | The API version for this call such as "serving.knative.dev/v1". |
| `metadata` | String | Metadata associated with this Configuration, including name, namespace, labels, and annotations. |
| `spec` | String | Spec holds the desired state of the Configuration (from the client). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access configuration outputs
configuration_id = configuration.id
configuration_status = configuration.status
configuration_kind = configuration.kind
configuration_api_version = configuration.api_version
configuration_metadata = configuration.metadata
configuration_spec = configuration.spec
```

---


### Workerpool

Creates a new WorkerPool. WorkerPool creation will trigger a new deployment. Use GetWorkerPool, and check worker_pool.status to determine if the WorkerPool is ready.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `metadata` | String |  | Metadata associated with this WorkerPool, including name, namespace, labels, and annotations. In Cloud Run, annotations with 'run.googleapis.com/' and 'autoscaling.knative.dev' are restricted, and the accepted annotations will be different depending on the resource type. The following Cloud Run-specific annotations are accepted in WorkerPool.metadata.annotations. * `run.googleapis.com/binary-authorization-breakglass` * `run.googleapis.com/binary-authorization` * `run.googleapis.com/client-name` * `run.googleapis.com/description` |
| `api_version` | String |  | The API version for this call. It must be "run.googleapis.com/v1". |
| `kind` | String |  | The kind of resource. It must be "WorkerPool". |
| `spec` | String |  | Holds the desired state of the WorkerPool (from the client). |
| `status` | String |  | Communicates the system-controlled state of the WorkerPool. |
| `parent` | String | ✅ | Required. The resource's parent. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/workerpools` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | Metadata associated with this WorkerPool, including name, namespace, labels, and annotations. In Cloud Run, annotations with 'run.googleapis.com/' and 'autoscaling.knative.dev' are restricted, and the accepted annotations will be different depending on the resource type. The following Cloud Run-specific annotations are accepted in WorkerPool.metadata.annotations. * `run.googleapis.com/binary-authorization-breakglass` * `run.googleapis.com/binary-authorization` * `run.googleapis.com/client-name` * `run.googleapis.com/description` |
| `api_version` | String | The API version for this call. It must be "run.googleapis.com/v1". |
| `kind` | String | The kind of resource. It must be "WorkerPool". |
| `spec` | String | Holds the desired state of the WorkerPool (from the client). |
| `status` | String | Communicates the system-controlled state of the WorkerPool. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workerpool
workerpool = provider.run_api.Workerpool {
    parent = "value"  # Required. The resource's parent. In Cloud Run, it may be one of the following: * `{project_id_or_number}` * `namespaces/{project_id_or_number}` * `namespaces/{project_id_or_number}/workerpools` * `projects/{project_id_or_number}/locations/{region}` * `projects/{project_id_or_number}/regions/{region}`
}

# Access workerpool outputs
workerpool_id = workerpool.id
workerpool_metadata = workerpool.metadata
workerpool_api_version = workerpool.api_version
workerpool_kind = workerpool.kind
workerpool_spec = workerpool.spec
workerpool_status = workerpool.status
```

---


### Task

Get information about a task.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spec` | String | Optional. Specification of the desired behavior of a task. |
| `api_version` | String | Optional. APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. |
| `status` | String | Output only. Current status of a task. |
| `metadata` | String | Optional. Standard object's metadata. |
| `kind` | String | Optional. Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access task outputs
task_id = task.id
task_spec = task.spec
task_api_version = task.api_version
task_status = task.status
task_metadata = task.metadata
task_kind = task.kind
```

---


### Revision

Get information about a revision.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | Metadata associated with this Revision, including name, namespace, labels, and annotations. |
| `spec` | String | Spec holds the desired state of the Revision (from the client). |
| `status` | String | Status communicates the observed state of the Revision (from the controller). |
| `api_version` | String | The API version for this call such as "serving.knative.dev/v1". |
| `kind` | String | The kind of this resource, in this case "Revision". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access revision outputs
revision_id = revision.id
revision_metadata = revision.metadata
revision_spec = revision.spec
revision_status = revision.status
revision_api_version = revision.api_version
revision_kind = revision.kind
```

---


### Location

Lists information about the supported locations for this service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The standard List next-page token. |
| `locations` | Vec<String> | A list of locations that matches the specified filter in the request. |


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
location_next_page_token = location.next_page_token
location_locations = location.locations
```

---


### Worker_pool

Creates a new WorkerPool in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client` | String |  | Arbitrary identifier for the API client. |
| `observed_generation` | String |  | Output only. The generation of this WorkerPool currently serving workloads. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a `string` instead of an `integer`. |
| `reconciling` | bool |  | Output only. Returns true if the WorkerPool is currently being acted upon by the system to bring it into the desired state. When a new WorkerPool is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the WorkerPool to the desired serving state. This process is called reconciliation. While reconciliation is in process, `observed_generation`, `latest_ready_revison`, `instance_split_statuses`, and `uri` will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the WorkerPool, or there was an error, and reconciliation failed. This state can be found in `terminal_condition.state`. If reconciliation succeeded, the following fields will match: `instance_splits` and `instance_split_statuses`, `observed_generation` and `generation`, `latest_ready_revision` and `latest_created_revision`. If reconciliation failed, `instance_split_statuses`, `observed_generation`, and `latest_ready_revision` will have the state of the last serving revision, or empty for newly created WorkerPools. Additional information on the failure can be found in `terminal_condition` and `conditions`. |
| `uid` | String |  | Output only. Server assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `instance_split_statuses` | Vec<String> |  | Output only. Detailed status information for corresponding instance splits. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `launch_stage` | String |  | Optional. The launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. If no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. |
| `delete_time` | String |  | Output only. The deletion time. It is only populated as a response to a Delete request. |
| `template` | String |  | Required. The template used to create revisions for this WorkerPool. |
| `instance_splits` | Vec<String> |  | Optional. Specifies how to distribute instances over a collection of Revisions belonging to the WorkerPool. If instance split is empty or not provided, defaults to 100% instances assigned to the latest `Ready` Revision. |
| `binary_authorization` | String |  | Optional. Settings for the Binary Authorization feature. |
| `custom_audiences` | Vec<String> |  | One or more custom audiences that you want this worker pool to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests. For more information, see https://cloud.google.com/run/docs/configuring/custom-audiences. |
| `etag` | String |  | Optional. A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. Cloud Run API v2 does not support annotations with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected in new resources. All system annotations in v1 now have a corresponding field in v2 WorkerPool. This field follows Kubernetes annotations' namespacing, limits, and rules. |
| `description` | String |  | User-provided description of the WorkerPool. This field currently has a 512-character limit. |
| `latest_created_revision` | String |  | Output only. Name of the last created revision. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `latest_ready_revision` | String |  | Output only. Name of the latest revision that is serving workloads. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `generation` | String |  | Output only. A number that monotonically increases every time the user modifies the desired state. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a `string` instead of an `integer`. |
| `create_time` | String |  | Output only. The creation time. |
| `scaling` | String |  | Optional. Specifies worker-pool-level scaling settings |
| `terminal_condition` | String |  | Output only. The Condition of this WorkerPool, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `creator` | String |  | Output only. Email address of the authenticated creator. |
| `conditions` | Vec<String> |  | Output only. The Conditions of all other associated sub-resources. They contain additional diagnostics information in case the WorkerPool does not reach its Serving state. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `expire_time` | String |  | Output only. For a deleted resource, the time after which it will be permamently deleted. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `update_time` | String |  | Output only. The last-modified time. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected. All system labels in v1 now have a corresponding field in v2 WorkerPool. |
| `client_version` | String |  | Arbitrary version identifier for the API client. |
| `name` | String |  | The fully qualified name of this WorkerPool. In CreateWorkerPoolRequest, this field is ignored, and instead composed from CreateWorkerPoolRequest.parent and CreateWorkerPoolRequest.worker_id. Format: `projects/{project}/locations/{location}/workerPools/{worker_id}` |
| `last_modifier` | String |  | Output only. Email address of the last authenticated modifier. |
| `parent` | String | ✅ | Required. The location and project in which this worker pool should be created. Format: `projects/{project}/locations/{location}`, where `{project}` can be project id or number. Only lowercase characters, digits, and hyphens. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client` | String | Arbitrary identifier for the API client. |
| `observed_generation` | String | Output only. The generation of this WorkerPool currently serving workloads. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a `string` instead of an `integer`. |
| `reconciling` | bool | Output only. Returns true if the WorkerPool is currently being acted upon by the system to bring it into the desired state. When a new WorkerPool is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the WorkerPool to the desired serving state. This process is called reconciliation. While reconciliation is in process, `observed_generation`, `latest_ready_revison`, `instance_split_statuses`, and `uri` will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the WorkerPool, or there was an error, and reconciliation failed. This state can be found in `terminal_condition.state`. If reconciliation succeeded, the following fields will match: `instance_splits` and `instance_split_statuses`, `observed_generation` and `generation`, `latest_ready_revision` and `latest_created_revision`. If reconciliation failed, `instance_split_statuses`, `observed_generation`, and `latest_ready_revision` will have the state of the last serving revision, or empty for newly created WorkerPools. Additional information on the failure can be found in `terminal_condition` and `conditions`. |
| `uid` | String | Output only. Server assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `instance_split_statuses` | Vec<String> | Output only. Detailed status information for corresponding instance splits. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `launch_stage` | String | Optional. The launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. If no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. |
| `delete_time` | String | Output only. The deletion time. It is only populated as a response to a Delete request. |
| `template` | String | Required. The template used to create revisions for this WorkerPool. |
| `instance_splits` | Vec<String> | Optional. Specifies how to distribute instances over a collection of Revisions belonging to the WorkerPool. If instance split is empty or not provided, defaults to 100% instances assigned to the latest `Ready` Revision. |
| `binary_authorization` | String | Optional. Settings for the Binary Authorization feature. |
| `custom_audiences` | Vec<String> | One or more custom audiences that you want this worker pool to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests. For more information, see https://cloud.google.com/run/docs/configuring/custom-audiences. |
| `etag` | String | Optional. A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. Cloud Run API v2 does not support annotations with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected in new resources. All system annotations in v1 now have a corresponding field in v2 WorkerPool. This field follows Kubernetes annotations' namespacing, limits, and rules. |
| `description` | String | User-provided description of the WorkerPool. This field currently has a 512-character limit. |
| `latest_created_revision` | String | Output only. Name of the last created revision. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `latest_ready_revision` | String | Output only. Name of the latest revision that is serving workloads. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `generation` | String | Output only. A number that monotonically increases every time the user modifies the desired state. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a `string` instead of an `integer`. |
| `create_time` | String | Output only. The creation time. |
| `scaling` | String | Optional. Specifies worker-pool-level scaling settings |
| `terminal_condition` | String | Output only. The Condition of this WorkerPool, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `creator` | String | Output only. Email address of the authenticated creator. |
| `conditions` | Vec<String> | Output only. The Conditions of all other associated sub-resources. They contain additional diagnostics information in case the WorkerPool does not reach its Serving state. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `expire_time` | String | Output only. For a deleted resource, the time after which it will be permamently deleted. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. The last-modified time. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected. All system labels in v1 now have a corresponding field in v2 WorkerPool. |
| `client_version` | String | Arbitrary version identifier for the API client. |
| `name` | String | The fully qualified name of this WorkerPool. In CreateWorkerPoolRequest, this field is ignored, and instead composed from CreateWorkerPoolRequest.parent and CreateWorkerPoolRequest.worker_id. Format: `projects/{project}/locations/{location}/workerPools/{worker_id}` |
| `last_modifier` | String | Output only. Email address of the last authenticated modifier. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create worker_pool
worker_pool = provider.run_api.Worker_pool {
    parent = "value"  # Required. The location and project in which this worker pool should be created. Format: `projects/{project}/locations/{location}`, where `{project}` can be project id or number. Only lowercase characters, digits, and hyphens.
}

# Access worker_pool outputs
worker_pool_id = worker_pool.id
worker_pool_client = worker_pool.client
worker_pool_observed_generation = worker_pool.observed_generation
worker_pool_reconciling = worker_pool.reconciling
worker_pool_uid = worker_pool.uid
worker_pool_instance_split_statuses = worker_pool.instance_split_statuses
worker_pool_launch_stage = worker_pool.launch_stage
worker_pool_delete_time = worker_pool.delete_time
worker_pool_template = worker_pool.template
worker_pool_instance_splits = worker_pool.instance_splits
worker_pool_binary_authorization = worker_pool.binary_authorization
worker_pool_custom_audiences = worker_pool.custom_audiences
worker_pool_etag = worker_pool.etag
worker_pool_annotations = worker_pool.annotations
worker_pool_description = worker_pool.description
worker_pool_latest_created_revision = worker_pool.latest_created_revision
worker_pool_latest_ready_revision = worker_pool.latest_ready_revision
worker_pool_generation = worker_pool.generation
worker_pool_create_time = worker_pool.create_time
worker_pool_scaling = worker_pool.scaling
worker_pool_terminal_condition = worker_pool.terminal_condition
worker_pool_creator = worker_pool.creator
worker_pool_conditions = worker_pool.conditions
worker_pool_expire_time = worker_pool.expire_time
worker_pool_satisfies_pzs = worker_pool.satisfies_pzs
worker_pool_update_time = worker_pool.update_time
worker_pool_labels = worker_pool.labels
worker_pool_client_version = worker_pool.client_version
worker_pool_name = worker_pool.name
worker_pool_last_modifier = worker_pool.last_modifier
```

---


### Revision

Gets information about a Revision.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. Server assigned unique identifier for the Revision. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `annotations` | HashMap<String, String> | Output only. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `name` | String | Output only. The unique name of this Revision. |
| `update_time` | String | Output only. The last-modified time. |
| `encryption_key` | String | A reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek |
| `containers` | Vec<String> | Holds the single container that defines the unit of execution for this Revision. |
| `encryption_key_shutdown_duration` | String | If encryption_key_revocation_action is SHUTDOWN, the duration before shutting down all instances. The minimum increment is 1 hour. |
| `observed_generation` | String | Output only. The generation of this Revision currently serving traffic. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `service` | String | Output only. The name of the parent service. |
| `reconciling` | bool | Output only. Indicates whether the resource's reconciliation is still in progress. See comments in `Service.reconciling` for additional information on reconciliation process in Cloud Run. |
| `gpu_zonal_redundancy_disabled` | bool | Optional. Output only. True if GPU zonal redundancy is disabled on this revision. |
| `execution_environment` | String | The execution environment being used to host this Revision. |
| `max_instance_request_concurrency` | i64 | Sets the maximum number of requests that each serving instance can receive. |
| `scaling` | String | Scaling settings for this revision. |
| `launch_stage` | String | The least stable launch stage needed to create this resource, as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. Note that this value might not be what was used as input. For example, if ALPHA was provided as input in the parent resource, but only BETA and GA-level features are were, this field will be BETA. |
| `encryption_key_revocation_action` | String | The action to take if the encryption key is revoked. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `vpc_access` | String | VPC Access configuration for this Revision. For more information, visit https://cloud.google.com/run/docs/configuring/connecting-vpc. |
| `session_affinity` | bool | Enable session affinity. |
| `delete_time` | String | Output only. For a deleted resource, the deletion time. It is only populated as a response to a Delete request. |
| `labels` | HashMap<String, String> | Output only. Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels. |
| `service_account` | String | Email address of the IAM service account associated with the revision of the service. The service account represents the identity of the running revision, and determines what permissions the revision has. |
| `log_uri` | String | Output only. The Google Console URI to obtain logs for the Revision. |
| `generation` | String | Output only. A number that monotonically increases every time the user modifies the desired state. |
| `volumes` | Vec<String> | A list of Volumes to make available to containers. |
| `conditions` | Vec<String> | Output only. The Condition of this Revision, containing its readiness status, and detailed error information in case it did not reach a serving state. |
| `create_time` | String | Output only. The creation time. |
| `creator` | String | Output only. Email address of the authenticated creator. |
| `scaling_status` | String | Output only. The current effective scaling settings for the revision. |
| `expire_time` | String | Output only. For a deleted resource, the time after which it will be permamently deleted. It is only populated as a response to a Delete request. |
| `etag` | String | Output only. A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates. |
| `node_selector` | String | The node selector for the revision. |
| `service_mesh` | String | Enables service mesh connectivity. |
| `timeout` | String | Max allowed time for an instance to respond to a request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access revision outputs
revision_id = revision.id
revision_uid = revision.uid
revision_annotations = revision.annotations
revision_name = revision.name
revision_update_time = revision.update_time
revision_encryption_key = revision.encryption_key
revision_containers = revision.containers
revision_encryption_key_shutdown_duration = revision.encryption_key_shutdown_duration
revision_observed_generation = revision.observed_generation
revision_service = revision.service
revision_reconciling = revision.reconciling
revision_gpu_zonal_redundancy_disabled = revision.gpu_zonal_redundancy_disabled
revision_execution_environment = revision.execution_environment
revision_max_instance_request_concurrency = revision.max_instance_request_concurrency
revision_scaling = revision.scaling
revision_launch_stage = revision.launch_stage
revision_encryption_key_revocation_action = revision.encryption_key_revocation_action
revision_satisfies_pzs = revision.satisfies_pzs
revision_vpc_access = revision.vpc_access
revision_session_affinity = revision.session_affinity
revision_delete_time = revision.delete_time
revision_labels = revision.labels
revision_service_account = revision.service_account
revision_log_uri = revision.log_uri
revision_generation = revision.generation
revision_volumes = revision.volumes
revision_conditions = revision.conditions
revision_create_time = revision.create_time
revision_creator = revision.creator
revision_scaling_status = revision.scaling_status
revision_expire_time = revision.expire_time
revision_etag = revision.etag
revision_node_selector = revision.node_selector
revision_service_mesh = revision.service_mesh
revision_timeout = revision.timeout
```

---


### Operation

Waits until the specified long-running operation is done or reaches at most a specified timeout, returning the latest state. If the operation is already done, the latest state is immediately returned. If the timeout specified is greater than the default HTTP/RPC timeout, the HTTP/RPC timeout is used. If the server does not support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Note that this method is on a best-effort basis. It may return the latest state before the specified timeout (including immediately), meaning even an immediate response is no guarantee that the operation is done.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `timeout` | String |  | The maximum duration to wait before timing out. If left blank, the wait will be at most the time permitted by the underlying HTTP/RPC protocol. If RPC context deadline is also specified, the shorter one will be used. |
| `name` | String | ✅ | The name of the operation resource to wait on. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.run_api.Operation {
    name = "value"  # The name of the operation resource to wait on.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
```

---


### Task

Gets information about a Task.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `timeout` | String | Max allowed time duration the Task may be active before the system will actively try to mark it failed and kill associated containers. This applies per attempt of a task, meaning each retry can run for the full timeout. |
| `gpu_zonal_redundancy_disabled` | bool | Optional. Output only. True if GPU zonal redundancy is disabled on this task. |
| `completion_time` | String | Output only. Represents time when the Task was completed. It is not guaranteed to be set in happens-before order across separate operations. |
| `execution_environment` | String | The execution environment being used to host this Task. |
| `max_retries` | i64 | Number of retries allowed per Task, before marking this Task failed. |
| `uid` | String | Output only. Server assigned unique identifier for the Task. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `node_selector` | String | Output only. The node selector for the task. |
| `index` | i64 | Output only. Index of the Task, unique per execution, and beginning at 0. |
| `labels` | HashMap<String, String> | Output only. Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels |
| `name` | String | Output only. The unique name of this Task. |
| `volumes` | Vec<String> | A list of Volumes to make available to containers. |
| `annotations` | HashMap<String, String> | Output only. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `job` | String | Output only. The name of the parent Job. |
| `etag` | String | Output only. A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates. |
| `reconciling` | bool | Output only. Indicates whether the resource's reconciliation is still in progress. See comments in `Job.reconciling` for additional information on reconciliation process in Cloud Run. |
| `scheduled_time` | String | Output only. Represents time when the task was scheduled to run by the system. It is not guaranteed to be set in happens-before order across separate operations. |
| `create_time` | String | Output only. Represents time when the task was created by the system. It is not guaranteed to be set in happens-before order across separate operations. |
| `log_uri` | String | Output only. URI where logs for this execution can be found in Cloud Console. |
| `generation` | String | Output only. A number that monotonically increases every time the user modifies the desired state. |
| `delete_time` | String | Output only. For a deleted resource, the deletion time. It is only populated as a response to a Delete request. |
| `service_account` | String | Email address of the IAM service account associated with the Task of a Job. The service account represents the identity of the running task, and determines what permissions the task has. If not provided, the task will use the project's default service account. |
| `vpc_access` | String | Output only. VPC Access configuration to use for this Task. For more information, visit https://cloud.google.com/run/docs/configuring/connecting-vpc. |
| `last_attempt_result` | String | Output only. Result of the last attempt of this Task. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `update_time` | String | Output only. The last-modified time. |
| `conditions` | Vec<String> | Output only. The Condition of this Task, containing its readiness status, and detailed error information in case it did not reach the desired state. |
| `start_time` | String | Output only. Represents time when the task started to run. It is not guaranteed to be set in happens-before order across separate operations. |
| `containers` | Vec<String> | Holds the single container that defines the unit of execution for this task. |
| `retried` | i64 | Output only. The number of times this Task was retried. Tasks are retried when they fail up to the maxRetries limit. |
| `execution` | String | Output only. The name of the parent Execution. |
| `encryption_key` | String | Output only. A reference to a customer managed encryption key (CMEK) to use to encrypt this container image. For more information, go to https://cloud.google.com/run/docs/securing/using-cmek |
| `expire_time` | String | Output only. For a deleted resource, the time after which it will be permamently deleted. It is only populated as a response to a Delete request. |
| `observed_generation` | String | Output only. The generation of this Task. See comments in `Job.reconciling` for additional information on reconciliation process in Cloud Run. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access task outputs
task_id = task.id
task_timeout = task.timeout
task_gpu_zonal_redundancy_disabled = task.gpu_zonal_redundancy_disabled
task_completion_time = task.completion_time
task_execution_environment = task.execution_environment
task_max_retries = task.max_retries
task_uid = task.uid
task_node_selector = task.node_selector
task_index = task.index
task_labels = task.labels
task_name = task.name
task_volumes = task.volumes
task_annotations = task.annotations
task_job = task.job
task_etag = task.etag
task_reconciling = task.reconciling
task_scheduled_time = task.scheduled_time
task_create_time = task.create_time
task_log_uri = task.log_uri
task_generation = task.generation
task_delete_time = task.delete_time
task_service_account = task.service_account
task_vpc_access = task.vpc_access
task_last_attempt_result = task.last_attempt_result
task_satisfies_pzs = task.satisfies_pzs
task_update_time = task.update_time
task_conditions = task.conditions
task_start_time = task.start_time
task_containers = task.containers
task_retried = task.retried
task_execution = task.execution
task_encryption_key = task.encryption_key
task_expire_time = task.expire_time
task_observed_generation = task.observed_generation
```

---


### Build

Submits a build in a given project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client` | String |  | Optional. The client that initiated the build request. |
| `machine_type` | String |  | Optional. The machine type from default pool to use for the build. If left blank, cloudbuild will use a sensible default. Currently only E2_HIGHCPU_8 is supported. If worker_pool is set, this field will be ignored. |
| `storage_source` | String |  | Required. Source for the build. |
| `tags` | Vec<String> |  | Optional. Additional tags to annotate the build. |
| `release_track` | String |  | Optional. The release track of the client that initiated the build request. |
| `image_uri` | String |  | Required. Artifact Registry URI to store the built image. |
| `buildpack_build` | String |  | Build the source using Buildpacks. |
| `service_account` | String |  | Optional. The service account to use for the build. If not set, the default Cloud Build service account for the project will be used. |
| `worker_pool` | String |  | Optional. Name of the Cloud Build Custom Worker Pool that should be used to build the function. The format of this field is `projects/{project}/locations/{region}/workerPools/{workerPool}` where `{project}` and `{region}` are the project id and region respectively where the worker pool is defined and `{workerPool}` is the short name of the worker pool. |
| `docker_build` | String |  | Build the source using Docker. This means the source has a Dockerfile. |
| `parent` | String | ✅ | Required. The project and location to build in. Location must be a region, e.g., 'us-central1' or 'global' if the global builder is to be used. Format: `projects/{project}/locations/{location}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create build
build = provider.run_api.Build {
    parent = "value"  # Required. The project and location to build in. Location must be a region, e.g., 'us-central1' or 'global' if the global builder is to be used. Format: `projects/{project}/locations/{location}`
}

```

---


### Location

Export image for a given resource.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_repo` | String |  | Required. The export destination url (the Artifact Registry repo). |
| `name` | String | ✅ | Required. The name of the resource of which image metadata should be exported. Format: `projects/{project_id_or_number}/locations/{location}/services/{service}/revisions/{revision}` for Revision `projects/{project_id_or_number}/locations/{location}/jobs/{job}/executions/{execution}` for Execution |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | JSON encoded Google-generated Customer Metadata for a given resource/project. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.run_api.Location {
    name = "value"  # Required. The name of the resource of which image metadata should be exported. Format: `projects/{project_id_or_number}/locations/{location}/services/{service}/revisions/{revision}` for Revision `projects/{project_id_or_number}/locations/{location}/jobs/{job}/executions/{execution}` for Execution
}

# Access location outputs
location_id = location.id
location_metadata = location.metadata
```

---


### Job

Creates a Job.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_version` | String |  | Arbitrary version identifier for the API client. |
| `last_modifier` | String |  | Output only. Email address of the last authenticated modifier. |
| `run_execution_token` | String |  | A unique string used as a suffix for creating a new execution. The Job will become ready when the execution is successfully completed. The sum of job name and token length must be fewer than 63 characters. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> |  | Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected. All system labels in v1 now have a corresponding field in v2 Job. |
| `reconciling` | bool |  | Output only. Returns true if the Job is currently being acted upon by the system to bring it into the desired state. When a new Job is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Job to the desired state. This process is called reconciliation. While reconciliation is in process, `observed_generation` and `latest_succeeded_execution`, will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the state matches the Job, or there was an error, and reconciliation failed. This state can be found in `terminal_condition.state`. If reconciliation succeeded, the following fields will match: `observed_generation` and `generation`, `latest_succeeded_execution` and `latest_created_execution`. If reconciliation failed, `observed_generation` and `latest_succeeded_execution` will have the state of the last succeeded execution or empty for newly created Job. Additional information on the failure can be found in `terminal_condition` and `conditions`. |
| `execution_count` | i64 |  | Output only. Number of executions created for this job. |
| `start_execution_token` | String |  | A unique string used as a suffix creating a new execution. The Job will become ready when the execution is successfully started. The sum of job name and token length must be fewer than 63 characters. |
| `binary_authorization` | String |  | Settings for the Binary Authorization feature. |
| `etag` | String |  | Optional. A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates. |
| `update_time` | String |  | Output only. The last-modified time. |
| `template` | String |  | Required. The template used to create executions for this Job. |
| `client` | String |  | Arbitrary identifier for the API client. |
| `delete_time` | String |  | Output only. The deletion time. It is only populated as a response to a Delete request. |
| `terminal_condition` | String |  | Output only. The Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state. |
| `uid` | String |  | Output only. Server assigned unique identifier for the Execution. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `latest_created_execution` | String |  | Output only. Name of the last created execution. |
| `name` | String |  | The fully qualified name of this Job. Format: projects/{project}/locations/{location}/jobs/{job} |
| `annotations` | HashMap<String, String> |  | Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. Cloud Run API v2 does not support annotations with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected on new resources. All system annotations in v1 now have a corresponding field in v2 Job. This field follows Kubernetes annotations' namespacing, limits, and rules. |
| `conditions` | Vec<String> |  | Output only. The Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Job does not reach its desired state. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `generation` | String |  | Output only. A number that monotonically increases every time the user modifies the desired state. |
| `creator` | String |  | Output only. Email address of the authenticated creator. |
| `expire_time` | String |  | Output only. For a deleted resource, the time after which it will be permamently deleted. |
| `launch_stage` | String |  | The launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. If no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. |
| `create_time` | String |  | Output only. The creation time. |
| `observed_generation` | String |  | Output only. The generation of this Job. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `parent` | String | ✅ | Required. The location and project in which this Job should be created. Format: projects/{project}/locations/{location}, where {project} can be project id or number. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client_version` | String | Arbitrary version identifier for the API client. |
| `last_modifier` | String | Output only. Email address of the last authenticated modifier. |
| `run_execution_token` | String | A unique string used as a suffix for creating a new execution. The Job will become ready when the execution is successfully completed. The sum of job name and token length must be fewer than 63 characters. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected. All system labels in v1 now have a corresponding field in v2 Job. |
| `reconciling` | bool | Output only. Returns true if the Job is currently being acted upon by the system to bring it into the desired state. When a new Job is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Job to the desired state. This process is called reconciliation. While reconciliation is in process, `observed_generation` and `latest_succeeded_execution`, will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the state matches the Job, or there was an error, and reconciliation failed. This state can be found in `terminal_condition.state`. If reconciliation succeeded, the following fields will match: `observed_generation` and `generation`, `latest_succeeded_execution` and `latest_created_execution`. If reconciliation failed, `observed_generation` and `latest_succeeded_execution` will have the state of the last succeeded execution or empty for newly created Job. Additional information on the failure can be found in `terminal_condition` and `conditions`. |
| `execution_count` | i64 | Output only. Number of executions created for this job. |
| `start_execution_token` | String | A unique string used as a suffix creating a new execution. The Job will become ready when the execution is successfully started. The sum of job name and token length must be fewer than 63 characters. |
| `binary_authorization` | String | Settings for the Binary Authorization feature. |
| `etag` | String | Optional. A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates. |
| `update_time` | String | Output only. The last-modified time. |
| `template` | String | Required. The template used to create executions for this Job. |
| `client` | String | Arbitrary identifier for the API client. |
| `delete_time` | String | Output only. The deletion time. It is only populated as a response to a Delete request. |
| `terminal_condition` | String | Output only. The Condition of this Job, containing its readiness status, and detailed error information in case it did not reach the desired state. |
| `uid` | String | Output only. Server assigned unique identifier for the Execution. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `latest_created_execution` | String | Output only. Name of the last created execution. |
| `name` | String | The fully qualified name of this Job. Format: projects/{project}/locations/{location}/jobs/{job} |
| `annotations` | HashMap<String, String> | Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. Cloud Run API v2 does not support annotations with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected on new resources. All system annotations in v1 now have a corresponding field in v2 Job. This field follows Kubernetes annotations' namespacing, limits, and rules. |
| `conditions` | Vec<String> | Output only. The Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Job does not reach its desired state. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `generation` | String | Output only. A number that monotonically increases every time the user modifies the desired state. |
| `creator` | String | Output only. Email address of the authenticated creator. |
| `expire_time` | String | Output only. For a deleted resource, the time after which it will be permamently deleted. |
| `launch_stage` | String | The launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. If no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. |
| `create_time` | String | Output only. The creation time. |
| `observed_generation` | String | Output only. The generation of this Job. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.run_api.Job {
    parent = "value"  # Required. The location and project in which this Job should be created. Format: projects/{project}/locations/{location}, where {project} can be project id or number.
}

# Access job outputs
job_id = job.id
job_client_version = job.client_version
job_last_modifier = job.last_modifier
job_run_execution_token = job.run_execution_token
job_satisfies_pzs = job.satisfies_pzs
job_labels = job.labels
job_reconciling = job.reconciling
job_execution_count = job.execution_count
job_start_execution_token = job.start_execution_token
job_binary_authorization = job.binary_authorization
job_etag = job.etag
job_update_time = job.update_time
job_template = job.template
job_client = job.client
job_delete_time = job.delete_time
job_terminal_condition = job.terminal_condition
job_uid = job.uid
job_latest_created_execution = job.latest_created_execution
job_name = job.name
job_annotations = job.annotations
job_conditions = job.conditions
job_generation = job.generation
job_creator = job.creator
job_expire_time = job.expire_time
job_launch_stage = job.launch_stage
job_create_time = job.create_time
job_observed_generation = job.observed_generation
```

---


### Execution

Cancels an Execution.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `validate_only` | bool |  | Indicates that the request should be validated without actually cancelling any resources. |
| `etag` | String |  | A system-generated fingerprint for this version of the resource. This may be used to detect modification conflict during updates. |
| `name` | String | ✅ | Required. The name of the Execution to cancel. Format: `projects/{project}/locations/{location}/jobs/{job}/executions/{execution}`, where `{project}` can be project id or number. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cancelled_count` | i64 | Output only. The number of tasks which reached phase Cancelled. |
| `retried_count` | i64 | Output only. The number of tasks which have retried at least once. |
| `job` | String | Output only. The name of the parent Job. |
| `creator` | String | Output only. Email address of the authenticated creator. |
| `expire_time` | String | Output only. For a deleted resource, the time after which it will be permamently deleted. It is only populated as a response to a Delete request. |
| `generation` | String | Output only. A number that monotonically increases every time the user modifies the desired state. |
| `reconciling` | bool | Output only. Indicates whether the resource's reconciliation is still in progress. See comments in `Job.reconciling` for additional information on reconciliation process in Cloud Run. |
| `running_count` | i64 | Output only. The number of actively running tasks. |
| `observed_generation` | String | Output only. The generation of this Execution. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `labels` | HashMap<String, String> | Output only. Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels |
| `etag` | String | Output only. A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates. |
| `completion_time` | String | Output only. Represents time when the execution was completed. It is not guaranteed to be set in happens-before order across separate operations. |
| `annotations` | HashMap<String, String> | Output only. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. |
| `create_time` | String | Output only. Represents time when the execution was acknowledged by the execution controller. It is not guaranteed to be set in happens-before order across separate operations. |
| `log_uri` | String | Output only. URI where logs for this execution can be found in Cloud Console. |
| `name` | String | Output only. The unique name of this Execution. |
| `start_time` | String | Output only. Represents time when the execution started to run. It is not guaranteed to be set in happens-before order across separate operations. |
| `succeeded_count` | i64 | Output only. The number of tasks which reached phase Succeeded. |
| `delete_time` | String | Output only. For a deleted resource, the deletion time. It is only populated as a response to a Delete request. |
| `task_count` | i64 | Output only. Specifies the desired number of tasks the execution should run. Setting to 1 means that parallelism is limited to 1 and the success of that task signals the success of the execution. |
| `failed_count` | i64 | Output only. The number of tasks which reached phase Failed. |
| `parallelism` | i64 | Output only. Specifies the maximum desired number of tasks the execution should run at any given time. Must be <= task_count. The actual number of tasks running in steady state will be less than this number when ((.spec.task_count - .status.successful) < .spec.parallelism), i.e. when the work left to do is less than max parallelism. |
| `template` | String | Output only. The template used to create tasks for this execution. |
| `conditions` | Vec<String> | Output only. The Condition of this Execution, containing its readiness status, and detailed error information in case it did not reach the desired state. |
| `update_time` | String | Output only. The last-modified time. |
| `uid` | String | Output only. Server assigned unique identifier for the Execution. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `launch_stage` | String | The least stable launch stage needed to create this resource, as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. Note that this value might not be what was used as input. For example, if ALPHA was provided as input in the parent resource, but only BETA and GA-level features are were, this field will be BETA. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create execution
execution = provider.run_api.Execution {
    name = "value"  # Required. The name of the Execution to cancel. Format: `projects/{project}/locations/{location}/jobs/{job}/executions/{execution}`, where `{project}` can be project id or number.
}

# Access execution outputs
execution_id = execution.id
execution_cancelled_count = execution.cancelled_count
execution_retried_count = execution.retried_count
execution_job = execution.job
execution_creator = execution.creator
execution_expire_time = execution.expire_time
execution_generation = execution.generation
execution_reconciling = execution.reconciling
execution_running_count = execution.running_count
execution_observed_generation = execution.observed_generation
execution_satisfies_pzs = execution.satisfies_pzs
execution_labels = execution.labels
execution_etag = execution.etag
execution_completion_time = execution.completion_time
execution_annotations = execution.annotations
execution_create_time = execution.create_time
execution_log_uri = execution.log_uri
execution_name = execution.name
execution_start_time = execution.start_time
execution_succeeded_count = execution.succeeded_count
execution_delete_time = execution.delete_time
execution_task_count = execution.task_count
execution_failed_count = execution.failed_count
execution_parallelism = execution.parallelism
execution_template = execution.template
execution_conditions = execution.conditions
execution_update_time = execution.update_time
execution_uid = execution.uid
execution_launch_stage = execution.launch_stage
```

---


### Service

Creates a new Service in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The creation time. |
| `annotations` | HashMap<String, String> |  | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. Cloud Run API v2 does not support annotations with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected in new resources. All system annotations in v1 now have a corresponding field in v2 Service. This field follows Kubernetes annotations' namespacing, limits, and rules. |
| `custom_audiences` | Vec<String> |  | One or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests. For more information, see https://cloud.google.com/run/docs/configuring/custom-audiences. |
| `binary_authorization` | String |  | Optional. Settings for the Binary Authorization feature. |
| `expire_time` | String |  | Output only. For a deleted resource, the time after which it will be permanently deleted. |
| `traffic_statuses` | Vec<String> |  | Output only. Detailed status information for corresponding traffic targets. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `uid` | String |  | Output only. Server assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `name` | String |  | Identifier. The fully qualified name of this Service. In CreateServiceRequest, this field is ignored, and instead composed from CreateServiceRequest.parent and CreateServiceRequest.service_id. Format: projects/{project}/locations/{location}/services/{service_id} |
| `urls` | Vec<String> |  | Output only. All URLs serving traffic for this Service. |
| `build_config` | String |  | Optional. Configuration for building a Cloud Run function. |
| `update_time` | String |  | Output only. The last-modified time. |
| `description` | String |  | User-provided description of the Service. This field currently has a 512-character limit. |
| `conditions` | Vec<String> |  | Output only. The Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Service does not reach its Serving state. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `labels` | HashMap<String, String> |  | Optional. Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected. All system labels in v1 now have a corresponding field in v2 Service. |
| `launch_stage` | String |  | Optional. The launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. If no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. |
| `generation` | String |  | Output only. A number that monotonically increases every time the user modifies the desired state. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a `string` instead of an `integer`. |
| `creator` | String |  | Output only. Email address of the authenticated creator. |
| `scaling` | String |  | Optional. Specifies service-level scaling settings |
| `invoker_iam_disabled` | bool |  | Optional. Disables IAM permission check for run.routes.invoke for callers of this service. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check. |
| `client_version` | String |  | Arbitrary version identifier for the API client. |
| `ingress` | String |  | Optional. Provides the ingress settings for this Service. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. |
| `delete_time` | String |  | Output only. The deletion time. It is only populated as a response to a Delete request. |
| `iap_enabled` | bool |  | Optional. IAP settings on the Service. |
| `template` | String |  | Required. The template used to create revisions for this Service. |
| `uri` | String |  | Output only. The main URI in which this Service is serving traffic. |
| `client` | String |  | Arbitrary identifier for the API client. |
| `latest_created_revision` | String |  | Output only. Name of the last created revision. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `etag` | String |  | Optional. A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates. |
| `default_uri_disabled` | bool |  | Optional. Disables public resolution of the default URI of this service. |
| `last_modifier` | String |  | Output only. Email address of the last authenticated modifier. |
| `multi_region_settings` | String |  | Optional. Settings for multi-region deployment. |
| `reconciling` | bool |  | Output only. Returns true if the Service is currently being acted upon by the system to bring it into the desired state. When a new Service is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Service to the desired serving state. This process is called reconciliation. While reconciliation is in process, `observed_generation`, `latest_ready_revision`, `traffic_statuses`, and `uri` will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the Service, or there was an error, and reconciliation failed. This state can be found in `terminal_condition.state`. If reconciliation succeeded, the following fields will match: `traffic` and `traffic_statuses`, `observed_generation` and `generation`, `latest_ready_revision` and `latest_created_revision`. If reconciliation failed, `traffic_statuses`, `observed_generation`, and `latest_ready_revision` will have the state of the last serving revision, or empty for newly created Services. Additional information on the failure can be found in `terminal_condition` and `conditions`. |
| `terminal_condition` | String |  | Output only. The Condition of this Service, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `threat_detection_enabled` | bool |  | Output only. True if Cloud Run Threat Detection monitoring is enabled for the parent project of this Service. |
| `traffic` | Vec<String> |  | Optional. Specifies how to distribute traffic over a collection of Revisions belonging to the Service. If traffic is empty or not provided, defaults to 100% traffic to the latest `Ready` Revision. |
| `observed_generation` | String |  | Output only. The generation of this Service currently serving traffic. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a `string` instead of an `integer`. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `latest_ready_revision` | String |  | Output only. Name of the latest revision that is serving traffic. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `parent` | String | ✅ | Required. The location and project in which this service should be created. Format: projects/{project}/locations/{location}, where {project} can be project id or number. Only lowercase characters, digits, and hyphens. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The creation time. |
| `annotations` | HashMap<String, String> | Optional. Unstructured key value map that may be set by external tools to store and arbitrary metadata. They are not queryable and should be preserved when modifying objects. Cloud Run API v2 does not support annotations with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected in new resources. All system annotations in v1 now have a corresponding field in v2 Service. This field follows Kubernetes annotations' namespacing, limits, and rules. |
| `custom_audiences` | Vec<String> | One or more custom audiences that you want this service to support. Specify each custom audience as the full URL in a string. The custom audiences are encoded in the token and used to authenticate requests. For more information, see https://cloud.google.com/run/docs/configuring/custom-audiences. |
| `binary_authorization` | String | Optional. Settings for the Binary Authorization feature. |
| `expire_time` | String | Output only. For a deleted resource, the time after which it will be permanently deleted. |
| `traffic_statuses` | Vec<String> | Output only. Detailed status information for corresponding traffic targets. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `uid` | String | Output only. Server assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `name` | String | Identifier. The fully qualified name of this Service. In CreateServiceRequest, this field is ignored, and instead composed from CreateServiceRequest.parent and CreateServiceRequest.service_id. Format: projects/{project}/locations/{location}/services/{service_id} |
| `urls` | Vec<String> | Output only. All URLs serving traffic for this Service. |
| `build_config` | String | Optional. Configuration for building a Cloud Run function. |
| `update_time` | String | Output only. The last-modified time. |
| `description` | String | User-provided description of the Service. This field currently has a 512-character limit. |
| `conditions` | Vec<String> | Output only. The Conditions of all other associated sub-resources. They contain additional diagnostics information in case the Service does not reach its Serving state. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `labels` | HashMap<String, String> | Optional. Unstructured key value map that can be used to organize and categorize objects. User-provided labels are shared with Google's billing system, so they can be used to filter, or break down billing charges by team, component, environment, state, etc. For more information, visit https://cloud.google.com/resource-manager/docs/creating-managing-labels or https://cloud.google.com/run/docs/configuring/labels. Cloud Run API v2 does not support labels with `run.googleapis.com`, `cloud.googleapis.com`, `serving.knative.dev`, or `autoscaling.knative.dev` namespaces, and they will be rejected. All system labels in v1 now have a corresponding field in v2 Service. |
| `launch_stage` | String | Optional. The launch stage as defined by [Google Cloud Platform Launch Stages](https://cloud.google.com/terms/launch-stages). Cloud Run supports `ALPHA`, `BETA`, and `GA`. If no value is specified, GA is assumed. Set the launch stage to a preview stage on input to allow use of preview features in that stage. On read (or output), describes whether the resource uses preview features. For example, if ALPHA is provided as input, but only BETA and GA-level features are used, this field will be BETA on output. |
| `generation` | String | Output only. A number that monotonically increases every time the user modifies the desired state. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a `string` instead of an `integer`. |
| `creator` | String | Output only. Email address of the authenticated creator. |
| `scaling` | String | Optional. Specifies service-level scaling settings |
| `invoker_iam_disabled` | bool | Optional. Disables IAM permission check for run.routes.invoke for callers of this service. For more information, visit https://cloud.google.com/run/docs/securing/managing-access#invoker_check. |
| `client_version` | String | Arbitrary version identifier for the API client. |
| `ingress` | String | Optional. Provides the ingress settings for this Service. On output, returns the currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no revision is active. |
| `delete_time` | String | Output only. The deletion time. It is only populated as a response to a Delete request. |
| `iap_enabled` | bool | Optional. IAP settings on the Service. |
| `template` | String | Required. The template used to create revisions for this Service. |
| `uri` | String | Output only. The main URI in which this Service is serving traffic. |
| `client` | String | Arbitrary identifier for the API client. |
| `latest_created_revision` | String | Output only. Name of the last created revision. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `etag` | String | Optional. A system-generated fingerprint for this version of the resource. May be used to detect modification conflict during updates. |
| `default_uri_disabled` | bool | Optional. Disables public resolution of the default URI of this service. |
| `last_modifier` | String | Output only. Email address of the last authenticated modifier. |
| `multi_region_settings` | String | Optional. Settings for multi-region deployment. |
| `reconciling` | bool | Output only. Returns true if the Service is currently being acted upon by the system to bring it into the desired state. When a new Service is created, or an existing one is updated, Cloud Run will asynchronously perform all necessary steps to bring the Service to the desired serving state. This process is called reconciliation. While reconciliation is in process, `observed_generation`, `latest_ready_revision`, `traffic_statuses`, and `uri` will have transient values that might mismatch the intended state: Once reconciliation is over (and this field is false), there are two possible outcomes: reconciliation succeeded and the serving state matches the Service, or there was an error, and reconciliation failed. This state can be found in `terminal_condition.state`. If reconciliation succeeded, the following fields will match: `traffic` and `traffic_statuses`, `observed_generation` and `generation`, `latest_ready_revision` and `latest_created_revision`. If reconciliation failed, `traffic_statuses`, `observed_generation`, and `latest_ready_revision` will have the state of the last serving revision, or empty for newly created Services. Additional information on the failure can be found in `terminal_condition` and `conditions`. |
| `terminal_condition` | String | Output only. The Condition of this Service, containing its readiness status, and detailed error information in case it did not reach a serving state. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |
| `threat_detection_enabled` | bool | Output only. True if Cloud Run Threat Detection monitoring is enabled for the parent project of this Service. |
| `traffic` | Vec<String> | Optional. Specifies how to distribute traffic over a collection of Revisions belonging to the Service. If traffic is empty or not provided, defaults to 100% traffic to the latest `Ready` Revision. |
| `observed_generation` | String | Output only. The generation of this Service currently serving traffic. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. Please note that unlike v1, this is an int64 value. As with most Google APIs, its JSON representation will be a `string` instead of an `integer`. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `latest_ready_revision` | String | Output only. Name of the latest revision that is serving traffic. See comments in `reconciling` for additional information on reconciliation process in Cloud Run. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.run_api.Service {
    parent = "value"  # Required. The location and project in which this service should be created. Format: projects/{project}/locations/{location}, where {project} can be project id or number. Only lowercase characters, digits, and hyphens.
}

# Access service outputs
service_id = service.id
service_create_time = service.create_time
service_annotations = service.annotations
service_custom_audiences = service.custom_audiences
service_binary_authorization = service.binary_authorization
service_expire_time = service.expire_time
service_traffic_statuses = service.traffic_statuses
service_uid = service.uid
service_name = service.name
service_urls = service.urls
service_build_config = service.build_config
service_update_time = service.update_time
service_description = service.description
service_conditions = service.conditions
service_labels = service.labels
service_launch_stage = service.launch_stage
service_generation = service.generation
service_creator = service.creator
service_scaling = service.scaling
service_invoker_iam_disabled = service.invoker_iam_disabled
service_client_version = service.client_version
service_ingress = service.ingress
service_delete_time = service.delete_time
service_iap_enabled = service.iap_enabled
service_template = service.template
service_uri = service.uri
service_client = service.client
service_latest_created_revision = service.latest_created_revision
service_etag = service.etag
service_default_uri_disabled = service.default_uri_disabled
service_last_modifier = service.last_modifier
service_multi_region_settings = service.multi_region_settings
service_reconciling = service.reconciling
service_terminal_condition = service.terminal_condition
service_threat_detection_enabled = service.threat_detection_enabled
service_traffic = service.traffic
service_observed_generation = service.observed_generation
service_satisfies_pzs = service.satisfies_pzs
service_latest_ready_revision = service.latest_ready_revision
```

---


### Customresourcedefinition

Rpc to get information about a CustomResourceDefinition.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_version` | String | The API version for this call such as "k8s.apiextensions.io/v1beta1". |
| `metadata` | String | Metadata associated with this CustomResourceDefinition. |
| `kind` | String | The kind of resource, in this case always "CustomResourceDefinition". |
| `spec` | String | Spec describes how the user wants the resources to appear |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access customresourcedefinition outputs
customresourcedefinition_id = customresourcedefinition.id
customresourcedefinition_api_version = customresourcedefinition.api_version
customresourcedefinition_metadata = customresourcedefinition.metadata
customresourcedefinition_kind = customresourcedefinition.kind
customresourcedefinition_spec = customresourcedefinition.spec
```

---


### Job

Create a job.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Optional. Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds +optional |
| `metadata` | String |  | Optional. Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata +optional |
| `spec` | String |  | Optional. Specification of the desired behavior of a job. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status +optional |
| `status` | String |  | Optional. Current status of a job. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status +optional |
| `api_version` | String |  | Optional. APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources +optional |
| `parent` | String | ✅ | Required. The namespace in which the job should be created. Replace {namespace_id} with the project ID or number. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Optional. Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds +optional |
| `metadata` | String | Optional. Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata +optional |
| `spec` | String | Optional. Specification of the desired behavior of a job. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status +optional |
| `status` | String | Optional. Current status of a job. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status +optional |
| `api_version` | String | Optional. APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources +optional |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create job
job = provider.run_api.Job {
    parent = "value"  # Required. The namespace in which the job should be created. Replace {namespace_id} with the project ID or number.
}

# Access job outputs
job_id = job.id
job_kind = job.kind
job_metadata = job.metadata
job_spec = job.spec
job_status = job.status
job_api_version = job.api_version
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
operation_0 = provider.run_api.Operation {
    name = "value-0"
}
operation_1 = provider.run_api.Operation {
    name = "value-1"
}
operation_2 = provider.run_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.run_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Run_api Documentation](https://cloud.google.com/run_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
