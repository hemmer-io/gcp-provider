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
| `cloud_resource` | String |  | Cloud Resource properties. |
| `last_modified_time` | String |  | Output only. The last update timestamp of the connection. |
| `salesforce_data_cloud` | String |  | Optional. Salesforce DataCloud properties. This field is intended for use only by Salesforce partner projects. This field contains properties for your Salesforce DataCloud connection. |
| `cloud_sql` | String |  | Cloud SQL properties. |
| `aws` | String |  | Amazon Web Services (AWS) properties. |
| `creation_time` | String |  | Output only. The creation timestamp of the connection. |
| `friendly_name` | String |  | User provided display name for the connection. |
| `azure` | String |  | Azure properties. |
| `configuration` | String |  | Optional. Connector configuration. |
| `description` | String |  | User provided description. |
| `has_credential` | bool |  | Output only. True, if credential is configured for this connection. |
| `name` | String |  | Output only. The resource name of the connection in the form of: `projects/{project_id}/locations/{location_id}/connections/{connection_id}` |
| `spark` | String |  | Spark properties. |
| `cloud_spanner` | String |  | Cloud Spanner properties. |
| `kms_key_name` | String |  | Optional. The Cloud KMS key that is used for credentials encryption. If omitted, internal Google owned encryption keys are used. Example: `projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]` |
| `parent` | String | ✅ | Required. Parent resource name. Must be in the format `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_resource` | String | Cloud Resource properties. |
| `last_modified_time` | String | Output only. The last update timestamp of the connection. |
| `salesforce_data_cloud` | String | Optional. Salesforce DataCloud properties. This field is intended for use only by Salesforce partner projects. This field contains properties for your Salesforce DataCloud connection. |
| `cloud_sql` | String | Cloud SQL properties. |
| `aws` | String | Amazon Web Services (AWS) properties. |
| `creation_time` | String | Output only. The creation timestamp of the connection. |
| `friendly_name` | String | User provided display name for the connection. |
| `azure` | String | Azure properties. |
| `configuration` | String | Optional. Connector configuration. |
| `description` | String | User provided description. |
| `has_credential` | bool | Output only. True, if credential is configured for this connection. |
| `name` | String | Output only. The resource name of the connection in the form of: `projects/{project_id}/locations/{location_id}/connections/{connection_id}` |
| `spark` | String | Spark properties. |
| `cloud_spanner` | String | Cloud Spanner properties. |
| `kms_key_name` | String | Optional. The Cloud KMS key that is used for credentials encryption. If omitted, internal Google owned encryption keys are used. Example: `projects/[kms_project_id]/locations/[region]/keyRings/[key_region]/cryptoKeys/[key]` |


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
connection_cloud_resource = connection.cloud_resource
connection_last_modified_time = connection.last_modified_time
connection_salesforce_data_cloud = connection.salesforce_data_cloud
connection_cloud_sql = connection.cloud_sql
connection_aws = connection.aws
connection_creation_time = connection.creation_time
connection_friendly_name = connection.friendly_name
connection_azure = connection.azure
connection_configuration = connection.configuration
connection_description = connection.description
connection_has_credential = connection.has_credential
connection_name = connection.name
connection_spark = connection.spark
connection_cloud_spanner = connection.cloud_spanner
connection_kms_key_name = connection.kms_key_name
```

---


### Connection

Creates a new connection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cloud_sql` | String |  | Cloud SQL properties. |
| `has_credential` | bool |  | Output only. True, if credential is configured for this connection. |
| `name` | String |  | The resource name of the connection in the form of: `projects/{project_id}/locations/{location_id}/connections/{connection_id}` |
| `creation_time` | String |  | Output only. The creation timestamp of the connection. |
| `last_modified_time` | String |  | Output only. The last update timestamp of the connection. |
| `description` | String |  | User provided description. |
| `friendly_name` | String |  | User provided display name for the connection. |
| `parent` | String | ✅ | Required. Parent resource name. Must be in the format `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cloud_sql` | String | Cloud SQL properties. |
| `has_credential` | bool | Output only. True, if credential is configured for this connection. |
| `name` | String | The resource name of the connection in the form of: `projects/{project_id}/locations/{location_id}/connections/{connection_id}` |
| `creation_time` | String | Output only. The creation timestamp of the connection. |
| `last_modified_time` | String | Output only. The last update timestamp of the connection. |
| `description` | String | User provided description. |
| `friendly_name` | String | User provided display name for the connection. |


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
connection_cloud_sql = connection.cloud_sql
connection_has_credential = connection.has_credential
connection_name = connection.name
connection_creation_time = connection.creation_time
connection_last_modified_time = connection.last_modified_time
connection_description = connection.description
connection_friendly_name = connection.friendly_name
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
