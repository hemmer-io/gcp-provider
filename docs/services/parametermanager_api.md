# Parametermanager_api Service



**Resources**: 3

---

## Overview

The parametermanager_api service provides access to 3 resource types:

- [Location](#location) [R]
- [Parameter](#parameter) [CRUD]
- [Version](#version) [CRUD]

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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
```

---


### Parameter

Creates a new Parameter in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. [Output only] Update time stamp |
| `format` | String |  | Optional. Specifies the format of a Parameter. |
| `kms_key` | String |  | Optional. Customer managed encryption key (CMEK) to use for encrypting the Parameter Versions. If not set, the default Google-managed encryption key will be used. Cloud KMS CryptoKeys must reside in the same location as the Parameter. The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `policy_member` | String |  | Output only. [Output-only] policy member strings of a Google Cloud resource. |
| `create_time` | String |  | Output only. [Output only] Create time stamp |
| `name` | String |  | Identifier. [Output only] The resource name of the Parameter in the format `projects/*/locations/*/parameters/*`. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `parent` | String | ✅ | Required. Value for parent in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. [Output only] Update time stamp |
| `format` | String | Optional. Specifies the format of a Parameter. |
| `kms_key` | String | Optional. Customer managed encryption key (CMEK) to use for encrypting the Parameter Versions. If not set, the default Google-managed encryption key will be used. Cloud KMS CryptoKeys must reside in the same location as the Parameter. The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `policy_member` | String | Output only. [Output-only] policy member strings of a Google Cloud resource. |
| `create_time` | String | Output only. [Output only] Create time stamp |
| `name` | String | Identifier. [Output only] The resource name of the Parameter in the format `projects/*/locations/*/parameters/*`. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create parameter
parameter = provider.parametermanager_api.Parameter {
    parent = "value"  # Required. Value for parent in the format `projects/*/locations/*`.
}

# Access parameter outputs
parameter_id = parameter.id
parameter_update_time = parameter.update_time
parameter_format = parameter.format
parameter_kms_key = parameter.kms_key
parameter_policy_member = parameter.policy_member
parameter_create_time = parameter.create_time
parameter_name = parameter.name
parameter_labels = parameter.labels
```

---


### Version

Creates a new ParameterVersion in a given project, location, and parameter.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. [Output only] Create time stamp |
| `name` | String |  | Identifier. [Output only] The resource name of the ParameterVersion in the format `projects/*/locations/*/parameters/*/versions/*`. |
| `disabled` | bool |  | Optional. Disabled boolean to determine if a ParameterVersion acts as a metadata only resource (payload is never returned if disabled is true). If true any calls will always default to BASIC view even if the user explicitly passes FULL view as part of the request. A render call on a disabled resource fails with an error. Default value is False. |
| `kms_key_version` | String |  | Optional. Output only. [Output only] The resource name of the KMS key version used to encrypt the ParameterVersion payload. This field is populated only if the Parameter resource has customer managed encryption key (CMEK) configured. |
| `payload` | String |  | Required. Immutable. Payload content of a ParameterVersion resource. This is only returned when the request provides the View value of FULL (default for GET request). |
| `update_time` | String |  | Output only. [Output only] Update time stamp |
| `parent` | String | ✅ | Required. Value for parent in the format `projects/*/locations/*/parameters/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. [Output only] Create time stamp |
| `name` | String | Identifier. [Output only] The resource name of the ParameterVersion in the format `projects/*/locations/*/parameters/*/versions/*`. |
| `disabled` | bool | Optional. Disabled boolean to determine if a ParameterVersion acts as a metadata only resource (payload is never returned if disabled is true). If true any calls will always default to BASIC view even if the user explicitly passes FULL view as part of the request. A render call on a disabled resource fails with an error. Default value is False. |
| `kms_key_version` | String | Optional. Output only. [Output only] The resource name of the KMS key version used to encrypt the ParameterVersion payload. This field is populated only if the Parameter resource has customer managed encryption key (CMEK) configured. |
| `payload` | String | Required. Immutable. Payload content of a ParameterVersion resource. This is only returned when the request provides the View value of FULL (default for GET request). |
| `update_time` | String | Output only. [Output only] Update time stamp |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.parametermanager_api.Version {
    parent = "value"  # Required. Value for parent in the format `projects/*/locations/*/parameters/*`.
}

# Access version outputs
version_id = version.id
version_create_time = version.create_time
version_name = version.name
version_disabled = version.disabled
version_kms_key_version = version.kms_key_version
version_payload = version.payload
version_update_time = version.update_time
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
location_0 = provider.parametermanager_api.Location {
}
location_1 = provider.parametermanager_api.Location {
}
location_2 = provider.parametermanager_api.Location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.parametermanager_api.Location {
    }
```

---

## Related Documentation

- [GCP Parametermanager_api Documentation](https://cloud.google.com/parametermanager_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
