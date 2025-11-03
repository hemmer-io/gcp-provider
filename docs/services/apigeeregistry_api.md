# Apigeeregistry_api Service



**Resources**: 10

---

## Overview

The apigeeregistry_api service provides access to 10 resource types:

- [Version](#version) [CRUD]
- [Spec](#spec) [CRUD]
- [Location](#location) [R]
- [Deployment](#deployment) [CRUD]
- [Runtime](#runtime) [CR]
- [Api](#api) [CRUD]
- [Document](#document) [CR]
- [Instance](#instance) [CRD]
- [Operation](#operation) [CRD]
- [Artifact](#artifact) [CRUD]

---

## Resources


### Version

Creates a specified version.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Human-meaningful name. |
| `labels` | HashMap<String, String> |  | Labels attach identifying metadata to resources. Identifying metadata can be used to filter list operations. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one resource (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `apigeeregistry.googleapis.com/` and cannot be changed. |
| `description` | String |  | A detailed description. |
| `primary_spec` | String |  | The primary spec for this version. Format: projects/{project}/locations/{location}/apis/{api}/versions/{version}/specs/{spec} |
| `create_time` | String |  | Output only. Creation timestamp. |
| `state` | String |  | A user-definable description of the lifecycle phase of this API version. Format: free-form, but we expect single words that describe API maturity, e.g., "CONCEPT", "DESIGN", "DEVELOPMENT", "STAGING", "PRODUCTION", "DEPRECATED", "RETIRED". |
| `update_time` | String |  | Output only. Last update timestamp. |
| `name` | String |  | Resource name. |
| `annotations` | HashMap<String, String> |  | Annotations attach non-identifying metadata to resources. Annotation keys and values are less restricted than those of labels, but should be generally used for small values of broad interest. Larger, topic- specific metadata should be stored in Artifacts. |
| `parent` | String | ✅ | Required. The parent, which owns this collection of versions. Format: `projects/*/locations/*/apis/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Human-meaningful name. |
| `labels` | HashMap<String, String> | Labels attach identifying metadata to resources. Identifying metadata can be used to filter list operations. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one resource (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `apigeeregistry.googleapis.com/` and cannot be changed. |
| `description` | String | A detailed description. |
| `primary_spec` | String | The primary spec for this version. Format: projects/{project}/locations/{location}/apis/{api}/versions/{version}/specs/{spec} |
| `create_time` | String | Output only. Creation timestamp. |
| `state` | String | A user-definable description of the lifecycle phase of this API version. Format: free-form, but we expect single words that describe API maturity, e.g., "CONCEPT", "DESIGN", "DEVELOPMENT", "STAGING", "PRODUCTION", "DEPRECATED", "RETIRED". |
| `update_time` | String | Output only. Last update timestamp. |
| `name` | String | Resource name. |
| `annotations` | HashMap<String, String> | Annotations attach non-identifying metadata to resources. Annotation keys and values are less restricted than those of labels, but should be generally used for small values of broad interest. Larger, topic- specific metadata should be stored in Artifacts. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.apigeeregistry_api.Version {
    parent = "value"  # Required. The parent, which owns this collection of versions. Format: `projects/*/locations/*/apis/*`
}

# Access version outputs
version_id = version.id
version_display_name = version.display_name
version_labels = version.labels
version_description = version.description
version_primary_spec = version.primary_spec
version_create_time = version.create_time
version_state = version.state
version_update_time = version.update_time
version_name = version.name
version_annotations = version.annotations
```

---


### Spec

Creates a specified spec.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filename` | String |  | A possibly-hierarchical name used to refer to the spec from other specs. |
| `name` | String |  | Resource name. |
| `create_time` | String |  | Output only. Creation timestamp; when the spec resource was created. |
| `annotations` | HashMap<String, String> |  | Annotations attach non-identifying metadata to resources. Annotation keys and values are less restricted than those of labels, but should be generally used for small values of broad interest. Larger, topic- specific metadata should be stored in Artifacts. |
| `labels` | HashMap<String, String> |  | Labels attach identifying metadata to resources. Identifying metadata can be used to filter list operations. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one resource (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `apigeeregistry.googleapis.com/` and cannot be changed. |
| `mime_type` | String |  | A style (format) descriptor for this spec that is specified as a [Media Type](https://en.wikipedia.org/wiki/Media_type). Possible values include `application/vnd.apigee.proto`, `application/vnd.apigee.openapi`, and `application/vnd.apigee.graphql`, with possible suffixes representing compression types. These hypothetical names are defined in the vendor tree defined in RFC6838 (https://tools.ietf.org/html/rfc6838) and are not final. Content types can specify compression. Currently only GZip compression is supported (indicated with "+gzip"). |
| `revision_create_time` | String |  | Output only. Revision creation timestamp; when the represented revision was created. |
| `size_bytes` | i64 |  | Output only. The size of the spec file in bytes. If the spec is gzipped, this is the size of the uncompressed spec. |
| `source_uri` | String |  | The original source URI of the spec (if one exists). This is an external location that can be used for reference purposes but which may not be authoritative since this external resource may change after the spec is retrieved. |
| `description` | String |  | A detailed description. |
| `revision_id` | String |  | Output only. Immutable. The revision ID of the spec. A new revision is committed whenever the spec contents are changed. The format is an 8-character hexadecimal string. |
| `contents` | String |  | Input only. The contents of the spec. Provided by API callers when specs are created or updated. To access the contents of a spec, use GetApiSpecContents. |
| `revision_update_time` | String |  | Output only. Last update timestamp: when the represented revision was last modified. |
| `hash` | String |  | Output only. A SHA-256 hash of the spec's contents. If the spec is gzipped, this is the hash of the uncompressed spec. |
| `parent` | String | ✅ | Required. The parent, which owns this collection of specs. Format: `projects/*/locations/*/apis/*/versions/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `filename` | String | A possibly-hierarchical name used to refer to the spec from other specs. |
| `name` | String | Resource name. |
| `create_time` | String | Output only. Creation timestamp; when the spec resource was created. |
| `annotations` | HashMap<String, String> | Annotations attach non-identifying metadata to resources. Annotation keys and values are less restricted than those of labels, but should be generally used for small values of broad interest. Larger, topic- specific metadata should be stored in Artifacts. |
| `labels` | HashMap<String, String> | Labels attach identifying metadata to resources. Identifying metadata can be used to filter list operations. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one resource (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `apigeeregistry.googleapis.com/` and cannot be changed. |
| `mime_type` | String | A style (format) descriptor for this spec that is specified as a [Media Type](https://en.wikipedia.org/wiki/Media_type). Possible values include `application/vnd.apigee.proto`, `application/vnd.apigee.openapi`, and `application/vnd.apigee.graphql`, with possible suffixes representing compression types. These hypothetical names are defined in the vendor tree defined in RFC6838 (https://tools.ietf.org/html/rfc6838) and are not final. Content types can specify compression. Currently only GZip compression is supported (indicated with "+gzip"). |
| `revision_create_time` | String | Output only. Revision creation timestamp; when the represented revision was created. |
| `size_bytes` | i64 | Output only. The size of the spec file in bytes. If the spec is gzipped, this is the size of the uncompressed spec. |
| `source_uri` | String | The original source URI of the spec (if one exists). This is an external location that can be used for reference purposes but which may not be authoritative since this external resource may change after the spec is retrieved. |
| `description` | String | A detailed description. |
| `revision_id` | String | Output only. Immutable. The revision ID of the spec. A new revision is committed whenever the spec contents are changed. The format is an 8-character hexadecimal string. |
| `contents` | String | Input only. The contents of the spec. Provided by API callers when specs are created or updated. To access the contents of a spec, use GetApiSpecContents. |
| `revision_update_time` | String | Output only. Last update timestamp: when the represented revision was last modified. |
| `hash` | String | Output only. A SHA-256 hash of the spec's contents. If the spec is gzipped, this is the hash of the uncompressed spec. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create spec
spec = provider.apigeeregistry_api.Spec {
    parent = "value"  # Required. The parent, which owns this collection of specs. Format: `projects/*/locations/*/apis/*/versions/*`
}

# Access spec outputs
spec_id = spec.id
spec_filename = spec.filename
spec_name = spec.name
spec_create_time = spec.create_time
spec_annotations = spec.annotations
spec_labels = spec.labels
spec_mime_type = spec.mime_type
spec_revision_create_time = spec.revision_create_time
spec_size_bytes = spec.size_bytes
spec_source_uri = spec.source_uri
spec_description = spec.description
spec_revision_id = spec.revision_id
spec_contents = spec.contents
spec_revision_update_time = spec.revision_update_time
spec_hash = spec.hash
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_labels = location.labels
location_display_name = location.display_name
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
```

---


### Deployment

Creates a specified deployment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Labels attach identifying metadata to resources. Identifying metadata can be used to filter list operations. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one resource (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `apigeeregistry.googleapis.com/` and cannot be changed. |
| `name` | String |  | Resource name. |
| `revision_update_time` | String |  | Output only. Last update timestamp: when the represented revision was last modified. |
| `revision_create_time` | String |  | Output only. Revision creation timestamp; when the represented revision was created. |
| `api_spec_revision` | String |  | The full resource name (including revision ID) of the spec of the API being served by the deployment. Changes to this value will update the revision. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}/specs/{spec@revision}` |
| `annotations` | HashMap<String, String> |  | Annotations attach non-identifying metadata to resources. Annotation keys and values are less restricted than those of labels, but should be generally used for small values of broad interest. Larger, topic- specific metadata should be stored in Artifacts. |
| `external_channel_uri` | String |  | The address of the external channel of the API (e.g., the Developer Portal). Changes to this value will not affect the revision. |
| `revision_id` | String |  | Output only. Immutable. The revision ID of the deployment. A new revision is committed whenever the deployment contents are changed. The format is an 8-character hexadecimal string. |
| `description` | String |  | A detailed description. |
| `access_guidance` | String |  | Text briefly describing how to access the endpoint. Changes to this value will not affect the revision. |
| `display_name` | String |  | Human-meaningful name. |
| `intended_audience` | String |  | Text briefly identifying the intended audience of the API. Changes to this value will not affect the revision. |
| `create_time` | String |  | Output only. Creation timestamp; when the deployment resource was created. |
| `endpoint_uri` | String |  | The address where the deployment is serving. Changes to this value will update the revision. |
| `parent` | String | ✅ | Required. The parent, which owns this collection of deployments. Format: `projects/*/locations/*/apis/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Labels attach identifying metadata to resources. Identifying metadata can be used to filter list operations. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one resource (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `apigeeregistry.googleapis.com/` and cannot be changed. |
| `name` | String | Resource name. |
| `revision_update_time` | String | Output only. Last update timestamp: when the represented revision was last modified. |
| `revision_create_time` | String | Output only. Revision creation timestamp; when the represented revision was created. |
| `api_spec_revision` | String | The full resource name (including revision ID) of the spec of the API being served by the deployment. Changes to this value will update the revision. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}/specs/{spec@revision}` |
| `annotations` | HashMap<String, String> | Annotations attach non-identifying metadata to resources. Annotation keys and values are less restricted than those of labels, but should be generally used for small values of broad interest. Larger, topic- specific metadata should be stored in Artifacts. |
| `external_channel_uri` | String | The address of the external channel of the API (e.g., the Developer Portal). Changes to this value will not affect the revision. |
| `revision_id` | String | Output only. Immutable. The revision ID of the deployment. A new revision is committed whenever the deployment contents are changed. The format is an 8-character hexadecimal string. |
| `description` | String | A detailed description. |
| `access_guidance` | String | Text briefly describing how to access the endpoint. Changes to this value will not affect the revision. |
| `display_name` | String | Human-meaningful name. |
| `intended_audience` | String | Text briefly identifying the intended audience of the API. Changes to this value will not affect the revision. |
| `create_time` | String | Output only. Creation timestamp; when the deployment resource was created. |
| `endpoint_uri` | String | The address where the deployment is serving. Changes to this value will update the revision. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deployment
deployment = provider.apigeeregistry_api.Deployment {
    parent = "value"  # Required. The parent, which owns this collection of deployments. Format: `projects/*/locations/*/apis/*`
}

# Access deployment outputs
deployment_id = deployment.id
deployment_labels = deployment.labels
deployment_name = deployment.name
deployment_revision_update_time = deployment.revision_update_time
deployment_revision_create_time = deployment.revision_create_time
deployment_api_spec_revision = deployment.api_spec_revision
deployment_annotations = deployment.annotations
deployment_external_channel_uri = deployment.external_channel_uri
deployment_revision_id = deployment.revision_id
deployment_description = deployment.description
deployment_access_guidance = deployment.access_guidance
deployment_display_name = deployment.display_name
deployment_intended_audience = deployment.intended_audience
deployment_create_time = deployment.create_time
deployment_endpoint_uri = deployment.endpoint_uri
```

---


### Runtime

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create runtime
runtime = provider.apigeeregistry_api.Runtime {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access runtime outputs
runtime_id = runtime.id
runtime_etag = runtime.etag
runtime_version = runtime.version
runtime_bindings = runtime.bindings
```

---


### Api

Creates a specified API.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | A detailed description. |
| `display_name` | String |  | Human-meaningful name. |
| `labels` | HashMap<String, String> |  | Labels attach identifying metadata to resources. Identifying metadata can be used to filter list operations. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores, and dashes. International characters are allowed. No more than 64 user labels can be associated with one resource (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `apigeeregistry.googleapis.com/` and cannot be changed. |
| `create_time` | String |  | Output only. Creation timestamp. |
| `name` | String |  | Resource name. |
| `availability` | String |  | A user-definable description of the availability of this service. Format: free-form, but we expect single words that describe availability, e.g., "NONE", "TESTING", "PREVIEW", "GENERAL", "DEPRECATED", "SHUTDOWN". |
| `recommended_deployment` | String |  | The recommended deployment of the API. Format: `projects/{project}/locations/{location}/apis/{api}/deployments/{deployment}` |
| `recommended_version` | String |  | The recommended version of the API. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}` |
| `annotations` | HashMap<String, String> |  | Annotations attach non-identifying metadata to resources. Annotation keys and values are less restricted than those of labels, but should be generally used for small values of broad interest. Larger, topic- specific metadata should be stored in Artifacts. |
| `update_time` | String |  | Output only. Last update timestamp. |
| `parent` | String | ✅ | Required. The parent, which owns this collection of APIs. Format: `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | A detailed description. |
| `display_name` | String | Human-meaningful name. |
| `labels` | HashMap<String, String> | Labels attach identifying metadata to resources. Identifying metadata can be used to filter list operations. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores, and dashes. International characters are allowed. No more than 64 user labels can be associated with one resource (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with `apigeeregistry.googleapis.com/` and cannot be changed. |
| `create_time` | String | Output only. Creation timestamp. |
| `name` | String | Resource name. |
| `availability` | String | A user-definable description of the availability of this service. Format: free-form, but we expect single words that describe availability, e.g., "NONE", "TESTING", "PREVIEW", "GENERAL", "DEPRECATED", "SHUTDOWN". |
| `recommended_deployment` | String | The recommended deployment of the API. Format: `projects/{project}/locations/{location}/apis/{api}/deployments/{deployment}` |
| `recommended_version` | String | The recommended version of the API. Format: `projects/{project}/locations/{location}/apis/{api}/versions/{version}` |
| `annotations` | HashMap<String, String> | Annotations attach non-identifying metadata to resources. Annotation keys and values are less restricted than those of labels, but should be generally used for small values of broad interest. Larger, topic- specific metadata should be stored in Artifacts. |
| `update_time` | String | Output only. Last update timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create api
api = provider.apigeeregistry_api.Api {
    parent = "value"  # Required. The parent, which owns this collection of APIs. Format: `projects/*/locations/*`
}

# Access api outputs
api_id = api.id
api_description = api.description
api_display_name = api.display_name
api_labels = api.labels
api_create_time = api.create_time
api_name = api.name
api_availability = api.availability
api_recommended_deployment = api.recommended_deployment
api_recommended_version = api.recommended_version
api_annotations = api.annotations
api_update_time = api.update_time
```

---


### Document

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create document
document = provider.apigeeregistry_api.Document {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access document outputs
document_id = document.id
document_etag = document.etag
document_version = document.version
document_bindings = document.bindings
```

---


### Instance

Provisions instance resources for the Registry.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The current state of the Instance. |
| `create_time` | String |  | Output only. Creation timestamp. |
| `update_time` | String |  | Output only. Last update timestamp. |
| `build` | String |  | Output only. Build info of the Instance if it's in `ACTIVE` state. |
| `config` | String |  | Required. Config of the Instance. |
| `name` | String |  | Format: `projects/*/locations/*/instance`. Currently only `locations/global` is supported. |
| `state_message` | String |  | Output only. Extra information of Instance.State if the state is `FAILED`. |
| `parent` | String | ✅ | Required. Parent resource of the Instance, of the form: `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The current state of the Instance. |
| `create_time` | String | Output only. Creation timestamp. |
| `update_time` | String | Output only. Last update timestamp. |
| `build` | String | Output only. Build info of the Instance if it's in `ACTIVE` state. |
| `config` | String | Required. Config of the Instance. |
| `name` | String | Format: `projects/*/locations/*/instance`. Currently only `locations/global` is supported. |
| `state_message` | String | Output only. Extra information of Instance.State if the state is `FAILED`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.apigeeregistry_api.Instance {
    parent = "value"  # Required. Parent resource of the Instance, of the form: `projects/*/locations/*`
}

# Access instance outputs
instance_id = instance.id
instance_state = instance.state
instance_create_time = instance.create_time
instance_update_time = instance.update_time
instance_build = instance.build
instance_config = instance.config
instance_name = instance.name
instance_state_message = instance.state_message
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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

# Create operation
operation = provider.apigeeregistry_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_error = operation.error
operation_response = operation.response
```

---


### Artifact

Creates a specified artifact.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Labels attach identifying metadata to resources. Identifying metadata can be used to filter list operations. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one resource (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "registry.googleapis.com/" and cannot be changed. |
| `name` | String |  | Resource name. |
| `update_time` | String |  | Output only. Last update timestamp. |
| `mime_type` | String |  | A content type specifier for the artifact. Content type specifiers are Media Types (https://en.wikipedia.org/wiki/Media_type) with a possible "schema" parameter that specifies a schema for the stored information. Content types can specify compression. Currently only GZip compression is supported (indicated with "+gzip"). |
| `hash` | String |  | Output only. A SHA-256 hash of the artifact's contents. If the artifact is gzipped, this is the hash of the uncompressed artifact. |
| `contents` | String |  | Input only. The contents of the artifact. Provided by API callers when artifacts are created or replaced. To access the contents of an artifact, use GetArtifactContents. |
| `size_bytes` | i64 |  | Output only. The size of the artifact in bytes. If the artifact is gzipped, this is the size of the uncompressed artifact. |
| `annotations` | HashMap<String, String> |  | Annotations attach non-identifying metadata to resources. Annotation keys and values are less restricted than those of labels, but should be generally used for small values of broad interest. Larger, topic- specific metadata should be stored in Artifacts. |
| `create_time` | String |  | Output only. Creation timestamp. |
| `parent` | String | ✅ | Required. The parent, which owns this collection of artifacts. Format: `{parent}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Labels attach identifying metadata to resources. Identifying metadata can be used to filter list operations. Label keys and values can be no longer than 64 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. No more than 64 user labels can be associated with one resource (System labels are excluded). See https://goo.gl/xmQnxf for more information and examples of labels. System reserved label keys are prefixed with "registry.googleapis.com/" and cannot be changed. |
| `name` | String | Resource name. |
| `update_time` | String | Output only. Last update timestamp. |
| `mime_type` | String | A content type specifier for the artifact. Content type specifiers are Media Types (https://en.wikipedia.org/wiki/Media_type) with a possible "schema" parameter that specifies a schema for the stored information. Content types can specify compression. Currently only GZip compression is supported (indicated with "+gzip"). |
| `hash` | String | Output only. A SHA-256 hash of the artifact's contents. If the artifact is gzipped, this is the hash of the uncompressed artifact. |
| `contents` | String | Input only. The contents of the artifact. Provided by API callers when artifacts are created or replaced. To access the contents of an artifact, use GetArtifactContents. |
| `size_bytes` | i64 | Output only. The size of the artifact in bytes. If the artifact is gzipped, this is the size of the uncompressed artifact. |
| `annotations` | HashMap<String, String> | Annotations attach non-identifying metadata to resources. Annotation keys and values are less restricted than those of labels, but should be generally used for small values of broad interest. Larger, topic- specific metadata should be stored in Artifacts. |
| `create_time` | String | Output only. Creation timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create artifact
artifact = provider.apigeeregistry_api.Artifact {
    parent = "value"  # Required. The parent, which owns this collection of artifacts. Format: `{parent}`
}

# Access artifact outputs
artifact_id = artifact.id
artifact_labels = artifact.labels
artifact_name = artifact.name
artifact_update_time = artifact.update_time
artifact_mime_type = artifact.mime_type
artifact_hash = artifact.hash
artifact_contents = artifact.contents
artifact_size_bytes = artifact.size_bytes
artifact_annotations = artifact.annotations
artifact_create_time = artifact.create_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple version resources
version_0 = provider.apigeeregistry_api.Version {
    parent = "value-0"
}
version_1 = provider.apigeeregistry_api.Version {
    parent = "value-1"
}
version_2 = provider.apigeeregistry_api.Version {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    version = provider.apigeeregistry_api.Version {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Apigeeregistry_api Documentation](https://cloud.google.com/apigeeregistry_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
