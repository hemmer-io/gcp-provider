# Bigqueryconnection_api Service



**Resources**: 2

---

## Overview

The bigqueryconnection_api service provides access to 2 resource types:

- [Connection](#connection) [CRUD]
- [Connection](#connection) [CRUD]

---

## Resources


### Connection

Creates a new connection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `has_credential` | bool |  | Output only. True, if credential is configured for this connection. |
| `name` | String |  | Output only. The resource name of the connection in the form of: `projects/{project_id}/locations/{location_id}/connections/{connection_id}` |
| `aws` | String |  | Amazon Web Services (AWS) properties. |
| `cloud_sql` | String |  | Cloud SQL properties. |
| `kms_key_name` | String |  | Optional. The Cloud KMS key that is used for credentials encryption. If omitted, internal Google owned encryption keys are used. Example: `projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]` |
| `description` | String |  | User provided description. |
| `azure` | String |  | Azure properties. |
| `creation_time` | String |  | Output only. The creation timestamp of the connection. |
| `last_modified_time` | String |  | Output only. The last update timestamp of the connection. |
| `friendly_name` | String |  | User provided display name for the connection. |
| `cloud_spanner` | String |  | Cloud Spanner properties. |
| `spark` | String |  | Spark properties. |
| `cloud_resource` | String |  | Cloud Resource properties. |
| `salesforce_data_cloud` | String |  | Optional. Salesforce DataCloud properties. This field is intended for use only by Salesforce partner projects. This field contains properties for your Salesforce DataCloud connection. |
| `configuration` | String |  | Optional. Connector configuration. |
| `parent` | String | ✅ | Required. Parent resource name. Must be in the format `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `has_credential` | bool | Output only. True, if credential is configured for this connection. |
| `name` | String | Output only. The resource name of the connection in the form of: `projects/{project_id}/locations/{location_id}/connections/{connection_id}` |
| `aws` | String | Amazon Web Services (AWS) properties. |
| `cloud_sql` | String | Cloud SQL properties. |
| `kms_key_name` | String | Optional. The Cloud KMS key that is used for credentials encryption. If omitted, internal Google owned encryption keys are used. Example: `projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]` |
| `description` | String | User provided description. |
| `azure` | String | Azure properties. |
| `creation_time` | String | Output only. The creation timestamp of the connection. |
| `last_modified_time` | String | Output only. The last update timestamp of the connection. |
| `friendly_name` | String | User provided display name for the connection. |
| `cloud_spanner` | String | Cloud Spanner properties. |
| `spark` | String | Spark properties. |
| `cloud_resource` | String | Cloud Resource properties. |
| `salesforce_data_cloud` | String | Optional. Salesforce DataCloud properties. This field is intended for use only by Salesforce partner projects. This field contains properties for your Salesforce DataCloud connection. |
| `configuration` | String | Optional. Connector configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection
connection = provider.bigqueryconnection_api.Connection {
    parent = "value"  # Required. Parent resource name. Must be in the format `projects/{project_id}/locations/{location_id}`
}

# Access connection outputs
connection_id = connection.id
connection_has_credential = connection.has_credential
connection_name = connection.name
connection_aws = connection.aws
connection_cloud_sql = connection.cloud_sql
connection_kms_key_name = connection.kms_key_name
connection_description = connection.description
connection_azure = connection.azure
connection_creation_time = connection.creation_time
connection_last_modified_time = connection.last_modified_time
connection_friendly_name = connection.friendly_name
connection_cloud_spanner = connection.cloud_spanner
connection_spark = connection.spark
connection_cloud_resource = connection.cloud_resource
connection_salesforce_data_cloud = connection.salesforce_data_cloud
connection_configuration = connection.configuration
```

---


### Connection

Creates a new connection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `friendly_name` | String |  | User provided display name for the connection. |
| `has_credential` | bool |  | Output only. True, if credential is configured for this connection. |
| `last_modified_time` | String |  | Output only. The last update timestamp of the connection. |
| `name` | String |  | The resource name of the connection in the form of: `projects/{project_id}/locations/{location_id}/connections/{connection_id}` |
| `cloud_sql` | String |  | Cloud SQL properties. |
| `description` | String |  | User provided description. |
| `creation_time` | String |  | Output only. The creation timestamp of the connection. |
| `parent` | String | ✅ | Required. Parent resource name. Must be in the format `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `friendly_name` | String | User provided display name for the connection. |
| `has_credential` | bool | Output only. True, if credential is configured for this connection. |
| `last_modified_time` | String | Output only. The last update timestamp of the connection. |
| `name` | String | The resource name of the connection in the form of: `projects/{project_id}/locations/{location_id}/connections/{connection_id}` |
| `cloud_sql` | String | Cloud SQL properties. |
| `description` | String | User provided description. |
| `creation_time` | String | Output only. The creation timestamp of the connection. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection
connection = provider.bigqueryconnection_api.Connection {
    parent = "value"  # Required. Parent resource name. Must be in the format `projects/{project_id}/locations/{location_id}`
}

# Access connection outputs
connection_id = connection.id
connection_friendly_name = connection.friendly_name
connection_has_credential = connection.has_credential
connection_last_modified_time = connection.last_modified_time
connection_name = connection.name
connection_cloud_sql = connection.cloud_sql
connection_description = connection.description
connection_creation_time = connection.creation_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple connection resources
connection_0 = provider.bigqueryconnection_api.Connection {
    parent = "value-0"
}
connection_1 = provider.bigqueryconnection_api.Connection {
    parent = "value-1"
}
connection_2 = provider.bigqueryconnection_api.Connection {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    connection = provider.bigqueryconnection_api.Connection {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Bigqueryconnection_api Documentation](https://cloud.google.com/bigqueryconnection_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
