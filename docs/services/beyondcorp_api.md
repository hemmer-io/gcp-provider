# Beyondcorp_api Service



**Resources**: 20

---

## Overview

The beyondcorp_api service provides access to 20 resource types:

- [Security_gateway](#security_gateway) [CRUD]
- [App_connector](#app_connector) [CRUD]
- [App_gateway](#app_gateway) [CRD]
- [Application](#application) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CRD]
- [App_connection](#app_connection) [CRUD]
- [Connector](#connector) [CRUD]
- [Insight](#insight) [R]
- [Partner_tenant](#partner_tenant) [RD]
- [Subscription](#subscription) [CRU]
- [App_gateway](#app_gateway) [CRD]
- [App_connector](#app_connector) [CRUD]
- [Connection](#connection) [CRUD]
- [Location](#location) [R]
- [Application_domain](#application_domain) [CR]
- [Security_gateway](#security_gateway) [CRUD]
- [Application](#application) [CRUD]
- [Operation](#operation) [CRD]
- [App_connection](#app_connection) [CRUD]

---

## Resources


### Security_gateway

Creates a new Security Gateway in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The operational state of the SecurityGateway. |
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `proxy_protocol_config` | String |  | Optional. Shared proxy configuration for all apps. |
| `external_ips` | Vec<String> |  | Output only. IP addresses that will be used for establishing connection to the endpoints. |
| `service_discovery` | String |  | Optional. Settings related to the Service Discovery. |
| `hubs` | HashMap<String, String> |  | Optional. Map of Hubs that represents regional data path deployment with GCP region as a key. |
| `display_name` | String |  | Optional. An arbitrary user-provided name for the SecurityGateway. Cannot exceed 64 characters. |
| `name` | String |  | Identifier. Name of the resource. |
| `delegating_service_account` | String |  | Output only. Service account used for operations that involve resources in consumer projects. |
| `parent` | String | ✅ | Required. The resource project name of the SecurityGateway location using the form: `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The operational state of the SecurityGateway. |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `proxy_protocol_config` | String | Optional. Shared proxy configuration for all apps. |
| `external_ips` | Vec<String> | Output only. IP addresses that will be used for establishing connection to the endpoints. |
| `service_discovery` | String | Optional. Settings related to the Service Discovery. |
| `hubs` | HashMap<String, String> | Optional. Map of Hubs that represents regional data path deployment with GCP region as a key. |
| `display_name` | String | Optional. An arbitrary user-provided name for the SecurityGateway. Cannot exceed 64 characters. |
| `name` | String | Identifier. Name of the resource. |
| `delegating_service_account` | String | Output only. Service account used for operations that involve resources in consumer projects. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_gateway
security_gateway = provider.beyondcorp_api.Security_gateway {
    parent = "value"  # Required. The resource project name of the SecurityGateway location using the form: `projects/{project_id}/locations/{location_id}`
}

# Access security_gateway outputs
security_gateway_id = security_gateway.id
security_gateway_state = security_gateway.state
security_gateway_update_time = security_gateway.update_time
security_gateway_create_time = security_gateway.create_time
security_gateway_proxy_protocol_config = security_gateway.proxy_protocol_config
security_gateway_external_ips = security_gateway.external_ips
security_gateway_service_discovery = security_gateway.service_discovery
security_gateway_hubs = security_gateway.hubs
security_gateway_display_name = security_gateway.display_name
security_gateway_name = security_gateway.name
security_gateway_delegating_service_account = security_gateway.delegating_service_account
```

---


### App_connector

Creates a new AppConnector in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `principal_info` | String |  | Required. Principal information about the Identity of the AppConnector. |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `state` | String |  | Output only. The current state of the AppConnector. |
| `display_name` | String |  | Optional. An arbitrary user-provided name for the AppConnector. Cannot exceed 64 characters. |
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `name` | String |  | Required. Unique resource name of the AppConnector. The name is ignored when creating a AppConnector. |
| `resource_info` | String |  | Optional. Resource info of the connector. |
| `uid` | String |  | Output only. A unique identifier for the instance generated by the system. |
| `parent` | String | ✅ | Required. The resource project name of the AppConnector location using the form: `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `principal_info` | String | Required. Principal information about the Identity of the AppConnector. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `state` | String | Output only. The current state of the AppConnector. |
| `display_name` | String | Optional. An arbitrary user-provided name for the AppConnector. Cannot exceed 64 characters. |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |
| `name` | String | Required. Unique resource name of the AppConnector. The name is ignored when creating a AppConnector. |
| `resource_info` | String | Optional. Resource info of the connector. |
| `uid` | String | Output only. A unique identifier for the instance generated by the system. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app_connector
app_connector = provider.beyondcorp_api.App_connector {
    parent = "value"  # Required. The resource project name of the AppConnector location using the form: `projects/{project_id}/locations/{location_id}`
}

# Access app_connector outputs
app_connector_id = app_connector.id
app_connector_principal_info = app_connector.principal_info
app_connector_create_time = app_connector.create_time
app_connector_labels = app_connector.labels
app_connector_state = app_connector.state
app_connector_display_name = app_connector.display_name
app_connector_update_time = app_connector.update_time
app_connector_name = app_connector.name
app_connector_resource_info = app_connector.resource_info
app_connector_uid = app_connector.uid
```

---


### App_gateway

Creates a new AppGateway in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `uid` | String |  | Output only. A unique identifier for the instance generated by the system. |
| `allocated_connections` | Vec<String> |  | Output only. A list of connections allocated for the Gateway |
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `name` | String |  | Required. Unique resource name of the AppGateway. The name is ignored when creating an AppGateway. |
| `display_name` | String |  | Optional. An arbitrary user-provided name for the AppGateway. Cannot exceed 64 characters. |
| `uri` | String |  | Output only. Server-defined URI for this resource. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `host_type` | String |  | Required. The type of hosting used by the AppGateway. |
| `state` | String |  | Output only. The current state of the AppGateway. |
| `type` | String |  | Required. The type of network connectivity used by the AppGateway. |
| `parent` | String | ✅ | Required. The resource project name of the AppGateway location using the form: `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `uid` | String | Output only. A unique identifier for the instance generated by the system. |
| `allocated_connections` | Vec<String> | Output only. A list of connections allocated for the Gateway |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `name` | String | Required. Unique resource name of the AppGateway. The name is ignored when creating an AppGateway. |
| `display_name` | String | Optional. An arbitrary user-provided name for the AppGateway. Cannot exceed 64 characters. |
| `uri` | String | Output only. Server-defined URI for this resource. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `host_type` | String | Required. The type of hosting used by the AppGateway. |
| `state` | String | Output only. The current state of the AppGateway. |
| `type` | String | Required. The type of network connectivity used by the AppGateway. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app_gateway
app_gateway = provider.beyondcorp_api.App_gateway {
    parent = "value"  # Required. The resource project name of the AppGateway location using the form: `projects/{project_id}/locations/{location_id}`
}

# Access app_gateway outputs
app_gateway_id = app_gateway.id
app_gateway_labels = app_gateway.labels
app_gateway_uid = app_gateway.uid
app_gateway_allocated_connections = app_gateway.allocated_connections
app_gateway_update_time = app_gateway.update_time
app_gateway_satisfies_pzi = app_gateway.satisfies_pzi
app_gateway_name = app_gateway.name
app_gateway_display_name = app_gateway.display_name
app_gateway_uri = app_gateway.uri
app_gateway_satisfies_pzs = app_gateway.satisfies_pzs
app_gateway_create_time = app_gateway.create_time
app_gateway_host_type = app_gateway.host_type
app_gateway_state = app_gateway.state
app_gateway_type = app_gateway.type
```

---


### Application

Creates a new Application in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema` | String |  | Optional. Type of the external application. |
| `upstreams` | Vec<String> |  | Optional. Which upstream resources to forward traffic to. |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `name` | String |  | Identifier. Name of the resource. |
| `display_name` | String |  | Optional. An arbitrary user-provided name for the application resource. Cannot exceed 64 characters. |
| `endpoint_matchers` | Vec<String> |  | Required. Endpoint matchers associated with an application. A combination of hostname and ports as endpoint matchers is used to match the application. Match conditions for OR logic. An array of match conditions to allow for multiple matching criteria. The rule is considered a match if one of the conditions is met. The conditions should be the following combination: (Hostname & Ports) EXAMPLES: Hostname and Ports - ("*.example.com", "443"), ("example.com" and "22"), ("example.com" and "22,33") etc |
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `parent` | String | ✅ | Required. The resource name of the parent SecurityGateway using the form: `projects/{project_id}/locations/global/securityGateways/{security_gateway_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema` | String | Optional. Type of the external application. |
| `upstreams` | Vec<String> | Optional. Which upstream resources to forward traffic to. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `name` | String | Identifier. Name of the resource. |
| `display_name` | String | Optional. An arbitrary user-provided name for the application resource. Cannot exceed 64 characters. |
| `endpoint_matchers` | Vec<String> | Required. Endpoint matchers associated with an application. A combination of hostname and ports as endpoint matchers is used to match the application. Match conditions for OR logic. An array of match conditions to allow for multiple matching criteria. The rule is considered a match if one of the conditions is met. The conditions should be the following combination: (Hostname & Ports) EXAMPLES: Hostname and Ports - ("*.example.com", "443"), ("example.com" and "22"), ("example.com" and "22,33") etc |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create application
application = provider.beyondcorp_api.Application {
    parent = "value"  # Required. The resource name of the parent SecurityGateway using the form: `projects/{project_id}/locations/global/securityGateways/{security_gateway_id}`
}

# Access application outputs
application_id = application.id
application_schema = application.schema
application_upstreams = application.upstreams
application_create_time = application.create_time
application_name = application.name
application_display_name = application.display_name
application_endpoint_matchers = application.endpoint_matchers
application_update_time = application.update_time
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


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
location_labels = location.labels
location_metadata = location.metadata
location_location_id = location.location_id
location_name = location.name
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.beyondcorp_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_response = operation.response
operation_name = operation.name
operation_error = operation.error
operation_done = operation.done
```

---


### App_connection

Creates a new AppConnection in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_endpoint` | String |  | Required. Address of the remote application endpoint for the BeyondCorp AppConnection. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `type` | String |  | Required. The type of network connectivity used by the AppConnection. |
| `gateway` | String |  | Optional. Gateway used by the AppConnection. |
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `name` | String |  | Required. Unique resource name of the AppConnection. The name is ignored when creating a AppConnection. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `uid` | String |  | Output only. A unique identifier for the instance generated by the system. |
| `connectors` | Vec<String> |  | Optional. List of [google.cloud.beyondcorp.v1main.Connector.name] that are authorised to be associated with this AppConnection. |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `display_name` | String |  | Optional. An arbitrary user-provided name for the AppConnection. Cannot exceed 64 characters. |
| `state` | String |  | Output only. The current state of the AppConnection. |
| `parent` | String | ✅ | Required. The resource project name of the AppConnection location using the form: `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_endpoint` | String | Required. Address of the remote application endpoint for the BeyondCorp AppConnection. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `type` | String | Required. The type of network connectivity used by the AppConnection. |
| `gateway` | String | Optional. Gateway used by the AppConnection. |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `name` | String | Required. Unique resource name of the AppConnection. The name is ignored when creating a AppConnection. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `uid` | String | Output only. A unique identifier for the instance generated by the system. |
| `connectors` | Vec<String> | Optional. List of [google.cloud.beyondcorp.v1main.Connector.name] that are authorised to be associated with this AppConnection. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `display_name` | String | Optional. An arbitrary user-provided name for the AppConnection. Cannot exceed 64 characters. |
| `state` | String | Output only. The current state of the AppConnection. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app_connection
app_connection = provider.beyondcorp_api.App_connection {
    parent = "value"  # Required. The resource project name of the AppConnection location using the form: `projects/{project_id}/locations/{location_id}`
}

# Access app_connection outputs
app_connection_id = app_connection.id
app_connection_application_endpoint = app_connection.application_endpoint
app_connection_satisfies_pzs = app_connection.satisfies_pzs
app_connection_type = app_connection.type
app_connection_gateway = app_connection.gateway
app_connection_update_time = app_connection.update_time
app_connection_labels = app_connection.labels
app_connection_name = app_connection.name
app_connection_satisfies_pzi = app_connection.satisfies_pzi
app_connection_uid = app_connection.uid
app_connection_connectors = app_connection.connectors
app_connection_create_time = app_connection.create_time
app_connection_display_name = app_connection.display_name
app_connection_state = app_connection.state
```

---


### Connector

Creates a new Connector in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `principal_info` | String |  | Required. Principal information about the Identity of the connector. |
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `display_name` | String |  | Optional. An arbitrary user-provided name for the connector. Cannot exceed 64 characters. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `name` | String |  | Required. Unique resource name of the connector. The name is ignored when creating a connector. |
| `resource_info` | String |  | Optional. Resource info of the connector. |
| `uid` | String |  | Output only. A unique identifier for the instance generated by the system. |
| `state` | String |  | Output only. The current state of the connector. |
| `parent` | String | ✅ | Required. The resource project name of the connector location using the form: `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `principal_info` | String | Required. Principal information about the Identity of the connector. |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `display_name` | String | Optional. An arbitrary user-provided name for the connector. Cannot exceed 64 characters. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `name` | String | Required. Unique resource name of the connector. The name is ignored when creating a connector. |
| `resource_info` | String | Optional. Resource info of the connector. |
| `uid` | String | Output only. A unique identifier for the instance generated by the system. |
| `state` | String | Output only. The current state of the connector. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connector
connector = provider.beyondcorp_api.Connector {
    parent = "value"  # Required. The resource project name of the connector location using the form: `projects/{project_id}/locations/{location_id}`
}

# Access connector outputs
connector_id = connector.id
connector_principal_info = connector.principal_info
connector_update_time = connector.update_time
connector_create_time = connector.create_time
connector_display_name = connector.display_name
connector_labels = connector.labels
connector_name = connector.name
connector_resource_info = connector.resource_info
connector_uid = connector.uid
connector_state = connector.state
```

---


### Insight

Gets the value for a selected particular insight with default configuration. The default aggregation level is 'DAILY' and no grouping will be applied or default grouping if applicable. The data will be returned for recent 7 days starting the day before. The insight data size will be limited to 50 rows. Use the organization level path for fetching at org level and project level path for fetching the insight value specific to a particular project. Setting the `view` to `BASIC` will only return the metadata for the insight.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `applied_config` | String | Output only. Applied insight config to generate the result data rows. |
| `rows` | Vec<String> | Output only. Result rows returned containing the required value(s). |
| `metadata` | String | Output only. Metadata for the Insight. |
| `name` | String | Output only. The insight resource name. e.g. `organizations/{organization_id}/locations/{location_id}/insights/{insight_id}` OR `projects/{project_id}/locations/{location_id}/insights/{insight_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access insight outputs
insight_id = insight.id
insight_applied_config = insight.applied_config
insight_rows = insight.rows
insight_metadata = insight.metadata
insight_name = insight.name
```

---


### Partner_tenant

Gets details of a single PartnerTenant.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `partner_metadata` | String | Optional. Metadata provided by the Partner associated with PartnerTenant. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |
| `name` | String | Output only. Unique resource name of the PartnerTenant. The name is ignored when creating PartnerTenant. |
| `display_name` | String | Optional. An arbitrary caller-provided name for the PartnerTenant. Cannot exceed 64 characters. |
| `group` | String | Optional. Group information for the users enabled to use the partnerTenant. If the group information is not provided then the partnerTenant will be enabled for all users. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access partner_tenant outputs
partner_tenant_id = partner_tenant.id
partner_tenant_partner_metadata = partner_tenant.partner_metadata
partner_tenant_create_time = partner_tenant.create_time
partner_tenant_update_time = partner_tenant.update_time
partner_tenant_name = partner_tenant.name
partner_tenant_display_name = partner_tenant.display_name
partner_tenant_group = partner_tenant.group
```

---


### Subscription

Creates a new BeyondCorp Enterprise Subscription in a given organization. Location will always be global as BeyondCorp subscriptions are per organization.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `start_time` | String |  | Optional. Start time of the subscription. |
| `name` | String |  | Identifier. Unique resource name of the Subscription. The name is ignored when creating a subscription. |
| `type` | String |  | Required. Type of subscription. |
| `seat_count` | String |  | Optional. Number of seats in the subscription. |
| `end_time` | String |  | Optional. End time of the subscription. |
| `billing_account` | String |  | Optional. Name of the billing account in the format. e.g. billingAccounts/123456-123456-123456 Required if Subscription is of Paid type. |
| `auto_renew_enabled` | bool |  | Output only. Represents that, if subscription will renew or end when the term ends. |
| `create_time` | String |  | Output only. Create time of the subscription. |
| `sku` | String |  | Required. SKU of subscription. |
| `state` | String |  | Output only. The current state of the subscription. |
| `subscriber_type` | String |  | Output only. Type of subscriber. |
| `csg_customer` | bool |  | Optional. Whether the subscription is being created as part of the Citrix flow. If this field is set to true, the subscription should have both the start_time and end_time set in the request and the billing account used will be the Citrix master billing account regardless of what its set to in the request. This field can only be set to true in create requests. |
| `parent` | String | ✅ | Required. The resource name of the subscription location using the form: `organizations/{organization_id}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | Optional. Start time of the subscription. |
| `name` | String | Identifier. Unique resource name of the Subscription. The name is ignored when creating a subscription. |
| `type` | String | Required. Type of subscription. |
| `seat_count` | String | Optional. Number of seats in the subscription. |
| `end_time` | String | Optional. End time of the subscription. |
| `billing_account` | String | Optional. Name of the billing account in the format. e.g. billingAccounts/123456-123456-123456 Required if Subscription is of Paid type. |
| `auto_renew_enabled` | bool | Output only. Represents that, if subscription will renew or end when the term ends. |
| `create_time` | String | Output only. Create time of the subscription. |
| `sku` | String | Required. SKU of subscription. |
| `state` | String | Output only. The current state of the subscription. |
| `subscriber_type` | String | Output only. Type of subscriber. |
| `csg_customer` | bool | Optional. Whether the subscription is being created as part of the Citrix flow. If this field is set to true, the subscription should have both the start_time and end_time set in the request and the billing account used will be the Citrix master billing account regardless of what its set to in the request. This field can only be set to true in create requests. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subscription
subscription = provider.beyondcorp_api.Subscription {
    parent = "value"  # Required. The resource name of the subscription location using the form: `organizations/{organization_id}/locations/{location}`
}

# Access subscription outputs
subscription_id = subscription.id
subscription_start_time = subscription.start_time
subscription_name = subscription.name
subscription_type = subscription.type
subscription_seat_count = subscription.seat_count
subscription_end_time = subscription.end_time
subscription_billing_account = subscription.billing_account
subscription_auto_renew_enabled = subscription.auto_renew_enabled
subscription_create_time = subscription.create_time
subscription_sku = subscription.sku
subscription_state = subscription.state
subscription_subscriber_type = subscription.subscriber_type
subscription_csg_customer = subscription.csg_customer
```

---


### App_gateway

Creates a new AppGateway in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `uid` | String |  | Output only. A unique identifier for the instance generated by the system. |
| `type` | String |  | Required. The type of network connectivity used by the AppGateway. |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `host_type` | String |  | Required. The type of hosting used by the AppGateway. |
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `allocated_connections` | Vec<String> |  | Output only. A list of connections allocated for the Gateway |
| `uri` | String |  | Output only. Server-defined URI for this resource. |
| `state` | String |  | Output only. The current state of the AppGateway. |
| `name` | String |  | Required. Unique resource name of the AppGateway. The name is ignored when creating an AppGateway. |
| `display_name` | String |  | Optional. An arbitrary user-provided name for the AppGateway. Cannot exceed 64 characters. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `parent` | String | ✅ | Required. The resource project name of the AppGateway location using the form: `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `uid` | String | Output only. A unique identifier for the instance generated by the system. |
| `type` | String | Required. The type of network connectivity used by the AppGateway. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `host_type` | String | Required. The type of hosting used by the AppGateway. |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `allocated_connections` | Vec<String> | Output only. A list of connections allocated for the Gateway |
| `uri` | String | Output only. Server-defined URI for this resource. |
| `state` | String | Output only. The current state of the AppGateway. |
| `name` | String | Required. Unique resource name of the AppGateway. The name is ignored when creating an AppGateway. |
| `display_name` | String | Optional. An arbitrary user-provided name for the AppGateway. Cannot exceed 64 characters. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app_gateway
app_gateway = provider.beyondcorp_api.App_gateway {
    parent = "value"  # Required. The resource project name of the AppGateway location using the form: `projects/{project_id}/locations/{location_id}`
}

# Access app_gateway outputs
app_gateway_id = app_gateway.id
app_gateway_satisfies_pzi = app_gateway.satisfies_pzi
app_gateway_uid = app_gateway.uid
app_gateway_type = app_gateway.type
app_gateway_create_time = app_gateway.create_time
app_gateway_host_type = app_gateway.host_type
app_gateway_update_time = app_gateway.update_time
app_gateway_satisfies_pzs = app_gateway.satisfies_pzs
app_gateway_allocated_connections = app_gateway.allocated_connections
app_gateway_uri = app_gateway.uri
app_gateway_state = app_gateway.state
app_gateway_name = app_gateway.name
app_gateway_display_name = app_gateway.display_name
app_gateway_labels = app_gateway.labels
```

---


### App_connector

Creates a new AppConnector in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `state` | String |  | Output only. The current state of the AppConnector. |
| `principal_info` | String |  | Required. Principal information about the Identity of the AppConnector. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `name` | String |  | Required. Unique resource name of the AppConnector. The name is ignored when creating a AppConnector. |
| `display_name` | String |  | Optional. An arbitrary user-provided name for the AppConnector. Cannot exceed 64 characters. |
| `resource_info` | String |  | Optional. Resource info of the connector. |
| `uid` | String |  | Output only. A unique identifier for the instance generated by the system. |
| `parent` | String | ✅ | Required. The resource project name of the AppConnector location using the form: `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Timestamp when the resource was last modified. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `state` | String | Output only. The current state of the AppConnector. |
| `principal_info` | String | Required. Principal information about the Identity of the AppConnector. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `name` | String | Required. Unique resource name of the AppConnector. The name is ignored when creating a AppConnector. |
| `display_name` | String | Optional. An arbitrary user-provided name for the AppConnector. Cannot exceed 64 characters. |
| `resource_info` | String | Optional. Resource info of the connector. |
| `uid` | String | Output only. A unique identifier for the instance generated by the system. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app_connector
app_connector = provider.beyondcorp_api.App_connector {
    parent = "value"  # Required. The resource project name of the AppConnector location using the form: `projects/{project_id}/locations/{location_id}`
}

# Access app_connector outputs
app_connector_id = app_connector.id
app_connector_update_time = app_connector.update_time
app_connector_create_time = app_connector.create_time
app_connector_state = app_connector.state
app_connector_principal_info = app_connector.principal_info
app_connector_labels = app_connector.labels
app_connector_name = app_connector.name
app_connector_display_name = app_connector.display_name
app_connector_resource_info = app_connector.resource_info
app_connector_uid = app_connector.uid
```

---


### Connection

Creates a new Connection in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_endpoint` | String |  | Required. Address of the remote application endpoint for the BeyondCorp Connection. |
| `connectors` | Vec<String> |  | Optional. List of [google.cloud.beyondcorp.v1main.Connector.name] that are authorised to be associated with this Connection. |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `name` | String |  | Required. Unique resource name of the connection. The name is ignored when creating a connection. |
| `display_name` | String |  | Optional. An arbitrary user-provided name for the connection. Cannot exceed 64 characters. |
| `gateway` | String |  | Optional. Gateway used by the connection. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `type` | String |  | Required. The type of network connectivity used by the connection. |
| `state` | String |  | Output only. The current state of the connection. |
| `uid` | String |  | Output only. A unique identifier for the instance generated by the system. |
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `parent` | String | ✅ | Required. The resource project name of the connection location using the form: `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `application_endpoint` | String | Required. Address of the remote application endpoint for the BeyondCorp Connection. |
| `connectors` | Vec<String> | Optional. List of [google.cloud.beyondcorp.v1main.Connector.name] that are authorised to be associated with this Connection. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `name` | String | Required. Unique resource name of the connection. The name is ignored when creating a connection. |
| `display_name` | String | Optional. An arbitrary user-provided name for the connection. Cannot exceed 64 characters. |
| `gateway` | String | Optional. Gateway used by the connection. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `type` | String | Required. The type of network connectivity used by the connection. |
| `state` | String | Output only. The current state of the connection. |
| `uid` | String | Output only. A unique identifier for the instance generated by the system. |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection
connection = provider.beyondcorp_api.Connection {
    parent = "value"  # Required. The resource project name of the connection location using the form: `projects/{project_id}/locations/{location_id}`
}

# Access connection outputs
connection_id = connection.id
connection_application_endpoint = connection.application_endpoint
connection_connectors = connection.connectors
connection_create_time = connection.create_time
connection_name = connection.name
connection_display_name = connection.display_name
connection_gateway = connection.gateway
connection_labels = connection.labels
connection_type = connection.type
connection_state = connection.state
connection_uid = connection.uid
connection_update_time = connection.update_time
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_metadata = location.metadata
location_name = location.name
location_labels = location.labels
location_location_id = location.location_id
```

---


### Application_domain

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create application_domain
application_domain = provider.beyondcorp_api.Application_domain {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access application_domain outputs
application_domain_id = application_domain.id
application_domain_audit_configs = application_domain.audit_configs
application_domain_version = application_domain.version
application_domain_bindings = application_domain.bindings
application_domain_etag = application_domain.etag
```

---


### Security_gateway

Creates a new Security Gateway in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. An arbitrary user-provided name for the SecurityGateway. Cannot exceed 64 characters. |
| `delegating_service_account` | String |  | Output only. Service account used for operations that involve resources in consumer projects. |
| `state` | String |  | Output only. The operational state of the SecurityGateway. |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `external_ips` | Vec<String> |  | Output only. IP addresses that will be used for establishing connection to the endpoints. |
| `proxy_protocol_config` | String |  | Optional. Shared proxy configuration for all apps. |
| `service_discovery` | String |  | Optional. Settings related to the Service Discovery. |
| `hubs` | HashMap<String, String> |  | Optional. Map of Hubs that represents regional data path deployment with GCP region as a key. |
| `name` | String |  | Identifier. Name of the resource. |
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `parent` | String | ✅ | Required. The resource project name of the SecurityGateway location using the form: `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. An arbitrary user-provided name for the SecurityGateway. Cannot exceed 64 characters. |
| `delegating_service_account` | String | Output only. Service account used for operations that involve resources in consumer projects. |
| `state` | String | Output only. The operational state of the SecurityGateway. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `external_ips` | Vec<String> | Output only. IP addresses that will be used for establishing connection to the endpoints. |
| `proxy_protocol_config` | String | Optional. Shared proxy configuration for all apps. |
| `service_discovery` | String | Optional. Settings related to the Service Discovery. |
| `hubs` | HashMap<String, String> | Optional. Map of Hubs that represents regional data path deployment with GCP region as a key. |
| `name` | String | Identifier. Name of the resource. |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_gateway
security_gateway = provider.beyondcorp_api.Security_gateway {
    parent = "value"  # Required. The resource project name of the SecurityGateway location using the form: `projects/{project_id}/locations/{location_id}`
}

# Access security_gateway outputs
security_gateway_id = security_gateway.id
security_gateway_display_name = security_gateway.display_name
security_gateway_delegating_service_account = security_gateway.delegating_service_account
security_gateway_state = security_gateway.state
security_gateway_create_time = security_gateway.create_time
security_gateway_external_ips = security_gateway.external_ips
security_gateway_proxy_protocol_config = security_gateway.proxy_protocol_config
security_gateway_service_discovery = security_gateway.service_discovery
security_gateway_hubs = security_gateway.hubs
security_gateway_name = security_gateway.name
security_gateway_update_time = security_gateway.update_time
```

---


### Application

Creates a new Application in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. An arbitrary user-provided name for the application resource. Cannot exceed 64 characters. |
| `schema` | String |  | Optional. Type of the external application. |
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `upstreams` | Vec<String> |  | Optional. Which upstream resources to forward traffic to. |
| `name` | String |  | Identifier. Name of the resource. |
| `endpoint_matchers` | Vec<String> |  | Required. Endpoint matchers associated with an application. A combination of hostname and ports as endpoint matchers is used to match the application. Match conditions for OR logic. An array of match conditions to allow for multiple matching criteria. The rule is considered a match if one of the conditions is met. The conditions should be the following combination: (Hostname & Ports) EXAMPLES: Hostname and Ports - ("*.example.com", "443"), ("example.com" and "22"), ("example.com" and "22,33") etc |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `parent` | String | ✅ | Required. The resource name of the parent SecurityGateway using the form: `projects/{project_id}/locations/global/securityGateways/{security_gateway_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. An arbitrary user-provided name for the application resource. Cannot exceed 64 characters. |
| `schema` | String | Optional. Type of the external application. |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |
| `upstreams` | Vec<String> | Optional. Which upstream resources to forward traffic to. |
| `name` | String | Identifier. Name of the resource. |
| `endpoint_matchers` | Vec<String> | Required. Endpoint matchers associated with an application. A combination of hostname and ports as endpoint matchers is used to match the application. Match conditions for OR logic. An array of match conditions to allow for multiple matching criteria. The rule is considered a match if one of the conditions is met. The conditions should be the following combination: (Hostname & Ports) EXAMPLES: Hostname and Ports - ("*.example.com", "443"), ("example.com" and "22"), ("example.com" and "22,33") etc |
| `create_time` | String | Output only. Timestamp when the resource was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create application
application = provider.beyondcorp_api.Application {
    parent = "value"  # Required. The resource name of the parent SecurityGateway using the form: `projects/{project_id}/locations/global/securityGateways/{security_gateway_id}`
}

# Access application outputs
application_id = application.id
application_display_name = application.display_name
application_schema = application.schema
application_update_time = application.update_time
application_upstreams = application.upstreams
application_name = application.name
application_endpoint_matchers = application.endpoint_matchers
application_create_time = application.create_time
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.beyondcorp_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### App_connection

Creates a new AppConnection in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user provided metadata. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `gateway` | String |  | Optional. Gateway used by the AppConnection. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `state` | String |  | Output only. The current state of the AppConnection. |
| `update_time` | String |  | Output only. Timestamp when the resource was last modified. |
| `type` | String |  | Required. The type of network connectivity used by the AppConnection. |
| `uid` | String |  | Output only. A unique identifier for the instance generated by the system. |
| `connectors` | Vec<String> |  | Optional. List of [google.cloud.beyondcorp.v1main.Connector.name] that are authorised to be associated with this AppConnection. |
| `display_name` | String |  | Optional. An arbitrary user-provided name for the AppConnection. Cannot exceed 64 characters. |
| `create_time` | String |  | Output only. Timestamp when the resource was created. |
| `application_endpoint` | String |  | Required. Address of the remote application endpoint for the BeyondCorp AppConnection. |
| `name` | String |  | Required. Unique resource name of the AppConnection. The name is ignored when creating a AppConnection. |
| `parent` | String | ✅ | Required. The resource project name of the AppConnection location using the form: `projects/{project_id}/locations/{location_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user provided metadata. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `gateway` | String | Optional. Gateway used by the AppConnection. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |
| `state` | String | Output only. The current state of the AppConnection. |
| `update_time` | String | Output only. Timestamp when the resource was last modified. |
| `type` | String | Required. The type of network connectivity used by the AppConnection. |
| `uid` | String | Output only. A unique identifier for the instance generated by the system. |
| `connectors` | Vec<String> | Optional. List of [google.cloud.beyondcorp.v1main.Connector.name] that are authorised to be associated with this AppConnection. |
| `display_name` | String | Optional. An arbitrary user-provided name for the AppConnection. Cannot exceed 64 characters. |
| `create_time` | String | Output only. Timestamp when the resource was created. |
| `application_endpoint` | String | Required. Address of the remote application endpoint for the BeyondCorp AppConnection. |
| `name` | String | Required. Unique resource name of the AppConnection. The name is ignored when creating a AppConnection. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app_connection
app_connection = provider.beyondcorp_api.App_connection {
    parent = "value"  # Required. The resource project name of the AppConnection location using the form: `projects/{project_id}/locations/{location_id}`
}

# Access app_connection outputs
app_connection_id = app_connection.id
app_connection_labels = app_connection.labels
app_connection_satisfies_pzs = app_connection.satisfies_pzs
app_connection_gateway = app_connection.gateway
app_connection_satisfies_pzi = app_connection.satisfies_pzi
app_connection_state = app_connection.state
app_connection_update_time = app_connection.update_time
app_connection_type = app_connection.type
app_connection_uid = app_connection.uid
app_connection_connectors = app_connection.connectors
app_connection_display_name = app_connection.display_name
app_connection_create_time = app_connection.create_time
app_connection_application_endpoint = app_connection.application_endpoint
app_connection_name = app_connection.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple security_gateway resources
security_gateway_0 = provider.beyondcorp_api.Security_gateway {
    parent = "value-0"
}
security_gateway_1 = provider.beyondcorp_api.Security_gateway {
    parent = "value-1"
}
security_gateway_2 = provider.beyondcorp_api.Security_gateway {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    security_gateway = provider.beyondcorp_api.Security_gateway {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Beyondcorp_api Documentation](https://cloud.google.com/beyondcorp_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
