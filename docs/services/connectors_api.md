# Connectors_api Service



**Resources**: 23

---

## Overview

The connectors_api service provides access to 23 resource types:

- [Custom_connector_version](#custom_connector_version) [CRD]
- [Runtime_entity_schema](#runtime_entity_schema) [R]
- [End_user_authentication](#end_user_authentication) [CRUD]
- [Version](#version) [R]
- [Connection](#connection) [CRUD]
- [Operation](#operation) [CRD]
- [Connector](#connector) [R]
- [Runtime_action_schema](#runtime_action_schema) [R]
- [Eventtype](#eventtype) [R]
- [Global](#global) [RU]
- [Custom_connector](#custom_connector) [CRUD]
- [Endpoint_attachment](#endpoint_attachment) [CRUD]
- [Managed_zone](#managed_zone) [CRUD]
- [Provider](#provider) [CR]
- [Event_subscription](#event_subscription) [CRUD]
- [Location](#location) [RU]
- [Connection_schema_metadata](#connection_schema_metadata) [CR]
- [Tool](#tool) [CR]
- [Connection](#connection) [CR]
- [Entity_type](#entity_type) [R]
- [Resource](#resource) [R]
- [Entitie](#entitie) [CRUD]
- [Action](#action) [CR]

---

## Resources


### Custom_connector_version

Creates a new CustomConnectorVersion in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Updated time. |
| `partner_metadata` | String |  | Optional. Partner metadata details. This should be populated only when publishing the custom connector to partner connector. |
| `async_operations_support` | bool |  | Optional. Indicates if Async Operations/Connector Job is supported. This is only available for SDK based custom connectors. |
| `publish_status` | String |  | Output only. Publish status of a custom connector. |
| `service_account` | String |  | Optional. Service account used by runtime plane to access auth config secrets. |
| `create_time` | String |  | Output only. Created time. |
| `destination_configs` | Vec<String> |  | Optional. Destination config(s) for accessing connector service (facade). This is used only when enable_backend_destination_config is true. |
| `auth_config_templates` | Vec<String> |  | Optional. Auth Config Templates is only used when connector backend is enabled. This is used to specify the auth configs supported by the connector backend service to talk to the actual application backend. |
| `state` | String |  | Output only. State of the custom connector version. |
| `auth_override_support` | bool |  | Optional. Auth override support. |
| `backend_variable_templates` | Vec<String> |  | Optional. Backend variable templates is only used when connector backend is enabled. This is used to specify the variables required by the connector backend service to talk to the actual application backend. This translates to additional variable templates in the connection config. |
| `enable_backend_destination_config` | bool |  | Optional. Indicates if an intermediatory connectorservice is used as backend. When this is enabled, the connector destination and connector auth config are required. For SDK based connectors, this is always enabled. |
| `auth_config` | String |  | Optional. Authentication config for accessing connector service (facade). This is used only when enable_backend_destination_config is true. |
| `spec_location` | String |  | Optional. Location of the custom connector spec. This is only used for Open API based custom connectors. The location can be either a public url like `https://public-url.com/spec` Or a Google Cloud Storage location like `gs:///`. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `name` | String |  | Output only. Identifier. Resource name of the Version. Format: projects/{project}/locations/{location}/customConnectors/{custom_connector}/customConnectorVersions/{custom_connector_version} |
| `spec_server_urls` | Vec<String> |  | Output only. Server URLs parsed from the Open API spec. This is only used for Open API based custom connectors. |
| `parent` | String | ✅ | Required. Parent resource of the CreateCustomConnector, of the form: `projects/{project}/locations/{location}/customConnectors/{custom_connector}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Updated time. |
| `partner_metadata` | String | Optional. Partner metadata details. This should be populated only when publishing the custom connector to partner connector. |
| `async_operations_support` | bool | Optional. Indicates if Async Operations/Connector Job is supported. This is only available for SDK based custom connectors. |
| `publish_status` | String | Output only. Publish status of a custom connector. |
| `service_account` | String | Optional. Service account used by runtime plane to access auth config secrets. |
| `create_time` | String | Output only. Created time. |
| `destination_configs` | Vec<String> | Optional. Destination config(s) for accessing connector service (facade). This is used only when enable_backend_destination_config is true. |
| `auth_config_templates` | Vec<String> | Optional. Auth Config Templates is only used when connector backend is enabled. This is used to specify the auth configs supported by the connector backend service to talk to the actual application backend. |
| `state` | String | Output only. State of the custom connector version. |
| `auth_override_support` | bool | Optional. Auth override support. |
| `backend_variable_templates` | Vec<String> | Optional. Backend variable templates is only used when connector backend is enabled. This is used to specify the variables required by the connector backend service to talk to the actual application backend. This translates to additional variable templates in the connection config. |
| `enable_backend_destination_config` | bool | Optional. Indicates if an intermediatory connectorservice is used as backend. When this is enabled, the connector destination and connector auth config are required. For SDK based connectors, this is always enabled. |
| `auth_config` | String | Optional. Authentication config for accessing connector service (facade). This is used only when enable_backend_destination_config is true. |
| `spec_location` | String | Optional. Location of the custom connector spec. This is only used for Open API based custom connectors. The location can be either a public url like `https://public-url.com/spec` Or a Google Cloud Storage location like `gs:///`. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `name` | String | Output only. Identifier. Resource name of the Version. Format: projects/{project}/locations/{location}/customConnectors/{custom_connector}/customConnectorVersions/{custom_connector_version} |
| `spec_server_urls` | Vec<String> | Output only. Server URLs parsed from the Open API spec. This is only used for Open API based custom connectors. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_connector_version
custom_connector_version = provider.connectors_api.Custom_connector_version {
    parent = "value"  # Required. Parent resource of the CreateCustomConnector, of the form: `projects/{project}/locations/{location}/customConnectors/{custom_connector}`
}

# Access custom_connector_version outputs
custom_connector_version_id = custom_connector_version.id
custom_connector_version_update_time = custom_connector_version.update_time
custom_connector_version_partner_metadata = custom_connector_version.partner_metadata
custom_connector_version_async_operations_support = custom_connector_version.async_operations_support
custom_connector_version_publish_status = custom_connector_version.publish_status
custom_connector_version_service_account = custom_connector_version.service_account
custom_connector_version_create_time = custom_connector_version.create_time
custom_connector_version_destination_configs = custom_connector_version.destination_configs
custom_connector_version_auth_config_templates = custom_connector_version.auth_config_templates
custom_connector_version_state = custom_connector_version.state
custom_connector_version_auth_override_support = custom_connector_version.auth_override_support
custom_connector_version_backend_variable_templates = custom_connector_version.backend_variable_templates
custom_connector_version_enable_backend_destination_config = custom_connector_version.enable_backend_destination_config
custom_connector_version_auth_config = custom_connector_version.auth_config
custom_connector_version_spec_location = custom_connector_version.spec_location
custom_connector_version_labels = custom_connector_version.labels
custom_connector_version_name = custom_connector_version.name
custom_connector_version_spec_server_urls = custom_connector_version.spec_server_urls
```

---


### Runtime_entity_schema

List schema of a runtime entities filtered by entity name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Next page token. |
| `runtime_entity_schemas` | Vec<String> | Runtime entity schemas. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access runtime_entity_schema outputs
runtime_entity_schema_id = runtime_entity_schema.id
runtime_entity_schema_next_page_token = runtime_entity_schema.next_page_token
runtime_entity_schema_runtime_entity_schemas = runtime_entity_schema.runtime_entity_schemas
```

---


### End_user_authentication

Creates a new EndUserAuthentication in a given project,location and connection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `config_variables` | Vec<String> |  | Optional. Config variables for the EndUserAuthentication. |
| `update_time` | String |  | Output only. Updated time. |
| `roles` | Vec<String> |  | Optional. Roles for the EndUserAuthentication. |
| `destination_configs` | Vec<String> |  | Optional. Destination configs for the EndUserAuthentication. |
| `user_id` | String |  | Optional. The user id of the user. |
| `status` | String |  | Optional. Status of the EndUserAuthentication. |
| `labels` | Vec<String> |  | Optional. Labels for the EndUserAuthentication. |
| `notify_endpoint_destination` | String |  | Optional. The destination to hit when we receive an event |
| `create_time` | String |  | Output only. Created time. |
| `name` | String |  | Required. Identifier. Resource name of the EndUserAuthentication. Format: projects/{project}/locations/{location}/connections/{connection}/endUserAuthentications/{end_user_authentication} |
| `end_user_authentication_config` | String |  | Optional. The EndUserAuthenticationConfig for the EndUserAuthentication. |
| `parent` | String | ✅ | Required. Parent resource of the EndUserAuthentication, of the form: `projects/*/locations/*/connections/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `config_variables` | Vec<String> | Optional. Config variables for the EndUserAuthentication. |
| `update_time` | String | Output only. Updated time. |
| `roles` | Vec<String> | Optional. Roles for the EndUserAuthentication. |
| `destination_configs` | Vec<String> | Optional. Destination configs for the EndUserAuthentication. |
| `user_id` | String | Optional. The user id of the user. |
| `status` | String | Optional. Status of the EndUserAuthentication. |
| `labels` | Vec<String> | Optional. Labels for the EndUserAuthentication. |
| `notify_endpoint_destination` | String | Optional. The destination to hit when we receive an event |
| `create_time` | String | Output only. Created time. |
| `name` | String | Required. Identifier. Resource name of the EndUserAuthentication. Format: projects/{project}/locations/{location}/connections/{connection}/endUserAuthentications/{end_user_authentication} |
| `end_user_authentication_config` | String | Optional. The EndUserAuthenticationConfig for the EndUserAuthentication. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create end_user_authentication
end_user_authentication = provider.connectors_api.End_user_authentication {
    parent = "value"  # Required. Parent resource of the EndUserAuthentication, of the form: `projects/*/locations/*/connections/*`
}

# Access end_user_authentication outputs
end_user_authentication_id = end_user_authentication.id
end_user_authentication_config_variables = end_user_authentication.config_variables
end_user_authentication_update_time = end_user_authentication.update_time
end_user_authentication_roles = end_user_authentication.roles
end_user_authentication_destination_configs = end_user_authentication.destination_configs
end_user_authentication_user_id = end_user_authentication.user_id
end_user_authentication_status = end_user_authentication.status
end_user_authentication_labels = end_user_authentication.labels
end_user_authentication_notify_endpoint_destination = end_user_authentication.notify_endpoint_destination
end_user_authentication_create_time = end_user_authentication.create_time
end_user_authentication_name = end_user_authentication.name
end_user_authentication_end_user_authentication_config = end_user_authentication.end_user_authentication_config
```

---


### Version

Gets details of a single connector version.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auth_config_templates` | Vec<String> | Output only. List of auth configs supported by the Connector Version. |
| `egress_control_config` | String | Output only. Configuration for Egress Control. |
| `role_grants` | Vec<String> | Output only. Role grant configurations for this connector version. |
| `schema_refresh_config` | String | Connection Schema Refresh Config |
| `create_time` | String | Output only. Created time. |
| `ssl_config_template` | String | Output only. Ssl configuration supported by the Connector. |
| `unsupported_connection_types` | Vec<String> | Output only. Unsupported connection types. |
| `connector_infra_config` | String | Output only. Infra configs supported by Connector. |
| `supported_runtime_features` | String | Output only. Information about the runtime features supported by the Connector. |
| `vpcsc_config` | String | Output only. VPCSC config for the connector. |
| `is_custom_actions_supported` | bool | Output only. Is custom actions supported. |
| `is_custom_entities_supported` | bool | Output only. Is custom entities supported. |
| `auth_override_enabled` | bool | Output only. Flag to mark the dynamic auth override. |
| `config_variable_templates` | Vec<String> | Output only. List of config variables needed to create a connection. |
| `role_grant` | String | Output only. Role grant configuration for this config variable. It will be DEPRECATED soon. |
| `name` | String | Output only. Resource name of the Version. Format: projects/{project}/locations/{location}/providers/{provider}/connectors/{connector}/versions/{version} Only global location is supported for Connector resource. |
| `destination_config_templates` | Vec<String> | Output only. List of destination configs needed to create a connection. |
| `display_name` | String | Output only. Display name. |
| `release_version` | String | Output only. ReleaseVersion of the connector, for example: "1.0.1-alpha". |
| `supported_standard_actions` | Vec<String> | Output only. Supported standard actions. |
| `supported_standard_entities` | Vec<String> | Output only. Supported standard entities. |
| `update_time` | String | Output only. Updated time. |
| `labels` | HashMap<String, String> | Output only. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `eventing_config_template` | String | Output only. Eventing configuration supported by the Connector. |
| `launch_stage` | String | Output only. Flag to mark the version indicating the launch stage. |


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
version_auth_config_templates = version.auth_config_templates
version_egress_control_config = version.egress_control_config
version_role_grants = version.role_grants
version_schema_refresh_config = version.schema_refresh_config
version_create_time = version.create_time
version_ssl_config_template = version.ssl_config_template
version_unsupported_connection_types = version.unsupported_connection_types
version_connector_infra_config = version.connector_infra_config
version_supported_runtime_features = version.supported_runtime_features
version_vpcsc_config = version.vpcsc_config
version_is_custom_actions_supported = version.is_custom_actions_supported
version_is_custom_entities_supported = version.is_custom_entities_supported
version_auth_override_enabled = version.auth_override_enabled
version_config_variable_templates = version.config_variable_templates
version_role_grant = version.role_grant
version_name = version.name
version_destination_config_templates = version.destination_config_templates
version_display_name = version.display_name
version_release_version = version.release_version
version_supported_standard_actions = version.supported_standard_actions
version_supported_standard_entities = version.supported_standard_entities
version_update_time = version.update_time
version_labels = version.labels
version_eventing_config_template = version.eventing_config_template
version_launch_stage = version.launch_stage
```

---


### Connection

Creates a new Connection in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `eua_oauth_auth_config` | String |  | Optional. Additional Oauth2.0 Auth config for EUA. If the connection is configured using non-OAuth authentication but OAuth needs to be used for EUA, this field can be populated with the OAuth config. This should be a OAuth2AuthCodeFlow Auth type only. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `lock_config` | String |  | Optional. Configuration that indicates whether or not the Connection can be edited. |
| `suspended` | bool |  | Optional. Suspended indicates if a user has suspended a connection or not. |
| `envoy_image_location` | String |  | Output only. GCR location where the envoy image is stored. formatted like: gcr.io/{bucketName}/{imageName} |
| `log_config` | String |  | Optional. Log configuration for the connection. |
| `connector_version_infra_config` | String |  | Output only. Infra configs supported by Connector Version. |
| `fallback_on_admin_credentials` | bool |  | Optional. Fallback on admin credentials for the connection. If this both auth_override_enabled and fallback_on_admin_credentials are set to true, the connection will use the admin credentials if the dynamic auth header is not present during auth override. |
| `name` | String |  | Output only. Resource name of the Connection. Format: projects/{project}/locations/{location}/connections/{connection} |
| `node_config` | String |  | Optional. Node configuration for the connection. |
| `create_time` | String |  | Output only. Created time. |
| `service_account` | String |  | Optional. Service account needed for runtime plane to access Google Cloud resources. |
| `ssl_config` | String |  | Optional. Ssl config of a connection |
| `eventing_runtime_data` | String |  | Output only. Eventing Runtime Data. |
| `async_operations_enabled` | bool |  | Optional. Async operations enabled for the connection. If Async Operations is enabled, Connection allows the customers to initiate async long running operations using the actions API. |
| `connector_version_launch_stage` | String |  | Output only. Flag to mark the version indicating the launch stage. |
| `connection_revision` | String |  | Output only. Connection revision. This field is only updated when the connection is created or updated by User. |
| `image_location` | String |  | Output only. GCR location where the runtime image is stored. formatted like: gcr.io/{bucketName}/{imageName} |
| `service_directory` | String |  | Output only. The name of the Service Directory service name. Used for Private Harpoon to resolve the ILB address. e.g. "projects/cloud-connectors-e2e-testing/locations/us-central1/namespaces/istio-system/services/istio-ingressgateway-connectors" |
| `connector_version` | String |  | Required. Connector version on which the connection is created. The format is: projects/*/locations/*/providers/*/connectors/*/versions/* Only global location is supported for ConnectorVersion resource. |
| `auth_override_enabled` | bool |  | Optional. Auth override enabled for the connection. If Auth Override is enabled, Connection allows the backend service auth to be overridden in the entities/actions API. |
| `config_variables` | Vec<String> |  | Optional. Configuration for configuring the connection with an external system. |
| `status` | String |  | Output only. Current status of the connection. |
| `description` | String |  | Optional. Description of the resource. |
| `is_trusted_tester` | bool |  | Output only. Is trusted tester program enabled for the project. |
| `billing_config` | String |  | Output only. Billing config for the connection. |
| `subscription_type` | String |  | Output only. This subscription type enum states the subscription type of the project. |
| `update_time` | String |  | Output only. Updated time. |
| `host` | String |  | Output only. The name of the Hostname of the Service Directory service with TLS. |
| `eventing_config` | String |  | Optional. Eventing config of a connection |
| `auth_config` | String |  | Optional. Configuration for establishing the connection's authentication with an external system. |
| `eventing_enablement_type` | String |  | Optional. Eventing enablement type. Will be nil if eventing is not enabled. |
| `destination_configs` | Vec<String> |  | Optional. Configuration of the Connector's destination. Only accepted for Connectors that accepts user defined destination(s). |
| `traffic_shaping_configs` | Vec<String> |  | Optional. Traffic shaping configuration for the connection. |
| `tls_service_directory` | String |  | Output only. The name of the Service Directory service with TLS. |
| `parent` | String | ✅ | Required. Parent resource of the Connection, of the form: `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `eua_oauth_auth_config` | String | Optional. Additional Oauth2.0 Auth config for EUA. If the connection is configured using non-OAuth authentication but OAuth needs to be used for EUA, this field can be populated with the OAuth config. This should be a OAuth2AuthCodeFlow Auth type only. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `lock_config` | String | Optional. Configuration that indicates whether or not the Connection can be edited. |
| `suspended` | bool | Optional. Suspended indicates if a user has suspended a connection or not. |
| `envoy_image_location` | String | Output only. GCR location where the envoy image is stored. formatted like: gcr.io/{bucketName}/{imageName} |
| `log_config` | String | Optional. Log configuration for the connection. |
| `connector_version_infra_config` | String | Output only. Infra configs supported by Connector Version. |
| `fallback_on_admin_credentials` | bool | Optional. Fallback on admin credentials for the connection. If this both auth_override_enabled and fallback_on_admin_credentials are set to true, the connection will use the admin credentials if the dynamic auth header is not present during auth override. |
| `name` | String | Output only. Resource name of the Connection. Format: projects/{project}/locations/{location}/connections/{connection} |
| `node_config` | String | Optional. Node configuration for the connection. |
| `create_time` | String | Output only. Created time. |
| `service_account` | String | Optional. Service account needed for runtime plane to access Google Cloud resources. |
| `ssl_config` | String | Optional. Ssl config of a connection |
| `eventing_runtime_data` | String | Output only. Eventing Runtime Data. |
| `async_operations_enabled` | bool | Optional. Async operations enabled for the connection. If Async Operations is enabled, Connection allows the customers to initiate async long running operations using the actions API. |
| `connector_version_launch_stage` | String | Output only. Flag to mark the version indicating the launch stage. |
| `connection_revision` | String | Output only. Connection revision. This field is only updated when the connection is created or updated by User. |
| `image_location` | String | Output only. GCR location where the runtime image is stored. formatted like: gcr.io/{bucketName}/{imageName} |
| `service_directory` | String | Output only. The name of the Service Directory service name. Used for Private Harpoon to resolve the ILB address. e.g. "projects/cloud-connectors-e2e-testing/locations/us-central1/namespaces/istio-system/services/istio-ingressgateway-connectors" |
| `connector_version` | String | Required. Connector version on which the connection is created. The format is: projects/*/locations/*/providers/*/connectors/*/versions/* Only global location is supported for ConnectorVersion resource. |
| `auth_override_enabled` | bool | Optional. Auth override enabled for the connection. If Auth Override is enabled, Connection allows the backend service auth to be overridden in the entities/actions API. |
| `config_variables` | Vec<String> | Optional. Configuration for configuring the connection with an external system. |
| `status` | String | Output only. Current status of the connection. |
| `description` | String | Optional. Description of the resource. |
| `is_trusted_tester` | bool | Output only. Is trusted tester program enabled for the project. |
| `billing_config` | String | Output only. Billing config for the connection. |
| `subscription_type` | String | Output only. This subscription type enum states the subscription type of the project. |
| `update_time` | String | Output only. Updated time. |
| `host` | String | Output only. The name of the Hostname of the Service Directory service with TLS. |
| `eventing_config` | String | Optional. Eventing config of a connection |
| `auth_config` | String | Optional. Configuration for establishing the connection's authentication with an external system. |
| `eventing_enablement_type` | String | Optional. Eventing enablement type. Will be nil if eventing is not enabled. |
| `destination_configs` | Vec<String> | Optional. Configuration of the Connector's destination. Only accepted for Connectors that accepts user defined destination(s). |
| `traffic_shaping_configs` | Vec<String> | Optional. Traffic shaping configuration for the connection. |
| `tls_service_directory` | String | Output only. The name of the Service Directory service with TLS. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection
connection = provider.connectors_api.Connection {
    parent = "value"  # Required. Parent resource of the Connection, of the form: `projects/*/locations/*`
}

# Access connection outputs
connection_id = connection.id
connection_eua_oauth_auth_config = connection.eua_oauth_auth_config
connection_labels = connection.labels
connection_lock_config = connection.lock_config
connection_suspended = connection.suspended
connection_envoy_image_location = connection.envoy_image_location
connection_log_config = connection.log_config
connection_connector_version_infra_config = connection.connector_version_infra_config
connection_fallback_on_admin_credentials = connection.fallback_on_admin_credentials
connection_name = connection.name
connection_node_config = connection.node_config
connection_create_time = connection.create_time
connection_service_account = connection.service_account
connection_ssl_config = connection.ssl_config
connection_eventing_runtime_data = connection.eventing_runtime_data
connection_async_operations_enabled = connection.async_operations_enabled
connection_connector_version_launch_stage = connection.connector_version_launch_stage
connection_connection_revision = connection.connection_revision
connection_image_location = connection.image_location
connection_service_directory = connection.service_directory
connection_connector_version = connection.connector_version
connection_auth_override_enabled = connection.auth_override_enabled
connection_config_variables = connection.config_variables
connection_status = connection.status
connection_description = connection.description
connection_is_trusted_tester = connection.is_trusted_tester
connection_billing_config = connection.billing_config
connection_subscription_type = connection.subscription_type
connection_update_time = connection.update_time
connection_host = connection.host
connection_eventing_config = connection.eventing_config
connection_auth_config = connection.auth_config
connection_eventing_enablement_type = connection.eventing_enablement_type
connection_destination_configs = connection.destination_configs
connection_traffic_shaping_configs = connection.traffic_shaping_configs
connection_tls_service_directory = connection.tls_service_directory
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.connectors_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
```

---


### Connector

Gets details of a single Connector.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `launch_stage` | String | Output only. Flag to mark the version indicating the launch stage. |
| `description` | String | Output only. Description of the resource. |
| `display_name` | String | Output only. Display name. |
| `connector_type` | String | Output only. The type of the connector. |
| `marketplace_connector_details` | String | Output only. Marketplace connector details. Will be null if the connector is not marketplace connector. |
| `eventing_details` | String | Output only. Eventing details. Will be null if eventing is not supported. |
| `web_assets_location` | String | Output only. Cloud storage location of icons etc consumed by UI. |
| `category` | String | Output only. Category of the connector. |
| `documentation_uri` | String | Output only. Link to documentation page. |
| `tags` | Vec<String> | Output only. Tags of the connector. |
| `external_uri` | String | Output only. Link to external page. |
| `update_time` | String | Output only. Updated time. |
| `labels` | HashMap<String, String> | Output only. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `create_time` | String | Output only. Created time. |
| `name` | String | Output only. Resource name of the Connector. Format: projects/{project}/locations/{location}/providers/{provider}/connectors/{connector} Only global location is supported for Connector resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access connector outputs
connector_id = connector.id
connector_launch_stage = connector.launch_stage
connector_description = connector.description
connector_display_name = connector.display_name
connector_connector_type = connector.connector_type
connector_marketplace_connector_details = connector.marketplace_connector_details
connector_eventing_details = connector.eventing_details
connector_web_assets_location = connector.web_assets_location
connector_category = connector.category
connector_documentation_uri = connector.documentation_uri
connector_tags = connector.tags
connector_external_uri = connector.external_uri
connector_update_time = connector.update_time
connector_labels = connector.labels
connector_create_time = connector.create_time
connector_name = connector.name
```

---


### Runtime_action_schema

List schema of a runtime actions filtered by action name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `runtime_action_schemas` | Vec<String> | Runtime action schemas. |
| `next_page_token` | String | Next page token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access runtime_action_schema outputs
runtime_action_schema_id = runtime_action_schema.id
runtime_action_schema_runtime_action_schemas = runtime_action_schema.runtime_action_schemas
runtime_action_schema_next_page_token = runtime_action_schema.next_page_token
```

---


### Eventtype

Gets details of a single event type.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entity_type` | String | Output only. Runtime entity type name. Will be null if entity type map is not available. Used for read before send feature. |
| `id_path` | String | Output only. Id path denotes the path of id in webhook payload. |
| `update_time` | String | Output only. Updated time. |
| `event_payload_schema` | String | Output only. Schema of webhook event payload. |
| `create_time` | String | Output only. Created time. |
| `name` | String | Output only. Resource name of the eventtype. Format: projects/{project}/locations/{location}/providers/{provider}/connectors/{connector}/versions/{version}/eventtypes/{eventtype} Only global location is supported for Connector resource. |
| `event_type_id` | String | Output only. Event type id. Example: `ticket.created`. |
| `enriched_event_payload_schema` | String | Output only. Schema of the event payload after enriched. Will be null if read before send is not supported. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access eventtype outputs
eventtype_id = eventtype.id
eventtype_entity_type = eventtype.entity_type
eventtype_id_path = eventtype.id_path
eventtype_update_time = eventtype.update_time
eventtype_event_payload_schema = eventtype.event_payload_schema
eventtype_create_time = eventtype.create_time
eventtype_name = eventtype.name
eventtype_event_type_id = eventtype.event_type_id
eventtype_enriched_event_payload_schema = eventtype.enriched_event_payload_schema
```

---


### Global

GetGlobalSettings gets settings of a project. GlobalSettings is a singleton resource.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of the Connection. Format: projects/{project}/locations/global/settings} |
| `tenant_project_id` | String |  | Output only. Tenant project id of the consumer project. |
| `vpcsc` | bool |  | Optional. Flag indicates whether vpc-sc is enabled. |
| `payg` | bool |  | Output only. Flag indicates if user is in PayG model |
| `name` | String | ✅ | Output only. Resource name of the Connection. Format: projects/{project}/locations/global/settings} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the Connection. Format: projects/{project}/locations/global/settings} |
| `tenant_project_id` | String | Output only. Tenant project id of the consumer project. |
| `vpcsc` | bool | Optional. Flag indicates whether vpc-sc is enabled. |
| `payg` | bool | Output only. Flag indicates if user is in PayG model |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access global outputs
global_id = global.id
global_name = global.name
global_tenant_project_id = global.tenant_project_id
global_vpcsc = global.vpcsc
global_payg = global.payg
```

---


### Custom_connector

Creates a new CustomConnector in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `all_marketplace_versions` | Vec<String> |  | Output only. All marketplace versions. |
| `custom_connector_type` | String |  | Required. Type of the custom connector. |
| `active_connector_versions` | Vec<String> |  | Output only. Active connector versions. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `name` | String |  | Identifier. Resource name of the CustomConnector. Format: projects/{project}/locations/{location}/customConnectors/{connector} |
| `description` | String |  | Optional. Description of the resource. |
| `published_marketplace_versions` | Vec<String> |  | Output only. Published marketplace versions. |
| `logo` | String |  | Optional. Logo of the resource. |
| `display_name` | String |  | Optional. Display name. |
| `update_time` | String |  | Output only. Updated time. |
| `create_time` | String |  | Output only. Created time. |
| `all_connector_versions` | Vec<String> |  | Output only. All connector versions. |
| `parent` | String | ✅ | Required. Parent resource of the CreateCustomConnector, of the form: `projects/{project}/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `all_marketplace_versions` | Vec<String> | Output only. All marketplace versions. |
| `custom_connector_type` | String | Required. Type of the custom connector. |
| `active_connector_versions` | Vec<String> | Output only. Active connector versions. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `name` | String | Identifier. Resource name of the CustomConnector. Format: projects/{project}/locations/{location}/customConnectors/{connector} |
| `description` | String | Optional. Description of the resource. |
| `published_marketplace_versions` | Vec<String> | Output only. Published marketplace versions. |
| `logo` | String | Optional. Logo of the resource. |
| `display_name` | String | Optional. Display name. |
| `update_time` | String | Output only. Updated time. |
| `create_time` | String | Output only. Created time. |
| `all_connector_versions` | Vec<String> | Output only. All connector versions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_connector
custom_connector = provider.connectors_api.Custom_connector {
    parent = "value"  # Required. Parent resource of the CreateCustomConnector, of the form: `projects/{project}/locations/*`
}

# Access custom_connector outputs
custom_connector_id = custom_connector.id
custom_connector_all_marketplace_versions = custom_connector.all_marketplace_versions
custom_connector_custom_connector_type = custom_connector.custom_connector_type
custom_connector_active_connector_versions = custom_connector.active_connector_versions
custom_connector_labels = custom_connector.labels
custom_connector_name = custom_connector.name
custom_connector_description = custom_connector.description
custom_connector_published_marketplace_versions = custom_connector.published_marketplace_versions
custom_connector_logo = custom_connector.logo
custom_connector_display_name = custom_connector.display_name
custom_connector_update_time = custom_connector.update_time
custom_connector_create_time = custom_connector.create_time
custom_connector_all_connector_versions = custom_connector.all_connector_versions
```

---


### Endpoint_attachment

Creates a new EndpointAttachment in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The Private Service Connect Connection Endpoint State. This value is only available in the Full view. |
| `description` | String |  | Optional. Description of the resource. |
| `name` | String |  | Output only. Resource name of the Endpoint Attachment. Format: projects/{project}/locations/{location}/endpointAttachments/{endpoint_attachment} |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `endpoint_global_access` | bool |  | Optional. The Private Service Connect Connection Endpoint Global Access. https://cloud.google.com/vpc/docs/about-accessing-vpc-hosted-services-endpoints#global-access |
| `create_time` | String |  | Output only. Created time. |
| `service_attachment` | String |  | Required. The path of the service attachment |
| `endpoint_ip` | String |  | Output only. The Private Service Connect connection endpoint ip |
| `update_time` | String |  | Output only. Updated time. |
| `parent` | String | ✅ | Required. Parent resource of the EndpointAttachment, of the form: `projects/*/locations/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The Private Service Connect Connection Endpoint State. This value is only available in the Full view. |
| `description` | String | Optional. Description of the resource. |
| `name` | String | Output only. Resource name of the Endpoint Attachment. Format: projects/{project}/locations/{location}/endpointAttachments/{endpoint_attachment} |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `endpoint_global_access` | bool | Optional. The Private Service Connect Connection Endpoint Global Access. https://cloud.google.com/vpc/docs/about-accessing-vpc-hosted-services-endpoints#global-access |
| `create_time` | String | Output only. Created time. |
| `service_attachment` | String | Required. The path of the service attachment |
| `endpoint_ip` | String | Output only. The Private Service Connect connection endpoint ip |
| `update_time` | String | Output only. Updated time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create endpoint_attachment
endpoint_attachment = provider.connectors_api.Endpoint_attachment {
    parent = "value"  # Required. Parent resource of the EndpointAttachment, of the form: `projects/*/locations/*`
}

# Access endpoint_attachment outputs
endpoint_attachment_id = endpoint_attachment.id
endpoint_attachment_state = endpoint_attachment.state
endpoint_attachment_description = endpoint_attachment.description
endpoint_attachment_name = endpoint_attachment.name
endpoint_attachment_labels = endpoint_attachment.labels
endpoint_attachment_endpoint_global_access = endpoint_attachment.endpoint_global_access
endpoint_attachment_create_time = endpoint_attachment.create_time
endpoint_attachment_service_attachment = endpoint_attachment.service_attachment
endpoint_attachment_endpoint_ip = endpoint_attachment.endpoint_ip
endpoint_attachment_update_time = endpoint_attachment.update_time
```

---


### Managed_zone

Creates a new ManagedZone in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dns` | String |  | Required. DNS Name of the resource |
| `target_project` | String |  | Required. The name of the Target Project |
| `description` | String |  | Optional. Description of the resource. |
| `create_time` | String |  | Output only. Created time. |
| `target_vpc` | String |  | Required. The name of the Target Project VPC Network |
| `update_time` | String |  | Output only. Updated time. |
| `name` | String |  | Output only. Resource name of the Managed Zone. Format: projects/{project}/locations/global/managedZones/{managed_zone} |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `parent` | String | ✅ | Required. Parent resource of the ManagedZone, of the form: `projects/*/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dns` | String | Required. DNS Name of the resource |
| `target_project` | String | Required. The name of the Target Project |
| `description` | String | Optional. Description of the resource. |
| `create_time` | String | Output only. Created time. |
| `target_vpc` | String | Required. The name of the Target Project VPC Network |
| `update_time` | String | Output only. Updated time. |
| `name` | String | Output only. Resource name of the Managed Zone. Format: projects/{project}/locations/global/managedZones/{managed_zone} |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create managed_zone
managed_zone = provider.connectors_api.Managed_zone {
    parent = "value"  # Required. Parent resource of the ManagedZone, of the form: `projects/*/locations/global`
}

# Access managed_zone outputs
managed_zone_id = managed_zone.id
managed_zone_dns = managed_zone.dns
managed_zone_target_project = managed_zone.target_project
managed_zone_description = managed_zone.description
managed_zone_create_time = managed_zone.create_time
managed_zone_target_vpc = managed_zone.target_vpc
managed_zone_update_time = managed_zone.update_time
managed_zone_name = managed_zone.name
managed_zone_labels = managed_zone.labels
```

---


### Provider

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Output only. Description of the resource. |
| `create_time` | String | Output only. Created time. |
| `launch_stage` | String | Output only. Flag to mark the version indicating the launch stage. |
| `external_uri` | String | Output only. Link to external page. |
| `name` | String | Output only. Resource name of the Provider. Format: projects/{project}/locations/{location}/providers/{provider} Only global location is supported for Provider resource. |
| `labels` | HashMap<String, String> | Output only. Resource labels to represent user-provided metadata. Refer to cloud documentation on labels for more details. https://cloud.google.com/compute/docs/labeling-resources |
| `update_time` | String | Output only. Updated time. |
| `documentation_uri` | String | Output only. Link to documentation page. |
| `web_assets_location` | String | Output only. Cloud storage location of icons etc consumed by UI. |
| `display_name` | String | Output only. Display name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create provider
provider = provider.connectors_api.Provider {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access provider outputs
provider_id = provider.id
provider_description = provider.description
provider_create_time = provider.create_time
provider_launch_stage = provider.launch_stage
provider_external_uri = provider.external_uri
provider_name = provider.name
provider_labels = provider.labels
provider_update_time = provider.update_time
provider_documentation_uri = provider.documentation_uri
provider_web_assets_location = provider.web_assets_location
provider_display_name = provider.display_name
```

---


### Event_subscription

Creates a new EventSubscription in a given project,location and connection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Created time. |
| `subscriber` | String |  | Optional. name of the Subscriber for the current EventSubscription. |
| `status` | String |  | Optional. Status indicates the status of the event subscription resource |
| `event_type_id` | String |  | Optional. Event type id of the event of current EventSubscription. |
| `jms` | String |  | Optional. JMS is the source for the event listener. |
| `name` | String |  | Required. Identifier. Resource name of the EventSubscription. Format: projects/{project}/locations/{location}/connections/{connection}/eventSubscriptions/{event_subscription} |
| `trigger_config_variables` | Vec<String> |  | Optional. Configuration for configuring the trigger |
| `subscriber_link` | String |  | Optional. Link for Subscriber of the current EventSubscription. |
| `update_time` | String |  | Output only. Updated time. |
| `destinations` | String |  | Optional. The destination to hit when we receive an event |
| `parent` | String | ✅ | Required. Parent resource of the EventSubscription, of the form: `projects/*/locations/*/connections/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Created time. |
| `subscriber` | String | Optional. name of the Subscriber for the current EventSubscription. |
| `status` | String | Optional. Status indicates the status of the event subscription resource |
| `event_type_id` | String | Optional. Event type id of the event of current EventSubscription. |
| `jms` | String | Optional. JMS is the source for the event listener. |
| `name` | String | Required. Identifier. Resource name of the EventSubscription. Format: projects/{project}/locations/{location}/connections/{connection}/eventSubscriptions/{event_subscription} |
| `trigger_config_variables` | Vec<String> | Optional. Configuration for configuring the trigger |
| `subscriber_link` | String | Optional. Link for Subscriber of the current EventSubscription. |
| `update_time` | String | Output only. Updated time. |
| `destinations` | String | Optional. The destination to hit when we receive an event |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create event_subscription
event_subscription = provider.connectors_api.Event_subscription {
    parent = "value"  # Required. Parent resource of the EventSubscription, of the form: `projects/*/locations/*/connections/*`
}

# Access event_subscription outputs
event_subscription_id = event_subscription.id
event_subscription_create_time = event_subscription.create_time
event_subscription_subscriber = event_subscription.subscriber
event_subscription_status = event_subscription.status
event_subscription_event_type_id = event_subscription.event_type_id
event_subscription_jms = event_subscription.jms
event_subscription_name = event_subscription.name
event_subscription_trigger_config_variables = event_subscription.trigger_config_variables
event_subscription_subscriber_link = event_subscription.subscriber_link
event_subscription_update_time = event_subscription.update_time
event_subscription_destinations = event_subscription.destinations
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network_config` | String |  | Optional. Regional network config. |
| `name` | String |  | Output only. Resource name of the Connection. Format: projects/{project}/locations/{location}/regionalSettings |
| `provisioned` | bool |  | Output only. Specifies whether the region is provisioned. |
| `encryption_config` | String |  | Optional. Regional encryption config to hold CMEK details. |
| `name` | String | ✅ | Output only. Resource name of the Connection. Format: projects/{project}/locations/{location}/regionalSettings |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
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
location_display_name = location.display_name
location_labels = location.labels
location_name = location.name
location_location_id = location.location_id
location_metadata = location.metadata
```

---


### Connection_schema_metadata

Refresh runtime schema of a connection.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. Resource name. Format: projects/{project}/locations/{location}/connections/{connection}/connectionSchemaMetadata |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entity_types` | Vec<String> | list of entity types |
| `next_page_token` | String | token for next page |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection_schema_metadata
connection_schema_metadata = provider.connectors_api.Connection_schema_metadata {
    name = "value"  # Required. Resource name. Format: projects/{project}/locations/{location}/connections/{connection}/connectionSchemaMetadata
}

# Access connection_schema_metadata outputs
connection_schema_metadata_id = connection_schema_metadata.id
connection_schema_metadata_entity_types = connection_schema_metadata.entity_types
connection_schema_metadata_next_page_token = connection_schema_metadata.next_page_token
```

---


### Tool

Executes a specific tool.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | HashMap<String, String> |  | Input parameters for the tool. |
| `name` | String | ✅ | Required. Resource name of the Tool. Format: projects/{project}/locations/{location}/connections/{connection}/tools/{tool} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, HashMap<String, String>> | Metadata like service latency, etc. |
| `tools` | Vec<String> | List of available tools. |
| `next_page_token` | String | Next page token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tool
tool = provider.connectors_api.Tool {
    name = "value"  # Required. Resource name of the Tool. Format: projects/{project}/locations/{location}/connections/{connection}/tools/{tool}
}

# Access tool outputs
tool_id = tool.id
tool_metadata = tool.metadata
tool_tools = tool.tools
tool_next_page_token = tool.next_page_token
```

---


### Connection

Executes a SQL statement specified in the body of the request. An example of this SQL statement in the case of Salesforce connector would be 'select * from Account a, Order o where a.Id = o.AccountId'.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query` | String |  | Required. SQL statement passed by clients like Integration Platform, the query is passed as-is to the driver used for interfacing with external systems. |
| `connection` | String | ✅ | Required. Resource name of the Connection. Format: projects/{project}/locations/{location}/connections/{connection} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, HashMap<String, String>> | Metadata like service latency, etc. |
| `state` | String | State of the connector. |
| `description` | String | When the connector is not in ACTIVE state, the description must be populated to specify the reason why it's not in ACTIVE state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connection
connection = provider.connectors_api.Connection {
    connection = "value"  # Required. Resource name of the Connection. Format: projects/{project}/locations/{location}/connections/{connection}
}

# Access connection outputs
connection_id = connection.id
connection_metadata = connection.metadata
connection_state = connection.state
connection_description = connection.description
```

---


### Entity_type

Gets metadata of given entity type

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_sort_by` | String |  |
| `metadata` | HashMap<String, HashMap<String, String>> | Metadata like service latency, etc. |
| `operations` | Vec<String> |  |
| `json_schema` | String | JsonSchema representation of this entity's schema |
| `fields` | Vec<String> | List containing metadata information about each field of the entity type. |
| `name` | String | The name of the entity type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access entity_type outputs
entity_type_id = entity_type.id
entity_type_default_sort_by = entity_type.default_sort_by
entity_type_metadata = entity_type.metadata
entity_type_operations = entity_type.operations
entity_type_json_schema = entity_type.json_schema
entity_type_fields = entity_type.fields
entity_type_name = entity_type.name
```

---


### Resource

Gets a specific resource.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The content of the resource. |
| `mime_type` | String | The MIME type of the resource. |
| `metadata` | HashMap<String, HashMap<String, String>> | Metadata like service latency, etc. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access resource outputs
resource_id = resource.id
resource_data = resource.data
resource_mime_type = resource.mime_type
resource_metadata = resource.metadata
```

---


### Entitie

Creates a new entity row of the specified entity type in the external system. The field values for creating the row are contained in the body of the request. The response message contains a `Entity` message object returned as a response by the external system.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fields` | HashMap<String, String> |  | Fields of the entity. The key is name of the field and the value contains the applicable `google.protobuf.Value` entry for this field. |
| `metadata` | HashMap<String, HashMap<String, String>> |  | Metadata like service latency, etc. |
| `name` | String |  | Output only. Resource name of the Entity. Format: projects/{project}/locations/{location}/connections/{connection}/entityTypes/{type}/entities/{id} |
| `parent` | String | ✅ | Required. Resource name of the Entity Type. Format: projects/{project}/locations/{location}/connections/{connection}/entityTypes/{type} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fields` | HashMap<String, String> | Fields of the entity. The key is name of the field and the value contains the applicable `google.protobuf.Value` entry for this field. |
| `metadata` | HashMap<String, HashMap<String, String>> | Metadata like service latency, etc. |
| `name` | String | Output only. Resource name of the Entity. Format: projects/{project}/locations/{location}/connections/{connection}/entityTypes/{type}/entities/{id} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entitie
entitie = provider.connectors_api.Entitie {
    parent = "value"  # Required. Resource name of the Entity Type. Format: projects/{project}/locations/{location}/connections/{connection}/entityTypes/{type}
}

# Access entitie outputs
entitie_id = entitie.id
entitie_fields = entitie.fields
entitie_metadata = entitie.metadata
entitie_name = entitie.name
```

---


### Action

Executes an action with the name specified in the request. The input parameters for executing the action are passed through the body of the ExecuteAction request.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | HashMap<String, String> |  | Parameters for executing the action. The parameters can be key/value pairs or nested structs. |
| `name` | String | ✅ | Required. Resource name of the Action. Format: projects/{project}/locations/{location}/connections/{connection}/actions/{action} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Display Name of action to be shown on client side |
| `result_metadata` | Vec<String> | List containing the metadata of result fields. |
| `description` | String | Brief Description of action |
| `input_parameters` | Vec<String> | List containing input parameter metadata. |
| `name` | String | Name of the action. |
| `result_json_schema` | String | JsonSchema representation of this actions's result schema |
| `metadata` | HashMap<String, HashMap<String, String>> | Metadata like service latency, etc. |
| `input_json_schema` | String | JsonSchema representation of this actions's input schema |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create action
action = provider.connectors_api.Action {
    name = "value"  # Required. Resource name of the Action. Format: projects/{project}/locations/{location}/connections/{connection}/actions/{action}
}

# Access action outputs
action_id = action.id
action_display_name = action.display_name
action_result_metadata = action.result_metadata
action_description = action.description
action_input_parameters = action.input_parameters
action_name = action.name
action_result_json_schema = action.result_json_schema
action_metadata = action.metadata
action_input_json_schema = action.input_json_schema
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple custom_connector_version resources
custom_connector_version_0 = provider.connectors_api.Custom_connector_version {
    parent = "value-0"
}
custom_connector_version_1 = provider.connectors_api.Custom_connector_version {
    parent = "value-1"
}
custom_connector_version_2 = provider.connectors_api.Custom_connector_version {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    custom_connector_version = provider.connectors_api.Custom_connector_version {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Connectors_api Documentation](https://cloud.google.com/connectors_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
