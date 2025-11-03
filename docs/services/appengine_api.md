# Appengine_api Service



**Resources**: 39

---

## Overview

The appengine_api service provides access to 39 resource types:

- [Instance](#instance) [CRD]
- [Module](#module) [RUD]
- [Location](#location) [R]
- [App](#app) [CRU]
- [Version](#version) [CRUD]
- [Operation](#operation) [R]
- [Authorized_domain](#authorized_domain) [R]
- [Location](#location) [R]
- [Operation](#operation) [R]
- [Service](#service) [RUD]
- [Application](#application) [U]
- [Instance](#instance) [CRD]
- [Version](#version) [CRUD]
- [Ingress_rule](#ingress_rule) [CRUD]
- [Domain_mapping](#domain_mapping) [CRUD]
- [Authorized_certificate](#authorized_certificate) [CRUD]
- [App](#app) [CRU]
- [Location](#location) [R]
- [Service](#service) [RUD]
- [Version](#version) [CRUD]
- [App](#app) [CRU]
- [Operation](#operation) [R]
- [Instance](#instance) [CRD]
- [Authorized_certificate](#authorized_certificate) [CRUD]
- [Domain_mapping](#domain_mapping) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [R]
- [Authorized_domain](#authorized_domain) [R]
- [Authorized_domain](#authorized_domain) [R]
- [Operation](#operation) [R]
- [Domain_mapping](#domain_mapping) [CRUD]
- [Version](#version) [CRUD]
- [Ingress_rule](#ingress_rule) [CRUD]
- [Instance](#instance) [CRD]
- [App](#app) [CRU]
- [Location](#location) [R]
- [Authorized_certificate](#authorized_certificate) [CRUD]
- [Service](#service) [RUD]
- [Application](#application) [U]

---

## Resources


### Instance

Enables debugging on a VM instance. This allows you to use the SSH command to connect to the virtual machine where the instance lives. While in "debug mode", the instance continues to serve live traffic. You should delete the instance when you are done debugging and then allow the system to take over and determine if another instance should be started.Only applicable for instances in App Engine flexible environment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ssh_key` | String |  | Public SSH key to add to the instance. Examples:
[USERNAME]:ssh-rsa [KEY_VALUE] [USERNAME]
[USERNAME]:ssh-rsa [KEY_VALUE] google-ssh {"userName":"[USERNAME]","expireOn":"[EXPIRE_TIME]"}For more information, see Adding and Removing SSH Keys (https://cloud.google.com/compute/docs/instances/adding-removing-ssh-keys). |
| `versions_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `modules_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `instances_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `apps_id` | String | ✅ | Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1/instances/instance-1. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `requests` | i64 | Number of requests since this instance was started.@OutputOnly |
| `vm_zone_name` | String | Zone where the virtual machine is located. Only applicable for instances in App Engine flexible environment.@OutputOnly |
| `qps` | f64 | Average queries per second (QPS) over the last minute.@OutputOnly |
| `app_engine_release` | String | App Engine release this instance is running on.@OutputOnly |
| `vm_id` | String | Virtual machine ID of this instance. Only applicable for instances in App Engine flexible environment.@OutputOnly |
| `vm_name` | String | Name of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment.@OutputOnly |
| `vm_status` | String | Status of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment.@OutputOnly |
| `vm_unlocked` | bool | Whether this instance is in debug mode. Only applicable for instances in App Engine flexible environment.@OutputOnly |
| `average_latency` | i64 | Average latency (ms) over the last minute.@OutputOnly |
| `errors` | i64 | Number of errors since this instance was started.@OutputOnly |
| `memory_usage` | String | Total memory in use (bytes).@OutputOnly |
| `id` | String | Relative name of the instance within the version. Example: instance-1.@OutputOnly |
| `name` | String | Full path to the Instance resource in the API. Example: apps/myapp/modules/default/versions/v1/instances/instance-1.@OutputOnly |
| `vm_ip` | String | The IP address of this instance. Only applicable for instances in App Engine flexible environment.@OutputOnly |
| `start_timestamp` | String | Time that this instance was started.@OutputOnly |
| `availability` | String | Availability of the instance.@OutputOnly |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.appengine_api.Instance {
    versions_id = "value"  # Part of `name`. See documentation of `appsId`.
    modules_id = "value"  # Part of `name`. See documentation of `appsId`.
    instances_id = "value"  # Part of `name`. See documentation of `appsId`.
    apps_id = "value"  # Part of `name`. Name of the resource requested. Example: apps/myapp/modules/default/versions/v1/instances/instance-1.
}

# Access instance outputs
instance_id = instance.id
instance_requests = instance.requests
instance_vm_zone_name = instance.vm_zone_name
instance_qps = instance.qps
instance_app_engine_release = instance.app_engine_release
instance_vm_id = instance.vm_id
instance_vm_name = instance.vm_name
instance_vm_status = instance.vm_status
instance_vm_unlocked = instance.vm_unlocked
instance_average_latency = instance.average_latency
instance_errors = instance.errors
instance_memory_usage = instance.memory_usage
instance_id = instance.id
instance_name = instance.name
instance_vm_ip = instance.vm_ip
instance_start_timestamp = instance.start_timestamp
instance_availability = instance.availability
```

---


### Module

Gets the current configuration of the specified module.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `split` | String |  | Mapping that defines fractional HTTP traffic diversion to different versions within the module. |
| `id` | String |  | Relative name of the module within the application. Example: default.@OutputOnly |
| `name` | String |  | Full path to the Module resource in the API. Example: apps/myapp/modules/default.@OutputOnly |
| `modules_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `apps_id` | String | ✅ | Part of `name`. Name of the resource to update. Example: apps/myapp/modules/default. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `split` | String | Mapping that defines fractional HTTP traffic diversion to different versions within the module. |
| `id` | String | Relative name of the module within the application. Example: default.@OutputOnly |
| `name` | String | Full path to the Module resource in the API. Example: apps/myapp/modules/default.@OutputOnly |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access module outputs
module_id = module.id
module_split = module.split
module_id = module.id
module_name = module.name
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example
{"cloud.googleapis.com/region": "us-east1"}
 |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1" |
| `location_id` | String | The canonical id for this location. For example: "us-east1". |


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
location_labels = location.labels
location_display_name = location.display_name
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
```

---


### App

Creates an App Engine application for a Google Cloud Platform project. Required fields:
id - The ID of the target Cloud Platform project.
location - The region (https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located.For more information about App Engine applications, see Managing Projects, Applications, and Billing (https://cloud.google.com/appengine/docs/python/console/).

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auth_domain` | String |  | Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account. |
| `default_bucket` | String |  | Google Cloud Storage bucket that can be used by this application to store content.@OutputOnly |
| `id` | String |  | Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp. |
| `code_bucket` | String |  | Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly |
| `name` | String |  | Full path to the Application resource in the API. Example: apps/myapp.@OutputOnly |
| `dispatch_rules` | Vec<String> |  | HTTP path dispatch rules for requests to the application that do not explicitly target a module or version. Rules are order-dependent.@OutputOnly |
| `iap` | String |  |  |
| `default_cookie_expiration` | String |  | Cookie expiration policy for this application. |
| `location` | String |  | Location from which this application will be run. Application instances will run out of data centers in the chosen location, which is also where all of the application's end user content is stored.Defaults to us-central.Options are:us-central - Central USeurope-west - Western Europeus-east1 - Eastern US |
| `default_hostname` | String |  | Hostname used to reach the application, as resolved by App Engine.@OutputOnly |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auth_domain` | String | Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account. |
| `default_bucket` | String | Google Cloud Storage bucket that can be used by this application to store content.@OutputOnly |
| `id` | String | Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp. |
| `code_bucket` | String | Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly |
| `name` | String | Full path to the Application resource in the API. Example: apps/myapp.@OutputOnly |
| `dispatch_rules` | Vec<String> | HTTP path dispatch rules for requests to the application that do not explicitly target a module or version. Rules are order-dependent.@OutputOnly |
| `iap` | String |  |
| `default_cookie_expiration` | String | Cookie expiration policy for this application. |
| `location` | String | Location from which this application will be run. Application instances will run out of data centers in the chosen location, which is also where all of the application's end user content is stored.Defaults to us-central.Options are:us-central - Central USeurope-west - Western Europeus-east1 - Eastern US |
| `default_hostname` | String | Hostname used to reach the application, as resolved by App Engine.@OutputOnly |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app
app = provider.appengine_api.App {
}

# Access app outputs
app_id = app.id
app_auth_domain = app.auth_domain
app_default_bucket = app.default_bucket
app_id = app.id
app_code_bucket = app.code_bucket
app_name = app.name
app_dispatch_rules = app.dispatch_rules
app_iap = app.iap
app_default_cookie_expiration = app.default_cookie_expiration
app_location = app.location
app_default_hostname = app.default_hostname
```

---


### Version

Deploys code and resource files to a new version.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `manual_scaling` | String |  | A module with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. |
| `error_handlers` | Vec<String> |  | Custom static error pages. Limited to 10KB per page.Only returned in GET requests if view=FULL is set. |
| `endpoints_api_service` | String |  | Cloud Endpoints configuration.If endpoints_api_service is set, the Cloud Endpoints Extensible Service Proxy will be provided to serve the API implemented by the app. |
| `beta_settings` | HashMap<String, String> |  | Metadata settings that are supplied to this version to enable beta runtime features. |
| `default_expiration` | String |  | Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#staticfileshandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set. |
| `deployment` | String |  | Code and application artifacts that make up this version.Only returned in GET requests if view=FULL is set. |
| `inbound_services` | Vec<String> |  | Before an application can receive email or XMPP messages, the application must be configured to enable the service. |
| `name` | String |  | Full path to the Version resource in the API. Example: apps/myapp/modules/default/versions/v1.@OutputOnly |
| `network` | String |  | Extra network settings. Only applicable for VM runtimes. |
| `id` | String |  | Relative name of the version within the module. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-". |
| `runtime_main_executable_path` | String |  | The path or name of the app's main executable. |
| `vm` | bool |  | Whether to deploy this version in a container on a virtual machine. |
| `creation_time` | String |  | Time that this version was created.@OutputOnly |
| `instance_class` | String |  | Instance class that is used to run this version. Valid values are:
AutomaticScaling: F1, F2, F4, F4_1G
ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling. |
| `deployer` | String |  | Email address of the user who created this version.@OutputOnly |
| `env_variables` | HashMap<String, String> |  | Environment variables made available to the application.Only returned in GET requests if view=FULL is set. |
| `automatic_scaling` | String |  | Automatic scaling is based on request rate, response latencies, and other application metrics. |
| `health_check` | String |  | Configures health checking for VM instances. Unhealthy instances are stopped and replaced with new instances. Only applicable for VM runtimes.Only returned in GET requests if view=FULL is set. |
| `env` | String |  | App Engine execution environment to use for this version.Defaults to 1. |
| `libraries` | Vec<String> |  | Configuration for third-party Python runtime libraries required by the application.Only returned in GET requests if view=FULL is set. |
| `resources` | String |  | Machine resources for this version. Only applicable for VM runtimes. |
| `serving_status` | String |  | Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING. |
| `handlers` | Vec<String> |  | An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted.Only returned in GET requests if view=FULL is set. |
| `basic_scaling` | String |  | A module with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity. |
| `runtime` | String |  | Desired runtime. Example: python27. |
| `threadsafe` | bool |  | Whether multiple requests can be dispatched to this version at once. |
| `nobuild_files_regex` | String |  | Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set. |
| `runtime_api_version` | String |  | The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard/<language>/config/appref |
| `api_config` | String |  | Serving configuration for Google Cloud Endpoints (https://cloud.google.com/appengine/docs/python/endpoints/).Only returned in GET requests if view=FULL is set. |
| `apps_id` | String | ✅ | Part of `name`. Name of the resource to update. Example: apps/myapp/modules/default. |
| `modules_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `manual_scaling` | String | A module with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. |
| `error_handlers` | Vec<String> | Custom static error pages. Limited to 10KB per page.Only returned in GET requests if view=FULL is set. |
| `endpoints_api_service` | String | Cloud Endpoints configuration.If endpoints_api_service is set, the Cloud Endpoints Extensible Service Proxy will be provided to serve the API implemented by the app. |
| `beta_settings` | HashMap<String, String> | Metadata settings that are supplied to this version to enable beta runtime features. |
| `default_expiration` | String | Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#staticfileshandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set. |
| `deployment` | String | Code and application artifacts that make up this version.Only returned in GET requests if view=FULL is set. |
| `inbound_services` | Vec<String> | Before an application can receive email or XMPP messages, the application must be configured to enable the service. |
| `name` | String | Full path to the Version resource in the API. Example: apps/myapp/modules/default/versions/v1.@OutputOnly |
| `network` | String | Extra network settings. Only applicable for VM runtimes. |
| `id` | String | Relative name of the version within the module. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-". |
| `runtime_main_executable_path` | String | The path or name of the app's main executable. |
| `vm` | bool | Whether to deploy this version in a container on a virtual machine. |
| `creation_time` | String | Time that this version was created.@OutputOnly |
| `instance_class` | String | Instance class that is used to run this version. Valid values are:
AutomaticScaling: F1, F2, F4, F4_1G
ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling. |
| `deployer` | String | Email address of the user who created this version.@OutputOnly |
| `env_variables` | HashMap<String, String> | Environment variables made available to the application.Only returned in GET requests if view=FULL is set. |
| `automatic_scaling` | String | Automatic scaling is based on request rate, response latencies, and other application metrics. |
| `health_check` | String | Configures health checking for VM instances. Unhealthy instances are stopped and replaced with new instances. Only applicable for VM runtimes.Only returned in GET requests if view=FULL is set. |
| `env` | String | App Engine execution environment to use for this version.Defaults to 1. |
| `libraries` | Vec<String> | Configuration for third-party Python runtime libraries required by the application.Only returned in GET requests if view=FULL is set. |
| `resources` | String | Machine resources for this version. Only applicable for VM runtimes. |
| `serving_status` | String | Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING. |
| `handlers` | Vec<String> | An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted.Only returned in GET requests if view=FULL is set. |
| `basic_scaling` | String | A module with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity. |
| `runtime` | String | Desired runtime. Example: python27. |
| `threadsafe` | bool | Whether multiple requests can be dispatched to this version at once. |
| `nobuild_files_regex` | String | Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set. |
| `runtime_api_version` | String | The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard/<language>/config/appref |
| `api_config` | String | Serving configuration for Google Cloud Endpoints (https://cloud.google.com/appengine/docs/python/endpoints/).Only returned in GET requests if view=FULL is set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.appengine_api.Version {
    apps_id = "value"  # Part of `name`. Name of the resource to update. Example: apps/myapp/modules/default.
    modules_id = "value"  # Part of `name`. See documentation of `appsId`.
}

# Access version outputs
version_id = version.id
version_manual_scaling = version.manual_scaling
version_error_handlers = version.error_handlers
version_endpoints_api_service = version.endpoints_api_service
version_beta_settings = version.beta_settings
version_default_expiration = version.default_expiration
version_deployment = version.deployment
version_inbound_services = version.inbound_services
version_name = version.name
version_network = version.network
version_id = version.id
version_runtime_main_executable_path = version.runtime_main_executable_path
version_vm = version.vm
version_creation_time = version.creation_time
version_instance_class = version.instance_class
version_deployer = version.deployer
version_env_variables = version.env_variables
version_automatic_scaling = version.automatic_scaling
version_health_check = version.health_check
version_env = version.env
version_libraries = version.libraries
version_resources = version.resources
version_serving_status = version.serving_status
version_handlers = version.handlers
version_basic_scaling = version.basic_scaling
version_runtime = version.runtime
version_threadsafe = version.threadsafe
version_nobuild_files_regex = version.nobuild_files_regex
version_runtime_api_version = version.runtime_api_version
version_api_config = version.api_config
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
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should have the format of operations/some/unique/name. |


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
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Authorized_domain

Lists all domains the user is authorized to administer.

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

# Access authorized_domain outputs
authorized_domain_id = authorized_domain.id
authorized_domain_next_page_token = authorized_domain.next_page_token
authorized_domain_domains = authorized_domain.domains
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}  |
| `location_id` | String | The canonical id for this location. For example: "us-east1". |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1" |


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
location_labels = location.labels
location_location_id = location.location_id
location_display_name = location.display_name
location_metadata = location.metadata
location_name = location.name
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


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
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### Service

Gets the current configuration of the specified service.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `split` | String |  | Mapping that defines fractional HTTP traffic diversion to different versions within the service. |
| `name` | String |  | Output only. Full path to the Service resource in the API. Example: apps/myapp/services/default.@OutputOnly |
| `id` | String |  | Output only. Relative name of the service within the application. Example: default.@OutputOnly |
| `labels` | HashMap<String, String> |  | A set of labels to apply to this service. Labels are key/value pairs that describe the service and all resources that belong to it (e.g., versions). The labels can be used to search and group resources, and are propagated to the usage and billing reports, enabling fine-grain analysis of costs. An example of using labels is to tag resources belonging to different environments (e.g., "env=prod", "env=qa"). Label keys and values can be no longer than 63 characters and can only contain lowercase letters, numeric characters, underscores, dashes, and international characters. Label keys must start with a lowercase letter or an international character. Each service can have at most 32 labels. |
| `network_settings` | String |  | Ingress settings for this service. Will apply to all versions. |
| `generated_customer_metadata` | HashMap<String, String> |  | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetServiceRequest |
| `locations_id` | String | ✅ | Part of `name`. See documentation of `projectsId`. |
| `projects_id` | String | ✅ | Part of `name`. Required. Name of the resource to update. Example: apps/myapp/services/default. |
| `applications_id` | String | ✅ | Part of `name`. See documentation of `projectsId`. |
| `services_id` | String | ✅ | Part of `name`. See documentation of `projectsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `split` | String | Mapping that defines fractional HTTP traffic diversion to different versions within the service. |
| `name` | String | Output only. Full path to the Service resource in the API. Example: apps/myapp/services/default.@OutputOnly |
| `id` | String | Output only. Relative name of the service within the application. Example: default.@OutputOnly |
| `labels` | HashMap<String, String> | A set of labels to apply to this service. Labels are key/value pairs that describe the service and all resources that belong to it (e.g., versions). The labels can be used to search and group resources, and are propagated to the usage and billing reports, enabling fine-grain analysis of costs. An example of using labels is to tag resources belonging to different environments (e.g., "env=prod", "env=qa"). Label keys and values can be no longer than 63 characters and can only contain lowercase letters, numeric characters, underscores, dashes, and international characters. Label keys must start with a lowercase letter or an international character. Each service can have at most 32 labels. |
| `network_settings` | String | Ingress settings for this service. Will apply to all versions. |
| `generated_customer_metadata` | HashMap<String, String> | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetServiceRequest |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access service outputs
service_id = service.id
service_split = service.split
service_name = service.name
service_id = service.id
service_labels = service.labels
service_network_settings = service.network_settings
service_generated_customer_metadata = service.generated_customer_metadata
```

---


### Application



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auth_domain` | String |  | Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account. |
| `gcr_domain` | String |  | Output only. The Google Container Registry domain used for storing managed build docker images for this application. |
| `iap` | String |  |  |
| `database_type` | String |  | The type of the Cloud Firestore or Cloud Datastore database associated with this application. |
| `generated_customer_metadata` | HashMap<String, String> |  | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetApplicationRequest |
| `ssl_policy` | String |  | The SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS < 1.2 and allow only Modern Ciphers suite |
| `name` | String |  |  |
| `default_cookie_expiration` | String |  | Cookie expiration policy for this application. |
| `service_account` | String |  | The service account associated with the application. This is the app-level default identity. If no identity provided during create version, Admin API will fallback to this one. |
| `code_bucket` | String |  | Output only. Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly |
| `default_bucket` | String |  | Output only. Google Cloud Storage bucket that can be used by this application to store content.@OutputOnly |
| `default_hostname` | String |  | Output only. Hostname used to reach this application, as resolved by App Engine.@OutputOnly |
| `dispatch_rules` | Vec<String> |  | HTTP path dispatch rules for requests to the application that do not explicitly target a service or version. Rules are order-dependent. Up to 20 dispatch rules can be supported. |
| `id` | String |  | Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp. |
| `location_id` | String |  | Location from which this application runs. Application instances run out of the data centers in the specified location, which is also where all of the application's end user content is stored.Defaults to us-central.View the list of supported locations (https://cloud.google.com/appengine/docs/locations). |
| `serving_status` | String |  | Serving status of this application. |
| `feature_settings` | String |  | The feature specific settings to be used in the application. |
| `locations_id` | String | ✅ | Part of `name`. See documentation of `projectsId`. |
| `projects_id` | String | ✅ | Part of `name`. Required. Name of the Application resource to update. Example: apps/myapp. |
| `applications_id` | String | ✅ | Part of `name`. See documentation of `projectsId`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

```

---


### Instance

Enables debugging on a VM instance. This allows you to use the SSH command to connect to the virtual machine where the instance lives. While in "debug mode", the instance continues to serve live traffic. You should delete the instance when you are done debugging and then allow the system to take over and determine if another instance should be started.Only applicable for instances in App Engine flexible environment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ssh_key` | String |  | Public SSH key to add to the instance. Examples: [USERNAME]:ssh-rsa [KEY_VALUE] [USERNAME] [USERNAME]:ssh-rsa [KEY_VALUE] google-ssh {"userName":"[USERNAME]","expireOn":"[EXPIRE_TIME]"}For more information, see Adding and Removing SSH Keys (https://cloud.google.com/compute/docs/instances/adding-removing-ssh-keys). |
| `services_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `versions_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `apps_id` | String | ✅ | Part of `name`. Required. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1. |
| `instances_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `vm_name` | String | Output only. Name of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment. |
| `vm_ip` | String | Output only. The IP address of this instance. Only applicable for instances in App Engine flexible environment. |
| `availability` | String | Output only. Availability of the instance. |
| `app_engine_release` | String | Output only. App Engine release this instance is running on. |
| `vm_liveness` | String | Output only. The liveness health check of this instance. Only applicable for instances in App Engine flexible environment. |
| `start_time` | String | Output only. Time that this instance was started.@OutputOnly |
| `errors` | i64 | Output only. Number of errors since this instance was started. |
| `vm_debug_enabled` | bool | Output only. Whether this instance is in debug mode. Only applicable for instances in App Engine flexible environment. |
| `qps` | f64 | Output only. Average queries per second (QPS) over the last minute. |
| `requests` | i64 | Output only. Number of requests since this instance was started. |
| `average_latency` | i64 | Output only. Average latency (ms) over the last minute. |
| `vm_zone_name` | String | Output only. Zone where the virtual machine is located. Only applicable for instances in App Engine flexible environment. |
| `id` | String | Output only. Relative name of the instance within the version. Example: instance-1. |
| `vm_id` | String | Output only. Virtual machine ID of this instance. Only applicable for instances in App Engine flexible environment. |
| `vm_status` | String | Output only. Status of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment. |
| `name` | String | Output only. Full path to the Instance resource in the API. Example: apps/myapp/services/default/versions/v1/instances/instance-1. |
| `memory_usage` | String | Output only. Total memory in use (bytes). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.appengine_api.Instance {
    services_id = "value"  # Part of `name`. See documentation of `appsId`.
    versions_id = "value"  # Part of `name`. See documentation of `appsId`.
    apps_id = "value"  # Part of `name`. Required. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1.
    instances_id = "value"  # Part of `name`. See documentation of `appsId`.
}

# Access instance outputs
instance_id = instance.id
instance_vm_name = instance.vm_name
instance_vm_ip = instance.vm_ip
instance_availability = instance.availability
instance_app_engine_release = instance.app_engine_release
instance_vm_liveness = instance.vm_liveness
instance_start_time = instance.start_time
instance_errors = instance.errors
instance_vm_debug_enabled = instance.vm_debug_enabled
instance_qps = instance.qps
instance_requests = instance.requests
instance_average_latency = instance.average_latency
instance_vm_zone_name = instance.vm_zone_name
instance_id = instance.id
instance_vm_id = instance.vm_id
instance_vm_status = instance.vm_status
instance_name = instance.name
instance_memory_usage = instance.memory_usage
```

---


### Version

Deploys code and resource files to a new version.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `runtime_main_executable_path` | String |  | The path or name of the app's main executable. |
| `automatic_scaling` | String |  | Automatic scaling is based on request rate, response latencies, and other application metrics. Instances are dynamically created and destroyed as needed in order to handle traffic. |
| `service_account` | String |  | The identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default if this field is neither provided in app.yaml file nor through CLI flag. |
| `version_url` | String |  | Output only. Serving URL for this version. Example: "https://myversion-dot-myservice-dot-myapp.appspot.com"@OutputOnly |
| `runtime_channel` | String |  | The channel of the runtime to use. Only available for some runtimes. Defaults to the default channel. |
| `env` | String |  | App Engine execution environment for this version.Defaults to standard. |
| `vpc_access_connector` | String |  | Enables VPC connectivity for standard apps. |
| `generated_customer_metadata` | HashMap<String, String> |  | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetVersionRequest |
| `manual_scaling` | String |  | A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. Manually scaled versions are sometimes referred to as "backends". |
| `health_check` | String |  | Configures health checking for instances. Unhealthy instances are stopped and replaced with new instances. Only applicable in the App Engine flexible environment. |
| `inbound_services` | Vec<String> |  | Before an application can receive email or XMPP messages, the application must be configured to enable the service. |
| `build_env_variables` | HashMap<String, String> |  | Environment variables available to the build environment.Only returned in GET requests if view=FULL is set. |
| `beta_settings` | HashMap<String, String> |  | Metadata settings that are supplied to this version to enable beta runtime features. |
| `endpoints_api_service` | String |  | Cloud Endpoints configuration.If endpoints_api_service is set, the Cloud Endpoints Extensible Service Proxy will be provided to serve the API implemented by the app. |
| `flexible_runtime_settings` | String |  | Settings for App Engine flexible runtimes. |
| `disk_usage_bytes` | String |  | Output only. Total size in bytes of all the files that are included in this version and currently hosted on the App Engine disk.@OutputOnly |
| `threadsafe` | bool |  | Whether multiple requests can be dispatched to this version at once. |
| `zones` | Vec<String> |  | The Google Compute Engine zones that are supported by this version in the App Engine flexible environment. Deprecated. |
| `serving_status` | String |  | Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING. |
| `instance_class` | String |  | Instance class that is used to run this version. Valid values are: AutomaticScaling: F1, F2, F4, F4_1G ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling. |
| `vm` | bool |  | Whether to deploy this version in a container on a virtual machine. |
| `api_config` | String |  | Serving configuration for Google Cloud Endpoints (https://cloud.google.com/endpoints).Only returned in GET requests if view=FULL is set. |
| `deployment` | String |  | Code and application artifacts that make up this version.Only returned in GET requests if view=FULL is set. |
| `id` | String |  | Relative name of the version within the service. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-". |
| `network` | String |  | Extra network settings. Only applicable in the App Engine flexible environment. |
| `name` | String |  | Output only. Full path to the Version resource in the API. Example: apps/myapp/services/default/versions/v1.@OutputOnly |
| `nobuild_files_regex` | String |  | Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set. |
| `app_engine_apis` | bool |  | Allows App Engine second generation runtimes to access the legacy bundled services. |
| `basic_scaling` | String |  | A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity. |
| `default_expiration` | String |  | Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StaticFilesHandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set. |
| `created_by` | String |  | Output only. Email address of the user who created this version.@OutputOnly |
| `entrypoint` | String |  | The entrypoint for the application. |
| `liveness_check` | String |  | Configures liveness health checking for instances. Unhealthy instances are stopped and replaced with new instances |
| `create_time` | String |  | Time that this version was created.@OutputOnly |
| `error_handlers` | Vec<String> |  | Custom static error pages. Limited to 10KB per page.Only returned in GET requests if view=FULL is set. |
| `readiness_check` | String |  | Configures readiness health checking for instances. Unhealthy instances are not put into the backend traffic rotation. |
| `libraries` | Vec<String> |  | Configuration for third-party Python runtime libraries that are required by the application.Only returned in GET requests if view=FULL is set. |
| `handlers` | Vec<String> |  | An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted.Only returned in GET requests if view=FULL is set. |
| `resources` | String |  | Machine resources for this version. Only applicable in the App Engine flexible environment. |
| `runtime` | String |  | Desired runtime. Example: python27. |
| `runtime_api_version` | String |  | The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard//config/appref |
| `env_variables` | HashMap<String, String> |  | Environment variables available to the application.Only returned in GET requests if view=FULL is set. |
| `apps_id` | String | ✅ | Part of `parent`. Required. Name of the parent resource to create this version under. Example: apps/myapp/services/default. |
| `services_id` | String | ✅ | Part of `parent`. See documentation of `appsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `runtime_main_executable_path` | String | The path or name of the app's main executable. |
| `automatic_scaling` | String | Automatic scaling is based on request rate, response latencies, and other application metrics. Instances are dynamically created and destroyed as needed in order to handle traffic. |
| `service_account` | String | The identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default if this field is neither provided in app.yaml file nor through CLI flag. |
| `version_url` | String | Output only. Serving URL for this version. Example: "https://myversion-dot-myservice-dot-myapp.appspot.com"@OutputOnly |
| `runtime_channel` | String | The channel of the runtime to use. Only available for some runtimes. Defaults to the default channel. |
| `env` | String | App Engine execution environment for this version.Defaults to standard. |
| `vpc_access_connector` | String | Enables VPC connectivity for standard apps. |
| `generated_customer_metadata` | HashMap<String, String> | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetVersionRequest |
| `manual_scaling` | String | A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. Manually scaled versions are sometimes referred to as "backends". |
| `health_check` | String | Configures health checking for instances. Unhealthy instances are stopped and replaced with new instances. Only applicable in the App Engine flexible environment. |
| `inbound_services` | Vec<String> | Before an application can receive email or XMPP messages, the application must be configured to enable the service. |
| `build_env_variables` | HashMap<String, String> | Environment variables available to the build environment.Only returned in GET requests if view=FULL is set. |
| `beta_settings` | HashMap<String, String> | Metadata settings that are supplied to this version to enable beta runtime features. |
| `endpoints_api_service` | String | Cloud Endpoints configuration.If endpoints_api_service is set, the Cloud Endpoints Extensible Service Proxy will be provided to serve the API implemented by the app. |
| `flexible_runtime_settings` | String | Settings for App Engine flexible runtimes. |
| `disk_usage_bytes` | String | Output only. Total size in bytes of all the files that are included in this version and currently hosted on the App Engine disk.@OutputOnly |
| `threadsafe` | bool | Whether multiple requests can be dispatched to this version at once. |
| `zones` | Vec<String> | The Google Compute Engine zones that are supported by this version in the App Engine flexible environment. Deprecated. |
| `serving_status` | String | Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING. |
| `instance_class` | String | Instance class that is used to run this version. Valid values are: AutomaticScaling: F1, F2, F4, F4_1G ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling. |
| `vm` | bool | Whether to deploy this version in a container on a virtual machine. |
| `api_config` | String | Serving configuration for Google Cloud Endpoints (https://cloud.google.com/endpoints).Only returned in GET requests if view=FULL is set. |
| `deployment` | String | Code and application artifacts that make up this version.Only returned in GET requests if view=FULL is set. |
| `id` | String | Relative name of the version within the service. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-". |
| `network` | String | Extra network settings. Only applicable in the App Engine flexible environment. |
| `name` | String | Output only. Full path to the Version resource in the API. Example: apps/myapp/services/default/versions/v1.@OutputOnly |
| `nobuild_files_regex` | String | Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set. |
| `app_engine_apis` | bool | Allows App Engine second generation runtimes to access the legacy bundled services. |
| `basic_scaling` | String | A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity. |
| `default_expiration` | String | Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#StaticFilesHandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set. |
| `created_by` | String | Output only. Email address of the user who created this version.@OutputOnly |
| `entrypoint` | String | The entrypoint for the application. |
| `liveness_check` | String | Configures liveness health checking for instances. Unhealthy instances are stopped and replaced with new instances |
| `create_time` | String | Time that this version was created.@OutputOnly |
| `error_handlers` | Vec<String> | Custom static error pages. Limited to 10KB per page.Only returned in GET requests if view=FULL is set. |
| `readiness_check` | String | Configures readiness health checking for instances. Unhealthy instances are not put into the backend traffic rotation. |
| `libraries` | Vec<String> | Configuration for third-party Python runtime libraries that are required by the application.Only returned in GET requests if view=FULL is set. |
| `handlers` | Vec<String> | An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted.Only returned in GET requests if view=FULL is set. |
| `resources` | String | Machine resources for this version. Only applicable in the App Engine flexible environment. |
| `runtime` | String | Desired runtime. Example: python27. |
| `runtime_api_version` | String | The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard//config/appref |
| `env_variables` | HashMap<String, String> | Environment variables available to the application.Only returned in GET requests if view=FULL is set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.appengine_api.Version {
    apps_id = "value"  # Part of `parent`. Required. Name of the parent resource to create this version under. Example: apps/myapp/services/default.
    services_id = "value"  # Part of `parent`. See documentation of `appsId`.
}

# Access version outputs
version_id = version.id
version_runtime_main_executable_path = version.runtime_main_executable_path
version_automatic_scaling = version.automatic_scaling
version_service_account = version.service_account
version_version_url = version.version_url
version_runtime_channel = version.runtime_channel
version_env = version.env
version_vpc_access_connector = version.vpc_access_connector
version_generated_customer_metadata = version.generated_customer_metadata
version_manual_scaling = version.manual_scaling
version_health_check = version.health_check
version_inbound_services = version.inbound_services
version_build_env_variables = version.build_env_variables
version_beta_settings = version.beta_settings
version_endpoints_api_service = version.endpoints_api_service
version_flexible_runtime_settings = version.flexible_runtime_settings
version_disk_usage_bytes = version.disk_usage_bytes
version_threadsafe = version.threadsafe
version_zones = version.zones
version_serving_status = version.serving_status
version_instance_class = version.instance_class
version_vm = version.vm
version_api_config = version.api_config
version_deployment = version.deployment
version_id = version.id
version_network = version.network
version_name = version.name
version_nobuild_files_regex = version.nobuild_files_regex
version_app_engine_apis = version.app_engine_apis
version_basic_scaling = version.basic_scaling
version_default_expiration = version.default_expiration
version_created_by = version.created_by
version_entrypoint = version.entrypoint
version_liveness_check = version.liveness_check
version_create_time = version.create_time
version_error_handlers = version.error_handlers
version_readiness_check = version.readiness_check
version_libraries = version.libraries
version_handlers = version.handlers
version_resources = version.resources
version_runtime = version.runtime
version_runtime_api_version = version.runtime_api_version
version_env_variables = version.env_variables
```

---


### Ingress_rule

Creates a firewall rule for the application.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String |  | The action to take on matched requests. |
| `priority` | i64 |  |  |
| `description` | String |  | An optional string description of this rule. This field has a maximum length of 400 characters. |
| `source_range` | String |  | IP address or range, defined using CIDR notation, of requests that this rule applies to. You can use the wildcard character "*" to match all IPs equivalent to "0/0" and "::/0" together. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. Truncation will be silently performed on addresses which are not properly truncated. For example, 1.2.3.4/24 is accepted as the same address as 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 is accepted as the same address as 2001:db8::/32. |
| `apps_id` | String | ✅ | Part of `parent`. Required. Name of the parent Firewall collection in which to create a new rule. Example: apps/myapp/firewall/ingressRules. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action` | String | The action to take on matched requests. |
| `priority` | i64 |  |
| `description` | String | An optional string description of this rule. This field has a maximum length of 400 characters. |
| `source_range` | String | IP address or range, defined using CIDR notation, of requests that this rule applies to. You can use the wildcard character "*" to match all IPs equivalent to "0/0" and "::/0" together. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. Truncation will be silently performed on addresses which are not properly truncated. For example, 1.2.3.4/24 is accepted as the same address as 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 is accepted as the same address as 2001:db8::/32. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ingress_rule
ingress_rule = provider.appengine_api.Ingress_rule {
    apps_id = "value"  # Part of `parent`. Required. Name of the parent Firewall collection in which to create a new rule. Example: apps/myapp/firewall/ingressRules.
}

# Access ingress_rule outputs
ingress_rule_id = ingress_rule.id
ingress_rule_action = ingress_rule.action
ingress_rule_priority = ingress_rule.priority
ingress_rule_description = ingress_rule.description
ingress_rule_source_range = ingress_rule.source_range
```

---


### Domain_mapping

Maps a domain to an application. A user must be authorized to administer a domain in order to map it to an application. For a list of available authorized domains, see AuthorizedDomains.ListAuthorizedDomains.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Relative name of the domain serving the application. Example: example.com. |
| `name` | String |  | Full path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com.@OutputOnly |
| `resource_records` | Vec<String> |  | The resource records required to configure this domain mapping. These records must be added to the domain's DNS configuration in order to serve the application via this domain mapping.@OutputOnly |
| `ssl_settings` | String |  | SSL configuration for this domain. If unconfigured, this domain will not serve with SSL. |
| `projects_id` | String | ✅ | Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp. |
| `locations_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |
| `applications_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Relative name of the domain serving the application. Example: example.com. |
| `name` | String | Full path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com.@OutputOnly |
| `resource_records` | Vec<String> | The resource records required to configure this domain mapping. These records must be added to the domain's DNS configuration in order to serve the application via this domain mapping.@OutputOnly |
| `ssl_settings` | String | SSL configuration for this domain. If unconfigured, this domain will not serve with SSL. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domain_mapping
domain_mapping = provider.appengine_api.Domain_mapping {
    projects_id = "value"  # Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp.
    locations_id = "value"  # Part of `parent`. See documentation of `projectsId`.
    applications_id = "value"  # Part of `parent`. See documentation of `projectsId`.
}

# Access domain_mapping outputs
domain_mapping_id = domain_mapping.id
domain_mapping_id = domain_mapping.id
domain_mapping_name = domain_mapping.name
domain_mapping_resource_records = domain_mapping.resource_records
domain_mapping_ssl_settings = domain_mapping.ssl_settings
```

---


### Authorized_certificate

Uploads the specified SSL certificate.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expire_time` | String |  | The time when this certificate expires. To update the renewal time on this certificate, upload an SSL certificate with a different expiration time using AuthorizedCertificates.UpdateAuthorizedCertificate.@OutputOnly |
| `domain_mappings_count` | i64 |  | Aggregate count of the domain mappings with this certificate mapped. This count includes domain mappings on applications for which the user does not have VIEWER permissions.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `id` | String |  | Relative name of the certificate. This is a unique value autogenerated on AuthorizedCertificate resource creation. Example: 12345.@OutputOnly |
| `name` | String |  | Full path to the AuthorizedCertificate resource in the API. Example: apps/myapp/authorizedCertificates/12345.@OutputOnly |
| `managed_certificate` | String |  | Only applicable if this certificate is managed by App Engine. Managed certificates are tied to the lifecycle of a DomainMapping and cannot be updated or deleted via the AuthorizedCertificates API. If this certificate is manually administered by the user, this field will be empty.@OutputOnly |
| `visible_domain_mappings` | Vec<String> |  | The full paths to user visible Domain Mapping resources that have this certificate mapped. Example: apps/myapp/domainMappings/example.com.This may not represent the full list of mapped domain mappings if the user does not have VIEWER permissions on all of the applications that have this certificate mapped. See domain_mappings_count for a complete count.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `domain_names` | Vec<String> |  | Topmost applicable domains of this certificate. This certificate applies to these domains and their subdomains. Example: example.com.@OutputOnly |
| `certificate_raw_data` | String |  | The SSL certificate serving the AuthorizedCertificate resource. This must be obtained independently from a certificate authority. |
| `display_name` | String |  | The user-specified display name of the certificate. This is not guaranteed to be unique. Example: My Certificate. |
| `projects_id` | String | ✅ | Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp. |
| `applications_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |
| `locations_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expire_time` | String | The time when this certificate expires. To update the renewal time on this certificate, upload an SSL certificate with a different expiration time using AuthorizedCertificates.UpdateAuthorizedCertificate.@OutputOnly |
| `domain_mappings_count` | i64 | Aggregate count of the domain mappings with this certificate mapped. This count includes domain mappings on applications for which the user does not have VIEWER permissions.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `id` | String | Relative name of the certificate. This is a unique value autogenerated on AuthorizedCertificate resource creation. Example: 12345.@OutputOnly |
| `name` | String | Full path to the AuthorizedCertificate resource in the API. Example: apps/myapp/authorizedCertificates/12345.@OutputOnly |
| `managed_certificate` | String | Only applicable if this certificate is managed by App Engine. Managed certificates are tied to the lifecycle of a DomainMapping and cannot be updated or deleted via the AuthorizedCertificates API. If this certificate is manually administered by the user, this field will be empty.@OutputOnly |
| `visible_domain_mappings` | Vec<String> | The full paths to user visible Domain Mapping resources that have this certificate mapped. Example: apps/myapp/domainMappings/example.com.This may not represent the full list of mapped domain mappings if the user does not have VIEWER permissions on all of the applications that have this certificate mapped. See domain_mappings_count for a complete count.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `domain_names` | Vec<String> | Topmost applicable domains of this certificate. This certificate applies to these domains and their subdomains. Example: example.com.@OutputOnly |
| `certificate_raw_data` | String | The SSL certificate serving the AuthorizedCertificate resource. This must be obtained independently from a certificate authority. |
| `display_name` | String | The user-specified display name of the certificate. This is not guaranteed to be unique. Example: My Certificate. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorized_certificate
authorized_certificate = provider.appengine_api.Authorized_certificate {
    projects_id = "value"  # Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp.
    applications_id = "value"  # Part of `parent`. See documentation of `projectsId`.
    locations_id = "value"  # Part of `parent`. See documentation of `projectsId`.
}

# Access authorized_certificate outputs
authorized_certificate_id = authorized_certificate.id
authorized_certificate_expire_time = authorized_certificate.expire_time
authorized_certificate_domain_mappings_count = authorized_certificate.domain_mappings_count
authorized_certificate_id = authorized_certificate.id
authorized_certificate_name = authorized_certificate.name
authorized_certificate_managed_certificate = authorized_certificate.managed_certificate
authorized_certificate_visible_domain_mappings = authorized_certificate.visible_domain_mappings
authorized_certificate_domain_names = authorized_certificate.domain_names
authorized_certificate_certificate_raw_data = authorized_certificate.certificate_raw_data
authorized_certificate_display_name = authorized_certificate.display_name
```

---


### App

Creates an App Engine application for a Google Cloud Platform project. Required fields: id - The ID of the target Cloud Platform project. location - The region (https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located.For more information about App Engine applications, see Managing Projects, Applications, and Billing (https://cloud.google.com/appengine/docs/standard/python/console/).

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auth_domain` | String |  | Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account. |
| `gcr_domain` | String |  | Output only. The Google Container Registry domain used for storing managed build docker images for this application. |
| `iap` | String |  |  |
| `database_type` | String |  | The type of the Cloud Firestore or Cloud Datastore database associated with this application. |
| `generated_customer_metadata` | HashMap<String, String> |  | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetApplicationRequest |
| `ssl_policy` | String |  | The SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS < 1.2 and allow only Modern Ciphers suite |
| `name` | String |  |  |
| `default_cookie_expiration` | String |  | Cookie expiration policy for this application. |
| `service_account` | String |  | The service account associated with the application. This is the app-level default identity. If no identity provided during create version, Admin API will fallback to this one. |
| `code_bucket` | String |  | Output only. Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly |
| `default_bucket` | String |  | Output only. Google Cloud Storage bucket that can be used by this application to store content.@OutputOnly |
| `default_hostname` | String |  | Output only. Hostname used to reach this application, as resolved by App Engine.@OutputOnly |
| `dispatch_rules` | Vec<String> |  | HTTP path dispatch rules for requests to the application that do not explicitly target a service or version. Rules are order-dependent. Up to 20 dispatch rules can be supported. |
| `id` | String |  | Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp. |
| `location_id` | String |  | Location from which this application runs. Application instances run out of the data centers in the specified location, which is also where all of the application's end user content is stored.Defaults to us-central.View the list of supported locations (https://cloud.google.com/appengine/docs/locations). |
| `serving_status` | String |  | Serving status of this application. |
| `feature_settings` | String |  | The feature specific settings to be used in the application. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auth_domain` | String | Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account. |
| `gcr_domain` | String | Output only. The Google Container Registry domain used for storing managed build docker images for this application. |
| `iap` | String |  |
| `database_type` | String | The type of the Cloud Firestore or Cloud Datastore database associated with this application. |
| `generated_customer_metadata` | HashMap<String, String> | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetApplicationRequest |
| `ssl_policy` | String | The SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS < 1.2 and allow only Modern Ciphers suite |
| `name` | String |  |
| `default_cookie_expiration` | String | Cookie expiration policy for this application. |
| `service_account` | String | The service account associated with the application. This is the app-level default identity. If no identity provided during create version, Admin API will fallback to this one. |
| `code_bucket` | String | Output only. Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly |
| `default_bucket` | String | Output only. Google Cloud Storage bucket that can be used by this application to store content.@OutputOnly |
| `default_hostname` | String | Output only. Hostname used to reach this application, as resolved by App Engine.@OutputOnly |
| `dispatch_rules` | Vec<String> | HTTP path dispatch rules for requests to the application that do not explicitly target a service or version. Rules are order-dependent. Up to 20 dispatch rules can be supported. |
| `id` | String | Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp. |
| `location_id` | String | Location from which this application runs. Application instances run out of the data centers in the specified location, which is also where all of the application's end user content is stored.Defaults to us-central.View the list of supported locations (https://cloud.google.com/appengine/docs/locations). |
| `serving_status` | String | Serving status of this application. |
| `feature_settings` | String | The feature specific settings to be used in the application. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app
app = provider.appengine_api.App {
}

# Access app outputs
app_id = app.id
app_auth_domain = app.auth_domain
app_gcr_domain = app.gcr_domain
app_iap = app.iap
app_database_type = app.database_type
app_generated_customer_metadata = app.generated_customer_metadata
app_ssl_policy = app.ssl_policy
app_name = app.name
app_default_cookie_expiration = app.default_cookie_expiration
app_service_account = app.service_account
app_code_bucket = app.code_bucket
app_default_bucket = app.default_bucket
app_default_hostname = app.default_hostname
app_dispatch_rules = app.dispatch_rules
app_id = app.id
app_location_id = app.location_id
app_serving_status = app.serving_status
app_feature_settings = app.feature_settings
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example
{"cloud.googleapis.com/region": "us-east1"}
 |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1" |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: "us-east1". |


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
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
location_display_name = location.display_name
location_location_id = location.location_id
```

---


### Service

Gets the current configuration of the specified service.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Full path to the Service resource in the API. Example: apps/myapp/services/default.@OutputOnly |
| `id` | String |  | Relative name of the service within the application. Example: default.@OutputOnly |
| `split` | String |  | Mapping that defines fractional HTTP traffic diversion to different versions within the service. |
| `apps_id` | String | ✅ | Part of `name`. Name of the resource to update. Example: apps/myapp/services/default. |
| `services_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Full path to the Service resource in the API. Example: apps/myapp/services/default.@OutputOnly |
| `id` | String | Relative name of the service within the application. Example: default.@OutputOnly |
| `split` | String | Mapping that defines fractional HTTP traffic diversion to different versions within the service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access service outputs
service_id = service.id
service_name = service.name
service_id = service.id
service_split = service.split
```

---


### Version

Deploys new code and resource files to a new version.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `env` | String |  | App Engine execution environment to use for this version.Defaults to 1. |
| `handlers` | Vec<String> |  | An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted.Only returned in GET requests if view=FULL is set. |
| `runtime_api_version` | String |  | The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard/<language>/config/appref |
| `vm` | bool |  | Whether to deploy this version in a container on a virtual machine. |
| `runtime` | String |  | Desired runtime. Example: python27. |
| `deployer` | String |  | Email address of the user who created this version.@OutputOnly |
| `manual_scaling` | String |  | A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. |
| `inbound_services` | Vec<String> |  | Before an application can receive email or XMPP messages, the application must be configured to enable the service. |
| `basic_scaling` | String |  | A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity. |
| `id` | String |  | Relative name of the version within the module. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-". |
| `endpoints_api_service` | String |  | Cloud Endpoints configuration.If endpoints_api_service is set, the Cloud Endpoints Extensible Service Proxy will be provided to serve the API implemented by the app. |
| `instance_class` | String |  | Instance class that is used to run this version. Valid values are:
AutomaticScaling: F1, F2, F4, F4_1G
ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling. |
| `libraries` | Vec<String> |  | Configuration for third-party Python runtime libraries required by the application.Only returned in GET requests if view=FULL is set. |
| `network` | String |  | Extra network settings. Only applicable for VM runtimes. |
| `nobuild_files_regex` | String |  | Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set. |
| `automatic_scaling` | String |  | Automatic scaling is based on request rate, response latencies, and other application metrics. |
| `deployment` | String |  | Code and application artifacts that make up this version.Only returned in GET requests if view=FULL is set. |
| `env_variables` | HashMap<String, String> |  | Environment variables made available to the application.Only returned in GET requests if view=FULL is set. |
| `threadsafe` | bool |  | Whether multiple requests can be dispatched to this version at once. |
| `name` | String |  | Full path to the Version resource in the API. Example: apps/myapp/services/default/versions/v1.@OutputOnly |
| `serving_status` | String |  | Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING. |
| `error_handlers` | Vec<String> |  | Custom static error pages. Limited to 10KB per page.Only returned in GET requests if view=FULL is set. |
| `resources` | String |  | Machine resources for this version. Only applicable for VM runtimes. |
| `runtime_main_executable_path` | String |  | The path or name of the app's main executable. |
| `default_expiration` | String |  | Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#staticfileshandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set. |
| `health_check` | String |  | Configures health checking for VM instances. Unhealthy instances are be stopped and replaced with new instances. Only applicable for VM runtimes.Only returned in GET requests if view=FULL is set. |
| `beta_settings` | HashMap<String, String> |  | Metadata settings that are supplied to this version to enable beta runtime features. |
| `creation_time` | String |  | Time that this version was created.@OutputOnly |
| `api_config` | String |  | Serving configuration for Google Cloud Endpoints (https://cloud.google.com/appengine/docs/python/endpoints/).Only returned in GET requests if view=FULL is set. |
| `disk_usage_bytes` | String |  | Total size of version files hosted on App Engine disk in bytes.@OutputOnly |
| `services_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `apps_id` | String | ✅ | Part of `name`. Name of the resource to update. For example: "apps/myapp/services/default". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `env` | String | App Engine execution environment to use for this version.Defaults to 1. |
| `handlers` | Vec<String> | An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted.Only returned in GET requests if view=FULL is set. |
| `runtime_api_version` | String | The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard/<language>/config/appref |
| `vm` | bool | Whether to deploy this version in a container on a virtual machine. |
| `runtime` | String | Desired runtime. Example: python27. |
| `deployer` | String | Email address of the user who created this version.@OutputOnly |
| `manual_scaling` | String | A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. |
| `inbound_services` | Vec<String> | Before an application can receive email or XMPP messages, the application must be configured to enable the service. |
| `basic_scaling` | String | A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity. |
| `id` | String | Relative name of the version within the module. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-". |
| `endpoints_api_service` | String | Cloud Endpoints configuration.If endpoints_api_service is set, the Cloud Endpoints Extensible Service Proxy will be provided to serve the API implemented by the app. |
| `instance_class` | String | Instance class that is used to run this version. Valid values are:
AutomaticScaling: F1, F2, F4, F4_1G
ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling. |
| `libraries` | Vec<String> | Configuration for third-party Python runtime libraries required by the application.Only returned in GET requests if view=FULL is set. |
| `network` | String | Extra network settings. Only applicable for VM runtimes. |
| `nobuild_files_regex` | String | Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set. |
| `automatic_scaling` | String | Automatic scaling is based on request rate, response latencies, and other application metrics. |
| `deployment` | String | Code and application artifacts that make up this version.Only returned in GET requests if view=FULL is set. |
| `env_variables` | HashMap<String, String> | Environment variables made available to the application.Only returned in GET requests if view=FULL is set. |
| `threadsafe` | bool | Whether multiple requests can be dispatched to this version at once. |
| `name` | String | Full path to the Version resource in the API. Example: apps/myapp/services/default/versions/v1.@OutputOnly |
| `serving_status` | String | Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING. |
| `error_handlers` | Vec<String> | Custom static error pages. Limited to 10KB per page.Only returned in GET requests if view=FULL is set. |
| `resources` | String | Machine resources for this version. Only applicable for VM runtimes. |
| `runtime_main_executable_path` | String | The path or name of the app's main executable. |
| `default_expiration` | String | Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#staticfileshandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set. |
| `health_check` | String | Configures health checking for VM instances. Unhealthy instances are be stopped and replaced with new instances. Only applicable for VM runtimes.Only returned in GET requests if view=FULL is set. |
| `beta_settings` | HashMap<String, String> | Metadata settings that are supplied to this version to enable beta runtime features. |
| `creation_time` | String | Time that this version was created.@OutputOnly |
| `api_config` | String | Serving configuration for Google Cloud Endpoints (https://cloud.google.com/appengine/docs/python/endpoints/).Only returned in GET requests if view=FULL is set. |
| `disk_usage_bytes` | String | Total size of version files hosted on App Engine disk in bytes.@OutputOnly |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.appengine_api.Version {
    services_id = "value"  # Part of `name`. See documentation of `appsId`.
    apps_id = "value"  # Part of `name`. Name of the resource to update. For example: "apps/myapp/services/default".
}

# Access version outputs
version_id = version.id
version_env = version.env
version_handlers = version.handlers
version_runtime_api_version = version.runtime_api_version
version_vm = version.vm
version_runtime = version.runtime
version_deployer = version.deployer
version_manual_scaling = version.manual_scaling
version_inbound_services = version.inbound_services
version_basic_scaling = version.basic_scaling
version_id = version.id
version_endpoints_api_service = version.endpoints_api_service
version_instance_class = version.instance_class
version_libraries = version.libraries
version_network = version.network
version_nobuild_files_regex = version.nobuild_files_regex
version_automatic_scaling = version.automatic_scaling
version_deployment = version.deployment
version_env_variables = version.env_variables
version_threadsafe = version.threadsafe
version_name = version.name
version_serving_status = version.serving_status
version_error_handlers = version.error_handlers
version_resources = version.resources
version_runtime_main_executable_path = version.runtime_main_executable_path
version_default_expiration = version.default_expiration
version_health_check = version.health_check
version_beta_settings = version.beta_settings
version_creation_time = version.creation_time
version_api_config = version.api_config
version_disk_usage_bytes = version.disk_usage_bytes
```

---


### App

Creates an App Engine application for a Google Cloud Platform project. Required fields:
id - The ID of the target Cloud Platform project.
location - The region (https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located.For more information about App Engine applications, see Managing Projects, Applications, and Billing (https://cloud.google.com/appengine/docs/python/console/).

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_hostname` | String |  | Hostname used to reach the application, as resolved by App Engine.@OutputOnly |
| `dispatch_rules` | Vec<String> |  | HTTP path dispatch rules for requests to the application that do not explicitly target a service or version. Rules are order-dependent.@OutputOnly |
| `id` | String |  | Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp. |
| `default_bucket` | String |  | A Google Cloud Storage bucket that can be used by the application to store content.@OutputOnly |
| `code_bucket` | String |  | A Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly |
| `name` | String |  | Full path to the Application resource in the API. Example: apps/myapp.@OutputOnly |
| `location` | String |  | Location from which this application will be run. Application instances will run out of data centers in the chosen location, which is also where all of the application's end user content is stored.Defaults to us-central.Options are:us-central - Central USeurope-west - Western Europeus-east1 - Eastern US |
| `default_cookie_expiration` | String |  | Cookie expiration policy for this application. |
| `iap` | String |  |  |
| `auth_domain` | String |  | Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_hostname` | String | Hostname used to reach the application, as resolved by App Engine.@OutputOnly |
| `dispatch_rules` | Vec<String> | HTTP path dispatch rules for requests to the application that do not explicitly target a service or version. Rules are order-dependent.@OutputOnly |
| `id` | String | Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp. |
| `default_bucket` | String | A Google Cloud Storage bucket that can be used by the application to store content.@OutputOnly |
| `code_bucket` | String | A Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly |
| `name` | String | Full path to the Application resource in the API. Example: apps/myapp.@OutputOnly |
| `location` | String | Location from which this application will be run. Application instances will run out of data centers in the chosen location, which is also where all of the application's end user content is stored.Defaults to us-central.Options are:us-central - Central USeurope-west - Western Europeus-east1 - Eastern US |
| `default_cookie_expiration` | String | Cookie expiration policy for this application. |
| `iap` | String |  |
| `auth_domain` | String | Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app
app = provider.appengine_api.App {
}

# Access app outputs
app_id = app.id
app_default_hostname = app.default_hostname
app_dispatch_rules = app.dispatch_rules
app_id = app.id
app_default_bucket = app.default_bucket
app_code_bucket = app.code_bucket
app_name = app.name
app_location = app.location
app_default_cookie_expiration = app.default_cookie_expiration
app_iap = app.iap
app_auth_domain = app.auth_domain
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
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should have the format of operations/some/unique/name. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |


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
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
```

---


### Instance

Enables debugging on a VM instance. This allows you to use the SSH command to connect to the virtual machine where the instance lives. While in "debug mode", the instance continues to serve live traffic. You should delete the instance when you are done debugging and then allow the system to take over and determine if another instance should be started.Only applicable for instances in App Engine flexible environment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ssh_key` | String |  | Public SSH key to add to the instance. Examples:
[USERNAME]:ssh-rsa [KEY_VALUE] [USERNAME]
[USERNAME]:ssh-rsa [KEY_VALUE] google-ssh {"userName":"[USERNAME]","expireOn":"[EXPIRE_TIME]"}For more information, see Adding and Removing SSH Keys (https://cloud.google.com/compute/docs/instances/adding-removing-ssh-keys). |
| `apps_id` | String | ✅ | Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1. |
| `versions_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `services_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `instances_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `requests` | i64 | Number of requests since this instance was started.@OutputOnly |
| `memory_usage` | String | Total memory in use (bytes).@OutputOnly |
| `average_latency` | i64 | Average latency (ms) over the last minute.@OutputOnly |
| `vm_name` | String | Name of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment.@OutputOnly |
| `vm_zone_name` | String | Zone where the virtual machine is located. Only applicable for instances in App Engine flexible environment.@OutputOnly |
| `vm_status` | String | Status of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment.@OutputOnly |
| `id` | String | Relative name of the instance within the version. Example: instance-1.@OutputOnly |
| `qps` | f64 | Average queries per second (QPS) over the last minute.@OutputOnly |
| `errors` | i64 | Number of errors since this instance was started.@OutputOnly |
| `vm_unlocked` | bool | Whether this instance is in debug mode. Only applicable for instances in App Engine flexible environment.@OutputOnly |
| `availability` | String | Availability of the instance.@OutputOnly |
| `start_timestamp` | String | Time that this instance was started.@OutputOnly |
| `vm_ip` | String | The IP address of this instance. Only applicable for instances in App Engine flexible environment.@OutputOnly |
| `app_engine_release` | String | App Engine release this instance is running on.@OutputOnly |
| `name` | String | Full path to the Instance resource in the API. Example: apps/myapp/services/default/versions/v1/instances/instance-1.@OutputOnly |
| `vm_id` | String | Virtual machine ID of this instance. Only applicable for instances in App Engine flexible environment.@OutputOnly |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.appengine_api.Instance {
    apps_id = "value"  # Part of `name`. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1.
    versions_id = "value"  # Part of `name`. See documentation of `appsId`.
    services_id = "value"  # Part of `name`. See documentation of `appsId`.
    instances_id = "value"  # Part of `name`. See documentation of `appsId`.
}

# Access instance outputs
instance_id = instance.id
instance_requests = instance.requests
instance_memory_usage = instance.memory_usage
instance_average_latency = instance.average_latency
instance_vm_name = instance.vm_name
instance_vm_zone_name = instance.vm_zone_name
instance_vm_status = instance.vm_status
instance_id = instance.id
instance_qps = instance.qps
instance_errors = instance.errors
instance_vm_unlocked = instance.vm_unlocked
instance_availability = instance.availability
instance_start_timestamp = instance.start_timestamp
instance_vm_ip = instance.vm_ip
instance_app_engine_release = instance.app_engine_release
instance_name = instance.name
instance_vm_id = instance.vm_id
```

---


### Authorized_certificate

Uploads the specified SSL certificate.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_mappings_count` | i64 |  | Aggregate count of the domain mappings with this certificate mapped. This count includes domain mappings on applications for which the user does not have VIEWER permissions.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `certificate_raw_data` | String |  | The SSL certificate serving the AuthorizedCertificate resource. This must be obtained independently from a certificate authority. |
| `domain_names` | Vec<String> |  | Topmost applicable domains of this certificate. This certificate applies to these domains and their subdomains. Example: example.com.@OutputOnly |
| `id` | String |  | Relative name of the certificate. This is a unique value autogenerated on AuthorizedCertificate resource creation. Example: 12345.@OutputOnly |
| `name` | String |  | Full path to the AuthorizedCertificate resource in the API. Example: apps/myapp/authorizedCertificates/12345.@OutputOnly |
| `expire_time` | String |  | The time when this certificate expires. To update the renewal time on this certificate, upload an SSL certificate with a different expiration time using AuthorizedCertificates.UpdateAuthorizedCertificate.@OutputOnly |
| `visible_domain_mappings` | Vec<String> |  | The full paths to user visible Domain Mapping resources that have this certificate mapped. Example: apps/myapp/domainMappings/example.com.This may not represent the full list of mapped domain mappings if the user does not have VIEWER permissions on all of the applications that have this certificate mapped. See domain_mappings_count for a complete count.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `managed_certificate` | String |  | Only applicable if this certificate is managed by App Engine. Managed certificates are tied to the lifecycle of a DomainMapping and cannot be updated or deleted via the AuthorizedCertificates API. If this certificate is manually administered by the user, this field will be empty.@OutputOnly |
| `display_name` | String |  | The user-specified display name of the certificate. This is not guaranteed to be unique. Example: My Certificate. |
| `applications_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |
| `projects_id` | String | ✅ | Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp. |
| `locations_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_mappings_count` | i64 | Aggregate count of the domain mappings with this certificate mapped. This count includes domain mappings on applications for which the user does not have VIEWER permissions.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `certificate_raw_data` | String | The SSL certificate serving the AuthorizedCertificate resource. This must be obtained independently from a certificate authority. |
| `domain_names` | Vec<String> | Topmost applicable domains of this certificate. This certificate applies to these domains and their subdomains. Example: example.com.@OutputOnly |
| `id` | String | Relative name of the certificate. This is a unique value autogenerated on AuthorizedCertificate resource creation. Example: 12345.@OutputOnly |
| `name` | String | Full path to the AuthorizedCertificate resource in the API. Example: apps/myapp/authorizedCertificates/12345.@OutputOnly |
| `expire_time` | String | The time when this certificate expires. To update the renewal time on this certificate, upload an SSL certificate with a different expiration time using AuthorizedCertificates.UpdateAuthorizedCertificate.@OutputOnly |
| `visible_domain_mappings` | Vec<String> | The full paths to user visible Domain Mapping resources that have this certificate mapped. Example: apps/myapp/domainMappings/example.com.This may not represent the full list of mapped domain mappings if the user does not have VIEWER permissions on all of the applications that have this certificate mapped. See domain_mappings_count for a complete count.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `managed_certificate` | String | Only applicable if this certificate is managed by App Engine. Managed certificates are tied to the lifecycle of a DomainMapping and cannot be updated or deleted via the AuthorizedCertificates API. If this certificate is manually administered by the user, this field will be empty.@OutputOnly |
| `display_name` | String | The user-specified display name of the certificate. This is not guaranteed to be unique. Example: My Certificate. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorized_certificate
authorized_certificate = provider.appengine_api.Authorized_certificate {
    applications_id = "value"  # Part of `parent`. See documentation of `projectsId`.
    projects_id = "value"  # Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp.
    locations_id = "value"  # Part of `parent`. See documentation of `projectsId`.
}

# Access authorized_certificate outputs
authorized_certificate_id = authorized_certificate.id
authorized_certificate_domain_mappings_count = authorized_certificate.domain_mappings_count
authorized_certificate_certificate_raw_data = authorized_certificate.certificate_raw_data
authorized_certificate_domain_names = authorized_certificate.domain_names
authorized_certificate_id = authorized_certificate.id
authorized_certificate_name = authorized_certificate.name
authorized_certificate_expire_time = authorized_certificate.expire_time
authorized_certificate_visible_domain_mappings = authorized_certificate.visible_domain_mappings
authorized_certificate_managed_certificate = authorized_certificate.managed_certificate
authorized_certificate_display_name = authorized_certificate.display_name
```

---


### Domain_mapping

Maps a domain to an application. A user must be authorized to administer a domain in order to map it to an application. For a list of available authorized domains, see AuthorizedDomains.ListAuthorizedDomains.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Relative name of the domain serving the application. Example: example.com. |
| `resource_records` | Vec<String> |  | The resource records required to configure this domain mapping. These records must be added to the domain's DNS configuration in order to serve the application via this domain mapping.@OutputOnly |
| `name` | String |  | Full path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com.@OutputOnly |
| `ssl_settings` | String |  | SSL configuration for this domain. If unconfigured, this domain will not serve with SSL. |
| `projects_id` | String | ✅ | Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp. |
| `applications_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |
| `locations_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Relative name of the domain serving the application. Example: example.com. |
| `resource_records` | Vec<String> | The resource records required to configure this domain mapping. These records must be added to the domain's DNS configuration in order to serve the application via this domain mapping.@OutputOnly |
| `name` | String | Full path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com.@OutputOnly |
| `ssl_settings` | String | SSL configuration for this domain. If unconfigured, this domain will not serve with SSL. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domain_mapping
domain_mapping = provider.appengine_api.Domain_mapping {
    projects_id = "value"  # Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp.
    applications_id = "value"  # Part of `parent`. See documentation of `projectsId`.
    locations_id = "value"  # Part of `parent`. See documentation of `projectsId`.
}

# Access domain_mapping outputs
domain_mapping_id = domain_mapping.id
domain_mapping_id = domain_mapping.id
domain_mapping_resource_records = domain_mapping.resource_records
domain_mapping_name = domain_mapping.name
domain_mapping_ssl_settings = domain_mapping.ssl_settings
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1" |
| `location_id` | String | The canonical id for this location. For example: "us-east1". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}  |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |


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
location_location_id = location.location_id
location_labels = location.labels
location_display_name = location.display_name
location_metadata = location.metadata
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
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
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

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Authorized_domain

Lists all domains the user is authorized to administer.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domains` | Vec<String> | The authorized domains belonging to the user. |
| `next_page_token` | String | Continuation token for fetching the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access authorized_domain outputs
authorized_domain_id = authorized_domain.id
authorized_domain_domains = authorized_domain.domains
authorized_domain_next_page_token = authorized_domain.next_page_token
```

---


### Authorized_domain

Lists all domains the user is authorized to administer.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domains` | Vec<String> | The authorized domains belonging to the user. |
| `next_page_token` | String | Continuation token for fetching the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access authorized_domain outputs
authorized_domain_id = authorized_domain.id
authorized_domain_domains = authorized_domain.domains
authorized_domain_next_page_token = authorized_domain.next_page_token
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as Delete, the response is google.protobuf.Empty. If the original method is standard Get/Create/Update, the response should be the resource. For other methods, the response should have the type XxxResponse, where Xxx is the original method name. For example, if the original method name is TakeSnapshot(), the inferred response type is TakeSnapshotResponse. |
| `done` | bool | If the value is false, it means the operation is still in progress. If true, the operation is completed, and either error or response is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the name should be a resource name ending with operations/{unique_id}. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


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
operation_done = operation.done
operation_name = operation.name
operation_metadata = operation.metadata
```

---


### Domain_mapping

Maps a domain to an application. A user must be authorized to administer a domain in order to map it to an application. For a list of available authorized domains, see AuthorizedDomains.ListAuthorizedDomains.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Full path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com.@OutputOnly |
| `resource_records` | Vec<String> |  | The resource records required to configure this domain mapping. These records must be added to the domain's DNS configuration in order to serve the application via this domain mapping.@OutputOnly |
| `id` | String |  | Relative name of the domain serving the application. Example: example.com. |
| `ssl_settings` | String |  | SSL configuration for this domain. If unconfigured, this domain will not serve with SSL. |
| `projects_id` | String | ✅ | Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp. |
| `locations_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |
| `applications_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Full path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com.@OutputOnly |
| `resource_records` | Vec<String> | The resource records required to configure this domain mapping. These records must be added to the domain's DNS configuration in order to serve the application via this domain mapping.@OutputOnly |
| `id` | String | Relative name of the domain serving the application. Example: example.com. |
| `ssl_settings` | String | SSL configuration for this domain. If unconfigured, this domain will not serve with SSL. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domain_mapping
domain_mapping = provider.appengine_api.Domain_mapping {
    projects_id = "value"  # Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp.
    locations_id = "value"  # Part of `parent`. See documentation of `projectsId`.
    applications_id = "value"  # Part of `parent`. See documentation of `projectsId`.
}

# Access domain_mapping outputs
domain_mapping_id = domain_mapping.id
domain_mapping_name = domain_mapping.name
domain_mapping_resource_records = domain_mapping.resource_records
domain_mapping_id = domain_mapping.id
domain_mapping_ssl_settings = domain_mapping.ssl_settings
```

---


### Version

Deploys code and resource files to a new version.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `entrypoint` | String |  | The entrypoint for the application. |
| `instance_class` | String |  | Instance class that is used to run this version. Valid values are: AutomaticScaling: F1, F2, F4, F4_1G ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling. |
| `runtime_main_executable_path` | String |  | The path or name of the app's main executable. |
| `version_url` | String |  | Output only. Serving URL for this version. Example: "https://myversion-dot-myservice-dot-myapp.appspot.com"@OutputOnly |
| `runtime` | String |  | Desired runtime. Example: python27. |
| `readiness_check` | String |  | Configures readiness health checking for instances. Unhealthy instances are not put into the backend traffic rotation. |
| `basic_scaling` | String |  | A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity. |
| `vm` | bool |  | Whether to deploy this version in a container on a virtual machine. |
| `zones` | Vec<String> |  | The Google Compute Engine zones that are supported by this version in the App Engine flexible environment. Deprecated. |
| `inbound_services` | Vec<String> |  | Before an application can receive email or XMPP messages, the application must be configured to enable the service. |
| `nobuild_files_regex` | String |  | Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set. |
| `created_by` | String |  | Output only. Email address of the user who created this version.@OutputOnly |
| `env_variables` | HashMap<String, String> |  | Environment variables available to the application.Only returned in GET requests if view=FULL is set. |
| `create_time` | String |  | Time that this version was created.@OutputOnly |
| `automatic_scaling` | String |  | Automatic scaling is based on request rate, response latencies, and other application metrics. Instances are dynamically created and destroyed as needed in order to handle traffic. |
| `app_engine_apis` | bool |  | Allows App Engine second generation runtimes to access the legacy bundled services. |
| `name` | String |  | Output only. Full path to the Version resource in the API. Example: apps/myapp/services/default/versions/v1.@OutputOnly |
| `api_config` | String |  | Serving configuration for Google Cloud Endpoints (https://cloud.google.com/endpoints).Only returned in GET requests if view=FULL is set. |
| `endpoints_api_service` | String |  | Cloud Endpoints configuration.If endpoints_api_service is set, the Cloud Endpoints Extensible Service Proxy will be provided to serve the API implemented by the app. |
| `flexible_runtime_settings` | String |  | Settings for App Engine flexible runtimes. |
| `runtime_api_version` | String |  | The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard//config/appref |
| `threadsafe` | bool |  | Whether multiple requests can be dispatched to this version at once. |
| `liveness_check` | String |  | Configures liveness health checking for instances. Unhealthy instances are stopped and replaced with new instances |
| `libraries` | Vec<String> |  | Configuration for third-party Python runtime libraries that are required by the application.Only returned in GET requests if view=FULL is set. |
| `default_expiration` | String |  | Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StaticFilesHandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set. |
| `manual_scaling` | String |  | A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. Manually scaled versions are sometimes referred to as "backends". |
| `resources` | String |  | Machine resources for this version. Only applicable in the App Engine flexible environment. |
| `handlers` | Vec<String> |  | An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted.Only returned in GET requests if view=FULL is set. |
| `health_check` | String |  | Configures health checking for instances. Unhealthy instances are stopped and replaced with new instances. Only applicable in the App Engine flexible environment. |
| `id` | String |  | Relative name of the version within the service. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-". |
| `build_env_variables` | HashMap<String, String> |  | Environment variables available to the build environment.Only returned in GET requests if view=FULL is set. |
| `error_handlers` | Vec<String> |  | Custom static error pages. Limited to 10KB per page.Only returned in GET requests if view=FULL is set. |
| `deployment` | String |  | Code and application artifacts that make up this version.Only returned in GET requests if view=FULL is set. |
| `runtime_channel` | String |  | The channel of the runtime to use. Only available for some runtimes. Defaults to the default channel. |
| `service_account` | String |  | The identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default if this field is neither provided in app.yaml file nor through CLI flag. |
| `network` | String |  | Extra network settings. Only applicable in the App Engine flexible environment. |
| `disk_usage_bytes` | String |  | Output only. Total size in bytes of all the files that are included in this version and currently hosted on the App Engine disk.@OutputOnly |
| `beta_settings` | HashMap<String, String> |  | Metadata settings that are supplied to this version to enable beta runtime features. |
| `vpc_access_connector` | String |  | Enables VPC connectivity for standard apps. |
| `vpc_egress` | String |  | Enables VPC egress connectivity for standard apps. |
| `generated_customer_metadata` | HashMap<String, String> |  | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetVersionRequest |
| `env` | String |  | App Engine execution environment for this version.Defaults to standard. |
| `serving_status` | String |  | Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING. |
| `services_id` | String | ✅ | Part of `parent`. See documentation of `appsId`. |
| `apps_id` | String | ✅ | Part of `parent`. Required. Name of the parent resource to create this version under. Example: apps/myapp/services/default. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entrypoint` | String | The entrypoint for the application. |
| `instance_class` | String | Instance class that is used to run this version. Valid values are: AutomaticScaling: F1, F2, F4, F4_1G ManualScaling or BasicScaling: B1, B2, B4, B8, B4_1GDefaults to F1 for AutomaticScaling and B1 for ManualScaling or BasicScaling. |
| `runtime_main_executable_path` | String | The path or name of the app's main executable. |
| `version_url` | String | Output only. Serving URL for this version. Example: "https://myversion-dot-myservice-dot-myapp.appspot.com"@OutputOnly |
| `runtime` | String | Desired runtime. Example: python27. |
| `readiness_check` | String | Configures readiness health checking for instances. Unhealthy instances are not put into the backend traffic rotation. |
| `basic_scaling` | String | A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity. |
| `vm` | bool | Whether to deploy this version in a container on a virtual machine. |
| `zones` | Vec<String> | The Google Compute Engine zones that are supported by this version in the App Engine flexible environment. Deprecated. |
| `inbound_services` | Vec<String> | Before an application can receive email or XMPP messages, the application must be configured to enable the service. |
| `nobuild_files_regex` | String | Files that match this pattern will not be built into this version. Only applicable for Go runtimes.Only returned in GET requests if view=FULL is set. |
| `created_by` | String | Output only. Email address of the user who created this version.@OutputOnly |
| `env_variables` | HashMap<String, String> | Environment variables available to the application.Only returned in GET requests if view=FULL is set. |
| `create_time` | String | Time that this version was created.@OutputOnly |
| `automatic_scaling` | String | Automatic scaling is based on request rate, response latencies, and other application metrics. Instances are dynamically created and destroyed as needed in order to handle traffic. |
| `app_engine_apis` | bool | Allows App Engine second generation runtimes to access the legacy bundled services. |
| `name` | String | Output only. Full path to the Version resource in the API. Example: apps/myapp/services/default/versions/v1.@OutputOnly |
| `api_config` | String | Serving configuration for Google Cloud Endpoints (https://cloud.google.com/endpoints).Only returned in GET requests if view=FULL is set. |
| `endpoints_api_service` | String | Cloud Endpoints configuration.If endpoints_api_service is set, the Cloud Endpoints Extensible Service Proxy will be provided to serve the API implemented by the app. |
| `flexible_runtime_settings` | String | Settings for App Engine flexible runtimes. |
| `runtime_api_version` | String | The version of the API in the given runtime environment. Please see the app.yaml reference for valid values at https://cloud.google.com/appengine/docs/standard//config/appref |
| `threadsafe` | bool | Whether multiple requests can be dispatched to this version at once. |
| `liveness_check` | String | Configures liveness health checking for instances. Unhealthy instances are stopped and replaced with new instances |
| `libraries` | Vec<String> | Configuration for third-party Python runtime libraries that are required by the application.Only returned in GET requests if view=FULL is set. |
| `default_expiration` | String | Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding StaticFilesHandler (https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1beta/apps.services.versions#StaticFilesHandler) does not specify its own expiration time.Only returned in GET requests if view=FULL is set. |
| `manual_scaling` | String | A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. Manually scaled versions are sometimes referred to as "backends". |
| `resources` | String | Machine resources for this version. Only applicable in the App Engine flexible environment. |
| `handlers` | Vec<String> | An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted.Only returned in GET requests if view=FULL is set. |
| `health_check` | String | Configures health checking for instances. Unhealthy instances are stopped and replaced with new instances. Only applicable in the App Engine flexible environment. |
| `id` | String | Relative name of the version within the service. Example: v1. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-". |
| `build_env_variables` | HashMap<String, String> | Environment variables available to the build environment.Only returned in GET requests if view=FULL is set. |
| `error_handlers` | Vec<String> | Custom static error pages. Limited to 10KB per page.Only returned in GET requests if view=FULL is set. |
| `deployment` | String | Code and application artifacts that make up this version.Only returned in GET requests if view=FULL is set. |
| `runtime_channel` | String | The channel of the runtime to use. Only available for some runtimes. Defaults to the default channel. |
| `service_account` | String | The identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default if this field is neither provided in app.yaml file nor through CLI flag. |
| `network` | String | Extra network settings. Only applicable in the App Engine flexible environment. |
| `disk_usage_bytes` | String | Output only. Total size in bytes of all the files that are included in this version and currently hosted on the App Engine disk.@OutputOnly |
| `beta_settings` | HashMap<String, String> | Metadata settings that are supplied to this version to enable beta runtime features. |
| `vpc_access_connector` | String | Enables VPC connectivity for standard apps. |
| `vpc_egress` | String | Enables VPC egress connectivity for standard apps. |
| `generated_customer_metadata` | HashMap<String, String> | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetVersionRequest |
| `env` | String | App Engine execution environment for this version.Defaults to standard. |
| `serving_status` | String | Current serving status of this version. Only the versions with a SERVING status create instances and can be billed.SERVING_STATUS_UNSPECIFIED is an invalid value. Defaults to SERVING. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.appengine_api.Version {
    services_id = "value"  # Part of `parent`. See documentation of `appsId`.
    apps_id = "value"  # Part of `parent`. Required. Name of the parent resource to create this version under. Example: apps/myapp/services/default.
}

# Access version outputs
version_id = version.id
version_entrypoint = version.entrypoint
version_instance_class = version.instance_class
version_runtime_main_executable_path = version.runtime_main_executable_path
version_version_url = version.version_url
version_runtime = version.runtime
version_readiness_check = version.readiness_check
version_basic_scaling = version.basic_scaling
version_vm = version.vm
version_zones = version.zones
version_inbound_services = version.inbound_services
version_nobuild_files_regex = version.nobuild_files_regex
version_created_by = version.created_by
version_env_variables = version.env_variables
version_create_time = version.create_time
version_automatic_scaling = version.automatic_scaling
version_app_engine_apis = version.app_engine_apis
version_name = version.name
version_api_config = version.api_config
version_endpoints_api_service = version.endpoints_api_service
version_flexible_runtime_settings = version.flexible_runtime_settings
version_runtime_api_version = version.runtime_api_version
version_threadsafe = version.threadsafe
version_liveness_check = version.liveness_check
version_libraries = version.libraries
version_default_expiration = version.default_expiration
version_manual_scaling = version.manual_scaling
version_resources = version.resources
version_handlers = version.handlers
version_health_check = version.health_check
version_id = version.id
version_build_env_variables = version.build_env_variables
version_error_handlers = version.error_handlers
version_deployment = version.deployment
version_runtime_channel = version.runtime_channel
version_service_account = version.service_account
version_network = version.network
version_disk_usage_bytes = version.disk_usage_bytes
version_beta_settings = version.beta_settings
version_vpc_access_connector = version.vpc_access_connector
version_vpc_egress = version.vpc_egress
version_generated_customer_metadata = version.generated_customer_metadata
version_env = version.env
version_serving_status = version.serving_status
```

---


### Ingress_rule

Creates a firewall rule for the application.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `source_range` | String |  | IP address or range, defined using CIDR notation, of requests that this rule applies to. You can use the wildcard character "*" to match all IPs equivalent to "0/0" and "::/0" together. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. Truncation will be silently performed on addresses which are not properly truncated. For example, 1.2.3.4/24 is accepted as the same address as 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 is accepted as the same address as 2001:db8::/32. |
| `description` | String |  | An optional string description of this rule. This field has a maximum length of 400 characters. |
| `action` | String |  | The action to take on matched requests. |
| `priority` | i64 |  |  |
| `apps_id` | String | ✅ | Part of `parent`. Required. Name of the parent Firewall collection in which to create a new rule. Example: apps/myapp/firewall/ingressRules. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_range` | String | IP address or range, defined using CIDR notation, of requests that this rule applies to. You can use the wildcard character "*" to match all IPs equivalent to "0/0" and "::/0" together. Examples: 192.168.1.1 or 192.168.0.0/16 or 2001:db8::/32 or 2001:0db8:0000:0042:0000:8a2e:0370:7334. Truncation will be silently performed on addresses which are not properly truncated. For example, 1.2.3.4/24 is accepted as the same address as 1.2.3.0/24. Similarly, for IPv6, 2001:db8::1/32 is accepted as the same address as 2001:db8::/32. |
| `description` | String | An optional string description of this rule. This field has a maximum length of 400 characters. |
| `action` | String | The action to take on matched requests. |
| `priority` | i64 |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ingress_rule
ingress_rule = provider.appengine_api.Ingress_rule {
    apps_id = "value"  # Part of `parent`. Required. Name of the parent Firewall collection in which to create a new rule. Example: apps/myapp/firewall/ingressRules.
}

# Access ingress_rule outputs
ingress_rule_id = ingress_rule.id
ingress_rule_source_range = ingress_rule.source_range
ingress_rule_description = ingress_rule.description
ingress_rule_action = ingress_rule.action
ingress_rule_priority = ingress_rule.priority
```

---


### Instance

Enables debugging on a VM instance. This allows you to use the SSH command to connect to the virtual machine where the instance lives. While in "debug mode", the instance continues to serve live traffic. You should delete the instance when you are done debugging and then allow the system to take over and determine if another instance should be started.Only applicable for instances in App Engine flexible environment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ssh_key` | String |  | Public SSH key to add to the instance. Examples: [USERNAME]:ssh-rsa [KEY_VALUE] [USERNAME] [USERNAME]:ssh-rsa [KEY_VALUE] google-ssh {"userName":"[USERNAME]","expireOn":"[EXPIRE_TIME]"}For more information, see Adding and Removing SSH Keys (https://cloud.google.com/compute/docs/instances/adding-removing-ssh-keys). |
| `versions_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `services_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `apps_id` | String | ✅ | Part of `name`. Required. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1. |
| `instances_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Full path to the Instance resource in the API. Example: apps/myapp/services/default/versions/v1/instances/instance-1. |
| `id` | String | Output only. Relative name of the instance within the version. Example: instance-1. |
| `app_engine_release` | String | Output only. App Engine release this instance is running on. |
| `start_time` | String | Output only. Time that this instance was started.@OutputOnly |
| `memory_usage` | String | Output only. Total memory in use (bytes). |
| `vm_debug_enabled` | bool | Output only. Whether this instance is in debug mode. Only applicable for instances in App Engine flexible environment. |
| `vm_id` | String | Output only. Virtual machine ID of this instance. Only applicable for instances in App Engine flexible environment. |
| `availability` | String | Output only. Availability of the instance. |
| `errors` | i64 | Output only. Number of errors since this instance was started. |
| `vm_liveness` | String | Output only. The liveness health check of this instance. Only applicable for instances in App Engine flexible environment. |
| `requests` | i64 | Output only. Number of requests since this instance was started. |
| `average_latency` | i64 | Output only. Average latency (ms) over the last minute. |
| `vm_zone_name` | String | Output only. Zone where the virtual machine is located. Only applicable for instances in App Engine flexible environment. |
| `qps` | f64 | Output only. Average queries per second (QPS) over the last minute. |
| `vm_name` | String | Output only. Name of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment. |
| `vm_status` | String | Output only. Status of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment. |
| `vm_ip` | String | Output only. The IP address of this instance. Only applicable for instances in App Engine flexible environment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.appengine_api.Instance {
    versions_id = "value"  # Part of `name`. See documentation of `appsId`.
    services_id = "value"  # Part of `name`. See documentation of `appsId`.
    apps_id = "value"  # Part of `name`. Required. Name of the resource requested. Example: apps/myapp/services/default/versions/v1/instances/instance-1.
    instances_id = "value"  # Part of `name`. See documentation of `appsId`.
}

# Access instance outputs
instance_id = instance.id
instance_name = instance.name
instance_id = instance.id
instance_app_engine_release = instance.app_engine_release
instance_start_time = instance.start_time
instance_memory_usage = instance.memory_usage
instance_vm_debug_enabled = instance.vm_debug_enabled
instance_vm_id = instance.vm_id
instance_availability = instance.availability
instance_errors = instance.errors
instance_vm_liveness = instance.vm_liveness
instance_requests = instance.requests
instance_average_latency = instance.average_latency
instance_vm_zone_name = instance.vm_zone_name
instance_qps = instance.qps
instance_vm_name = instance.vm_name
instance_vm_status = instance.vm_status
instance_vm_ip = instance.vm_ip
```

---


### App

Creates an App Engine application for a Google Cloud Platform project. Required fields: id - The ID of the target Cloud Platform project. location - The region (https://cloud.google.com/appengine/docs/locations) where you want the App Engine application located.For more information about App Engine applications, see Managing Projects, Applications, and Billing (https://cloud.google.com/appengine/docs/standard/python/console/).

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_bucket` | String |  | Output only. Google Cloud Storage bucket that can be used by this application to store content.@OutputOnly |
| `default_hostname` | String |  | Output only. Hostname used to reach this application, as resolved by App Engine.@OutputOnly |
| `auth_domain` | String |  | Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account. |
| `gcr_domain` | String |  | Output only. The Google Container Registry domain used for storing managed build docker images for this application. |
| `generated_customer_metadata` | HashMap<String, String> |  | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetApplicationRequest |
| `iap` | String |  |  |
| `dispatch_rules` | Vec<String> |  | HTTP path dispatch rules for requests to the application that do not explicitly target a service or version. Rules are order-dependent. Up to 20 dispatch rules can be supported. |
| `name` | String |  |  |
| `default_cookie_expiration` | String |  | Cookie expiration policy for this application. |
| `id` | String |  | Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp. |
| `service_account` | String |  | The service account associated with the application. This is the app-level default identity. If no identity provided during create version, Admin API will fallback to this one. |
| `feature_settings` | String |  | The feature specific settings to be used in the application. |
| `serving_status` | String |  | Serving status of this application. |
| `code_bucket` | String |  | Output only. Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly |
| `ssl_policy` | String |  | The SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS < 1.2 and allow only Modern Ciphers suite |
| `location_id` | String |  | Location from which this application runs. Application instances run out of the data centers in the specified location, which is also where all of the application's end user content is stored.Defaults to us-central.View the list of supported locations (https://cloud.google.com/appengine/docs/locations). |
| `database_type` | String |  | The type of the Cloud Firestore or Cloud Datastore database associated with this application. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_bucket` | String | Output only. Google Cloud Storage bucket that can be used by this application to store content.@OutputOnly |
| `default_hostname` | String | Output only. Hostname used to reach this application, as resolved by App Engine.@OutputOnly |
| `auth_domain` | String | Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account. |
| `gcr_domain` | String | Output only. The Google Container Registry domain used for storing managed build docker images for this application. |
| `generated_customer_metadata` | HashMap<String, String> | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetApplicationRequest |
| `iap` | String |  |
| `dispatch_rules` | Vec<String> | HTTP path dispatch rules for requests to the application that do not explicitly target a service or version. Rules are order-dependent. Up to 20 dispatch rules can be supported. |
| `name` | String |  |
| `default_cookie_expiration` | String | Cookie expiration policy for this application. |
| `id` | String | Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp. |
| `service_account` | String | The service account associated with the application. This is the app-level default identity. If no identity provided during create version, Admin API will fallback to this one. |
| `feature_settings` | String | The feature specific settings to be used in the application. |
| `serving_status` | String | Serving status of this application. |
| `code_bucket` | String | Output only. Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly |
| `ssl_policy` | String | The SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS < 1.2 and allow only Modern Ciphers suite |
| `location_id` | String | Location from which this application runs. Application instances run out of the data centers in the specified location, which is also where all of the application's end user content is stored.Defaults to us-central.View the list of supported locations (https://cloud.google.com/appengine/docs/locations). |
| `database_type` | String | The type of the Cloud Firestore or Cloud Datastore database associated with this application. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app
app = provider.appengine_api.App {
}

# Access app outputs
app_id = app.id
app_default_bucket = app.default_bucket
app_default_hostname = app.default_hostname
app_auth_domain = app.auth_domain
app_gcr_domain = app.gcr_domain
app_generated_customer_metadata = app.generated_customer_metadata
app_iap = app.iap
app_dispatch_rules = app.dispatch_rules
app_name = app.name
app_default_cookie_expiration = app.default_cookie_expiration
app_id = app.id
app_service_account = app.service_account
app_feature_settings = app.feature_settings
app_serving_status = app.serving_status
app_code_bucket = app.code_bucket
app_ssl_policy = app.ssl_policy
app_location_id = app.location_id
app_database_type = app.database_type
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}  |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: "projects/example-project/locations/us-east1" |
| `location_id` | String | The canonical id for this location. For example: "us-east1". |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |


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
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
location_display_name = location.display_name
```

---


### Authorized_certificate

Uploads the specified SSL certificate.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_names` | Vec<String> |  | Topmost applicable domains of this certificate. This certificate applies to these domains and their subdomains. Example: example.com.@OutputOnly |
| `certificate_raw_data` | String |  | The SSL certificate serving the AuthorizedCertificate resource. This must be obtained independently from a certificate authority. |
| `domain_mappings_count` | i64 |  | Aggregate count of the domain mappings with this certificate mapped. This count includes domain mappings on applications for which the user does not have VIEWER permissions.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `managed_certificate` | String |  | Only applicable if this certificate is managed by App Engine. Managed certificates are tied to the lifecycle of a DomainMapping and cannot be updated or deleted via the AuthorizedCertificates API. If this certificate is manually administered by the user, this field will be empty.@OutputOnly |
| `visible_domain_mappings` | Vec<String> |  | The full paths to user visible Domain Mapping resources that have this certificate mapped. Example: apps/myapp/domainMappings/example.com.This may not represent the full list of mapped domain mappings if the user does not have VIEWER permissions on all of the applications that have this certificate mapped. See domain_mappings_count for a complete count.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `display_name` | String |  | The user-specified display name of the certificate. This is not guaranteed to be unique. Example: My Certificate. |
| `id` | String |  | Relative name of the certificate. This is a unique value autogenerated on AuthorizedCertificate resource creation. Example: 12345.@OutputOnly |
| `expire_time` | String |  | The time when this certificate expires. To update the renewal time on this certificate, upload an SSL certificate with a different expiration time using AuthorizedCertificates.UpdateAuthorizedCertificate.@OutputOnly |
| `name` | String |  | Full path to the AuthorizedCertificate resource in the API. Example: apps/myapp/authorizedCertificates/12345.@OutputOnly |
| `applications_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |
| `locations_id` | String | ✅ | Part of `parent`. See documentation of `projectsId`. |
| `projects_id` | String | ✅ | Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_names` | Vec<String> | Topmost applicable domains of this certificate. This certificate applies to these domains and their subdomains. Example: example.com.@OutputOnly |
| `certificate_raw_data` | String | The SSL certificate serving the AuthorizedCertificate resource. This must be obtained independently from a certificate authority. |
| `domain_mappings_count` | i64 | Aggregate count of the domain mappings with this certificate mapped. This count includes domain mappings on applications for which the user does not have VIEWER permissions.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `managed_certificate` | String | Only applicable if this certificate is managed by App Engine. Managed certificates are tied to the lifecycle of a DomainMapping and cannot be updated or deleted via the AuthorizedCertificates API. If this certificate is manually administered by the user, this field will be empty.@OutputOnly |
| `visible_domain_mappings` | Vec<String> | The full paths to user visible Domain Mapping resources that have this certificate mapped. Example: apps/myapp/domainMappings/example.com.This may not represent the full list of mapped domain mappings if the user does not have VIEWER permissions on all of the applications that have this certificate mapped. See domain_mappings_count for a complete count.Only returned by GET or LIST requests when specifically requested by the view=FULL_CERTIFICATE option.@OutputOnly |
| `display_name` | String | The user-specified display name of the certificate. This is not guaranteed to be unique. Example: My Certificate. |
| `id` | String | Relative name of the certificate. This is a unique value autogenerated on AuthorizedCertificate resource creation. Example: 12345.@OutputOnly |
| `expire_time` | String | The time when this certificate expires. To update the renewal time on this certificate, upload an SSL certificate with a different expiration time using AuthorizedCertificates.UpdateAuthorizedCertificate.@OutputOnly |
| `name` | String | Full path to the AuthorizedCertificate resource in the API. Example: apps/myapp/authorizedCertificates/12345.@OutputOnly |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorized_certificate
authorized_certificate = provider.appengine_api.Authorized_certificate {
    applications_id = "value"  # Part of `parent`. See documentation of `projectsId`.
    locations_id = "value"  # Part of `parent`. See documentation of `projectsId`.
    projects_id = "value"  # Part of `parent`. Required. Name of the parent Application resource. Example: apps/myapp.
}

# Access authorized_certificate outputs
authorized_certificate_id = authorized_certificate.id
authorized_certificate_domain_names = authorized_certificate.domain_names
authorized_certificate_certificate_raw_data = authorized_certificate.certificate_raw_data
authorized_certificate_domain_mappings_count = authorized_certificate.domain_mappings_count
authorized_certificate_managed_certificate = authorized_certificate.managed_certificate
authorized_certificate_visible_domain_mappings = authorized_certificate.visible_domain_mappings
authorized_certificate_display_name = authorized_certificate.display_name
authorized_certificate_id = authorized_certificate.id
authorized_certificate_expire_time = authorized_certificate.expire_time
authorized_certificate_name = authorized_certificate.name
```

---


### Service

Gets the current configuration of the specified service.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `generated_customer_metadata` | HashMap<String, String> |  | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetServiceRequest |
| `labels` | HashMap<String, String> |  | A set of labels to apply to this service. Labels are key/value pairs that describe the service and all resources that belong to it (e.g., versions). The labels can be used to search and group resources, and are propagated to the usage and billing reports, enabling fine-grain analysis of costs. An example of using labels is to tag resources belonging to different environments (e.g., "env=prod", "env=qa"). Label keys and values can be no longer than 63 characters and can only contain lowercase letters, numeric characters, underscores, dashes, and international characters. Label keys must start with a lowercase letter or an international character. Each service can have at most 32 labels. |
| `id` | String |  | Output only. Relative name of the service within the application. Example: default.@OutputOnly |
| `name` | String |  | Output only. Full path to the Service resource in the API. Example: apps/myapp/services/default.@OutputOnly |
| `network_settings` | String |  | Ingress settings for this service. Will apply to all versions. |
| `split` | String |  | Mapping that defines fractional HTTP traffic diversion to different versions within the service. |
| `applications_id` | String | ✅ | Part of `name`. See documentation of `projectsId`. |
| `projects_id` | String | ✅ | Part of `name`. Required. Name of the resource to update. Example: apps/myapp/services/default. |
| `locations_id` | String | ✅ | Part of `name`. See documentation of `projectsId`. |
| `services_id` | String | ✅ | Part of `name`. See documentation of `projectsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `generated_customer_metadata` | HashMap<String, String> | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetServiceRequest |
| `labels` | HashMap<String, String> | A set of labels to apply to this service. Labels are key/value pairs that describe the service and all resources that belong to it (e.g., versions). The labels can be used to search and group resources, and are propagated to the usage and billing reports, enabling fine-grain analysis of costs. An example of using labels is to tag resources belonging to different environments (e.g., "env=prod", "env=qa"). Label keys and values can be no longer than 63 characters and can only contain lowercase letters, numeric characters, underscores, dashes, and international characters. Label keys must start with a lowercase letter or an international character. Each service can have at most 32 labels. |
| `id` | String | Output only. Relative name of the service within the application. Example: default.@OutputOnly |
| `name` | String | Output only. Full path to the Service resource in the API. Example: apps/myapp/services/default.@OutputOnly |
| `network_settings` | String | Ingress settings for this service. Will apply to all versions. |
| `split` | String | Mapping that defines fractional HTTP traffic diversion to different versions within the service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access service outputs
service_id = service.id
service_generated_customer_metadata = service.generated_customer_metadata
service_labels = service.labels
service_id = service.id
service_name = service.name
service_network_settings = service.network_settings
service_split = service.split
```

---


### Application



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_bucket` | String |  | Output only. Google Cloud Storage bucket that can be used by this application to store content.@OutputOnly |
| `default_hostname` | String |  | Output only. Hostname used to reach this application, as resolved by App Engine.@OutputOnly |
| `auth_domain` | String |  | Google Apps authentication domain that controls which users can access this application.Defaults to open access for any Google Account. |
| `gcr_domain` | String |  | Output only. The Google Container Registry domain used for storing managed build docker images for this application. |
| `generated_customer_metadata` | HashMap<String, String> |  | Additional Google Generated Customer Metadata, this field won't be provided by default and can be requested by setting the IncludeExtraData field in GetApplicationRequest |
| `iap` | String |  |  |
| `dispatch_rules` | Vec<String> |  | HTTP path dispatch rules for requests to the application that do not explicitly target a service or version. Rules are order-dependent. Up to 20 dispatch rules can be supported. |
| `name` | String |  |  |
| `default_cookie_expiration` | String |  | Cookie expiration policy for this application. |
| `id` | String |  | Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: myapp. |
| `service_account` | String |  | The service account associated with the application. This is the app-level default identity. If no identity provided during create version, Admin API will fallback to this one. |
| `feature_settings` | String |  | The feature specific settings to be used in the application. |
| `serving_status` | String |  | Serving status of this application. |
| `code_bucket` | String |  | Output only. Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands.@OutputOnly |
| `ssl_policy` | String |  | The SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS < 1.2 and allow only Modern Ciphers suite |
| `location_id` | String |  | Location from which this application runs. Application instances run out of the data centers in the specified location, which is also where all of the application's end user content is stored.Defaults to us-central.View the list of supported locations (https://cloud.google.com/appengine/docs/locations). |
| `database_type` | String |  | The type of the Cloud Firestore or Cloud Datastore database associated with this application. |
| `locations_id` | String | ✅ | Part of `name`. See documentation of `projectsId`. |
| `projects_id` | String | ✅ | Part of `name`. Required. Name of the Application resource to update. Example: apps/myapp. |
| `applications_id` | String | ✅ | Part of `name`. See documentation of `projectsId`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple instance resources
instance_0 = provider.appengine_api.Instance {
    versions_id = "value-0"
    modules_id = "value-0"
    instances_id = "value-0"
    apps_id = "value-0"
}
instance_1 = provider.appengine_api.Instance {
    versions_id = "value-1"
    modules_id = "value-1"
    instances_id = "value-1"
    apps_id = "value-1"
}
instance_2 = provider.appengine_api.Instance {
    versions_id = "value-2"
    modules_id = "value-2"
    instances_id = "value-2"
    apps_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    instance = provider.appengine_api.Instance {
        versions_id = "production-value"
        modules_id = "production-value"
        instances_id = "production-value"
        apps_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Appengine_api Documentation](https://cloud.google.com/appengine_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
