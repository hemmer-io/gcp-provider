# Servicebroker_api Service



**Resources**: 13

---

## Overview

The servicebroker_api service provides access to 13 resource types:

- [Servicebroker](#servicebroker) [CR]
- [Instance](#instance) [R]
- [Binding](#binding) [R]
- [Service_binding](#service_binding) [CRD]
- [Service_instance](#service_instance) [CRUD]
- [Catalog](#catalog) [R]
- [Broker](#broker) [CRD]
- [Servicebroker](#servicebroker) [CR]
- [Service_binding](#service_binding) [CRD]
- [Service_instance](#service_instance) [CRUD]
- [Servicebroker](#servicebroker) [CR]
- [Catalog](#catalog) [R]
- [Instance](#instance) [R]

---

## Resources


### Servicebroker

Sets the access control policy on the specified resource. Replaces any
existing policy.

Can return Public Errors: NOT_FOUND, INVALID_ARGUMENT and PERMISSION_DENIED

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of
the policy is limited to a few 10s of KB. An empty policy is a
valid policy but certain Cloud Platform services (such as Projects)
might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified.
See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a policy from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform policy updates in order to avoid race
conditions: An `etag` is returned in the response to `getIamPolicy`, and
systems are expected to put that etag in the request to `setIamPolicy` to
ensure that their change will be applied to the same version of the policy.

**Important:** If you use IAM Conditions, you must include the `etag` field
whenever you call `setIamPolicy`. If you omit this field, then IAM allows
you to overwrite a version `3` policy with a version `1` policy, and all of
the conditions in the version `3` policy are lost. |
| `version` | i64 | Specifies the format of the policy.

Valid values are `0`, `1`, and `3`. Requests that specify an invalid value
are rejected.

Any operation that affects conditional role bindings must specify version
`3`. This requirement applies to the following operations:

* Getting a policy that includes a conditional role binding
* Adding a conditional role binding to a policy
* Changing a conditional role binding in a policy
* Removing any role binding, with or without a condition, from a policy
  that includes conditions

**Important:** If you use IAM Conditions, you must include the `etag` field
whenever you call `setIamPolicy`. If you omit this field, then IAM allows
you to overwrite a version `3` policy with a version `1` policy, and all of
the conditions in the version `3` policy are lost.

If a policy does not include any conditions, operations on that policy may
specify any valid version or leave the field unset. |
| `bindings` | Vec<String> | Associates a list of `members` to a `role`. Optionally, may specify a
`condition` that determines how and when the `bindings` are applied. Each
of the `bindings` must contain at least one member. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create servicebroker
servicebroker = provider.servicebroker_api.Servicebroker {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified.
See the operation documentation for the appropriate value for this field.
}

# Access servicebroker outputs
servicebroker_id = servicebroker.id
servicebroker_etag = servicebroker.etag
servicebroker_version = servicebroker.version
servicebroker_bindings = servicebroker.bindings
```

---


### Instance

Gets the given service instance from the system.
The API call accepts both OSB style API and standard google style API
resource path.
i.e. both `projects/*/brokers/*/instances/*`
 and `projects/*/brokers/*/v2/service_instances/*` are acceptable paths.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `context` | HashMap<String, String> | Platform specific contextual information under which the service instance
is to be provisioned. This replaces organization_guid and space_guid.
But can also contain anything.
Currently only used for logging context information. |
| `description` | String | To return errors when GetInstance call is done via HTTP to be unified with
other methods. |
| `service_id` | String | The id of the service. Must be a valid identifier of a service
contained in the list from a `ListServices()` call.
Maximum length is 64, GUID recommended.
Required. |
| `create_time` | String | Output only. Timestamp for when the instance was created. |
| `space_guid` | String | The identifier for the project space within the platform organization.
Required. |
| `deployment_name` | String | Output only. String containing the Deployment Manager deployment name that was created
for this instance, |
| `instance_id` | String | The id of the service instance. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |
| `organization_guid` | String | The platform GUID for the organization under which the service is to be
provisioned.
Required. |
| `previous_values` | HashMap<String, String> | Used only in UpdateServiceInstance request to optionally specify previous
fields. |
| `resource_name` | String | Output only. The resource name of the instance, e.g.
projects/project_id/brokers/broker_id/service_instances/instance_id |
| `parameters` | HashMap<String, String> | Configuration options for the service instance.
Parameters is JSON object serialized to string. |
| `plan_id` | String | The ID of the plan. See `Service` and `Plan` resources for details.
Maximum length is 64, GUID recommended.
Required. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access instance outputs
instance_id = instance.id
instance_context = instance.context
instance_description = instance.description
instance_service_id = instance.service_id
instance_create_time = instance.create_time
instance_space_guid = instance.space_guid
instance_deployment_name = instance.deployment_name
instance_instance_id = instance.instance_id
instance_organization_guid = instance.organization_guid
instance_previous_values = instance.previous_values
instance_resource_name = instance.resource_name
instance_parameters = instance.parameters
instance_plan_id = instance.plan_id
```

---


### Binding

Returns the state of the last operation for the binding.
Only last (or current) operation can be polled.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional description of the Operation state. |
| `state` | String | The state of the operation.
Valid values are: "in progress", "succeeded", and "failed". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access binding outputs
binding_id = binding.id
binding_description = binding.description
binding_state = binding.state
```

---


### Service_binding

CreateBinding generates a service binding to an existing service instance.
See ProviServiceInstance for async operation details.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `binding_id` | String |  | The id of the binding. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |
| `bind_resource` | HashMap<String, String> |  | A JSON object that contains data for platform resources associated with
the binding to be created. |
| `create_time` | String |  | Output only. Timestamp for when the binding was created. |
| `parameters` | HashMap<String, String> |  | Configuration options for the service binding. |
| `plan_id` | String |  | The ID of the plan. See `Service` and `Plan` resources for details.
Maximum length is 64, GUID recommended.
Required. |
| `deployment_name` | String |  | Output only. String containing the Deployment Manager deployment name that was created
for this binding, |
| `resource_name` | String |  | Output only. The resource name of the binding, e.g.
projects/project_id/brokers/broker_id/service_instances/instance_id/bindings/binding_id. |
| `service_id` | String |  | The id of the service. Must be a valid identifier of a service
contained in the list from a `ListServices()` call.
Maximum length is 64, GUID recommended.
Required. |
| `parent` | String | ✅ | The GCP container.
Must match
`projects/[PROJECT_ID]/brokers/[BROKER_ID]/v2/service_instances/[INSTANCE_ID]`. |
| `binding_id` | String | ✅ | The id of the binding. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deployment_name` | String | String containing the Deployment Manager deployment name that was created
for this binding, |
| `description` | String | Used to communicate description of the response. Usually for non-standard
error codes.
https://github.com/openservicebrokerapi/servicebroker/blob/master/spec.md#service-broker-errors |
| `syslog_drain_url` | String | From where to read system logs. |
| `volume_mounts` | Vec<HashMap<String, String>> | An array of configurations for mounting volumes. |
| `route_service_url` | String | A URL to which the platform may proxy requests for the address sent with
bind_resource.route |
| `resource_name` | String | Output only. The resource name of the binding, e.g.
projects/project_id/brokers/broker_id/service_instances/instance_id/bindings/binding_id. |
| `credentials` | HashMap<String, String> | Credentials to use the binding. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_binding
service_binding = provider.servicebroker_api.Service_binding {
    parent = "value"  # The GCP container.
Must match
`projects/[PROJECT_ID]/brokers/[BROKER_ID]/v2/service_instances/[INSTANCE_ID]`.
    binding_id = "value"  # The id of the binding. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required.
}

# Access service_binding outputs
service_binding_id = service_binding.id
service_binding_deployment_name = service_binding.deployment_name
service_binding_description = service_binding.description
service_binding_syslog_drain_url = service_binding.syslog_drain_url
service_binding_volume_mounts = service_binding.volume_mounts
service_binding_route_service_url = service_binding.route_service_url
service_binding_resource_name = service_binding.resource_name
service_binding_credentials = service_binding.credentials
```

---


### Service_instance

Provisions a service instance.
If `request.accepts_incomplete` is false and Broker cannot execute request
synchronously HTTP 422 error will be returned along with
FAILED_PRECONDITION status.
If `request.accepts_incomplete` is true and the Broker decides to execute
resource asynchronously then HTTP 202 response code will be returned and a
valid polling operation in the response will be included.
If Broker executes the request synchronously and it succeeds HTTP 201
response will be furnished.
If identical instance exists, then HTTP 200 response will be returned.
If an instance with identical ID but mismatching parameters exists, then
HTTP 409 status code will be returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `context` | HashMap<String, String> |  | Platform specific contextual information under which the service instance
is to be provisioned. This replaces organization_guid and space_guid.
But can also contain anything.
Currently only used for logging context information. |
| `description` | String |  | To return errors when GetInstance call is done via HTTP to be unified with
other methods. |
| `service_id` | String |  | The id of the service. Must be a valid identifier of a service
contained in the list from a `ListServices()` call.
Maximum length is 64, GUID recommended.
Required. |
| `create_time` | String |  | Output only. Timestamp for when the instance was created. |
| `space_guid` | String |  | The identifier for the project space within the platform organization.
Required. |
| `deployment_name` | String |  | Output only. String containing the Deployment Manager deployment name that was created
for this instance, |
| `instance_id` | String |  | The id of the service instance. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |
| `organization_guid` | String |  | The platform GUID for the organization under which the service is to be
provisioned.
Required. |
| `previous_values` | HashMap<String, String> |  | Used only in UpdateServiceInstance request to optionally specify previous
fields. |
| `resource_name` | String |  | Output only. The resource name of the instance, e.g.
projects/project_id/brokers/broker_id/service_instances/instance_id |
| `parameters` | HashMap<String, String> |  | Configuration options for the service instance.
Parameters is JSON object serialized to string. |
| `plan_id` | String |  | The ID of the plan. See `Service` and `Plan` resources for details.
Maximum length is 64, GUID recommended.
Required. |
| `instance_id` | String | ✅ | The id of the service instance. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |
| `parent` | String | ✅ | Parent must match `projects/[PROJECT_ID]/brokers/[BROKER_ID]`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `context` | HashMap<String, String> | Platform specific contextual information under which the service instance
is to be provisioned. This replaces organization_guid and space_guid.
But can also contain anything.
Currently only used for logging context information. |
| `description` | String | To return errors when GetInstance call is done via HTTP to be unified with
other methods. |
| `service_id` | String | The id of the service. Must be a valid identifier of a service
contained in the list from a `ListServices()` call.
Maximum length is 64, GUID recommended.
Required. |
| `create_time` | String | Output only. Timestamp for when the instance was created. |
| `space_guid` | String | The identifier for the project space within the platform organization.
Required. |
| `deployment_name` | String | Output only. String containing the Deployment Manager deployment name that was created
for this instance, |
| `instance_id` | String | The id of the service instance. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |
| `organization_guid` | String | The platform GUID for the organization under which the service is to be
provisioned.
Required. |
| `previous_values` | HashMap<String, String> | Used only in UpdateServiceInstance request to optionally specify previous
fields. |
| `resource_name` | String | Output only. The resource name of the instance, e.g.
projects/project_id/brokers/broker_id/service_instances/instance_id |
| `parameters` | HashMap<String, String> | Configuration options for the service instance.
Parameters is JSON object serialized to string. |
| `plan_id` | String | The ID of the plan. See `Service` and `Plan` resources for details.
Maximum length is 64, GUID recommended.
Required. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_instance
service_instance = provider.servicebroker_api.Service_instance {
    instance_id = "value"  # The id of the service instance. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required.
    parent = "value"  # Parent must match `projects/[PROJECT_ID]/brokers/[BROKER_ID]`.
}

# Access service_instance outputs
service_instance_id = service_instance.id
service_instance_context = service_instance.context
service_instance_description = service_instance.description
service_instance_service_id = service_instance.service_id
service_instance_create_time = service_instance.create_time
service_instance_space_guid = service_instance.space_guid
service_instance_deployment_name = service_instance.deployment_name
service_instance_instance_id = service_instance.instance_id
service_instance_organization_guid = service_instance.organization_guid
service_instance_previous_values = service_instance.previous_values
service_instance_resource_name = service_instance.resource_name
service_instance_parameters = service_instance.parameters
service_instance_plan_id = service_instance.plan_id
```

---


### Catalog

Lists all the Services registered with this broker for consumption for
given service registry broker, which contains an set of services.
Note, that Service producer API is separate from Broker API.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Used to communicate description of the response. Usually for non-standard
error codes.
https://github.com/openservicebrokerapi/servicebroker/blob/master/spec.md#service-broker-errors |
| `services` | Vec<String> | The services available for the requested GCP project. |
| `next_page_token` | String | This token allows you to get the next page of results for list requests.
If the number of results is larger than `pageSize`, use the `nextPageToken`
as a value for the query parameter `pageToken` in the next list request.
Subsequent list requests will have their own `nextPageToken` to continue
paging through the results |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access catalog outputs
catalog_id = catalog.id
catalog_description = catalog.description
catalog_services = catalog.services
catalog_next_page_token = catalog.next_page_token
```

---


### Broker

CreateBroker creates a Broker.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp for when the broker was created. |
| `name` | String |  | Name of the broker in the format:
<projects>/<project-id>/brokers/<broker>.
This allows for multiple brokers per project which can be used to
enable having custom brokers per GKE cluster, for example. |
| `url` | String |  | Output only. URL of the broker OSB-compliant endpoint, for example:
https://servicebroker.googleapis.com/projects/<project>/brokers/<broker> |
| `title` | String |  | User friendly title of the broker.
Limited to 1024 characters. Requests with longer titles will be rejected. |
| `parent` | String | ✅ | The project in which to create broker. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | This token allows you to get the next page of results for list requests.
If the number of results is larger than `pageSize`, use the `nextPageToken`
as a value for the query parameter `pageToken` in the next list request.
Subsequent list requests will have their own `nextPageToken` to continue
paging through the results |
| `brokers` | Vec<String> | The list of brokers in the container. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create broker
broker = provider.servicebroker_api.Broker {
    parent = "value"  # The project in which to create broker.
}

# Access broker outputs
broker_id = broker.id
broker_next_page_token = broker.next_page_token
broker_brokers = broker.brokers
```

---


### Servicebroker

Returns permissions that a caller has on the specified resource.
If the resource does not exist, this will return an empty set of
permissions, not a NOT_FOUND error.

Note: This operation is designed to be used for building permission-aware
UIs and command-line tools, not for authorization checking. This operation
may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with
wildcards (such as '*' or 'storage.*') are not allowed. For more
information see
[IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested.
See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a policy from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform policy updates in order to avoid race
conditions: An `etag` is returned in the response to `getIamPolicy`, and
systems are expected to put that etag in the request to `setIamPolicy` to
ensure that their change will be applied to the same version of the policy.

**Important:** If you use IAM Conditions, you must include the `etag` field
whenever you call `setIamPolicy`. If you omit this field, then IAM allows
you to overwrite a version `3` policy with a version `1` policy, and all of
the conditions in the version `3` policy are lost. |
| `bindings` | Vec<String> | Associates a list of `members` to a `role`. Optionally, may specify a
`condition` that determines how and when the `bindings` are applied. Each
of the `bindings` must contain at least one member. |
| `version` | i64 | Specifies the format of the policy.

Valid values are `0`, `1`, and `3`. Requests that specify an invalid value
are rejected.

Any operation that affects conditional role bindings must specify version
`3`. This requirement applies to the following operations:

* Getting a policy that includes a conditional role binding
* Adding a conditional role binding to a policy
* Changing a conditional role binding in a policy
* Removing any role binding, with or without a condition, from a policy
  that includes conditions

**Important:** If you use IAM Conditions, you must include the `etag` field
whenever you call `setIamPolicy`. If you omit this field, then IAM allows
you to overwrite a version `3` policy with a version `1` policy, and all of
the conditions in the version `3` policy are lost.

If a policy does not include any conditions, operations on that policy may
specify any valid version or leave the field unset. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create servicebroker
servicebroker = provider.servicebroker_api.Servicebroker {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested.
See the operation documentation for the appropriate value for this field.
}

# Access servicebroker outputs
servicebroker_id = servicebroker.id
servicebroker_etag = servicebroker.etag
servicebroker_bindings = servicebroker.bindings
servicebroker_version = servicebroker.version
```

---


### Service_binding

CreateBinding generates a service binding to an existing service instance.
See ProviServiceInstance for async operation details.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | HashMap<String, String> |  | Configuration options for the service binding. |
| `create_time` | String |  | Output only. Timestamp for when the binding was created. |
| `bind_resource` | HashMap<String, String> |  | A JSON object that contains data for platform resources associated with
the binding to be created. |
| `service_id` | String |  | The id of the service. Must be a valid identifier of a service
contained in the list from a `ListServices()` call.
Maximum length is 64, GUID recommended.
Required. |
| `plan_id` | String |  | The ID of the plan. See `Service` and `Plan` resources for details.
Maximum length is 64, GUID recommended.
Required. |
| `binding_id` | String |  | The id of the binding. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |
| `instance_id` | String | ✅ | The service instance to which to bind. |
| `binding_id` | String | ✅ | The id of the binding. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |
| `parent` | String | ✅ | The GCP container.
Must match
`projects/[PROJECT_ID]/brokers/[BROKER_ID]`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Used to communicate description of the response. Usually for non-standard
error codes.
https://github.com/openservicebrokerapi/servicebroker/blob/master/spec.md#service-broker-errors |
| `volume_mounts` | Vec<HashMap<String, String>> | An array of configuration for mounting volumes. |
| `credentials` | HashMap<String, String> | Credentials to use the binding. |
| `route_service_url` | String | A URL to which the platform may proxy requests for the address sent with
bind_resource.route |
| `syslog_drain_url` | String | From where to read system logs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_binding
service_binding = provider.servicebroker_api.Service_binding {
    instance_id = "value"  # The service instance to which to bind.
    binding_id = "value"  # The id of the binding. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required.
    parent = "value"  # The GCP container.
Must match
`projects/[PROJECT_ID]/brokers/[BROKER_ID]`.
}

# Access service_binding outputs
service_binding_id = service_binding.id
service_binding_description = service_binding.description
service_binding_volume_mounts = service_binding.volume_mounts
service_binding_credentials = service_binding.credentials
service_binding_route_service_url = service_binding.route_service_url
service_binding_syslog_drain_url = service_binding.syslog_drain_url
```

---


### Service_instance

Provisions a service instance.
If `request.accepts_incomplete` is false and Broker cannot execute request
synchronously HTTP 422 error will be returned along with
FAILED_PRECONDITION status.
If `request.accepts_incomplete` is true and the Broker decides to execute
resource asynchronously then HTTP 202 response code will be returned and a
valid polling operation in the response will be included.
If Broker executes the request synchronously and it succeeds HTTP 201
response will be furnished.
If identical instance exists, then HTTP 200 response will be returned.
If an instance with identical ID but mismatching parameters exists, then
HTTP 409 status code will be returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `space_guid` | String |  | The identifier for the project space within the platform organization.
Required. |
| `organization_guid` | String |  | The platform GUID for the organization under which the service is to be
provisioned.
Required. |
| `context` | HashMap<String, String> |  | Platform specific contextual information under which the service instance
is to be provisioned. This replaces organization_guid and space_guid.
But can also contain anything.
Currently only used for logging context information. |
| `deployment_name` | String |  | Output only. Name of the Deployment Manager deployment used for provisioning of this
service instance. |
| `service_id` | String |  | The id of the service. Must be a valid identifier of a service
contained in the list from a `ListServices()` call.
Maximum length is 64, GUID recommended.
Required. |
| `instance_id` | String |  | The id of the service instance. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |
| `resource_name` | String |  | Output only. The resource name of the instance, e.g.
projects/project_id/brokers/broker_id/service_instances/instance_id |
| `plan_id` | String |  | The ID of the plan. See `Service` and `Plan` resources for details.
Maximum length is 64, GUID recommended.
Required. |
| `parameters` | HashMap<String, String> |  | Configuration options for the service instance.
Parameters is JSON object serialized to string. |
| `create_time` | String |  | Output only. Timestamp for when the instance was created. |
| `previous_values` | HashMap<String, String> |  | Used only in UpdateServiceInstance request to optionally specify previous
fields. |
| `parent` | String | ✅ | Parent must match `projects/[PROJECT_ID]/brokers/[BROKER_ID]`. |
| `instance_id` | String | ✅ | The id of the service instance. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `space_guid` | String | The identifier for the project space within the platform organization.
Required. |
| `organization_guid` | String | The platform GUID for the organization under which the service is to be
provisioned.
Required. |
| `context` | HashMap<String, String> | Platform specific contextual information under which the service instance
is to be provisioned. This replaces organization_guid and space_guid.
But can also contain anything.
Currently only used for logging context information. |
| `deployment_name` | String | Output only. Name of the Deployment Manager deployment used for provisioning of this
service instance. |
| `service_id` | String | The id of the service. Must be a valid identifier of a service
contained in the list from a `ListServices()` call.
Maximum length is 64, GUID recommended.
Required. |
| `instance_id` | String | The id of the service instance. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |
| `resource_name` | String | Output only. The resource name of the instance, e.g.
projects/project_id/brokers/broker_id/service_instances/instance_id |
| `plan_id` | String | The ID of the plan. See `Service` and `Plan` resources for details.
Maximum length is 64, GUID recommended.
Required. |
| `parameters` | HashMap<String, String> | Configuration options for the service instance.
Parameters is JSON object serialized to string. |
| `create_time` | String | Output only. Timestamp for when the instance was created. |
| `previous_values` | HashMap<String, String> | Used only in UpdateServiceInstance request to optionally specify previous
fields. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_instance
service_instance = provider.servicebroker_api.Service_instance {
    parent = "value"  # Parent must match `projects/[PROJECT_ID]/brokers/[BROKER_ID]`.
    instance_id = "value"  # The id of the service instance. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required.
}

# Access service_instance outputs
service_instance_id = service_instance.id
service_instance_space_guid = service_instance.space_guid
service_instance_organization_guid = service_instance.organization_guid
service_instance_context = service_instance.context
service_instance_deployment_name = service_instance.deployment_name
service_instance_service_id = service_instance.service_id
service_instance_instance_id = service_instance.instance_id
service_instance_resource_name = service_instance.resource_name
service_instance_plan_id = service_instance.plan_id
service_instance_parameters = service_instance.parameters
service_instance_create_time = service_instance.create_time
service_instance_previous_values = service_instance.previous_values
```

---


### Servicebroker

Returns permissions that a caller has on the specified resource.
If the resource does not exist, this will return an empty set of
permissions, not a NOT_FOUND error.

Note: This operation is designed to be used for building permission-aware
UIs and command-line tools, not for authorization checking. This operation
may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with
wildcards (such as '*' or 'storage.*') are not allowed. For more
information see
[IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested.
See the operation documentation for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bindings` | Vec<String> | Associates a list of `members` to a `role`. Optionally, may specify a
`condition` that determines how and when the `bindings` are applied. Each
of the `bindings` must contain at least one member. |
| `version` | i64 | Specifies the format of the policy.

Valid values are `0`, `1`, and `3`. Requests that specify an invalid value
are rejected.

Any operation that affects conditional role bindings must specify version
`3`. This requirement applies to the following operations:

* Getting a policy that includes a conditional role binding
* Adding a conditional role binding to a policy
* Changing a conditional role binding in a policy
* Removing any role binding, with or without a condition, from a policy
  that includes conditions

**Important:** If you use IAM Conditions, you must include the `etag` field
whenever you call `setIamPolicy`. If you omit this field, then IAM allows
you to overwrite a version `3` policy with a version `1` policy, and all of
the conditions in the version `3` policy are lost.

If a policy does not include any conditions, operations on that policy may
specify any valid version or leave the field unset. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help
prevent simultaneous updates of a policy from overwriting each other.
It is strongly suggested that systems make use of the `etag` in the
read-modify-write cycle to perform policy updates in order to avoid race
conditions: An `etag` is returned in the response to `getIamPolicy`, and
systems are expected to put that etag in the request to `setIamPolicy` to
ensure that their change will be applied to the same version of the policy.

**Important:** If you use IAM Conditions, you must include the `etag` field
whenever you call `setIamPolicy`. If you omit this field, then IAM allows
you to overwrite a version `3` policy with a version `1` policy, and all of
the conditions in the version `3` policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create servicebroker
servicebroker = provider.servicebroker_api.Servicebroker {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested.
See the operation documentation for the appropriate value for this field.
}

# Access servicebroker outputs
servicebroker_id = servicebroker.id
servicebroker_bindings = servicebroker.bindings
servicebroker_version = servicebroker.version
servicebroker_etag = servicebroker.etag
```

---


### Catalog

Lists all the Services registered with this broker for consumption for
given service registry broker, which contains an set of services.
Note, that Service producer API is separate from Broker API.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `services` | Vec<String> | The services available for the requested GCP project. |
| `next_page_token` | String | This token allows you to get the next page of results for list requests.
If the number of results is larger than `pageSize`, use the `nextPageToken`
as a value for the query parameter `pageToken` in the next list request.
Subsequent list requests will have their own `nextPageToken` to continue
paging through the results |
| `description` | String | Used to communicate description of the response. Usually for non-standard
error codes.
https://github.com/openservicebrokerapi/servicebroker/blob/master/spec.md#service-broker-errors |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access catalog outputs
catalog_id = catalog.id
catalog_services = catalog.services
catalog_next_page_token = catalog.next_page_token
catalog_description = catalog.description
```

---


### Instance

Gets the given service instance from the system.
This API is an extension and not part of the OSB spec.
Hence the path is a standard Google API URL.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `space_guid` | String | The identifier for the project space within the platform organization.
Required. |
| `organization_guid` | String | The platform GUID for the organization under which the service is to be
provisioned.
Required. |
| `context` | HashMap<String, String> | Platform specific contextual information under which the service instance
is to be provisioned. This replaces organization_guid and space_guid.
But can also contain anything.
Currently only used for logging context information. |
| `deployment_name` | String | Output only. Name of the Deployment Manager deployment used for provisioning of this
service instance. |
| `service_id` | String | The id of the service. Must be a valid identifier of a service
contained in the list from a `ListServices()` call.
Maximum length is 64, GUID recommended.
Required. |
| `instance_id` | String | The id of the service instance. Must be unique within GCP project.
Maximum length is 64, GUID recommended.
Required. |
| `resource_name` | String | Output only. The resource name of the instance, e.g.
projects/project_id/brokers/broker_id/service_instances/instance_id |
| `plan_id` | String | The ID of the plan. See `Service` and `Plan` resources for details.
Maximum length is 64, GUID recommended.
Required. |
| `parameters` | HashMap<String, String> | Configuration options for the service instance.
Parameters is JSON object serialized to string. |
| `create_time` | String | Output only. Timestamp for when the instance was created. |
| `previous_values` | HashMap<String, String> | Used only in UpdateServiceInstance request to optionally specify previous
fields. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access instance outputs
instance_id = instance.id
instance_space_guid = instance.space_guid
instance_organization_guid = instance.organization_guid
instance_context = instance.context
instance_deployment_name = instance.deployment_name
instance_service_id = instance.service_id
instance_instance_id = instance.instance_id
instance_resource_name = instance.resource_name
instance_plan_id = instance.plan_id
instance_parameters = instance.parameters
instance_create_time = instance.create_time
instance_previous_values = instance.previous_values
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple servicebroker resources
servicebroker_0 = provider.servicebroker_api.Servicebroker {
    resource = "value-0"
}
servicebroker_1 = provider.servicebroker_api.Servicebroker {
    resource = "value-1"
}
servicebroker_2 = provider.servicebroker_api.Servicebroker {
    resource = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    servicebroker = provider.servicebroker_api.Servicebroker {
        resource = "production-value"
    }
```

---

## Related Documentation

- [GCP Servicebroker_api Documentation](https://cloud.google.com/servicebroker_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
