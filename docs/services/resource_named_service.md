# Resource_named_service Service



**Resources**: 6

---

## Overview

The resource_named_service service provides access to 6 resource types:

- [App](#app) [CR]
- [Service](#service) [RUD]
- [Instance](#instance) [CRD]
- [Location](#location) [R]
- [Operation](#operation) [R]
- [Version](#version) [CRUD]

---

## Resources


### App

Recreates the required App Engine features for the application in your project, for example a Cloud Storage bucket or App Engine service account. Use this method if you receive an error message about a missing feature, for example "*Error retrieving the App Engine service account*".

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `apps_id` | String | ✅ | Part of `name`. Name of the application to repair. Example: `apps/myapp` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_hostname` | String | Hostname used to reach this application, as resolved by App Engine. @OutputOnly |
| `default_cookie_expiration` | String | Cookie expiration policy for this application. @OutputOnly |
| `auth_domain` | String | Google Apps authentication domain that controls which users can access this application. Defaults to open access for any Google Account. |
| `id` | String | Identifier of the Application resource. This identifier is equivalent to the project ID of the Google Cloud Platform project where you want to deploy your application. Example: `myapp`. |
| `dispatch_rules` | Vec<String> | HTTP path dispatch rules for requests to the application that do not explicitly target a service or version. Rules are order-dependent. @OutputOnly |
| `name` | String | Full path to the Application resource in the API. Example: `apps/myapp`. @OutputOnly |
| `code_bucket` | String | Google Cloud Storage bucket that can be used for storing files associated with this application. This bucket is associated with the application and can be used by the gcloud deployment commands. @OutputOnly |
| `default_bucket` | String | Google Cloud Storage bucket that can be used by this application to store content. @OutputOnly |
| `location_id` | String | Location from which this application will be run. Application instances will run out of data centers in the chosen location, which is also where all of the application's end user content is stored. Defaults to `us-central`. Options are: `us-central` - Central US `europe-west` - Western Europe `us-east1` - Eastern US |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app
app = provider.resource_named_service.App {
    apps_id = "value"  # Part of `name`. Name of the application to repair. Example: `apps/myapp`
}

# Access app outputs
app_id = app.id
app_default_hostname = app.default_hostname
app_default_cookie_expiration = app.default_cookie_expiration
app_auth_domain = app.auth_domain
app_id = app.id
app_dispatch_rules = app.dispatch_rules
app_name = app.name
app_code_bucket = app.code_bucket
app_default_bucket = app.default_bucket
app_location_id = app.location_id
```

---


### Service

Gets the current configuration of the specified service.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Relative name of the service within the application. Example: `default`. @OutputOnly |
| `name` | String |  | Full path to the Service resource in the API. Example: `apps/myapp/services/default`. @OutputOnly |
| `split` | String |  | Mapping that defines fractional HTTP traffic diversion to different versions within the service. |
| `services_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `apps_id` | String | ✅ | Part of `name`. Name of the resource to update. Example: `apps/myapp/services/default`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Relative name of the service within the application. Example: `default`. @OutputOnly |
| `name` | String | Full path to the Service resource in the API. Example: `apps/myapp/services/default`. @OutputOnly |
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
service_id = service.id
service_name = service.name
service_split = service.split
```

---


### Instance

Enables debugging on a VM instance. This allows you to use the SSH command to connect to the virtual machine where the instance lives. While in "debug mode", the instance continues to serve live traffic. You should delete the instance when you are done debugging and then allow the system to take over and determine if another instance should be started. Only applicable for instances in App Engine flexible environment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instances_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `services_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `versions_id` | String | ✅ | Part of `name`. See documentation of `appsId`. |
| `apps_id` | String | ✅ | Part of `name`. Name of the resource requested. Example: `apps/myapp/services/default/versions/v1/instances/instance-1`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `availability` | String | Availability of the instance. @OutputOnly |
| `average_latency` | i64 | Average latency (ms) over the last minute. @OutputOnly |
| `vm_name` | String | Name of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment. @OutputOnly |
| `vm_status` | String | Status of the virtual machine where this instance lives. Only applicable for instances in App Engine flexible environment. @OutputOnly |
| `start_time` | String | Time that this instance was started. @OutputOnly |
| `requests` | i64 | Number of requests since this instance was started. @OutputOnly |
| `name` | String | Full path to the Instance resource in the API. Example: `apps/myapp/services/default/versions/v1/instances/instance-1`. @OutputOnly |
| `app_engine_release` | String | App Engine release this instance is running on. @OutputOnly |
| `vm_zone_name` | String | Zone where the virtual machine is located. Only applicable for instances in App Engine flexible environment. @OutputOnly |
| `qps` | f64 | Average queries per second (QPS) over the last minute. @OutputOnly |
| `memory_usage` | String | Total memory in use (bytes). @OutputOnly |
| `vm_debug_enabled` | bool | Whether this instance is in debug mode. Only applicable for instances in App Engine flexible environment. @OutputOnly |
| `errors` | i64 | Number of errors since this instance was started. @OutputOnly |
| `vm_id` | String | Virtual machine ID of this instance. Only applicable for instances in App Engine flexible environment. @OutputOnly |
| `id` | String | Relative name of the instance within the version. Example: `instance-1`. @OutputOnly |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.resource_named_service.Instance {
    instances_id = "value"  # Part of `name`. See documentation of `appsId`.
    services_id = "value"  # Part of `name`. See documentation of `appsId`.
    versions_id = "value"  # Part of `name`. See documentation of `appsId`.
    apps_id = "value"  # Part of `name`. Name of the resource requested. Example: `apps/myapp/services/default/versions/v1/instances/instance-1`.
}

# Access instance outputs
instance_id = instance.id
instance_availability = instance.availability
instance_average_latency = instance.average_latency
instance_vm_name = instance.vm_name
instance_vm_status = instance.vm_status
instance_start_time = instance.start_time
instance_requests = instance.requests
instance_name = instance.name
instance_app_engine_release = instance.app_engine_release
instance_vm_zone_name = instance.vm_zone_name
instance_qps = instance.qps
instance_memory_usage = instance.memory_usage
instance_vm_debug_enabled = instance.vm_debug_enabled
instance_errors = instance.errors
instance_vm_id = instance.vm_id
instance_id = instance.id
```

---


### Location

Get information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The cononical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


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
location_location_id = location.location_id
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should have the format of `operations/some/unique/name`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If true, the operation is completed, and either `error` or `response` is available. |


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
operation_name = operation.name
operation_metadata = operation.metadata
operation_error = operation.error
operation_response = operation.response
operation_done = operation.done
```

---


### Version

Deploys code and resource files to a new version.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `env` | String |  | App Engine execution environment for this version. Defaults to `standard`. |
| `version_url` | String |  | Serving URL for this version. Example: "https://myversion-dot-myservice-dot-myapp.appspot.com" @OutputOnly |
| `created_by` | String |  | Email address of the user who created this version. @OutputOnly |
| `deployment` | String |  | Code and application artifacts that make up this version. Only returned in `GET` requests if `view=FULL` is set. |
| `id` | String |  | Relative name of the version within the service. Example: `v1`. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-". |
| `disk_usage_bytes` | String |  | Total size in bytes of all the files that are included in this version and curerntly hosted on the App Engine disk. @OutputOnly |
| `libraries` | Vec<String> |  | Configuration for third-party Python runtime libraries that are required by the application. Only returned in `GET` requests if `view=FULL` is set. |
| `basic_scaling` | String |  | A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity. |
| `vm` | bool |  | Whether to deploy this version in a container on a virtual machine. |
| `network` | String |  | Extra network settings. Only applicable for VM runtimes. |
| `error_handlers` | Vec<String> |  | Custom static error pages. Limited to 10KB per page. Only returned in `GET` requests if `view=FULL` is set. |
| `handlers` | Vec<String> |  | An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted. Only returned in `GET` requests if `view=FULL` is set. |
| `manual_scaling` | String |  | A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. |
| `threadsafe` | bool |  | Whether multiple requests can be dispatched to this version at once. |
| `api_config` | String |  | Serving configuration for [Google Cloud Endpoints](https://cloud.google.com/appengine/docs/python/endpoints/). Only returned in `GET` requests if `view=FULL` is set. |
| `env_variables` | HashMap<String, String> |  | Environment variables available to the application. Only returned in `GET` requests if `view=FULL` is set. |
| `health_check` | String |  | Configures health checking for VM instances. Unhealthy instances are stopped and replaced with new instances. Only applicable for VM runtimes. Only returned in `GET` requests if `view=FULL` is set. |
| `inbound_services` | Vec<String> |  | Before an application can receive email or XMPP messages, the application must be configured to enable the service. |
| `instance_class` | String |  | Instance class that is used to run this version. Valid values are: * AutomaticScaling: `F1`, `F2`, `F4`, `F4_1G` * ManualScaling or BasicScaling: `B1`, `B2`, `B4`, `B8`, `B4_1G` Defaults to `F1` for AutomaticScaling and `B1` for ManualScaling or BasicScaling. |
| `beta_settings` | HashMap<String, String> |  | Metadata settings that are supplied to this version to enable beta runtime features. |
| `serving_status` | String |  | Current serving status of this version. Only the versions with a `SERVING` status create instances and can be billed. `SERVING_STATUS_UNSPECIFIED` is an invalid value. Defaults to `SERVING`. |
| `create_time` | String |  | Time that this version was created. @OutputOnly |
| `automatic_scaling` | String |  | Automatic scaling is based on request rate, response latencies, and other application metrics. |
| `default_expiration` | String |  | Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding [StaticFilesHandler](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#staticfileshandler) does not specify its own expiration time. Only returned in `GET` requests if `view=FULL` is set. |
| `name` | String |  | Full path to the Version resource in the API. Example: `apps/myapp/services/default/versions/v1`. @OutputOnly |
| `resources` | String |  | Machine resources for this version. Only applicable for VM runtimes. |
| `runtime` | String |  | Desired runtime. Example: `python27`. |
| `nobuild_files_regex` | String |  | Files that match this pattern will not be built into this version. Only applicable for Go runtimes. Only returned in `GET` requests if `view=FULL` is set. |
| `apps_id` | String | ✅ | Part of `parent`. Name of the parent resource to create this version under. Example: `apps/myapp/services/default`. |
| `services_id` | String | ✅ | Part of `parent`. See documentation of `appsId`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `env` | String | App Engine execution environment for this version. Defaults to `standard`. |
| `version_url` | String | Serving URL for this version. Example: "https://myversion-dot-myservice-dot-myapp.appspot.com" @OutputOnly |
| `created_by` | String | Email address of the user who created this version. @OutputOnly |
| `deployment` | String | Code and application artifacts that make up this version. Only returned in `GET` requests if `view=FULL` is set. |
| `id` | String | Relative name of the version within the service. Example: `v1`. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names: "default", "latest", and any name with the prefix "ah-". |
| `disk_usage_bytes` | String | Total size in bytes of all the files that are included in this version and curerntly hosted on the App Engine disk. @OutputOnly |
| `libraries` | Vec<String> | Configuration for third-party Python runtime libraries that are required by the application. Only returned in `GET` requests if `view=FULL` is set. |
| `basic_scaling` | String | A service with basic scaling will create an instance when the application receives a request. The instance will be turned down when the app becomes idle. Basic scaling is ideal for work that is intermittent or driven by user activity. |
| `vm` | bool | Whether to deploy this version in a container on a virtual machine. |
| `network` | String | Extra network settings. Only applicable for VM runtimes. |
| `error_handlers` | Vec<String> | Custom static error pages. Limited to 10KB per page. Only returned in `GET` requests if `view=FULL` is set. |
| `handlers` | Vec<String> | An ordered list of URL-matching patterns that should be applied to incoming requests. The first matching URL handles the request and other request handlers are not attempted. Only returned in `GET` requests if `view=FULL` is set. |
| `manual_scaling` | String | A service with manual scaling runs continuously, allowing you to perform complex initialization and rely on the state of its memory over time. |
| `threadsafe` | bool | Whether multiple requests can be dispatched to this version at once. |
| `api_config` | String | Serving configuration for [Google Cloud Endpoints](https://cloud.google.com/appengine/docs/python/endpoints/). Only returned in `GET` requests if `view=FULL` is set. |
| `env_variables` | HashMap<String, String> | Environment variables available to the application. Only returned in `GET` requests if `view=FULL` is set. |
| `health_check` | String | Configures health checking for VM instances. Unhealthy instances are stopped and replaced with new instances. Only applicable for VM runtimes. Only returned in `GET` requests if `view=FULL` is set. |
| `inbound_services` | Vec<String> | Before an application can receive email or XMPP messages, the application must be configured to enable the service. |
| `instance_class` | String | Instance class that is used to run this version. Valid values are: * AutomaticScaling: `F1`, `F2`, `F4`, `F4_1G` * ManualScaling or BasicScaling: `B1`, `B2`, `B4`, `B8`, `B4_1G` Defaults to `F1` for AutomaticScaling and `B1` for ManualScaling or BasicScaling. |
| `beta_settings` | HashMap<String, String> | Metadata settings that are supplied to this version to enable beta runtime features. |
| `serving_status` | String | Current serving status of this version. Only the versions with a `SERVING` status create instances and can be billed. `SERVING_STATUS_UNSPECIFIED` is an invalid value. Defaults to `SERVING`. |
| `create_time` | String | Time that this version was created. @OutputOnly |
| `automatic_scaling` | String | Automatic scaling is based on request rate, response latencies, and other application metrics. |
| `default_expiration` | String | Duration that static files should be cached by web proxies and browsers. Only applicable if the corresponding [StaticFilesHandler](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#staticfileshandler) does not specify its own expiration time. Only returned in `GET` requests if `view=FULL` is set. |
| `name` | String | Full path to the Version resource in the API. Example: `apps/myapp/services/default/versions/v1`. @OutputOnly |
| `resources` | String | Machine resources for this version. Only applicable for VM runtimes. |
| `runtime` | String | Desired runtime. Example: `python27`. |
| `nobuild_files_regex` | String | Files that match this pattern will not be built into this version. Only applicable for Go runtimes. Only returned in `GET` requests if `view=FULL` is set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.resource_named_service.Version {
    apps_id = "value"  # Part of `parent`. Name of the parent resource to create this version under. Example: `apps/myapp/services/default`.
    services_id = "value"  # Part of `parent`. See documentation of `appsId`.
}

# Access version outputs
version_id = version.id
version_env = version.env
version_version_url = version.version_url
version_created_by = version.created_by
version_deployment = version.deployment
version_id = version.id
version_disk_usage_bytes = version.disk_usage_bytes
version_libraries = version.libraries
version_basic_scaling = version.basic_scaling
version_vm = version.vm
version_network = version.network
version_error_handlers = version.error_handlers
version_handlers = version.handlers
version_manual_scaling = version.manual_scaling
version_threadsafe = version.threadsafe
version_api_config = version.api_config
version_env_variables = version.env_variables
version_health_check = version.health_check
version_inbound_services = version.inbound_services
version_instance_class = version.instance_class
version_beta_settings = version.beta_settings
version_serving_status = version.serving_status
version_create_time = version.create_time
version_automatic_scaling = version.automatic_scaling
version_default_expiration = version.default_expiration
version_name = version.name
version_resources = version.resources
version_runtime = version.runtime
version_nobuild_files_regex = version.nobuild_files_regex
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple app resources
app_0 = provider.resource_named_service.App {
    apps_id = "value-0"
}
app_1 = provider.resource_named_service.App {
    apps_id = "value-1"
}
app_2 = provider.resource_named_service.App {
    apps_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    app = provider.resource_named_service.App {
        apps_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Resource_named_service Documentation](https://cloud.google.com/resource_named_service/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
