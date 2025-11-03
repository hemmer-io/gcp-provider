# Domains_api Service



**Resources**: 9

---

## Overview

The domains_api service provides access to 9 resource types:

- [Registration](#registration) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [R]
- [Location](#location) [R]
- [Operation](#operation) [R]
- [Registration](#registration) [CRUD]
- [Registration](#registration) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [R]

---

## Resources


### Registration

Registers a new domain name and creates a corresponding `Registration` resource. Call `RetrieveRegisterParameters` first to check availability of the domain name and determine parameters like price that are needed to build a call to this method. A successful call creates a `Registration` resource in state `REGISTRATION_PENDING`, which resolves to `ACTIVE` within 1-2 minutes, indicating that the domain was successfully registered. If the resource ends up in state `REGISTRATION_FAILED`, it indicates that the domain was not registered successfully, and you can safely delete the resource and retry registration.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `domain_notices` | Vec<String> |  | The list of domain notices that you acknowledge. Call `RetrieveRegisterParameters` to see the notices that need acknowledgement. |
| `registration` | String |  | Required. The complete `Registration` resource to be created. |
| `contact_notices` | Vec<String> |  | The list of contact notices that the caller acknowledges. The notices needed here depend on the values specified in `registration.contact_settings`. |
| `yearly_price` | String |  | Required. Yearly price to register or renew the domain. The value that should be put here can be obtained from RetrieveRegisterParameters or SearchDomains calls. |
| `validate_only` | bool |  | When true, only validation is performed, without actually registering the domain. Follows: https://cloud.google.com/apis/design/design_patterns#request_validation |
| `parent` | String | ✅ | Required. The parent resource of the `Registration`. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `supported_privacy` | Vec<String> | Output only. Set of options for the `contact_settings.privacy` field that this `Registration` supports. |
| `dns_settings` | String | Settings controlling the DNS configuration of the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureDnsSettings` method. |
| `pending_contact_settings` | String | Output only. Pending contact settings for the `Registration`. Updates to the `contact_settings` field that change its `registrant_contact` or `privacy` fields require email confirmation by the `registrant_contact` before taking effect. This field is set only if there are pending updates to the `contact_settings` that have not been confirmed. To confirm the changes, the `registrant_contact` must follow the instructions in the email they receive. |
| `expire_time` | String | Output only. The expiration timestamp of the `Registration`. |
| `domain_properties` | Vec<String> | Output only. Special properties of the domain. |
| `create_time` | String | Output only. The creation timestamp of the `Registration` resource. |
| `labels` | HashMap<String, String> | Set of labels associated with the `Registration`. |
| `issues` | Vec<String> | Output only. The set of issues with the `Registration` that require attention. |
| `management_settings` | String | Settings for management of the `Registration`, including renewal, billing, and transfer. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureManagementSettings` method. |
| `name` | String | Output only. Name of the `Registration` resource, in the format `projects/*/locations/*/registrations/`. |
| `transfer_failure_reason` | String | Output only. Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). The reason the domain transfer failed. Only set for domains in TRANSFER_FAILED state. |
| `state` | String | Output only. The state of the `Registration` |
| `domain_name` | String | Required. Immutable. The domain name. Unicode domain names must be expressed in Punycode format. |
| `contact_settings` | String | Required. Settings for contact information linked to the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureContactSettings` method. |
| `register_failure_reason` | String | Output only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create registration
registration = provider.domains_api.Registration {
    parent = "value"  # Required. The parent resource of the `Registration`. Must be in the format `projects/*/locations/*`.
}

# Access registration outputs
registration_id = registration.id
registration_supported_privacy = registration.supported_privacy
registration_dns_settings = registration.dns_settings
registration_pending_contact_settings = registration.pending_contact_settings
registration_expire_time = registration.expire_time
registration_domain_properties = registration.domain_properties
registration_create_time = registration.create_time
registration_labels = registration.labels
registration_issues = registration.issues
registration_management_settings = registration.management_settings
registration_name = registration.name
registration_transfer_failure_reason = registration.transfer_failure_reason
registration_state = registration.state
registration_domain_name = registration.domain_name
registration_contact_settings = registration.contact_settings
registration_register_failure_reason = registration.register_failure_reason
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_metadata = location.metadata
location_display_name = location.display_name
location_location_id = location.location_id
location_name = location.name
location_labels = location.labels
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation_error = operation.error
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |


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
location_name = location.name
location_metadata = location.metadata
location_location_id = location.location_id
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


### Registration

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dns_settings` | String | Settings controlling the DNS configuration of the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureDnsSettings` method. |
| `pending_contact_settings` | String | Output only. Pending contact settings for the `Registration`. Updates to the `contact_settings` field that change its `registrant_contact` or `privacy` fields require email confirmation by the `registrant_contact` before taking effect. This field is set only if there are pending updates to the `contact_settings` that have not been confirmed. To confirm the changes, the `registrant_contact` must follow the instructions in the email they receive. |
| `create_time` | String | Output only. The creation timestamp of the `Registration` resource. |
| `domain_properties` | Vec<String> | Output only. Special properties of the domain. |
| `labels` | HashMap<String, String> | Set of labels associated with the `Registration`. |
| `state` | String | Output only. The state of the `Registration` |
| `provider` | String | Output only. Current domain management provider. |
| `expire_time` | String | Output only. The expiration timestamp of the `Registration`. |
| `issues` | Vec<String> | Output only. The set of issues with the `Registration` that require attention. |
| `contact_settings` | String | Required. Settings for contact information linked to the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureContactSettings` method. |
| `supported_privacy` | Vec<String> | Output only. Set of options for the `contact_settings.privacy` field that this `Registration` supports. |
| `transfer_failure_reason` | String | Output only. Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). The reason the domain transfer failed. Only set for domains in TRANSFER_FAILED state. |
| `management_settings` | String | Settings for management of the `Registration`, including renewal, billing, and transfer. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureManagementSettings` method. |
| `register_failure_reason` | String | Output only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state. |
| `domain_name` | String | Required. Immutable. The domain name. Unicode domain names must be expressed in Punycode format. |
| `name` | String | Output only. Name of the `Registration` resource, in the format `projects/*/locations/*/registrations/`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create registration
registration = provider.domains_api.Registration {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access registration outputs
registration_id = registration.id
registration_dns_settings = registration.dns_settings
registration_pending_contact_settings = registration.pending_contact_settings
registration_create_time = registration.create_time
registration_domain_properties = registration.domain_properties
registration_labels = registration.labels
registration_state = registration.state
registration_provider = registration.provider
registration_expire_time = registration.expire_time
registration_issues = registration.issues
registration_contact_settings = registration.contact_settings
registration_supported_privacy = registration.supported_privacy
registration_transfer_failure_reason = registration.transfer_failure_reason
registration_management_settings = registration.management_settings
registration_register_failure_reason = registration.register_failure_reason
registration_domain_name = registration.domain_name
registration_name = registration.name
```

---


### Registration

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The state of the `Registration` |
| `pending_contact_settings` | String | Output only. Pending contact settings for the `Registration`. Updates to the `contact_settings` field that change its `registrant_contact` or `privacy` fields require email confirmation by the `registrant_contact` before taking effect. This field is set only if there are pending updates to the `contact_settings` that have not been confirmed. To confirm the changes, the `registrant_contact` must follow the instructions in the email they receive. |
| `issues` | Vec<String> | Output only. The set of issues with the `Registration` that require attention. |
| `name` | String | Output only. Name of the `Registration` resource, in the format `projects/*/locations/*/registrations/`. |
| `dns_settings` | String | Settings controlling the DNS configuration of the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureDnsSettings` method. |
| `register_failure_reason` | String | Output only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state. |
| `management_settings` | String | Settings for management of the `Registration`, including renewal, billing, and transfer. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureManagementSettings` method. |
| `labels` | HashMap<String, String> | Set of labels associated with the `Registration`. |
| `expire_time` | String | Output only. The expiration timestamp of the `Registration`. |
| `transfer_failure_reason` | String | Output only. Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). The reason the domain transfer failed. Only set for domains in TRANSFER_FAILED state. |
| `create_time` | String | Output only. The creation timestamp of the `Registration` resource. |
| `supported_privacy` | Vec<String> | Output only. Set of options for the `contact_settings.privacy` field that this `Registration` supports. |
| `contact_settings` | String | Required. Settings for contact information linked to the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureContactSettings` method. |
| `domain_name` | String | Required. Immutable. The domain name. Unicode domain names must be expressed in Punycode format. |
| `domain_properties` | Vec<String> | Output only. Special properties of the domain. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create registration
registration = provider.domains_api.Registration {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access registration outputs
registration_id = registration.id
registration_state = registration.state
registration_pending_contact_settings = registration.pending_contact_settings
registration_issues = registration.issues
registration_name = registration.name
registration_dns_settings = registration.dns_settings
registration_register_failure_reason = registration.register_failure_reason
registration_management_settings = registration.management_settings
registration_labels = registration.labels
registration_expire_time = registration.expire_time
registration_transfer_failure_reason = registration.transfer_failure_reason
registration_create_time = registration.create_time
registration_supported_privacy = registration.supported_privacy
registration_contact_settings = registration.contact_settings
registration_domain_name = registration.domain_name
registration_domain_properties = registration.domain_properties
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_display_name = location.display_name
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
location_labels = location.labels
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_done = operation.done
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple registration resources
registration_0 = provider.domains_api.Registration {
    parent = "value-0"
}
registration_1 = provider.domains_api.Registration {
    parent = "value-1"
}
registration_2 = provider.domains_api.Registration {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    registration = provider.domains_api.Registration {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Domains_api Documentation](https://cloud.google.com/domains_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
