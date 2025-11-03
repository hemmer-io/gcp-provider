# Apihub_api Service



**Resources**: 19

---

## Overview

The apihub_api service provides access to 19 resource types:

- [Instance](#instance) [CRUD]
- [Discovered_api_operation](#discovered_api_operation) [R]
- [Style_guide](#style_guide) [R]
- [Runtime_project_attachment](#runtime_project_attachment) [CRD]
- [Operation](#operation) [CRUD]
- [Api_hub_instance](#api_hub_instance) [CRD]
- [Spec](#spec) [CRUD]
- [External_api](#external_api) [CRUD]
- [Definition](#definition) [R]
- [Attribute](#attribute) [CRUD]
- [Discovered_api_observation](#discovered_api_observation) [R]
- [Deployment](#deployment) [CRUD]
- [Host_project_registration](#host_project_registration) [CR]
- [Curation](#curation) [CRUD]
- [Version](#version) [CRUD]
- [Api](#api) [CRUD]
- [Plugin](#plugin) [CRUD]
- [Dependencie](#dependencie) [CRUD]
- [Location](#location) [CR]

---

## Resources


### Instance

Creates a Plugin instance in the API hub.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Timestamp indicating when the plugin instance was last updated. |
| `state` | String |  | Output only. The current state of the plugin instance (e.g., enabled, disabled, provisioning). |
| `error_message` | String |  | Output only. Error message describing the failure, if any, during Create, Delete or ApplyConfig operation corresponding to the plugin instance.This field will only be populated if the plugin instance is in the ERROR or FAILED state. |
| `actions` | Vec<String> |  | Required. The action status for the plugin instance. |
| `additional_config` | HashMap<String, String> |  | Optional. The additional information for this plugin instance corresponding to the additional config template of the plugin. This information will be sent to plugin hosting service on each call to plugin hosted service. The key will be the config_variable_template.display_name to uniquely identify the config variable. |
| `display_name` | String |  | Required. The display name for this plugin instance. Max length is 255 characters. |
| `auth_config` | String |  | Optional. The authentication information for this plugin instance. |
| `source_project_id` | String |  | Optional. The source project id of the plugin instance. This will be the id of runtime project in case of gcp based plugins and org id in case of non gcp based plugins. This field will be a required field for Google provided on-ramp plugins. |
| `name` | String |  | Identifier. The unique name of the plugin instance resource. Format: `projects/{project}/locations/{location}/plugins/{plugin}/instances/{instance}` |
| `create_time` | String |  | Output only. Timestamp indicating when the plugin instance was created. |
| `parent` | String | ✅ | Required. The parent of the plugin instance resource. Format: `projects/{project}/locations/{location}/plugins/{plugin}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Timestamp indicating when the plugin instance was last updated. |
| `state` | String | Output only. The current state of the plugin instance (e.g., enabled, disabled, provisioning). |
| `error_message` | String | Output only. Error message describing the failure, if any, during Create, Delete or ApplyConfig operation corresponding to the plugin instance.This field will only be populated if the plugin instance is in the ERROR or FAILED state. |
| `actions` | Vec<String> | Required. The action status for the plugin instance. |
| `additional_config` | HashMap<String, String> | Optional. The additional information for this plugin instance corresponding to the additional config template of the plugin. This information will be sent to plugin hosting service on each call to plugin hosted service. The key will be the config_variable_template.display_name to uniquely identify the config variable. |
| `display_name` | String | Required. The display name for this plugin instance. Max length is 255 characters. |
| `auth_config` | String | Optional. The authentication information for this plugin instance. |
| `source_project_id` | String | Optional. The source project id of the plugin instance. This will be the id of runtime project in case of gcp based plugins and org id in case of non gcp based plugins. This field will be a required field for Google provided on-ramp plugins. |
| `name` | String | Identifier. The unique name of the plugin instance resource. Format: `projects/{project}/locations/{location}/plugins/{plugin}/instances/{instance}` |
| `create_time` | String | Output only. Timestamp indicating when the plugin instance was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.apihub_api.Instance {
    parent = "value"  # Required. The parent of the plugin instance resource. Format: `projects/{project}/locations/{location}/plugins/{plugin}`
}

# Access instance outputs
instance_id = instance.id
instance_update_time = instance.update_time
instance_state = instance.state
instance_error_message = instance.error_message
instance_actions = instance.actions
instance_additional_config = instance.additional_config
instance_display_name = instance.display_name
instance_auth_config = instance.auth_config
instance_source_project_id = instance.source_project_id
instance_name = instance.name
instance_create_time = instance.create_time
```

---


### Discovered_api_operation

Gets a DiscoveredAPIOperation in a given project, location, ApiObservation and ApiOperation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `source_metadata` | String | Output only. The metadata of the source from which the api operation was collected. |
| `create_time` | String | Output only. Create time stamp of the discovered API operation in API Hub. |
| `match_results` | Vec<String> | Output only. The list of matched results for the discovered API operation. This will be populated only if the classification is known. The current usecase is for a single match. Keeping it repeated to support multiple matches in future. |
| `name` | String | Identifier. The name of the discovered API Operation. Format: `projects/{project}/locations/{location}/discoveredApiObservations/{discovered_api_observation}/discoveredApiOperations/{discovered_api_operation}` |
| `update_time` | String | Output only. Update time stamp of the discovered API operation in API Hub. |
| `first_seen_time` | String | Optional. First seen time stamp |
| `http_operation` | String | Optional. An HTTP Operation. |
| `classification` | String | Output only. The classification of the discovered API operation. |
| `count` | String | Optional. The number of occurrences of this API Operation. |
| `last_seen_time` | String | Optional. Last seen time stamp |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access discovered_api_operation outputs
discovered_api_operation_id = discovered_api_operation.id
discovered_api_operation_source_metadata = discovered_api_operation.source_metadata
discovered_api_operation_create_time = discovered_api_operation.create_time
discovered_api_operation_match_results = discovered_api_operation.match_results
discovered_api_operation_name = discovered_api_operation.name
discovered_api_operation_update_time = discovered_api_operation.update_time
discovered_api_operation_first_seen_time = discovered_api_operation.first_seen_time
discovered_api_operation_http_operation = discovered_api_operation.http_operation
discovered_api_operation_classification = discovered_api_operation.classification
discovered_api_operation_count = discovered_api_operation.count
discovered_api_operation_last_seen_time = discovered_api_operation.last_seen_time
```

---


### Style_guide

Get the contents of the style guide.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mime_type` | String | Required. The mime type of the content. |
| `contents` | String | Required. The contents of the style guide. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access style_guide outputs
style_guide_id = style_guide.id
style_guide_mime_type = style_guide.mime_type
style_guide_contents = style_guide.contents
```

---


### Runtime_project_attachment

Attaches a runtime project to the host project.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `runtime_project` | String |  | Required. Immutable. Google cloud project name in the format: "projects/abc" or "projects/123". As input, project name with either project id or number are accepted. As output, this field will contain project number. |
| `name` | String |  | Identifier. The resource name of a runtime project attachment. Format: "projects/{project}/locations/{location}/runtimeProjectAttachments/{runtime_project_attachment}". |
| `create_time` | String |  | Output only. Create time. |
| `parent` | String | ✅ | Required. The parent resource for the Runtime Project Attachment. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `runtime_project` | String | Required. Immutable. Google cloud project name in the format: "projects/abc" or "projects/123". As input, project name with either project id or number are accepted. As output, this field will contain project number. |
| `name` | String | Identifier. The resource name of a runtime project attachment. Format: "projects/{project}/locations/{location}/runtimeProjectAttachments/{runtime_project_attachment}". |
| `create_time` | String | Output only. Create time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create runtime_project_attachment
runtime_project_attachment = provider.apihub_api.Runtime_project_attachment {
    parent = "value"  # Required. The parent resource for the Runtime Project Attachment. Format: `projects/{project}/locations/{location}`
}

# Access runtime_project_attachment outputs
runtime_project_attachment_id = runtime_project_attachment.id
runtime_project_attachment_runtime_project = runtime_project_attachment.runtime_project
runtime_project_attachment_name = runtime_project_attachment.name
runtime_project_attachment_create_time = runtime_project_attachment.create_time
```

---


### Operation

Create an apiOperation in an API version. An apiOperation can be created only if the version has no apiOperations which were created by parsing a spec.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `details` | String |  | Optional. Operation details. Note: Even though this field is optional, it is required for CreateApiOperation API and we will fail the request if not provided. |
| `update_time` | String |  | Output only. The time at which the operation was last updated. |
| `create_time` | String |  | Output only. The time at which the operation was created. |
| `name` | String |  | Identifier. The name of the operation. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}/operations/{operation}` |
| `attributes` | HashMap<String, String> |  | Optional. The list of user defined attributes associated with the API operation resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `source_metadata` | Vec<String> |  | Output only. The list of sources and metadata from the sources of the API operation. |
| `spec` | String |  | Output only. The name of the spec will be of the format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}/specs/{spec}` Note:The name of the spec will be empty if the operation is created via CreateApiOperation API. |
| `parent` | String | ✅ | Required. The parent resource for the operation resource. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.apihub_api.Operation {
    parent = "value"  # Required. The parent resource for the operation resource. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}`
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
```

---


### Api_hub_instance

Provisions instance resources for the API Hub.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Format: `projects/{project}/locations/{location}/apiHubInstances/{apiHubInstance}`. |
| `update_time` | String |  | Output only. Last update timestamp. |
| `create_time` | String |  | Output only. Creation timestamp. |
| `state` | String |  | Output only. The current state of the ApiHub instance. |
| `description` | String |  | Optional. Description of the ApiHub instance. |
| `state_message` | String |  | Output only. Extra information about ApiHub instance state. Currently the message would be populated when state is `FAILED`. |
| `config` | String |  | Required. Config of the ApiHub instance. |
| `labels` | HashMap<String, String> |  | Optional. Instance labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `parent` | String | ✅ | Required. The parent resource for the Api Hub instance resource. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Format: `projects/{project}/locations/{location}/apiHubInstances/{apiHubInstance}`. |
| `update_time` | String | Output only. Last update timestamp. |
| `create_time` | String | Output only. Creation timestamp. |
| `state` | String | Output only. The current state of the ApiHub instance. |
| `description` | String | Optional. Description of the ApiHub instance. |
| `state_message` | String | Output only. Extra information about ApiHub instance state. Currently the message would be populated when state is `FAILED`. |
| `config` | String | Required. Config of the ApiHub instance. |
| `labels` | HashMap<String, String> | Optional. Instance labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create api_hub_instance
api_hub_instance = provider.apihub_api.Api_hub_instance {
    parent = "value"  # Required. The parent resource for the Api Hub instance resource. Format: `projects/{project}/locations/{location}`
}

# Access api_hub_instance outputs
api_hub_instance_id = api_hub_instance.id
api_hub_instance_name = api_hub_instance.name
api_hub_instance_update_time = api_hub_instance.update_time
api_hub_instance_create_time = api_hub_instance.create_time
api_hub_instance_state = api_hub_instance.state
api_hub_instance_description = api_hub_instance.description
api_hub_instance_state_message = api_hub_instance.state_message
api_hub_instance_config = api_hub_instance.config
api_hub_instance_labels = api_hub_instance.labels
```

---


### Spec

Add a spec to an API version in the API hub. Multiple specs can be added to an API version. Note, while adding a spec, at least one of `contents` or `source_uri` must be provided. If `contents` is provided, then `spec_type` must also be provided. On adding a spec with contents to the version, the operations present in it will be added to the version.Note that the file contents in the spec should be of the same type as defined in the `projects/{project}/locations/{location}/attributes/system-spec-type` attribute associated with spec resource. Note that specs of various types can be uploaded, however parsing of details is supported for OpenAPI spec currently. In order to access the information parsed from the spec, use the GetSpec method. In order to access the raw contents for a particular spec, use the GetSpecContents method. In order to access the operations parsed from the spec, use the ListAPIOperations method.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Required. The display name of the spec. This can contain the file name of the spec. |
| `name` | String |  | Identifier. The name of the spec. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}/specs/{spec}` |
| `parsing_mode` | String |  | Optional. Input only. Enum specifying the parsing mode for OpenAPI Specification (OAS) parsing. |
| `lint_response` | String |  | Optional. The lint response for the spec. |
| `source_uri` | String |  | Optional. The URI of the spec source in case file is uploaded from an external version control system. |
| `update_time` | String |  | Output only. The time at which the spec was last updated. |
| `spec_type` | String |  | Required. The type of spec. The value should be one of the allowed values defined for `projects/{project}/locations/{location}/attributes/system-spec-type` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. Note, this field is mandatory if content is provided. |
| `contents` | String |  | Optional. Input only. The contents of the uploaded spec. |
| `source_metadata` | Vec<String> |  | Output only. The list of sources and metadata from the sources of the spec. |
| `attributes` | HashMap<String, String> |  | Optional. The list of user defined attributes associated with the spec. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `create_time` | String |  | Output only. The time at which the spec was created. |
| `documentation` | String |  | Optional. The documentation of the spec. For OpenAPI spec, this will be populated from `externalDocs` in OpenAPI spec. |
| `details` | String |  | Output only. Details parsed from the spec. |
| `parent` | String | ✅ | Required. The parent resource for Spec. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Required. The display name of the spec. This can contain the file name of the spec. |
| `name` | String | Identifier. The name of the spec. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}/specs/{spec}` |
| `parsing_mode` | String | Optional. Input only. Enum specifying the parsing mode for OpenAPI Specification (OAS) parsing. |
| `lint_response` | String | Optional. The lint response for the spec. |
| `source_uri` | String | Optional. The URI of the spec source in case file is uploaded from an external version control system. |
| `update_time` | String | Output only. The time at which the spec was last updated. |
| `spec_type` | String | Required. The type of spec. The value should be one of the allowed values defined for `projects/{project}/locations/{location}/attributes/system-spec-type` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. Note, this field is mandatory if content is provided. |
| `contents` | String | Optional. Input only. The contents of the uploaded spec. |
| `source_metadata` | Vec<String> | Output only. The list of sources and metadata from the sources of the spec. |
| `attributes` | HashMap<String, String> | Optional. The list of user defined attributes associated with the spec. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `create_time` | String | Output only. The time at which the spec was created. |
| `documentation` | String | Optional. The documentation of the spec. For OpenAPI spec, this will be populated from `externalDocs` in OpenAPI spec. |
| `details` | String | Output only. Details parsed from the spec. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create spec
spec = provider.apihub_api.Spec {
    parent = "value"  # Required. The parent resource for Spec. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}`
}

# Access spec outputs
spec_id = spec.id
spec_display_name = spec.display_name
spec_name = spec.name
spec_parsing_mode = spec.parsing_mode
spec_lint_response = spec.lint_response
spec_source_uri = spec.source_uri
spec_update_time = spec.update_time
spec_spec_type = spec.spec_type
spec_contents = spec.contents
spec_source_metadata = spec.source_metadata
spec_attributes = spec.attributes
spec_create_time = spec.create_time
spec_documentation = spec.documentation
spec_details = spec.details
```

---


### External_api

Create an External API resource in the API hub.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `documentation` | String |  | Optional. Documentation of the external API. |
| `create_time` | String |  | Output only. Creation timestamp. |
| `attributes` | HashMap<String, String> |  | Optional. The list of user defined attributes associated with the Version resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `description` | String |  | Optional. Description of the external API. Max length is 2000 characters (Unicode Code Points). |
| `endpoints` | Vec<String> |  | Optional. List of endpoints on which this API is accessible. |
| `name` | String |  | Identifier. Format: `projects/{project}/locations/{location}/externalApi/{externalApi}`. |
| `display_name` | String |  | Required. Display name of the external API. Max length is 63 characters (Unicode Code Points). |
| `paths` | Vec<String> |  | Optional. List of paths served by this API. |
| `update_time` | String |  | Output only. Last update timestamp. |
| `parent` | String | ✅ | Required. The parent resource for the External API resource. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `documentation` | String | Optional. Documentation of the external API. |
| `create_time` | String | Output only. Creation timestamp. |
| `attributes` | HashMap<String, String> | Optional. The list of user defined attributes associated with the Version resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `description` | String | Optional. Description of the external API. Max length is 2000 characters (Unicode Code Points). |
| `endpoints` | Vec<String> | Optional. List of endpoints on which this API is accessible. |
| `name` | String | Identifier. Format: `projects/{project}/locations/{location}/externalApi/{externalApi}`. |
| `display_name` | String | Required. Display name of the external API. Max length is 63 characters (Unicode Code Points). |
| `paths` | Vec<String> | Optional. List of paths served by this API. |
| `update_time` | String | Output only. Last update timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create external_api
external_api = provider.apihub_api.External_api {
    parent = "value"  # Required. The parent resource for the External API resource. Format: `projects/{project}/locations/{location}`
}

# Access external_api outputs
external_api_id = external_api.id
external_api_documentation = external_api.documentation
external_api_create_time = external_api.create_time
external_api_attributes = external_api.attributes
external_api_description = external_api.description
external_api_endpoints = external_api.endpoints
external_api_name = external_api.name
external_api_display_name = external_api.display_name
external_api_paths = external_api.paths
external_api_update_time = external_api.update_time
```

---


### Definition

Get details about a definition in an API version.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | HashMap<String, String> | Optional. The list of user defined attributes associated with the definition resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `create_time` | String | Output only. The time at which the definition was created. |
| `update_time` | String | Output only. The time at which the definition was last updated. |
| `spec` | String | Output only. The name of the spec from where the definition was parsed. Format is `projects/{project}/locations/{location}/apis/{api}/versions/{version}/specs/{spec}` |
| `type` | String | Output only. The type of the definition. |
| `schema` | String | Output only. The value of a schema definition. |
| `name` | String | Identifier. The name of the definition. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}/definitions/{definition}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access definition outputs
definition_id = definition.id
definition_attributes = definition.attributes
definition_create_time = definition.create_time
definition_update_time = definition.update_time
definition_spec = definition.spec
definition_type = definition.type
definition_schema = definition.schema
definition_name = definition.name
```

---


### Attribute

Create a user defined attribute. Certain pre defined attributes are already created by the API hub. These attributes will have type as `SYSTEM_DEFINED` and can be listed via ListAttributes method. Allowed values for the same can be updated via UpdateAttribute method.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mandatory` | bool |  | Output only. When mandatory is true, the attribute is mandatory for the resource specified in the scope. Only System defined attributes can be mandatory. |
| `display_name` | String |  | Required. The display name of the attribute. |
| `create_time` | String |  | Output only. The time at which the attribute was created. |
| `definition_type` | String |  | Output only. The definition type of the attribute. |
| `allowed_values` | Vec<String> |  | Optional. The list of allowed values when the attribute value is of type enum. This is required when the data_type of the attribute is ENUM. The maximum number of allowed values of an attribute will be 1000. |
| `cardinality` | i64 |  | Optional. The maximum number of values that the attribute can have when associated with an API Hub resource. Cardinality 1 would represent a single-valued attribute. It must not be less than 1 or greater than 20. If not specified, the cardinality would be set to 1 by default and represent a single-valued attribute. |
| `name` | String |  | Identifier. The name of the attribute in the API Hub. Format: `projects/{project}/locations/{location}/attributes/{attribute}` |
| `description` | String |  | Optional. The description of the attribute. |
| `update_time` | String |  | Output only. The time at which the attribute was last updated. |
| `data_type` | String |  | Required. The type of the data of the attribute. |
| `scope` | String |  | Required. The scope of the attribute. It represents the resource in the API Hub to which the attribute can be linked. |
| `parent` | String | ✅ | Required. The parent resource for Attribute. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mandatory` | bool | Output only. When mandatory is true, the attribute is mandatory for the resource specified in the scope. Only System defined attributes can be mandatory. |
| `display_name` | String | Required. The display name of the attribute. |
| `create_time` | String | Output only. The time at which the attribute was created. |
| `definition_type` | String | Output only. The definition type of the attribute. |
| `allowed_values` | Vec<String> | Optional. The list of allowed values when the attribute value is of type enum. This is required when the data_type of the attribute is ENUM. The maximum number of allowed values of an attribute will be 1000. |
| `cardinality` | i64 | Optional. The maximum number of values that the attribute can have when associated with an API Hub resource. Cardinality 1 would represent a single-valued attribute. It must not be less than 1 or greater than 20. If not specified, the cardinality would be set to 1 by default and represent a single-valued attribute. |
| `name` | String | Identifier. The name of the attribute in the API Hub. Format: `projects/{project}/locations/{location}/attributes/{attribute}` |
| `description` | String | Optional. The description of the attribute. |
| `update_time` | String | Output only. The time at which the attribute was last updated. |
| `data_type` | String | Required. The type of the data of the attribute. |
| `scope` | String | Required. The scope of the attribute. It represents the resource in the API Hub to which the attribute can be linked. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attribute
attribute = provider.apihub_api.Attribute {
    parent = "value"  # Required. The parent resource for Attribute. Format: `projects/{project}/locations/{location}`
}

# Access attribute outputs
attribute_id = attribute.id
attribute_mandatory = attribute.mandatory
attribute_display_name = attribute.display_name
attribute_create_time = attribute.create_time
attribute_definition_type = attribute.definition_type
attribute_allowed_values = attribute.allowed_values
attribute_cardinality = attribute.cardinality
attribute_name = attribute.name
attribute_description = attribute.description
attribute_update_time = attribute.update_time
attribute_data_type = attribute.data_type
attribute_scope = attribute.scope
```

---


### Discovered_api_observation

Gets a DiscoveredAPIObservation in a given project, location and ApiObservation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `known_operations_count` | String | Output only. The number of known API Operations. |
| `source_metadata` | String | Output only. The metadata of the source from which the observation was collected. |
| `origin` | String | Optional. For an observation pushed from a gcp resource, this would be the gcp project id. |
| `server_ips` | Vec<String> | Optional. The IP address (IPv4 or IPv6) of the origin server that the request was sent to. This field can include port information. Examples: `"192.168.1.1"`, `"10.0.0.1:80"`, `"FE80::0202:B3FF:FE1E:8329"`. |
| `source_locations` | Vec<String> | Optional. The location of the observation source. |
| `name` | String | Identifier. The name of the discovered API Observation. Format: `projects/{project}/locations/{location}/discoveredApiObservations/{discovered_api_observation}` |
| `source_types` | Vec<String> | Optional. The type of the source from which the observation was collected. |
| `style` | String | Optional. Style of ApiObservation |
| `create_time` | String | Output only. Create time stamp of the observation in API Hub. |
| `update_time` | String | Output only. Update time stamp of the observation in API Hub. |
| `last_event_detected_time` | String | Optional. Last event detected time stamp |
| `api_operation_count` | String | Optional. The number of observed API Operations. |
| `hostname` | String | Optional. The hostname of requests processed for this Observation. |
| `unknown_operations_count` | String | Output only. The number of unknown API Operations. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access discovered_api_observation outputs
discovered_api_observation_id = discovered_api_observation.id
discovered_api_observation_known_operations_count = discovered_api_observation.known_operations_count
discovered_api_observation_source_metadata = discovered_api_observation.source_metadata
discovered_api_observation_origin = discovered_api_observation.origin
discovered_api_observation_server_ips = discovered_api_observation.server_ips
discovered_api_observation_source_locations = discovered_api_observation.source_locations
discovered_api_observation_name = discovered_api_observation.name
discovered_api_observation_source_types = discovered_api_observation.source_types
discovered_api_observation_style = discovered_api_observation.style
discovered_api_observation_create_time = discovered_api_observation.create_time
discovered_api_observation_update_time = discovered_api_observation.update_time
discovered_api_observation_last_event_detected_time = discovered_api_observation.last_event_detected_time
discovered_api_observation_api_operation_count = discovered_api_observation.api_operation_count
discovered_api_observation_hostname = discovered_api_observation.hostname
discovered_api_observation_unknown_operations_count = discovered_api_observation.unknown_operations_count
```

---


### Deployment

Create a deployment resource in the API hub. Once a deployment resource is created, it can be associated with API versions.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attributes` | HashMap<String, String> |  | Optional. The list of user defined attributes associated with the deployment resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `source_uri` | String |  | Optional. The uri where additional source specific information for this deployment can be found. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-source-uri` The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. The value of the attribute should be a valid URI, and in case of Cloud Storage URI, it should point to a Cloud Storage object, not a directory. |
| `documentation` | String |  | Optional. The documentation of the deployment. |
| `resource_uri` | String |  | Required. The resource URI identifies the deployment within its gateway. For Apigee gateways, its recommended to use the format: organizations/{org}/environments/{env}/apis/{api}. For ex: if a proxy with name `orders` is deployed in `staging` environment of `cymbal` organization, the resource URI would be: `organizations/cymbal/environments/staging/apis/orders`. |
| `deployment_type` | String |  | Required. The type of deployment. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-deployment-type` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `management_url` | String |  | Optional. The uri where users can navigate to for the management of the deployment. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-management-url` The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. The value of the attribute should be a valid URL. |
| `environment` | String |  | Optional. The environment mapping to this deployment. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-environment` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `name` | String |  | Identifier. The name of the deployment. Format: `projects/{project}/locations/{location}/deployments/{deployment}` |
| `display_name` | String |  | Required. The display name of the deployment. |
| `update_time` | String |  | Output only. The time at which the deployment was last updated. |
| `description` | String |  | Optional. The description of the deployment. |
| `api_versions` | Vec<String> |  | Output only. The API versions linked to this deployment. Note: A particular deployment could be linked to multiple different API versions (of same or different APIs). |
| `slo` | String |  | Optional. The SLO for this deployment. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-slo` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `endpoints` | Vec<String> |  | Required. The endpoints at which this deployment resource is listening for API requests. This could be a list of complete URIs, hostnames or an IP addresses. |
| `source_environment` | String |  | Optional. The environment at source for the deployment. For example: prod, dev, staging, etc. |
| `create_time` | String |  | Output only. The time at which the deployment was created. |
| `source_metadata` | Vec<String> |  | Output only. The list of sources and metadata from the sources of the deployment. |
| `source_project` | String |  | Optional. The project to which the deployment belongs. For GCP gateways, this will refer to the project identifier. For others like Edge/OPDK, this will refer to the org identifier. |
| `parent` | String | ✅ | Required. The parent resource for the deployment resource. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attributes` | HashMap<String, String> | Optional. The list of user defined attributes associated with the deployment resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `source_uri` | String | Optional. The uri where additional source specific information for this deployment can be found. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-source-uri` The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. The value of the attribute should be a valid URI, and in case of Cloud Storage URI, it should point to a Cloud Storage object, not a directory. |
| `documentation` | String | Optional. The documentation of the deployment. |
| `resource_uri` | String | Required. The resource URI identifies the deployment within its gateway. For Apigee gateways, its recommended to use the format: organizations/{org}/environments/{env}/apis/{api}. For ex: if a proxy with name `orders` is deployed in `staging` environment of `cymbal` organization, the resource URI would be: `organizations/cymbal/environments/staging/apis/orders`. |
| `deployment_type` | String | Required. The type of deployment. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-deployment-type` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `management_url` | String | Optional. The uri where users can navigate to for the management of the deployment. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-management-url` The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. The value of the attribute should be a valid URL. |
| `environment` | String | Optional. The environment mapping to this deployment. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-environment` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `name` | String | Identifier. The name of the deployment. Format: `projects/{project}/locations/{location}/deployments/{deployment}` |
| `display_name` | String | Required. The display name of the deployment. |
| `update_time` | String | Output only. The time at which the deployment was last updated. |
| `description` | String | Optional. The description of the deployment. |
| `api_versions` | Vec<String> | Output only. The API versions linked to this deployment. Note: A particular deployment could be linked to multiple different API versions (of same or different APIs). |
| `slo` | String | Optional. The SLO for this deployment. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-slo` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `endpoints` | Vec<String> | Required. The endpoints at which this deployment resource is listening for API requests. This could be a list of complete URIs, hostnames or an IP addresses. |
| `source_environment` | String | Optional. The environment at source for the deployment. For example: prod, dev, staging, etc. |
| `create_time` | String | Output only. The time at which the deployment was created. |
| `source_metadata` | Vec<String> | Output only. The list of sources and metadata from the sources of the deployment. |
| `source_project` | String | Optional. The project to which the deployment belongs. For GCP gateways, this will refer to the project identifier. For others like Edge/OPDK, this will refer to the org identifier. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deployment
deployment = provider.apihub_api.Deployment {
    parent = "value"  # Required. The parent resource for the deployment resource. Format: `projects/{project}/locations/{location}`
}

# Access deployment outputs
deployment_id = deployment.id
deployment_attributes = deployment.attributes
deployment_source_uri = deployment.source_uri
deployment_documentation = deployment.documentation
deployment_resource_uri = deployment.resource_uri
deployment_deployment_type = deployment.deployment_type
deployment_management_url = deployment.management_url
deployment_environment = deployment.environment
deployment_name = deployment.name
deployment_display_name = deployment.display_name
deployment_update_time = deployment.update_time
deployment_description = deployment.description
deployment_api_versions = deployment.api_versions
deployment_slo = deployment.slo
deployment_endpoints = deployment.endpoints
deployment_source_environment = deployment.source_environment
deployment_create_time = deployment.create_time
deployment_source_metadata = deployment.source_metadata
deployment_source_project = deployment.source_project
```

---


### Host_project_registration

Create a host project registration. A Google cloud project can be registered as a host project if it is not attached as a runtime project to another host project. A project can be registered as a host project only once. Subsequent register calls for the same project will fail.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcp_project` | String |  | Required. Immutable. Google cloud project name in the format: "projects/abc" or "projects/123". As input, project name with either project id or number are accepted. As output, this field will contain project number. |
| `create_time` | String |  | Output only. The time at which the host project registration was created. |
| `name` | String |  | Identifier. The name of the host project registration. Format: "projects/{project}/locations/{location}/hostProjectRegistrations/{host_project_registration}". |
| `parent` | String | ✅ | Required. The parent resource for the host project. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gcp_project` | String | Required. Immutable. Google cloud project name in the format: "projects/abc" or "projects/123". As input, project name with either project id or number are accepted. As output, this field will contain project number. |
| `create_time` | String | Output only. The time at which the host project registration was created. |
| `name` | String | Identifier. The name of the host project registration. Format: "projects/{project}/locations/{location}/hostProjectRegistrations/{host_project_registration}". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create host_project_registration
host_project_registration = provider.apihub_api.Host_project_registration {
    parent = "value"  # Required. The parent resource for the host project. Format: `projects/{project}/locations/{location}`
}

# Access host_project_registration outputs
host_project_registration_id = host_project_registration.id
host_project_registration_gcp_project = host_project_registration.gcp_project
host_project_registration_create_time = host_project_registration.create_time
host_project_registration_name = host_project_registration.name
```

---


### Curation

Create a curation resource in the API hub. Once a curation resource is created, plugin instances can start using it.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `plugin_instance_actions` | Vec<String> |  | Output only. The plugin instances and associated actions that are using the curation. Note: A particular curation could be used by multiple plugin instances or multiple actions in a plugin instance. |
| `last_execution_error_code` | String |  | Output only. The error code of the last execution of the curation. The error code is populated only when the last execution state is failed. |
| `display_name` | String |  | Required. The display name of the curation. |
| `last_execution_error_message` | String |  | Output only. Error message describing the failure, if any, during the last execution of the curation. |
| `last_execution_state` | String |  | Output only. The last execution state of the curation. |
| `update_time` | String |  | Output only. The time at which the curation was last updated. |
| `description` | String |  | Optional. The description of the curation. |
| `endpoint` | String |  | Required. The endpoint to be triggered for curation. |
| `create_time` | String |  | Output only. The time at which the curation was created. |
| `name` | String |  | Identifier. The name of the curation. Format: `projects/{project}/locations/{location}/curations/{curation}` |
| `parent` | String | ✅ | Required. The parent resource for the curation resource. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `plugin_instance_actions` | Vec<String> | Output only. The plugin instances and associated actions that are using the curation. Note: A particular curation could be used by multiple plugin instances or multiple actions in a plugin instance. |
| `last_execution_error_code` | String | Output only. The error code of the last execution of the curation. The error code is populated only when the last execution state is failed. |
| `display_name` | String | Required. The display name of the curation. |
| `last_execution_error_message` | String | Output only. Error message describing the failure, if any, during the last execution of the curation. |
| `last_execution_state` | String | Output only. The last execution state of the curation. |
| `update_time` | String | Output only. The time at which the curation was last updated. |
| `description` | String | Optional. The description of the curation. |
| `endpoint` | String | Required. The endpoint to be triggered for curation. |
| `create_time` | String | Output only. The time at which the curation was created. |
| `name` | String | Identifier. The name of the curation. Format: `projects/{project}/locations/{location}/curations/{curation}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create curation
curation = provider.apihub_api.Curation {
    parent = "value"  # Required. The parent resource for the curation resource. Format: `projects/{project}/locations/{location}`
}

# Access curation outputs
curation_id = curation.id
curation_plugin_instance_actions = curation.plugin_instance_actions
curation_last_execution_error_code = curation.last_execution_error_code
curation_display_name = curation.display_name
curation_last_execution_error_message = curation.last_execution_error_message
curation_last_execution_state = curation.last_execution_state
curation_update_time = curation.update_time
curation_description = curation.description
curation_endpoint = curation.endpoint
curation_create_time = curation.create_time
curation_name = curation.name
```

---


### Version

Create an API version for an API resource in the API hub.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `definitions` | Vec<String> |  | Output only. The definitions contained in the API version. These definitions will be added to the version when a new spec is added or when an existing spec is updated. Format is `projects/{project}/locations/{location}/apis/{api}/versions/{version}/definitions/{definition}` |
| `deployments` | Vec<String> |  | Optional. The deployments linked to this API version. Note: A particular API version could be deployed to multiple deployments (for dev deployment, UAT deployment, etc) Format is `projects/{project}/locations/{location}/deployments/{deployment}` |
| `specs` | Vec<String> |  | Output only. The specs associated with this version. Note that an API version can be associated with multiple specs. Format is `projects/{project}/locations/{location}/apis/{api}/versions/{version}/specs/{spec}` |
| `source_metadata` | Vec<String> |  | Output only. The list of sources and metadata from the sources of the version. |
| `name` | String |  | Identifier. The name of the version. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}` |
| `lifecycle` | String |  | Optional. The lifecycle of the API version. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-lifecycle` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `selected_deployment` | String |  | Optional. The selected deployment for a Version resource. This can be used when special handling is needed on client side for a particular deployment linked to the version. Format is `projects/{project}/locations/{location}/deployments/{deployment}` |
| `accreditation` | String |  | Optional. The accreditations associated with the API version. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-accreditation` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `attributes` | HashMap<String, String> |  | Optional. The list of user defined attributes associated with the Version resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `description` | String |  | Optional. The description of the version. |
| `update_time` | String |  | Output only. The time at which the version was last updated. |
| `compliance` | String |  | Optional. The compliance associated with the API version. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-compliance` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `display_name` | String |  | Required. The display name of the version. |
| `documentation` | String |  | Optional. The documentation of the version. |
| `create_time` | String |  | Output only. The time at which the version was created. |
| `api_operations` | Vec<String> |  | Output only. The operations contained in the API version. These operations will be added to the version when a new spec is added or when an existing spec is updated. Format is `projects/{project}/locations/{location}/apis/{api}/versions/{version}/operations/{operation}` |
| `parent` | String | ✅ | Required. The parent resource for API version. Format: `projects/{project}/locations/{location}/apis/{api}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `definitions` | Vec<String> | Output only. The definitions contained in the API version. These definitions will be added to the version when a new spec is added or when an existing spec is updated. Format is `projects/{project}/locations/{location}/apis/{api}/versions/{version}/definitions/{definition}` |
| `deployments` | Vec<String> | Optional. The deployments linked to this API version. Note: A particular API version could be deployed to multiple deployments (for dev deployment, UAT deployment, etc) Format is `projects/{project}/locations/{location}/deployments/{deployment}` |
| `specs` | Vec<String> | Output only. The specs associated with this version. Note that an API version can be associated with multiple specs. Format is `projects/{project}/locations/{location}/apis/{api}/versions/{version}/specs/{spec}` |
| `source_metadata` | Vec<String> | Output only. The list of sources and metadata from the sources of the version. |
| `name` | String | Identifier. The name of the version. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}` |
| `lifecycle` | String | Optional. The lifecycle of the API version. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-lifecycle` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `selected_deployment` | String | Optional. The selected deployment for a Version resource. This can be used when special handling is needed on client side for a particular deployment linked to the version. Format is `projects/{project}/locations/{location}/deployments/{deployment}` |
| `accreditation` | String | Optional. The accreditations associated with the API version. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-accreditation` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `attributes` | HashMap<String, String> | Optional. The list of user defined attributes associated with the Version resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `description` | String | Optional. The description of the version. |
| `update_time` | String | Output only. The time at which the version was last updated. |
| `compliance` | String | Optional. The compliance associated with the API version. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-compliance` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `display_name` | String | Required. The display name of the version. |
| `documentation` | String | Optional. The documentation of the version. |
| `create_time` | String | Output only. The time at which the version was created. |
| `api_operations` | Vec<String> | Output only. The operations contained in the API version. These operations will be added to the version when a new spec is added or when an existing spec is updated. Format is `projects/{project}/locations/{location}/apis/{api}/versions/{version}/operations/{operation}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.apihub_api.Version {
    parent = "value"  # Required. The parent resource for API version. Format: `projects/{project}/locations/{location}/apis/{api}`
}

# Access version outputs
version_id = version.id
version_definitions = version.definitions
version_deployments = version.deployments
version_specs = version.specs
version_source_metadata = version.source_metadata
version_name = version.name
version_lifecycle = version.lifecycle
version_selected_deployment = version.selected_deployment
version_accreditation = version.accreditation
version_attributes = version.attributes
version_description = version.description
version_update_time = version.update_time
version_compliance = version.compliance
version_display_name = version.display_name
version_documentation = version.documentation
version_create_time = version.create_time
version_api_operations = version.api_operations
```

---


### Api

Create an API resource in the API hub. Once an API resource is created, versions can be added to it.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_style` | String |  | Optional. The style of the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-api-style` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `description` | String |  | Optional. The description of the API resource. |
| `documentation` | String |  | Optional. The documentation for the API resource. |
| `api_functional_requirements` | String |  | Optional. The api functional requirements associated with the API resource. Carinality is 1 for this attribute. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-api-functional-requirements` attribute. The value of the attribute should be a proper URI, and in case of Cloud Storage URI, it should point to a Cloud Storage object, not a directory. |
| `name` | String |  | Identifier. The name of the API resource in the API Hub. Format: `projects/{project}/locations/{location}/apis/{api}` |
| `api_requirements` | String |  | Optional. The api requirement doc associated with the API resource. Carinality is 1 for this attribute. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-api-requirements` attribute. The value of the attribute should be a proper URI, and in case of Cloud Storage URI, it should point to a Cloud Storage object, not a directory. |
| `display_name` | String |  | Required. The display name of the API resource. |
| `versions` | Vec<String> |  | Output only. The list of versions present in an API resource. Note: An API resource can be associated with more than 1 version. Format is `projects/{project}/locations/{location}/apis/{api}/versions/{version}` |
| `selected_version` | String |  | Optional. The selected version for an API resource. This can be used when special handling is needed on client side for particular version of the API. Format is `projects/{project}/locations/{location}/apis/{api}/versions/{version}` |
| `fingerprint` | String |  | Optional. Fingerprint of the API resource. This must be unique for each API resource. It can neither be unset nor be updated to an existing fingerprint of another API resource. |
| `source_metadata` | Vec<String> |  | Output only. The list of sources and metadata from the sources of the API resource. |
| `business_unit` | String |  | Optional. The business unit owning the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-business-unit` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `update_time` | String |  | Output only. The time at which the API resource was last updated. |
| `attributes` | HashMap<String, String> |  | Optional. The list of user defined attributes associated with the API resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `create_time` | String |  | Output only. The time at which the API resource was created. |
| `maturity_level` | String |  | Optional. The maturity level of the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-maturity-level` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `target_user` | String |  | Optional. The target users for the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-target-user` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `owner` | String |  | Optional. Owner details for the API resource. |
| `team` | String |  | Optional. The team owning the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-team` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `api_technical_requirements` | String |  | Optional. The api technical requirements associated with the API resource. Carinality is 1 for this attribute. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-api-technical-requirements` attribute. The value of the attribute should be a proper URI, and in case of Cloud Storage URI, it should point to a Cloud Storage object, not a directory. |
| `parent` | String | ✅ | Required. The parent resource for the API resource. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_style` | String | Optional. The style of the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-api-style` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `description` | String | Optional. The description of the API resource. |
| `documentation` | String | Optional. The documentation for the API resource. |
| `api_functional_requirements` | String | Optional. The api functional requirements associated with the API resource. Carinality is 1 for this attribute. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-api-functional-requirements` attribute. The value of the attribute should be a proper URI, and in case of Cloud Storage URI, it should point to a Cloud Storage object, not a directory. |
| `name` | String | Identifier. The name of the API resource in the API Hub. Format: `projects/{project}/locations/{location}/apis/{api}` |
| `api_requirements` | String | Optional. The api requirement doc associated with the API resource. Carinality is 1 for this attribute. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-api-requirements` attribute. The value of the attribute should be a proper URI, and in case of Cloud Storage URI, it should point to a Cloud Storage object, not a directory. |
| `display_name` | String | Required. The display name of the API resource. |
| `versions` | Vec<String> | Output only. The list of versions present in an API resource. Note: An API resource can be associated with more than 1 version. Format is `projects/{project}/locations/{location}/apis/{api}/versions/{version}` |
| `selected_version` | String | Optional. The selected version for an API resource. This can be used when special handling is needed on client side for particular version of the API. Format is `projects/{project}/locations/{location}/apis/{api}/versions/{version}` |
| `fingerprint` | String | Optional. Fingerprint of the API resource. This must be unique for each API resource. It can neither be unset nor be updated to an existing fingerprint of another API resource. |
| `source_metadata` | Vec<String> | Output only. The list of sources and metadata from the sources of the API resource. |
| `business_unit` | String | Optional. The business unit owning the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-business-unit` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `update_time` | String | Output only. The time at which the API resource was last updated. |
| `attributes` | HashMap<String, String> | Optional. The list of user defined attributes associated with the API resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `create_time` | String | Output only. The time at which the API resource was created. |
| `maturity_level` | String | Optional. The maturity level of the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-maturity-level` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `target_user` | String | Optional. The target users for the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-target-user` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `owner` | String | Optional. Owner details for the API resource. |
| `team` | String | Optional. The team owning the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-team` attribute. The number of values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. |
| `api_technical_requirements` | String | Optional. The api technical requirements associated with the API resource. Carinality is 1 for this attribute. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-api-technical-requirements` attribute. The value of the attribute should be a proper URI, and in case of Cloud Storage URI, it should point to a Cloud Storage object, not a directory. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create api
api = provider.apihub_api.Api {
    parent = "value"  # Required. The parent resource for the API resource. Format: `projects/{project}/locations/{location}`
}

# Access api outputs
api_id = api.id
api_api_style = api.api_style
api_description = api.description
api_documentation = api.documentation
api_api_functional_requirements = api.api_functional_requirements
api_name = api.name
api_api_requirements = api.api_requirements
api_display_name = api.display_name
api_versions = api.versions
api_selected_version = api.selected_version
api_fingerprint = api.fingerprint
api_source_metadata = api.source_metadata
api_business_unit = api.business_unit
api_update_time = api.update_time
api_attributes = api.attributes
api_create_time = api.create_time
api_maturity_level = api.maturity_level
api_target_user = api.target_user
api_owner = api.owner
api_team = api.team
api_api_technical_requirements = api.api_technical_requirements
```

---


### Plugin

Create an API Hub plugin resource in the API hub. Once a plugin is created, it can be used to create plugin instances.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The name of the plugin. Format: `projects/{project}/locations/{location}/plugins/{plugin}` |
| `actions_config` | Vec<String> |  | Optional. The configuration of actions supported by the plugin. **REQUIRED**: This field must be provided when creating or updating a Plugin. The server will reject requests if this field is missing. |
| `config_template` | String |  | Optional. The configuration template for the plugin. |
| `gateway_type` | String |  | Optional. The type of the gateway. |
| `state` | String |  | Output only. Represents the state of the plugin. Note this field will not be set for plugins developed via plugin framework as the state will be managed at plugin instance level. |
| `update_time` | String |  | Output only. Timestamp indicating when the plugin was last updated. |
| `create_time` | String |  | Output only. Timestamp indicating when the plugin was created. |
| `hosting_service` | String |  | Optional. This field is optional. It is used to notify the plugin hosting service for any lifecycle changes of the plugin instance and trigger execution of plugin instance actions in case of API hub managed actions. This field should be provided if the plugin instance lifecycle of the developed plugin needs to be managed from API hub. Also, in this case the plugin hosting service interface needs to be implemented. This field should not be provided if the plugin wants to manage plugin instance lifecycle events outside of hub interface and use plugin framework for only registering of plugin and plugin instances to capture the source of data into hub. Note, in this case the plugin hosting service interface is not required to be implemented. Also, the plugin instance lifecycle actions will be disabled from API hub's UI. |
| `ownership_type` | String |  | Output only. The type of the plugin, indicating whether it is 'SYSTEM_OWNED' or 'USER_OWNED'. |
| `display_name` | String |  | Required. The display name of the plugin. Max length is 50 characters (Unicode code points). |
| `type` | String |  | Optional. The type of the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-plugin-type` attribute. The number of allowed values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. Note this field is not required for plugins developed via plugin framework. |
| `documentation` | String |  | Optional. The documentation of the plugin, that explains how to set up and use the plugin. |
| `plugin_category` | String |  | Optional. The category of the plugin, identifying its primary category or purpose. This field is required for all plugins. |
| `description` | String |  | Optional. The plugin description. Max length is 2000 characters (Unicode code points). |
| `parent` | String | ✅ | Required. The parent resource where this plugin will be created. Format: `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The name of the plugin. Format: `projects/{project}/locations/{location}/plugins/{plugin}` |
| `actions_config` | Vec<String> | Optional. The configuration of actions supported by the plugin. **REQUIRED**: This field must be provided when creating or updating a Plugin. The server will reject requests if this field is missing. |
| `config_template` | String | Optional. The configuration template for the plugin. |
| `gateway_type` | String | Optional. The type of the gateway. |
| `state` | String | Output only. Represents the state of the plugin. Note this field will not be set for plugins developed via plugin framework as the state will be managed at plugin instance level. |
| `update_time` | String | Output only. Timestamp indicating when the plugin was last updated. |
| `create_time` | String | Output only. Timestamp indicating when the plugin was created. |
| `hosting_service` | String | Optional. This field is optional. It is used to notify the plugin hosting service for any lifecycle changes of the plugin instance and trigger execution of plugin instance actions in case of API hub managed actions. This field should be provided if the plugin instance lifecycle of the developed plugin needs to be managed from API hub. Also, in this case the plugin hosting service interface needs to be implemented. This field should not be provided if the plugin wants to manage plugin instance lifecycle events outside of hub interface and use plugin framework for only registering of plugin and plugin instances to capture the source of data into hub. Note, in this case the plugin hosting service interface is not required to be implemented. Also, the plugin instance lifecycle actions will be disabled from API hub's UI. |
| `ownership_type` | String | Output only. The type of the plugin, indicating whether it is 'SYSTEM_OWNED' or 'USER_OWNED'. |
| `display_name` | String | Required. The display name of the plugin. Max length is 50 characters (Unicode code points). |
| `type` | String | Optional. The type of the API. This maps to the following system defined attribute: `projects/{project}/locations/{location}/attributes/system-plugin-type` attribute. The number of allowed values for this attribute will be based on the cardinality of the attribute. The same can be retrieved via GetAttribute API. All values should be from the list of allowed values defined for the attribute. Note this field is not required for plugins developed via plugin framework. |
| `documentation` | String | Optional. The documentation of the plugin, that explains how to set up and use the plugin. |
| `plugin_category` | String | Optional. The category of the plugin, identifying its primary category or purpose. This field is required for all plugins. |
| `description` | String | Optional. The plugin description. Max length is 2000 characters (Unicode code points). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create plugin
plugin = provider.apihub_api.Plugin {
    parent = "value"  # Required. The parent resource where this plugin will be created. Format: `projects/{project}/locations/{location}`.
}

# Access plugin outputs
plugin_id = plugin.id
plugin_name = plugin.name
plugin_actions_config = plugin.actions_config
plugin_config_template = plugin.config_template
plugin_gateway_type = plugin.gateway_type
plugin_state = plugin.state
plugin_update_time = plugin.update_time
plugin_create_time = plugin.create_time
plugin_hosting_service = plugin.hosting_service
plugin_ownership_type = plugin.ownership_type
plugin_display_name = plugin.display_name
plugin_type = plugin.type
plugin_documentation = plugin.documentation
plugin_plugin_category = plugin.plugin_category
plugin_description = plugin.description
```

---


### Dependencie

Create a dependency between two entities in the API hub.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `consumer` | String |  | Required. Immutable. The entity acting as the consumer in the dependency. |
| `supplier` | String |  | Required. Immutable. The entity acting as the supplier in the dependency. |
| `update_time` | String |  | Output only. The time at which the dependency was last updated. |
| `description` | String |  | Optional. Human readable description corresponding of the dependency. |
| `discovery_mode` | String |  | Output only. Discovery mode of the dependency. |
| `name` | String |  | Identifier. The name of the dependency in the API Hub. Format: `projects/{project}/locations/{location}/dependencies/{dependency}` |
| `error_detail` | String |  | Output only. Error details of a dependency if the system has detected it internally. |
| `state` | String |  | Output only. State of the dependency. |
| `attributes` | HashMap<String, String> |  | Optional. The list of user defined attributes associated with the dependency resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `create_time` | String |  | Output only. The time at which the dependency was created. |
| `parent` | String | ✅ | Required. The parent resource for the dependency resource. Format: `projects/{project}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `consumer` | String | Required. Immutable. The entity acting as the consumer in the dependency. |
| `supplier` | String | Required. Immutable. The entity acting as the supplier in the dependency. |
| `update_time` | String | Output only. The time at which the dependency was last updated. |
| `description` | String | Optional. Human readable description corresponding of the dependency. |
| `discovery_mode` | String | Output only. Discovery mode of the dependency. |
| `name` | String | Identifier. The name of the dependency in the API Hub. Format: `projects/{project}/locations/{location}/dependencies/{dependency}` |
| `error_detail` | String | Output only. Error details of a dependency if the system has detected it internally. |
| `state` | String | Output only. State of the dependency. |
| `attributes` | HashMap<String, String> | Optional. The list of user defined attributes associated with the dependency resource. The key is the attribute name. It will be of the format: `projects/{project}/locations/{location}/attributes/{attribute}`. The value is the attribute values associated with the resource. |
| `create_time` | String | Output only. The time at which the dependency was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dependencie
dependencie = provider.apihub_api.Dependencie {
    parent = "value"  # Required. The parent resource for the dependency resource. Format: `projects/{project}/locations/{location}`
}

# Access dependencie outputs
dependencie_id = dependencie.id
dependencie_consumer = dependencie.consumer
dependencie_supplier = dependencie.supplier
dependencie_update_time = dependencie.update_time
dependencie_description = dependencie.description
dependencie_discovery_mode = dependencie.discovery_mode
dependencie_name = dependencie.name
dependencie_error_detail = dependencie.error_detail
dependencie_state = dependencie.state
dependencie_attributes = dependencie.attributes
dependencie_create_time = dependencie.create_time
```

---


### Location

Collect API data from a source and push it to Hub's collect layer.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `plugin_instance` | String |  | Required. The plugin instance collecting the API data. Format: `projects/{project}/locations/{location}/plugins/{plugin}/instances/{instance}`. |
| `collection_type` | String |  | Required. The type of collection. Applies to all entries in api_data. |
| `action_id` | String |  | Required. The action ID to be used for collecting the API data. This should map to one of the action IDs specified in action configs in the plugin. |
| `api_data` | String |  | Required. The API data to be collected. |
| `location` | String | ✅ | Required. The regional location of the API hub instance and its resources. Format: `projects/{project}/locations/{location}` |


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

# Create location
location = provider.apihub_api.Location {
    location = "value"  # Required. The regional location of the API hub instance and its resources. Format: `projects/{project}/locations/{location}`
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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple instance resources
instance_0 = provider.apihub_api.Instance {
    parent = "value-0"
}
instance_1 = provider.apihub_api.Instance {
    parent = "value-1"
}
instance_2 = provider.apihub_api.Instance {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    instance = provider.apihub_api.Instance {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Apihub_api Documentation](https://cloud.google.com/apihub_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
