# Domains_api Service



**Resources**: 9

---

## Overview

The domains_api service provides access to 9 resource types:

- [Location](#location) [R]
- [Operation](#operation) [R]
- [Registration](#registration) [CRUD]
- [Location](#location) [R]
- [Registration](#registration) [CRUD]
- [Operation](#operation) [R]
- [Location](#location) [R]
- [Registration](#registration) [CRUD]
- [Operation](#operation) [R]

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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_metadata = location.metadata
location_location_id = location.location_id
location_name = location.name
location_labels = location.labels
location_display_name = location.display_name
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
```

---


### Registration

Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations) Transfers a domain name from another registrar to Cloud Domains. For domains already managed by [Google Domains](https://domains.google/), use `ImportDomain` instead. Before calling this method, go to the domain's current registrar to unlock the domain for transfer and retrieve the domain's transfer authorization code. Then call `RetrieveTransferParameters` to confirm that the domain is unlocked and to get values needed to build a call to this method. A successful call creates a `Registration` resource in state `TRANSFER_PENDING`. It can take several days to complete the transfer process. The registrant can often speed up this process by approving the transfer through the current registrar, either by clicking a link in an email from the registrar or by visiting the registrar's website. A few minutes after transfer approval, the resource transitions to state `ACTIVE`, indicating that the transfer was successful. If the transfer is rejected or the request expires without being approved, the resource can end up in state `TRANSFER_FAILED`. If transfer fails, you can safely delete the resource and retry the transfer.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `yearly_price` | String |  | Required. Acknowledgement of the price to transfer or renew the domain for one year. Call `RetrieveTransferParameters` to obtain the price, which you must acknowledge. |
| `registration` | String |  | Required. The complete `Registration` resource to be created. You can leave `registration.dns_settings` unset to import the domain's current DNS configuration from its current registrar. Use this option only if you are sure that the domain's current DNS service does not cease upon transfer, as is often the case for DNS services provided for free by the registrar. |
| `contact_notices` | Vec<String> |  | The list of contact notices that you acknowledge. The notices needed here depend on the values specified in `registration.contact_settings`. |
| `validate_only` | bool |  | Validate the request without actually transferring the domain. |
| `authorization_code` | String |  | The domain's transfer authorization code. You can obtain this from the domain's current registrar. |
| `parent` | String | ✅ | Required. The parent resource of the `Registration`. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The creation timestamp of the `Registration` resource. |
| `domain_properties` | Vec<String> | Output only. Special properties of the domain. |
| `labels` | HashMap<String, String> | Set of labels associated with the `Registration`. |
| `register_failure_reason` | String | Output only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state. |
| `transfer_failure_reason` | String | Output only. Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). The reason the domain transfer failed. Only set for domains in TRANSFER_FAILED state. |
| `contact_settings` | String | Required. Settings for contact information linked to the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureContactSettings` method. |
| `issues` | Vec<String> | Output only. The set of issues with the `Registration` that require attention. |
| `name` | String | Output only. Name of the `Registration` resource, in the format `projects/*/locations/*/registrations/`. |
| `pending_contact_settings` | String | Output only. Pending contact settings for the `Registration`. Updates to the `contact_settings` field that change its `registrant_contact` or `privacy` fields require email confirmation by the `registrant_contact` before taking effect. This field is set only if there are pending updates to the `contact_settings` that have not been confirmed. To confirm the changes, the `registrant_contact` must follow the instructions in the email they receive. |
| `state` | String | Output only. The state of the `Registration` |
| `supported_privacy` | Vec<String> | Output only. Set of options for the `contact_settings.privacy` field that this `Registration` supports. |
| `domain_name` | String | Required. Immutable. The domain name. Unicode domain names must be expressed in Punycode format. |
| `dns_settings` | String | Settings controlling the DNS configuration of the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureDnsSettings` method. |
| `expire_time` | String | Output only. The expiration timestamp of the `Registration`. |
| `management_settings` | String | Settings for management of the `Registration`, including renewal, billing, and transfer. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureManagementSettings` method. |


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
registration_create_time = registration.create_time
registration_domain_properties = registration.domain_properties
registration_labels = registration.labels
registration_register_failure_reason = registration.register_failure_reason
registration_transfer_failure_reason = registration.transfer_failure_reason
registration_contact_settings = registration.contact_settings
registration_issues = registration.issues
registration_name = registration.name
registration_pending_contact_settings = registration.pending_contact_settings
registration_state = registration.state
registration_supported_privacy = registration.supported_privacy
registration_domain_name = registration.domain_name
registration_dns_settings = registration.dns_settings
registration_expire_time = registration.expire_time
registration_management_settings = registration.management_settings
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_metadata = location.metadata
location_name = location.name
location_labels = location.labels
location_location_id = location.location_id
location_display_name = location.display_name
```

---


### Registration

Resets the authorization code of the `Registration` to a new random string. You can call this method only after 60 days have elapsed since the initial domain registration. Domains that have the `REQUIRE_PUSH_TRANSFER` property in the list of `domain_properties` don't support authorization codes and must use the `InitiatePushTransfer` method to initiate the process to transfer the domain to a different registrar.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `registration` | String | ✅ | Required. The name of the `Registration` whose authorization code is being reset, in the format `projects/*/locations/*/registrations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `domain_name` | String | Required. Immutable. The domain name. Unicode domain names must be expressed in Punycode format. |
| `contact_settings` | String | Required. Settings for contact information linked to the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureContactSettings` method. |
| `state` | String | Output only. The state of the `Registration` |
| `expire_time` | String | Output only. The expiration timestamp of the `Registration`. |
| `register_failure_reason` | String | Output only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state. |
| `issues` | Vec<String> | Output only. The set of issues with the `Registration` that require attention. |
| `transfer_failure_reason` | String | Output only. Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). The reason the domain transfer failed. Only set for domains in TRANSFER_FAILED state. |
| `domain_properties` | Vec<String> | Output only. Special properties of the domain. |
| `create_time` | String | Output only. The creation timestamp of the `Registration` resource. |
| `dns_settings` | String | Settings controlling the DNS configuration of the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureDnsSettings` method. |
| `management_settings` | String | Settings for management of the `Registration`, including renewal, billing, and transfer. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureManagementSettings` method. |
| `pending_contact_settings` | String | Output only. Pending contact settings for the `Registration`. Updates to the `contact_settings` field that change its `registrant_contact` or `privacy` fields require email confirmation by the `registrant_contact` before taking effect. This field is set only if there are pending updates to the `contact_settings` that have not been confirmed. To confirm the changes, the `registrant_contact` must follow the instructions in the email they receive. |
| `provider` | String | Output only. Current domain management provider. |
| `name` | String | Output only. Name of the `Registration` resource, in the format `projects/*/locations/*/registrations/`. |
| `labels` | HashMap<String, String> | Set of labels associated with the `Registration`. |
| `supported_privacy` | Vec<String> | Output only. Set of options for the `contact_settings.privacy` field that this `Registration` supports. |


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
    registration = "value"  # Required. The name of the `Registration` whose authorization code is being reset, in the format `projects/*/locations/*/registrations/*`.
}

# Access registration outputs
registration_id = registration.id
registration_domain_name = registration.domain_name
registration_contact_settings = registration.contact_settings
registration_state = registration.state
registration_expire_time = registration.expire_time
registration_register_failure_reason = registration.register_failure_reason
registration_issues = registration.issues
registration_transfer_failure_reason = registration.transfer_failure_reason
registration_domain_properties = registration.domain_properties
registration_create_time = registration.create_time
registration_dns_settings = registration.dns_settings
registration_management_settings = registration.management_settings
registration_pending_contact_settings = registration.pending_contact_settings
registration_provider = registration.provider
registration_name = registration.name
registration_labels = registration.labels
registration_supported_privacy = registration.supported_privacy
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


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
operation_done = operation.done
operation_response = operation.response
operation_error = operation.error
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_name = location.name
location_display_name = location.display_name
location_metadata = location.metadata
location_location_id = location.location_id
location_labels = location.labels
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
| `transfer_failure_reason` | String | Output only. Deprecated: For more information, see [Cloud Domains feature deprecation](https://cloud.google.com/domains/docs/deprecations/feature-deprecations). The reason the domain transfer failed. Only set for domains in TRANSFER_FAILED state. |
| `name` | String | Output only. Name of the `Registration` resource, in the format `projects/*/locations/*/registrations/`. |
| `management_settings` | String | Settings for management of the `Registration`, including renewal, billing, and transfer. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureManagementSettings` method. |
| `state` | String | Output only. The state of the `Registration` |
| `domain_properties` | Vec<String> | Output only. Special properties of the domain. |
| `issues` | Vec<String> | Output only. The set of issues with the `Registration` that require attention. |
| `create_time` | String | Output only. The creation timestamp of the `Registration` resource. |
| `expire_time` | String | Output only. The expiration timestamp of the `Registration`. |
| `supported_privacy` | Vec<String> | Output only. Set of options for the `contact_settings.privacy` field that this `Registration` supports. |
| `register_failure_reason` | String | Output only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state. |
| `contact_settings` | String | Required. Settings for contact information linked to the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureContactSettings` method. |
| `labels` | HashMap<String, String> | Set of labels associated with the `Registration`. |
| `pending_contact_settings` | String | Output only. Pending contact settings for the `Registration`. Updates to the `contact_settings` field that change its `registrant_contact` or `privacy` fields require email confirmation by the `registrant_contact` before taking effect. This field is set only if there are pending updates to the `contact_settings` that have not been confirmed. To confirm the changes, the `registrant_contact` must follow the instructions in the email they receive. |
| `dns_settings` | String | Settings controlling the DNS configuration of the `Registration`. You cannot update these with the `UpdateRegistration` method. To update these settings, use the `ConfigureDnsSettings` method. |
| `domain_name` | String | Required. Immutable. The domain name. Unicode domain names must be expressed in Punycode format. |


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
registration_transfer_failure_reason = registration.transfer_failure_reason
registration_name = registration.name
registration_management_settings = registration.management_settings
registration_state = registration.state
registration_domain_properties = registration.domain_properties
registration_issues = registration.issues
registration_create_time = registration.create_time
registration_expire_time = registration.expire_time
registration_supported_privacy = registration.supported_privacy
registration_register_failure_reason = registration.register_failure_reason
registration_contact_settings = registration.contact_settings
registration_labels = registration.labels
registration_pending_contact_settings = registration.pending_contact_settings
registration_dns_settings = registration.dns_settings
registration_domain_name = registration.domain_name
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation_name = operation.name
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
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
location_0 = provider.domains_api.Location {
}
location_1 = provider.domains_api.Location {
}
location_2 = provider.domains_api.Location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.domains_api.Location {
    }
```

---

## Related Documentation

- [GCP Domains_api Documentation](https://cloud.google.com/domains_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
