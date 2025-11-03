# Gkehub_api Service



**Resources**: 42

---

## Overview

The gkehub_api service provides access to 42 resource types:

- [Operation](#operation) [CRD]
- [Rbacrolebinding](#rbacrolebinding) [CRUD]
- [Feature](#feature) [CRUD]
- [Membership](#membership) [CRUD]
- [Fleet](#fleet) [CRUD]
- [Location](#location) [R]
- [Binding](#binding) [CRUD]
- [Namespace](#namespace) [CRUD]
- [Scope](#scope) [CRUD]
- [Feature](#feature) [CRUD]
- [Membership](#membership) [CRUD]
- [Fleet](#fleet) [CRUD]
- [Rbacrolebinding](#rbacrolebinding) [CRUD]
- [Namespace](#namespace) [CRUD]
- [Scope](#scope) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Binding](#binding) [CRUD]
- [Location](#location) [R]
- [Membership](#membership) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Operation](#operation) [CR]
- [Feature](#feature) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CR]
- [Feature](#feature) [CRUD]
- [Scope](#scope) [CRUD]
- [Fleet](#fleet) [CRUD]
- [Location](#location) [R]
- [Namespace](#namespace) [CRUD]
- [Membership](#membership) [CRUD]
- [Operation](#operation) [CRD]
- [Rbacrolebinding](#rbacrolebinding) [CRUD]
- [Binding](#binding) [CRUD]
- [Feature](#feature) [CRUD]
- [Membership](#membership) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Location](#location) [R]
- [Operation](#operation) [CR]
- [Feature](#feature) [CRUD]

---

## Resources


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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.gkehub_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_error = operation.error
operation_done = operation.done
operation_response = operation.response
operation_metadata = operation.metadata
```

---


### Rbacrolebinding

Creates a Membership RBACRoleBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group` | String |  | group is the group, as seen by the kubernetes cluster. |
| `name` | String |  | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `delete_time` | String |  | Output only. When the rbacrolebinding was deleted. |
| `role` | String |  | Required. Role to bind to the principal |
| `update_time` | String |  | Output only. When the rbacrolebinding was last updated. |
| `user` | String |  | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `state` | String |  | Output only. State of the rbacrolebinding resource. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this RBACRolebinding. |
| `create_time` | String |  | Output only. When the rbacrolebinding was created. |
| `parent` | String | ✅ | Required. The parent (project and location) where the RBACRoleBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group` | String | group is the group, as seen by the kubernetes cluster. |
| `name` | String | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `delete_time` | String | Output only. When the rbacrolebinding was deleted. |
| `role` | String | Required. Role to bind to the principal |
| `update_time` | String | Output only. When the rbacrolebinding was last updated. |
| `user` | String | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `state` | String | Output only. State of the rbacrolebinding resource. |
| `labels` | HashMap<String, String> | Optional. Labels for this RBACRolebinding. |
| `create_time` | String | Output only. When the rbacrolebinding was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rbacrolebinding
rbacrolebinding = provider.gkehub_api.Rbacrolebinding {
    parent = "value"  # Required. The parent (project and location) where the RBACRoleBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`.
}

# Access rbacrolebinding outputs
rbacrolebinding_id = rbacrolebinding.id
rbacrolebinding_group = rbacrolebinding.group
rbacrolebinding_name = rbacrolebinding.name
rbacrolebinding_delete_time = rbacrolebinding.delete_time
rbacrolebinding_role = rbacrolebinding.role
rbacrolebinding_update_time = rbacrolebinding.update_time
rbacrolebinding_user = rbacrolebinding.user
rbacrolebinding_uid = rbacrolebinding.uid
rbacrolebinding_state = rbacrolebinding.state
rbacrolebinding_labels = rbacrolebinding.labels
rbacrolebinding_create_time = rbacrolebinding.create_time
```

---


### Feature

Adds a new Feature.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `membership_states` | HashMap<String, String> |  | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `state` | String |  | Output only. The Fleet-wide Feature state. |
| `delete_time` | String |  | Output only. When the Feature resource was deleted. |
| `unreachable` | Vec<String> |  | Output only. List of locations that could not be reached while fetching this feature. |
| `update_time` | String |  | Output only. When the Feature resource was last updated. |
| `fleet_default_member_config` | String |  | Optional. Feature configuration applicable to all memberships of the fleet. |
| `create_time` | String |  | Output only. When the Feature resource was created. |
| `membership_specs` | HashMap<String, String> |  | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `resource_state` | String |  | Output only. State of the Feature resource itself. |
| `scope_specs` | HashMap<String, String> |  | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `labels` | HashMap<String, String> |  | Labels for this Feature. |
| `scope_states` | HashMap<String, String> |  | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `spec` | String |  | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `name` | String |  | Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Feature will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `membership_states` | HashMap<String, String> | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `state` | String | Output only. The Fleet-wide Feature state. |
| `delete_time` | String | Output only. When the Feature resource was deleted. |
| `unreachable` | Vec<String> | Output only. List of locations that could not be reached while fetching this feature. |
| `update_time` | String | Output only. When the Feature resource was last updated. |
| `fleet_default_member_config` | String | Optional. Feature configuration applicable to all memberships of the fleet. |
| `create_time` | String | Output only. When the Feature resource was created. |
| `membership_specs` | HashMap<String, String> | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `resource_state` | String | Output only. State of the Feature resource itself. |
| `scope_specs` | HashMap<String, String> | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `labels` | HashMap<String, String> | Labels for this Feature. |
| `scope_states` | HashMap<String, String> | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `spec` | String | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `name` | String | Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature
feature = provider.gkehub_api.Feature {
    parent = "value"  # Required. The parent (project and location) where the Feature will be created. Specified in the format `projects/*/locations/*`.
}

# Access feature outputs
feature_id = feature.id
feature_membership_states = feature.membership_states
feature_state = feature.state
feature_delete_time = feature.delete_time
feature_unreachable = feature.unreachable
feature_update_time = feature.update_time
feature_fleet_default_member_config = feature.fleet_default_member_config
feature_create_time = feature.create_time
feature_membership_specs = feature.membership_specs
feature_resource_state = feature.resource_state
feature_scope_specs = feature.scope_specs
feature_labels = feature.labels
feature_scope_states = feature.scope_states
feature_spec = feature.spec
feature_name = feature.name
```

---


### Membership

Creates a new Membership. **This is currently only supported for GKE clusters on Google Cloud**. To register other clusters, follow the instructions at https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. When the Membership was deleted. |
| `authority` | String |  | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `create_time` | String |  | Output only. When the Membership was created. |
| `membership_type` | String |  | Output only. The type of the membership. |
| `description` | String |  | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `endpoint` | String |  | Optional. Endpoint information to reach this member. |
| `last_connection_time` | String |  | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `monitoring_config` | String |  | Optional. The monitoring config information for this membership. |
| `cluster_tier` | String |  | Output only. The tier of the cluster. |
| `name` | String |  | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `state` | String |  | Output only. State of the Membership resource. |
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `update_time` | String |  | Output only. When the Membership was last updated. |
| `external_id` | String |  | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. When the Membership was deleted. |
| `authority` | String | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `create_time` | String | Output only. When the Membership was created. |
| `membership_type` | String | Output only. The type of the membership. |
| `description` | String | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `endpoint` | String | Optional. Endpoint information to reach this member. |
| `last_connection_time` | String | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `monitoring_config` | String | Optional. The monitoring config information for this membership. |
| `cluster_tier` | String | Output only. The tier of the cluster. |
| `name` | String | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `labels` | HashMap<String, String> | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `state` | String | Output only. State of the Membership resource. |
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `update_time` | String | Output only. When the Membership was last updated. |
| `external_id` | String | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create membership
membership = provider.gkehub_api.Membership {
    parent = "value"  # Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`.
}

# Access membership outputs
membership_id = membership.id
membership_delete_time = membership.delete_time
membership_authority = membership.authority
membership_create_time = membership.create_time
membership_membership_type = membership.membership_type
membership_description = membership.description
membership_endpoint = membership.endpoint
membership_last_connection_time = membership.last_connection_time
membership_monitoring_config = membership.monitoring_config
membership_cluster_tier = membership.cluster_tier
membership_name = membership.name
membership_labels = membership.labels
membership_state = membership.state
membership_unique_id = membership.unique_id
membership_update_time = membership.update_time
membership_external_id = membership.external_id
```

---


### Fleet

Creates a fleet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. When the Fleet was deleted. |
| `name` | String |  | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |
| `default_cluster_config` | String |  | Optional. The default cluster configurations to apply across the fleet. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Fleet. |
| `display_name` | String |  | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `state` | String |  | Output only. State of the namespace resource. |
| `create_time` | String |  | Output only. When the Fleet was created. |
| `update_time` | String |  | Output only. When the Fleet was last updated. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Fleet will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. When the Fleet was deleted. |
| `name` | String | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |
| `default_cluster_config` | String | Optional. The default cluster configurations to apply across the fleet. |
| `labels` | HashMap<String, String> | Optional. Labels for this Fleet. |
| `display_name` | String | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `state` | String | Output only. State of the namespace resource. |
| `create_time` | String | Output only. When the Fleet was created. |
| `update_time` | String | Output only. When the Fleet was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create fleet
fleet = provider.gkehub_api.Fleet {
    parent = "value"  # Required. The parent (project and location) where the Fleet will be created. Specified in the format `projects/*/locations/*`.
}

# Access fleet outputs
fleet_id = fleet.id
fleet_delete_time = fleet.delete_time
fleet_name = fleet.name
fleet_default_cluster_config = fleet.default_cluster_config
fleet_labels = fleet.labels
fleet_display_name = fleet.display_name
fleet_uid = fleet.uid
fleet_state = fleet.state
fleet_create_time = fleet.create_time
fleet_update_time = fleet.update_time
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_location_id = location.location_id
location_display_name = location.display_name
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
```

---


### Binding

Creates a MembershipBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this MembershipBinding. |
| `update_time` | String |  | Output only. When the membership binding was last updated. |
| `delete_time` | String |  | Output only. When the membership binding was deleted. |
| `scope` | String |  | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |
| `state` | String |  | Output only. State of the membership binding resource. |
| `create_time` | String |  | Output only. When the membership binding was created. |
| `name` | String |  | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |
| `parent` | String | ✅ | Required. The parent (project and location) where the MembershipBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `labels` | HashMap<String, String> | Optional. Labels for this MembershipBinding. |
| `update_time` | String | Output only. When the membership binding was last updated. |
| `delete_time` | String | Output only. When the membership binding was deleted. |
| `scope` | String | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |
| `state` | String | Output only. State of the membership binding resource. |
| `create_time` | String | Output only. When the membership binding was created. |
| `name` | String | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create binding
binding = provider.gkehub_api.Binding {
    parent = "value"  # Required. The parent (project and location) where the MembershipBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`.
}

# Access binding outputs
binding_id = binding.id
binding_uid = binding.uid
binding_labels = binding.labels
binding_update_time = binding.update_time
binding_delete_time = binding.delete_time
binding_scope = binding.scope
binding_state = binding.state
binding_create_time = binding.create_time
binding_name = binding.name
```

---


### Namespace

Creates a fleet namespace.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `create_time` | String |  | Output only. When the namespace was created. |
| `state` | String |  | Output only. State of the namespace resource. |
| `update_time` | String |  | Output only. When the namespace was last updated. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Namespace. |
| `delete_time` | String |  | Output only. When the namespace was deleted. |
| `namespace_labels` | HashMap<String, String> |  | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `scope` | String |  | Required. Scope associated with the namespace |
| `parent` | String | ✅ | Required. The parent (project and location) where the Namespace will be created. Specified in the format `projects/*/locations/*/scopes/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `create_time` | String | Output only. When the namespace was created. |
| `state` | String | Output only. State of the namespace resource. |
| `update_time` | String | Output only. When the namespace was last updated. |
| `labels` | HashMap<String, String> | Optional. Labels for this Namespace. |
| `delete_time` | String | Output only. When the namespace was deleted. |
| `namespace_labels` | HashMap<String, String> | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `scope` | String | Required. Scope associated with the namespace |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create namespace
namespace = provider.gkehub_api.Namespace {
    parent = "value"  # Required. The parent (project and location) where the Namespace will be created. Specified in the format `projects/*/locations/*/scopes/*`.
}

# Access namespace outputs
namespace_id = namespace.id
namespace_name = namespace.name
namespace_uid = namespace.uid
namespace_create_time = namespace.create_time
namespace_state = namespace.state
namespace_update_time = namespace.update_time
namespace_labels = namespace.labels
namespace_delete_time = namespace.delete_time
namespace_namespace_labels = namespace.namespace_labels
namespace_scope = namespace.scope
```

---


### Scope

Creates a Scope.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `update_time` | String |  | Output only. When the scope was last updated. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |
| `state` | String |  | Output only. State of the scope resource. |
| `delete_time` | String |  | Output only. When the scope was deleted. |
| `create_time` | String |  | Output only. When the scope was created. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Scope. |
| `namespace_labels` | HashMap<String, String> |  | Optional. Scope-level cluster namespace labels. For the member clusters bound to the Scope, these labels are applied to each namespace under the Scope. Scope-level labels take precedence over Namespace-level labels (`namespace_labels` in the Fleet Namespace resource) if they share a key. Keys and values must be Kubernetes-conformant. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Scope will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `update_time` | String | Output only. When the scope was last updated. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |
| `state` | String | Output only. State of the scope resource. |
| `delete_time` | String | Output only. When the scope was deleted. |
| `create_time` | String | Output only. When the scope was created. |
| `labels` | HashMap<String, String> | Optional. Labels for this Scope. |
| `namespace_labels` | HashMap<String, String> | Optional. Scope-level cluster namespace labels. For the member clusters bound to the Scope, these labels are applied to each namespace under the Scope. Scope-level labels take precedence over Namespace-level labels (`namespace_labels` in the Fleet Namespace resource) if they share a key. Keys and values must be Kubernetes-conformant. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create scope
scope = provider.gkehub_api.Scope {
    parent = "value"  # Required. The parent (project and location) where the Scope will be created. Specified in the format `projects/*/locations/*`.
}

# Access scope outputs
scope_id = scope.id
scope_name = scope.name
scope_update_time = scope.update_time
scope_uid = scope.uid
scope_state = scope.state
scope_delete_time = scope.delete_time
scope_create_time = scope.create_time
scope_labels = scope.labels
scope_namespace_labels = scope.namespace_labels
```

---


### Feature

Adds a new Feature.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_state` | String |  | Output only. State of the Feature resource itself. |
| `scope_states` | HashMap<String, String> |  | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `unreachable` | Vec<String> |  | Output only. List of locations that could not be reached while fetching this feature. |
| `state` | String |  | Output only. The Fleet-wide Feature state. |
| `spec` | String |  | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `membership_specs` | HashMap<String, String> |  | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `fleet_default_member_config` | String |  | Optional. Feature configuration applicable to all memberships of the fleet. |
| `update_time` | String |  | Output only. When the Feature resource was last updated. |
| `scope_specs` | HashMap<String, String> |  | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `create_time` | String |  | Output only. When the Feature resource was created. |
| `delete_time` | String |  | Output only. When the Feature resource was deleted. |
| `labels` | HashMap<String, String> |  | Labels for this Feature. |
| `membership_states` | HashMap<String, String> |  | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `name` | String |  | Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Feature will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_state` | String | Output only. State of the Feature resource itself. |
| `scope_states` | HashMap<String, String> | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `unreachable` | Vec<String> | Output only. List of locations that could not be reached while fetching this feature. |
| `state` | String | Output only. The Fleet-wide Feature state. |
| `spec` | String | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `membership_specs` | HashMap<String, String> | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `fleet_default_member_config` | String | Optional. Feature configuration applicable to all memberships of the fleet. |
| `update_time` | String | Output only. When the Feature resource was last updated. |
| `scope_specs` | HashMap<String, String> | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `create_time` | String | Output only. When the Feature resource was created. |
| `delete_time` | String | Output only. When the Feature resource was deleted. |
| `labels` | HashMap<String, String> | Labels for this Feature. |
| `membership_states` | HashMap<String, String> | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `name` | String | Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature
feature = provider.gkehub_api.Feature {
    parent = "value"  # Required. The parent (project and location) where the Feature will be created. Specified in the format `projects/*/locations/*`.
}

# Access feature outputs
feature_id = feature.id
feature_resource_state = feature.resource_state
feature_scope_states = feature.scope_states
feature_unreachable = feature.unreachable
feature_state = feature.state
feature_spec = feature.spec
feature_membership_specs = feature.membership_specs
feature_fleet_default_member_config = feature.fleet_default_member_config
feature_update_time = feature.update_time
feature_scope_specs = feature.scope_specs
feature_create_time = feature.create_time
feature_delete_time = feature.delete_time
feature_labels = feature.labels
feature_membership_states = feature.membership_states
feature_name = feature.name
```

---


### Membership

Creates a new Membership. **This is currently only supported for GKE clusters on Google Cloud**. To register other clusters, follow the instructions at https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `description` | String |  | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `name` | String |  | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `delete_time` | String |  | Output only. When the Membership was deleted. |
| `membership_type` | String |  | Output only. The type of the membership. |
| `last_connection_time` | String |  | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `cluster_tier` | String |  | Output only. The tier of the cluster. |
| `endpoint` | String |  | Optional. Endpoint information to reach this member. |
| `external_id` | String |  | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `update_time` | String |  | Output only. When the Membership was last updated. |
| `authority` | String |  | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `create_time` | String |  | Output only. When the Membership was created. |
| `monitoring_config` | String |  | Optional. The monitoring config information for this membership. |
| `state` | String |  | Output only. State of the Membership resource. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `description` | String | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `labels` | HashMap<String, String> | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `name` | String | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `delete_time` | String | Output only. When the Membership was deleted. |
| `membership_type` | String | Output only. The type of the membership. |
| `last_connection_time` | String | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `cluster_tier` | String | Output only. The tier of the cluster. |
| `endpoint` | String | Optional. Endpoint information to reach this member. |
| `external_id` | String | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `update_time` | String | Output only. When the Membership was last updated. |
| `authority` | String | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `create_time` | String | Output only. When the Membership was created. |
| `monitoring_config` | String | Optional. The monitoring config information for this membership. |
| `state` | String | Output only. State of the Membership resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create membership
membership = provider.gkehub_api.Membership {
    parent = "value"  # Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`.
}

# Access membership outputs
membership_id = membership.id
membership_unique_id = membership.unique_id
membership_description = membership.description
membership_labels = membership.labels
membership_name = membership.name
membership_delete_time = membership.delete_time
membership_membership_type = membership.membership_type
membership_last_connection_time = membership.last_connection_time
membership_cluster_tier = membership.cluster_tier
membership_endpoint = membership.endpoint
membership_external_id = membership.external_id
membership_update_time = membership.update_time
membership_authority = membership.authority
membership_create_time = membership.create_time
membership_monitoring_config = membership.monitoring_config
membership_state = membership.state
```

---


### Fleet

Creates a fleet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. When the Fleet was last updated. |
| `delete_time` | String |  | Output only. When the Fleet was deleted. |
| `display_name` | String |  | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `create_time` | String |  | Output only. When the Fleet was created. |
| `name` | String |  | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |
| `default_cluster_config` | String |  | Optional. The default cluster configurations to apply across the fleet. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Fleet. |
| `state` | String |  | Output only. State of the namespace resource. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Fleet will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. When the Fleet was last updated. |
| `delete_time` | String | Output only. When the Fleet was deleted. |
| `display_name` | String | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `create_time` | String | Output only. When the Fleet was created. |
| `name` | String | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |
| `default_cluster_config` | String | Optional. The default cluster configurations to apply across the fleet. |
| `labels` | HashMap<String, String> | Optional. Labels for this Fleet. |
| `state` | String | Output only. State of the namespace resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create fleet
fleet = provider.gkehub_api.Fleet {
    parent = "value"  # Required. The parent (project and location) where the Fleet will be created. Specified in the format `projects/*/locations/*`.
}

# Access fleet outputs
fleet_id = fleet.id
fleet_update_time = fleet.update_time
fleet_delete_time = fleet.delete_time
fleet_display_name = fleet.display_name
fleet_uid = fleet.uid
fleet_create_time = fleet.create_time
fleet_name = fleet.name
fleet_default_cluster_config = fleet.default_cluster_config
fleet_labels = fleet.labels
fleet_state = fleet.state
```

---


### Rbacrolebinding

Creates a Membership RBACRoleBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `group` | String |  | group is the group, as seen by the kubernetes cluster. |
| `state` | String |  | Output only. State of the rbacrolebinding resource. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this RBACRolebinding. |
| `create_time` | String |  | Output only. When the rbacrolebinding was created. |
| `role` | String |  | Required. Role to bind to the principal |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `user` | String |  | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |
| `name` | String |  | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `delete_time` | String |  | Output only. When the rbacrolebinding was deleted. |
| `update_time` | String |  | Output only. When the rbacrolebinding was last updated. |
| `parent` | String | ✅ | Required. The parent (project and location) where the RBACRoleBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `group` | String | group is the group, as seen by the kubernetes cluster. |
| `state` | String | Output only. State of the rbacrolebinding resource. |
| `labels` | HashMap<String, String> | Optional. Labels for this RBACRolebinding. |
| `create_time` | String | Output only. When the rbacrolebinding was created. |
| `role` | String | Required. Role to bind to the principal |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `user` | String | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |
| `name` | String | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `delete_time` | String | Output only. When the rbacrolebinding was deleted. |
| `update_time` | String | Output only. When the rbacrolebinding was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rbacrolebinding
rbacrolebinding = provider.gkehub_api.Rbacrolebinding {
    parent = "value"  # Required. The parent (project and location) where the RBACRoleBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`.
}

# Access rbacrolebinding outputs
rbacrolebinding_id = rbacrolebinding.id
rbacrolebinding_group = rbacrolebinding.group
rbacrolebinding_state = rbacrolebinding.state
rbacrolebinding_labels = rbacrolebinding.labels
rbacrolebinding_create_time = rbacrolebinding.create_time
rbacrolebinding_role = rbacrolebinding.role
rbacrolebinding_uid = rbacrolebinding.uid
rbacrolebinding_user = rbacrolebinding.user
rbacrolebinding_name = rbacrolebinding.name
rbacrolebinding_delete_time = rbacrolebinding.delete_time
rbacrolebinding_update_time = rbacrolebinding.update_time
```

---


### Namespace

Creates a fleet namespace.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the namespace resource. |
| `delete_time` | String |  | Output only. When the namespace was deleted. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Namespace. |
| `name` | String |  | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `namespace_labels` | HashMap<String, String> |  | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `scope` | String |  | Required. Scope associated with the namespace |
| `create_time` | String |  | Output only. When the namespace was created. |
| `update_time` | String |  | Output only. When the namespace was last updated. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Namespace will be created. Specified in the format `projects/*/locations/*/scopes/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the namespace resource. |
| `delete_time` | String | Output only. When the namespace was deleted. |
| `labels` | HashMap<String, String> | Optional. Labels for this Namespace. |
| `name` | String | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `namespace_labels` | HashMap<String, String> | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `scope` | String | Required. Scope associated with the namespace |
| `create_time` | String | Output only. When the namespace was created. |
| `update_time` | String | Output only. When the namespace was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create namespace
namespace = provider.gkehub_api.Namespace {
    parent = "value"  # Required. The parent (project and location) where the Namespace will be created. Specified in the format `projects/*/locations/*/scopes/*`.
}

# Access namespace outputs
namespace_id = namespace.id
namespace_state = namespace.state
namespace_delete_time = namespace.delete_time
namespace_labels = namespace.labels
namespace_name = namespace.name
namespace_namespace_labels = namespace.namespace_labels
namespace_uid = namespace.uid
namespace_scope = namespace.scope
namespace_create_time = namespace.create_time
namespace_update_time = namespace.update_time
```

---


### Scope

Creates a Scope.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. When the scope was deleted. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Scope. |
| `update_time` | String |  | Output only. When the scope was last updated. |
| `name` | String |  | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `namespace_labels` | HashMap<String, String> |  | Optional. Scope-level cluster namespace labels. For the member clusters bound to the Scope, these labels are applied to each namespace under the Scope. Scope-level labels take precedence over Namespace-level labels (`namespace_labels` in the Fleet Namespace resource) if they share a key. Keys and values must be Kubernetes-conformant. |
| `create_time` | String |  | Output only. When the scope was created. |
| `state` | String |  | Output only. State of the scope resource. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Scope will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. When the scope was deleted. |
| `labels` | HashMap<String, String> | Optional. Labels for this Scope. |
| `update_time` | String | Output only. When the scope was last updated. |
| `name` | String | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `namespace_labels` | HashMap<String, String> | Optional. Scope-level cluster namespace labels. For the member clusters bound to the Scope, these labels are applied to each namespace under the Scope. Scope-level labels take precedence over Namespace-level labels (`namespace_labels` in the Fleet Namespace resource) if they share a key. Keys and values must be Kubernetes-conformant. |
| `create_time` | String | Output only. When the scope was created. |
| `state` | String | Output only. State of the scope resource. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create scope
scope = provider.gkehub_api.Scope {
    parent = "value"  # Required. The parent (project and location) where the Scope will be created. Specified in the format `projects/*/locations/*`.
}

# Access scope outputs
scope_id = scope.id
scope_delete_time = scope.delete_time
scope_labels = scope.labels
scope_update_time = scope.update_time
scope_name = scope.name
scope_namespace_labels = scope.namespace_labels
scope_create_time = scope.create_time
scope_state = scope.state
scope_uid = scope.uid
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.gkehub_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_labels = location.labels
location_display_name = location.display_name
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
```

---


### Binding

Creates a MembershipBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Labels for this MembershipBinding. |
| `update_time` | String |  | Output only. When the membership binding was last updated. |
| `name` | String |  | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |
| `delete_time` | String |  | Output only. When the membership binding was deleted. |
| `scope` | String |  | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |
| `state` | String |  | Output only. State of the membership binding resource. |
| `create_time` | String |  | Output only. When the membership binding was created. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `parent` | String | ✅ | Required. The parent (project and location) where the MembershipBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Labels for this MembershipBinding. |
| `update_time` | String | Output only. When the membership binding was last updated. |
| `name` | String | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |
| `delete_time` | String | Output only. When the membership binding was deleted. |
| `scope` | String | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |
| `state` | String | Output only. State of the membership binding resource. |
| `create_time` | String | Output only. When the membership binding was created. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create binding
binding = provider.gkehub_api.Binding {
    parent = "value"  # Required. The parent (project and location) where the MembershipBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`.
}

# Access binding outputs
binding_id = binding.id
binding_labels = binding.labels
binding_update_time = binding.update_time
binding_name = binding.name
binding_delete_time = binding.delete_time
binding_scope = binding.scope
binding_state = binding.state
binding_create_time = binding.create_time
binding_uid = binding.uid
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
location_display_name = location.display_name
location_labels = location.labels
```

---


### Membership

Creates a new Membership. **This is currently only supported for GKE clusters on Google Cloud**. To register other clusters, follow the instructions at https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. When the Membership was last updated. |
| `delete_time` | String |  | Output only. When the Membership was deleted. |
| `create_time` | String |  | Output only. When the Membership was created. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this membership. |
| `name` | String |  | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `state` | String |  | Output only. State of the Membership resource. |
| `description` | String |  | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `endpoint` | String |  | Optional. Endpoint information to reach this member. |
| `last_connection_time` | String |  | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `monitoring_config` | String |  | Optional. The monitoring config information for this membership. |
| `infrastructure_type` | String |  | Optional. The infrastructure type this Membership is running on. |
| `external_id` | String |  | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. For GKE clusters, external_id is managed by the Hub API and updates will be ignored. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `authority` | String |  | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `parent` | String | ✅ | Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. When the Membership was last updated. |
| `delete_time` | String | Output only. When the Membership was deleted. |
| `create_time` | String | Output only. When the Membership was created. |
| `labels` | HashMap<String, String> | Optional. Labels for this membership. |
| `name` | String | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `state` | String | Output only. State of the Membership resource. |
| `description` | String | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `endpoint` | String | Optional. Endpoint information to reach this member. |
| `last_connection_time` | String | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `monitoring_config` | String | Optional. The monitoring config information for this membership. |
| `infrastructure_type` | String | Optional. The infrastructure type this Membership is running on. |
| `external_id` | String | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. For GKE clusters, external_id is managed by the Hub API and updates will be ignored. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `authority` | String | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create membership
membership = provider.gkehub_api.Membership {
    parent = "value"  # Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`.
}

# Access membership outputs
membership_id = membership.id
membership_update_time = membership.update_time
membership_delete_time = membership.delete_time
membership_create_time = membership.create_time
membership_labels = membership.labels
membership_name = membership.name
membership_state = membership.state
membership_description = membership.description
membership_endpoint = membership.endpoint
membership_last_connection_time = membership.last_connection_time
membership_unique_id = membership.unique_id
membership_monitoring_config = membership.monitoring_config
membership_infrastructure_type = membership.infrastructure_type
membership_external_id = membership.external_id
membership_authority = membership.authority
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.

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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.gkehub_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_name = operation.name
operation_error = operation.error
operation_response = operation.response
operation_done = operation.done
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_name = location.name
location_display_name = location.display_name
location_location_id = location.location_id
location_labels = location.labels
location_metadata = location.metadata
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read

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
operation = provider.gkehub_api.Operation {
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


### Feature

Creates membershipFeature under a given parent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lifecycle_state` | String |  | Output only. Lifecycle information of the resource itself. |
| `update_time` | String |  | Output only. When the MembershipFeature resource was last updated. |
| `state` | String |  | Output only. State of the this membershipFeature. |
| `name` | String |  | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `delete_time` | String |  | Output only. When the MembershipFeature resource was deleted. |
| `spec` | String |  | Optional. Spec of this membershipFeature. |
| `create_time` | String |  | Output only. When the MembershipFeature resource was created. |
| `labels` | HashMap<String, String> |  | GCP labels for this MembershipFeature. |
| `parent` | String | ✅ | Required. The name of parent where the MembershipFeature will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lifecycle_state` | String | Output only. Lifecycle information of the resource itself. |
| `update_time` | String | Output only. When the MembershipFeature resource was last updated. |
| `state` | String | Output only. State of the this membershipFeature. |
| `name` | String | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `delete_time` | String | Output only. When the MembershipFeature resource was deleted. |
| `spec` | String | Optional. Spec of this membershipFeature. |
| `create_time` | String | Output only. When the MembershipFeature resource was created. |
| `labels` | HashMap<String, String> | GCP labels for this MembershipFeature. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature
feature = provider.gkehub_api.Feature {
    parent = "value"  # Required. The name of parent where the MembershipFeature will be created. Specified in the format `projects/*/locations/*/memberships/*`.
}

# Access feature outputs
feature_id = feature.id
feature_lifecycle_state = feature.lifecycle_state
feature_update_time = feature.update_time
feature_state = feature.state
feature_name = feature.name
feature_delete_time = feature.delete_time
feature_spec = feature.spec
feature_create_time = feature.create_time
feature_labels = feature.labels
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_labels = location.labels
location_display_name = location.display_name
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.gkehub_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Feature

Creates membershipFeature under a given parent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. When the MembershipFeature resource was deleted. |
| `labels` | HashMap<String, String> |  | GCP labels for this MembershipFeature. |
| `spec` | String |  | Optional. Spec of this membershipFeature. |
| `create_time` | String |  | Output only. When the MembershipFeature resource was created. |
| `lifecycle_state` | String |  | Output only. Lifecycle information of the resource itself. |
| `name` | String |  | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `update_time` | String |  | Output only. When the MembershipFeature resource was last updated. |
| `state` | String |  | Output only. State of the this membershipFeature. |
| `parent` | String | ✅ | Required. The name of parent where the MembershipFeature will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. When the MembershipFeature resource was deleted. |
| `labels` | HashMap<String, String> | GCP labels for this MembershipFeature. |
| `spec` | String | Optional. Spec of this membershipFeature. |
| `create_time` | String | Output only. When the MembershipFeature resource was created. |
| `lifecycle_state` | String | Output only. Lifecycle information of the resource itself. |
| `name` | String | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `update_time` | String | Output only. When the MembershipFeature resource was last updated. |
| `state` | String | Output only. State of the this membershipFeature. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature
feature = provider.gkehub_api.Feature {
    parent = "value"  # Required. The name of parent where the MembershipFeature will be created. Specified in the format `projects/*/locations/*/memberships/*`.
}

# Access feature outputs
feature_id = feature.id
feature_delete_time = feature.delete_time
feature_labels = feature.labels
feature_spec = feature.spec
feature_create_time = feature.create_time
feature_lifecycle_state = feature.lifecycle_state
feature_name = feature.name
feature_update_time = feature.update_time
feature_state = feature.state
```

---


### Scope

Creates a Scope.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `update_time` | String |  | Output only. When the scope was last updated. |
| `delete_time` | String |  | Output only. When the scope was deleted. |
| `create_time` | String |  | Output only. When the scope was created. |
| `state` | String |  | Output only. State of the scope resource. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Scope. |
| `namespace_labels` | HashMap<String, String> |  | Optional. Scope-level cluster namespace labels. For the member clusters bound to the Scope, these labels are applied to each namespace under the Scope. Scope-level labels take precedence over Namespace-level labels (`namespace_labels` in the Fleet Namespace resource) if they share a key. Keys and values must be Kubernetes-conformant. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Scope will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `update_time` | String | Output only. When the scope was last updated. |
| `delete_time` | String | Output only. When the scope was deleted. |
| `create_time` | String | Output only. When the scope was created. |
| `state` | String | Output only. State of the scope resource. |
| `labels` | HashMap<String, String> | Optional. Labels for this Scope. |
| `namespace_labels` | HashMap<String, String> | Optional. Scope-level cluster namespace labels. For the member clusters bound to the Scope, these labels are applied to each namespace under the Scope. Scope-level labels take precedence over Namespace-level labels (`namespace_labels` in the Fleet Namespace resource) if they share a key. Keys and values must be Kubernetes-conformant. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create scope
scope = provider.gkehub_api.Scope {
    parent = "value"  # Required. The parent (project and location) where the Scope will be created. Specified in the format `projects/*/locations/*`.
}

# Access scope outputs
scope_id = scope.id
scope_name = scope.name
scope_update_time = scope.update_time
scope_delete_time = scope.delete_time
scope_create_time = scope.create_time
scope_state = scope.state
scope_labels = scope.labels
scope_namespace_labels = scope.namespace_labels
scope_uid = scope.uid
```

---


### Fleet

Creates a fleet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `update_time` | String |  | Output only. When the Fleet was last updated. |
| `delete_time` | String |  | Output only. When the Fleet was deleted. |
| `create_time` | String |  | Output only. When the Fleet was created. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `default_cluster_config` | String |  | Optional. The default cluster configurations to apply across the fleet. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Fleet. |
| `state` | String |  | Output only. State of the namespace resource. |
| `name` | String |  | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |
| `parent` | String | ✅ | Required. The parent (project and location) where the Fleet will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `update_time` | String | Output only. When the Fleet was last updated. |
| `delete_time` | String | Output only. When the Fleet was deleted. |
| `create_time` | String | Output only. When the Fleet was created. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `default_cluster_config` | String | Optional. The default cluster configurations to apply across the fleet. |
| `labels` | HashMap<String, String> | Optional. Labels for this Fleet. |
| `state` | String | Output only. State of the namespace resource. |
| `name` | String | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create fleet
fleet = provider.gkehub_api.Fleet {
    parent = "value"  # Required. The parent (project and location) where the Fleet will be created. Specified in the format `projects/*/locations/*`.
}

# Access fleet outputs
fleet_id = fleet.id
fleet_display_name = fleet.display_name
fleet_update_time = fleet.update_time
fleet_delete_time = fleet.delete_time
fleet_create_time = fleet.create_time
fleet_uid = fleet.uid
fleet_default_cluster_config = fleet.default_cluster_config
fleet_labels = fleet.labels
fleet_state = fleet.state
fleet_name = fleet.name
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_name = location.name
location_metadata = location.metadata
location_location_id = location.location_id
location_display_name = location.display_name
location_labels = location.labels
```

---


### Namespace

Creates a fleet namespace.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the namespace resource. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `update_time` | String |  | Output only. When the namespace was last updated. |
| `namespace_labels` | HashMap<String, String> |  | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `delete_time` | String |  | Output only. When the namespace was deleted. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Namespace. |
| `create_time` | String |  | Output only. When the namespace was created. |
| `name` | String |  | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `scope` | String |  | Required. Scope associated with the namespace |
| `parent` | String | ✅ | Required. The parent (project and location) where the Namespace will be created. Specified in the format `projects/*/locations/*/scopes/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the namespace resource. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `update_time` | String | Output only. When the namespace was last updated. |
| `namespace_labels` | HashMap<String, String> | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `delete_time` | String | Output only. When the namespace was deleted. |
| `labels` | HashMap<String, String> | Optional. Labels for this Namespace. |
| `create_time` | String | Output only. When the namespace was created. |
| `name` | String | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `scope` | String | Required. Scope associated with the namespace |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create namespace
namespace = provider.gkehub_api.Namespace {
    parent = "value"  # Required. The parent (project and location) where the Namespace will be created. Specified in the format `projects/*/locations/*/scopes/*`.
}

# Access namespace outputs
namespace_id = namespace.id
namespace_state = namespace.state
namespace_uid = namespace.uid
namespace_update_time = namespace.update_time
namespace_namespace_labels = namespace.namespace_labels
namespace_delete_time = namespace.delete_time
namespace_labels = namespace.labels
namespace_create_time = namespace.create_time
namespace_name = namespace.name
namespace_scope = namespace.scope
```

---


### Membership

Creates a new Membership. **This is currently only supported for GKE clusters on Google Cloud**. To register other clusters, follow the instructions at https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cluster_tier` | String |  | Output only. The tier of the cluster. |
| `delete_time` | String |  | Output only. When the Membership was deleted. |
| `last_connection_time` | String |  | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `name` | String |  | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `external_id` | String |  | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `endpoint` | String |  | Optional. Endpoint information to reach this member. |
| `membership_type` | String |  | Output only. The type of the membership. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `description` | String |  | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `state` | String |  | Output only. State of the Membership resource. |
| `create_time` | String |  | Output only. When the Membership was created. |
| `authority` | String |  | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `monitoring_config` | String |  | Optional. The monitoring config information for this membership. |
| `update_time` | String |  | Output only. When the Membership was last updated. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cluster_tier` | String | Output only. The tier of the cluster. |
| `delete_time` | String | Output only. When the Membership was deleted. |
| `last_connection_time` | String | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `name` | String | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `external_id` | String | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `endpoint` | String | Optional. Endpoint information to reach this member. |
| `membership_type` | String | Output only. The type of the membership. |
| `labels` | HashMap<String, String> | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `description` | String | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `state` | String | Output only. State of the Membership resource. |
| `create_time` | String | Output only. When the Membership was created. |
| `authority` | String | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `monitoring_config` | String | Optional. The monitoring config information for this membership. |
| `update_time` | String | Output only. When the Membership was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create membership
membership = provider.gkehub_api.Membership {
    parent = "value"  # Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`.
}

# Access membership outputs
membership_id = membership.id
membership_cluster_tier = membership.cluster_tier
membership_delete_time = membership.delete_time
membership_last_connection_time = membership.last_connection_time
membership_name = membership.name
membership_unique_id = membership.unique_id
membership_external_id = membership.external_id
membership_endpoint = membership.endpoint
membership_membership_type = membership.membership_type
membership_labels = membership.labels
membership_description = membership.description
membership_state = membership.state
membership_create_time = membership.create_time
membership_authority = membership.authority
membership_monitoring_config = membership.monitoring_config
membership_update_time = membership.update_time
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.gkehub_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Rbacrolebinding

Creates a Membership RBACRoleBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the rbacrolebinding resource. |
| `create_time` | String |  | Output only. When the rbacrolebinding was created. |
| `group` | String |  | group is the group, as seen by the kubernetes cluster. |
| `name` | String |  | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `role` | String |  | Required. Role to bind to the principal |
| `delete_time` | String |  | Output only. When the rbacrolebinding was deleted. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `update_time` | String |  | Output only. When the rbacrolebinding was last updated. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this RBACRolebinding. |
| `user` | String |  | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |
| `parent` | String | ✅ | Required. The parent (project and location) where the RBACRoleBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the rbacrolebinding resource. |
| `create_time` | String | Output only. When the rbacrolebinding was created. |
| `group` | String | group is the group, as seen by the kubernetes cluster. |
| `name` | String | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `role` | String | Required. Role to bind to the principal |
| `delete_time` | String | Output only. When the rbacrolebinding was deleted. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `update_time` | String | Output only. When the rbacrolebinding was last updated. |
| `labels` | HashMap<String, String> | Optional. Labels for this RBACRolebinding. |
| `user` | String | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rbacrolebinding
rbacrolebinding = provider.gkehub_api.Rbacrolebinding {
    parent = "value"  # Required. The parent (project and location) where the RBACRoleBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`.
}

# Access rbacrolebinding outputs
rbacrolebinding_id = rbacrolebinding.id
rbacrolebinding_state = rbacrolebinding.state
rbacrolebinding_create_time = rbacrolebinding.create_time
rbacrolebinding_group = rbacrolebinding.group
rbacrolebinding_name = rbacrolebinding.name
rbacrolebinding_role = rbacrolebinding.role
rbacrolebinding_delete_time = rbacrolebinding.delete_time
rbacrolebinding_uid = rbacrolebinding.uid
rbacrolebinding_update_time = rbacrolebinding.update_time
rbacrolebinding_labels = rbacrolebinding.labels
rbacrolebinding_user = rbacrolebinding.user
```

---


### Binding

Creates a MembershipBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the membership binding resource. |
| `delete_time` | String |  | Output only. When the membership binding was deleted. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this MembershipBinding. |
| `create_time` | String |  | Output only. When the membership binding was created. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `scope` | String |  | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |
| `name` | String |  | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |
| `update_time` | String |  | Output only. When the membership binding was last updated. |
| `parent` | String | ✅ | Required. The parent (project and location) where the MembershipBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the membership binding resource. |
| `delete_time` | String | Output only. When the membership binding was deleted. |
| `labels` | HashMap<String, String> | Optional. Labels for this MembershipBinding. |
| `create_time` | String | Output only. When the membership binding was created. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `scope` | String | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |
| `name` | String | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |
| `update_time` | String | Output only. When the membership binding was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create binding
binding = provider.gkehub_api.Binding {
    parent = "value"  # Required. The parent (project and location) where the MembershipBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`.
}

# Access binding outputs
binding_id = binding.id
binding_state = binding.state
binding_delete_time = binding.delete_time
binding_labels = binding.labels
binding_create_time = binding.create_time
binding_uid = binding.uid
binding_scope = binding.scope
binding_name = binding.name
binding_update_time = binding.update_time
```

---


### Feature

Adds a new Feature.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `spec` | String |  | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `name` | String |  | Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`. |
| `labels` | HashMap<String, String> |  | Labels for this Feature. |
| `scope_states` | HashMap<String, String> |  | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `create_time` | String |  | Output only. When the Feature resource was created. |
| `resource_state` | String |  | Output only. State of the Feature resource itself. |
| `scope_specs` | HashMap<String, String> |  | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `unreachable` | Vec<String> |  | Output only. List of locations that could not be reached while fetching this feature. |
| `update_time` | String |  | Output only. When the Feature resource was last updated. |
| `state` | String |  | Output only. The Fleet-wide Feature state. |
| `membership_specs` | HashMap<String, String> |  | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `membership_states` | HashMap<String, String> |  | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `delete_time` | String |  | Output only. When the Feature resource was deleted. |
| `fleet_default_member_config` | String |  | Optional. Feature configuration applicable to all memberships of the fleet. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Feature will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spec` | String | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `name` | String | Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`. |
| `labels` | HashMap<String, String> | Labels for this Feature. |
| `scope_states` | HashMap<String, String> | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `create_time` | String | Output only. When the Feature resource was created. |
| `resource_state` | String | Output only. State of the Feature resource itself. |
| `scope_specs` | HashMap<String, String> | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `unreachable` | Vec<String> | Output only. List of locations that could not be reached while fetching this feature. |
| `update_time` | String | Output only. When the Feature resource was last updated. |
| `state` | String | Output only. The Fleet-wide Feature state. |
| `membership_specs` | HashMap<String, String> | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `membership_states` | HashMap<String, String> | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `delete_time` | String | Output only. When the Feature resource was deleted. |
| `fleet_default_member_config` | String | Optional. Feature configuration applicable to all memberships of the fleet. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature
feature = provider.gkehub_api.Feature {
    parent = "value"  # Required. The parent (project and location) where the Feature will be created. Specified in the format `projects/*/locations/*`.
}

# Access feature outputs
feature_id = feature.id
feature_spec = feature.spec
feature_name = feature.name
feature_labels = feature.labels
feature_scope_states = feature.scope_states
feature_create_time = feature.create_time
feature_resource_state = feature.resource_state
feature_scope_specs = feature.scope_specs
feature_unreachable = feature.unreachable
feature_update_time = feature.update_time
feature_state = feature.state
feature_membership_specs = feature.membership_specs
feature_membership_states = feature.membership_states
feature_delete_time = feature.delete_time
feature_fleet_default_member_config = feature.fleet_default_member_config
```

---


### Membership

Creates a new Membership. **This is currently only supported for GKE clusters on Google Cloud**. To register other clusters, follow the instructions at https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. When the Membership was created. |
| `update_time` | String |  | Output only. When the Membership was last updated. |
| `last_connection_time` | String |  | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `infrastructure_type` | String |  | Optional. The infrastructure type this Membership is running on. |
| `membership_type` | String |  | Output only. The type of the membership. |
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `description` | String |  | Optional. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` |
| `endpoint` | String |  | Optional. Endpoint information to reach this member. |
| `labels` | HashMap<String, String> |  | Optional. GCP labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `authority` | String |  | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `monitoring_config` | String |  | Optional. The monitoring config information for this membership. |
| `external_id` | String |  | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. For GKE clusters, external_id is managed by the Hub API and updates will be ignored. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `delete_time` | String |  | Output only. When the Membership was deleted. |
| `name` | String |  | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `state` | String |  | Output only. State of the Membership resource. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. When the Membership was created. |
| `update_time` | String | Output only. When the Membership was last updated. |
| `last_connection_time` | String | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `infrastructure_type` | String | Optional. The infrastructure type this Membership is running on. |
| `membership_type` | String | Output only. The type of the membership. |
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `description` | String | Optional. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` |
| `endpoint` | String | Optional. Endpoint information to reach this member. |
| `labels` | HashMap<String, String> | Optional. GCP labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `authority` | String | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `monitoring_config` | String | Optional. The monitoring config information for this membership. |
| `external_id` | String | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. For GKE clusters, external_id is managed by the Hub API and updates will be ignored. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `delete_time` | String | Output only. When the Membership was deleted. |
| `name` | String | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `state` | String | Output only. State of the Membership resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create membership
membership = provider.gkehub_api.Membership {
    parent = "value"  # Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`.
}

# Access membership outputs
membership_id = membership.id
membership_create_time = membership.create_time
membership_update_time = membership.update_time
membership_last_connection_time = membership.last_connection_time
membership_infrastructure_type = membership.infrastructure_type
membership_membership_type = membership.membership_type
membership_unique_id = membership.unique_id
membership_description = membership.description
membership_endpoint = membership.endpoint
membership_labels = membership.labels
membership_authority = membership.authority
membership_monitoring_config = membership.monitoring_config
membership_external_id = membership.external_id
membership_delete_time = membership.delete_time
membership_name = membership.name
membership_state = membership.state
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.gkehub_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_metadata = operation.metadata
operation_response = operation.response
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_metadata = location.metadata
location_name = location.name
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_metadata = location.metadata
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
location_name = location.name
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.gkehub_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
operation_error = operation.error
```

---


### Feature

Creates membershipFeature under a given parent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | GCP labels for this MembershipFeature. |
| `name` | String |  | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `state` | String |  | Output only. State of the this membershipFeature. |
| `update_time` | String |  | Output only. When the MembershipFeature resource was last updated. |
| `spec` | String |  | Optional. Spec of this membershipFeature. |
| `lifecycle_state` | String |  | Output only. Lifecycle information of the resource itself. |
| `delete_time` | String |  | Output only. When the MembershipFeature resource was deleted. |
| `create_time` | String |  | Output only. When the MembershipFeature resource was created. |
| `parent` | String | ✅ | Required. The name of parent where the MembershipFeature will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | GCP labels for this MembershipFeature. |
| `name` | String | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `state` | String | Output only. State of the this membershipFeature. |
| `update_time` | String | Output only. When the MembershipFeature resource was last updated. |
| `spec` | String | Optional. Spec of this membershipFeature. |
| `lifecycle_state` | String | Output only. Lifecycle information of the resource itself. |
| `delete_time` | String | Output only. When the MembershipFeature resource was deleted. |
| `create_time` | String | Output only. When the MembershipFeature resource was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature
feature = provider.gkehub_api.Feature {
    parent = "value"  # Required. The name of parent where the MembershipFeature will be created. Specified in the format `projects/*/locations/*/memberships/*`.
}

# Access feature outputs
feature_id = feature.id
feature_labels = feature.labels
feature_name = feature.name
feature_state = feature.state
feature_update_time = feature.update_time
feature_spec = feature.spec
feature_lifecycle_state = feature.lifecycle_state
feature_delete_time = feature.delete_time
feature_create_time = feature.create_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple operation resources
operation_0 = provider.gkehub_api.Operation {
    name = "value-0"
}
operation_1 = provider.gkehub_api.Operation {
    name = "value-1"
}
operation_2 = provider.gkehub_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.gkehub_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Gkehub_api Documentation](https://cloud.google.com/gkehub_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
