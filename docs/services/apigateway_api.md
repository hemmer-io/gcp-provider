# Apigateway_api Service



**Resources**: 20

---

## Overview

The apigateway_api service provides access to 20 resource types:

- [Location](#location) [R]
- [Gateway](#gateway) [CRUD]
- [Operation](#operation) [CRD]
- [Api](#api) [CRUD]
- [Config](#config) [CRUD]
- [Location](#location) [R]
- [Gateway](#gateway) [CR]
- [Config](#config) [CR]
- [Operation](#operation) [CRD]
- [Api](#api) [CR]
- [Config](#config) [CRUD]
- [Gateway](#gateway) [CRUD]
- [Location](#location) [R]
- [Api](#api) [CRUD]
- [Operation](#operation) [CRD]
- [Gateway](#gateway) [CR]
- [Api](#api) [CR]
- [Location](#location) [R]
- [Config](#config) [CR]
- [Operation](#operation) [CRD]

---

## Resources


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_location_id = location.location_id
location_display_name = location.display_name
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
```

---


### Gateway

Creates a new Gateway in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The current state of the Gateway. |
| `api_config` | String |  | Required. Resource name of the API Config for this Gateway. Format: projects/{project}/locations/global/apis/{api}/configs/{apiConfig} |
| `create_time` | String |  | Output only. Created time. |
| `display_name` | String |  | Optional. Display name. |
| `name` | String |  | Output only. Resource name of the Gateway. Format: projects/{project}/locations/{location}/gateways/{gateway} |
| `update_time` | String |  | Output only. Updated time. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `default_hostname` | String |  | Output only. The default API Gateway host name of the form `{gateway_id}-{hash}.{region_code}.gateway.dev`. |
| `parent` | String | ✅ | Required. Parent resource of the Gateway, of the form: `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The current state of the Gateway. |
| `api_config` | String | Required. Resource name of the API Config for this Gateway. Format: projects/{project}/locations/global/apis/{api}/configs/{apiConfig} |
| `create_time` | String | Output only. Created time. |
| `display_name` | String | Optional. Display name. |
| `name` | String | Output only. Resource name of the Gateway. Format: projects/{project}/locations/{location}/gateways/{gateway} |
| `update_time` | String | Output only. Updated time. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `default_hostname` | String | Output only. The default API Gateway host name of the form `{gateway_id}-{hash}.{region_code}.gateway.dev`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gateway
gateway = provider.apigateway_api.Gateway {
    parent = "value"  # Required. Parent resource of the Gateway, of the form: `projects/*/locations/*`
}

# Access gateway outputs
gateway_id = gateway.id
gateway_state = gateway.state
gateway_api_config = gateway.api_config
gateway_create_time = gateway.create_time
gateway_display_name = gateway.display_name
gateway_name = gateway.name
gateway_update_time = gateway.update_time
gateway_labels = gateway.labels
gateway_default_hostname = gateway.default_hostname
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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

# Create operation
operation = provider.apigateway_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
```

---


### Api

Creates a new Api in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the API. |
| `display_name` | String |  | Optional. Display name. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `managed_service` | String |  | Optional. Immutable. The name of a Google Managed Service ( https://cloud.google.com/service-infrastructure/docs/glossary#managed). If not specified, a new Service will automatically be created in the same project as this API. |
| `update_time` | String |  | Output only. Updated time. |
| `create_time` | String |  | Output only. Created time. |
| `name` | String |  | Output only. Resource name of the API. Format: projects/{project}/locations/global/apis/{api} |
| `parent` | String | ✅ | Required. Parent resource of the API, of the form: `projects/*/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the API. |
| `display_name` | String | Optional. Display name. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `managed_service` | String | Optional. Immutable. The name of a Google Managed Service ( https://cloud.google.com/service-infrastructure/docs/glossary#managed). If not specified, a new Service will automatically be created in the same project as this API. |
| `update_time` | String | Output only. Updated time. |
| `create_time` | String | Output only. Created time. |
| `name` | String | Output only. Resource name of the API. Format: projects/{project}/locations/global/apis/{api} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create api
api = provider.apigateway_api.Api {
    parent = "value"  # Required. Parent resource of the API, of the form: `projects/*/locations/global`
}

# Access api outputs
api_id = api.id
api_state = api.state
api_display_name = api.display_name
api_labels = api.labels
api_managed_service = api.managed_service
api_update_time = api.update_time
api_create_time = api.create_time
api_name = api.name
```

---


### Config

Creates a new ApiConfig in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. Display name. |
| `update_time` | String |  | Output only. Updated time. |
| `grpc_services` | Vec<String> |  | Optional. gRPC service definition files. If specified, openapi_documents must not be included. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `service_config_id` | String |  | Output only. The ID of the associated Service Config ( https://cloud.google.com/service-infrastructure/docs/glossary#config). |
| `managed_service_configs` | Vec<String> |  | Optional. Service Configuration files. At least one must be included when using gRPC service definitions. See https://cloud.google.com/endpoints/docs/grpc/grpc-service-config#service_configuration_overview for the expected file contents. If multiple files are specified, the files are merged with the following rules: * All singular scalar fields are merged using "last one wins" semantics in the order of the files uploaded. * Repeated fields are concatenated. * Singular embedded messages are merged using these rules for nested fields. |
| `openapi_documents` | Vec<String> |  | Optional. OpenAPI specification documents. If specified, grpc_services and managed_service_configs must not be included. |
| `gateway_service_account` | String |  | Immutable. The Google Cloud IAM Service Account that Gateways serving this config should use to authenticate to other services. This may either be the Service Account's email (`{ACCOUNT_ID}@{PROJECT}.iam.gserviceaccount.com`) or its full resource name (`projects/{PROJECT}/accounts/{UNIQUE_ID}`). This is most often used when the service is a GCP resource such as a Cloud Run Service or an IAP-secured service. |
| `name` | String |  | Output only. Resource name of the API Config. Format: projects/{project}/locations/global/apis/{api}/configs/{api_config} |
| `state` | String |  | Output only. State of the API Config. |
| `create_time` | String |  | Output only. Created time. |
| `parent` | String | ✅ | Required. Parent resource of the API Config, of the form: `projects/*/locations/global/apis/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. Display name. |
| `update_time` | String | Output only. Updated time. |
| `grpc_services` | Vec<String> | Optional. gRPC service definition files. If specified, openapi_documents must not be included. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `service_config_id` | String | Output only. The ID of the associated Service Config ( https://cloud.google.com/service-infrastructure/docs/glossary#config). |
| `managed_service_configs` | Vec<String> | Optional. Service Configuration files. At least one must be included when using gRPC service definitions. See https://cloud.google.com/endpoints/docs/grpc/grpc-service-config#service_configuration_overview for the expected file contents. If multiple files are specified, the files are merged with the following rules: * All singular scalar fields are merged using "last one wins" semantics in the order of the files uploaded. * Repeated fields are concatenated. * Singular embedded messages are merged using these rules for nested fields. |
| `openapi_documents` | Vec<String> | Optional. OpenAPI specification documents. If specified, grpc_services and managed_service_configs must not be included. |
| `gateway_service_account` | String | Immutable. The Google Cloud IAM Service Account that Gateways serving this config should use to authenticate to other services. This may either be the Service Account's email (`{ACCOUNT_ID}@{PROJECT}.iam.gserviceaccount.com`) or its full resource name (`projects/{PROJECT}/accounts/{UNIQUE_ID}`). This is most often used when the service is a GCP resource such as a Cloud Run Service or an IAP-secured service. |
| `name` | String | Output only. Resource name of the API Config. Format: projects/{project}/locations/global/apis/{api}/configs/{api_config} |
| `state` | String | Output only. State of the API Config. |
| `create_time` | String | Output only. Created time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create config
config = provider.apigateway_api.Config {
    parent = "value"  # Required. Parent resource of the API Config, of the form: `projects/*/locations/global/apis/*`
}

# Access config outputs
config_id = config.id
config_display_name = config.display_name
config_update_time = config.update_time
config_grpc_services = config.grpc_services
config_labels = config.labels
config_service_config_id = config.service_config_id
config_managed_service_configs = config.managed_service_configs
config_openapi_documents = config.openapi_documents
config_gateway_service_account = config.gateway_service_account
config_name = config.name
config_state = config.state
config_create_time = config.create_time
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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


### Gateway

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bindings` | Vec<String> | Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gateway
gateway = provider.apigateway_api.Gateway {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field.
}

# Access gateway outputs
gateway_id = gateway.id
gateway_bindings = gateway.bindings
gateway_audit_configs = gateway.audit_configs
gateway_version = gateway.version
gateway_etag = gateway.etag
```

---


### Config

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bindings` | Vec<String> | Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create config
config = provider.apigateway_api.Config {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field.
}

# Access config outputs
config_id = config.id
config_bindings = config.bindings
config_audit_configs = config.audit_configs
config_version = config.version
config_etag = config.etag
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.apigateway_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
operation_error = operation.error
```

---


### Api

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bindings` | Vec<String> | Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create api
api = provider.apigateway_api.Api {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field.
}

# Access api outputs
api_id = api.id
api_bindings = api.bindings
api_audit_configs = api.audit_configs
api_version = api.version
api_etag = api.etag
```

---


### Config

Creates a new ApiConfig in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gateway_config` | String |  | Immutable. Gateway specific configuration. |
| `openapi_documents` | Vec<String> |  | Optional. OpenAPI specification documents. If specified, grpc_services and managed_service_configs must not be included. |
| `grpc_services` | Vec<String> |  | Optional. gRPC service definition files. If specified, openapi_documents must not be included. |
| `create_time` | String |  | Output only. Created time. |
| `managed_service_configs` | Vec<String> |  | Optional. Service Configuration files. At least one must be included when using gRPC service definitions. See https://cloud.google.com/endpoints/docs/grpc/grpc-service-config#service_configuration_overview for the expected file contents. If multiple files are specified, the files are merged with the following rules: * All singular scalar fields are merged using "last one wins" semantics in the order of the files uploaded. * Repeated fields are concatenated. * Singular embedded messages are merged using these rules for nested fields. |
| `name` | String |  | Output only. Resource name of the API Config. Format: projects/{project}/locations/global/apis/{api}/configs/{api_config} |
| `service_config_id` | String |  | Output only. The ID of the associated Service Config ( https://cloud.google.com/service-infrastructure/docs/glossary#config). |
| `display_name` | String |  | Optional. Display name. |
| `state` | String |  | Output only. State of the API Config. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `update_time` | String |  | Output only. Updated time. |
| `gateway_service_account` | String |  | Immutable. The Google Cloud IAM Service Account that Gateways serving this config should use to authenticate to other services. This may either be the Service Account's email (`{ACCOUNT_ID}@{PROJECT}.iam.gserviceaccount.com`) or its full resource name (`projects/{PROJECT}/accounts/{UNIQUE_ID}`). This is most often used when the service is a GCP resource such as a Cloud Run Service or an IAP-secured service. |
| `parent` | String | ✅ | Required. Parent resource of the API Config, of the form: `projects/*/locations/global/apis/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gateway_config` | String | Immutable. Gateway specific configuration. |
| `openapi_documents` | Vec<String> | Optional. OpenAPI specification documents. If specified, grpc_services and managed_service_configs must not be included. |
| `grpc_services` | Vec<String> | Optional. gRPC service definition files. If specified, openapi_documents must not be included. |
| `create_time` | String | Output only. Created time. |
| `managed_service_configs` | Vec<String> | Optional. Service Configuration files. At least one must be included when using gRPC service definitions. See https://cloud.google.com/endpoints/docs/grpc/grpc-service-config#service_configuration_overview for the expected file contents. If multiple files are specified, the files are merged with the following rules: * All singular scalar fields are merged using "last one wins" semantics in the order of the files uploaded. * Repeated fields are concatenated. * Singular embedded messages are merged using these rules for nested fields. |
| `name` | String | Output only. Resource name of the API Config. Format: projects/{project}/locations/global/apis/{api}/configs/{api_config} |
| `service_config_id` | String | Output only. The ID of the associated Service Config ( https://cloud.google.com/service-infrastructure/docs/glossary#config). |
| `display_name` | String | Optional. Display name. |
| `state` | String | Output only. State of the API Config. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `update_time` | String | Output only. Updated time. |
| `gateway_service_account` | String | Immutable. The Google Cloud IAM Service Account that Gateways serving this config should use to authenticate to other services. This may either be the Service Account's email (`{ACCOUNT_ID}@{PROJECT}.iam.gserviceaccount.com`) or its full resource name (`projects/{PROJECT}/accounts/{UNIQUE_ID}`). This is most often used when the service is a GCP resource such as a Cloud Run Service or an IAP-secured service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create config
config = provider.apigateway_api.Config {
    parent = "value"  # Required. Parent resource of the API Config, of the form: `projects/*/locations/global/apis/*`
}

# Access config outputs
config_id = config.id
config_gateway_config = config.gateway_config
config_openapi_documents = config.openapi_documents
config_grpc_services = config.grpc_services
config_create_time = config.create_time
config_managed_service_configs = config.managed_service_configs
config_name = config.name
config_service_config_id = config.service_config_id
config_display_name = config.display_name
config_state = config.state
config_labels = config.labels
config_update_time = config.update_time
config_gateway_service_account = config.gateway_service_account
```

---


### Gateway

Creates a new Gateway in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_hostname` | String |  | Output only. The default API Gateway host name of the form `{gateway_id}-{hash}.{region_code}.gateway.dev`. |
| `state` | String |  | Output only. The current state of the Gateway. |
| `name` | String |  | Output only. Resource name of the Gateway. Format: projects/{project}/locations/{location}/gateways/{gateway} |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `api_config` | String |  | Required. Resource name of the API Config for this Gateway. Format: projects/{project}/locations/global/apis/{api}/configs/{apiConfig} |
| `update_time` | String |  | Output only. Updated time. |
| `create_time` | String |  | Output only. Created time. |
| `display_name` | String |  | Optional. Display name. |
| `parent` | String | ✅ | Required. Parent resource of the Gateway, of the form: `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_hostname` | String | Output only. The default API Gateway host name of the form `{gateway_id}-{hash}.{region_code}.gateway.dev`. |
| `state` | String | Output only. The current state of the Gateway. |
| `name` | String | Output only. Resource name of the Gateway. Format: projects/{project}/locations/{location}/gateways/{gateway} |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `api_config` | String | Required. Resource name of the API Config for this Gateway. Format: projects/{project}/locations/global/apis/{api}/configs/{apiConfig} |
| `update_time` | String | Output only. Updated time. |
| `create_time` | String | Output only. Created time. |
| `display_name` | String | Optional. Display name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gateway
gateway = provider.apigateway_api.Gateway {
    parent = "value"  # Required. Parent resource of the Gateway, of the form: `projects/*/locations/*`
}

# Access gateway outputs
gateway_id = gateway.id
gateway_default_hostname = gateway.default_hostname
gateway_state = gateway.state
gateway_name = gateway.name
gateway_labels = gateway.labels
gateway_api_config = gateway.api_config
gateway_update_time = gateway.update_time
gateway_create_time = gateway.create_time
gateway_display_name = gateway.display_name
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
```

---


### Api

Creates a new Api in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the API. |
| `display_name` | String |  | Optional. Display name. |
| `update_time` | String |  | Output only. Updated time. |
| `create_time` | String |  | Output only. Created time. |
| `managed_service` | String |  | Optional. Immutable. The name of a Google Managed Service ( https://cloud.google.com/service-infrastructure/docs/glossary#managed). If not specified, a new Service will automatically be created in the same project as this API. |
| `name` | String |  | Output only. Resource name of the API. Format: projects/{project}/locations/global/apis/{api} |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `parent` | String | ✅ | Required. Parent resource of the API, of the form: `projects/*/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the API. |
| `display_name` | String | Optional. Display name. |
| `update_time` | String | Output only. Updated time. |
| `create_time` | String | Output only. Created time. |
| `managed_service` | String | Optional. Immutable. The name of a Google Managed Service ( https://cloud.google.com/service-infrastructure/docs/glossary#managed). If not specified, a new Service will automatically be created in the same project as this API. |
| `name` | String | Output only. Resource name of the API. Format: projects/{project}/locations/global/apis/{api} |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create api
api = provider.apigateway_api.Api {
    parent = "value"  # Required. Parent resource of the API, of the form: `projects/*/locations/global`
}

# Access api outputs
api_id = api.id
api_state = api.state
api_display_name = api.display_name
api_update_time = api.update_time
api_create_time = api.create_time
api_managed_service = api.managed_service
api_name = api.name
api_labels = api.labels
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.apigateway_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_name = operation.name
operation_metadata = operation.metadata
```

---


### Gateway

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `bindings` | Vec<String> | Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gateway
gateway = provider.apigateway_api.Gateway {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field.
}

# Access gateway outputs
gateway_id = gateway.id
gateway_version = gateway.version
gateway_etag = gateway.etag
gateway_audit_configs = gateway.audit_configs
gateway_bindings = gateway.bindings
```

---


### Api

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `bindings` | Vec<String> | Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create api
api = provider.apigateway_api.Api {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See the operation documentation for the appropriate value for this field.
}

# Access api outputs
api_id = api.id
api_version = api.version
api_etag = api.etag
api_audit_configs = api.audit_configs
api_bindings = api.bindings
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_name = location.name
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
location_display_name = location.display_name
```

---


### Config

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `bindings` | Vec<String> | Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create config
config = provider.apigateway_api.Config {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See the operation documentation for the appropriate value for this field.
}

# Access config outputs
config_id = config.id
config_version = config.version
config_etag = config.etag
config_audit_configs = config.audit_configs
config_bindings = config.bindings
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.apigateway_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
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

# Create multiple location resources
location_0 = provider.apigateway_api.Location {
}
location_1 = provider.apigateway_api.Location {
}
location_2 = provider.apigateway_api.Location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.apigateway_api.Location {
    }
```

---

## Related Documentation

- [GCP Apigateway_api Documentation](https://cloud.google.com/apigateway_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
