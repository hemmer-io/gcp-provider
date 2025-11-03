# Networksecurity_api Service



**Resources**: 51

---

## Overview

The networksecurity_api service provides access to 51 resource types:

- [Security_profile_group](#security_profile_group) [CRUD]
- [Firewall_endpoint](#firewall_endpoint) [CRUD]
- [Intercept_deployment_group](#intercept_deployment_group) [CRUD]
- [Intercept_deployment](#intercept_deployment) [CRUD]
- [Intercept_endpoint_group](#intercept_endpoint_group) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Authz_policie](#authz_policie) [CRUD]
- [Mirroring_endpoint_group_association](#mirroring_endpoint_group_association) [CRUD]
- [Mirroring_deployment](#mirroring_deployment) [CRUD]
- [Intercept_endpoint_group_association](#intercept_endpoint_group_association) [CRUD]
- [Url_list](#url_list) [CRUD]
- [Firewall_endpoint_association](#firewall_endpoint_association) [CRUD]
- [Client_tls_policie](#client_tls_policie) [CRUD]
- [Backend_authentication_config](#backend_authentication_config) [CRUD]
- [Authorization_policie](#authorization_policie) [CRUD]
- [Address_group](#address_group) [CRUD]
- [Mirroring_deployment_group](#mirroring_deployment_group) [CRUD]
- [Rule](#rule) [CRUD]
- [Security_profile](#security_profile) [CRUD]
- [Gateway_security_policie](#gateway_security_policie) [CRUD]
- [Server_tls_policie](#server_tls_policie) [CRUD]
- [Tls_inspection_policie](#tls_inspection_policie) [CRUD]
- [Mirroring_endpoint_group](#mirroring_endpoint_group) [CRUD]
- [Operation](#operation) [CRD]
- [Address_group](#address_group) [CRUD]
- [Firewall_endpoint](#firewall_endpoint) [CRUD]
- [Location](#location) [R]
- [Sac_realm](#sac_realm) [CRD]
- [Security_profile_group](#security_profile_group) [CRUD]
- [Server_tls_policie](#server_tls_policie) [CRUD]
- [Mirroring_endpoint_group](#mirroring_endpoint_group) [CRUD]
- [Firewall_endpoint_association](#firewall_endpoint_association) [CRUD]
- [Security_profile](#security_profile) [CRUD]
- [Dns_threat_detector](#dns_threat_detector) [CRUD]
- [Authorization_policie](#authorization_policie) [CRUD]
- [Intercept_deployment_group](#intercept_deployment_group) [CRUD]
- [Gateway_security_policie](#gateway_security_policie) [CRUD]
- [Rule](#rule) [CRUD]
- [Mirroring_deployment_group](#mirroring_deployment_group) [CRUD]
- [Mirroring_endpoint_group_association](#mirroring_endpoint_group_association) [CRUD]
- [Authz_policie](#authz_policie) [CRUD]
- [Sac_attachment](#sac_attachment) [CRD]
- [Intercept_endpoint_group](#intercept_endpoint_group) [CRUD]
- [Intercept_endpoint_group_association](#intercept_endpoint_group_association) [CRUD]
- [Mirroring_deployment](#mirroring_deployment) [CRUD]
- [Url_list](#url_list) [CRUD]
- [Tls_inspection_policie](#tls_inspection_policie) [CRUD]
- [Intercept_deployment](#intercept_deployment) [CRUD]
- [Client_tls_policie](#client_tls_policie) [CRUD]
- [Backend_authentication_config](#backend_authentication_config) [CRUD]

---

## Resources


### Security_profile_group

Creates a new SecurityProfileGroup in a given organization and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `threat_prevention_profile` | String |  | Optional. Reference to a SecurityProfile with the ThreatPrevention configuration. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `description` | String |  | Optional. An optional description of the profile group. Max length 2048 characters. |
| `data_path_id` | String |  | Output only. Identifier used by the data-path. Unique within {container, location}. |
| `create_time` | String |  | Output only. Resource creation timestamp. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `custom_mirroring_profile` | String |  | Optional. Reference to a SecurityProfile with the CustomMirroring configuration. |
| `name` | String |  | Immutable. Identifier. Name of the SecurityProfileGroup resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfileGroups/{security_profile_group}`. |
| `custom_intercept_profile` | String |  | Optional. Reference to a SecurityProfile with the CustomIntercept configuration. |
| `update_time` | String |  | Output only. Last resource update timestamp. |
| `parent` | String | ✅ | Required. The parent resource of the SecurityProfileGroup. Must be in the format `projects|organizations/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `threat_prevention_profile` | String | Optional. Reference to a SecurityProfile with the ThreatPrevention configuration. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `description` | String | Optional. An optional description of the profile group. Max length 2048 characters. |
| `data_path_id` | String | Output only. Identifier used by the data-path. Unique within {container, location}. |
| `create_time` | String | Output only. Resource creation timestamp. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `custom_mirroring_profile` | String | Optional. Reference to a SecurityProfile with the CustomMirroring configuration. |
| `name` | String | Immutable. Identifier. Name of the SecurityProfileGroup resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfileGroups/{security_profile_group}`. |
| `custom_intercept_profile` | String | Optional. Reference to a SecurityProfile with the CustomIntercept configuration. |
| `update_time` | String | Output only. Last resource update timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_profile_group
security_profile_group = provider.networksecurity_api.Security_profile_group {
    parent = "value"  # Required. The parent resource of the SecurityProfileGroup. Must be in the format `projects|organizations/*/locations/{location}`.
}

# Access security_profile_group outputs
security_profile_group_id = security_profile_group.id
security_profile_group_threat_prevention_profile = security_profile_group.threat_prevention_profile
security_profile_group_etag = security_profile_group.etag
security_profile_group_description = security_profile_group.description
security_profile_group_data_path_id = security_profile_group.data_path_id
security_profile_group_create_time = security_profile_group.create_time
security_profile_group_labels = security_profile_group.labels
security_profile_group_custom_mirroring_profile = security_profile_group.custom_mirroring_profile
security_profile_group_name = security_profile_group.name
security_profile_group_custom_intercept_profile = security_profile_group.custom_intercept_profile
security_profile_group_update_time = security_profile_group.update_time
```

---


### Firewall_endpoint

Creates a new FirewallEndpoint in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `associated_networks` | Vec<String> |  | Output only. List of networks that are associated with this endpoint in the local zone. This is a projection of the FirewallEndpointAssociations pointing at this endpoint. A network will only appear in this list after traffic routing is fully configured. Format: projects/{project}/global/networks/{name}. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `create_time` | String |  | Output only. Create time stamp. |
| `associations` | Vec<String> |  | Output only. List of FirewallEndpointAssociations that are associated to this endpoint. An association will only appear in this list after traffic routing is fully configured. |
| `endpoint_settings` | String |  | Optional. Settings for the endpoint. |
| `update_time` | String |  | Output only. Update time stamp |
| `description` | String |  | Optional. Description of the firewall endpoint. Max length 2048 characters. |
| `state` | String |  | Output only. Current state of the endpoint. |
| `name` | String |  | Immutable. Identifier. Name of resource. |
| `billing_project_id` | String |  | Required. Project to bill on endpoint uptime usage. |
| `reconciling` | bool |  | Output only. Whether reconciling is in progress, recommended per https://google.aip.dev/128. |
| `satisfies_pzi` | bool |  | Output only. [Output Only] Reserved for future use. |
| `satisfies_pzs` | bool |  | Output only. [Output Only] Reserved for future use. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associated_networks` | Vec<String> | Output only. List of networks that are associated with this endpoint in the local zone. This is a projection of the FirewallEndpointAssociations pointing at this endpoint. A network will only appear in this list after traffic routing is fully configured. Format: projects/{project}/global/networks/{name}. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `create_time` | String | Output only. Create time stamp. |
| `associations` | Vec<String> | Output only. List of FirewallEndpointAssociations that are associated to this endpoint. An association will only appear in this list after traffic routing is fully configured. |
| `endpoint_settings` | String | Optional. Settings for the endpoint. |
| `update_time` | String | Output only. Update time stamp |
| `description` | String | Optional. Description of the firewall endpoint. Max length 2048 characters. |
| `state` | String | Output only. Current state of the endpoint. |
| `name` | String | Immutable. Identifier. Name of resource. |
| `billing_project_id` | String | Required. Project to bill on endpoint uptime usage. |
| `reconciling` | bool | Output only. Whether reconciling is in progress, recommended per https://google.aip.dev/128. |
| `satisfies_pzi` | bool | Output only. [Output Only] Reserved for future use. |
| `satisfies_pzs` | bool | Output only. [Output Only] Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create firewall_endpoint
firewall_endpoint = provider.networksecurity_api.Firewall_endpoint {
    parent = "value"  # Required. Value for parent.
}

# Access firewall_endpoint outputs
firewall_endpoint_id = firewall_endpoint.id
firewall_endpoint_associated_networks = firewall_endpoint.associated_networks
firewall_endpoint_labels = firewall_endpoint.labels
firewall_endpoint_create_time = firewall_endpoint.create_time
firewall_endpoint_associations = firewall_endpoint.associations
firewall_endpoint_endpoint_settings = firewall_endpoint.endpoint_settings
firewall_endpoint_update_time = firewall_endpoint.update_time
firewall_endpoint_description = firewall_endpoint.description
firewall_endpoint_state = firewall_endpoint.state
firewall_endpoint_name = firewall_endpoint.name
firewall_endpoint_billing_project_id = firewall_endpoint.billing_project_id
firewall_endpoint_reconciling = firewall_endpoint.reconciling
firewall_endpoint_satisfies_pzi = firewall_endpoint.satisfies_pzi
firewall_endpoint_satisfies_pzs = firewall_endpoint.satisfies_pzs
```

---


### Intercept_deployment_group

Creates a deployment group in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The current state of the deployment group. See https://google.aip.dev/216. |
| `nested_deployments` | Vec<String> |  | Output only. The list of Intercept Deployments that belong to this group. |
| `locations` | Vec<String> |  | Output only. The list of locations where the deployment group is present. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `name` | String |  | Immutable. Identifier. The resource name of this deployment group, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/122 for more details. |
| `network` | String |  | Required. Immutable. The network that will be used for all child deployments, for example: `projects/{project}/global/networks/{network}`. See https://google.aip.dev/124. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `connected_endpoint_groups` | Vec<String> |  | Output only. The list of endpoint groups that are connected to this resource. |
| `description` | String |  | Optional. User-provided description of the deployment group. Used as additional context for the deployment group. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new deployment to the group) See https://google.aip.dev/128. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `parent` | String | ✅ | Required. The parent resource where this deployment group will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The current state of the deployment group. See https://google.aip.dev/216. |
| `nested_deployments` | Vec<String> | Output only. The list of Intercept Deployments that belong to this group. |
| `locations` | Vec<String> | Output only. The list of locations where the deployment group is present. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `name` | String | Immutable. Identifier. The resource name of this deployment group, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/122 for more details. |
| `network` | String | Required. Immutable. The network that will be used for all child deployments, for example: `projects/{project}/global/networks/{network}`. See https://google.aip.dev/124. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `connected_endpoint_groups` | Vec<String> | Output only. The list of endpoint groups that are connected to this resource. |
| `description` | String | Optional. User-provided description of the deployment group. Used as additional context for the deployment group. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new deployment to the group) See https://google.aip.dev/128. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intercept_deployment_group
intercept_deployment_group = provider.networksecurity_api.Intercept_deployment_group {
    parent = "value"  # Required. The parent resource where this deployment group will be created. Format: projects/{project}/locations/{location}
}

# Access intercept_deployment_group outputs
intercept_deployment_group_id = intercept_deployment_group.id
intercept_deployment_group_state = intercept_deployment_group.state
intercept_deployment_group_nested_deployments = intercept_deployment_group.nested_deployments
intercept_deployment_group_locations = intercept_deployment_group.locations
intercept_deployment_group_labels = intercept_deployment_group.labels
intercept_deployment_group_name = intercept_deployment_group.name
intercept_deployment_group_network = intercept_deployment_group.network
intercept_deployment_group_update_time = intercept_deployment_group.update_time
intercept_deployment_group_connected_endpoint_groups = intercept_deployment_group.connected_endpoint_groups
intercept_deployment_group_description = intercept_deployment_group.description
intercept_deployment_group_reconciling = intercept_deployment_group.reconciling
intercept_deployment_group_create_time = intercept_deployment_group.create_time
```

---


### Intercept_deployment

Creates a deployment in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `state` | String |  | Output only. The current state of the deployment. See https://google.aip.dev/216. |
| `description` | String |  | Optional. User-provided description of the deployment. Used as additional context for the deployment. |
| `name` | String |  | Immutable. Identifier. The resource name of this deployment, for example: `projects/123456789/locations/us-central1-a/interceptDeployments/my-dep`. See https://google.aip.dev/122 for more details. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `forwarding_rule` | String |  | Required. Immutable. The regional forwarding rule that fronts the interceptors, for example: `projects/123456789/regions/us-central1/forwardingRules/my-rule`. See https://google.aip.dev/124. |
| `intercept_deployment_group` | String |  | Required. Immutable. The deployment group that this deployment is a part of, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. linking a new association to the parent group). See https://google.aip.dev/128. |
| `parent` | String | ✅ | Required. The parent resource where this deployment will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `state` | String | Output only. The current state of the deployment. See https://google.aip.dev/216. |
| `description` | String | Optional. User-provided description of the deployment. Used as additional context for the deployment. |
| `name` | String | Immutable. Identifier. The resource name of this deployment, for example: `projects/123456789/locations/us-central1-a/interceptDeployments/my-dep`. See https://google.aip.dev/122 for more details. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `forwarding_rule` | String | Required. Immutable. The regional forwarding rule that fronts the interceptors, for example: `projects/123456789/regions/us-central1/forwardingRules/my-rule`. See https://google.aip.dev/124. |
| `intercept_deployment_group` | String | Required. Immutable. The deployment group that this deployment is a part of, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. linking a new association to the parent group). See https://google.aip.dev/128. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intercept_deployment
intercept_deployment = provider.networksecurity_api.Intercept_deployment {
    parent = "value"  # Required. The parent resource where this deployment will be created. Format: projects/{project}/locations/{location}
}

# Access intercept_deployment outputs
intercept_deployment_id = intercept_deployment.id
intercept_deployment_create_time = intercept_deployment.create_time
intercept_deployment_state = intercept_deployment.state
intercept_deployment_description = intercept_deployment.description
intercept_deployment_name = intercept_deployment.name
intercept_deployment_update_time = intercept_deployment.update_time
intercept_deployment_forwarding_rule = intercept_deployment.forwarding_rule
intercept_deployment_intercept_deployment_group = intercept_deployment.intercept_deployment_group
intercept_deployment_labels = intercept_deployment.labels
intercept_deployment_reconciling = intercept_deployment.reconciling
```

---


### Intercept_endpoint_group

Creates an endpoint group in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `intercept_deployment_group` | String |  | Required. Immutable. The deployment group that this endpoint group is connected to, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `description` | String |  | Optional. User-provided description of the endpoint group. Used as additional context for the endpoint group. |
| `associations` | Vec<String> |  | Output only. List of associations to this endpoint group. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new association to the group). See https://google.aip.dev/128. |
| `state` | String |  | Output only. The current state of the endpoint group. See https://google.aip.dev/216. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `name` | String |  | Immutable. Identifier. The resource name of this endpoint group, for example: `projects/123456789/locations/global/interceptEndpointGroups/my-eg`. See https://google.aip.dev/122 for more details. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `connected_deployment_group` | String |  | Output only. Details about the connected deployment group to this endpoint group. |
| `parent` | String | ✅ | Required. The parent resource where this endpoint group will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `intercept_deployment_group` | String | Required. Immutable. The deployment group that this endpoint group is connected to, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `description` | String | Optional. User-provided description of the endpoint group. Used as additional context for the endpoint group. |
| `associations` | Vec<String> | Output only. List of associations to this endpoint group. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new association to the group). See https://google.aip.dev/128. |
| `state` | String | Output only. The current state of the endpoint group. See https://google.aip.dev/216. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `name` | String | Immutable. Identifier. The resource name of this endpoint group, for example: `projects/123456789/locations/global/interceptEndpointGroups/my-eg`. See https://google.aip.dev/122 for more details. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `connected_deployment_group` | String | Output only. Details about the connected deployment group to this endpoint group. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intercept_endpoint_group
intercept_endpoint_group = provider.networksecurity_api.Intercept_endpoint_group {
    parent = "value"  # Required. The parent resource where this endpoint group will be created. Format: projects/{project}/locations/{location}
}

# Access intercept_endpoint_group outputs
intercept_endpoint_group_id = intercept_endpoint_group.id
intercept_endpoint_group_intercept_deployment_group = intercept_endpoint_group.intercept_deployment_group
intercept_endpoint_group_labels = intercept_endpoint_group.labels
intercept_endpoint_group_description = intercept_endpoint_group.description
intercept_endpoint_group_associations = intercept_endpoint_group.associations
intercept_endpoint_group_reconciling = intercept_endpoint_group.reconciling
intercept_endpoint_group_state = intercept_endpoint_group.state
intercept_endpoint_group_create_time = intercept_endpoint_group.create_time
intercept_endpoint_group_name = intercept_endpoint_group.name
intercept_endpoint_group_update_time = intercept_endpoint_group.update_time
intercept_endpoint_group_connected_deployment_group = intercept_endpoint_group.connected_deployment_group
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.networksecurity_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
operation_name = operation.name
operation_metadata = operation.metadata
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_location_id = location.location_id
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
```

---


### Authz_policie

Creates a new AuthzPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String |  | Required. Can be one of `ALLOW`, `DENY`, `CUSTOM`. When the action is `CUSTOM`, `customProvider` must be specified. When the action is `ALLOW`, only requests matching the policy will be allowed. When the action is `DENY`, only requests matching the policy will be denied. When a request arrives, the policies are evaluated in the following order: 1. If there is a `CUSTOM` policy that matches the request, the `CUSTOM` policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request. 2. If there are any `DENY` policies that match the request, the request is denied. 3. If there are no `ALLOW` policies for the resource or if any of the `ALLOW` policies match the request, the request is allowed. 4. Else the request is denied by default if none of the configured AuthzPolicies with `ALLOW` action match the request. |
| `name` | String |  | Required. Identifier. Name of the `AuthzPolicy` resource in the following format: `projects/{project}/locations/{location}/authzPolicies/{authz_policy}`. |
| `http_rules` | Vec<String> |  | Optional. A list of authorization HTTP rules to match against the incoming request. A policy match occurs when at least one HTTP rule matches the request or when no HTTP rules are specified in the policy. At least one HTTP Rule is required for Allow or Deny Action. Limited to 5 rules. |
| `custom_provider` | String |  | Optional. Required if the action is `CUSTOM`. Allows delegating authorization decisions to Cloud IAP or to Service Extensions. One of `cloudIap` or `authzExtension` must be specified. |
| `target` | String |  | Required. Specifies the set of resources to which this policy should be applied to. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `AuthzPolicy` resource. The format must comply with [the following requirements](/compute/docs/labeling-resources#requirements). |
| `parent` | String | ✅ | Required. The parent resource of the `AuthzPolicy` resource. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action` | String | Required. Can be one of `ALLOW`, `DENY`, `CUSTOM`. When the action is `CUSTOM`, `customProvider` must be specified. When the action is `ALLOW`, only requests matching the policy will be allowed. When the action is `DENY`, only requests matching the policy will be denied. When a request arrives, the policies are evaluated in the following order: 1. If there is a `CUSTOM` policy that matches the request, the `CUSTOM` policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request. 2. If there are any `DENY` policies that match the request, the request is denied. 3. If there are no `ALLOW` policies for the resource or if any of the `ALLOW` policies match the request, the request is allowed. 4. Else the request is denied by default if none of the configured AuthzPolicies with `ALLOW` action match the request. |
| `name` | String | Required. Identifier. Name of the `AuthzPolicy` resource in the following format: `projects/{project}/locations/{location}/authzPolicies/{authz_policy}`. |
| `http_rules` | Vec<String> | Optional. A list of authorization HTTP rules to match against the incoming request. A policy match occurs when at least one HTTP rule matches the request or when no HTTP rules are specified in the policy. At least one HTTP Rule is required for Allow or Deny Action. Limited to 5 rules. |
| `custom_provider` | String | Optional. Required if the action is `CUSTOM`. Allows delegating authorization decisions to Cloud IAP or to Service Extensions. One of `cloudIap` or `authzExtension` must be specified. |
| `target` | String | Required. Specifies the set of resources to which this policy should be applied to. |
| `description` | String | Optional. A human-readable description of the resource. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `AuthzPolicy` resource. The format must comply with [the following requirements](/compute/docs/labeling-resources#requirements). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authz_policie
authz_policie = provider.networksecurity_api.Authz_policie {
    parent = "value"  # Required. The parent resource of the `AuthzPolicy` resource. Must be in the format `projects/{project}/locations/{location}`.
}

# Access authz_policie outputs
authz_policie_id = authz_policie.id
authz_policie_action = authz_policie.action
authz_policie_name = authz_policie.name
authz_policie_http_rules = authz_policie.http_rules
authz_policie_custom_provider = authz_policie.custom_provider
authz_policie_target = authz_policie.target
authz_policie_description = authz_policie.description
authz_policie_update_time = authz_policie.update_time
authz_policie_create_time = authz_policie.create_time
authz_policie_labels = authz_policie.labels
```

---


### Mirroring_endpoint_group_association

Creates an association in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network` | String |  | Immutable. The VPC network that is associated. for example: `projects/123456789/global/networks/my-network`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `locations` | Vec<String> |  | Output only. The list of locations where the association is configured. This information is retrieved from the linked endpoint group. |
| `name` | String |  | Immutable. Identifier. The resource name of this endpoint group association, for example: `projects/123456789/locations/global/mirroringEndpointGroupAssociations/my-eg-association`. See https://google.aip.dev/122 for more details. |
| `locations_details` | Vec<String> |  | Output only. The list of locations where the association is present. This information is retrieved from the linked endpoint group, and not configured as part of the association itself. |
| `state` | String |  | Output only. Current state of the endpoint group association. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. adding a new location to the target deployment group). See https://google.aip.dev/128. |
| `mirroring_endpoint_group` | String |  | Immutable. The endpoint group that this association is connected to, for example: `projects/123456789/locations/global/mirroringEndpointGroups/my-eg`. See https://google.aip.dev/124. |
| `parent` | String | ✅ | Required. The parent resource where this association will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network` | String | Immutable. The VPC network that is associated. for example: `projects/123456789/global/networks/my-network`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `locations` | Vec<String> | Output only. The list of locations where the association is configured. This information is retrieved from the linked endpoint group. |
| `name` | String | Immutable. Identifier. The resource name of this endpoint group association, for example: `projects/123456789/locations/global/mirroringEndpointGroupAssociations/my-eg-association`. See https://google.aip.dev/122 for more details. |
| `locations_details` | Vec<String> | Output only. The list of locations where the association is present. This information is retrieved from the linked endpoint group, and not configured as part of the association itself. |
| `state` | String | Output only. Current state of the endpoint group association. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. adding a new location to the target deployment group). See https://google.aip.dev/128. |
| `mirroring_endpoint_group` | String | Immutable. The endpoint group that this association is connected to, for example: `projects/123456789/locations/global/mirroringEndpointGroups/my-eg`. See https://google.aip.dev/124. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mirroring_endpoint_group_association
mirroring_endpoint_group_association = provider.networksecurity_api.Mirroring_endpoint_group_association {
    parent = "value"  # Required. The parent resource where this association will be created. Format: projects/{project}/locations/{location}
}

# Access mirroring_endpoint_group_association outputs
mirroring_endpoint_group_association_id = mirroring_endpoint_group_association.id
mirroring_endpoint_group_association_network = mirroring_endpoint_group_association.network
mirroring_endpoint_group_association_labels = mirroring_endpoint_group_association.labels
mirroring_endpoint_group_association_locations = mirroring_endpoint_group_association.locations
mirroring_endpoint_group_association_name = mirroring_endpoint_group_association.name
mirroring_endpoint_group_association_locations_details = mirroring_endpoint_group_association.locations_details
mirroring_endpoint_group_association_state = mirroring_endpoint_group_association.state
mirroring_endpoint_group_association_update_time = mirroring_endpoint_group_association.update_time
mirroring_endpoint_group_association_create_time = mirroring_endpoint_group_association.create_time
mirroring_endpoint_group_association_reconciling = mirroring_endpoint_group_association.reconciling
mirroring_endpoint_group_association_mirroring_endpoint_group = mirroring_endpoint_group_association.mirroring_endpoint_group
```

---


### Mirroring_deployment

Creates a deployment in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. linking a new association to the parent group). See https://google.aip.dev/128. |
| `state` | String |  | Output only. The current state of the deployment. See https://google.aip.dev/216. |
| `description` | String |  | Optional. User-provided description of the deployment. Used as additional context for the deployment. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `forwarding_rule` | String |  | Required. Immutable. The regional forwarding rule that fronts the mirroring collectors, for example: `projects/123456789/regions/us-central1/forwardingRules/my-rule`. See https://google.aip.dev/124. |
| `name` | String |  | Immutable. Identifier. The resource name of this deployment, for example: `projects/123456789/locations/us-central1-a/mirroringDeployments/my-dep`. See https://google.aip.dev/122 for more details. |
| `mirroring_deployment_group` | String |  | Required. Immutable. The deployment group that this deployment is a part of, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `parent` | String | ✅ | Required. The parent resource where this deployment will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. linking a new association to the parent group). See https://google.aip.dev/128. |
| `state` | String | Output only. The current state of the deployment. See https://google.aip.dev/216. |
| `description` | String | Optional. User-provided description of the deployment. Used as additional context for the deployment. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `forwarding_rule` | String | Required. Immutable. The regional forwarding rule that fronts the mirroring collectors, for example: `projects/123456789/regions/us-central1/forwardingRules/my-rule`. See https://google.aip.dev/124. |
| `name` | String | Immutable. Identifier. The resource name of this deployment, for example: `projects/123456789/locations/us-central1-a/mirroringDeployments/my-dep`. See https://google.aip.dev/122 for more details. |
| `mirroring_deployment_group` | String | Required. Immutable. The deployment group that this deployment is a part of, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/124. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mirroring_deployment
mirroring_deployment = provider.networksecurity_api.Mirroring_deployment {
    parent = "value"  # Required. The parent resource where this deployment will be created. Format: projects/{project}/locations/{location}
}

# Access mirroring_deployment outputs
mirroring_deployment_id = mirroring_deployment.id
mirroring_deployment_update_time = mirroring_deployment.update_time
mirroring_deployment_reconciling = mirroring_deployment.reconciling
mirroring_deployment_state = mirroring_deployment.state
mirroring_deployment_description = mirroring_deployment.description
mirroring_deployment_labels = mirroring_deployment.labels
mirroring_deployment_create_time = mirroring_deployment.create_time
mirroring_deployment_forwarding_rule = mirroring_deployment.forwarding_rule
mirroring_deployment_name = mirroring_deployment.name
mirroring_deployment_mirroring_deployment_group = mirroring_deployment.mirroring_deployment_group
```

---


### Intercept_endpoint_group_association

Creates an association in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. adding a new location to the target deployment group). See https://google.aip.dev/128. |
| `state` | String |  | Output only. Current state of the endpoint group association. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `name` | String |  | Immutable. Identifier. The resource name of this endpoint group association, for example: `projects/123456789/locations/global/interceptEndpointGroupAssociations/my-eg-association`. See https://google.aip.dev/122 for more details. |
| `locations` | Vec<String> |  | Output only. The list of locations where the association is configured. This information is retrieved from the linked endpoint group. |
| `network` | String |  | Required. Immutable. The VPC network that is associated. for example: `projects/123456789/global/networks/my-network`. See https://google.aip.dev/124. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `locations_details` | Vec<String> |  | Output only. The list of locations where the association is present. This information is retrieved from the linked endpoint group, and not configured as part of the association itself. |
| `intercept_endpoint_group` | String |  | Required. Immutable. The endpoint group that this association is connected to, for example: `projects/123456789/locations/global/interceptEndpointGroups/my-eg`. See https://google.aip.dev/124. |
| `parent` | String | ✅ | Required. The parent resource where this association will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. adding a new location to the target deployment group). See https://google.aip.dev/128. |
| `state` | String | Output only. Current state of the endpoint group association. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `name` | String | Immutable. Identifier. The resource name of this endpoint group association, for example: `projects/123456789/locations/global/interceptEndpointGroupAssociations/my-eg-association`. See https://google.aip.dev/122 for more details. |
| `locations` | Vec<String> | Output only. The list of locations where the association is configured. This information is retrieved from the linked endpoint group. |
| `network` | String | Required. Immutable. The VPC network that is associated. for example: `projects/123456789/global/networks/my-network`. See https://google.aip.dev/124. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `locations_details` | Vec<String> | Output only. The list of locations where the association is present. This information is retrieved from the linked endpoint group, and not configured as part of the association itself. |
| `intercept_endpoint_group` | String | Required. Immutable. The endpoint group that this association is connected to, for example: `projects/123456789/locations/global/interceptEndpointGroups/my-eg`. See https://google.aip.dev/124. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intercept_endpoint_group_association
intercept_endpoint_group_association = provider.networksecurity_api.Intercept_endpoint_group_association {
    parent = "value"  # Required. The parent resource where this association will be created. Format: projects/{project}/locations/{location}
}

# Access intercept_endpoint_group_association outputs
intercept_endpoint_group_association_id = intercept_endpoint_group_association.id
intercept_endpoint_group_association_reconciling = intercept_endpoint_group_association.reconciling
intercept_endpoint_group_association_state = intercept_endpoint_group_association.state
intercept_endpoint_group_association_labels = intercept_endpoint_group_association.labels
intercept_endpoint_group_association_name = intercept_endpoint_group_association.name
intercept_endpoint_group_association_locations = intercept_endpoint_group_association.locations
intercept_endpoint_group_association_network = intercept_endpoint_group_association.network
intercept_endpoint_group_association_create_time = intercept_endpoint_group_association.create_time
intercept_endpoint_group_association_update_time = intercept_endpoint_group_association.update_time
intercept_endpoint_group_association_locations_details = intercept_endpoint_group_association.locations_details
intercept_endpoint_group_association_intercept_endpoint_group = intercept_endpoint_group_association.intercept_endpoint_group
```

---


### Url_list

Creates a new UrlList in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. Name of the resource provided by the user. Name is of the form projects/{project}/locations/{location}/urlLists/{url_list} url_list should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `create_time` | String |  | Output only. Time when the security policy was created. |
| `update_time` | String |  | Output only. Time when the security policy was updated. |
| `values` | Vec<String> |  | Required. FQDNs and URLs. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `parent` | String | ✅ | Required. The parent resource of the UrlList. Must be in the format `projects/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. Name of the resource provided by the user. Name is of the form projects/{project}/locations/{location}/urlLists/{url_list} url_list should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `create_time` | String | Output only. Time when the security policy was created. |
| `update_time` | String | Output only. Time when the security policy was updated. |
| `values` | Vec<String> | Required. FQDNs and URLs. |
| `description` | String | Optional. Free-text description of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create url_list
url_list = provider.networksecurity_api.Url_list {
    parent = "value"  # Required. The parent resource of the UrlList. Must be in the format `projects/*/locations/{location}`.
}

# Access url_list outputs
url_list_id = url_list.id
url_list_name = url_list.name
url_list_create_time = url_list.create_time
url_list_update_time = url_list.update_time
url_list_values = url_list.values
url_list_description = url_list.description
```

---


### Firewall_endpoint_association

Creates a new FirewallEndpointAssociation in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `update_time` | String |  | Output only. Update time stamp |
| `disabled` | bool |  | Optional. Whether the association is disabled. True indicates that traffic won't be intercepted |
| `state` | String |  | Output only. Current state of the association. |
| `create_time` | String |  | Output only. Create time stamp |
| `network` | String |  | Required. The URL of the network that is being associated. |
| `name` | String |  | Immutable. Identifier. name of resource |
| `firewall_endpoint` | String |  | Required. The URL of the FirewallEndpoint that is being associated. |
| `reconciling` | bool |  | Output only. Whether reconciling is in progress, recommended per https://google.aip.dev/128. |
| `tls_inspection_policy` | String |  | Optional. The URL of the TlsInspectionPolicy that is being associated. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `update_time` | String | Output only. Update time stamp |
| `disabled` | bool | Optional. Whether the association is disabled. True indicates that traffic won't be intercepted |
| `state` | String | Output only. Current state of the association. |
| `create_time` | String | Output only. Create time stamp |
| `network` | String | Required. The URL of the network that is being associated. |
| `name` | String | Immutable. Identifier. name of resource |
| `firewall_endpoint` | String | Required. The URL of the FirewallEndpoint that is being associated. |
| `reconciling` | bool | Output only. Whether reconciling is in progress, recommended per https://google.aip.dev/128. |
| `tls_inspection_policy` | String | Optional. The URL of the TlsInspectionPolicy that is being associated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create firewall_endpoint_association
firewall_endpoint_association = provider.networksecurity_api.Firewall_endpoint_association {
    parent = "value"  # Required. Value for parent.
}

# Access firewall_endpoint_association outputs
firewall_endpoint_association_id = firewall_endpoint_association.id
firewall_endpoint_association_labels = firewall_endpoint_association.labels
firewall_endpoint_association_update_time = firewall_endpoint_association.update_time
firewall_endpoint_association_disabled = firewall_endpoint_association.disabled
firewall_endpoint_association_state = firewall_endpoint_association.state
firewall_endpoint_association_create_time = firewall_endpoint_association.create_time
firewall_endpoint_association_network = firewall_endpoint_association.network
firewall_endpoint_association_name = firewall_endpoint_association.name
firewall_endpoint_association_firewall_endpoint = firewall_endpoint_association.firewall_endpoint
firewall_endpoint_association_reconciling = firewall_endpoint_association.reconciling
firewall_endpoint_association_tls_inspection_policy = firewall_endpoint_association.tls_inspection_policy
```

---


### Client_tls_policie

Creates a new ClientTlsPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `server_validation_ca` | Vec<String> |  | Optional. Defines the mechanism to obtain the Certificate Authority certificate to validate the server certificate. If empty, client does not validate the server certificate. |
| `name` | String |  | Required. Name of the ClientTlsPolicy resource. It matches the pattern `projects/{project}/locations/{location}/clientTlsPolicies/{client_tls_policy}` |
| `client_certificate` | String |  | Optional. Defines a mechanism to provision client identity (public and private keys) for peer to peer authentication. The presence of this dictates mTLS. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the resource. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `sni` | String |  | Optional. Server Name Indication string to present to the server during TLS handshake. E.g: "secure.example.com". |
| `parent` | String | ✅ | Required. The parent resource of the ClientTlsPolicy. Must be in the format `projects/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `server_validation_ca` | Vec<String> | Optional. Defines the mechanism to obtain the Certificate Authority certificate to validate the server certificate. If empty, client does not validate the server certificate. |
| `name` | String | Required. Name of the ClientTlsPolicy resource. It matches the pattern `projects/{project}/locations/{location}/clientTlsPolicies/{client_tls_policy}` |
| `client_certificate` | String | Optional. Defines a mechanism to provision client identity (public and private keys) for peer to peer authentication. The presence of this dictates mTLS. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the resource. |
| `description` | String | Optional. Free-text description of the resource. |
| `sni` | String | Optional. Server Name Indication string to present to the server during TLS handshake. E.g: "secure.example.com". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create client_tls_policie
client_tls_policie = provider.networksecurity_api.Client_tls_policie {
    parent = "value"  # Required. The parent resource of the ClientTlsPolicy. Must be in the format `projects/*/locations/{location}`.
}

# Access client_tls_policie outputs
client_tls_policie_id = client_tls_policie.id
client_tls_policie_update_time = client_tls_policie.update_time
client_tls_policie_create_time = client_tls_policie.create_time
client_tls_policie_server_validation_ca = client_tls_policie.server_validation_ca
client_tls_policie_name = client_tls_policie.name
client_tls_policie_client_certificate = client_tls_policie.client_certificate
client_tls_policie_labels = client_tls_policie.labels
client_tls_policie_description = client_tls_policie.description
client_tls_policie_sni = client_tls_policie.sni
```

---


### Backend_authentication_config

Creates a new BackendAuthenticationConfig in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. Name of the BackendAuthenticationConfig resource. It matches the pattern `projects/*/locations/{location}/backendAuthenticationConfigs/{backend_authentication_config}` |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `client_certificate` | String |  | Optional. A reference to a certificatemanager.googleapis.com.Certificate resource. This is a relative resource path following the form "projects/{project}/locations/{location}/certificates/{certificate}". Used by a BackendService to negotiate mTLS when the backend connection uses TLS and the backend requests a client certificate. Must have a CLIENT_AUTH scope. |
| `etag` | String |  | Output only. Etag of the resource. |
| `labels` | HashMap<String, String> |  | Set of label tags associated with the resource. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `trust_config` | String |  | Optional. A reference to a TrustConfig resource from the certificatemanager.googleapis.com namespace. This is a relative resource path following the form "projects/{project}/locations/{location}/trustConfigs/{trust_config}". A BackendService uses the chain of trust represented by this TrustConfig, if specified, to validate the server certificates presented by the backend. Required unless wellKnownRoots is set to PUBLIC_ROOTS. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `well_known_roots` | String |  | Well known roots to use for server certificate validation. |
| `parent` | String | ✅ | Required. The parent resource of the BackendAuthenticationConfig. Must be in the format `projects/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. Name of the BackendAuthenticationConfig resource. It matches the pattern `projects/*/locations/{location}/backendAuthenticationConfigs/{backend_authentication_config}` |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `client_certificate` | String | Optional. A reference to a certificatemanager.googleapis.com.Certificate resource. This is a relative resource path following the form "projects/{project}/locations/{location}/certificates/{certificate}". Used by a BackendService to negotiate mTLS when the backend connection uses TLS and the backend requests a client certificate. Must have a CLIENT_AUTH scope. |
| `etag` | String | Output only. Etag of the resource. |
| `labels` | HashMap<String, String> | Set of label tags associated with the resource. |
| `description` | String | Optional. Free-text description of the resource. |
| `trust_config` | String | Optional. A reference to a TrustConfig resource from the certificatemanager.googleapis.com namespace. This is a relative resource path following the form "projects/{project}/locations/{location}/trustConfigs/{trust_config}". A BackendService uses the chain of trust represented by this TrustConfig, if specified, to validate the server certificates presented by the backend. Required unless wellKnownRoots is set to PUBLIC_ROOTS. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `well_known_roots` | String | Well known roots to use for server certificate validation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backend_authentication_config
backend_authentication_config = provider.networksecurity_api.Backend_authentication_config {
    parent = "value"  # Required. The parent resource of the BackendAuthenticationConfig. Must be in the format `projects/*/locations/{location}`.
}

# Access backend_authentication_config outputs
backend_authentication_config_id = backend_authentication_config.id
backend_authentication_config_name = backend_authentication_config.name
backend_authentication_config_update_time = backend_authentication_config.update_time
backend_authentication_config_client_certificate = backend_authentication_config.client_certificate
backend_authentication_config_etag = backend_authentication_config.etag
backend_authentication_config_labels = backend_authentication_config.labels
backend_authentication_config_description = backend_authentication_config.description
backend_authentication_config_trust_config = backend_authentication_config.trust_config
backend_authentication_config_create_time = backend_authentication_config.create_time
backend_authentication_config_well_known_roots = backend_authentication_config.well_known_roots
```

---


### Authorization_policie

Creates a new AuthorizationPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. Name of the AuthorizationPolicy resource. It matches pattern `projects/{project}/locations/{location}/authorizationPolicies/`. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the AuthorizationPolicy resource. |
| `rules` | Vec<String> |  | Optional. List of rules to match. Note that at least one of the rules must match in order for the action specified in the 'action' field to be taken. A rule is a match if there is a matching source and destination. If left blank, the action specified in the `action` field will be applied on every request. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `action` | String |  | Required. The action to take when a rule match is found. Possible values are "ALLOW" or "DENY". |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `parent` | String | ✅ | Required. The parent resource of the AuthorizationPolicy. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. Name of the AuthorizationPolicy resource. It matches pattern `projects/{project}/locations/{location}/authorizationPolicies/`. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the AuthorizationPolicy resource. |
| `rules` | Vec<String> | Optional. List of rules to match. Note that at least one of the rules must match in order for the action specified in the 'action' field to be taken. A rule is a match if there is a matching source and destination. If left blank, the action specified in the `action` field will be applied on every request. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `action` | String | Required. The action to take when a rule match is found. Possible values are "ALLOW" or "DENY". |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `description` | String | Optional. Free-text description of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorization_policie
authorization_policie = provider.networksecurity_api.Authorization_policie {
    parent = "value"  # Required. The parent resource of the AuthorizationPolicy. Must be in the format `projects/{project}/locations/{location}`.
}

# Access authorization_policie outputs
authorization_policie_id = authorization_policie.id
authorization_policie_name = authorization_policie.name
authorization_policie_labels = authorization_policie.labels
authorization_policie_rules = authorization_policie.rules
authorization_policie_update_time = authorization_policie.update_time
authorization_policie_action = authorization_policie.action
authorization_policie_create_time = authorization_policie.create_time
authorization_policie_description = authorization_policie.description
```

---


### Address_group

Creates a new address group in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the AddressGroup resource. |
| `purpose` | Vec<String> |  | Optional. List of supported purposes of the Address Group. |
| `items` | Vec<String> |  | Optional. List of items. |
| `self_link` | String |  | Output only. Server-defined fully-qualified URL for this resource. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `capacity` | i64 |  | Required. Capacity of the Address Group |
| `type` | String |  | Required. The type of the Address Group. Possible values are "IPv4" or "IPV6". |
| `name` | String |  | Required. Name of the AddressGroup resource. It matches pattern `projects/*/locations/{location}/addressGroups/`. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `parent` | String | ✅ | Required. The parent resource of the AddressGroup. Must be in the format `projects/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the AddressGroup resource. |
| `purpose` | Vec<String> | Optional. List of supported purposes of the Address Group. |
| `items` | Vec<String> | Optional. List of items. |
| `self_link` | String | Output only. Server-defined fully-qualified URL for this resource. |
| `description` | String | Optional. Free-text description of the resource. |
| `capacity` | i64 | Required. Capacity of the Address Group |
| `type` | String | Required. The type of the Address Group. Possible values are "IPv4" or "IPV6". |
| `name` | String | Required. Name of the AddressGroup resource. It matches pattern `projects/*/locations/{location}/addressGroups/`. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create address_group
address_group = provider.networksecurity_api.Address_group {
    parent = "value"  # Required. The parent resource of the AddressGroup. Must be in the format `projects/*/locations/{location}`.
}

# Access address_group outputs
address_group_id = address_group.id
address_group_labels = address_group.labels
address_group_purpose = address_group.purpose
address_group_items = address_group.items
address_group_self_link = address_group.self_link
address_group_description = address_group.description
address_group_capacity = address_group.capacity
address_group_type = address_group.type
address_group_name = address_group.name
address_group_create_time = address_group.create_time
address_group_update_time = address_group.update_time
```

---


### Mirroring_deployment_group

Creates a deployment group in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `connected_endpoint_groups` | Vec<String> |  | Output only. The list of endpoint groups that are connected to this resource. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `description` | String |  | Optional. User-provided description of the deployment group. Used as additional context for the deployment group. |
| `network` | String |  | Required. Immutable. The network that will be used for all child deployments, for example: `projects/{project}/global/networks/{network}`. See https://google.aip.dev/124. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `name` | String |  | Immutable. Identifier. The resource name of this deployment group, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/122 for more details. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `nested_deployments` | Vec<String> |  | Output only. The list of Mirroring Deployments that belong to this group. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new deployment to the group) See https://google.aip.dev/128. |
| `locations` | Vec<String> |  | Output only. The list of locations where the deployment group is present. |
| `state` | String |  | Output only. The current state of the deployment group. See https://google.aip.dev/216. |
| `parent` | String | ✅ | Required. The parent resource where this deployment group will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `connected_endpoint_groups` | Vec<String> | Output only. The list of endpoint groups that are connected to this resource. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `description` | String | Optional. User-provided description of the deployment group. Used as additional context for the deployment group. |
| `network` | String | Required. Immutable. The network that will be used for all child deployments, for example: `projects/{project}/global/networks/{network}`. See https://google.aip.dev/124. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `name` | String | Immutable. Identifier. The resource name of this deployment group, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/122 for more details. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `nested_deployments` | Vec<String> | Output only. The list of Mirroring Deployments that belong to this group. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new deployment to the group) See https://google.aip.dev/128. |
| `locations` | Vec<String> | Output only. The list of locations where the deployment group is present. |
| `state` | String | Output only. The current state of the deployment group. See https://google.aip.dev/216. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mirroring_deployment_group
mirroring_deployment_group = provider.networksecurity_api.Mirroring_deployment_group {
    parent = "value"  # Required. The parent resource where this deployment group will be created. Format: projects/{project}/locations/{location}
}

# Access mirroring_deployment_group outputs
mirroring_deployment_group_id = mirroring_deployment_group.id
mirroring_deployment_group_connected_endpoint_groups = mirroring_deployment_group.connected_endpoint_groups
mirroring_deployment_group_update_time = mirroring_deployment_group.update_time
mirroring_deployment_group_description = mirroring_deployment_group.description
mirroring_deployment_group_network = mirroring_deployment_group.network
mirroring_deployment_group_create_time = mirroring_deployment_group.create_time
mirroring_deployment_group_name = mirroring_deployment_group.name
mirroring_deployment_group_labels = mirroring_deployment_group.labels
mirroring_deployment_group_nested_deployments = mirroring_deployment_group.nested_deployments
mirroring_deployment_group_reconciling = mirroring_deployment_group.reconciling
mirroring_deployment_group_locations = mirroring_deployment_group.locations
mirroring_deployment_group_state = mirroring_deployment_group.state
```

---


### Rule

Creates a new GatewaySecurityPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enabled` | bool |  | Required. Whether the rule is enforced. |
| `name` | String |  | Required. Immutable. Name of the resource. ame is the full resource name so projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy}/rules/{rule} rule should match the pattern: (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `basic_profile` | String |  | Required. Profile which tells what the primitive action should be. |
| `create_time` | String |  | Output only. Time when the rule was created. |
| `priority` | i64 |  | Required. Priority of the rule. Lower number corresponds to higher precedence. |
| `update_time` | String |  | Output only. Time when the rule was updated. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `session_matcher` | String |  | Required. CEL expression for matching on session criteria. |
| `application_matcher` | String |  | Optional. CEL expression for matching on L7/application level criteria. |
| `tls_inspection_enabled` | bool |  | Optional. Flag to enable TLS inspection of traffic matching on , can only be true if the parent GatewaySecurityPolicy references a TLSInspectionConfig. |
| `parent` | String | ✅ | Required. The parent where this rule will be created. Format : projects/{project}/location/{location}/gatewaySecurityPolicies/* |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enabled` | bool | Required. Whether the rule is enforced. |
| `name` | String | Required. Immutable. Name of the resource. ame is the full resource name so projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy}/rules/{rule} rule should match the pattern: (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `basic_profile` | String | Required. Profile which tells what the primitive action should be. |
| `create_time` | String | Output only. Time when the rule was created. |
| `priority` | i64 | Required. Priority of the rule. Lower number corresponds to higher precedence. |
| `update_time` | String | Output only. Time when the rule was updated. |
| `description` | String | Optional. Free-text description of the resource. |
| `session_matcher` | String | Required. CEL expression for matching on session criteria. |
| `application_matcher` | String | Optional. CEL expression for matching on L7/application level criteria. |
| `tls_inspection_enabled` | bool | Optional. Flag to enable TLS inspection of traffic matching on , can only be true if the parent GatewaySecurityPolicy references a TLSInspectionConfig. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rule
rule = provider.networksecurity_api.Rule {
    parent = "value"  # Required. The parent where this rule will be created. Format : projects/{project}/location/{location}/gatewaySecurityPolicies/*
}

# Access rule outputs
rule_id = rule.id
rule_enabled = rule.enabled
rule_name = rule.name
rule_basic_profile = rule.basic_profile
rule_create_time = rule.create_time
rule_priority = rule.priority
rule_update_time = rule.update_time
rule_description = rule.description
rule_session_matcher = rule.session_matcher
rule_application_matcher = rule.application_matcher
rule_tls_inspection_enabled = rule.tls_inspection_enabled
```

---


### Security_profile

Creates a new SecurityProfile in a given organization and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. Identifier. Name of the SecurityProfile resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfiles/{security_profile}`. |
| `custom_mirroring_profile` | String |  | The custom Packet Mirroring v2 configuration for the SecurityProfile. |
| `create_time` | String |  | Output only. Resource creation timestamp. |
| `custom_intercept_profile` | String |  | The custom TPPI configuration for the SecurityProfile. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `type` | String |  | Immutable. The single ProfileType that the SecurityProfile resource configures. |
| `threat_prevention_profile` | String |  | The threat prevention configuration for the SecurityProfile. |
| `update_time` | String |  | Output only. Last resource update timestamp. |
| `description` | String |  | Optional. An optional description of the profile. Max length 512 characters. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `parent` | String | ✅ | Required. The parent resource of the SecurityProfile. Must be in the format `projects|organizations/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Identifier. Name of the SecurityProfile resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfiles/{security_profile}`. |
| `custom_mirroring_profile` | String | The custom Packet Mirroring v2 configuration for the SecurityProfile. |
| `create_time` | String | Output only. Resource creation timestamp. |
| `custom_intercept_profile` | String | The custom TPPI configuration for the SecurityProfile. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `type` | String | Immutable. The single ProfileType that the SecurityProfile resource configures. |
| `threat_prevention_profile` | String | The threat prevention configuration for the SecurityProfile. |
| `update_time` | String | Output only. Last resource update timestamp. |
| `description` | String | Optional. An optional description of the profile. Max length 512 characters. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_profile
security_profile = provider.networksecurity_api.Security_profile {
    parent = "value"  # Required. The parent resource of the SecurityProfile. Must be in the format `projects|organizations/*/locations/{location}`.
}

# Access security_profile outputs
security_profile_id = security_profile.id
security_profile_name = security_profile.name
security_profile_custom_mirroring_profile = security_profile.custom_mirroring_profile
security_profile_create_time = security_profile.create_time
security_profile_custom_intercept_profile = security_profile.custom_intercept_profile
security_profile_etag = security_profile.etag
security_profile_type = security_profile.type
security_profile_threat_prevention_profile = security_profile.threat_prevention_profile
security_profile_update_time = security_profile.update_time
security_profile_description = security_profile.description
security_profile_labels = security_profile.labels
```

---


### Gateway_security_policie

Creates a new GatewaySecurityPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `tls_inspection_policy` | String |  | Optional. Name of a TLS Inspection Policy resource that defines how TLS inspection will be performed for any rule(s) which enables it. |
| `name` | String |  | Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy} gateway_security_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `parent` | String | ✅ | Required. The parent resource of the GatewaySecurityPolicy. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `description` | String | Optional. Free-text description of the resource. |
| `tls_inspection_policy` | String | Optional. Name of a TLS Inspection Policy resource that defines how TLS inspection will be performed for any rule(s) which enables it. |
| `name` | String | Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy} gateway_security_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `create_time` | String | Output only. The timestamp when the resource was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gateway_security_policie
gateway_security_policie = provider.networksecurity_api.Gateway_security_policie {
    parent = "value"  # Required. The parent resource of the GatewaySecurityPolicy. Must be in the format `projects/{project}/locations/{location}`.
}

# Access gateway_security_policie outputs
gateway_security_policie_id = gateway_security_policie.id
gateway_security_policie_update_time = gateway_security_policie.update_time
gateway_security_policie_description = gateway_security_policie.description
gateway_security_policie_tls_inspection_policy = gateway_security_policie.tls_inspection_policy
gateway_security_policie_name = gateway_security_policie.name
gateway_security_policie_create_time = gateway_security_policie.create_time
```

---


### Server_tls_policie

Creates a new ServerTlsPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `name` | String |  | Required. Name of the ServerTlsPolicy resource. It matches the pattern `projects/*/locations/{location}/serverTlsPolicies/{server_tls_policy}` |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `description` | String |  | Free-text description of the resource. |
| `server_certificate` | String |  | Optional if policy is to be used with Traffic Director. For Application Load Balancers must be empty. Defines a mechanism to provision server identity (public and private keys). Cannot be combined with `allow_open` as a permissive mode that allows both plain text and TLS is not supported. |
| `allow_open` | bool |  | This field applies only for Traffic Director policies. It is must be set to false for Application Load Balancer policies. Determines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if `allow_open` and `mtls_policy` are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility. Consider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80. |
| `mtls_policy` | String |  | This field is required if the policy is used with Application Load Balancers. This field can be empty for Traffic Director. Defines a mechanism to provision peer validation certificates for peer to peer authentication (Mutual TLS - mTLS). If not specified, client certificate will not be requested. The connection is treated as TLS and not mTLS. If `allow_open` and `mtls_policy` are set, server allows both plain text and mTLS connections. |
| `labels` | HashMap<String, String> |  | Set of label tags associated with the resource. |
| `parent` | String | ✅ | Required. The parent resource of the ServerTlsPolicy. Must be in the format `projects/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `name` | String | Required. Name of the ServerTlsPolicy resource. It matches the pattern `projects/*/locations/{location}/serverTlsPolicies/{server_tls_policy}` |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `description` | String | Free-text description of the resource. |
| `server_certificate` | String | Optional if policy is to be used with Traffic Director. For Application Load Balancers must be empty. Defines a mechanism to provision server identity (public and private keys). Cannot be combined with `allow_open` as a permissive mode that allows both plain text and TLS is not supported. |
| `allow_open` | bool | This field applies only for Traffic Director policies. It is must be set to false for Application Load Balancer policies. Determines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if `allow_open` and `mtls_policy` are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility. Consider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80. |
| `mtls_policy` | String | This field is required if the policy is used with Application Load Balancers. This field can be empty for Traffic Director. Defines a mechanism to provision peer validation certificates for peer to peer authentication (Mutual TLS - mTLS). If not specified, client certificate will not be requested. The connection is treated as TLS and not mTLS. If `allow_open` and `mtls_policy` are set, server allows both plain text and mTLS connections. |
| `labels` | HashMap<String, String> | Set of label tags associated with the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create server_tls_policie
server_tls_policie = provider.networksecurity_api.Server_tls_policie {
    parent = "value"  # Required. The parent resource of the ServerTlsPolicy. Must be in the format `projects/*/locations/{location}`.
}

# Access server_tls_policie outputs
server_tls_policie_id = server_tls_policie.id
server_tls_policie_update_time = server_tls_policie.update_time
server_tls_policie_name = server_tls_policie.name
server_tls_policie_create_time = server_tls_policie.create_time
server_tls_policie_description = server_tls_policie.description
server_tls_policie_server_certificate = server_tls_policie.server_certificate
server_tls_policie_allow_open = server_tls_policie.allow_open
server_tls_policie_mtls_policy = server_tls_policie.mtls_policy
server_tls_policie_labels = server_tls_policie.labels
```

---


### Tls_inspection_policie

Creates a new TlsInspectionPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `tls_feature_profile` | String |  | Optional. The selected Profile. If this is not set, then the default value is to allow the broadest set of clients and servers ("PROFILE_COMPATIBLE"). Setting this to more restrictive values may improve security, but may also prevent the TLS inspection proxy from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field. |
| `name` | String |  | Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/tlsInspectionPolicies/{tls_inspection_policy} tls_inspection_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `exclude_public_ca_set` | bool |  | Optional. If FALSE (the default), use our default set of public CAs in addition to any CAs specified in trust_config. These public CAs are currently based on the Mozilla Root Program and are subject to change over time. If TRUE, do not accept our default set of public CAs. Only CAs specified in trust_config will be accepted. This defaults to FALSE (use public CAs in addition to trust_config) for backwards compatibility, but trusting public root CAs is *not recommended* unless the traffic in question is outbound to public web servers. When possible, prefer setting this to "false" and explicitly specifying trusted CAs and certificates in a TrustConfig. Note that Secure Web Proxy does not yet honor this field. |
| `min_tls_version` | String |  | Optional. Minimum TLS version that the firewall should use when negotiating connections with both clients and servers. If this is not set, then the default value is to allow the broadest set of clients and servers (TLS 1.0 or higher). Setting this to more restrictive values may improve security, but may also prevent the firewall from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `ca_pool` | String |  | Required. A CA pool resource used to issue interception certificates. The CA pool string has a relative resource path following the form "projects/{project}/locations/{location}/caPools/{ca_pool}". |
| `trust_config` | String |  | Optional. A TrustConfig resource used when making a connection to the TLS server. This is a relative resource path following the form "projects/{project}/locations/{location}/trustConfigs/{trust_config}". This is necessary to intercept TLS connections to servers with certificates signed by a private CA or self-signed certificates. Note that Secure Web Proxy does not yet honor this field. |
| `custom_tls_features` | Vec<String> |  | Optional. List of custom TLS cipher suites selected. This field is valid only if the selected tls_feature_profile is CUSTOM. The compute.SslPoliciesService.ListAvailableFeatures method returns the set of features that can be specified in this list. Note that Secure Web Proxy does not yet honor this field. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `parent` | String | ✅ | Required. The parent resource of the TlsInspectionPolicy. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `tls_feature_profile` | String | Optional. The selected Profile. If this is not set, then the default value is to allow the broadest set of clients and servers ("PROFILE_COMPATIBLE"). Setting this to more restrictive values may improve security, but may also prevent the TLS inspection proxy from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field. |
| `name` | String | Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/tlsInspectionPolicies/{tls_inspection_policy} tls_inspection_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `exclude_public_ca_set` | bool | Optional. If FALSE (the default), use our default set of public CAs in addition to any CAs specified in trust_config. These public CAs are currently based on the Mozilla Root Program and are subject to change over time. If TRUE, do not accept our default set of public CAs. Only CAs specified in trust_config will be accepted. This defaults to FALSE (use public CAs in addition to trust_config) for backwards compatibility, but trusting public root CAs is *not recommended* unless the traffic in question is outbound to public web servers. When possible, prefer setting this to "false" and explicitly specifying trusted CAs and certificates in a TrustConfig. Note that Secure Web Proxy does not yet honor this field. |
| `min_tls_version` | String | Optional. Minimum TLS version that the firewall should use when negotiating connections with both clients and servers. If this is not set, then the default value is to allow the broadest set of clients and servers (TLS 1.0 or higher). Setting this to more restrictive values may improve security, but may also prevent the firewall from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `ca_pool` | String | Required. A CA pool resource used to issue interception certificates. The CA pool string has a relative resource path following the form "projects/{project}/locations/{location}/caPools/{ca_pool}". |
| `trust_config` | String | Optional. A TrustConfig resource used when making a connection to the TLS server. This is a relative resource path following the form "projects/{project}/locations/{location}/trustConfigs/{trust_config}". This is necessary to intercept TLS connections to servers with certificates signed by a private CA or self-signed certificates. Note that Secure Web Proxy does not yet honor this field. |
| `custom_tls_features` | Vec<String> | Optional. List of custom TLS cipher suites selected. This field is valid only if the selected tls_feature_profile is CUSTOM. The compute.SslPoliciesService.ListAvailableFeatures method returns the set of features that can be specified in this list. Note that Secure Web Proxy does not yet honor this field. |
| `description` | String | Optional. Free-text description of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tls_inspection_policie
tls_inspection_policie = provider.networksecurity_api.Tls_inspection_policie {
    parent = "value"  # Required. The parent resource of the TlsInspectionPolicy. Must be in the format `projects/{project}/locations/{location}`.
}

# Access tls_inspection_policie outputs
tls_inspection_policie_id = tls_inspection_policie.id
tls_inspection_policie_create_time = tls_inspection_policie.create_time
tls_inspection_policie_tls_feature_profile = tls_inspection_policie.tls_feature_profile
tls_inspection_policie_name = tls_inspection_policie.name
tls_inspection_policie_exclude_public_ca_set = tls_inspection_policie.exclude_public_ca_set
tls_inspection_policie_min_tls_version = tls_inspection_policie.min_tls_version
tls_inspection_policie_update_time = tls_inspection_policie.update_time
tls_inspection_policie_ca_pool = tls_inspection_policie.ca_pool
tls_inspection_policie_trust_config = tls_inspection_policie.trust_config
tls_inspection_policie_custom_tls_features = tls_inspection_policie.custom_tls_features
tls_inspection_policie_description = tls_inspection_policie.description
```

---


### Mirroring_endpoint_group

Creates an endpoint group in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `associations` | Vec<String> |  | Output only. List of associations to this endpoint group. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `connected_deployment_groups` | Vec<String> |  | Output only. List of details about the connected deployment groups to this endpoint group. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new association to the group). See https://google.aip.dev/128. |
| `mirroring_deployment_group` | String |  | Immutable. The deployment group that this DIRECT endpoint group is connected to, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `state` | String |  | Output only. The current state of the endpoint group. See https://google.aip.dev/216. |
| `description` | String |  | Optional. User-provided description of the endpoint group. Used as additional context for the endpoint group. |
| `name` | String |  | Immutable. Identifier. The resource name of this endpoint group, for example: `projects/123456789/locations/global/mirroringEndpointGroups/my-eg`. See https://google.aip.dev/122 for more details. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `parent` | String | ✅ | Required. The parent resource where this endpoint group will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associations` | Vec<String> | Output only. List of associations to this endpoint group. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `connected_deployment_groups` | Vec<String> | Output only. List of details about the connected deployment groups to this endpoint group. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new association to the group). See https://google.aip.dev/128. |
| `mirroring_deployment_group` | String | Immutable. The deployment group that this DIRECT endpoint group is connected to, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `state` | String | Output only. The current state of the endpoint group. See https://google.aip.dev/216. |
| `description` | String | Optional. User-provided description of the endpoint group. Used as additional context for the endpoint group. |
| `name` | String | Immutable. Identifier. The resource name of this endpoint group, for example: `projects/123456789/locations/global/mirroringEndpointGroups/my-eg`. See https://google.aip.dev/122 for more details. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mirroring_endpoint_group
mirroring_endpoint_group = provider.networksecurity_api.Mirroring_endpoint_group {
    parent = "value"  # Required. The parent resource where this endpoint group will be created. Format: projects/{project}/locations/{location}
}

# Access mirroring_endpoint_group outputs
mirroring_endpoint_group_id = mirroring_endpoint_group.id
mirroring_endpoint_group_associations = mirroring_endpoint_group.associations
mirroring_endpoint_group_labels = mirroring_endpoint_group.labels
mirroring_endpoint_group_create_time = mirroring_endpoint_group.create_time
mirroring_endpoint_group_connected_deployment_groups = mirroring_endpoint_group.connected_deployment_groups
mirroring_endpoint_group_reconciling = mirroring_endpoint_group.reconciling
mirroring_endpoint_group_mirroring_deployment_group = mirroring_endpoint_group.mirroring_deployment_group
mirroring_endpoint_group_state = mirroring_endpoint_group.state
mirroring_endpoint_group_description = mirroring_endpoint_group.description
mirroring_endpoint_group_name = mirroring_endpoint_group.name
mirroring_endpoint_group_update_time = mirroring_endpoint_group.update_time
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.networksecurity_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
```

---


### Address_group

Creates a new address group in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `purpose` | Vec<String> |  | Optional. List of supported purposes of the Address Group. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the AddressGroup resource. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `type` | String |  | Required. The type of the Address Group. Possible values are "IPv4" or "IPV6". |
| `items` | Vec<String> |  | Optional. List of items. |
| `capacity` | i64 |  | Required. Capacity of the Address Group |
| `description` | String |  | Optional. Free-text description of the resource. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `name` | String |  | Required. Name of the AddressGroup resource. It matches pattern `projects/*/locations/{location}/addressGroups/`. |
| `self_link` | String |  | Output only. Server-defined fully-qualified URL for this resource. |
| `parent` | String | ✅ | Required. The parent resource of the AddressGroup. Must be in the format `projects/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `purpose` | Vec<String> | Optional. List of supported purposes of the Address Group. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the AddressGroup resource. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `type` | String | Required. The type of the Address Group. Possible values are "IPv4" or "IPV6". |
| `items` | Vec<String> | Optional. List of items. |
| `capacity` | i64 | Required. Capacity of the Address Group |
| `description` | String | Optional. Free-text description of the resource. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `name` | String | Required. Name of the AddressGroup resource. It matches pattern `projects/*/locations/{location}/addressGroups/`. |
| `self_link` | String | Output only. Server-defined fully-qualified URL for this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create address_group
address_group = provider.networksecurity_api.Address_group {
    parent = "value"  # Required. The parent resource of the AddressGroup. Must be in the format `projects/*/locations/{location}`.
}

# Access address_group outputs
address_group_id = address_group.id
address_group_purpose = address_group.purpose
address_group_labels = address_group.labels
address_group_update_time = address_group.update_time
address_group_type = address_group.type
address_group_items = address_group.items
address_group_capacity = address_group.capacity
address_group_description = address_group.description
address_group_create_time = address_group.create_time
address_group_name = address_group.name
address_group_self_link = address_group.self_link
```

---


### Firewall_endpoint

Creates a new FirewallEndpoint in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. Identifier. Name of resource. |
| `associated_networks` | Vec<String> |  | Output only. List of networks that are associated with this endpoint in the local zone. This is a projection of the FirewallEndpointAssociations pointing at this endpoint. A network will only appear in this list after traffic routing is fully configured. Format: projects/{project}/global/networks/{name}. |
| `billing_project_id` | String |  | Required. Project to bill on endpoint uptime usage. |
| `description` | String |  | Optional. Description of the firewall endpoint. Max length 2048 characters. |
| `satisfies_pzs` | bool |  | Output only. [Output Only] Reserved for future use. |
| `update_time` | String |  | Output only. Update time stamp |
| `satisfies_pzi` | bool |  | Output only. [Output Only] Reserved for future use. |
| `create_time` | String |  | Output only. Create time stamp. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `endpoint_settings` | String |  | Optional. Settings for the endpoint. |
| `associations` | Vec<String> |  | Output only. List of FirewallEndpointAssociations that are associated to this endpoint. An association will only appear in this list after traffic routing is fully configured. |
| `reconciling` | bool |  | Output only. Whether reconciling is in progress, recommended per https://google.aip.dev/128. |
| `state` | String |  | Output only. Current state of the endpoint. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Identifier. Name of resource. |
| `associated_networks` | Vec<String> | Output only. List of networks that are associated with this endpoint in the local zone. This is a projection of the FirewallEndpointAssociations pointing at this endpoint. A network will only appear in this list after traffic routing is fully configured. Format: projects/{project}/global/networks/{name}. |
| `billing_project_id` | String | Required. Project to bill on endpoint uptime usage. |
| `description` | String | Optional. Description of the firewall endpoint. Max length 2048 characters. |
| `satisfies_pzs` | bool | Output only. [Output Only] Reserved for future use. |
| `update_time` | String | Output only. Update time stamp |
| `satisfies_pzi` | bool | Output only. [Output Only] Reserved for future use. |
| `create_time` | String | Output only. Create time stamp. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `endpoint_settings` | String | Optional. Settings for the endpoint. |
| `associations` | Vec<String> | Output only. List of FirewallEndpointAssociations that are associated to this endpoint. An association will only appear in this list after traffic routing is fully configured. |
| `reconciling` | bool | Output only. Whether reconciling is in progress, recommended per https://google.aip.dev/128. |
| `state` | String | Output only. Current state of the endpoint. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create firewall_endpoint
firewall_endpoint = provider.networksecurity_api.Firewall_endpoint {
    parent = "value"  # Required. Value for parent.
}

# Access firewall_endpoint outputs
firewall_endpoint_id = firewall_endpoint.id
firewall_endpoint_name = firewall_endpoint.name
firewall_endpoint_associated_networks = firewall_endpoint.associated_networks
firewall_endpoint_billing_project_id = firewall_endpoint.billing_project_id
firewall_endpoint_description = firewall_endpoint.description
firewall_endpoint_satisfies_pzs = firewall_endpoint.satisfies_pzs
firewall_endpoint_update_time = firewall_endpoint.update_time
firewall_endpoint_satisfies_pzi = firewall_endpoint.satisfies_pzi
firewall_endpoint_create_time = firewall_endpoint.create_time
firewall_endpoint_labels = firewall_endpoint.labels
firewall_endpoint_endpoint_settings = firewall_endpoint.endpoint_settings
firewall_endpoint_associations = firewall_endpoint.associations
firewall_endpoint_reconciling = firewall_endpoint.reconciling
firewall_endpoint_state = firewall_endpoint.state
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
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
location_location_id = location.location_id
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
```

---


### Sac_realm

Creates a new SACRealm in a given project.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Timestamp when the realm was created. |
| `name` | String |  | Identifier. Resource name, in the form `projects/{project}/locations/global/sacRealms/{sacRealm}`. |
| `pairing_key` | String |  | Output only. Key to be shared with SSE service provider during pairing. |
| `labels` | HashMap<String, String> |  | Optional. Optional list of labels applied to the resource. |
| `update_time` | String |  | Output only. Timestamp when the realm was last updated. |
| `symantec_options` | String |  | Optional. Configuration required for Symantec realms. |
| `security_service` | String |  | Immutable. SSE service provider associated with the realm. |
| `state` | String |  | Output only. State of the realm. |
| `parent` | String | ✅ | Required. The parent, in the form `projects/{project}/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Timestamp when the realm was created. |
| `name` | String | Identifier. Resource name, in the form `projects/{project}/locations/global/sacRealms/{sacRealm}`. |
| `pairing_key` | String | Output only. Key to be shared with SSE service provider during pairing. |
| `labels` | HashMap<String, String> | Optional. Optional list of labels applied to the resource. |
| `update_time` | String | Output only. Timestamp when the realm was last updated. |
| `symantec_options` | String | Optional. Configuration required for Symantec realms. |
| `security_service` | String | Immutable. SSE service provider associated with the realm. |
| `state` | String | Output only. State of the realm. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sac_realm
sac_realm = provider.networksecurity_api.Sac_realm {
    parent = "value"  # Required. The parent, in the form `projects/{project}/locations/global`.
}

# Access sac_realm outputs
sac_realm_id = sac_realm.id
sac_realm_create_time = sac_realm.create_time
sac_realm_name = sac_realm.name
sac_realm_pairing_key = sac_realm.pairing_key
sac_realm_labels = sac_realm.labels
sac_realm_update_time = sac_realm.update_time
sac_realm_symantec_options = sac_realm.symantec_options
sac_realm_security_service = sac_realm.security_service
sac_realm_state = sac_realm.state
```

---


### Security_profile_group

Creates a new SecurityProfileGroup in a given organization and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_path_id` | String |  | Output only. Identifier used by the data-path. Unique within {container, location}. |
| `custom_intercept_profile` | String |  | Optional. Reference to a SecurityProfile with the CustomIntercept configuration. |
| `create_time` | String |  | Output only. Resource creation timestamp. |
| `custom_mirroring_profile` | String |  | Optional. Reference to a SecurityProfile with the CustomMirroring configuration. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `update_time` | String |  | Output only. Last resource update timestamp. |
| `url_filtering_profile` | String |  | Optional. Reference to a SecurityProfile with the UrlFiltering configuration. |
| `name` | String |  | Immutable. Identifier. Name of the SecurityProfileGroup resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfileGroups/{security_profile_group}`. |
| `description` | String |  | Optional. An optional description of the profile group. Max length 2048 characters. |
| `threat_prevention_profile` | String |  | Optional. Reference to a SecurityProfile with the ThreatPrevention configuration. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `parent` | String | ✅ | Required. The parent resource of the SecurityProfileGroup. Must be in the format `projects|organizations/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data_path_id` | String | Output only. Identifier used by the data-path. Unique within {container, location}. |
| `custom_intercept_profile` | String | Optional. Reference to a SecurityProfile with the CustomIntercept configuration. |
| `create_time` | String | Output only. Resource creation timestamp. |
| `custom_mirroring_profile` | String | Optional. Reference to a SecurityProfile with the CustomMirroring configuration. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `update_time` | String | Output only. Last resource update timestamp. |
| `url_filtering_profile` | String | Optional. Reference to a SecurityProfile with the UrlFiltering configuration. |
| `name` | String | Immutable. Identifier. Name of the SecurityProfileGroup resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfileGroups/{security_profile_group}`. |
| `description` | String | Optional. An optional description of the profile group. Max length 2048 characters. |
| `threat_prevention_profile` | String | Optional. Reference to a SecurityProfile with the ThreatPrevention configuration. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_profile_group
security_profile_group = provider.networksecurity_api.Security_profile_group {
    parent = "value"  # Required. The parent resource of the SecurityProfileGroup. Must be in the format `projects|organizations/*/locations/{location}`.
}

# Access security_profile_group outputs
security_profile_group_id = security_profile_group.id
security_profile_group_data_path_id = security_profile_group.data_path_id
security_profile_group_custom_intercept_profile = security_profile_group.custom_intercept_profile
security_profile_group_create_time = security_profile_group.create_time
security_profile_group_custom_mirroring_profile = security_profile_group.custom_mirroring_profile
security_profile_group_labels = security_profile_group.labels
security_profile_group_update_time = security_profile_group.update_time
security_profile_group_url_filtering_profile = security_profile_group.url_filtering_profile
security_profile_group_name = security_profile_group.name
security_profile_group_description = security_profile_group.description
security_profile_group_threat_prevention_profile = security_profile_group.threat_prevention_profile
security_profile_group_etag = security_profile_group.etag
```

---


### Server_tls_policie

Creates a new ServerTlsPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `labels` | HashMap<String, String> |  | Set of label tags associated with the resource. |
| `server_certificate` | String |  | Optional if policy is to be used with Traffic Director. For Application Load Balancers must be empty. Defines a mechanism to provision server identity (public and private keys). Cannot be combined with `allow_open` as a permissive mode that allows both plain text and TLS is not supported. |
| `allow_open` | bool |  | This field applies only for Traffic Director policies. It is must be set to false for Application Load Balancer policies. Determines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if `allow_open` and `mtls_policy` are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility. Consider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80. |
| `description` | String |  | Free-text description of the resource. |
| `mtls_policy` | String |  | This field is required if the policy is used with Application Load Balancers. This field can be empty for Traffic Director. Defines a mechanism to provision peer validation certificates for peer to peer authentication (Mutual TLS - mTLS). If not specified, client certificate will not be requested. The connection is treated as TLS and not mTLS. If `allow_open` and `mtls_policy` are set, server allows both plain text and mTLS connections. |
| `name` | String |  | Required. Name of the ServerTlsPolicy resource. It matches the pattern `projects/*/locations/{location}/serverTlsPolicies/{server_tls_policy}` |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `parent` | String | ✅ | Required. The parent resource of the ServerTlsPolicy. Must be in the format `projects/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `labels` | HashMap<String, String> | Set of label tags associated with the resource. |
| `server_certificate` | String | Optional if policy is to be used with Traffic Director. For Application Load Balancers must be empty. Defines a mechanism to provision server identity (public and private keys). Cannot be combined with `allow_open` as a permissive mode that allows both plain text and TLS is not supported. |
| `allow_open` | bool | This field applies only for Traffic Director policies. It is must be set to false for Application Load Balancer policies. Determines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if `allow_open` and `mtls_policy` are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility. Consider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80. |
| `description` | String | Free-text description of the resource. |
| `mtls_policy` | String | This field is required if the policy is used with Application Load Balancers. This field can be empty for Traffic Director. Defines a mechanism to provision peer validation certificates for peer to peer authentication (Mutual TLS - mTLS). If not specified, client certificate will not be requested. The connection is treated as TLS and not mTLS. If `allow_open` and `mtls_policy` are set, server allows both plain text and mTLS connections. |
| `name` | String | Required. Name of the ServerTlsPolicy resource. It matches the pattern `projects/*/locations/{location}/serverTlsPolicies/{server_tls_policy}` |
| `create_time` | String | Output only. The timestamp when the resource was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create server_tls_policie
server_tls_policie = provider.networksecurity_api.Server_tls_policie {
    parent = "value"  # Required. The parent resource of the ServerTlsPolicy. Must be in the format `projects/*/locations/{location}`.
}

# Access server_tls_policie outputs
server_tls_policie_id = server_tls_policie.id
server_tls_policie_update_time = server_tls_policie.update_time
server_tls_policie_labels = server_tls_policie.labels
server_tls_policie_server_certificate = server_tls_policie.server_certificate
server_tls_policie_allow_open = server_tls_policie.allow_open
server_tls_policie_description = server_tls_policie.description
server_tls_policie_mtls_policy = server_tls_policie.mtls_policy
server_tls_policie_name = server_tls_policie.name
server_tls_policie_create_time = server_tls_policie.create_time
```

---


### Mirroring_endpoint_group

Creates an endpoint group in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mirroring_deployment_group` | String |  | Immutable. The deployment group that this DIRECT endpoint group is connected to, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new association to the group). See https://google.aip.dev/128. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `name` | String |  | Immutable. Identifier. The resource name of this endpoint group, for example: `projects/123456789/locations/global/mirroringEndpointGroups/my-eg`. See https://google.aip.dev/122 for more details. |
| `state` | String |  | Output only. The current state of the endpoint group. See https://google.aip.dev/216. |
| `associations` | Vec<String> |  | Output only. List of associations to this endpoint group. |
| `description` | String |  | Optional. User-provided description of the endpoint group. Used as additional context for the endpoint group. |
| `connected_deployment_groups` | Vec<String> |  | Output only. List of details about the connected deployment groups to this endpoint group. |
| `type` | String |  | Immutable. The type of the endpoint group. If left unspecified, defaults to DIRECT. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `parent` | String | ✅ | Required. The parent resource where this endpoint group will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mirroring_deployment_group` | String | Immutable. The deployment group that this DIRECT endpoint group is connected to, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new association to the group). See https://google.aip.dev/128. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `name` | String | Immutable. Identifier. The resource name of this endpoint group, for example: `projects/123456789/locations/global/mirroringEndpointGroups/my-eg`. See https://google.aip.dev/122 for more details. |
| `state` | String | Output only. The current state of the endpoint group. See https://google.aip.dev/216. |
| `associations` | Vec<String> | Output only. List of associations to this endpoint group. |
| `description` | String | Optional. User-provided description of the endpoint group. Used as additional context for the endpoint group. |
| `connected_deployment_groups` | Vec<String> | Output only. List of details about the connected deployment groups to this endpoint group. |
| `type` | String | Immutable. The type of the endpoint group. If left unspecified, defaults to DIRECT. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mirroring_endpoint_group
mirroring_endpoint_group = provider.networksecurity_api.Mirroring_endpoint_group {
    parent = "value"  # Required. The parent resource where this endpoint group will be created. Format: projects/{project}/locations/{location}
}

# Access mirroring_endpoint_group outputs
mirroring_endpoint_group_id = mirroring_endpoint_group.id
mirroring_endpoint_group_mirroring_deployment_group = mirroring_endpoint_group.mirroring_deployment_group
mirroring_endpoint_group_reconciling = mirroring_endpoint_group.reconciling
mirroring_endpoint_group_create_time = mirroring_endpoint_group.create_time
mirroring_endpoint_group_labels = mirroring_endpoint_group.labels
mirroring_endpoint_group_name = mirroring_endpoint_group.name
mirroring_endpoint_group_state = mirroring_endpoint_group.state
mirroring_endpoint_group_associations = mirroring_endpoint_group.associations
mirroring_endpoint_group_description = mirroring_endpoint_group.description
mirroring_endpoint_group_connected_deployment_groups = mirroring_endpoint_group.connected_deployment_groups
mirroring_endpoint_group_type = mirroring_endpoint_group.type
mirroring_endpoint_group_update_time = mirroring_endpoint_group.update_time
```

---


### Firewall_endpoint_association

Creates a new FirewallEndpointAssociation in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time stamp |
| `network` | String |  | Required. The URL of the network that is being associated. |
| `reconciling` | bool |  | Output only. Whether reconciling is in progress, recommended per https://google.aip.dev/128. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs |
| `firewall_endpoint` | String |  | Required. The URL of the FirewallEndpoint that is being associated. |
| `name` | String |  | Immutable. Identifier. name of resource |
| `disabled` | bool |  | Optional. Whether the association is disabled. True indicates that traffic won't be intercepted |
| `update_time` | String |  | Output only. Update time stamp |
| `tls_inspection_policy` | String |  | Optional. The URL of the TlsInspectionPolicy that is being associated. |
| `state` | String |  | Output only. Current state of the association. |
| `parent` | String | ✅ | Required. Value for parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time stamp |
| `network` | String | Required. The URL of the network that is being associated. |
| `reconciling` | bool | Output only. Whether reconciling is in progress, recommended per https://google.aip.dev/128. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs |
| `firewall_endpoint` | String | Required. The URL of the FirewallEndpoint that is being associated. |
| `name` | String | Immutable. Identifier. name of resource |
| `disabled` | bool | Optional. Whether the association is disabled. True indicates that traffic won't be intercepted |
| `update_time` | String | Output only. Update time stamp |
| `tls_inspection_policy` | String | Optional. The URL of the TlsInspectionPolicy that is being associated. |
| `state` | String | Output only. Current state of the association. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create firewall_endpoint_association
firewall_endpoint_association = provider.networksecurity_api.Firewall_endpoint_association {
    parent = "value"  # Required. Value for parent.
}

# Access firewall_endpoint_association outputs
firewall_endpoint_association_id = firewall_endpoint_association.id
firewall_endpoint_association_create_time = firewall_endpoint_association.create_time
firewall_endpoint_association_network = firewall_endpoint_association.network
firewall_endpoint_association_reconciling = firewall_endpoint_association.reconciling
firewall_endpoint_association_labels = firewall_endpoint_association.labels
firewall_endpoint_association_firewall_endpoint = firewall_endpoint_association.firewall_endpoint
firewall_endpoint_association_name = firewall_endpoint_association.name
firewall_endpoint_association_disabled = firewall_endpoint_association.disabled
firewall_endpoint_association_update_time = firewall_endpoint_association.update_time
firewall_endpoint_association_tls_inspection_policy = firewall_endpoint_association.tls_inspection_policy
firewall_endpoint_association_state = firewall_endpoint_association.state
```

---


### Security_profile

Creates a new SecurityProfile in a given organization and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Last resource update timestamp. |
| `url_filtering_profile` | String |  | The URL filtering configuration for the SecurityProfile. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `create_time` | String |  | Output only. Resource creation timestamp. |
| `custom_intercept_profile` | String |  | The custom TPPI configuration for the SecurityProfile. |
| `description` | String |  | Optional. An optional description of the profile. Max length 512 characters. |
| `threat_prevention_profile` | String |  | The threat prevention configuration for the SecurityProfile. |
| `custom_mirroring_profile` | String |  | The custom Packet Mirroring v2 configuration for the SecurityProfile. |
| `labels` | HashMap<String, String> |  | Optional. Labels as key value pairs. |
| `name` | String |  | Immutable. Identifier. Name of the SecurityProfile resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfiles/{security_profile}`. |
| `type` | String |  | Immutable. The single ProfileType that the SecurityProfile resource configures. |
| `parent` | String | ✅ | Required. The parent resource of the SecurityProfile. Must be in the format `projects|organizations/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Last resource update timestamp. |
| `url_filtering_profile` | String | The URL filtering configuration for the SecurityProfile. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `create_time` | String | Output only. Resource creation timestamp. |
| `custom_intercept_profile` | String | The custom TPPI configuration for the SecurityProfile. |
| `description` | String | Optional. An optional description of the profile. Max length 512 characters. |
| `threat_prevention_profile` | String | The threat prevention configuration for the SecurityProfile. |
| `custom_mirroring_profile` | String | The custom Packet Mirroring v2 configuration for the SecurityProfile. |
| `labels` | HashMap<String, String> | Optional. Labels as key value pairs. |
| `name` | String | Immutable. Identifier. Name of the SecurityProfile resource. It matches pattern `projects|organizations/*/locations/{location}/securityProfiles/{security_profile}`. |
| `type` | String | Immutable. The single ProfileType that the SecurityProfile resource configures. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create security_profile
security_profile = provider.networksecurity_api.Security_profile {
    parent = "value"  # Required. The parent resource of the SecurityProfile. Must be in the format `projects|organizations/*/locations/{location}`.
}

# Access security_profile outputs
security_profile_id = security_profile.id
security_profile_update_time = security_profile.update_time
security_profile_url_filtering_profile = security_profile.url_filtering_profile
security_profile_etag = security_profile.etag
security_profile_create_time = security_profile.create_time
security_profile_custom_intercept_profile = security_profile.custom_intercept_profile
security_profile_description = security_profile.description
security_profile_threat_prevention_profile = security_profile.threat_prevention_profile
security_profile_custom_mirroring_profile = security_profile.custom_mirroring_profile
security_profile_labels = security_profile.labels
security_profile_name = security_profile.name
security_profile_type = security_profile.type
```

---


### Dns_threat_detector

Creates a new DnsThreatDetector in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Create time stamp. |
| `excluded_networks` | Vec<String> |  | Optional. A list of network resource names which aren't monitored by this DnsThreatDetector. Example: `projects/PROJECT_ID/global/networks/NETWORK_NAME`. |
| `labels` | HashMap<String, String> |  | Optional. Any labels associated with the DnsThreatDetector, listed as key value pairs. |
| `name` | String |  | Immutable. Identifier. Name of the DnsThreatDetector resource. |
| `provider` | String |  | Required. The provider used for DNS threat analysis. |
| `update_time` | String |  | Output only. Update time stamp. |
| `parent` | String | ✅ | Required. The value for the parent of the DnsThreatDetector resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Create time stamp. |
| `excluded_networks` | Vec<String> | Optional. A list of network resource names which aren't monitored by this DnsThreatDetector. Example: `projects/PROJECT_ID/global/networks/NETWORK_NAME`. |
| `labels` | HashMap<String, String> | Optional. Any labels associated with the DnsThreatDetector, listed as key value pairs. |
| `name` | String | Immutable. Identifier. Name of the DnsThreatDetector resource. |
| `provider` | String | Required. The provider used for DNS threat analysis. |
| `update_time` | String | Output only. Update time stamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dns_threat_detector
dns_threat_detector = provider.networksecurity_api.Dns_threat_detector {
    parent = "value"  # Required. The value for the parent of the DnsThreatDetector resource.
}

# Access dns_threat_detector outputs
dns_threat_detector_id = dns_threat_detector.id
dns_threat_detector_create_time = dns_threat_detector.create_time
dns_threat_detector_excluded_networks = dns_threat_detector.excluded_networks
dns_threat_detector_labels = dns_threat_detector.labels
dns_threat_detector_name = dns_threat_detector.name
dns_threat_detector_provider = dns_threat_detector.provider
dns_threat_detector_update_time = dns_threat_detector.update_time
```

---


### Authorization_policie

Creates a new AuthorizationPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. Name of the AuthorizationPolicy resource. It matches pattern `projects/{project}/locations/{location}/authorizationPolicies/`. |
| `rules` | Vec<String> |  | Optional. List of rules to match. Note that at least one of the rules must match in order for the action specified in the 'action' field to be taken. A rule is a match if there is a matching source and destination. If left blank, the action specified in the `action` field will be applied on every request. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `action` | String |  | Required. The action to take when a rule match is found. Possible values are "ALLOW" or "DENY". |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the AuthorizationPolicy resource. |
| `parent` | String | ✅ | Required. The parent resource of the AuthorizationPolicy. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. Name of the AuthorizationPolicy resource. It matches pattern `projects/{project}/locations/{location}/authorizationPolicies/`. |
| `rules` | Vec<String> | Optional. List of rules to match. Note that at least one of the rules must match in order for the action specified in the 'action' field to be taken. A rule is a match if there is a matching source and destination. If left blank, the action specified in the `action` field will be applied on every request. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `action` | String | Required. The action to take when a rule match is found. Possible values are "ALLOW" or "DENY". |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `description` | String | Optional. Free-text description of the resource. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the AuthorizationPolicy resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorization_policie
authorization_policie = provider.networksecurity_api.Authorization_policie {
    parent = "value"  # Required. The parent resource of the AuthorizationPolicy. Must be in the format `projects/{project}/locations/{location}`.
}

# Access authorization_policie outputs
authorization_policie_id = authorization_policie.id
authorization_policie_name = authorization_policie.name
authorization_policie_rules = authorization_policie.rules
authorization_policie_update_time = authorization_policie.update_time
authorization_policie_action = authorization_policie.action
authorization_policie_create_time = authorization_policie.create_time
authorization_policie_description = authorization_policie.description
authorization_policie_labels = authorization_policie.labels
```

---


### Intercept_deployment_group

Creates a deployment group in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `network` | String |  | Required. Immutable. The network that will be used for all child deployments, for example: `projects/{project}/global/networks/{network}`. See https://google.aip.dev/124. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `connected_endpoint_groups` | Vec<String> |  | Output only. The list of endpoint groups that are connected to this resource. |
| `locations` | Vec<String> |  | Output only. The list of locations where the deployment group is present. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `nested_deployments` | Vec<String> |  | Output only. The list of Intercept Deployments that belong to this group. |
| `state` | String |  | Output only. The current state of the deployment group. See https://google.aip.dev/216. |
| `name` | String |  | Immutable. Identifier. The resource name of this deployment group, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/122 for more details. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new deployment to the group) See https://google.aip.dev/128. |
| `description` | String |  | Optional. User-provided description of the deployment group. Used as additional context for the deployment group. |
| `parent` | String | ✅ | Required. The parent resource where this deployment group will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `network` | String | Required. Immutable. The network that will be used for all child deployments, for example: `projects/{project}/global/networks/{network}`. See https://google.aip.dev/124. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `connected_endpoint_groups` | Vec<String> | Output only. The list of endpoint groups that are connected to this resource. |
| `locations` | Vec<String> | Output only. The list of locations where the deployment group is present. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `nested_deployments` | Vec<String> | Output only. The list of Intercept Deployments that belong to this group. |
| `state` | String | Output only. The current state of the deployment group. See https://google.aip.dev/216. |
| `name` | String | Immutable. Identifier. The resource name of this deployment group, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/122 for more details. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new deployment to the group) See https://google.aip.dev/128. |
| `description` | String | Optional. User-provided description of the deployment group. Used as additional context for the deployment group. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intercept_deployment_group
intercept_deployment_group = provider.networksecurity_api.Intercept_deployment_group {
    parent = "value"  # Required. The parent resource where this deployment group will be created. Format: projects/{project}/locations/{location}
}

# Access intercept_deployment_group outputs
intercept_deployment_group_id = intercept_deployment_group.id
intercept_deployment_group_network = intercept_deployment_group.network
intercept_deployment_group_update_time = intercept_deployment_group.update_time
intercept_deployment_group_connected_endpoint_groups = intercept_deployment_group.connected_endpoint_groups
intercept_deployment_group_locations = intercept_deployment_group.locations
intercept_deployment_group_create_time = intercept_deployment_group.create_time
intercept_deployment_group_nested_deployments = intercept_deployment_group.nested_deployments
intercept_deployment_group_state = intercept_deployment_group.state
intercept_deployment_group_name = intercept_deployment_group.name
intercept_deployment_group_labels = intercept_deployment_group.labels
intercept_deployment_group_reconciling = intercept_deployment_group.reconciling
intercept_deployment_group_description = intercept_deployment_group.description
```

---


### Gateway_security_policie

Creates a new GatewaySecurityPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `tls_inspection_policy` | String |  | Optional. Name of a TLS Inspection Policy resource that defines how TLS inspection will be performed for any rule(s) which enables it. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `name` | String |  | Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy} gateway_security_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `parent` | String | ✅ | Required. The parent resource of the GatewaySecurityPolicy. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `description` | String | Optional. Free-text description of the resource. |
| `tls_inspection_policy` | String | Optional. Name of a TLS Inspection Policy resource that defines how TLS inspection will be performed for any rule(s) which enables it. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `name` | String | Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy} gateway_security_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gateway_security_policie
gateway_security_policie = provider.networksecurity_api.Gateway_security_policie {
    parent = "value"  # Required. The parent resource of the GatewaySecurityPolicy. Must be in the format `projects/{project}/locations/{location}`.
}

# Access gateway_security_policie outputs
gateway_security_policie_id = gateway_security_policie.id
gateway_security_policie_create_time = gateway_security_policie.create_time
gateway_security_policie_description = gateway_security_policie.description
gateway_security_policie_tls_inspection_policy = gateway_security_policie.tls_inspection_policy
gateway_security_policie_update_time = gateway_security_policie.update_time
gateway_security_policie_name = gateway_security_policie.name
```

---


### Rule

Creates a new GatewaySecurityPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session_matcher` | String |  | Required. CEL expression for matching on session criteria. |
| `update_time` | String |  | Output only. Time when the rule was updated. |
| `enabled` | bool |  | Required. Whether the rule is enforced. |
| `name` | String |  | Required. Immutable. Name of the resource. ame is the full resource name so projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy}/rules/{rule} rule should match the pattern: (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `description` | String |  | Optional. Free-text description of the resource. |
| `priority` | i64 |  | Required. Priority of the rule. Lower number corresponds to higher precedence. |
| `application_matcher` | String |  | Optional. CEL expression for matching on L7/application level criteria. |
| `tls_inspection_enabled` | bool |  | Optional. Flag to enable TLS inspection of traffic matching on , can only be true if the parent GatewaySecurityPolicy references a TLSInspectionConfig. |
| `basic_profile` | String |  | Required. Profile which tells what the primitive action should be. |
| `create_time` | String |  | Output only. Time when the rule was created. |
| `parent` | String | ✅ | Required. The parent where this rule will be created. Format : projects/{project}/location/{location}/gatewaySecurityPolicies/* |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `session_matcher` | String | Required. CEL expression for matching on session criteria. |
| `update_time` | String | Output only. Time when the rule was updated. |
| `enabled` | bool | Required. Whether the rule is enforced. |
| `name` | String | Required. Immutable. Name of the resource. ame is the full resource name so projects/{project}/locations/{location}/gatewaySecurityPolicies/{gateway_security_policy}/rules/{rule} rule should match the pattern: (^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `description` | String | Optional. Free-text description of the resource. |
| `priority` | i64 | Required. Priority of the rule. Lower number corresponds to higher precedence. |
| `application_matcher` | String | Optional. CEL expression for matching on L7/application level criteria. |
| `tls_inspection_enabled` | bool | Optional. Flag to enable TLS inspection of traffic matching on , can only be true if the parent GatewaySecurityPolicy references a TLSInspectionConfig. |
| `basic_profile` | String | Required. Profile which tells what the primitive action should be. |
| `create_time` | String | Output only. Time when the rule was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rule
rule = provider.networksecurity_api.Rule {
    parent = "value"  # Required. The parent where this rule will be created. Format : projects/{project}/location/{location}/gatewaySecurityPolicies/*
}

# Access rule outputs
rule_id = rule.id
rule_session_matcher = rule.session_matcher
rule_update_time = rule.update_time
rule_enabled = rule.enabled
rule_name = rule.name
rule_description = rule.description
rule_priority = rule.priority
rule_application_matcher = rule.application_matcher
rule_tls_inspection_enabled = rule.tls_inspection_enabled
rule_basic_profile = rule.basic_profile
rule_create_time = rule.create_time
```

---


### Mirroring_deployment_group

Creates a deployment group in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `nested_deployments` | Vec<String> |  | Output only. The list of Mirroring Deployments that belong to this group. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `description` | String |  | Optional. User-provided description of the deployment group. Used as additional context for the deployment group. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new deployment to the group) See https://google.aip.dev/128. |
| `state` | String |  | Output only. The current state of the deployment group. See https://google.aip.dev/216. |
| `name` | String |  | Immutable. Identifier. The resource name of this deployment group, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/122 for more details. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `locations` | Vec<String> |  | Output only. The list of locations where the deployment group is present. |
| `network` | String |  | Required. Immutable. The network that will be used for all child deployments, for example: `projects/{project}/global/networks/{network}`. See https://google.aip.dev/124. |
| `connected_endpoint_groups` | Vec<String> |  | Output only. The list of endpoint groups that are connected to this resource. |
| `parent` | String | ✅ | Required. The parent resource where this deployment group will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `nested_deployments` | Vec<String> | Output only. The list of Mirroring Deployments that belong to this group. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `description` | String | Optional. User-provided description of the deployment group. Used as additional context for the deployment group. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new deployment to the group) See https://google.aip.dev/128. |
| `state` | String | Output only. The current state of the deployment group. See https://google.aip.dev/216. |
| `name` | String | Immutable. Identifier. The resource name of this deployment group, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/122 for more details. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `locations` | Vec<String> | Output only. The list of locations where the deployment group is present. |
| `network` | String | Required. Immutable. The network that will be used for all child deployments, for example: `projects/{project}/global/networks/{network}`. See https://google.aip.dev/124. |
| `connected_endpoint_groups` | Vec<String> | Output only. The list of endpoint groups that are connected to this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mirroring_deployment_group
mirroring_deployment_group = provider.networksecurity_api.Mirroring_deployment_group {
    parent = "value"  # Required. The parent resource where this deployment group will be created. Format: projects/{project}/locations/{location}
}

# Access mirroring_deployment_group outputs
mirroring_deployment_group_id = mirroring_deployment_group.id
mirroring_deployment_group_nested_deployments = mirroring_deployment_group.nested_deployments
mirroring_deployment_group_labels = mirroring_deployment_group.labels
mirroring_deployment_group_create_time = mirroring_deployment_group.create_time
mirroring_deployment_group_description = mirroring_deployment_group.description
mirroring_deployment_group_reconciling = mirroring_deployment_group.reconciling
mirroring_deployment_group_state = mirroring_deployment_group.state
mirroring_deployment_group_name = mirroring_deployment_group.name
mirroring_deployment_group_update_time = mirroring_deployment_group.update_time
mirroring_deployment_group_locations = mirroring_deployment_group.locations
mirroring_deployment_group_network = mirroring_deployment_group.network
mirroring_deployment_group_connected_endpoint_groups = mirroring_deployment_group.connected_endpoint_groups
```

---


### Mirroring_endpoint_group_association

Creates an association in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. adding a new location to the target deployment group). See https://google.aip.dev/128. |
| `locations` | Vec<String> |  | Output only. The list of locations where the association is configured. This information is retrieved from the linked endpoint group. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `network` | String |  | Immutable. The VPC network that is associated. for example: `projects/123456789/global/networks/my-network`. See https://google.aip.dev/124. |
| `mirroring_endpoint_group` | String |  | Immutable. The endpoint group that this association is connected to, for example: `projects/123456789/locations/global/mirroringEndpointGroups/my-eg`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `name` | String |  | Immutable. Identifier. The resource name of this endpoint group association, for example: `projects/123456789/locations/global/mirroringEndpointGroupAssociations/my-eg-association`. See https://google.aip.dev/122 for more details. |
| `state` | String |  | Output only. Current state of the endpoint group association. |
| `locations_details` | Vec<String> |  | Output only. The list of locations where the association is present. This information is retrieved from the linked endpoint group, and not configured as part of the association itself. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `parent` | String | ✅ | Required. The parent resource where this association will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. adding a new location to the target deployment group). See https://google.aip.dev/128. |
| `locations` | Vec<String> | Output only. The list of locations where the association is configured. This information is retrieved from the linked endpoint group. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `network` | String | Immutable. The VPC network that is associated. for example: `projects/123456789/global/networks/my-network`. See https://google.aip.dev/124. |
| `mirroring_endpoint_group` | String | Immutable. The endpoint group that this association is connected to, for example: `projects/123456789/locations/global/mirroringEndpointGroups/my-eg`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `name` | String | Immutable. Identifier. The resource name of this endpoint group association, for example: `projects/123456789/locations/global/mirroringEndpointGroupAssociations/my-eg-association`. See https://google.aip.dev/122 for more details. |
| `state` | String | Output only. Current state of the endpoint group association. |
| `locations_details` | Vec<String> | Output only. The list of locations where the association is present. This information is retrieved from the linked endpoint group, and not configured as part of the association itself. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mirroring_endpoint_group_association
mirroring_endpoint_group_association = provider.networksecurity_api.Mirroring_endpoint_group_association {
    parent = "value"  # Required. The parent resource where this association will be created. Format: projects/{project}/locations/{location}
}

# Access mirroring_endpoint_group_association outputs
mirroring_endpoint_group_association_id = mirroring_endpoint_group_association.id
mirroring_endpoint_group_association_reconciling = mirroring_endpoint_group_association.reconciling
mirroring_endpoint_group_association_locations = mirroring_endpoint_group_association.locations
mirroring_endpoint_group_association_create_time = mirroring_endpoint_group_association.create_time
mirroring_endpoint_group_association_network = mirroring_endpoint_group_association.network
mirroring_endpoint_group_association_mirroring_endpoint_group = mirroring_endpoint_group_association.mirroring_endpoint_group
mirroring_endpoint_group_association_labels = mirroring_endpoint_group_association.labels
mirroring_endpoint_group_association_name = mirroring_endpoint_group_association.name
mirroring_endpoint_group_association_state = mirroring_endpoint_group_association.state
mirroring_endpoint_group_association_locations_details = mirroring_endpoint_group_association.locations_details
mirroring_endpoint_group_association_update_time = mirroring_endpoint_group_association.update_time
```

---


### Authz_policie

Creates a new AuthzPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String |  | Required. Can be one of `ALLOW`, `DENY`, `CUSTOM`. When the action is `CUSTOM`, `customProvider` must be specified. When the action is `ALLOW`, only requests matching the policy will be allowed. When the action is `DENY`, only requests matching the policy will be denied. When a request arrives, the policies are evaluated in the following order: 1. If there is a `CUSTOM` policy that matches the request, the `CUSTOM` policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request. 2. If there are any `DENY` policies that match the request, the request is denied. 3. If there are no `ALLOW` policies for the resource or if any of the `ALLOW` policies match the request, the request is allowed. 4. Else the request is denied by default if none of the configured AuthzPolicies with `ALLOW` action match the request. |
| `http_rules` | Vec<String> |  | Optional. A list of authorization HTTP rules to match against the incoming request. A policy match occurs when at least one HTTP rule matches the request or when no HTTP rules are specified in the policy. At least one HTTP Rule is required for Allow or Deny Action. Limited to 5 rules. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `name` | String |  | Required. Identifier. Name of the `AuthzPolicy` resource in the following format: `projects/{project}/locations/{location}/authzPolicies/{authz_policy}`. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `target` | String |  | Required. Specifies the set of resources to which this policy should be applied to. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `AuthzPolicy` resource. The format must comply with [the following requirements](/compute/docs/labeling-resources#requirements). |
| `custom_provider` | String |  | Optional. Required if the action is `CUSTOM`. Allows delegating authorization decisions to Cloud IAP or to Service Extensions. One of `cloudIap` or `authzExtension` must be specified. |
| `parent` | String | ✅ | Required. The parent resource of the `AuthzPolicy` resource. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action` | String | Required. Can be one of `ALLOW`, `DENY`, `CUSTOM`. When the action is `CUSTOM`, `customProvider` must be specified. When the action is `ALLOW`, only requests matching the policy will be allowed. When the action is `DENY`, only requests matching the policy will be denied. When a request arrives, the policies are evaluated in the following order: 1. If there is a `CUSTOM` policy that matches the request, the `CUSTOM` policy is evaluated using the custom authorization providers and the request is denied if the provider rejects the request. 2. If there are any `DENY` policies that match the request, the request is denied. 3. If there are no `ALLOW` policies for the resource or if any of the `ALLOW` policies match the request, the request is allowed. 4. Else the request is denied by default if none of the configured AuthzPolicies with `ALLOW` action match the request. |
| `http_rules` | Vec<String> | Optional. A list of authorization HTTP rules to match against the incoming request. A policy match occurs when at least one HTTP rule matches the request or when no HTTP rules are specified in the policy. At least one HTTP Rule is required for Allow or Deny Action. Limited to 5 rules. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `name` | String | Required. Identifier. Name of the `AuthzPolicy` resource in the following format: `projects/{project}/locations/{location}/authzPolicies/{authz_policy}`. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `target` | String | Required. Specifies the set of resources to which this policy should be applied to. |
| `description` | String | Optional. A human-readable description of the resource. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `AuthzPolicy` resource. The format must comply with [the following requirements](/compute/docs/labeling-resources#requirements). |
| `custom_provider` | String | Optional. Required if the action is `CUSTOM`. Allows delegating authorization decisions to Cloud IAP or to Service Extensions. One of `cloudIap` or `authzExtension` must be specified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authz_policie
authz_policie = provider.networksecurity_api.Authz_policie {
    parent = "value"  # Required. The parent resource of the `AuthzPolicy` resource. Must be in the format `projects/{project}/locations/{location}`.
}

# Access authz_policie outputs
authz_policie_id = authz_policie.id
authz_policie_action = authz_policie.action
authz_policie_http_rules = authz_policie.http_rules
authz_policie_create_time = authz_policie.create_time
authz_policie_name = authz_policie.name
authz_policie_update_time = authz_policie.update_time
authz_policie_target = authz_policie.target
authz_policie_description = authz_policie.description
authz_policie_labels = authz_policie.labels
authz_policie_custom_provider = authz_policie.custom_provider
```

---


### Sac_attachment

Creates a new SACAttachment in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Timestamp when the attachment was last updated. |
| `sac_realm` | String |  | Required. SAC Realm which owns the attachment. This can be input as an ID or a full resource name. The output always has the form `projects/{project_number}/locations/{location}/sacRealms/{sac_realm}`. |
| `ncc_gateway` | String |  | Required. NCC Gateway associated with the attachment. This can be input as an ID or a full resource name. The output always has the form `projects/{project_number}/locations/{location}/spokes/{ncc_gateway}`. |
| `country` | String |  | Optional. Case-insensitive ISO-3166 alpha-2 country code used for localization. Only valid for Symantec attachments. |
| `symantec_options` | String |  | Optional. Configuration required for Symantec attachments. |
| `state` | String |  | Output only. State of the attachment. |
| `name` | String |  | Identifier. Resource name, in the form `projects/{project}/locations/{location}/sacAttachments/{sac_attachment}`. |
| `time_zone` | String |  | Optional. Case-sensitive tzinfo identifier used for localization. Only valid for Symantec attachments. |
| `create_time` | String |  | Output only. Timestamp when the attachment was created. |
| `labels` | HashMap<String, String> |  | Optional. Optional list of labels applied to the resource. |
| `parent` | String | ✅ | Required. The parent, in the form `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Timestamp when the attachment was last updated. |
| `sac_realm` | String | Required. SAC Realm which owns the attachment. This can be input as an ID or a full resource name. The output always has the form `projects/{project_number}/locations/{location}/sacRealms/{sac_realm}`. |
| `ncc_gateway` | String | Required. NCC Gateway associated with the attachment. This can be input as an ID or a full resource name. The output always has the form `projects/{project_number}/locations/{location}/spokes/{ncc_gateway}`. |
| `country` | String | Optional. Case-insensitive ISO-3166 alpha-2 country code used for localization. Only valid for Symantec attachments. |
| `symantec_options` | String | Optional. Configuration required for Symantec attachments. |
| `state` | String | Output only. State of the attachment. |
| `name` | String | Identifier. Resource name, in the form `projects/{project}/locations/{location}/sacAttachments/{sac_attachment}`. |
| `time_zone` | String | Optional. Case-sensitive tzinfo identifier used for localization. Only valid for Symantec attachments. |
| `create_time` | String | Output only. Timestamp when the attachment was created. |
| `labels` | HashMap<String, String> | Optional. Optional list of labels applied to the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sac_attachment
sac_attachment = provider.networksecurity_api.Sac_attachment {
    parent = "value"  # Required. The parent, in the form `projects/{project}/locations/{location}`.
}

# Access sac_attachment outputs
sac_attachment_id = sac_attachment.id
sac_attachment_update_time = sac_attachment.update_time
sac_attachment_sac_realm = sac_attachment.sac_realm
sac_attachment_ncc_gateway = sac_attachment.ncc_gateway
sac_attachment_country = sac_attachment.country
sac_attachment_symantec_options = sac_attachment.symantec_options
sac_attachment_state = sac_attachment.state
sac_attachment_name = sac_attachment.name
sac_attachment_time_zone = sac_attachment.time_zone
sac_attachment_create_time = sac_attachment.create_time
sac_attachment_labels = sac_attachment.labels
```

---


### Intercept_endpoint_group

Creates an endpoint group in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new association to the group). See https://google.aip.dev/128. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `connected_deployment_group` | String |  | Output only. Details about the connected deployment group to this endpoint group. |
| `state` | String |  | Output only. The current state of the endpoint group. See https://google.aip.dev/216. |
| `intercept_deployment_group` | String |  | Required. Immutable. The deployment group that this endpoint group is connected to, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `associations` | Vec<String> |  | Output only. List of associations to this endpoint group. |
| `description` | String |  | Optional. User-provided description of the endpoint group. Used as additional context for the endpoint group. |
| `name` | String |  | Immutable. Identifier. The resource name of this endpoint group, for example: `projects/123456789/locations/global/interceptEndpointGroups/my-eg`. See https://google.aip.dev/122 for more details. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `parent` | String | ✅ | Required. The parent resource where this endpoint group will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This is part of the normal operation (e.g. adding a new association to the group). See https://google.aip.dev/128. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `connected_deployment_group` | String | Output only. Details about the connected deployment group to this endpoint group. |
| `state` | String | Output only. The current state of the endpoint group. See https://google.aip.dev/216. |
| `intercept_deployment_group` | String | Required. Immutable. The deployment group that this endpoint group is connected to, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `associations` | Vec<String> | Output only. List of associations to this endpoint group. |
| `description` | String | Optional. User-provided description of the endpoint group. Used as additional context for the endpoint group. |
| `name` | String | Immutable. Identifier. The resource name of this endpoint group, for example: `projects/123456789/locations/global/interceptEndpointGroups/my-eg`. See https://google.aip.dev/122 for more details. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intercept_endpoint_group
intercept_endpoint_group = provider.networksecurity_api.Intercept_endpoint_group {
    parent = "value"  # Required. The parent resource where this endpoint group will be created. Format: projects/{project}/locations/{location}
}

# Access intercept_endpoint_group outputs
intercept_endpoint_group_id = intercept_endpoint_group.id
intercept_endpoint_group_reconciling = intercept_endpoint_group.reconciling
intercept_endpoint_group_update_time = intercept_endpoint_group.update_time
intercept_endpoint_group_connected_deployment_group = intercept_endpoint_group.connected_deployment_group
intercept_endpoint_group_state = intercept_endpoint_group.state
intercept_endpoint_group_intercept_deployment_group = intercept_endpoint_group.intercept_deployment_group
intercept_endpoint_group_create_time = intercept_endpoint_group.create_time
intercept_endpoint_group_associations = intercept_endpoint_group.associations
intercept_endpoint_group_description = intercept_endpoint_group.description
intercept_endpoint_group_name = intercept_endpoint_group.name
intercept_endpoint_group_labels = intercept_endpoint_group.labels
```

---


### Intercept_endpoint_group_association

Creates an association in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. Current state of the endpoint group association. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. adding a new location to the target deployment group). See https://google.aip.dev/128. |
| `intercept_endpoint_group` | String |  | Required. Immutable. The endpoint group that this association is connected to, for example: `projects/123456789/locations/global/interceptEndpointGroups/my-eg`. See https://google.aip.dev/124. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `name` | String |  | Immutable. Identifier. The resource name of this endpoint group association, for example: `projects/123456789/locations/global/interceptEndpointGroupAssociations/my-eg-association`. See https://google.aip.dev/122 for more details. |
| `locations_details` | Vec<String> |  | Output only. The list of locations where the association is present. This information is retrieved from the linked endpoint group, and not configured as part of the association itself. |
| `locations` | Vec<String> |  | Output only. The list of locations where the association is configured. This information is retrieved from the linked endpoint group. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `network` | String |  | Required. Immutable. The VPC network that is associated. for example: `projects/123456789/global/networks/my-network`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `parent` | String | ✅ | Required. The parent resource where this association will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. Current state of the endpoint group association. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. adding a new location to the target deployment group). See https://google.aip.dev/128. |
| `intercept_endpoint_group` | String | Required. Immutable. The endpoint group that this association is connected to, for example: `projects/123456789/locations/global/interceptEndpointGroups/my-eg`. See https://google.aip.dev/124. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `name` | String | Immutable. Identifier. The resource name of this endpoint group association, for example: `projects/123456789/locations/global/interceptEndpointGroupAssociations/my-eg-association`. See https://google.aip.dev/122 for more details. |
| `locations_details` | Vec<String> | Output only. The list of locations where the association is present. This information is retrieved from the linked endpoint group, and not configured as part of the association itself. |
| `locations` | Vec<String> | Output only. The list of locations where the association is configured. This information is retrieved from the linked endpoint group. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `network` | String | Required. Immutable. The VPC network that is associated. for example: `projects/123456789/global/networks/my-network`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intercept_endpoint_group_association
intercept_endpoint_group_association = provider.networksecurity_api.Intercept_endpoint_group_association {
    parent = "value"  # Required. The parent resource where this association will be created. Format: projects/{project}/locations/{location}
}

# Access intercept_endpoint_group_association outputs
intercept_endpoint_group_association_id = intercept_endpoint_group_association.id
intercept_endpoint_group_association_state = intercept_endpoint_group_association.state
intercept_endpoint_group_association_reconciling = intercept_endpoint_group_association.reconciling
intercept_endpoint_group_association_intercept_endpoint_group = intercept_endpoint_group_association.intercept_endpoint_group
intercept_endpoint_group_association_update_time = intercept_endpoint_group_association.update_time
intercept_endpoint_group_association_name = intercept_endpoint_group_association.name
intercept_endpoint_group_association_locations_details = intercept_endpoint_group_association.locations_details
intercept_endpoint_group_association_locations = intercept_endpoint_group_association.locations
intercept_endpoint_group_association_create_time = intercept_endpoint_group_association.create_time
intercept_endpoint_group_association_network = intercept_endpoint_group_association.network
intercept_endpoint_group_association_labels = intercept_endpoint_group_association.labels
```

---


### Mirroring_deployment

Creates a deployment in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mirroring_deployment_group` | String |  | Required. Immutable. The deployment group that this deployment is a part of, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. linking a new association to the parent group). See https://google.aip.dev/128. |
| `description` | String |  | Optional. User-provided description of the deployment. Used as additional context for the deployment. |
| `state` | String |  | Output only. The current state of the deployment. See https://google.aip.dev/216. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `forwarding_rule` | String |  | Required. Immutable. The regional forwarding rule that fronts the mirroring collectors, for example: `projects/123456789/regions/us-central1/forwardingRules/my-rule`. See https://google.aip.dev/124. |
| `name` | String |  | Immutable. Identifier. The resource name of this deployment, for example: `projects/123456789/locations/us-central1-a/mirroringDeployments/my-dep`. See https://google.aip.dev/122 for more details. |
| `parent` | String | ✅ | Required. The parent resource where this deployment will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mirroring_deployment_group` | String | Required. Immutable. The deployment group that this deployment is a part of, for example: `projects/123456789/locations/global/mirroringDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. linking a new association to the parent group). See https://google.aip.dev/128. |
| `description` | String | Optional. User-provided description of the deployment. Used as additional context for the deployment. |
| `state` | String | Output only. The current state of the deployment. See https://google.aip.dev/216. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `forwarding_rule` | String | Required. Immutable. The regional forwarding rule that fronts the mirroring collectors, for example: `projects/123456789/regions/us-central1/forwardingRules/my-rule`. See https://google.aip.dev/124. |
| `name` | String | Immutable. Identifier. The resource name of this deployment, for example: `projects/123456789/locations/us-central1-a/mirroringDeployments/my-dep`. See https://google.aip.dev/122 for more details. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mirroring_deployment
mirroring_deployment = provider.networksecurity_api.Mirroring_deployment {
    parent = "value"  # Required. The parent resource where this deployment will be created. Format: projects/{project}/locations/{location}
}

# Access mirroring_deployment outputs
mirroring_deployment_id = mirroring_deployment.id
mirroring_deployment_mirroring_deployment_group = mirroring_deployment.mirroring_deployment_group
mirroring_deployment_reconciling = mirroring_deployment.reconciling
mirroring_deployment_description = mirroring_deployment.description
mirroring_deployment_state = mirroring_deployment.state
mirroring_deployment_update_time = mirroring_deployment.update_time
mirroring_deployment_create_time = mirroring_deployment.create_time
mirroring_deployment_labels = mirroring_deployment.labels
mirroring_deployment_forwarding_rule = mirroring_deployment.forwarding_rule
mirroring_deployment_name = mirroring_deployment.name
```

---


### Url_list

Creates a new UrlList in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time when the security policy was created. |
| `values` | Vec<String> |  | Required. FQDNs and URLs. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `name` | String |  | Required. Name of the resource provided by the user. Name is of the form projects/{project}/locations/{location}/urlLists/{url_list} url_list should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `update_time` | String |  | Output only. Time when the security policy was updated. |
| `parent` | String | ✅ | Required. The parent resource of the UrlList. Must be in the format `projects/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time when the security policy was created. |
| `values` | Vec<String> | Required. FQDNs and URLs. |
| `description` | String | Optional. Free-text description of the resource. |
| `name` | String | Required. Name of the resource provided by the user. Name is of the form projects/{project}/locations/{location}/urlLists/{url_list} url_list should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `update_time` | String | Output only. Time when the security policy was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create url_list
url_list = provider.networksecurity_api.Url_list {
    parent = "value"  # Required. The parent resource of the UrlList. Must be in the format `projects/*/locations/{location}`.
}

# Access url_list outputs
url_list_id = url_list.id
url_list_create_time = url_list.create_time
url_list_values = url_list.values
url_list_description = url_list.description
url_list_name = url_list.name
url_list_update_time = url_list.update_time
```

---


### Tls_inspection_policie

Creates a new TlsInspectionPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `trust_config` | String |  | Optional. A TrustConfig resource used when making a connection to the TLS server. This is a relative resource path following the form "projects/{project}/locations/{location}/trustConfigs/{trust_config}". This is necessary to intercept TLS connections to servers with certificates signed by a private CA or self-signed certificates. Note that Secure Web Proxy does not yet honor this field. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `tls_feature_profile` | String |  | Optional. The selected Profile. If this is not set, then the default value is to allow the broadest set of clients and servers ("PROFILE_COMPATIBLE"). Setting this to more restrictive values may improve security, but may also prevent the TLS inspection proxy from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `min_tls_version` | String |  | Optional. Minimum TLS version that the firewall should use when negotiating connections with both clients and servers. If this is not set, then the default value is to allow the broadest set of clients and servers (TLS 1.0 or higher). Setting this to more restrictive values may improve security, but may also prevent the firewall from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `name` | String |  | Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/tlsInspectionPolicies/{tls_inspection_policy} tls_inspection_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `ca_pool` | String |  | Required. A CA pool resource used to issue interception certificates. The CA pool string has a relative resource path following the form "projects/{project}/locations/{location}/caPools/{ca_pool}". |
| `exclude_public_ca_set` | bool |  | Optional. If FALSE (the default), use our default set of public CAs in addition to any CAs specified in trust_config. These public CAs are currently based on the Mozilla Root Program and are subject to change over time. If TRUE, do not accept our default set of public CAs. Only CAs specified in trust_config will be accepted. This defaults to FALSE (use public CAs in addition to trust_config) for backwards compatibility, but trusting public root CAs is *not recommended* unless the traffic in question is outbound to public web servers. When possible, prefer setting this to "false" and explicitly specifying trusted CAs and certificates in a TrustConfig. Note that Secure Web Proxy does not yet honor this field. |
| `custom_tls_features` | Vec<String> |  | Optional. List of custom TLS cipher suites selected. This field is valid only if the selected tls_feature_profile is CUSTOM. The compute.SslPoliciesService.ListAvailableFeatures method returns the set of features that can be specified in this list. Note that Secure Web Proxy does not yet honor this field. |
| `parent` | String | ✅ | Required. The parent resource of the TlsInspectionPolicy. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `trust_config` | String | Optional. A TrustConfig resource used when making a connection to the TLS server. This is a relative resource path following the form "projects/{project}/locations/{location}/trustConfigs/{trust_config}". This is necessary to intercept TLS connections to servers with certificates signed by a private CA or self-signed certificates. Note that Secure Web Proxy does not yet honor this field. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `tls_feature_profile` | String | Optional. The selected Profile. If this is not set, then the default value is to allow the broadest set of clients and servers ("PROFILE_COMPATIBLE"). Setting this to more restrictive values may improve security, but may also prevent the TLS inspection proxy from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `min_tls_version` | String | Optional. Minimum TLS version that the firewall should use when negotiating connections with both clients and servers. If this is not set, then the default value is to allow the broadest set of clients and servers (TLS 1.0 or higher). Setting this to more restrictive values may improve security, but may also prevent the firewall from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field. |
| `description` | String | Optional. Free-text description of the resource. |
| `name` | String | Required. Name of the resource. Name is of the form projects/{project}/locations/{location}/tlsInspectionPolicies/{tls_inspection_policy} tls_inspection_policy should match the pattern:(^[a-z]([a-z0-9-]{0,61}[a-z0-9])?$). |
| `ca_pool` | String | Required. A CA pool resource used to issue interception certificates. The CA pool string has a relative resource path following the form "projects/{project}/locations/{location}/caPools/{ca_pool}". |
| `exclude_public_ca_set` | bool | Optional. If FALSE (the default), use our default set of public CAs in addition to any CAs specified in trust_config. These public CAs are currently based on the Mozilla Root Program and are subject to change over time. If TRUE, do not accept our default set of public CAs. Only CAs specified in trust_config will be accepted. This defaults to FALSE (use public CAs in addition to trust_config) for backwards compatibility, but trusting public root CAs is *not recommended* unless the traffic in question is outbound to public web servers. When possible, prefer setting this to "false" and explicitly specifying trusted CAs and certificates in a TrustConfig. Note that Secure Web Proxy does not yet honor this field. |
| `custom_tls_features` | Vec<String> | Optional. List of custom TLS cipher suites selected. This field is valid only if the selected tls_feature_profile is CUSTOM. The compute.SslPoliciesService.ListAvailableFeatures method returns the set of features that can be specified in this list. Note that Secure Web Proxy does not yet honor this field. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tls_inspection_policie
tls_inspection_policie = provider.networksecurity_api.Tls_inspection_policie {
    parent = "value"  # Required. The parent resource of the TlsInspectionPolicy. Must be in the format `projects/{project}/locations/{location}`.
}

# Access tls_inspection_policie outputs
tls_inspection_policie_id = tls_inspection_policie.id
tls_inspection_policie_trust_config = tls_inspection_policie.trust_config
tls_inspection_policie_create_time = tls_inspection_policie.create_time
tls_inspection_policie_tls_feature_profile = tls_inspection_policie.tls_feature_profile
tls_inspection_policie_update_time = tls_inspection_policie.update_time
tls_inspection_policie_min_tls_version = tls_inspection_policie.min_tls_version
tls_inspection_policie_description = tls_inspection_policie.description
tls_inspection_policie_name = tls_inspection_policie.name
tls_inspection_policie_ca_pool = tls_inspection_policie.ca_pool
tls_inspection_policie_exclude_public_ca_set = tls_inspection_policie.exclude_public_ca_set
tls_inspection_policie_custom_tls_features = tls_inspection_policie.custom_tls_features
```

---


### Intercept_deployment

Creates a deployment in a given project and location. See https://google.aip.dev/133.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The current state of the deployment. See https://google.aip.dev/216. |
| `intercept_deployment_group` | String |  | Required. Immutable. The deployment group that this deployment is a part of, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `reconciling` | bool |  | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. linking a new association to the parent group). See https://google.aip.dev/128. |
| `name` | String |  | Immutable. Identifier. The resource name of this deployment, for example: `projects/123456789/locations/us-central1-a/interceptDeployments/my-dep`. See https://google.aip.dev/122 for more details. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `description` | String |  | Optional. User-provided description of the deployment. Used as additional context for the deployment. |
| `forwarding_rule` | String |  | Required. Immutable. The regional forwarding rule that fronts the interceptors, for example: `projects/123456789/regions/us-central1/forwardingRules/my-rule`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> |  | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `update_time` | String |  | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |
| `parent` | String | ✅ | Required. The parent resource where this deployment will be created. Format: projects/{project}/locations/{location} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The current state of the deployment. See https://google.aip.dev/216. |
| `intercept_deployment_group` | String | Required. Immutable. The deployment group that this deployment is a part of, for example: `projects/123456789/locations/global/interceptDeploymentGroups/my-dg`. See https://google.aip.dev/124. |
| `reconciling` | bool | Output only. The current state of the resource does not match the user's intended state, and the system is working to reconcile them. This part of the normal operation (e.g. linking a new association to the parent group). See https://google.aip.dev/128. |
| `name` | String | Immutable. Identifier. The resource name of this deployment, for example: `projects/123456789/locations/us-central1-a/interceptDeployments/my-dep`. See https://google.aip.dev/122 for more details. |
| `create_time` | String | Output only. The timestamp when the resource was created. See https://google.aip.dev/148#timestamps. |
| `description` | String | Optional. User-provided description of the deployment. Used as additional context for the deployment. |
| `forwarding_rule` | String | Required. Immutable. The regional forwarding rule that fronts the interceptors, for example: `projects/123456789/regions/us-central1/forwardingRules/my-rule`. See https://google.aip.dev/124. |
| `labels` | HashMap<String, String> | Optional. Labels are key/value pairs that help to organize and filter resources. |
| `update_time` | String | Output only. The timestamp when the resource was most recently updated. See https://google.aip.dev/148#timestamps. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create intercept_deployment
intercept_deployment = provider.networksecurity_api.Intercept_deployment {
    parent = "value"  # Required. The parent resource where this deployment will be created. Format: projects/{project}/locations/{location}
}

# Access intercept_deployment outputs
intercept_deployment_id = intercept_deployment.id
intercept_deployment_state = intercept_deployment.state
intercept_deployment_intercept_deployment_group = intercept_deployment.intercept_deployment_group
intercept_deployment_reconciling = intercept_deployment.reconciling
intercept_deployment_name = intercept_deployment.name
intercept_deployment_create_time = intercept_deployment.create_time
intercept_deployment_description = intercept_deployment.description
intercept_deployment_forwarding_rule = intercept_deployment.forwarding_rule
intercept_deployment_labels = intercept_deployment.labels
intercept_deployment_update_time = intercept_deployment.update_time
```

---


### Client_tls_policie

Creates a new ClientTlsPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sni` | String |  | Optional. Server Name Indication string to present to the server during TLS handshake. E.g: "secure.example.com". |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the resource. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `server_validation_ca` | Vec<String> |  | Optional. Defines the mechanism to obtain the Certificate Authority certificate to validate the server certificate. If empty, client does not validate the server certificate. |
| `name` | String |  | Required. Name of the ClientTlsPolicy resource. It matches the pattern `projects/{project}/locations/{location}/clientTlsPolicies/{client_tls_policy}` |
| `description` | String |  | Optional. Free-text description of the resource. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `client_certificate` | String |  | Optional. Defines a mechanism to provision client identity (public and private keys) for peer to peer authentication. The presence of this dictates mTLS. |
| `parent` | String | ✅ | Required. The parent resource of the ClientTlsPolicy. Must be in the format `projects/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sni` | String | Optional. Server Name Indication string to present to the server during TLS handshake. E.g: "secure.example.com". |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the resource. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `server_validation_ca` | Vec<String> | Optional. Defines the mechanism to obtain the Certificate Authority certificate to validate the server certificate. If empty, client does not validate the server certificate. |
| `name` | String | Required. Name of the ClientTlsPolicy resource. It matches the pattern `projects/{project}/locations/{location}/clientTlsPolicies/{client_tls_policy}` |
| `description` | String | Optional. Free-text description of the resource. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `client_certificate` | String | Optional. Defines a mechanism to provision client identity (public and private keys) for peer to peer authentication. The presence of this dictates mTLS. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create client_tls_policie
client_tls_policie = provider.networksecurity_api.Client_tls_policie {
    parent = "value"  # Required. The parent resource of the ClientTlsPolicy. Must be in the format `projects/*/locations/{location}`.
}

# Access client_tls_policie outputs
client_tls_policie_id = client_tls_policie.id
client_tls_policie_sni = client_tls_policie.sni
client_tls_policie_labels = client_tls_policie.labels
client_tls_policie_update_time = client_tls_policie.update_time
client_tls_policie_server_validation_ca = client_tls_policie.server_validation_ca
client_tls_policie_name = client_tls_policie.name
client_tls_policie_description = client_tls_policie.description
client_tls_policie_create_time = client_tls_policie.create_time
client_tls_policie_client_certificate = client_tls_policie.client_certificate
```

---


### Backend_authentication_config

Creates a new BackendAuthenticationConfig in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Output only. Etag of the resource. |
| `description` | String |  | Optional. Free-text description of the resource. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `client_certificate` | String |  | Optional. A reference to a certificatemanager.googleapis.com.Certificate resource. This is a relative resource path following the form "projects/{project}/locations/{location}/certificates/{certificate}". Used by a BackendService to negotiate mTLS when the backend connection uses TLS and the backend requests a client certificate. Must have a CLIENT_AUTH scope. |
| `name` | String |  | Required. Name of the BackendAuthenticationConfig resource. It matches the pattern `projects/*/locations/{location}/backendAuthenticationConfigs/{backend_authentication_config}` |
| `trust_config` | String |  | Optional. A reference to a TrustConfig resource from the certificatemanager.googleapis.com namespace. This is a relative resource path following the form "projects/{project}/locations/{location}/trustConfigs/{trust_config}". A BackendService uses the chain of trust represented by this TrustConfig, if specified, to validate the server certificates presented by the backend. Required unless wellKnownRoots is set to PUBLIC_ROOTS. |
| `labels` | HashMap<String, String> |  | Set of label tags associated with the resource. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `well_known_roots` | String |  | Well known roots to use for server certificate validation. |
| `parent` | String | ✅ | Required. The parent resource of the BackendAuthenticationConfig. Must be in the format `projects/*/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Output only. Etag of the resource. |
| `description` | String | Optional. Free-text description of the resource. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `client_certificate` | String | Optional. A reference to a certificatemanager.googleapis.com.Certificate resource. This is a relative resource path following the form "projects/{project}/locations/{location}/certificates/{certificate}". Used by a BackendService to negotiate mTLS when the backend connection uses TLS and the backend requests a client certificate. Must have a CLIENT_AUTH scope. |
| `name` | String | Required. Name of the BackendAuthenticationConfig resource. It matches the pattern `projects/*/locations/{location}/backendAuthenticationConfigs/{backend_authentication_config}` |
| `trust_config` | String | Optional. A reference to a TrustConfig resource from the certificatemanager.googleapis.com namespace. This is a relative resource path following the form "projects/{project}/locations/{location}/trustConfigs/{trust_config}". A BackendService uses the chain of trust represented by this TrustConfig, if specified, to validate the server certificates presented by the backend. Required unless wellKnownRoots is set to PUBLIC_ROOTS. |
| `labels` | HashMap<String, String> | Set of label tags associated with the resource. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `well_known_roots` | String | Well known roots to use for server certificate validation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create backend_authentication_config
backend_authentication_config = provider.networksecurity_api.Backend_authentication_config {
    parent = "value"  # Required. The parent resource of the BackendAuthenticationConfig. Must be in the format `projects/*/locations/{location}`.
}

# Access backend_authentication_config outputs
backend_authentication_config_id = backend_authentication_config.id
backend_authentication_config_etag = backend_authentication_config.etag
backend_authentication_config_description = backend_authentication_config.description
backend_authentication_config_create_time = backend_authentication_config.create_time
backend_authentication_config_client_certificate = backend_authentication_config.client_certificate
backend_authentication_config_name = backend_authentication_config.name
backend_authentication_config_trust_config = backend_authentication_config.trust_config
backend_authentication_config_labels = backend_authentication_config.labels
backend_authentication_config_update_time = backend_authentication_config.update_time
backend_authentication_config_well_known_roots = backend_authentication_config.well_known_roots
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple security_profile_group resources
security_profile_group_0 = provider.networksecurity_api.Security_profile_group {
    parent = "value-0"
}
security_profile_group_1 = provider.networksecurity_api.Security_profile_group {
    parent = "value-1"
}
security_profile_group_2 = provider.networksecurity_api.Security_profile_group {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    security_profile_group = provider.networksecurity_api.Security_profile_group {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Networksecurity_api Documentation](https://cloud.google.com/networksecurity_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
