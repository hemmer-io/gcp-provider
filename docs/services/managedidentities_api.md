# Managedidentities_api Service



**Resources**: 18

---

## Overview

The managedidentities_api service provides access to 18 resource types:

- [Backup](#backup) [CRUD]
- [Domain](#domain) [CRUD]
- [Operation](#operation) [CRD]
- [Sql_integration](#sql_integration) [R]
- [Location](#location) [R]
- [Peering](#peering) [CRUD]
- [Sql_integration](#sql_integration) [R]
- [Domain](#domain) [CRUD]
- [Operation](#operation) [CRD]
- [Backup](#backup) [CRUD]
- [Location](#location) [R]
- [Peering](#peering) [CRUD]
- [Domain](#domain) [CRUD]
- [Sql_integration](#sql_integration) [R]
- [Backup](#backup) [CRUD]
- [Peering](#peering) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]

---

## Resources


### Backup

Creates a Backup for a domain.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The unique name of the Backup in the form of `projects/{project_id}/locations/global/domains/{domain_name}/backups/{name}` |
| `status_message` | String |  | Output only. Additional information about the current status of this backup, if available. |
| `state` | String |  | Output only. The current state of the backup. |
| `create_time` | String |  | Output only. The time the backups was created. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `type` | String |  | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `update_time` | String |  | Output only. Last update time. |
| `parent` | String | ✅ | Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The unique name of the Backup in the form of `projects/{project_id}/locations/global/domains/{domain_name}/backups/{name}` |
| `status_message` | String | Output only. Additional information about the current status of this backup, if available. |
| `state` | String | Output only. The current state of the backup. |
| `create_time` | String | Output only. The time the backups was created. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `type` | String | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `update_time` | String | Output only. Last update time. |


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
backup_name = backup.name
backup_status_message = backup.status_message
backup_state = backup.state
backup_create_time = backup.create_time
backup_labels = backup.labels
backup_type = backup.type
backup_update_time = backup.update_time
```

---


### Domain

Creates a Microsoft AD domain.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `trusts` | Vec<String> |  | Output only. The current trusts associated with the domain. |
| `update_time` | String |  | Output only. The last update time. |
| `fqdn` | String |  | Output only. The fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory set up on an internal network. |
| `reserved_ip_range` | String |  | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |
| `authorized_networks` | Vec<String> |  | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) the domain instance is connected to. Networks can be added using UpdateDomain. The domain is only available on networks listed in `authorized_networks`. If CIDR subnets overlap between networks, domain creation will fail. |
| `admin` | String |  | Optional. The name of delegated administrator account used to perform Active Directory operations. If not specified, `setupadmin` will be used. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `audit_logs_enabled` | bool |  | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels that can contain user-provided metadata. |
| `locations` | Vec<String> |  | Required. Locations where domain needs to be provisioned. The locations can be specified according to https://cloud.google.com/compute/docs/regions-zones, such as `us-west1` or `us-east4`. Each domain supports up to 4 locations, separated by commas. Each location will use a /26 block. |
| `name` | String |  | Required. The unique name of the domain using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `status_message` | String |  | Output only. Additional information about the current status of this domain, if available. |
| `state` | String |  | Output only. The current state of this domain. |
| `parent` | String | ✅ | Required. The resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `trusts` | Vec<String> | Output only. The current trusts associated with the domain. |
| `update_time` | String | Output only. The last update time. |
| `fqdn` | String | Output only. The fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory set up on an internal network. |
| `reserved_ip_range` | String | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |
| `authorized_networks` | Vec<String> | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) the domain instance is connected to. Networks can be added using UpdateDomain. The domain is only available on networks listed in `authorized_networks`. If CIDR subnets overlap between networks, domain creation will fail. |
| `admin` | String | Optional. The name of delegated administrator account used to perform Active Directory operations. If not specified, `setupadmin` will be used. |
| `create_time` | String | Output only. The time the instance was created. |
| `audit_logs_enabled` | bool | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `labels` | HashMap<String, String> | Optional. Resource labels that can contain user-provided metadata. |
| `locations` | Vec<String> | Required. Locations where domain needs to be provisioned. The locations can be specified according to https://cloud.google.com/compute/docs/regions-zones, such as `us-west1` or `us-east4`. Each domain supports up to 4 locations, separated by commas. Each location will use a /26 block. |
| `name` | String | Required. The unique name of the domain using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `status_message` | String | Output only. Additional information about the current status of this domain, if available. |
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
domain_trusts = domain.trusts
domain_update_time = domain.update_time
domain_fqdn = domain.fqdn
domain_reserved_ip_range = domain.reserved_ip_range
domain_authorized_networks = domain.authorized_networks
domain_admin = domain.admin
domain_create_time = domain.create_time
domain_audit_logs_enabled = domain.audit_logs_enabled
domain_labels = domain.labels
domain_locations = domain.locations
domain_name = domain.name
domain_status_message = domain.status_message
domain_state = domain.state
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.managedidentities_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
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
| `create_time` | String | Output only. The time the SQL integration was created. |
| `update_time` | String | Output only. The time the SQL integration was updated. |
| `name` | String | The unique name of the SQL integration in the form of `projects/{project_id}/locations/global/domains/{domain_name}/sqlIntegrations/{sql_integration}` |


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
sql_integration_create_time = sql_integration.create_time
sql_integration_update_time = sql_integration.update_time
sql_integration_name = sql_integration.name
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


### Peering

Creates a Peering for Managed AD instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time the instance was created. |
| `update_time` | String |  | Output only. Last update time. |
| `state` | String |  | Output only. The current state of this Peering. |
| `authorized_network` | String |  | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. |
| `name` | String |  | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |
| `status_message` | String |  | Output only. Additional information about the current status of this peering, if available. |
| `domain_resource` | String |  | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |
| `parent` | String | ✅ | Required. Resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time the instance was created. |
| `update_time` | String | Output only. Last update time. |
| `state` | String | Output only. The current state of this Peering. |
| `authorized_network` | String | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. |
| `name` | String | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |
| `status_message` | String | Output only. Additional information about the current status of this peering, if available. |
| `domain_resource` | String | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |


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
peering_create_time = peering.create_time
peering_update_time = peering.update_time
peering_state = peering.state
peering_authorized_network = peering.authorized_network
peering_labels = peering.labels
peering_name = peering.name
peering_status_message = peering.status_message
peering_domain_resource = peering.domain_resource
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
| `sql_instance` | String | The full resource name of an integrated sql instance |
| `name` | String | The unique name of the sql integration in the form of `projects/{project_id}/locations/global/domains/{domain_name}/sqlIntegrations/{sql_integration}` |
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
sql_integration_sql_instance = sql_integration.sql_instance
sql_integration_name = sql_integration.name
sql_integration_create_time = sql_integration.create_time
sql_integration_state = sql_integration.state
```

---


### Domain

Creates a Microsoft AD domain.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `admin` | String |  | Optional. The name of delegated administrator account used to perform Active Directory operations. If not specified, `setupadmin` will be used. |
| `locations` | Vec<String> |  | Required. Locations where domain needs to be provisioned. regions e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block. |
| `state` | String |  | Output only. The current state of this domain. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels that can contain user-provided metadata. |
| `name` | String |  | Output only. The unique name of the domain using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `authorized_networks` | Vec<String> |  | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) the domain instance is connected to. Networks can be added using UpdateDomain. The domain is only available on networks listed in `authorized_networks`. If CIDR subnets overlap between networks, domain creation will fail. |
| `fqdn` | String |  | Output only. The fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory set up on an internal network. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `status_message` | String |  | Output only. Additional information about the current status of this domain, if available. |
| `trusts` | Vec<String> |  | Output only. The current trusts associated with the domain. |
| `update_time` | String |  | Output only. The last update time. |
| `audit_logs_enabled` | bool |  | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `reserved_ip_range` | String |  | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |
| `parent` | String | ✅ | Required. The resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `admin` | String | Optional. The name of delegated administrator account used to perform Active Directory operations. If not specified, `setupadmin` will be used. |
| `locations` | Vec<String> | Required. Locations where domain needs to be provisioned. regions e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block. |
| `state` | String | Output only. The current state of this domain. |
| `labels` | HashMap<String, String> | Optional. Resource labels that can contain user-provided metadata. |
| `name` | String | Output only. The unique name of the domain using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `authorized_networks` | Vec<String> | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) the domain instance is connected to. Networks can be added using UpdateDomain. The domain is only available on networks listed in `authorized_networks`. If CIDR subnets overlap between networks, domain creation will fail. |
| `fqdn` | String | Output only. The fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory set up on an internal network. |
| `create_time` | String | Output only. The time the instance was created. |
| `status_message` | String | Output only. Additional information about the current status of this domain, if available. |
| `trusts` | Vec<String> | Output only. The current trusts associated with the domain. |
| `update_time` | String | Output only. The last update time. |
| `audit_logs_enabled` | bool | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `reserved_ip_range` | String | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |


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
domain_admin = domain.admin
domain_locations = domain.locations
domain_state = domain.state
domain_labels = domain.labels
domain_name = domain.name
domain_authorized_networks = domain.authorized_networks
domain_fqdn = domain.fqdn
domain_create_time = domain.create_time
domain_status_message = domain.status_message
domain_trusts = domain.trusts
domain_update_time = domain.update_time
domain_audit_logs_enabled = domain.audit_logs_enabled
domain_reserved_ip_range = domain.reserved_ip_range
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.managedidentities_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Backup

Creates a Backup for a domain.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status_message` | String |  | Output only. Additional information about the current status of this backup, if available. |
| `name` | String |  | Output only. The unique name of the Backup in the form of projects/{project_id}/locations/global/domains/{domain_name}/backups/{name} |
| `type` | String |  | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `update_time` | String |  | Output only. Last update time. |
| `create_time` | String |  | Output only. The time the backups was created. |
| `description` | String |  | Optional. A short description of the backup. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `state` | String |  | Output only. The current state of the backup. |
| `parent` | String | ✅ | Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status_message` | String | Output only. Additional information about the current status of this backup, if available. |
| `name` | String | Output only. The unique name of the Backup in the form of projects/{project_id}/locations/global/domains/{domain_name}/backups/{name} |
| `type` | String | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `update_time` | String | Output only. Last update time. |
| `create_time` | String | Output only. The time the backups was created. |
| `description` | String | Optional. A short description of the backup. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `state` | String | Output only. The current state of the backup. |


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
backup_status_message = backup.status_message
backup_name = backup.name
backup_type = backup.type
backup_update_time = backup.update_time
backup_create_time = backup.create_time
backup_description = backup.description
backup_labels = backup.labels
backup_state = backup.state
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


### Peering

Creates a Peering for Managed AD instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Last update time. |
| `state` | String |  | Output only. The current state of this Peering. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `authorized_network` | String |  | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `name` | String |  | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |
| `status_message` | String |  | Output only. Additional information about the current status of this peering, if available. |
| `domain_resource` | String |  | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |
| `parent` | String | ✅ | Required. Resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Last update time. |
| `state` | String | Output only. The current state of this Peering. |
| `create_time` | String | Output only. The time the instance was created. |
| `authorized_network` | String | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `name` | String | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |
| `status_message` | String | Output only. Additional information about the current status of this peering, if available. |
| `domain_resource` | String | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |


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
peering_update_time = peering.update_time
peering_state = peering.state
peering_create_time = peering.create_time
peering_authorized_network = peering.authorized_network
peering_labels = peering.labels
peering_name = peering.name
peering_status_message = peering.status_message
peering_domain_resource = peering.domain_resource
```

---


### Domain

Creates a Microsoft AD Domain in a given project. Operation

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `managed_identities_admin_name` | String |  | Optional. Name of customer-visible admin used to perform Active Directory operations. If not specified `setupadmin` would be used. |
| `name` | String |  | Output only. Unique name of the domain in this scope including projects and location using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `trusts` | Vec<String> |  | Output only. The current trusts associated with the domain. |
| `update_time` | String |  | Output only. Last update time. Synthetic field is populated automatically by CCFE. |
| `audit_logs_enabled` | bool |  | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `authorized_networks` | Vec<String> |  | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Network can be added using UpdateDomain later. Domain is only available on network part of authorized_networks. Caller needs to make sure that CIDR subnets do not overlap between networks, else domain creation will fail. |
| `create_time` | String |  | Output only. The time the instance was created. Synthetic field is populated automatically by CCFE. go/ccfe-synthetic-field-user-guide |
| `locations` | Vec<String> |  | Required. Locations where domain needs to be provisioned. regions e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block. |
| `reserved_ip_range` | String |  | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |
| `status_message` | String |  | Output only. Additional information about the current status of this domain, if available. |
| `state` | String |  | Output only. The current state of this domain. |
| `fqdn` | String |  | Output only. Fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory that is set up on an internal network. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata |
| `parent` | String | ✅ | Resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_identities_admin_name` | String | Optional. Name of customer-visible admin used to perform Active Directory operations. If not specified `setupadmin` would be used. |
| `name` | String | Output only. Unique name of the domain in this scope including projects and location using the form: `projects/{project_id}/locations/global/domains/{domain_name}`. |
| `trusts` | Vec<String> | Output only. The current trusts associated with the domain. |
| `update_time` | String | Output only. Last update time. Synthetic field is populated automatically by CCFE. |
| `audit_logs_enabled` | bool | Optional. Configuration for audit logs. True if audit logs are enabled, else false. Default is audit logs disabled. |
| `authorized_networks` | Vec<String> | Optional. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Network can be added using UpdateDomain later. Domain is only available on network part of authorized_networks. Caller needs to make sure that CIDR subnets do not overlap between networks, else domain creation will fail. |
| `create_time` | String | Output only. The time the instance was created. Synthetic field is populated automatically by CCFE. go/ccfe-synthetic-field-user-guide |
| `locations` | Vec<String> | Required. Locations where domain needs to be provisioned. regions e.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block. |
| `reserved_ip_range` | String | Required. The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger. Ranges must be unique and non-overlapping with existing subnets in [Domain].[authorized_networks]. |
| `status_message` | String | Output only. Additional information about the current status of this domain, if available. |
| `state` | String | Output only. The current state of this domain. |
| `fqdn` | String | Output only. Fully-qualified domain name of the exposed domain used by clients to connect to the service. Similar to what would be chosen for an Active Directory that is set up on an internal network. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata |


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
domain_managed_identities_admin_name = domain.managed_identities_admin_name
domain_name = domain.name
domain_trusts = domain.trusts
domain_update_time = domain.update_time
domain_audit_logs_enabled = domain.audit_logs_enabled
domain_authorized_networks = domain.authorized_networks
domain_create_time = domain.create_time
domain_locations = domain.locations
domain_reserved_ip_range = domain.reserved_ip_range
domain_status_message = domain.status_message
domain_state = domain.state
domain_fqdn = domain.fqdn
domain_labels = domain.labels
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
| `name` | String | The unique name of the sql integration in the form of `projects/{project_id}/locations/global/domains/{domain_name}/sqlIntegrations/{sql_integration}` |
| `state` | String | Output only. The current state of the managed OU. |
| `update_time` | String | Output only. Last update time for this SQL instance. |
| `sql_instance` | String | The full resource name of an integrated sql instance |
| `create_time` | String | Output only. The time the instance was created. |


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
sql_integration_name = sql_integration.name
sql_integration_state = sql_integration.state
sql_integration_update_time = sql_integration.update_time
sql_integration_sql_instance = sql_integration.sql_instance
sql_integration_create_time = sql_integration.create_time
```

---


### Backup

Creates a Backup for a domain.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `description` | String |  | Optional. A short description of the backup. |
| `type` | String |  | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `name` | String |  | Output only. The unique name of the Backup in the form of projects/{project_id}/locations/global/domains/{domain_name}/backups/{name} |
| `create_time` | String |  | Output only. The time the backups was created. |
| `status_message` | String |  | Output only. Additional information about the current status of this backup, if available. |
| `state` | String |  | Output only. The current state of the backup. |
| `update_time` | String |  | Output only. Last update time. |
| `parent` | String | ✅ | Required. The domain resource name using the form: `projects/{project_id}/locations/global/domains/{domain_name}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `description` | String | Optional. A short description of the backup. |
| `type` | String | Output only. Indicates whether it’s an on-demand backup or scheduled. |
| `name` | String | Output only. The unique name of the Backup in the form of projects/{project_id}/locations/global/domains/{domain_name}/backups/{name} |
| `create_time` | String | Output only. The time the backups was created. |
| `status_message` | String | Output only. Additional information about the current status of this backup, if available. |
| `state` | String | Output only. The current state of the backup. |
| `update_time` | String | Output only. Last update time. |


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
backup_description = backup.description
backup_type = backup.type
backup_name = backup.name
backup_create_time = backup.create_time
backup_status_message = backup.status_message
backup_state = backup.state
backup_update_time = backup.update_time
```

---


### Peering

Creates a Peering for Managed AD instance.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authorized_network` | String |  | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `create_time` | String |  | Output only. The time the instance was created. |
| `name` | String |  | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |
| `status_message` | String |  | Output only. Additional information about the current status of this peering, if available. |
| `update_time` | String |  | Output only. Last update time. |
| `state` | String |  | Output only. The current state of this Peering. |
| `domain_resource` | String |  | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `parent` | String | ✅ | Required. Resource project name and location using the form: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authorized_network` | String | Required. The full names of the Google Compute Engine [networks](/compute/docs/networks-and-firewalls#networks) to which the instance is connected. Caller needs to make sure that CIDR subnets do not overlap between networks, else peering creation will fail. |
| `create_time` | String | Output only. The time the instance was created. |
| `name` | String | Output only. Unique name of the peering in this scope including projects and location using the form: `projects/{project_id}/locations/global/peerings/{peering_id}`. |
| `status_message` | String | Output only. Additional information about the current status of this peering, if available. |
| `update_time` | String | Output only. Last update time. |
| `state` | String | Output only. The current state of this Peering. |
| `domain_resource` | String | Required. Full domain resource path for the Managed AD Domain involved in peering. The resource path should be in the form: `projects/{project_id}/locations/global/domains/{domain_name}` |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |


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
peering_create_time = peering.create_time
peering_name = peering.name
peering_status_message = peering.status_message
peering_update_time = peering.update_time
peering_state = peering.state
peering_domain_resource = peering.domain_resource
peering_labels = peering.labels
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_name = location.name
location_metadata = location.metadata
location_labels = location.labels
location_location_id = location.location_id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple backup resources
backup_0 = provider.managedidentities_api.Backup {
    parent = "value-0"
}
backup_1 = provider.managedidentities_api.Backup {
    parent = "value-1"
}
backup_2 = provider.managedidentities_api.Backup {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    backup = provider.managedidentities_api.Backup {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Managedidentities_api Documentation](https://cloud.google.com/managedidentities_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
