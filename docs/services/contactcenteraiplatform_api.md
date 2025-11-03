# Contactcenteraiplatform_api Service



**Resources**: 3

---

## Overview

The contactcenteraiplatform_api service provides access to 3 resource types:

- [Contact_center](#contact_center) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]

---

## Resources


### Contact_center

Creates a new ContactCenter in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uris` | String |  | Output only. URIs to access the deployed ContactCenters. |
| `early` | String |  | Optional. Early release channel. |
| `saml_params` | String |  | Optional. Params that sets up Google as IdP. |
| `labels` | HashMap<String, String> |  | Labels as key value pairs |
| `admin_user` | String |  | Optional. Info about the first admin user, such as given name and family name. |
| `normal` | String |  | Optional. Normal release channel. |
| `release_version` | String |  | Output only. UJET release version, unique for each new release. |
| `instance_config` | String |  | The configuration of this instance, it is currently immutable once created. |
| `customer_domain_prefix` | String |  | Required. Immutable. At least 2 and max 16 char long, must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). |
| `kms_key` | String |  | Immutable. The KMS key name to encrypt the user input (`ContactCenter`). |
| `user_email` | String |  | Optional. Email address of the first admin user. |
| `name` | String |  | name of resource |
| `private_access` | String |  | Optional. VPC-SC related networking configuration. |
| `critical` | String |  | Optional. Critical release channel. |
| `ccaip_managed_users` | bool |  | Optional. Whether to enable users to be created in the CCAIP-instance concurrently to having users in Cloud identity |
| `feature_config` | String |  | Optional. Feature configuration to populate the feature flags. |
| `create_time` | String |  | Output only. [Output only] Create time stamp |
| `update_time` | String |  | Output only. [Output only] Update time stamp |
| `advanced_reporting_enabled` | bool |  | Optional. Whether the advanced reporting feature is enabled. |
| `private_components` | Vec<String> |  | Output only. TODO(b/283407860) Deprecate this field. |
| `display_name` | String |  | Required. A user friendly name for the ContactCenter. |
| `state` | String |  | Output only. The state of this contact center. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uris` | String | Output only. URIs to access the deployed ContactCenters. |
| `early` | String | Optional. Early release channel. |
| `saml_params` | String | Optional. Params that sets up Google as IdP. |
| `labels` | HashMap<String, String> | Labels as key value pairs |
| `admin_user` | String | Optional. Info about the first admin user, such as given name and family name. |
| `normal` | String | Optional. Normal release channel. |
| `release_version` | String | Output only. UJET release version, unique for each new release. |
| `instance_config` | String | The configuration of this instance, it is currently immutable once created. |
| `customer_domain_prefix` | String | Required. Immutable. At least 2 and max 16 char long, must conform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). |
| `kms_key` | String | Immutable. The KMS key name to encrypt the user input (`ContactCenter`). |
| `user_email` | String | Optional. Email address of the first admin user. |
| `name` | String | name of resource |
| `private_access` | String | Optional. VPC-SC related networking configuration. |
| `critical` | String | Optional. Critical release channel. |
| `ccaip_managed_users` | bool | Optional. Whether to enable users to be created in the CCAIP-instance concurrently to having users in Cloud identity |
| `feature_config` | String | Optional. Feature configuration to populate the feature flags. |
| `create_time` | String | Output only. [Output only] Create time stamp |
| `update_time` | String | Output only. [Output only] Update time stamp |
| `advanced_reporting_enabled` | bool | Optional. Whether the advanced reporting feature is enabled. |
| `private_components` | Vec<String> | Output only. TODO(b/283407860) Deprecate this field. |
| `display_name` | String | Required. A user friendly name for the ContactCenter. |
| `state` | String | Output only. The state of this contact center. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create contact_center
contact_center = provider.contactcenteraiplatform_api.Contact_center {
    parent = "value"  # Required. Value for parent.
}

# Access contact_center outputs
contact_center_id = contact_center.id
contact_center_uris = contact_center.uris
contact_center_early = contact_center.early
contact_center_saml_params = contact_center.saml_params
contact_center_labels = contact_center.labels
contact_center_admin_user = contact_center.admin_user
contact_center_normal = contact_center.normal
contact_center_release_version = contact_center.release_version
contact_center_instance_config = contact_center.instance_config
contact_center_customer_domain_prefix = contact_center.customer_domain_prefix
contact_center_kms_key = contact_center.kms_key
contact_center_user_email = contact_center.user_email
contact_center_name = contact_center.name
contact_center_private_access = contact_center.private_access
contact_center_critical = contact_center.critical
contact_center_ccaip_managed_users = contact_center.ccaip_managed_users
contact_center_feature_config = contact_center.feature_config
contact_center_create_time = contact_center.create_time
contact_center_update_time = contact_center.update_time
contact_center_advanced_reporting_enabled = contact_center.advanced_reporting_enabled
contact_center_private_components = contact_center.private_components
contact_center_display_name = contact_center.display_name
contact_center_state = contact_center.state
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_display_name = location.display_name
location_location_id = location.location_id
location_name = location.name
location_labels = location.labels
location_metadata = location.metadata
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

# Create operation
operation = provider.contactcenteraiplatform_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
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

# Create multiple contact_center resources
contact_center_0 = provider.contactcenteraiplatform_api.Contact_center {
    parent = "value-0"
}
contact_center_1 = provider.contactcenteraiplatform_api.Contact_center {
    parent = "value-1"
}
contact_center_2 = provider.contactcenteraiplatform_api.Contact_center {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    contact_center = provider.contactcenteraiplatform_api.Contact_center {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Contactcenteraiplatform_api Documentation](https://cloud.google.com/contactcenteraiplatform_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
