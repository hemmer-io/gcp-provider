# Resourcesettings_api Service



**Resources**: 1

---

## Overview

The resourcesettings_api service provides access to 1 resource type:

- [Setting](#setting) [RU]

---

## Resources


### Setting

Returns a specified setting. Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the setting does not exist.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The resource name of the setting. Must be in one of the following forms: * `projects/{project_number}/settings/{setting_name}` * `folders/{folder_id}/settings/{setting_name}` * `organizations/{organization_id}/settings/{setting_name}` For example, "/projects/123/settings/gcp-enableMyFeature" |
| `effective_value` | String |  | Output only. The effective value of the setting at the given parent resource, evaluated based on the resource hierarchy The effective value evaluates to one of the following options, in this order. If an option is not valid or doesn't exist, then the next option is used: 1. The local setting value on the given resource: Setting.local_value 2. If one of the given resource's ancestors in the resource hierarchy have a local setting value, the local value at the nearest such ancestor. 3. The setting's default value: SettingMetadata.default_value 4. An empty value, defined as a `Value` with all fields unset. The data type of Value must always be consistent with the data type defined in Setting.metadata. |
| `metadata` | String |  | Output only. Metadata about a setting which is not editable by the end user. |
| `local_value` | String |  | The configured value of the setting at the given parent resource, ignoring the resource hierarchy. The data type of Value must always be consistent with the data type defined in Setting.metadata. |
| `etag` | String |  | A fingerprint used for optimistic concurrency. See UpdateSetting for more details. |
| `name` | String | ✅ | The resource name of the setting. Must be in one of the following forms: * `projects/{project_number}/settings/{setting_name}` * `folders/{folder_id}/settings/{setting_name}` * `organizations/{organization_id}/settings/{setting_name}` For example, "/projects/123/settings/gcp-enableMyFeature" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the setting. Must be in one of the following forms: * `projects/{project_number}/settings/{setting_name}` * `folders/{folder_id}/settings/{setting_name}` * `organizations/{organization_id}/settings/{setting_name}` For example, "/projects/123/settings/gcp-enableMyFeature" |
| `effective_value` | String | Output only. The effective value of the setting at the given parent resource, evaluated based on the resource hierarchy The effective value evaluates to one of the following options, in this order. If an option is not valid or doesn't exist, then the next option is used: 1. The local setting value on the given resource: Setting.local_value 2. If one of the given resource's ancestors in the resource hierarchy have a local setting value, the local value at the nearest such ancestor. 3. The setting's default value: SettingMetadata.default_value 4. An empty value, defined as a `Value` with all fields unset. The data type of Value must always be consistent with the data type defined in Setting.metadata. |
| `metadata` | String | Output only. Metadata about a setting which is not editable by the end user. |
| `local_value` | String | The configured value of the setting at the given parent resource, ignoring the resource hierarchy. The data type of Value must always be consistent with the data type defined in Setting.metadata. |
| `etag` | String | A fingerprint used for optimistic concurrency. See UpdateSetting for more details. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access setting outputs
setting_id = setting.id
setting_name = setting.name
setting_effective_value = setting.effective_value
setting_metadata = setting.metadata
setting_local_value = setting.local_value
setting_etag = setting.etag
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple setting resources
setting_0 = provider.resourcesettings_api.Setting {
    name = "value-0"
}
setting_1 = provider.resourcesettings_api.Setting {
    name = "value-1"
}
setting_2 = provider.resourcesettings_api.Setting {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    setting = provider.resourcesettings_api.Setting {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Resourcesettings_api Documentation](https://cloud.google.com/resourcesettings_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
