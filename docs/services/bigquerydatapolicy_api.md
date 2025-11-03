# Bigquerydatapolicy_api Service



**Resources**: 2

---

## Overview

The bigquerydatapolicy_api service provides access to 2 resource types:

- [Data_policie](#data_policie) [CRUD]
- [Data_policie](#data_policie) [CRUD]

---

## Resources


### Data_policie

Creates a new data policy under a project with the given `dataPolicyId` (used as the display name), policy tag, and data policy type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of this data policy, in the format of `projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}`. |
| `data_masking_policy` | String |  | The data masking policy that specifies the data masking rule to use. |
| `data_policy_type` | String |  | Required. Data policy type. Type of data policy. |
| `policy_tag` | String |  | Policy tag resource name, in the format of `projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{policyTag_id}`. |
| `data_policy_id` | String |  | User-assigned (human readable) ID of the data policy that needs to be unique within a project. Used as {data_policy_id} in part of the resource name. |
| `parent` | String | ✅ | Required. Resource name of the project that the data policy will belong to. The format is `projects/{project_number}/locations/{location_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of this data policy, in the format of `projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}`. |
| `data_masking_policy` | String | The data masking policy that specifies the data masking rule to use. |
| `data_policy_type` | String | Required. Data policy type. Type of data policy. |
| `policy_tag` | String | Policy tag resource name, in the format of `projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{policyTag_id}`. |
| `data_policy_id` | String | User-assigned (human readable) ID of the data policy that needs to be unique within a project. Used as {data_policy_id} in part of the resource name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_policie
data_policie = provider.bigquerydatapolicy_api.Data_policie {
    parent = "value"  # Required. Resource name of the project that the data policy will belong to. The format is `projects/{project_number}/locations/{location_id}`.
}

# Access data_policie outputs
data_policie_id = data_policie.id
data_policie_name = data_policie.name
data_policie_data_masking_policy = data_policie.data_masking_policy
data_policie_data_policy_type = data_policie.data_policy_type
data_policie_policy_tag = data_policie.policy_tag
data_policie_data_policy_id = data_policie.data_policy_id
```

---


### Data_policie

Creates a new data policy under a project with the given `data_policy_id` (used as the display name), and data policy type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_policy` | String |  | Required. The data policy to create. The `name` field does not need to be provided for the data policy creation. |
| `data_policy_id` | String |  | Required. User-assigned (human readable) ID of the data policy that needs to be unique within a project. Used as {data_policy_id} in part of the resource name. |
| `parent` | String | ✅ | Required. Resource name of the project that the data policy will belong to. The format is `projects/{project_number}/locations/{location_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_masking_policy` | String | Optional. The data masking policy that specifies the data masking rule to use. It must be set if the data policy type is DATA_MASKING_POLICY. |
| `data_policy_type` | String | Required. Type of data policy. |
| `policy_tag` | String | Output only. Policy tag resource name, in the format of `projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{policyTag_id}`. policy_tag is supported only for V1 data policies. |
| `version` | String | Output only. The version of the Data Policy resource. |
| `name` | String | Identifier. Resource name of this data policy, in the format of `projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}`. |
| `grantees` | Vec<String> | Optional. The list of IAM principals that have Fine Grained Access to the underlying data goverened by this data policy. Uses the [IAM V2 principal syntax](https://cloud.google.com/iam/docs/principal-identifiers#v2) Only supports principal types users, groups, serviceaccounts, cloudidentity. This field is supported in V2 Data Policy only. In case of V1 data policies (i.e. verion = 1 and policy_tag is set), this field is not populated. |
| `etag` | String | The etag for this Data Policy. This field is used for UpdateDataPolicy calls. If Data Policy exists, this field is required and must match the server's etag. It will also be populated in the response of GetDataPolicy, CreateDataPolicy, and UpdateDataPolicy calls. |
| `data_policy_id` | String | Output only. User-assigned (human readable) ID of the data policy that needs to be unique within a project. Used as {data_policy_id} in part of the resource name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create data_policie
data_policie = provider.bigquerydatapolicy_api.Data_policie {
    parent = "value"  # Required. Resource name of the project that the data policy will belong to. The format is `projects/{project_number}/locations/{location_id}`.
}

# Access data_policie outputs
data_policie_id = data_policie.id
data_policie_data_masking_policy = data_policie.data_masking_policy
data_policie_data_policy_type = data_policie.data_policy_type
data_policie_policy_tag = data_policie.policy_tag
data_policie_version = data_policie.version
data_policie_name = data_policie.name
data_policie_grantees = data_policie.grantees
data_policie_etag = data_policie.etag
data_policie_data_policy_id = data_policie.data_policy_id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple data_policie resources
data_policie_0 = provider.bigquerydatapolicy_api.Data_policie {
    parent = "value-0"
}
data_policie_1 = provider.bigquerydatapolicy_api.Data_policie {
    parent = "value-1"
}
data_policie_2 = provider.bigquerydatapolicy_api.Data_policie {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    data_policie = provider.bigquerydatapolicy_api.Data_policie {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Bigquerydatapolicy_api Documentation](https://cloud.google.com/bigquerydatapolicy_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
