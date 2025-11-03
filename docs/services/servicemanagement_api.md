# Servicemanagement_api Service



**Resources**: 5

---

## Overview

The servicemanagement_api service provides access to 5 resource types:

- [Operation](#operation) [R]
- [Consumer](#consumer) [C]
- [Config](#config) [CR]
- [Service](#service) [CRD]
- [Rollout](#rollout) [CR]

---

## Resources


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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Consumer

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create consumer
consumer = provider.servicemanagement_api.Consumer {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

```

---


### Config

Creates a new service configuration (version) for a managed service. This method only stores the service configuration. To roll out the service configuration to backend systems please call CreateServiceRollout. Only the 100 most recent service configurations and ones referenced by existing rollouts are kept for each service. The rest will be deleted eventually.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String |  | The product title for this service, it is the name displayed in Google Cloud Console. |
| `usage` | String |  | Configuration controlling usage of this service. |
| `quota` | String |  | Quota configuration. |
| `config_version` | i64 |  | Obsolete. Do not use. This field has no semantic meaning. The service config compiler always sets this field to `3`. |
| `logging` | String |  | Logging configuration. |
| `monitored_resources` | Vec<String> |  | Defines the monitored resources used by this service. This is required by the Service.monitoring and Service.logging configurations. |
| `apis` | Vec<String> |  | A list of API interfaces exported by this service. Only the `name` field of the google.protobuf.Api needs to be provided by the configuration author, as the remaining fields will be derived from the IDL during the normalization process. It is an error to specify an API interface here which cannot be resolved against the associated IDL files. |
| `documentation` | String |  | Additional API documentation. |
| `billing` | String |  | Billing configuration. |
| `system_parameters` | String |  | System parameter configuration. |
| `monitoring` | String |  | Monitoring configuration. |
| `logs` | Vec<String> |  | Defines the logs used by this service. |
| `http` | String |  | HTTP configuration. |
| `aspects` | Vec<String> |  | Configuration aspects. This is a repeated field to allow multiple aspects to be configured. The kind field in each ConfigAspect specifies the type of aspect. The spec field contains the configuration for that aspect. The schema for the spec field is defined by the backend service owners. |
| `control` | String |  | Configuration for the service control plane. |
| `backend` | String |  | API backend configuration. |
| `types` | Vec<String> |  | A list of all proto message types included in this API service. Types referenced directly or indirectly by the `apis` are automatically included. Messages which are not referenced but shall be included, such as types used by the `google.protobuf.Any` type, should be listed here by name by the configuration author. Example: types: - name: google.protobuf.Int32 |
| `name` | String |  | The service name, which is a DNS-like logical identifier for the service, such as `calendar.googleapis.com`. The service name typically goes through DNS verification to make sure the owner of the service also owns the DNS name. |
| `publishing` | String |  | Settings for [Google Cloud Client libraries](https://cloud.google.com/apis/docs/cloud-client-libraries) generated from APIs defined as protocol buffers. |
| `endpoints` | Vec<String> |  | Configuration for network endpoints. If this is empty, then an endpoint with the same name as the service is automatically generated to service all defined APIs. |
| `producer_project_id` | String |  | The Google project that owns this service. |
| `system_types` | Vec<String> |  | A list of all proto message types included in this API service. It serves similar purpose as [google.api.Service.types], except that these types are not needed by user-defined APIs. Therefore, they will not show up in the generated discovery doc. This field should only be used to define system APIs in ESF. |
| `context` | String |  | Context configuration. |
| `authentication` | String |  | Auth configuration. |
| `custom_error` | String |  | Custom error configuration. |
| `enums` | Vec<String> |  | A list of all enum types included in this API service. Enums referenced directly or indirectly by the `apis` are automatically included. Enums which are not referenced but shall be included should be listed here by name by the configuration author. Example: enums: - name: google.someapi.v1.SomeEnum |
| `metrics` | Vec<String> |  | Defines the metrics used by this service. |
| `source_info` | String |  | Output only. The source information for this configuration if available. |
| `id` | String |  | A unique ID for a specific instance of this message, typically assigned by the client for tracking purpose. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If empty, the server may choose to generate one instead. |
| `service_name` | String | ✅ | Required. The name of the service. See the [overview](https://cloud.google.com/service-management/overview) for naming requirements. For example: `example.googleapis.com`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `title` | String | The product title for this service, it is the name displayed in Google Cloud Console. |
| `usage` | String | Configuration controlling usage of this service. |
| `quota` | String | Quota configuration. |
| `config_version` | i64 | Obsolete. Do not use. This field has no semantic meaning. The service config compiler always sets this field to `3`. |
| `logging` | String | Logging configuration. |
| `monitored_resources` | Vec<String> | Defines the monitored resources used by this service. This is required by the Service.monitoring and Service.logging configurations. |
| `apis` | Vec<String> | A list of API interfaces exported by this service. Only the `name` field of the google.protobuf.Api needs to be provided by the configuration author, as the remaining fields will be derived from the IDL during the normalization process. It is an error to specify an API interface here which cannot be resolved against the associated IDL files. |
| `documentation` | String | Additional API documentation. |
| `billing` | String | Billing configuration. |
| `system_parameters` | String | System parameter configuration. |
| `monitoring` | String | Monitoring configuration. |
| `logs` | Vec<String> | Defines the logs used by this service. |
| `http` | String | HTTP configuration. |
| `aspects` | Vec<String> | Configuration aspects. This is a repeated field to allow multiple aspects to be configured. The kind field in each ConfigAspect specifies the type of aspect. The spec field contains the configuration for that aspect. The schema for the spec field is defined by the backend service owners. |
| `control` | String | Configuration for the service control plane. |
| `backend` | String | API backend configuration. |
| `types` | Vec<String> | A list of all proto message types included in this API service. Types referenced directly or indirectly by the `apis` are automatically included. Messages which are not referenced but shall be included, such as types used by the `google.protobuf.Any` type, should be listed here by name by the configuration author. Example: types: - name: google.protobuf.Int32 |
| `name` | String | The service name, which is a DNS-like logical identifier for the service, such as `calendar.googleapis.com`. The service name typically goes through DNS verification to make sure the owner of the service also owns the DNS name. |
| `publishing` | String | Settings for [Google Cloud Client libraries](https://cloud.google.com/apis/docs/cloud-client-libraries) generated from APIs defined as protocol buffers. |
| `endpoints` | Vec<String> | Configuration for network endpoints. If this is empty, then an endpoint with the same name as the service is automatically generated to service all defined APIs. |
| `producer_project_id` | String | The Google project that owns this service. |
| `system_types` | Vec<String> | A list of all proto message types included in this API service. It serves similar purpose as [google.api.Service.types], except that these types are not needed by user-defined APIs. Therefore, they will not show up in the generated discovery doc. This field should only be used to define system APIs in ESF. |
| `context` | String | Context configuration. |
| `authentication` | String | Auth configuration. |
| `custom_error` | String | Custom error configuration. |
| `enums` | Vec<String> | A list of all enum types included in this API service. Enums referenced directly or indirectly by the `apis` are automatically included. Enums which are not referenced but shall be included should be listed here by name by the configuration author. Example: enums: - name: google.someapi.v1.SomeEnum |
| `metrics` | Vec<String> | Defines the metrics used by this service. |
| `source_info` | String | Output only. The source information for this configuration if available. |
| `id` | String | A unique ID for a specific instance of this message, typically assigned by the client for tracking purpose. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If empty, the server may choose to generate one instead. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create config
config = provider.servicemanagement_api.Config {
    service_name = "value"  # Required. The name of the service. See the [overview](https://cloud.google.com/service-management/overview) for naming requirements. For example: `example.googleapis.com`.
}

# Access config outputs
config_id = config.id
config_title = config.title
config_usage = config.usage
config_quota = config.quota
config_config_version = config.config_version
config_logging = config.logging
config_monitored_resources = config.monitored_resources
config_apis = config.apis
config_documentation = config.documentation
config_billing = config.billing
config_system_parameters = config.system_parameters
config_monitoring = config.monitoring
config_logs = config.logs
config_http = config.http
config_aspects = config.aspects
config_control = config.control
config_backend = config.backend
config_types = config.types
config_name = config.name
config_publishing = config.publishing
config_endpoints = config.endpoints
config_producer_project_id = config.producer_project_id
config_system_types = config.system_types
config_context = config.context
config_authentication = config.authentication
config_custom_error = config.custom_error
config_enums = config.enums
config_metrics = config.metrics
config_source_info = config.source_info
config_id = config.id
```

---


### Service

Creates a new managed service. A managed service is immutable, and is subject to mandatory 30-day data retention. You cannot move a service or recreate it within 30 days after deletion. One producer project can own no more than 500 services. For security and reliability purposes, a production service should be hosted in a dedicated producer project. Operation

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_name` | String |  | The name of the service. See the [overview](https://cloud.google.com/service-infrastructure/docs/overview) for naming requirements. |
| `producer_project_id` | String |  | ID of the project that produces and owns this service. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_name` | String | The name of the service. See the [overview](https://cloud.google.com/service-infrastructure/docs/overview) for naming requirements. |
| `producer_project_id` | String | ID of the project that produces and owns this service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.servicemanagement_api.Service {
}

# Access service outputs
service_id = service.id
service_service_name = service.service_name
service_producer_project_id = service.producer_project_id
```

---


### Rollout

Creates a new service configuration rollout. Based on rollout, the Google Service Management will roll out the service configurations to different backend services. For example, the logging configuration will be pushed to Google Cloud Logging. Please note that any previous pending and running Rollouts and associated Operations will be automatically cancelled so that the latest Rollout will not be blocked by previous Rollouts. Only the 100 most recent (in any state) and the last 10 successful (if not already part of the set of 100 most recent) rollouts are kept for each service. The rest will be deleted eventually. Operation

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `created_by` | String |  | The user who created the Rollout. Readonly. |
| `delete_service_strategy` | String |  | The strategy associated with a rollout to delete a `ManagedService`. Readonly. |
| `service_name` | String |  | The name of the service associated with this Rollout. |
| `create_time` | String |  | Creation time of the rollout. Readonly. |
| `rollout_id` | String |  | Optional. Unique identifier of this Rollout. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If not specified by client, the server will generate one. The generated id will have the form of , where "date" is the create date in ISO 8601 format. "revision number" is a monotonically increasing positive number that is reset every day for each service. An example of the generated rollout_id is '2016-02-16r1' |
| `status` | String |  | The status of this rollout. Readonly. In case of a failed rollout, the system will automatically rollback to the current Rollout version. Readonly. |
| `traffic_percent_strategy` | String |  | Google Service Control selects service configurations based on traffic percentage. |
| `service_name` | String | ✅ | Required. The name of the service. See the [overview](https://cloud.google.com/service-management/overview) for naming requirements. For example: `example.googleapis.com`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_by` | String | The user who created the Rollout. Readonly. |
| `delete_service_strategy` | String | The strategy associated with a rollout to delete a `ManagedService`. Readonly. |
| `service_name` | String | The name of the service associated with this Rollout. |
| `create_time` | String | Creation time of the rollout. Readonly. |
| `rollout_id` | String | Optional. Unique identifier of this Rollout. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If not specified by client, the server will generate one. The generated id will have the form of , where "date" is the create date in ISO 8601 format. "revision number" is a monotonically increasing positive number that is reset every day for each service. An example of the generated rollout_id is '2016-02-16r1' |
| `status` | String | The status of this rollout. Readonly. In case of a failed rollout, the system will automatically rollback to the current Rollout version. Readonly. |
| `traffic_percent_strategy` | String | Google Service Control selects service configurations based on traffic percentage. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rollout
rollout = provider.servicemanagement_api.Rollout {
    service_name = "value"  # Required. The name of the service. See the [overview](https://cloud.google.com/service-management/overview) for naming requirements. For example: `example.googleapis.com`.
}

# Access rollout outputs
rollout_id = rollout.id
rollout_created_by = rollout.created_by
rollout_delete_service_strategy = rollout.delete_service_strategy
rollout_service_name = rollout.service_name
rollout_create_time = rollout.create_time
rollout_rollout_id = rollout.rollout_id
rollout_status = rollout.status
rollout_traffic_percent_strategy = rollout.traffic_percent_strategy
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
operation_0 = provider.servicemanagement_api.Operation {
}
operation_1 = provider.servicemanagement_api.Operation {
}
operation_2 = provider.servicemanagement_api.Operation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.servicemanagement_api.Operation {
    }
```

---

## Related Documentation

- [GCP Servicemanagement_api Documentation](https://cloud.google.com/servicemanagement_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
