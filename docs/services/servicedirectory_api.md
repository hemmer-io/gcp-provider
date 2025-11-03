# Servicedirectory_api Service



**Resources**: 9

---

## Overview

The servicedirectory_api service provides access to 9 resource types:

- [Namespace](#namespace) [CRUD]
- [Location](#location) [R]
- [Service](#service) [CRUD]
- [Endpoint](#endpoint) [CRUD]
- [Endpoint](#endpoint) [CRUD]
- [Namespace](#namespace) [CRUD]
- [Workload](#workload) [C]
- [Service](#service) [CRUD]
- [Location](#location) [R]

---

## Resources


### Namespace

Creates a namespace, and returns the new namespace.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. The globally unique identifier of the namespace in the UUID4 format. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels associated with this namespace. No more than 64 user labels can be associated with a given resource. Label keys and values can be no longer than 63 characters. |
| `name` | String |  | Immutable. The resource name for the namespace in the format `projects/*/locations/*/namespaces/*`. |
| `parent` | String | ✅ | Required. The resource name of the project and location the namespace will be created in. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. The globally unique identifier of the namespace in the UUID4 format. |
| `labels` | HashMap<String, String> | Optional. Resource labels associated with this namespace. No more than 64 user labels can be associated with a given resource. Label keys and values can be no longer than 63 characters. |
| `name` | String | Immutable. The resource name for the namespace in the format `projects/*/locations/*/namespaces/*`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create namespace
namespace = provider.servicedirectory_api.Namespace {
    parent = "value"  # Required. The resource name of the project and location the namespace will be created in.
}

# Access namespace outputs
namespace_id = namespace.id
namespace_uid = namespace.uid
namespace_labels = namespace.labels
namespace_name = namespace.name
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


### Service

Creates a service, and returns the new service.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `endpoints` | Vec<String> |  | Output only. Endpoints associated with this service. Returned on LookupService.ResolveService. Control plane clients should use RegistrationService.ListEndpoints. |
| `annotations` | HashMap<String, String> |  | Optional. Annotations for the service. This data can be consumed by service clients. Restrictions: * The entire annotations dictionary may contain up to 2000 characters, spread accoss all key-value pairs. Annotations that go beyond this limit are rejected * Valid annotation keys have two segments: an optional prefix and name, separated by a slash (/). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (.), not longer than 253 characters in total, followed by a slash (/). Annotations that fails to meet these requirements are rejected Note: This field is equivalent to the `metadata` field in the v1beta1 API. They have the same syntax and read/write to the same location in Service Directory. |
| `name` | String |  | Immutable. The resource name for the service in the format `projects/*/locations/*/namespaces/*/services/*`. |
| `uid` | String |  | Output only. The globally unique identifier of the service in the UUID4 format. |
| `parent` | String | ✅ | Required. The resource name of the namespace this service will belong to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `endpoints` | Vec<String> | Output only. Endpoints associated with this service. Returned on LookupService.ResolveService. Control plane clients should use RegistrationService.ListEndpoints. |
| `annotations` | HashMap<String, String> | Optional. Annotations for the service. This data can be consumed by service clients. Restrictions: * The entire annotations dictionary may contain up to 2000 characters, spread accoss all key-value pairs. Annotations that go beyond this limit are rejected * Valid annotation keys have two segments: an optional prefix and name, separated by a slash (/). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (.), not longer than 253 characters in total, followed by a slash (/). Annotations that fails to meet these requirements are rejected Note: This field is equivalent to the `metadata` field in the v1beta1 API. They have the same syntax and read/write to the same location in Service Directory. |
| `name` | String | Immutable. The resource name for the service in the format `projects/*/locations/*/namespaces/*/services/*`. |
| `uid` | String | Output only. The globally unique identifier of the service in the UUID4 format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.servicedirectory_api.Service {
    parent = "value"  # Required. The resource name of the namespace this service will belong to.
}

# Access service outputs
service_id = service.id
service_endpoints = service.endpoints
service_annotations = service.annotations
service_name = service.name
service_uid = service.uid
```

---


### Endpoint

Creates an endpoint, and returns the new endpoint.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `address` | String |  | Optional. An IPv4 or IPv6 address. Service Directory rejects bad addresses like: * `8.8.8` * `8.8.8.8:53` * `test:bad:address` * `[::1]` * `[::1]:8080` Limited to 45 characters. |
| `name` | String |  | Immutable. The resource name for the endpoint in the format `projects/*/locations/*/namespaces/*/services/*/endpoints/*`. |
| `uid` | String |  | Output only. The globally unique identifier of the endpoint in the UUID4 format. |
| `annotations` | HashMap<String, String> |  | Optional. Annotations for the endpoint. This data can be consumed by service clients. Restrictions: * The entire annotations dictionary may contain up to 512 characters, spread accoss all key-value pairs. Annotations that go beyond this limit are rejected * Valid annotation keys have two segments: an optional prefix and name, separated by a slash (/). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (.), not longer than 253 characters in total, followed by a slash (/) Annotations that fails to meet these requirements are rejected. Note: This field is equivalent to the `metadata` field in the v1beta1 API. They have the same syntax and read/write to the same location in Service Directory. |
| `port` | i64 |  | Optional. Service Directory rejects values outside of `[0, 65535]`. |
| `network` | String |  | Immutable. The Google Compute Engine network (VPC) of the endpoint in the format `projects//locations/global/networks/*`. The project must be specified by project number (project id is rejected). Incorrectly formatted networks are rejected, we also check to make sure that you have the servicedirectory.networks.attach permission on the project specified. |
| `parent` | String | ✅ | Required. The resource name of the service that this endpoint provides. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `address` | String | Optional. An IPv4 or IPv6 address. Service Directory rejects bad addresses like: * `8.8.8` * `8.8.8.8:53` * `test:bad:address` * `[::1]` * `[::1]:8080` Limited to 45 characters. |
| `name` | String | Immutable. The resource name for the endpoint in the format `projects/*/locations/*/namespaces/*/services/*/endpoints/*`. |
| `uid` | String | Output only. The globally unique identifier of the endpoint in the UUID4 format. |
| `annotations` | HashMap<String, String> | Optional. Annotations for the endpoint. This data can be consumed by service clients. Restrictions: * The entire annotations dictionary may contain up to 512 characters, spread accoss all key-value pairs. Annotations that go beyond this limit are rejected * Valid annotation keys have two segments: an optional prefix and name, separated by a slash (/). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (.), not longer than 253 characters in total, followed by a slash (/) Annotations that fails to meet these requirements are rejected. Note: This field is equivalent to the `metadata` field in the v1beta1 API. They have the same syntax and read/write to the same location in Service Directory. |
| `port` | i64 | Optional. Service Directory rejects values outside of `[0, 65535]`. |
| `network` | String | Immutable. The Google Compute Engine network (VPC) of the endpoint in the format `projects//locations/global/networks/*`. The project must be specified by project number (project id is rejected). Incorrectly formatted networks are rejected, we also check to make sure that you have the servicedirectory.networks.attach permission on the project specified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create endpoint
endpoint = provider.servicedirectory_api.Endpoint {
    parent = "value"  # Required. The resource name of the service that this endpoint provides.
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_address = endpoint.address
endpoint_name = endpoint.name
endpoint_uid = endpoint.uid
endpoint_annotations = endpoint.annotations
endpoint_port = endpoint.port
endpoint_network = endpoint.network
```

---


### Endpoint

Creates an endpoint, and returns the new endpoint.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `port` | i64 |  | Optional. Service Directory rejects values outside of `[0, 65535]`. |
| `update_time` | String |  | Output only. The timestamp when the endpoint was last updated. |
| `create_time` | String |  | Output only. The timestamp when the endpoint was created. |
| `name` | String |  | Immutable. The resource name for the endpoint in the format `projects/*/locations/*/namespaces/*/services/*/endpoints/*`. |
| `network` | String |  | Immutable. The Google Compute Engine network (VPC) of the endpoint in the format `projects//locations/global/networks/*`. The project must be specified by project number (project id is rejected). Incorrectly formatted networks are rejected, but no other validation is performed on this field (ex. network or project existence, reachability, or permissions). |
| `address` | String |  | Optional. An IPv4 or IPv6 address. Service Directory rejects bad addresses like: * `8.8.8` * `8.8.8.8:53` * `test:bad:address` * `[::1]` * `[::1]:8080` Limited to 45 characters. |
| `uid` | String |  | Output only. A globally unique identifier (in UUID4 format) for this endpoint. |
| `metadata` | HashMap<String, String> |  | Optional. Metadata for the endpoint. This data can be consumed by service clients. Restrictions: * The entire metadata dictionary may contain up to 512 characters, spread accoss all key-value pairs. Metadata that goes beyond this limit are rejected * Valid metadata keys have two segments: an optional prefix and name, separated by a slash (/). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (.), not longer than 253 characters in total, followed by a slash (/). Metadata that fails to meet these requirements are rejected Note: This field is equivalent to the `annotations` field in the v1 API. They have the same syntax and read/write to the same location in Service Directory. |
| `parent` | String | ✅ | Required. The resource name of the service that this endpoint provides. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `port` | i64 | Optional. Service Directory rejects values outside of `[0, 65535]`. |
| `update_time` | String | Output only. The timestamp when the endpoint was last updated. |
| `create_time` | String | Output only. The timestamp when the endpoint was created. |
| `name` | String | Immutable. The resource name for the endpoint in the format `projects/*/locations/*/namespaces/*/services/*/endpoints/*`. |
| `network` | String | Immutable. The Google Compute Engine network (VPC) of the endpoint in the format `projects//locations/global/networks/*`. The project must be specified by project number (project id is rejected). Incorrectly formatted networks are rejected, but no other validation is performed on this field (ex. network or project existence, reachability, or permissions). |
| `address` | String | Optional. An IPv4 or IPv6 address. Service Directory rejects bad addresses like: * `8.8.8` * `8.8.8.8:53` * `test:bad:address` * `[::1]` * `[::1]:8080` Limited to 45 characters. |
| `uid` | String | Output only. A globally unique identifier (in UUID4 format) for this endpoint. |
| `metadata` | HashMap<String, String> | Optional. Metadata for the endpoint. This data can be consumed by service clients. Restrictions: * The entire metadata dictionary may contain up to 512 characters, spread accoss all key-value pairs. Metadata that goes beyond this limit are rejected * Valid metadata keys have two segments: an optional prefix and name, separated by a slash (/). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (.), not longer than 253 characters in total, followed by a slash (/). Metadata that fails to meet these requirements are rejected Note: This field is equivalent to the `annotations` field in the v1 API. They have the same syntax and read/write to the same location in Service Directory. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create endpoint
endpoint = provider.servicedirectory_api.Endpoint {
    parent = "value"  # Required. The resource name of the service that this endpoint provides.
}

# Access endpoint outputs
endpoint_id = endpoint.id
endpoint_port = endpoint.port
endpoint_update_time = endpoint.update_time
endpoint_create_time = endpoint.create_time
endpoint_name = endpoint.name
endpoint_network = endpoint.network
endpoint_address = endpoint.address
endpoint_uid = endpoint.uid
endpoint_metadata = endpoint.metadata
```

---


### Namespace

Creates a namespace, and returns the new namespace.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the namespace was created. |
| `update_time` | String |  | Output only. The timestamp when the namespace was last updated. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels associated with this namespace. No more than 64 user labels can be associated with a given resource. Label keys and values can be no longer than 63 characters. |
| `name` | String |  | Immutable. The resource name for the namespace in the format `projects/*/locations/*/namespaces/*`. |
| `uid` | String |  | Output only. A globally unique identifier (in UUID4 format) for this namespace. |
| `parent` | String | ✅ | Required. The resource name of the project and location the namespace will be created in. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the namespace was created. |
| `update_time` | String | Output only. The timestamp when the namespace was last updated. |
| `labels` | HashMap<String, String> | Optional. Resource labels associated with this namespace. No more than 64 user labels can be associated with a given resource. Label keys and values can be no longer than 63 characters. |
| `name` | String | Immutable. The resource name for the namespace in the format `projects/*/locations/*/namespaces/*`. |
| `uid` | String | Output only. A globally unique identifier (in UUID4 format) for this namespace. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create namespace
namespace = provider.servicedirectory_api.Namespace {
    parent = "value"  # Required. The resource name of the project and location the namespace will be created in.
}

# Access namespace outputs
namespace_id = namespace.id
namespace_create_time = namespace.create_time
namespace_update_time = namespace.update_time
namespace_labels = namespace.labels
namespace_name = namespace.name
namespace_uid = namespace.uid
```

---


### Workload

Tests IAM permissions for a resource (namespace, service or service workload only).

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

# Create workload
workload = provider.servicedirectory_api.Workload {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

```

---


### Service

Creates a service, and returns the new service.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the service was created. |
| `endpoints` | Vec<String> |  | Output only. Endpoints associated with this service. Returned on LookupService.ResolveService. Control plane clients should use RegistrationService.ListEndpoints. |
| `metadata` | HashMap<String, String> |  | Optional. Metadata for the service. This data can be consumed by service clients. Restrictions: * The entire metadata dictionary may contain up to 2000 characters, spread accoss all key-value pairs. Metadata that goes beyond this limit are rejected * Valid metadata keys have two segments: an optional prefix and name, separated by a slash (/). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (.), not longer than 253 characters in total, followed by a slash (/). Metadata that fails to meet these requirements are rejected Note: This field is equivalent to the `annotations` field in the v1 API. They have the same syntax and read/write to the same location in Service Directory. |
| `uid` | String |  | Output only. A globally unique identifier (in UUID4 format) for this service. |
| `name` | String |  | Immutable. The resource name for the service in the format `projects/*/locations/*/namespaces/*/services/*`. |
| `update_time` | String |  | Output only. The timestamp when the service was last updated. Note: endpoints being created/deleted/updated within the service are not considered service updates for the purpose of this timestamp. |
| `parent` | String | ✅ | Required. The resource name of the namespace this service will belong to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the service was created. |
| `endpoints` | Vec<String> | Output only. Endpoints associated with this service. Returned on LookupService.ResolveService. Control plane clients should use RegistrationService.ListEndpoints. |
| `metadata` | HashMap<String, String> | Optional. Metadata for the service. This data can be consumed by service clients. Restrictions: * The entire metadata dictionary may contain up to 2000 characters, spread accoss all key-value pairs. Metadata that goes beyond this limit are rejected * Valid metadata keys have two segments: an optional prefix and name, separated by a slash (/). The name segment is required and must be 63 characters or less, beginning and ending with an alphanumeric character ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and alphanumerics between. The prefix is optional. If specified, the prefix must be a DNS subdomain: a series of DNS labels separated by dots (.), not longer than 253 characters in total, followed by a slash (/). Metadata that fails to meet these requirements are rejected Note: This field is equivalent to the `annotations` field in the v1 API. They have the same syntax and read/write to the same location in Service Directory. |
| `uid` | String | Output only. A globally unique identifier (in UUID4 format) for this service. |
| `name` | String | Immutable. The resource name for the service in the format `projects/*/locations/*/namespaces/*/services/*`. |
| `update_time` | String | Output only. The timestamp when the service was last updated. Note: endpoints being created/deleted/updated within the service are not considered service updates for the purpose of this timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service
service = provider.servicedirectory_api.Service {
    parent = "value"  # Required. The resource name of the namespace this service will belong to.
}

# Access service outputs
service_id = service.id
service_create_time = service.create_time
service_endpoints = service.endpoints
service_metadata = service.metadata
service_uid = service.uid
service_name = service.name
service_update_time = service.update_time
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


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
location_metadata = location.metadata
location_name = location.name
location_display_name = location.display_name
location_labels = location.labels
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple namespace resources
namespace_0 = provider.servicedirectory_api.Namespace {
    parent = "value-0"
}
namespace_1 = provider.servicedirectory_api.Namespace {
    parent = "value-1"
}
namespace_2 = provider.servicedirectory_api.Namespace {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    namespace = provider.servicedirectory_api.Namespace {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Servicedirectory_api Documentation](https://cloud.google.com/servicedirectory_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
