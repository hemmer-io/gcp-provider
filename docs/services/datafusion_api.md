# Datafusion_api Service



**Resources**: 11

---

## Overview

The datafusion_api service provides access to 11 resource types:

- [Dns_peering](#dns_peering) [CRD]
- [Location](#location) [R]
- [Instance](#instance) [CRUD]
- [Operation](#operation) [CRD]
- [Version](#version) [R]
- [Location](#location) [CR]
- [Instance](#instance) [CRUD]
- [Version](#version) [R]
- [Namespace](#namespace) [CR]
- [Operation](#operation) [CRD]
- [Dns_peering](#dns_peering) [CRD]

---

## Resources


### Dns_peering

Creates DNS peering on the given resource.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target_network` | String |  | Optional. Optional target network to which dns peering should happen. |
| `target_project` | String |  | Optional. Optional target project to which dns peering should happen. |
| `description` | String |  | Optional. Optional description of the dns zone. |
| `domain` | String |  | Required. The dns name suffix of the zone. |
| `name` | String |  | Identifier. The resource name of the dns peering zone. Format: projects/{project}/locations/{location}/instances/{instance}/dnsPeerings/{dns_peering} |
| `parent` | String | ✅ | Required. The resource on which DNS peering will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `dns_peerings` | Vec<String> | List of dns peering. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dns_peering
dns_peering = provider.datafusion_api.Dns_peering {
    parent = "value"  # Required. The resource on which DNS peering will be created.
}

# Access dns_peering outputs
dns_peering_id = dns_peering.id
dns_peering_next_page_token = dns_peering.next_page_token
dns_peering_dns_peerings = dns_peering.dns_peerings
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
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
location_labels = location.labels
location_display_name = location.display_name
```

---


### Instance

Creates a new Data Fusion instance in the specified project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state_message` | String |  | Output only. Additional information about the current state of this Data Fusion instance if available. |
| `version` | String |  | Optional. Current version of the Data Fusion. Only specifiable in Update. |
| `enable_zone_separation` | bool |  | Output only. Option to enable granular zone separation. |
| `workforce_identity_service_endpoint` | String |  | Output only. Endpoint on which the Data Fusion UI is accessible to third-party users |
| `zone` | String |  | Optional. Name of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field. |
| `logging_config` | String |  | Optional. The logging configuration for this instance. This field is supported only in CDF versions 6.11.0 and above. |
| `private_instance` | bool |  | Optional. Specifies whether the Data Fusion instance should be private. If set to true, all Data Fusion nodes will have private IP addresses and will not be able to access the public internet. |
| `service_endpoint` | String |  | Output only. Endpoint on which the Data Fusion UI is accessible. |
| `description` | String |  | Optional. A description of this instance. |
| `available_version` | Vec<String> |  | Output only. Available versions that the instance can be upgraded to using UpdateInstanceRequest. |
| `tenant_project_id` | String |  | Output only. The name of the tenant project. |
| `options` | HashMap<String, String> |  | Optional. Map of additional options used to configure the behavior of Data Fusion instance. |
| `service_account` | String |  | Output only. Deprecated. Use tenant_project_id instead to extract the tenant project ID. |
| `accelerators` | Vec<String> |  | Output only. List of accelerators enabled for this CDF instance. |
| `enable_stackdriver_logging` | bool |  | Optional. Option to enable Dataproc Stackdriver Logging. |
| `p4_service_account` | String |  | Output only. Service agent for the customer project. |
| `labels` | HashMap<String, String> |  | The resource labels for instance to use to annotate any related underlying resources such as Compute Engine VMs. The character '=' is not allowed to be used within the labels. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `state` | String |  | Output only. The current state of this Data Fusion instance. |
| `event_publish_config` | String |  | Optional. Option to enable and pass metadata for event publishing. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `display_name` | String |  | Optional. Display name for an instance. |
| `name` | String |  | Output only. The name of this instance is in the form of projects/{project}/locations/{location}/instances/{instance}. |
| `enable_rbac` | bool |  | Optional. Option to enable granular role-based access control. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `gcs_bucket` | String |  | Output only. Cloud Storage bucket generated by Data Fusion in the customer project. |
| `dataplex_data_lineage_integration_enabled` | bool |  | Optional. Option to enable the Dataplex Lineage Integration feature. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `type` | String |  | Required. Instance type. |
| `disabled_reason` | Vec<String> |  | Output only. If the instance state is DISABLED, the reason for disabling the instance. |
| `network_config` | String |  | Optional. Network configuration options. These are required when a private Data Fusion instance is to be created. |
| `patch_revision` | String |  | Optional. Current patch revision of the Data Fusion. |
| `api_endpoint` | String |  | Output only. Endpoint on which the REST APIs is accessible. |
| `enable_stackdriver_monitoring` | bool |  | Optional. Option to enable Stackdriver Monitoring. |
| `update_time` | String |  | Output only. The time the instance was last updated. |
| `dataproc_service_account` | String |  | Optional. User-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines. This allows users to have fine-grained access control on Dataproc's accesses to cloud resources. |
| `maintenance_events` | Vec<String> |  | Output only. The maintenance events for this instance. |
| `maintenance_policy` | String |  | Optional. Configure the maintenance policy for this instance. |
| `crypto_key_config` | String |  | Optional. The crypto key configuration. This field is used by the Customer-Managed Encryption Keys (CMEK) feature. |
| `parent` | String | ✅ | Required. The instance's project and location in the format projects/{project}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state_message` | String | Output only. Additional information about the current state of this Data Fusion instance if available. |
| `version` | String | Optional. Current version of the Data Fusion. Only specifiable in Update. |
| `enable_zone_separation` | bool | Output only. Option to enable granular zone separation. |
| `workforce_identity_service_endpoint` | String | Output only. Endpoint on which the Data Fusion UI is accessible to third-party users |
| `zone` | String | Optional. Name of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field. |
| `logging_config` | String | Optional. The logging configuration for this instance. This field is supported only in CDF versions 6.11.0 and above. |
| `private_instance` | bool | Optional. Specifies whether the Data Fusion instance should be private. If set to true, all Data Fusion nodes will have private IP addresses and will not be able to access the public internet. |
| `service_endpoint` | String | Output only. Endpoint on which the Data Fusion UI is accessible. |
| `description` | String | Optional. A description of this instance. |
| `available_version` | Vec<String> | Output only. Available versions that the instance can be upgraded to using UpdateInstanceRequest. |
| `tenant_project_id` | String | Output only. The name of the tenant project. |
| `options` | HashMap<String, String> | Optional. Map of additional options used to configure the behavior of Data Fusion instance. |
| `service_account` | String | Output only. Deprecated. Use tenant_project_id instead to extract the tenant project ID. |
| `accelerators` | Vec<String> | Output only. List of accelerators enabled for this CDF instance. |
| `enable_stackdriver_logging` | bool | Optional. Option to enable Dataproc Stackdriver Logging. |
| `p4_service_account` | String | Output only. Service agent for the customer project. |
| `labels` | HashMap<String, String> | The resource labels for instance to use to annotate any related underlying resources such as Compute Engine VMs. The character '=' is not allowed to be used within the labels. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `state` | String | Output only. The current state of this Data Fusion instance. |
| `event_publish_config` | String | Optional. Option to enable and pass metadata for event publishing. |
| `create_time` | String | Output only. The time the instance was created. |
| `display_name` | String | Optional. Display name for an instance. |
| `name` | String | Output only. The name of this instance is in the form of projects/{project}/locations/{location}/instances/{instance}. |
| `enable_rbac` | bool | Optional. Option to enable granular role-based access control. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `gcs_bucket` | String | Output only. Cloud Storage bucket generated by Data Fusion in the customer project. |
| `dataplex_data_lineage_integration_enabled` | bool | Optional. Option to enable the Dataplex Lineage Integration feature. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `type` | String | Required. Instance type. |
| `disabled_reason` | Vec<String> | Output only. If the instance state is DISABLED, the reason for disabling the instance. |
| `network_config` | String | Optional. Network configuration options. These are required when a private Data Fusion instance is to be created. |
| `patch_revision` | String | Optional. Current patch revision of the Data Fusion. |
| `api_endpoint` | String | Output only. Endpoint on which the REST APIs is accessible. |
| `enable_stackdriver_monitoring` | bool | Optional. Option to enable Stackdriver Monitoring. |
| `update_time` | String | Output only. The time the instance was last updated. |
| `dataproc_service_account` | String | Optional. User-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines. This allows users to have fine-grained access control on Dataproc's accesses to cloud resources. |
| `maintenance_events` | Vec<String> | Output only. The maintenance events for this instance. |
| `maintenance_policy` | String | Optional. Configure the maintenance policy for this instance. |
| `crypto_key_config` | String | Optional. The crypto key configuration. This field is used by the Customer-Managed Encryption Keys (CMEK) feature. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.datafusion_api.Instance {
    parent = "value"  # Required. The instance's project and location in the format projects/{project}/locations/{location}.
}

# Access instance outputs
instance_id = instance.id
instance_state_message = instance.state_message
instance_version = instance.version
instance_enable_zone_separation = instance.enable_zone_separation
instance_workforce_identity_service_endpoint = instance.workforce_identity_service_endpoint
instance_zone = instance.zone
instance_logging_config = instance.logging_config
instance_private_instance = instance.private_instance
instance_service_endpoint = instance.service_endpoint
instance_description = instance.description
instance_available_version = instance.available_version
instance_tenant_project_id = instance.tenant_project_id
instance_options = instance.options
instance_service_account = instance.service_account
instance_accelerators = instance.accelerators
instance_enable_stackdriver_logging = instance.enable_stackdriver_logging
instance_p4_service_account = instance.p4_service_account
instance_labels = instance.labels
instance_tags = instance.tags
instance_state = instance.state
instance_event_publish_config = instance.event_publish_config
instance_create_time = instance.create_time
instance_display_name = instance.display_name
instance_name = instance.name
instance_enable_rbac = instance.enable_rbac
instance_satisfies_pzs = instance.satisfies_pzs
instance_gcs_bucket = instance.gcs_bucket
instance_dataplex_data_lineage_integration_enabled = instance.dataplex_data_lineage_integration_enabled
instance_satisfies_pzi = instance.satisfies_pzi
instance_type = instance.type
instance_disabled_reason = instance.disabled_reason
instance_network_config = instance.network_config
instance_patch_revision = instance.patch_revision
instance_api_endpoint = instance.api_endpoint
instance_enable_stackdriver_monitoring = instance.enable_stackdriver_monitoring
instance_update_time = instance.update_time
instance_dataproc_service_account = instance.dataproc_service_account
instance_maintenance_events = instance.maintenance_events
instance_maintenance_policy = instance.maintenance_policy
instance_crypto_key_config = instance.crypto_key_config
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.datafusion_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_response = operation.response
```

---


### Version

Lists possible versions for Data Fusion instances in the specified project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `versions` | Vec<String> | Represents a list of all versions. |
| `available_versions` | Vec<String> | Represents a list of versions that are supported. Deprecated: Use versions field instead. |
| `next_page_token` | String | Token to retrieve the next page of results or empty if there are no more results in the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access version outputs
version_id = version.id
version_versions = version.versions
version_available_versions = version.available_versions
version_next_page_token = version.next_page_token
```

---


### Location

Remove IAM policy that is currently set on the given resource.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource` | String | ✅ | Required. The resource on which IAM policy to be removed is attached to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.datafusion_api.Location {
    resource = "value"  # Required. The resource on which IAM policy to be removed is attached to.
}

# Access location outputs
location_id = location.id
location_name = location.name
location_location_id = location.location_id
location_display_name = location.display_name
location_metadata = location.metadata
location_labels = location.labels
```

---


### Instance

Creates a new Data Fusion instance in the specified project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tenant_project_id` | String |  | Output only. The name of the tenant project. |
| `workforce_identity_service_endpoint` | String |  | Output only. Endpoint on which the Data Fusion UI is accessible to third-party users. |
| `version` | String |  | Optional. Current version of Data Fusion. |
| `state` | String |  | Output only. The current state of this Data Fusion instance. |
| `enable_stackdriver_monitoring` | bool |  | Optional. Option to enable Stackdriver Monitoring. |
| `description` | String |  | Optional. A description of this instance. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `patch_revision` | String |  | Optional. Current patch revision of the Data Fusion. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `service_account` | String |  | Output only. Deprecated. Use tenant_project_id instead to extract the tenant project ID. |
| `display_name` | String |  | Optional. Display name for an instance. |
| `options` | HashMap<String, String> |  | Optional. Map of additional options used to configure the behavior of Data Fusion instance. |
| `accelerators` | Vec<String> |  | Output only. List of accelerators enabled for this CDF instance. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `gcs_bucket` | String |  | Output only. Cloud Storage bucket generated by Data Fusion in the customer project. |
| `dataplex_data_lineage_integration_enabled` | bool |  | Optional. Option to enable the Dataplex Lineage Integration feature. |
| `labels` | HashMap<String, String> |  | The resource labels for instance to use to annotate any related underlying resources such as Compute Engine VMs. The character '=' is not allowed to be used within the labels. |
| `p4_service_account` | String |  | Output only. Service agent for the customer project. |
| `maintenance_policy` | String |  | Optional. Configure the maintenance policy for this instance. |
| `disabled_reason` | Vec<String> |  | Output only. If the instance state is DISABLED, the reason for disabling the instance. |
| `logging_config` | String |  | Optional. The logging configuration for this instance. This field is supported only in CDF versions 6.11.0 and above. |
| `enable_rbac` | bool |  | Optional. Option to enable granular role-based access control. |
| `state_message` | String |  | Output only. Additional information about the current state of this Data Fusion instance if available. |
| `available_version` | Vec<String> |  | Output only. Available versions that the instance can be upgraded to using UpdateInstanceRequest. |
| `enable_zone_separation` | bool |  | Output only. Option to enable zone separation. |
| `private_instance` | bool |  | Optional. Specifies whether the Data Fusion instance should be private. If set to true, all Data Fusion nodes will have private IP addresses and will not be able to access the public internet. |
| `update_time` | String |  | Output only. The time the instance was last updated. |
| `enable_stackdriver_logging` | bool |  | Optional. Option to enable Dataproc Stackdriver Logging. |
| `name` | String |  | Output only. The name of this instance is in the form of projects/{project}/locations/{location}/instances/{instance}. |
| `event_publish_config` | String |  | Optional. Option to enable and pass metadata for event publishing. |
| `dataproc_service_account` | String |  | Optional. User-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines. This allows users to have fine-grained access control on Dataproc's accesses to cloud resources. |
| `type` | String |  | Required. Instance type. |
| `zone` | String |  | Optional. Name of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field. |
| `service_endpoint` | String |  | Output only. Endpoint on which the Data Fusion UI is accessible. |
| `network_config` | String |  | Optional. Network configuration options. These are required when a private Data Fusion instance is to be created. |
| `maintenance_events` | Vec<String> |  | Output only. The maintenance events for this instance. |
| `crypto_key_config` | String |  | Optional. The crypto key configuration. This field is used by the Customer-Managed Encryption Keys (CMEK) feature. |
| `api_endpoint` | String |  | Output only. Endpoint on which the REST APIs is accessible. |
| `parent` | String | ✅ | Required. The instance's project and location in the format projects/{project}/locations/{location}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tenant_project_id` | String | Output only. The name of the tenant project. |
| `workforce_identity_service_endpoint` | String | Output only. Endpoint on which the Data Fusion UI is accessible to third-party users. |
| `version` | String | Optional. Current version of Data Fusion. |
| `state` | String | Output only. The current state of this Data Fusion instance. |
| `enable_stackdriver_monitoring` | bool | Optional. Option to enable Stackdriver Monitoring. |
| `description` | String | Optional. A description of this instance. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" |
| `patch_revision` | String | Optional. Current patch revision of the Data Fusion. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `service_account` | String | Output only. Deprecated. Use tenant_project_id instead to extract the tenant project ID. |
| `display_name` | String | Optional. Display name for an instance. |
| `options` | HashMap<String, String> | Optional. Map of additional options used to configure the behavior of Data Fusion instance. |
| `accelerators` | Vec<String> | Output only. List of accelerators enabled for this CDF instance. |
| `create_time` | String | Output only. The time the instance was created. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `gcs_bucket` | String | Output only. Cloud Storage bucket generated by Data Fusion in the customer project. |
| `dataplex_data_lineage_integration_enabled` | bool | Optional. Option to enable the Dataplex Lineage Integration feature. |
| `labels` | HashMap<String, String> | The resource labels for instance to use to annotate any related underlying resources such as Compute Engine VMs. The character '=' is not allowed to be used within the labels. |
| `p4_service_account` | String | Output only. Service agent for the customer project. |
| `maintenance_policy` | String | Optional. Configure the maintenance policy for this instance. |
| `disabled_reason` | Vec<String> | Output only. If the instance state is DISABLED, the reason for disabling the instance. |
| `logging_config` | String | Optional. The logging configuration for this instance. This field is supported only in CDF versions 6.11.0 and above. |
| `enable_rbac` | bool | Optional. Option to enable granular role-based access control. |
| `state_message` | String | Output only. Additional information about the current state of this Data Fusion instance if available. |
| `available_version` | Vec<String> | Output only. Available versions that the instance can be upgraded to using UpdateInstanceRequest. |
| `enable_zone_separation` | bool | Output only. Option to enable zone separation. |
| `private_instance` | bool | Optional. Specifies whether the Data Fusion instance should be private. If set to true, all Data Fusion nodes will have private IP addresses and will not be able to access the public internet. |
| `update_time` | String | Output only. The time the instance was last updated. |
| `enable_stackdriver_logging` | bool | Optional. Option to enable Dataproc Stackdriver Logging. |
| `name` | String | Output only. The name of this instance is in the form of projects/{project}/locations/{location}/instances/{instance}. |
| `event_publish_config` | String | Optional. Option to enable and pass metadata for event publishing. |
| `dataproc_service_account` | String | Optional. User-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines. This allows users to have fine-grained access control on Dataproc's accesses to cloud resources. |
| `type` | String | Required. Instance type. |
| `zone` | String | Optional. Name of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field. |
| `service_endpoint` | String | Output only. Endpoint on which the Data Fusion UI is accessible. |
| `network_config` | String | Optional. Network configuration options. These are required when a private Data Fusion instance is to be created. |
| `maintenance_events` | Vec<String> | Output only. The maintenance events for this instance. |
| `crypto_key_config` | String | Optional. The crypto key configuration. This field is used by the Customer-Managed Encryption Keys (CMEK) feature. |
| `api_endpoint` | String | Output only. Endpoint on which the REST APIs is accessible. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instance
instance = provider.datafusion_api.Instance {
    parent = "value"  # Required. The instance's project and location in the format projects/{project}/locations/{location}.
}

# Access instance outputs
instance_id = instance.id
instance_tenant_project_id = instance.tenant_project_id
instance_workforce_identity_service_endpoint = instance.workforce_identity_service_endpoint
instance_version = instance.version
instance_state = instance.state
instance_enable_stackdriver_monitoring = instance.enable_stackdriver_monitoring
instance_description = instance.description
instance_tags = instance.tags
instance_patch_revision = instance.patch_revision
instance_satisfies_pzi = instance.satisfies_pzi
instance_service_account = instance.service_account
instance_display_name = instance.display_name
instance_options = instance.options
instance_accelerators = instance.accelerators
instance_create_time = instance.create_time
instance_satisfies_pzs = instance.satisfies_pzs
instance_gcs_bucket = instance.gcs_bucket
instance_dataplex_data_lineage_integration_enabled = instance.dataplex_data_lineage_integration_enabled
instance_labels = instance.labels
instance_p4_service_account = instance.p4_service_account
instance_maintenance_policy = instance.maintenance_policy
instance_disabled_reason = instance.disabled_reason
instance_logging_config = instance.logging_config
instance_enable_rbac = instance.enable_rbac
instance_state_message = instance.state_message
instance_available_version = instance.available_version
instance_enable_zone_separation = instance.enable_zone_separation
instance_private_instance = instance.private_instance
instance_update_time = instance.update_time
instance_enable_stackdriver_logging = instance.enable_stackdriver_logging
instance_name = instance.name
instance_event_publish_config = instance.event_publish_config
instance_dataproc_service_account = instance.dataproc_service_account
instance_type = instance.type
instance_zone = instance.zone
instance_service_endpoint = instance.service_endpoint
instance_network_config = instance.network_config
instance_maintenance_events = instance.maintenance_events
instance_crypto_key_config = instance.crypto_key_config
instance_api_endpoint = instance.api_endpoint
```

---


### Version

Lists possible versions for Data Fusion instances in the specified project and location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to retrieve the next page of results or empty if there are no more results in the list. |
| `available_versions` | Vec<String> | Represents a list of versions that are supported. Deprecated: Use versions field instead. |
| `versions` | Vec<String> | Represents a list of all versions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access version outputs
version_id = version.id
version_next_page_token = version.next_page_token
version_available_versions = version.available_versions
version_versions = version.versions
```

---


### Namespace

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create namespace
namespace = provider.datafusion_api.Namespace {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access namespace outputs
namespace_id = namespace.id
namespace_audit_configs = namespace.audit_configs
namespace_bindings = namespace.bindings
namespace_etag = namespace.etag
namespace_version = namespace.version
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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

# Create operation
operation = provider.datafusion_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Dns_peering

Creates DNS peering on the given resource.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the dns peering zone. Format: projects/{project}/locations/{location}/instances/{instance}/dnsPeerings/{dns_peering} |
| `domain` | String |  | Required. The dns name suffix of the zone. |
| `description` | String |  | Optional. Optional description of the dns zone. |
| `target_network` | String |  | Optional. Optional target network to which dns peering should happen. |
| `target_project` | String |  | Optional. Optional target project to which dns peering should happen. |
| `parent` | String | ✅ | Required. The resource on which DNS peering will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dns_peerings` | Vec<String> | List of dns peering. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dns_peering
dns_peering = provider.datafusion_api.Dns_peering {
    parent = "value"  # Required. The resource on which DNS peering will be created.
}

# Access dns_peering outputs
dns_peering_id = dns_peering.id
dns_peering_dns_peerings = dns_peering.dns_peerings
dns_peering_next_page_token = dns_peering.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple dns_peering resources
dns_peering_0 = provider.datafusion_api.Dns_peering {
    parent = "value-0"
}
dns_peering_1 = provider.datafusion_api.Dns_peering {
    parent = "value-1"
}
dns_peering_2 = provider.datafusion_api.Dns_peering {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    dns_peering = provider.datafusion_api.Dns_peering {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Datafusion_api Documentation](https://cloud.google.com/datafusion_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
