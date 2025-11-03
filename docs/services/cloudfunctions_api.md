# Cloudfunctions_api Service



**Resources**: 18

---

## Overview

The cloudfunctions_api service provides access to 18 resource types:

- [Function](#function) [CRUD]
- [Operation](#operation) [R]
- [Location](#location) [R]
- [Location](#location) [R]
- [Function](#function) [CRUD]
- [Operation](#operation) [R]
- [Operation](#operation) [R]
- [Function](#function) [CRUD]
- [Location](#location) [R]
- [Runtime](#runtime) [R]
- [Function](#function) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [R]
- [Runtime](#runtime) [R]
- [Location](#location) [R]
- [Runtime](#runtime) [R]
- [Operation](#operation) [R]
- [Function](#function) [CRUD]

---

## Resources


### Function

Creates a new function. If a function with the given name already exists in the specified project, the long running operation will return `ALREADY_EXISTS` error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `environment_variables` | HashMap<String, String> |  | Environment variables that shall be available during function execution. |
| `vpc_connector_egress_settings` | String |  | The egress settings for the connector, controlling what traffic is diverted through it. |
| `build_name` | String |  | Output only. The Cloud Build Name of the function deployment. `projects//locations//builds/`. |
| `satisfies_pzs` | bool |  | Output only. |
| `source_token` | String |  | Input only. An identifier for Firebase function sources. Disclaimer: This field is only supported for Firebase function deployments. |
| `docker_registry` | String |  | Docker Registry to use for this deployment. Deprecated: as of March 2025, `CONTAINER_REGISTRY` option is no longer available in response to Container Registry's deprecation: https://cloud.google.com/artifact-registry/docs/transition/transition-from-gcr Please use Artifact Registry instead, which is the default choice. If unspecified, it defaults to `ARTIFACT_REGISTRY`. If `docker_repository` field is specified, this field should either be left unspecified or set to `ARTIFACT_REGISTRY`. |
| `status` | String |  | Output only. Status of the function deployment. |
| `description` | String |  | User-provided description of a function. |
| `secret_volumes` | Vec<String> |  | Secret volumes configuration. |
| `entry_point` | String |  | The name of the function (as defined in source code) that will be executed. Defaults to the resource name suffix (ID of the function), if not specified. |
| `labels` | HashMap<String, String> |  | Labels associated with this Cloud Function. |
| `satisfies_pzi` | bool |  | Output only. |
| `source_repository` | String |  | **Beta Feature** The source repository where a function is hosted. |
| `https_trigger` | String |  | An HTTPS endpoint type of source that can be triggered via URL. |
| `on_deploy_update_policy` | String |  |  |
| `min_instances` | i64 |  | A lower bound for the number function instances that may coexist at a given time. |
| `name` | String |  | A user-defined name of the function. Function names must be unique globally and match pattern `projects/*/locations/*/functions/*` |
| `network` | String |  | Deprecated: use vpc_connector |
| `timeout` | String |  | The function execution timeout. Execution is considered failed and can be terminated if the function is not completed at the end of the timeout period. Defaults to 60 seconds. |
| `event_trigger` | String |  | A source that fires events in response to a condition in another service. |
| `build_environment_variables` | HashMap<String, String> |  | Build environment variables that shall be available during build time. |
| `docker_repository` | String |  | User-managed repository created in Artifact Registry to which the function's Docker image will be pushed after it is built by Cloud Build. May optionally be encrypted with a customer-managed encryption key (CMEK). If unspecified and `docker_registry` is not explicitly set to `CONTAINER_REGISTRY`, GCF will create and use a default Artifact Registry repository named 'gcf-artifacts' in the region. It must match the pattern `projects/{project}/locations/{location}/repositories/{repository}`. Cross-project repositories are not supported. Cross-location repositories are not supported. Repository format must be 'DOCKER'. |
| `ingress_settings` | String |  | The ingress settings for the function, controlling what traffic can reach it. |
| `build_worker_pool` | String |  | Name of the Cloud Build Custom Worker Pool that should be used to build the function. The format of this field is `projects/{project}/locations/{region}/workerPools/{workerPool}` where `{project}` and `{region}` are the project id and region respectively where the worker pool is defined and `{workerPool}` is the short name of the worker pool. If the project id is not the same as the function, then the Cloud Functions Service Agent (`service-@gcf-admin-robot.iam.gserviceaccount.com`) must be granted the role Cloud Build Custom Workers Builder (`roles/cloudbuild.customworkers.builder`) in the project. |
| `kms_key_name` | String |  | Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources. It must match the pattern `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. If specified, you must also provide an artifact registry repository using the `docker_repository` field that was created with the same KMS crypto key. The following service accounts need to be granted the role 'Cloud KMS CryptoKey Encrypter/Decrypter (roles/cloudkms.cryptoKeyEncrypterDecrypter)' on the Key/KeyRing/Project/Organization (least access preferred). 1. Google Cloud Functions service account (service-{project_number}@gcf-admin-robot.iam.gserviceaccount.com) - Required to protect the function's image. 2. Google Storage service account (service-{project_number}@gs-project-accounts.iam.gserviceaccount.com) - Required to protect the function's source code. If this service account does not exist, deploying a function without a KMS key or retrieving the service agent name provisions it. For more information, see https://cloud.google.com/storage/docs/projects#service-agents and https://cloud.google.com/storage/docs/getting-service-agent#gsutil. Google Cloud Functions delegates access to service agents to protect function resources in internal projects that are not accessible by the end user. |
| `service_account_email` | String |  | The email of the function's service account. If empty, defaults to `{project_id}@appspot.gserviceaccount.com`. |
| `runtime` | String |  | The runtime in which to run the function. Required when deploying a new function, optional when updating an existing function. For a complete list of possible choices, see the [`gcloud` command reference](https://cloud.google.com/sdk/gcloud/reference/functions/deploy#--runtime). |
| `source_archive_url` | String |  | The Google Cloud Storage URL, starting with `gs://`, pointing to the zip archive which contains the function. |
| `source_upload_url` | String |  | The Google Cloud Storage signed URL used for source uploading, generated by calling [google.cloud.functions.v1.GenerateUploadUrl]. The signature is validated on write methods (Create, Update) The signature is stripped from the Function object on read methods (Get, List) |
| `update_time` | String |  | Output only. The last update timestamp of a Cloud Function. |
| `version_id` | String |  | Output only. The version identifier of the Cloud Function. Each deployment attempt results in a new version of a function being created. |
| `automatic_update_policy` | String |  |  |
| `build_service_account` | String |  | A service account the user provides for use with Cloud Build. The format of this field is `projects/{projectId}/serviceAccounts/{serviceAccountEmail}`. |
| `vpc_connector` | String |  | The VPC Network Connector that this cloud function can connect to. It can be either the fully-qualified URI, or the short name of the network connector resource. The format of this field is `projects/*/locations/*/connectors/*` This field is mutually exclusive with `network` field and will eventually replace it. See [the VPC documentation](https://cloud.google.com/compute/docs/vpc) for more information on connecting Cloud projects. |
| `secret_environment_variables` | Vec<String> |  | Secret environment variables configuration. |
| `available_memory_mb` | i64 |  | The amount of memory in MB available for a function. Defaults to 256MB. |
| `max_instances` | i64 |  | The limit on the maximum number of function instances that may coexist at a given time. In some cases, such as rapid traffic surges, Cloud Functions may, for a short period of time, create more instances than the specified max instances limit. If your function cannot tolerate this temporary behavior, you may want to factor in a safety margin and set a lower max instances value than your function can tolerate. See the [Max Instances](https://cloud.google.com/functions/docs/max-instances) Guide for more details. |
| `build_id` | String |  | Output only. The Cloud Build ID of the latest successful deployment of the function. |
| `location` | String | ✅ | Required. The project and location in which the function should be created, specified in the format `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `environment_variables` | HashMap<String, String> | Environment variables that shall be available during function execution. |
| `vpc_connector_egress_settings` | String | The egress settings for the connector, controlling what traffic is diverted through it. |
| `build_name` | String | Output only. The Cloud Build Name of the function deployment. `projects//locations//builds/`. |
| `satisfies_pzs` | bool | Output only. |
| `source_token` | String | Input only. An identifier for Firebase function sources. Disclaimer: This field is only supported for Firebase function deployments. |
| `docker_registry` | String | Docker Registry to use for this deployment. Deprecated: as of March 2025, `CONTAINER_REGISTRY` option is no longer available in response to Container Registry's deprecation: https://cloud.google.com/artifact-registry/docs/transition/transition-from-gcr Please use Artifact Registry instead, which is the default choice. If unspecified, it defaults to `ARTIFACT_REGISTRY`. If `docker_repository` field is specified, this field should either be left unspecified or set to `ARTIFACT_REGISTRY`. |
| `status` | String | Output only. Status of the function deployment. |
| `description` | String | User-provided description of a function. |
| `secret_volumes` | Vec<String> | Secret volumes configuration. |
| `entry_point` | String | The name of the function (as defined in source code) that will be executed. Defaults to the resource name suffix (ID of the function), if not specified. |
| `labels` | HashMap<String, String> | Labels associated with this Cloud Function. |
| `satisfies_pzi` | bool | Output only. |
| `source_repository` | String | **Beta Feature** The source repository where a function is hosted. |
| `https_trigger` | String | An HTTPS endpoint type of source that can be triggered via URL. |
| `on_deploy_update_policy` | String |  |
| `min_instances` | i64 | A lower bound for the number function instances that may coexist at a given time. |
| `name` | String | A user-defined name of the function. Function names must be unique globally and match pattern `projects/*/locations/*/functions/*` |
| `network` | String | Deprecated: use vpc_connector |
| `timeout` | String | The function execution timeout. Execution is considered failed and can be terminated if the function is not completed at the end of the timeout period. Defaults to 60 seconds. |
| `event_trigger` | String | A source that fires events in response to a condition in another service. |
| `build_environment_variables` | HashMap<String, String> | Build environment variables that shall be available during build time. |
| `docker_repository` | String | User-managed repository created in Artifact Registry to which the function's Docker image will be pushed after it is built by Cloud Build. May optionally be encrypted with a customer-managed encryption key (CMEK). If unspecified and `docker_registry` is not explicitly set to `CONTAINER_REGISTRY`, GCF will create and use a default Artifact Registry repository named 'gcf-artifacts' in the region. It must match the pattern `projects/{project}/locations/{location}/repositories/{repository}`. Cross-project repositories are not supported. Cross-location repositories are not supported. Repository format must be 'DOCKER'. |
| `ingress_settings` | String | The ingress settings for the function, controlling what traffic can reach it. |
| `build_worker_pool` | String | Name of the Cloud Build Custom Worker Pool that should be used to build the function. The format of this field is `projects/{project}/locations/{region}/workerPools/{workerPool}` where `{project}` and `{region}` are the project id and region respectively where the worker pool is defined and `{workerPool}` is the short name of the worker pool. If the project id is not the same as the function, then the Cloud Functions Service Agent (`service-@gcf-admin-robot.iam.gserviceaccount.com`) must be granted the role Cloud Build Custom Workers Builder (`roles/cloudbuild.customworkers.builder`) in the project. |
| `kms_key_name` | String | Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources. It must match the pattern `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. If specified, you must also provide an artifact registry repository using the `docker_repository` field that was created with the same KMS crypto key. The following service accounts need to be granted the role 'Cloud KMS CryptoKey Encrypter/Decrypter (roles/cloudkms.cryptoKeyEncrypterDecrypter)' on the Key/KeyRing/Project/Organization (least access preferred). 1. Google Cloud Functions service account (service-{project_number}@gcf-admin-robot.iam.gserviceaccount.com) - Required to protect the function's image. 2. Google Storage service account (service-{project_number}@gs-project-accounts.iam.gserviceaccount.com) - Required to protect the function's source code. If this service account does not exist, deploying a function without a KMS key or retrieving the service agent name provisions it. For more information, see https://cloud.google.com/storage/docs/projects#service-agents and https://cloud.google.com/storage/docs/getting-service-agent#gsutil. Google Cloud Functions delegates access to service agents to protect function resources in internal projects that are not accessible by the end user. |
| `service_account_email` | String | The email of the function's service account. If empty, defaults to `{project_id}@appspot.gserviceaccount.com`. |
| `runtime` | String | The runtime in which to run the function. Required when deploying a new function, optional when updating an existing function. For a complete list of possible choices, see the [`gcloud` command reference](https://cloud.google.com/sdk/gcloud/reference/functions/deploy#--runtime). |
| `source_archive_url` | String | The Google Cloud Storage URL, starting with `gs://`, pointing to the zip archive which contains the function. |
| `source_upload_url` | String | The Google Cloud Storage signed URL used for source uploading, generated by calling [google.cloud.functions.v1.GenerateUploadUrl]. The signature is validated on write methods (Create, Update) The signature is stripped from the Function object on read methods (Get, List) |
| `update_time` | String | Output only. The last update timestamp of a Cloud Function. |
| `version_id` | String | Output only. The version identifier of the Cloud Function. Each deployment attempt results in a new version of a function being created. |
| `automatic_update_policy` | String |  |
| `build_service_account` | String | A service account the user provides for use with Cloud Build. The format of this field is `projects/{projectId}/serviceAccounts/{serviceAccountEmail}`. |
| `vpc_connector` | String | The VPC Network Connector that this cloud function can connect to. It can be either the fully-qualified URI, or the short name of the network connector resource. The format of this field is `projects/*/locations/*/connectors/*` This field is mutually exclusive with `network` field and will eventually replace it. See [the VPC documentation](https://cloud.google.com/compute/docs/vpc) for more information on connecting Cloud projects. |
| `secret_environment_variables` | Vec<String> | Secret environment variables configuration. |
| `available_memory_mb` | i64 | The amount of memory in MB available for a function. Defaults to 256MB. |
| `max_instances` | i64 | The limit on the maximum number of function instances that may coexist at a given time. In some cases, such as rapid traffic surges, Cloud Functions may, for a short period of time, create more instances than the specified max instances limit. If your function cannot tolerate this temporary behavior, you may want to factor in a safety margin and set a lower max instances value than your function can tolerate. See the [Max Instances](https://cloud.google.com/functions/docs/max-instances) Guide for more details. |
| `build_id` | String | Output only. The Cloud Build ID of the latest successful deployment of the function. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create function
function = provider.cloudfunctions_api.Function {
    location = "value"  # Required. The project and location in which the function should be created, specified in the format `projects/*/locations/*`
}

# Access function outputs
function_id = function.id
function_environment_variables = function.environment_variables
function_vpc_connector_egress_settings = function.vpc_connector_egress_settings
function_build_name = function.build_name
function_satisfies_pzs = function.satisfies_pzs
function_source_token = function.source_token
function_docker_registry = function.docker_registry
function_status = function.status
function_description = function.description
function_secret_volumes = function.secret_volumes
function_entry_point = function.entry_point
function_labels = function.labels
function_satisfies_pzi = function.satisfies_pzi
function_source_repository = function.source_repository
function_https_trigger = function.https_trigger
function_on_deploy_update_policy = function.on_deploy_update_policy
function_min_instances = function.min_instances
function_name = function.name
function_network = function.network
function_timeout = function.timeout
function_event_trigger = function.event_trigger
function_build_environment_variables = function.build_environment_variables
function_docker_repository = function.docker_repository
function_ingress_settings = function.ingress_settings
function_build_worker_pool = function.build_worker_pool
function_kms_key_name = function.kms_key_name
function_service_account_email = function.service_account_email
function_runtime = function.runtime
function_source_archive_url = function.source_archive_url
function_source_upload_url = function.source_upload_url
function_update_time = function.update_time
function_version_id = function.version_id
function_automatic_update_policy = function.automatic_update_policy
function_build_service_account = function.build_service_account
function_vpc_connector = function.vpc_connector
function_secret_environment_variables = function.secret_environment_variables
function_available_memory_mb = function.available_memory_mb
function_max_instances = function.max_instances
function_build_id = function.build_id
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
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
| `locations` | Vec<String> | A list of locations that matches the specified filter in the request. |
| `next_page_token` | String | The standard List next-page token. |


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
location_locations = location.locations
location_next_page_token = location.next_page_token
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
| `locations` | Vec<String> | A list of locations that matches the specified filter in the request. |
| `next_page_token` | String | The standard List next-page token. |


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
location_locations = location.locations
location_next_page_token = location.next_page_token
```

---


### Function

Creates a new function. If a function with the given name already exists in
the specified project, the long running operation will return
`ALREADY_EXISTS` error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `https_trigger` | String |  | An HTTPS endpoint type of source that can be triggered via URL. |
| `entry_point` | String |  | The name of the function (as defined in source code) that will be
executed. Defaults to the resource name suffix, if not specified. For
backward compatibility, if function with given name is not found, then the
system will try to use function named "function".
For Node.js this is name of a function exported by the module specified
in `source_location`. |
| `labels` | HashMap<String, String> |  | Labels associated with this Cloud Function. |
| `service_account` | String |  | The email of the function's service account. If empty, defaults to
`{project_id}@appspot.gserviceaccount.com`. |
| `available_memory_mb` | i64 |  | The amount of memory in MB available for a function.
Defaults to 256MB. |
| `version_id` | String |  | Output only. The version identifier of the Cloud Function. Each deployment attempt
results in a new version of a function being created. |
| `environment_variables` | HashMap<String, String> |  | Environment variables that shall be available during function execution. |
| `runtime` | String |  | The runtime in which to run the function. Required when deploying a new
function, optional when updating an existing function. For a complete
list of possible choices, see the
[`gcloud` command
reference](/sdk/gcloud/reference/functions/deploy#--runtime). |
| `source_repository_url` | String |  | The URL pointing to the hosted repository where the function is defined.
There are supported Cloud Source Repository URLs in the following
formats:

To refer to a specific commit:
`https://source.developers.google.com/projects/*/repos/*/revisions/*/paths/*`
To refer to a moveable alias (branch):
`https://source.developers.google.com/projects/*/repos/*/moveable-aliases/*/paths/*`
In particular, to refer to HEAD use `master` moveable alias.
To refer to a specific fixed alias (tag):
`https://source.developers.google.com/projects/*/repos/*/fixed-aliases/*/paths/*`

You may omit `paths/*` if you want to use the main directory. |
| `status` | String |  | Output only. Status of the function deployment. |
| `timeout` | String |  | The function execution timeout. Execution is considered failed and
can be terminated if the function is not completed at the end of the
timeout period. Defaults to 60 seconds. |
| `name` | String |  | A user-defined name of the function. Function names must be unique
globally and match pattern `projects/*/locations/*/functions/*` |
| `network` | String |  | The VPC Network that this cloud function can connect to. It can be
either the fully-qualified URI, or the short name of the network resource.
If the short network name is used, the network must belong to the same
project. Otherwise, it must belong to a project within the same
organization. The format of this field is either
`projects/{project}/global/networks/{network}` or `{network}`, where
{project} is a project id where the network is defined, and {network} is
the short name of the network.

This field is mutually exclusive with `vpc_connector` and will be replaced
by it.

See [the VPC documentation](https://cloud.google.com/compute/docs/vpc) for
more information on connecting Cloud projects. |
| `source_upload_url` | String |  | The Google Cloud Storage signed URL used for source uploading, generated
by google.cloud.functions.v1beta2.GenerateUploadUrl |
| `update_time` | String |  | Output only. The last update timestamp of a Cloud Function. |
| `source_archive_url` | String |  | The Google Cloud Storage URL, starting with gs://, pointing to the zip
archive which contains the function. |
| `source_repository` | String |  | The hosted repository where the function is defined. |
| `latest_operation` | String |  | Output only. Name of the most recent operation modifying the function. If
the function status is `DEPLOYING` or `DELETING`, then it points to the
active operation. |
| `vpc_connector` | String |  | The VPC Network Connector that this cloud function can connect to. It can
be either the fully-qualified URI, or the short name of the network
connector resource. The format of this field is
`projects/*/locations/*/connectors/*`

This field is mutually exclusive with `network` field and will eventually
replace it.

See [the VPC documentation](https://cloud.google.com/compute/docs/vpc) for
more information on connecting Cloud projects. |
| `event_trigger` | String |  | A source that fires events in response to a condition in another service. |
| `max_instances` | i64 |  | The limit on the maximum number of function instances that may coexist at a
given time. |
| `location` | String | ✅ | Required. The project and location in which the function should be created, specified
in the format `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `https_trigger` | String | An HTTPS endpoint type of source that can be triggered via URL. |
| `entry_point` | String | The name of the function (as defined in source code) that will be
executed. Defaults to the resource name suffix, if not specified. For
backward compatibility, if function with given name is not found, then the
system will try to use function named "function".
For Node.js this is name of a function exported by the module specified
in `source_location`. |
| `labels` | HashMap<String, String> | Labels associated with this Cloud Function. |
| `service_account` | String | The email of the function's service account. If empty, defaults to
`{project_id}@appspot.gserviceaccount.com`. |
| `available_memory_mb` | i64 | The amount of memory in MB available for a function.
Defaults to 256MB. |
| `version_id` | String | Output only. The version identifier of the Cloud Function. Each deployment attempt
results in a new version of a function being created. |
| `environment_variables` | HashMap<String, String> | Environment variables that shall be available during function execution. |
| `runtime` | String | The runtime in which to run the function. Required when deploying a new
function, optional when updating an existing function. For a complete
list of possible choices, see the
[`gcloud` command
reference](/sdk/gcloud/reference/functions/deploy#--runtime). |
| `source_repository_url` | String | The URL pointing to the hosted repository where the function is defined.
There are supported Cloud Source Repository URLs in the following
formats:

To refer to a specific commit:
`https://source.developers.google.com/projects/*/repos/*/revisions/*/paths/*`
To refer to a moveable alias (branch):
`https://source.developers.google.com/projects/*/repos/*/moveable-aliases/*/paths/*`
In particular, to refer to HEAD use `master` moveable alias.
To refer to a specific fixed alias (tag):
`https://source.developers.google.com/projects/*/repos/*/fixed-aliases/*/paths/*`

You may omit `paths/*` if you want to use the main directory. |
| `status` | String | Output only. Status of the function deployment. |
| `timeout` | String | The function execution timeout. Execution is considered failed and
can be terminated if the function is not completed at the end of the
timeout period. Defaults to 60 seconds. |
| `name` | String | A user-defined name of the function. Function names must be unique
globally and match pattern `projects/*/locations/*/functions/*` |
| `network` | String | The VPC Network that this cloud function can connect to. It can be
either the fully-qualified URI, or the short name of the network resource.
If the short network name is used, the network must belong to the same
project. Otherwise, it must belong to a project within the same
organization. The format of this field is either
`projects/{project}/global/networks/{network}` or `{network}`, where
{project} is a project id where the network is defined, and {network} is
the short name of the network.

This field is mutually exclusive with `vpc_connector` and will be replaced
by it.

See [the VPC documentation](https://cloud.google.com/compute/docs/vpc) for
more information on connecting Cloud projects. |
| `source_upload_url` | String | The Google Cloud Storage signed URL used for source uploading, generated
by google.cloud.functions.v1beta2.GenerateUploadUrl |
| `update_time` | String | Output only. The last update timestamp of a Cloud Function. |
| `source_archive_url` | String | The Google Cloud Storage URL, starting with gs://, pointing to the zip
archive which contains the function. |
| `source_repository` | String | The hosted repository where the function is defined. |
| `latest_operation` | String | Output only. Name of the most recent operation modifying the function. If
the function status is `DEPLOYING` or `DELETING`, then it points to the
active operation. |
| `vpc_connector` | String | The VPC Network Connector that this cloud function can connect to. It can
be either the fully-qualified URI, or the short name of the network
connector resource. The format of this field is
`projects/*/locations/*/connectors/*`

This field is mutually exclusive with `network` field and will eventually
replace it.

See [the VPC documentation](https://cloud.google.com/compute/docs/vpc) for
more information on connecting Cloud projects. |
| `event_trigger` | String | A source that fires events in response to a condition in another service. |
| `max_instances` | i64 | The limit on the maximum number of function instances that may coexist at a
given time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create function
function = provider.cloudfunctions_api.Function {
    location = "value"  # Required. The project and location in which the function should be created, specified
in the format `projects/*/locations/*`
}

# Access function outputs
function_id = function.id
function_https_trigger = function.https_trigger
function_entry_point = function.entry_point
function_labels = function.labels
function_service_account = function.service_account
function_available_memory_mb = function.available_memory_mb
function_version_id = function.version_id
function_environment_variables = function.environment_variables
function_runtime = function.runtime
function_source_repository_url = function.source_repository_url
function_status = function.status
function_timeout = function.timeout
function_name = function.name
function_network = function.network
function_source_upload_url = function.source_upload_url
function_update_time = function.update_time
function_source_archive_url = function.source_archive_url
function_source_repository = function.source_repository
function_latest_operation = function.latest_operation
function_vpc_connector = function.vpc_connector
function_event_trigger = function.event_trigger
function_max_instances = function.max_instances
```

---


### Operation

Gets the latest state of a long-running operation.  Clients can use this
method to poll the operation result at intervals as recommended by the API
service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal response of the operation in case of success.  If the original
method returns no data on success, such as `Delete`, the response is
`google.protobuf.Empty`.  If the original method is standard
`Get`/`Create`/`Update`, the response should be the resource.  For other
methods, the response should have the type `XxxResponse`, where `Xxx`
is the original method name.  For example, if the original method name
is `TakeSnapshot()`, the inferred response type is
`TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that
originally returns it. If you use the default HTTP mapping, the
`name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation.  It typically
contains progress information and common metadata such as create time.
Some services might not provide such metadata.  Any method that returns a
long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress.
If `true`, the operation is completed, and either `error` or `response` is
available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_name = operation.name
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_name = operation.name
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
```

---


### Function

Creates a new function. If a function with the given name already exists in the specified project, the long running operation will return `ALREADY_EXISTS` error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The last update timestamp of a Cloud Function. |
| `state_messages` | Vec<String> |  | Output only. State Messages for this Cloud Function. |
| `upgrade_info` | String |  | Output only. UpgradeInfo for this Cloud Function |
| `url` | String |  | Output only. The deployed url for the function. |
| `environment` | String |  | Describe whether the function is 1st Gen or 2nd Gen. |
| `labels` | HashMap<String, String> |  | Labels associated with this Cloud Function. |
| `service_config` | String |  | Describes the Service being deployed. Currently deploys services to Cloud Run (fully managed). |
| `description` | String |  | User-provided description of a function. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `state` | String |  | Output only. State of the function. |
| `name` | String |  | A user-defined name of the function. Function names must be unique globally and match pattern `projects/*/locations/*/functions/*` |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `create_time` | String |  | Output only. The create timestamp of a Cloud Function. This is only applicable to 2nd Gen functions. |
| `build_config` | String |  | Describes the Build step of the function that builds a container from the given source. |
| `event_trigger` | String |  | An Eventarc trigger managed by Google Cloud Functions that fires events in response to a condition in another service. |
| `kms_key_name` | String |  | Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources. It must match the pattern `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. |
| `parent` | String | ✅ | Required. The project and location in which the function should be created, specified in the format `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The last update timestamp of a Cloud Function. |
| `state_messages` | Vec<String> | Output only. State Messages for this Cloud Function. |
| `upgrade_info` | String | Output only. UpgradeInfo for this Cloud Function |
| `url` | String | Output only. The deployed url for the function. |
| `environment` | String | Describe whether the function is 1st Gen or 2nd Gen. |
| `labels` | HashMap<String, String> | Labels associated with this Cloud Function. |
| `service_config` | String | Describes the Service being deployed. Currently deploys services to Cloud Run (fully managed). |
| `description` | String | User-provided description of a function. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `state` | String | Output only. State of the function. |
| `name` | String | A user-defined name of the function. Function names must be unique globally and match pattern `projects/*/locations/*/functions/*` |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `create_time` | String | Output only. The create timestamp of a Cloud Function. This is only applicable to 2nd Gen functions. |
| `build_config` | String | Describes the Build step of the function that builds a container from the given source. |
| `event_trigger` | String | An Eventarc trigger managed by Google Cloud Functions that fires events in response to a condition in another service. |
| `kms_key_name` | String | Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources. It must match the pattern `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create function
function = provider.cloudfunctions_api.Function {
    parent = "value"  # Required. The project and location in which the function should be created, specified in the format `projects/*/locations/*`
}

# Access function outputs
function_id = function.id
function_update_time = function.update_time
function_state_messages = function.state_messages
function_upgrade_info = function.upgrade_info
function_url = function.url
function_environment = function.environment
function_labels = function.labels
function_service_config = function.service_config
function_description = function.description
function_satisfies_pzs = function.satisfies_pzs
function_state = function.state
function_name = function.name
function_satisfies_pzi = function.satisfies_pzi
function_create_time = function.create_time
function_build_config = function.build_config
function_event_trigger = function.event_trigger
function_kms_key_name = function.kms_key_name
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
| `locations` | Vec<String> | A list of locations that matches the specified filter in the request. |
| `next_page_token` | String | The standard List next-page token. |


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
location_locations = location.locations
location_next_page_token = location.next_page_token
```

---


### Runtime

Returns a list of runtimes that are supported for the requested project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `runtimes` | Vec<String> | The runtimes that match the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access runtime outputs
runtime_id = runtime.id
runtime_runtimes = runtime.runtimes
```

---


### Function

Creates a new function. If a function with the given name already exists in the specified project, the long running operation will return `ALREADY_EXISTS` error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The last update timestamp of a Cloud Function. |
| `name` | String |  | A user-defined name of the function. Function names must be unique globally and match pattern `projects/*/locations/*/functions/*` |
| `upgrade_info` | String |  | Output only. UpgradeInfo for this Cloud Function |
| `description` | String |  | User-provided description of a function. |
| `service_config` | String |  | Describes the Service being deployed. Currently deploys services to Cloud Run (fully managed). |
| `create_time` | String |  | Output only. The create timestamp of a Cloud Function. This is only applicable to 2nd Gen functions. |
| `state_messages` | Vec<String> |  | Output only. State Messages for this Cloud Function. |
| `labels` | HashMap<String, String> |  | Labels associated with this Cloud Function. |
| `event_trigger` | String |  | An Eventarc trigger managed by Google Cloud Functions that fires events in response to a condition in another service. |
| `state` | String |  | Output only. State of the function. |
| `build_config` | String |  | Describes the Build step of the function that builds a container from the given source. |
| `environment` | String |  | Describe whether the function is 1st Gen or 2nd Gen. |
| `kms_key_name` | String |  | Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources. It must match the pattern `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `url` | String |  | Output only. The deployed url for the function. |
| `parent` | String | ✅ | Required. The project and location in which the function should be created, specified in the format `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The last update timestamp of a Cloud Function. |
| `name` | String | A user-defined name of the function. Function names must be unique globally and match pattern `projects/*/locations/*/functions/*` |
| `upgrade_info` | String | Output only. UpgradeInfo for this Cloud Function |
| `description` | String | User-provided description of a function. |
| `service_config` | String | Describes the Service being deployed. Currently deploys services to Cloud Run (fully managed). |
| `create_time` | String | Output only. The create timestamp of a Cloud Function. This is only applicable to 2nd Gen functions. |
| `state_messages` | Vec<String> | Output only. State Messages for this Cloud Function. |
| `labels` | HashMap<String, String> | Labels associated with this Cloud Function. |
| `event_trigger` | String | An Eventarc trigger managed by Google Cloud Functions that fires events in response to a condition in another service. |
| `state` | String | Output only. State of the function. |
| `build_config` | String | Describes the Build step of the function that builds a container from the given source. |
| `environment` | String | Describe whether the function is 1st Gen or 2nd Gen. |
| `kms_key_name` | String | Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources. It must match the pattern `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `url` | String | Output only. The deployed url for the function. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create function
function = provider.cloudfunctions_api.Function {
    parent = "value"  # Required. The project and location in which the function should be created, specified in the format `projects/*/locations/*`
}

# Access function outputs
function_id = function.id
function_update_time = function.update_time
function_name = function.name
function_upgrade_info = function.upgrade_info
function_description = function.description
function_service_config = function.service_config
function_create_time = function.create_time
function_state_messages = function.state_messages
function_labels = function.labels
function_event_trigger = function.event_trigger
function_state = function.state
function_build_config = function.build_config
function_environment = function.environment
function_kms_key_name = function.kms_key_name
function_satisfies_pzi = function.satisfies_pzi
function_satisfies_pzs = function.satisfies_pzs
function_url = function.url
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


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_name = operation.name
operation_metadata = operation.metadata
operation_response = operation.response
operation_done = operation.done
```

---


### Runtime

Returns a list of runtimes that are supported for the requested project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `runtimes` | Vec<String> | The runtimes that match the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access runtime outputs
runtime_id = runtime.id
runtime_runtimes = runtime.runtimes
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
| `locations` | Vec<String> | A list of locations that matches the specified filter in the request. |
| `next_page_token` | String | The standard List next-page token. |


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
location_locations = location.locations
location_next_page_token = location.next_page_token
```

---


### Runtime

Returns a list of runtimes that are supported for the requested project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `runtimes` | Vec<String> | The runtimes that match the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access runtime outputs
runtime_id = runtime.id
runtime_runtimes = runtime.runtimes
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Function

Creates a new function. If a function with the given name already exists in the specified project, the long running operation will return `ALREADY_EXISTS` error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The last update timestamp of a Cloud Function. |
| `create_time` | String |  | Output only. The create timestamp of a Cloud Function. This is only applicable to 2nd Gen functions. |
| `kms_key_name` | String |  | Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources. It must match the pattern `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. |
| `environment` | String |  | Describe whether the function is 1st Gen or 2nd Gen. |
| `name` | String |  | A user-defined name of the function. Function names must be unique globally and match pattern `projects/*/locations/*/functions/*` |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `upgrade_info` | String |  | Output only. UpgradeInfo for this Cloud Function |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `state_messages` | Vec<String> |  | Output only. State Messages for this Cloud Function. |
| `service_config` | String |  | Describes the Service being deployed. Currently deploys services to Cloud Run (fully managed). |
| `state` | String |  | Output only. State of the function. |
| `url` | String |  | Output only. The deployed url for the function. |
| `labels` | HashMap<String, String> |  | Labels associated with this Cloud Function. |
| `build_config` | String |  | Describes the Build step of the function that builds a container from the given source. |
| `event_trigger` | String |  | An Eventarc trigger managed by Google Cloud Functions that fires events in response to a condition in another service. |
| `description` | String |  | User-provided description of a function. |
| `parent` | String | ✅ | Required. The project and location in which the function should be created, specified in the format `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The last update timestamp of a Cloud Function. |
| `create_time` | String | Output only. The create timestamp of a Cloud Function. This is only applicable to 2nd Gen functions. |
| `kms_key_name` | String | Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources. It must match the pattern `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. |
| `environment` | String | Describe whether the function is 1st Gen or 2nd Gen. |
| `name` | String | A user-defined name of the function. Function names must be unique globally and match pattern `projects/*/locations/*/functions/*` |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `upgrade_info` | String | Output only. UpgradeInfo for this Cloud Function |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `state_messages` | Vec<String> | Output only. State Messages for this Cloud Function. |
| `service_config` | String | Describes the Service being deployed. Currently deploys services to Cloud Run (fully managed). |
| `state` | String | Output only. State of the function. |
| `url` | String | Output only. The deployed url for the function. |
| `labels` | HashMap<String, String> | Labels associated with this Cloud Function. |
| `build_config` | String | Describes the Build step of the function that builds a container from the given source. |
| `event_trigger` | String | An Eventarc trigger managed by Google Cloud Functions that fires events in response to a condition in another service. |
| `description` | String | User-provided description of a function. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create function
function = provider.cloudfunctions_api.Function {
    parent = "value"  # Required. The project and location in which the function should be created, specified in the format `projects/*/locations/*`
}

# Access function outputs
function_id = function.id
function_update_time = function.update_time
function_create_time = function.create_time
function_kms_key_name = function.kms_key_name
function_environment = function.environment
function_name = function.name
function_satisfies_pzs = function.satisfies_pzs
function_upgrade_info = function.upgrade_info
function_satisfies_pzi = function.satisfies_pzi
function_state_messages = function.state_messages
function_service_config = function.service_config
function_state = function.state
function_url = function.url
function_labels = function.labels
function_build_config = function.build_config
function_event_trigger = function.event_trigger
function_description = function.description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple function resources
function_0 = provider.cloudfunctions_api.Function {
    location = "value-0"
}
function_1 = provider.cloudfunctions_api.Function {
    location = "value-1"
}
function_2 = provider.cloudfunctions_api.Function {
    location = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    function = provider.cloudfunctions_api.Function {
        location = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudfunctions_api Documentation](https://cloud.google.com/cloudfunctions_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
