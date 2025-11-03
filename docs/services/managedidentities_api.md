# Managedidentities_api Service



**Resources**: 18

---

## Overview

The managedidentities_api service provides access to 18 resource types:

- [Domain](#domain) [CRUD]
- [Operation](#operation) [CRD]
- [Sql_integration](#sql_integration) [R]
- [Location](#location) [R]
- [Peering](#peering) [CRUD]
- [Backup](#backup) [CRUD]
- [Operation](#operation) [CRD]
- [Peering](#peering) [CRUD]
- [Sql_integration](#sql_integration) [R]
- [Location](#location) [R]
- [Domain](#domain) [CRUD]
- [Backup](#backup) [CRUD]
- [Peering](#peering) [CRUD]
- [Sql_integration](#sql_integration) [R]
- [Operation](#operation) [CRD]
- [Domain](#domain) [CRUD]
- [Location](#location) [R]
- [Backup](#backup) [CRUD]

---

## Resources


### Domain

Creates a Microsoft AD domain.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The unique name of the domain using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `audit_logs_enabled` | bool |  | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `locations` | Vec<String> |  | Required. Locations where domain needs to be provisioned. The locations can be specified according to https://cloud.google.com/compute/docs/regions-zones, such as `us-west1` or `us-east4`. Each domain supports up to 4 locations, separated by commas. Each location will use a /26 block. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels that can contain user-provided metadata. |
| `state` | String |  | Output only. The current state of this domain. |
| `reserved_ip_range` | String |  | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |
| `update_time` | String |  | Output only. The last update time. |
| `fqdn` | String |  | Output only. The fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory set up on an internal network. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `trusts` | Vec<String> |  | Output only. The current trusts associated with the domain. |
| `status_message` | String |  | Output only. Additional information about the current status of this domain, if available. |
| `authorized_networks` | Vec<String> |  | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) the domain instance is connected to. Networks can be added using UpdateDomain. The domain is only available on networks listed in `authorized_networks`. If CIDR subnets overlap between networks, domain creation will fail. |
| `admin` | String |  | Optional. The name of delegated administrator account used to perform Active Directory operations. If not specified, `setupadmin` will be used. |
| `parent` | String | ✅ | Required. The resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The unique name of the domain using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `audit_logs_enabled` | bool | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `locations` | Vec<String> | Required. Locations where domain needs to be provisioned. The locations can be specified according to https://cloud.google.com/compute/docs/regions-zones, such as `us-west1` or `us-east4`. Each domain supports up to 4 locations, separated by commas. Each location will use a /26 block. |
| `labels` | HashMap<String, String> | Optional. Resource labels that can contain user-provided metadata. |
| `state` | String | Output only. The current state of this domain. |
| `reserved_ip_range` | String | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |
| `update_time` | String | Output only. The last update time. |
| `fqdn` | String | Output only. The fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory set up on an internal network. |
| `create_time` | String | Output only. The time the instance was created. |
| `trusts` | Vec<String> | Output only. The current trusts associated with the domain. |
| `status_message` | String | Output only. Additional information about the current status of this domain, if available. |
| `authorized_networks` | Vec<String> | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) the domain instance is connected to. Networks can be added using UpdateDomain. The domain is only available on networks listed in `authorized_networks`. If CIDR subnets overlap between networks, domain creation will fail. |
| `admin` | String | Optional. The name of delegated administrator account used to perform Active Directory operations. If not specified, `setupadmin` will be used. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domain
domain = provider.managedidentities_api.Domain {
    parent = "value"  # Required. The resource project name and location using the form: `projects/{project_id}/locations/global`
}

# Access domain outputs
domain_id = domain.id
domain_name = domain.name
domain_audit_logs_enabled = domain.audit_logs_enabled
domain_locations = domain.locations
domain_labels = domain.labels
domain_state = domain.state
domain_reserved_ip_range = domain.reserved_ip_range
domain_update_time = domain.update_time
domain_fqdn = domain.fqdn
domain_create_time = domain.create_time
domain_trusts = domain.trusts
domain_status_message = domain.status_message
domain_authorized_networks = domain.authorized_networks
domain_admin = domain.admin
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.managedidentities_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_metadata = operation.metadata
operation_name = operation.name
operation_done = operation.done
operation_error = operation.error
```

---


### Sql_integration

Gets details of a single sqlIntegration.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sql_instance` | String | The full resource name of an integrated SQL instance |
| `state` | String | Output only. The current state of the SQL integration. |
| `name` | String | The unique name of the SQL integration in the form of `projects/{project_id}/locations/global/domains/{domain_name}/sqlIntegrations/{sql_integration}` |
| `update_time` | String | Output only. The time the SQL integration was updated. |
| `create_time` | String | Output only. The time the SQL integration was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access sql_integration outputs
sql_integration_id = sql_integration.id
sql_integration_sql_instance = sql_integration.sql_instance
sql_integration_state = sql_integration.state
sql_integration_name = sql_integration.name
sql_integration_update_time = sql_integration.update_time
sql_integration_create_time = sql_integration.create_time
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_labels = location.labels
location_name = location.name
location_metadata = location.metadata
```

---


### Peering

Creates a Peering for Managed AD instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status_message` | String |  | Output only. Additional information about the current status of this peering, if available. |
| `update_time` | String |  | Output only. Last update time. |
| `authorized_network` | String |  | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `state` | String |  | Output only. The current state of this Peering. |
| `domain_resource` | String |  | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |
| `name` | String |  | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |
| `parent` | String | ✅ | Required. Resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_message` | String | Output only. Additional information about the current status of this peering, if available. |
| `update_time` | String | Output only. Last update time. |
| `authorized_network` | String | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. |
| `create_time` | String | Output only. The time the instance was created. |
| `state` | String | Output only. The current state of this Peering. |
| `domain_resource` | String | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |
| `name` | String | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create peering
peering = provider.managedidentities_api.Peering {
    parent = "value"  # Required. Resource project name and location using the form: `projects/{project_id}/locations/global`
}

# Access peering outputs
peering_id = peering.id
peering_status_message = peering.status_message
peering_update_time = peering.update_time
peering_authorized_network = peering.authorized_network
peering_labels = peering.labels
peering_create_time = peering.create_time
peering_state = peering.state
peering_domain_resource = peering.domain_resource
peering_name = peering.name
```

---


### Backup

Creates a Backup for a domain.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `state` | String |  | Output only. The current state of the backup. |
| `create_time` | String |  | Output only. The time the backups was created. |
| `update_time` | String |  | Output only. Last update time. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `name` | String |  | Output only. The unique name of the Backup in the form of `projects/{project_id}/locations/global/domains/{domain_name}/backups/{name}` |
| `status_message` | String |  | Output only. Additional information about the current status of this backup, if available. |
| `parent` | String | ✅ | Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `state` | String | Output only. The current state of the backup. |
| `create_time` | String | Output only. The time the backups was created. |
| `update_time` | String | Output only. Last update time. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `name` | String | Output only. The unique name of the Backup in the form of `projects/{project_id}/locations/global/domains/{domain_name}/backups/{name}` |
| `status_message` | String | Output only. Additional information about the current status of this backup, if available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.managedidentities_api.Backup {
    parent = "value"  # Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
}

# Access backup outputs
backup_id = backup.id
backup_type = backup.type
backup_state = backup.state
backup_create_time = backup.create_time
backup_update_time = backup.update_time
backup_labels = backup.labels
backup_name = backup.name
backup_status_message = backup.status_message
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.managedidentities_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_response = operation.response
operation_done = operation.done
operation_name = operation.name
operation_error = operation.error
```

---


### Peering

Creates a Peering for Managed AD instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authorized_network` | String |  | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `state` | String |  | Output only. The current state of this Peering. |
| `domain_resource` | String |  | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |
| `update_time` | String |  | Output only. Last update time. |
| `status_message` | String |  | Output only. Additional information about the current status of this peering, if available. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `name` | String |  | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `parent` | String | ✅ | Required. Resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authorized_network` | String | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `state` | String | Output only. The current state of this Peering. |
| `domain_resource` | String | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |
| `update_time` | String | Output only. Last update time. |
| `status_message` | String | Output only. Additional information about the current status of this peering, if available. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `name` | String | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |
| `create_time` | String | Output only. The time the instance was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create peering
peering = provider.managedidentities_api.Peering {
    parent = "value"  # Required. Resource project name and location using the form: `projects/{project_id}/locations/global`
}

# Access peering outputs
peering_id = peering.id
peering_authorized_network = peering.authorized_network
peering_state = peering.state
peering_domain_resource = peering.domain_resource
peering_update_time = peering.update_time
peering_status_message = peering.status_message
peering_labels = peering.labels
peering_name = peering.name
peering_create_time = peering.create_time
```

---


### Sql_integration

Gets details of a single sqlIntegration.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time sql integration was updated. |
| `name` | String | The unique name of the sql integration in the form of `projects/{project_id}/locations/global/domains/{domain_name}/sqlIntegrations/{sql_integration}` |
| `sql_instance` | String | The full resource name of an integrated sql instance |
| `create_time` | String | Output only. The time sql integration was created. |
| `state` | String | Output only. The current state of the sql integration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access sql_integration outputs
sql_integration_id = sql_integration.id
sql_integration_update_time = sql_integration.update_time
sql_integration_name = sql_integration.name
sql_integration_sql_instance = sql_integration.sql_instance
sql_integration_create_time = sql_integration.create_time
sql_integration_state = sql_integration.state
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_display_name = location.display_name
location_metadata = location.metadata
location_labels = location.labels
location_name = location.name
location_location_id = location.location_id
```

---


### Domain

Creates a Microsoft AD domain.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authorized_networks` | Vec<String> |  | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) the domain instance is connected to. Networks can be added using UpdateDomain. The domain is only available on networks listed in `authorized_networks`. If CIDR subnets overlap between networks, domain creation will fail. |
| `name` | String |  | Output only. The unique name of the domain using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `reserved_ip_range` | String |  | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |
| `status_message` | String |  | Output only. Additional information about the current status of this domain, if available. |
| `trusts` | Vec<String> |  | Output only. The current trusts associated with the domain. |
| `update_time` | String |  | Output only. The last update time. |
| `fqdn` | String |  | Output only. The fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory set up on an internal network. |
| `admin` | String |  | Optional. The name of delegated administrator account used to perform Active Directory operations. If not specified, `setupadmin` will be used. |
| `locations` | Vec<String> |  | Required. Locations where domain needs to be provisioned. regions e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels that can contain user-provided metadata. |
| `audit_logs_enabled` | bool |  | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `state` | String |  | Output only. The current state of this domain. |
| `parent` | String | ✅ | Required. The resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authorized_networks` | Vec<String> | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) the domain instance is connected to. Networks can be added using UpdateDomain. The domain is only available on networks listed in `authorized_networks`. If CIDR subnets overlap between networks, domain creation will fail. |
| `name` | String | Output only. The unique name of the domain using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `reserved_ip_range` | String | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |
| `status_message` | String | Output only. Additional information about the current status of this domain, if available. |
| `trusts` | Vec<String> | Output only. The current trusts associated with the domain. |
| `update_time` | String | Output only. The last update time. |
| `fqdn` | String | Output only. The fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory set up on an internal network. |
| `admin` | String | Optional. The name of delegated administrator account used to perform Active Directory operations. If not specified, `setupadmin` will be used. |
| `locations` | Vec<String> | Required. Locations where domain needs to be provisioned. regions e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block. |
| `create_time` | String | Output only. The time the instance was created. |
| `labels` | HashMap<String, String> | Optional. Resource labels that can contain user-provided metadata. |
| `audit_logs_enabled` | bool | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `state` | String | Output only. The current state of this domain. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domain
domain = provider.managedidentities_api.Domain {
    parent = "value"  # Required. The resource project name and location using the form: `projects/{project_id}/locations/global`
}

# Access domain outputs
domain_id = domain.id
domain_authorized_networks = domain.authorized_networks
domain_name = domain.name
domain_reserved_ip_range = domain.reserved_ip_range
domain_status_message = domain.status_message
domain_trusts = domain.trusts
domain_update_time = domain.update_time
domain_fqdn = domain.fqdn
domain_admin = domain.admin
domain_locations = domain.locations
domain_create_time = domain.create_time
domain_labels = domain.labels
domain_audit_logs_enabled = domain.audit_logs_enabled
domain_state = domain.state
```

---


### Backup

Creates a Backup for a domain.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `type` | String |  | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `state` | String |  | Output only. The current state of the backup. |
| `name` | String |  | Output only. The unique name of the Backup in the form of projects/{project_id}/locations/global/domains/{domain_name}/backups/{name} |
| `update_time` | String |  | Output only. Last update time. |
| `status_message` | String |  | Output only. Additional information about the current status of this backup, if available. |
| `create_time` | String |  | Output only. The time the backups was created. |
| `description` | String |  | Optional. A short description of the backup. |
| `parent` | String | ✅ | Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `type` | String | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `state` | String | Output only. The current state of the backup. |
| `name` | String | Output only. The unique name of the Backup in the form of projects/{project_id}/locations/global/domains/{domain_name}/backups/{name} |
| `update_time` | String | Output only. Last update time. |
| `status_message` | String | Output only. Additional information about the current status of this backup, if available. |
| `create_time` | String | Output only. The time the backups was created. |
| `description` | String | Optional. A short description of the backup. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.managedidentities_api.Backup {
    parent = "value"  # Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
}

# Access backup outputs
backup_id = backup.id
backup_labels = backup.labels
backup_type = backup.type
backup_state = backup.state
backup_name = backup.name
backup_update_time = backup.update_time
backup_status_message = backup.status_message
backup_create_time = backup.create_time
backup_description = backup.description
```

---


### Peering

Creates a Peering for Managed AD instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |
| `status_message` | String |  | Output only. Additional information about the current status of this peering, if available. |
| `authorized_network` | String |  | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `update_time` | String |  | Output only. Last update time. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `domain_resource` | String |  | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |
| `state` | String |  | Output only. The current state of this Peering. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `parent` | String | ✅ | Required. Resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |
| `status_message` | String | Output only. Additional information about the current status of this peering, if available. |
| `authorized_network` | String | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `update_time` | String | Output only. Last update time. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `domain_resource` | String | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |
| `state` | String | Output only. The current state of this Peering. |
| `create_time` | String | Output only. The time the instance was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create peering
peering = provider.managedidentities_api.Peering {
    parent = "value"  # Required. Resource project name and location using the form: `projects/{project_id}/locations/global`
}

# Access peering outputs
peering_id = peering.id
peering_name = peering.name
peering_status_message = peering.status_message
peering_authorized_network = peering.authorized_network
peering_update_time = peering.update_time
peering_labels = peering.labels
peering_domain_resource = peering.domain_resource
peering_state = peering.state
peering_create_time = peering.create_time
```

---


### Sql_integration

Gets details of a single sqlIntegration.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sql_instance` | String | The full resource name of an integrated sql instance |
| `name` | String | The unique name of the sql integration in the form of `projects/{project_id}/locations/global/domains/{domain_name}/sqlIntegrations/{sql_integration}` |
| `create_time` | String | Output only. The time the instance was created. |
| `update_time` | String | Output only. Last update time for this SQL instance. |
| `state` | String | Output only. The current state of the managed OU. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access sql_integration outputs
sql_integration_id = sql_integration.id
sql_integration_sql_instance = sql_integration.sql_instance
sql_integration_name = sql_integration.name
sql_integration_create_time = sql_integration.create_time
sql_integration_update_time = sql_integration.update_time
sql_integration_state = sql_integration.state
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.managedidentities_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_error = operation.error
```

---


### Domain

Creates a Microsoft AD Domain in a given project. Operation

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The current state of this domain. |
| `authorized_networks` | Vec<String> |  | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Network can be added using UpdateDomain later. Domain is only available on network part of authorized_networks. Caller needs to make sure that CIDR subnets do not overlap between networks, else domain creation will fail. |
| `trusts` | Vec<String> |  | Output only. The current trusts associated with the domain. |
| `status_message` | String |  | Output only. Additional information about the current status of this domain, if available. |
| `locations` | Vec<String> |  | Required. Locations where domain needs to be provisioned. regions e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block. |
| `audit_logs_enabled` | bool |  | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `update_time` | String |  | Output only. Last update time. Synthetic field is populated automatically by CCFE. |
| `reserved_ip_range` | String |  | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |
| `fqdn` | String |  | Output only. Fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory that is set up on an internal network. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata |
| `managed_identities_admin_name` | String |  | Optional. Name of customer-visible admin used to perform Active Directory operations. If not specified `setupadmin` would be used. |
| `name` | String |  | Output only. Unique name of the domain in this scope including projects and location using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `create_time` | String |  | Output only. The time the instance was created. Synthetic field is populated automatically by CCFE. go/ccfe-synthetic-field-user-guide |
| `parent` | String | ✅ | Resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The current state of this domain. |
| `authorized_networks` | Vec<String> | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Network can be added using UpdateDomain later. Domain is only available on network part of authorized_networks. Caller needs to make sure that CIDR subnets do not overlap between networks, else domain creation will fail. |
| `trusts` | Vec<String> | Output only. The current trusts associated with the domain. |
| `status_message` | String | Output only. Additional information about the current status of this domain, if available. |
| `locations` | Vec<String> | Required. Locations where domain needs to be provisioned. regions e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block. |
| `audit_logs_enabled` | bool | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `update_time` | String | Output only. Last update time. Synthetic field is populated automatically by CCFE. |
| `reserved_ip_range` | String | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |
| `fqdn` | String | Output only. Fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory that is set up on an internal network. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata |
| `managed_identities_admin_name` | String | Optional. Name of customer-visible admin used to perform Active Directory operations. If not specified `setupadmin` would be used. |
| `name` | String | Output only. Unique name of the domain in this scope including projects and location using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `create_time` | String | Output only. The time the instance was created. Synthetic field is populated automatically by CCFE. go/ccfe-synthetic-field-user-guide |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domain
domain = provider.managedidentities_api.Domain {
    parent = "value"  # Resource project name and location using the form: `projects/{project_id}/locations/global`
}

# Access domain outputs
domain_id = domain.id
domain_state = domain.state
domain_authorized_networks = domain.authorized_networks
domain_trusts = domain.trusts
domain_status_message = domain.status_message
domain_locations = domain.locations
domain_audit_logs_enabled = domain.audit_logs_enabled
domain_update_time = domain.update_time
domain_reserved_ip_range = domain.reserved_ip_range
domain_fqdn = domain.fqdn
domain_labels = domain.labels
domain_managed_identities_admin_name = domain.managed_identities_admin_name
domain_name = domain.name
domain_create_time = domain.create_time
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


### Backup

Creates a Backup for a domain.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The current state of the backup. |
| `type` | String |  | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `update_time` | String |  | Output only. Last update time. |
| `name` | String |  | Output only. The unique name of the Backup in the form of projects/{project_id}/locations/global/domains/{domain_name}/backups/{name} |
| `status_message` | String |  | Output only. Additional information about the current status of this backup, if available. |
| `create_time` | String |  | Output only. The time the backups was created. |
| `description` | String |  | Optional. A short description of the backup. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `parent` | String | ✅ | Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The current state of the backup. |
| `type` | String | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `update_time` | String | Output only. Last update time. |
| `name` | String | Output only. The unique name of the Backup in the form of projects/{project_id}/locations/global/domains/{domain_name}/backups/{name} |
| `status_message` | String | Output only. Additional information about the current status of this backup, if available. |
| `create_time` | String | Output only. The time the backups was created. |
| `description` | String | Optional. A short description of the backup. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backup
backup = provider.managedidentities_api.Backup {
    parent = "value"  # Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}`
}

# Access backup outputs
backup_id = backup.id
backup_state = backup.state
backup_type = backup.type
backup_update_time = backup.update_time
backup_name = backup.name
backup_status_message = backup.status_message
backup_create_time = backup.create_time
backup_description = backup.description
backup_labels = backup.labels
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple domain resources
domain_0 = provider.managedidentities_api.Domain {
    parent = "value-0"
}
domain_1 = provider.managedidentities_api.Domain {
    parent = "value-1"
}
domain_2 = provider.managedidentities_api.Domain {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    domain = provider.managedidentities_api.Domain {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Managedidentities_api Documentation](https://cloud.google.com/managedidentities_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
