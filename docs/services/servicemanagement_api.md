# Servicemanagement_api Service



**Resources**: 5

---

## Overview

The servicemanagement_api service provides access to 5 resource types:

- [Rollout](#rollout) [CR]
- [Consumer](#consumer) [C]
- [Service](#service) [CRD]
- [Config](#config) [CR]
- [Operation](#operation) [R]

---

## Resources


### Rollout

Creates a new service configuration rollout. Based on rollout, the Google Service Management will roll out the service configurations to different backend services. For example, the logging configuration will be pushed to Google Cloud Logging. Please note that any previous pending and running Rollouts and associated Operations will be automatically cancelled so that the latest Rollout will not be blocked by previous Rollouts. Only the 100 most recent (in any state) and the last 10 successful (if not already part of the set of 100 most recent) rollouts are kept for each service. The rest will be deleted eventually. Operation

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rollout_id` | String |  | Optional. Unique identifier of this Rollout. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If not specified by client, the server will generate one. The generated id will have the form of , where "date" is the create date in ISO 8601 format. "revision number" is a monotonically increasing positive number that is reset every day for each service. An example of the generated rollout_id is '2016-02-16r1' |
| `created_by` | String |  | The user who created the Rollout. Readonly. |
| `traffic_percent_strategy` | String |  | Google Service Control selects service configurations based on traffic percentage. |
| `service_name` | String |  | The name of the service associated with this Rollout. |
| `create_time` | String |  | Creation time of the rollout. Readonly. |
| `status` | String |  | The status of this rollout. Readonly. In case of a failed rollout, the system will automatically rollback to the current Rollout version. Readonly. |
| `delete_service_strategy` | String |  | The strategy associated with a rollout to delete a `ManagedService`. Readonly. |
| `service_name` | String | ✅ | Required. The name of the service. See the [overview](https://cloud.google.com/service-management/overview) for naming requirements. For example: `example.googleapis.com`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rollout_id` | String | Optional. Unique identifier of this Rollout. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If not specified by client, the server will generate one. The generated id will have the form of , where "date" is the create date in ISO 8601 format. "revision number" is a monotonically increasing positive number that is reset every day for each service. An example of the generated rollout_id is '2016-02-16r1' |
| `created_by` | String | The user who created the Rollout. Readonly. |
| `traffic_percent_strategy` | String | Google Service Control selects service configurations based on traffic percentage. |
| `service_name` | String | The name of the service associated with this Rollout. |
| `create_time` | String | Creation time of the rollout. Readonly. |
| `status` | String | The status of this rollout. Readonly. In case of a failed rollout, the system will automatically rollback to the current Rollout version. Readonly. |
| `delete_service_strategy` | String | The strategy associated with a rollout to delete a `ManagedService`. Readonly. |


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
rollout_rollout_id = rollout.rollout_id
rollout_created_by = rollout.created_by
rollout_traffic_percent_strategy = rollout.traffic_percent_strategy
rollout_service_name = rollout.service_name
rollout_create_time = rollout.create_time
rollout_status = rollout.status
rollout_delete_service_strategy = rollout.delete_service_strategy
```

---


### Consumer

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |



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
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

```

---


### Service

Creates a new managed service. A managed service is immutable, and is subject to mandatory 30-day data retention. You cannot move a service or recreate it within 30 days after deletion. One producer project can own no more than 500 services. For security and reliability purposes, a production service should be hosted in a dedicated producer project. Operation

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `producer_project_id` | String |  | ID of the project that produces and owns this service. |
| `service_name` | String |  | The name of the service. See the [overview](https://cloud.google.com/service-infrastructure/docs/overview) for naming requirements. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `producer_project_id` | String | ID of the project that produces and owns this service. |
| `service_name` | String | The name of the service. See the [overview](https://cloud.google.com/service-infrastructure/docs/overview) for naming requirements. |


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
service_producer_project_id = service.producer_project_id
service_service_name = service.service_name
```

---


### Config

Creates a new service configuration (version) for a managed service. This method only stores the service configuration. To roll out the service configuration to backend systems please call CreateServiceRollout. Only the 100 most recent service configurations and ones referenced by existing rollouts are kept for each service. The rest will be deleted eventually.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `types` | Vec<String> |  | A list of all proto message types included in this API service. Types referenced directly or indirectly by the `apis` are automatically included. Messages which are not referenced but shall be included, such as types used by the `google.protobuf.Any` type, should be listed here by name by the configuration author. Example: types: - name: google.protobuf.Int32 |
| `control` | String |  | Configuration for the service control plane. |
| `usage` | String |  | Configuration controlling usage of this service. |
| `endpoints` | Vec<String> |  | Configuration for network endpoints. If this is empty, then an endpoint with the same name as the service is automatically generated to service all defined APIs. |
| `apis` | Vec<String> |  | A list of API interfaces exported by this service. Only the `name` field of the google.protobuf.Api needs to be provided by the configuration author, as the remaining fields will be derived from the IDL during the normalization process. It is an error to specify an API interface here which cannot be resolved against the associated IDL files. |
| `enums` | Vec<String> |  | A list of all enum types included in this API service. Enums referenced directly or indirectly by the `apis` are automatically included. Enums which are not referenced but shall be included should be listed here by name by the configuration author. Example: enums: - name: google.someapi.v1.SomeEnum |
| `source_info` | String |  | Output only. The source information for this configuration if available. |
| `backend` | String |  | API backend configuration. |
| `context` | String |  | Context configuration. |
| `documentation` | String |  | Additional API documentation. |
| `authentication` | String |  | Auth configuration. |
| `metrics` | Vec<String> |  | Defines the metrics used by this service. |
| `custom_error` | String |  | Custom error configuration. |
| `monitored_resources` | Vec<String> |  | Defines the monitored resources used by this service. This is required by the Service.monitoring and Service.logging configurations. |
| `billing` | String |  | Billing configuration. |
| `system_parameters` | String |  | System parameter configuration. |
| `aspects` | Vec<String> |  | Configuration aspects. This is a repeated field to allow multiple aspects to be configured. The kind field in each ConfigAspect specifies the type of aspect. The spec field contains the configuration for that aspect. The schema for the spec field is defined by the backend service owners. |
| `logging` | String |  | Logging configuration. |
| `monitoring` | String |  | Monitoring configuration. |
| `publishing` | String |  | Settings for [Google Cloud Client libraries](https://cloud.google.com/apis/docs/cloud-client-libraries) generated from APIs defined as protocol buffers. |
| `id` | String |  | A unique ID for a specific instance of this message, typically assigned by the client for tracking purpose. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If empty, the server may choose to generate one instead. |
| `name` | String |  | The service name, which is a DNS-like logical identifier for the service, such as `calendar.googleapis.com`. The service name typically goes through DNS verification to make sure the owner of the service also owns the DNS name. |
| `http` | String |  | HTTP configuration. |
| `config_version` | i64 |  | Obsolete. Do not use. This field has no semantic meaning. The service config compiler always sets this field to `3`. |
| `title` | String |  | The product title for this service, it is the name displayed in Google Cloud Console. |
| `logs` | Vec<String> |  | Defines the logs used by this service. |
| `quota` | String |  | Quota configuration. |
| `system_types` | Vec<String> |  | A list of all proto message types included in this API service. It serves similar purpose as [google.api.Service.types], except that these types are not needed by user-defined APIs. Therefore, they will not show up in the generated discovery doc. This field should only be used to define system APIs in ESF. |
| `producer_project_id` | String |  | The Google project that owns this service. |
| `service_name` | String | ✅ | Required. The name of the service. See the [overview](https://cloud.google.com/service-management/overview) for naming requirements. For example: `example.googleapis.com`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `types` | Vec<String> | A list of all proto message types included in this API service. Types referenced directly or indirectly by the `apis` are automatically included. Messages which are not referenced but shall be included, such as types used by the `google.protobuf.Any` type, should be listed here by name by the configuration author. Example: types: - name: google.protobuf.Int32 |
| `control` | String | Configuration for the service control plane. |
| `usage` | String | Configuration controlling usage of this service. |
| `endpoints` | Vec<String> | Configuration for network endpoints. If this is empty, then an endpoint with the same name as the service is automatically generated to service all defined APIs. |
| `apis` | Vec<String> | A list of API interfaces exported by this service. Only the `name` field of the google.protobuf.Api needs to be provided by the configuration author, as the remaining fields will be derived from the IDL during the normalization process. It is an error to specify an API interface here which cannot be resolved against the associated IDL files. |
| `enums` | Vec<String> | A list of all enum types included in this API service. Enums referenced directly or indirectly by the `apis` are automatically included. Enums which are not referenced but shall be included should be listed here by name by the configuration author. Example: enums: - name: google.someapi.v1.SomeEnum |
| `source_info` | String | Output only. The source information for this configuration if available. |
| `backend` | String | API backend configuration. |
| `context` | String | Context configuration. |
| `documentation` | String | Additional API documentation. |
| `authentication` | String | Auth configuration. |
| `metrics` | Vec<String> | Defines the metrics used by this service. |
| `custom_error` | String | Custom error configuration. |
| `monitored_resources` | Vec<String> | Defines the monitored resources used by this service. This is required by the Service.monitoring and Service.logging configurations. |
| `billing` | String | Billing configuration. |
| `system_parameters` | String | System parameter configuration. |
| `aspects` | Vec<String> | Configuration aspects. This is a repeated field to allow multiple aspects to be configured. The kind field in each ConfigAspect specifies the type of aspect. The spec field contains the configuration for that aspect. The schema for the spec field is defined by the backend service owners. |
| `logging` | String | Logging configuration. |
| `monitoring` | String | Monitoring configuration. |
| `publishing` | String | Settings for [Google Cloud Client libraries](https://cloud.google.com/apis/docs/cloud-client-libraries) generated from APIs defined as protocol buffers. |
| `id` | String | A unique ID for a specific instance of this message, typically assigned by the client for tracking purpose. Must be no longer than 63 characters and only lower case letters, digits, '.', '_' and '-' are allowed. If empty, the server may choose to generate one instead. |
| `name` | String | The service name, which is a DNS-like logical identifier for the service, such as `calendar.googleapis.com`. The service name typically goes through DNS verification to make sure the owner of the service also owns the DNS name. |
| `http` | String | HTTP configuration. |
| `config_version` | i64 | Obsolete. Do not use. This field has no semantic meaning. The service config compiler always sets this field to `3`. |
| `title` | String | The product title for this service, it is the name displayed in Google Cloud Console. |
| `logs` | Vec<String> | Defines the logs used by this service. |
| `quota` | String | Quota configuration. |
| `system_types` | Vec<String> | A list of all proto message types included in this API service. It serves similar purpose as [google.api.Service.types], except that these types are not needed by user-defined APIs. Therefore, they will not show up in the generated discovery doc. This field should only be used to define system APIs in ESF. |
| `producer_project_id` | String | The Google project that owns this service. |


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
config_types = config.types
config_control = config.control
config_usage = config.usage
config_endpoints = config.endpoints
config_apis = config.apis
config_enums = config.enums
config_source_info = config.source_info
config_backend = config.backend
config_context = config.context
config_documentation = config.documentation
config_authentication = config.authentication
config_metrics = config.metrics
config_custom_error = config.custom_error
config_monitored_resources = config.monitored_resources
config_billing = config.billing
config_system_parameters = config.system_parameters
config_aspects = config.aspects
config_logging = config.logging
config_monitoring = config.monitoring
config_publishing = config.publishing
config_id = config.id
config_name = config.name
config_http = config.http
config_config_version = config.config_version
config_title = config.title
config_logs = config.logs
config_quota = config.quota
config_system_types = config.system_types
config_producer_project_id = config.producer_project_id
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

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple rollout resources
rollout_0 = provider.servicemanagement_api.Rollout {
    service_name = "value-0"
}
rollout_1 = provider.servicemanagement_api.Rollout {
    service_name = "value-1"
}
rollout_2 = provider.servicemanagement_api.Rollout {
    service_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    rollout = provider.servicemanagement_api.Rollout {
        service_name = "production-value"
    }
```

---

## Related Documentation

- [GCP Servicemanagement_api Documentation](https://cloud.google.com/servicemanagement_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
