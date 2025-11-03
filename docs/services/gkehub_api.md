# Gkehub_api Service



**Resources**: 42

---

## Overview

The gkehub_api service provides access to 42 resource types:

- [Membership](#membership) [CRUD]
- [Binding](#binding) [CRUD]
- [Namespace](#namespace) [CRUD]
- [Operation](#operation) [CRD]
- [Fleet](#fleet) [CRUD]
- [Location](#location) [R]
- [Feature](#feature) [CRUD]
- [Scope](#scope) [CRUD]
- [Rbacrolebinding](#rbacrolebinding) [CRUD]
- [Scope](#scope) [CRUD]
- [Namespace](#namespace) [CRUD]
- [Binding](#binding) [CRUD]
- [Feature](#feature) [CRUD]
- [Fleet](#fleet) [CRUD]
- [Rbacrolebinding](#rbacrolebinding) [CRUD]
- [Location](#location) [R]
- [Membership](#membership) [CRUD]
- [Operation](#operation) [CRD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Membership](#membership) [CRUD]
- [Location](#location) [R]
- [Operation](#operation) [CR]
- [Feature](#feature) [CRUD]
- [Location](#location) [R]
- [Feature](#feature) [CRUD]
- [Operation](#operation) [CR]
- [Membership](#membership) [CRUD]
- [Rbacrolebinding](#rbacrolebinding) [CRUD]
- [Namespace](#namespace) [CRUD]
- [Feature](#feature) [CRUD]
- [Fleet](#fleet) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Scope](#scope) [CRUD]
- [Binding](#binding) [CRUD]
- [Operation](#operation) [CRD]
- [Membership](#membership) [CRUD]
- [Location](#location) [R]
- [Location](#location) [R]
- [Operation](#operation) [CR]
- [Feature](#feature) [CRUD]

---

## Resources


### Membership

Creates a new Membership. **This is currently only supported for GKE clusters on Google Cloud**. To register other clusters, follow the instructions at https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. When the Membership was created. |
| `monitoring_config` | String |  | Optional. The monitoring config information for this membership. |
| `membership_type` | String |  | Output only. The type of the membership. |
| `endpoint` | String |  | Optional. Endpoint information to reach this member. |
| `authority` | String |  | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `update_time` | String |  | Output only. When the Membership was last updated. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `external_id` | String |  | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `last_connection_time` | String |  | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `name` | String |  | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `state` | String |  | Output only. State of the Membership resource. |
| `cluster_tier` | String |  | Output only. The tier of the cluster. |
| `delete_time` | String |  | Output only. When the Membership was deleted. |
| `description` | String |  | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. When the Membership was created. |
| `monitoring_config` | String | Optional. The monitoring config information for this membership. |
| `membership_type` | String | Output only. The type of the membership. |
| `endpoint` | String | Optional. Endpoint information to reach this member. |
| `authority` | String | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `update_time` | String | Output only. When the Membership was last updated. |
| `labels` | HashMap<String, String> | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `external_id` | String | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `last_connection_time` | String | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `name` | String | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `state` | String | Output only. State of the Membership resource. |
| `cluster_tier` | String | Output only. The tier of the cluster. |
| `delete_time` | String | Output only. When the Membership was deleted. |
| `description` | String | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |


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
membership_monitoring_config = membership.monitoring_config
membership_membership_type = membership.membership_type
membership_endpoint = membership.endpoint
membership_authority = membership.authority
membership_update_time = membership.update_time
membership_labels = membership.labels
membership_external_id = membership.external_id
membership_last_connection_time = membership.last_connection_time
membership_name = membership.name
membership_unique_id = membership.unique_id
membership_state = membership.state
membership_cluster_tier = membership.cluster_tier
membership_delete_time = membership.delete_time
membership_description = membership.description
```

---


### Binding

Creates a MembershipBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. When the membership binding was deleted. |
| `create_time` | String |  | Output only. When the membership binding was created. |
| `name` | String |  | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |
| `update_time` | String |  | Output only. When the membership binding was last updated. |
| `scope` | String |  | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this MembershipBinding. |
| `state` | String |  | Output only. State of the membership binding resource. |
| `parent` | String | ✅ | Required. The parent (project and location) where the MembershipBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. When the membership binding was deleted. |
| `create_time` | String | Output only. When the membership binding was created. |
| `name` | String | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |
| `update_time` | String | Output only. When the membership binding was last updated. |
| `scope` | String | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `labels` | HashMap<String, String> | Optional. Labels for this MembershipBinding. |
| `state` | String | Output only. State of the membership binding resource. |


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
binding_delete_time = binding.delete_time
binding_create_time = binding.create_time
binding_name = binding.name
binding_update_time = binding.update_time
binding_scope = binding.scope
binding_uid = binding.uid
binding_labels = binding.labels
binding_state = binding.state
```

---


### Namespace

Creates a fleet namespace.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. When the namespace was deleted. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `scope` | String |  | Required. Scope associated with the namespace |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Namespace. |
| `name` | String |  | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `state` | String |  | Output only. State of the namespace resource. |
| `update_time` | String |  | Output only. When the namespace was last updated. |
| `namespace_labels` | HashMap<String, String> |  | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `create_time` | String |  | Output only. When the namespace was created. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Namespace will be created. Specified in the format `projects/*/locations/*/scopes/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. When the namespace was deleted. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `scope` | String | Required. Scope associated with the namespace |
| `labels` | HashMap<String, String> | Optional. Labels for this Namespace. |
| `name` | String | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `state` | String | Output only. State of the namespace resource. |
| `update_time` | String | Output only. When the namespace was last updated. |
| `namespace_labels` | HashMap<String, String> | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `create_time` | String | Output only. When the namespace was created. |


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
namespace_delete_time = namespace.delete_time
namespace_uid = namespace.uid
namespace_scope = namespace.scope
namespace_labels = namespace.labels
namespace_name = namespace.name
namespace_state = namespace.state
namespace_update_time = namespace.update_time
namespace_namespace_labels = namespace.namespace_labels
namespace_create_time = namespace.create_time
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
operation = provider.gkehub_api.Operation {
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


### Fleet

Creates a fleet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Fleet. |
| `update_time` | String |  | Output only. When the Fleet was last updated. |
| `default_cluster_config` | String |  | Optional. The default cluster configurations to apply across the fleet. |
| `display_name` | String |  | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `name` | String |  | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |
| `create_time` | String |  | Output only. When the Fleet was created. |
| `state` | String |  | Output only. State of the namespace resource. |
| `delete_time` | String |  | Output only. When the Fleet was deleted. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Fleet will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `labels` | HashMap<String, String> | Optional. Labels for this Fleet. |
| `update_time` | String | Output only. When the Fleet was last updated. |
| `default_cluster_config` | String | Optional. The default cluster configurations to apply across the fleet. |
| `display_name` | String | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `name` | String | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |
| `create_time` | String | Output only. When the Fleet was created. |
| `state` | String | Output only. State of the namespace resource. |
| `delete_time` | String | Output only. When the Fleet was deleted. |


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
fleet_uid = fleet.uid
fleet_labels = fleet.labels
fleet_update_time = fleet.update_time
fleet_default_cluster_config = fleet.default_cluster_config
fleet_display_name = fleet.display_name
fleet_name = fleet.name
fleet_create_time = fleet.create_time
fleet_state = fleet.state
fleet_delete_time = fleet.delete_time
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
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
location_location_id = location.location_id
```

---


### Feature

Adds a new Feature.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The Fleet-wide Feature state. |
| `unreachable` | Vec<String> |  | Output only. List of locations that could not be reached while fetching this feature. |
| `update_time` | String |  | Output only. When the Feature resource was last updated. |
| `create_time` | String |  | Output only. When the Feature resource was created. |
| `labels` | HashMap<String, String> |  | Labels for this Feature. |
| `resource_state` | String |  | Output only. State of the Feature resource itself. |
| `spec` | String |  | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `delete_time` | String |  | Output only. When the Feature resource was deleted. |
| `scope_specs` | HashMap<String, String> |  | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `scope_states` | HashMap<String, String> |  | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `membership_states` | HashMap<String, String> |  | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `name` | String |  | Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`. |
| `fleet_default_member_config` | String |  | Optional. Feature configuration applicable to all memberships of the fleet. |
| `membership_specs` | HashMap<String, String> |  | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Feature will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The Fleet-wide Feature state. |
| `unreachable` | Vec<String> | Output only. List of locations that could not be reached while fetching this feature. |
| `update_time` | String | Output only. When the Feature resource was last updated. |
| `create_time` | String | Output only. When the Feature resource was created. |
| `labels` | HashMap<String, String> | Labels for this Feature. |
| `resource_state` | String | Output only. State of the Feature resource itself. |
| `spec` | String | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `delete_time` | String | Output only. When the Feature resource was deleted. |
| `scope_specs` | HashMap<String, String> | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `scope_states` | HashMap<String, String> | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `membership_states` | HashMap<String, String> | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `name` | String | Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`. |
| `fleet_default_member_config` | String | Optional. Feature configuration applicable to all memberships of the fleet. |
| `membership_specs` | HashMap<String, String> | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |


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
feature_state = feature.state
feature_unreachable = feature.unreachable
feature_update_time = feature.update_time
feature_create_time = feature.create_time
feature_labels = feature.labels
feature_resource_state = feature.resource_state
feature_spec = feature.spec
feature_delete_time = feature.delete_time
feature_scope_specs = feature.scope_specs
feature_scope_states = feature.scope_states
feature_membership_states = feature.membership_states
feature_name = feature.name
feature_fleet_default_member_config = feature.fleet_default_member_config
feature_membership_specs = feature.membership_specs
```

---


### Scope

Creates a Scope.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. When the scope was created. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |
| `update_time` | String |  | Output only. When the scope was last updated. |
| `name` | String |  | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `state` | String |  | Output only. State of the scope resource. |
| `namespace_labels` | HashMap<String, String> |  | Optional. Scope-level cluster namespace labels. For the member clusters bound to the Scope, these labels are applied to each namespace under the Scope. Scope-level labels take precedence over Namespace-level labels (`namespace_labels` in the Fleet Namespace resource) if they share a key. Keys and values must be Kubernetes-conformant. |
| `delete_time` | String |  | Output only. When the scope was deleted. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Scope. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Scope will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. When the scope was created. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |
| `update_time` | String | Output only. When the scope was last updated. |
| `name` | String | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `state` | String | Output only. State of the scope resource. |
| `namespace_labels` | HashMap<String, String> | Optional. Scope-level cluster namespace labels. For the member clusters bound to the Scope, these labels are applied to each namespace under the Scope. Scope-level labels take precedence over Namespace-level labels (`namespace_labels` in the Fleet Namespace resource) if they share a key. Keys and values must be Kubernetes-conformant. |
| `delete_time` | String | Output only. When the scope was deleted. |
| `labels` | HashMap<String, String> | Optional. Labels for this Scope. |


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
scope_create_time = scope.create_time
scope_uid = scope.uid
scope_update_time = scope.update_time
scope_name = scope.name
scope_state = scope.state
scope_namespace_labels = scope.namespace_labels
scope_delete_time = scope.delete_time
scope_labels = scope.labels
```

---


### Rbacrolebinding

Creates a Membership RBACRoleBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. When the rbacrolebinding was deleted. |
| `state` | String |  | Output only. State of the rbacrolebinding resource. |
| `user` | String |  | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `name` | String |  | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `create_time` | String |  | Output only. When the rbacrolebinding was created. |
| `group` | String |  | group is the group, as seen by the kubernetes cluster. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this RBACRolebinding. |
| `role` | String |  | Required. Role to bind to the principal |
| `update_time` | String |  | Output only. When the rbacrolebinding was last updated. |
| `parent` | String | ✅ | Required. The parent (project and location) where the RBACRoleBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. When the rbacrolebinding was deleted. |
| `state` | String | Output only. State of the rbacrolebinding resource. |
| `user` | String | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `name` | String | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `create_time` | String | Output only. When the rbacrolebinding was created. |
| `group` | String | group is the group, as seen by the kubernetes cluster. |
| `labels` | HashMap<String, String> | Optional. Labels for this RBACRolebinding. |
| `role` | String | Required. Role to bind to the principal |
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
rbacrolebinding_delete_time = rbacrolebinding.delete_time
rbacrolebinding_state = rbacrolebinding.state
rbacrolebinding_user = rbacrolebinding.user
rbacrolebinding_uid = rbacrolebinding.uid
rbacrolebinding_name = rbacrolebinding.name
rbacrolebinding_create_time = rbacrolebinding.create_time
rbacrolebinding_group = rbacrolebinding.group
rbacrolebinding_labels = rbacrolebinding.labels
rbacrolebinding_role = rbacrolebinding.role
rbacrolebinding_update_time = rbacrolebinding.update_time
```

---


### Scope

Creates a Scope.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. When the scope was created. |
| `delete_time` | String |  | Output only. When the scope was deleted. |
| `state` | String |  | Output only. State of the scope resource. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |
| `name` | String |  | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `update_time` | String |  | Output only. When the scope was last updated. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Scope. |
| `namespace_labels` | HashMap<String, String> |  | Optional. Scope-level cluster namespace labels. For the member clusters bound to the Scope, these labels are applied to each namespace under the Scope. Scope-level labels take precedence over Namespace-level labels (`namespace_labels` in the Fleet Namespace resource) if they share a key. Keys and values must be Kubernetes-conformant. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Scope will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. When the scope was created. |
| `delete_time` | String | Output only. When the scope was deleted. |
| `state` | String | Output only. State of the scope resource. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |
| `name` | String | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `update_time` | String | Output only. When the scope was last updated. |
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
scope_create_time = scope.create_time
scope_delete_time = scope.delete_time
scope_state = scope.state
scope_uid = scope.uid
scope_name = scope.name
scope_update_time = scope.update_time
scope_labels = scope.labels
scope_namespace_labels = scope.namespace_labels
```

---


### Namespace

Creates a fleet namespace.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the namespace resource. |
| `create_time` | String |  | Output only. When the namespace was created. |
| `update_time` | String |  | Output only. When the namespace was last updated. |
| `namespace_labels` | HashMap<String, String> |  | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `name` | String |  | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `delete_time` | String |  | Output only. When the namespace was deleted. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Namespace. |
| `scope` | String |  | Required. Scope associated with the namespace |
| `parent` | String | ✅ | Required. The parent (project and location) where the Namespace will be created. Specified in the format `projects/*/locations/*/scopes/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the namespace resource. |
| `create_time` | String | Output only. When the namespace was created. |
| `update_time` | String | Output only. When the namespace was last updated. |
| `namespace_labels` | HashMap<String, String> | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `name` | String | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `delete_time` | String | Output only. When the namespace was deleted. |
| `labels` | HashMap<String, String> | Optional. Labels for this Namespace. |
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
namespace_create_time = namespace.create_time
namespace_update_time = namespace.update_time
namespace_namespace_labels = namespace.namespace_labels
namespace_uid = namespace.uid
namespace_name = namespace.name
namespace_delete_time = namespace.delete_time
namespace_labels = namespace.labels
namespace_scope = namespace.scope
```

---


### Binding

Creates a MembershipBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. When the membership binding was created. |
| `delete_time` | String |  | Output only. When the membership binding was deleted. |
| `name` | String |  | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |
| `labels` | HashMap<String, String> |  | Optional. Labels for this MembershipBinding. |
| `scope` | String |  | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |
| `state` | String |  | Output only. State of the membership binding resource. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `update_time` | String |  | Output only. When the membership binding was last updated. |
| `parent` | String | ✅ | Required. The parent (project and location) where the MembershipBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. When the membership binding was created. |
| `delete_time` | String | Output only. When the membership binding was deleted. |
| `name` | String | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |
| `labels` | HashMap<String, String> | Optional. Labels for this MembershipBinding. |
| `scope` | String | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |
| `state` | String | Output only. State of the membership binding resource. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |
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
binding_create_time = binding.create_time
binding_delete_time = binding.delete_time
binding_name = binding.name
binding_labels = binding.labels
binding_scope = binding.scope
binding_state = binding.state
binding_uid = binding.uid
binding_update_time = binding.update_time
```

---


### Feature

Adds a new Feature.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scope_specs` | HashMap<String, String> |  | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `scope_states` | HashMap<String, String> |  | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `fleet_default_member_config` | String |  | Optional. Feature configuration applicable to all memberships of the fleet. |
| `delete_time` | String |  | Output only. When the Feature resource was deleted. |
| `unreachable` | Vec<String> |  | Output only. List of locations that could not be reached while fetching this feature. |
| `resource_state` | String |  | Output only. State of the Feature resource itself. |
| `labels` | HashMap<String, String> |  | Labels for this Feature. |
| `membership_specs` | HashMap<String, String> |  | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `create_time` | String |  | Output only. When the Feature resource was created. |
| `spec` | String |  | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `state` | String |  | Output only. The Fleet-wide Feature state. |
| `membership_states` | HashMap<String, String> |  | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `update_time` | String |  | Output only. When the Feature resource was last updated. |
| `name` | String |  | Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Feature will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scope_specs` | HashMap<String, String> | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `scope_states` | HashMap<String, String> | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `fleet_default_member_config` | String | Optional. Feature configuration applicable to all memberships of the fleet. |
| `delete_time` | String | Output only. When the Feature resource was deleted. |
| `unreachable` | Vec<String> | Output only. List of locations that could not be reached while fetching this feature. |
| `resource_state` | String | Output only. State of the Feature resource itself. |
| `labels` | HashMap<String, String> | Labels for this Feature. |
| `membership_specs` | HashMap<String, String> | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `create_time` | String | Output only. When the Feature resource was created. |
| `spec` | String | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `state` | String | Output only. The Fleet-wide Feature state. |
| `membership_states` | HashMap<String, String> | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `update_time` | String | Output only. When the Feature resource was last updated. |
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
feature_scope_specs = feature.scope_specs
feature_scope_states = feature.scope_states
feature_fleet_default_member_config = feature.fleet_default_member_config
feature_delete_time = feature.delete_time
feature_unreachable = feature.unreachable
feature_resource_state = feature.resource_state
feature_labels = feature.labels
feature_membership_specs = feature.membership_specs
feature_create_time = feature.create_time
feature_spec = feature.spec
feature_state = feature.state
feature_membership_states = feature.membership_states
feature_update_time = feature.update_time
feature_name = feature.name
```

---


### Fleet

Creates a fleet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_cluster_config` | String |  | Optional. The default cluster configurations to apply across the fleet. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Fleet. |
| `display_name` | String |  | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `create_time` | String |  | Output only. When the Fleet was created. |
| `name` | String |  | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |
| `state` | String |  | Output only. State of the namespace resource. |
| `delete_time` | String |  | Output only. When the Fleet was deleted. |
| `update_time` | String |  | Output only. When the Fleet was last updated. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Fleet will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_cluster_config` | String | Optional. The default cluster configurations to apply across the fleet. |
| `labels` | HashMap<String, String> | Optional. Labels for this Fleet. |
| `display_name` | String | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `create_time` | String | Output only. When the Fleet was created. |
| `name` | String | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |
| `state` | String | Output only. State of the namespace resource. |
| `delete_time` | String | Output only. When the Fleet was deleted. |
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
fleet_default_cluster_config = fleet.default_cluster_config
fleet_labels = fleet.labels
fleet_display_name = fleet.display_name
fleet_uid = fleet.uid
fleet_create_time = fleet.create_time
fleet_name = fleet.name
fleet_state = fleet.state
fleet_delete_time = fleet.delete_time
fleet_update_time = fleet.update_time
```

---


### Rbacrolebinding

Creates a Membership RBACRoleBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `state` | String |  | Output only. State of the rbacrolebinding resource. |
| `name` | String |  | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `create_time` | String |  | Output only. When the rbacrolebinding was created. |
| `user` | String |  | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |
| `delete_time` | String |  | Output only. When the rbacrolebinding was deleted. |
| `group` | String |  | group is the group, as seen by the kubernetes cluster. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this RBACRolebinding. |
| `update_time` | String |  | Output only. When the rbacrolebinding was last updated. |
| `role` | String |  | Required. Role to bind to the principal |
| `parent` | String | ✅ | Required. The parent (project and location) where the RBACRoleBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `state` | String | Output only. State of the rbacrolebinding resource. |
| `name` | String | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `create_time` | String | Output only. When the rbacrolebinding was created. |
| `user` | String | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |
| `delete_time` | String | Output only. When the rbacrolebinding was deleted. |
| `group` | String | group is the group, as seen by the kubernetes cluster. |
| `labels` | HashMap<String, String> | Optional. Labels for this RBACRolebinding. |
| `update_time` | String | Output only. When the rbacrolebinding was last updated. |
| `role` | String | Required. Role to bind to the principal |


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
rbacrolebinding_uid = rbacrolebinding.uid
rbacrolebinding_state = rbacrolebinding.state
rbacrolebinding_name = rbacrolebinding.name
rbacrolebinding_create_time = rbacrolebinding.create_time
rbacrolebinding_user = rbacrolebinding.user
rbacrolebinding_delete_time = rbacrolebinding.delete_time
rbacrolebinding_group = rbacrolebinding.group
rbacrolebinding_labels = rbacrolebinding.labels
rbacrolebinding_update_time = rbacrolebinding.update_time
rbacrolebinding_role = rbacrolebinding.role
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_metadata = location.metadata
location_labels = location.labels
location_name = location.name
```

---


### Membership

Creates a new Membership. **This is currently only supported for GKE clusters on Google Cloud**. To register other clusters, follow the instructions at https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `external_id` | String |  | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `endpoint` | String |  | Optional. Endpoint information to reach this member. |
| `membership_type` | String |  | Output only. The type of the membership. |
| `description` | String |  | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `delete_time` | String |  | Output only. When the Membership was deleted. |
| `last_connection_time` | String |  | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `update_time` | String |  | Output only. When the Membership was last updated. |
| `cluster_tier` | String |  | Output only. The tier of the cluster. |
| `create_time` | String |  | Output only. When the Membership was created. |
| `authority` | String |  | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `monitoring_config` | String |  | Optional. The monitoring config information for this membership. |
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `name` | String |  | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `state` | String |  | Output only. State of the Membership resource. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `external_id` | String | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `endpoint` | String | Optional. Endpoint information to reach this member. |
| `membership_type` | String | Output only. The type of the membership. |
| `description` | String | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `delete_time` | String | Output only. When the Membership was deleted. |
| `last_connection_time` | String | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `update_time` | String | Output only. When the Membership was last updated. |
| `cluster_tier` | String | Output only. The tier of the cluster. |
| `create_time` | String | Output only. When the Membership was created. |
| `authority` | String | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `monitoring_config` | String | Optional. The monitoring config information for this membership. |
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
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
membership_labels = membership.labels
membership_external_id = membership.external_id
membership_endpoint = membership.endpoint
membership_membership_type = membership.membership_type
membership_description = membership.description
membership_delete_time = membership.delete_time
membership_last_connection_time = membership.last_connection_time
membership_update_time = membership.update_time
membership_cluster_tier = membership.cluster_tier
membership_create_time = membership.create_time
membership_authority = membership.authority
membership_monitoring_config = membership.monitoring_config
membership_unique_id = membership.unique_id
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_name = operation.name
operation_done = operation.done
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
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_name = location.name
location_labels = location.labels
location_metadata = location.metadata
location_display_name = location.display_name
location_location_id = location.location_id
```

---


### Membership

Creates a new Membership. **This is currently only supported for GKE clusters on Google Cloud**. To register other clusters, follow the instructions at https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `monitoring_config` | String |  | Optional. The monitoring config information for this membership. |
| `name` | String |  | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `infrastructure_type` | String |  | Optional. The infrastructure type this Membership is running on. |
| `create_time` | String |  | Output only. When the Membership was created. |
| `endpoint` | String |  | Optional. Endpoint information to reach this member. |
| `external_id` | String |  | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. For GKE clusters, external_id is managed by the Hub API and updates will be ignored. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `authority` | String |  | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `delete_time` | String |  | Output only. When the Membership was deleted. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this membership. |
| `last_connection_time` | String |  | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `description` | String |  | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `state` | String |  | Output only. State of the Membership resource. |
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `update_time` | String |  | Output only. When the Membership was last updated. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `monitoring_config` | String | Optional. The monitoring config information for this membership. |
| `name` | String | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `infrastructure_type` | String | Optional. The infrastructure type this Membership is running on. |
| `create_time` | String | Output only. When the Membership was created. |
| `endpoint` | String | Optional. Endpoint information to reach this member. |
| `external_id` | String | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. For GKE clusters, external_id is managed by the Hub API and updates will be ignored. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `authority` | String | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `delete_time` | String | Output only. When the Membership was deleted. |
| `labels` | HashMap<String, String> | Optional. Labels for this membership. |
| `last_connection_time` | String | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `description` | String | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `state` | String | Output only. State of the Membership resource. |
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
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
membership_monitoring_config = membership.monitoring_config
membership_name = membership.name
membership_infrastructure_type = membership.infrastructure_type
membership_create_time = membership.create_time
membership_endpoint = membership.endpoint
membership_external_id = membership.external_id
membership_authority = membership.authority
membership_delete_time = membership.delete_time
membership_labels = membership.labels
membership_last_connection_time = membership.last_connection_time
membership_description = membership.description
membership_state = membership.state
membership_unique_id = membership.unique_id
membership_update_time = membership.update_time
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.gkehub_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_error = operation.error
operation_name = operation.name
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### Feature

Creates membershipFeature under a given parent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. When the MembershipFeature resource was deleted. |
| `update_time` | String |  | Output only. When the MembershipFeature resource was last updated. |
| `lifecycle_state` | String |  | Output only. Lifecycle information of the resource itself. |
| `labels` | HashMap<String, String> |  | GCP labels for this MembershipFeature. |
| `name` | String |  | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `state` | String |  | Output only. State of the this membershipFeature. |
| `create_time` | String |  | Output only. When the MembershipFeature resource was created. |
| `spec` | String |  | Optional. Spec of this membershipFeature. |
| `parent` | String | ✅ | Required. The name of parent where the MembershipFeature will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. When the MembershipFeature resource was deleted. |
| `update_time` | String | Output only. When the MembershipFeature resource was last updated. |
| `lifecycle_state` | String | Output only. Lifecycle information of the resource itself. |
| `labels` | HashMap<String, String> | GCP labels for this MembershipFeature. |
| `name` | String | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `state` | String | Output only. State of the this membershipFeature. |
| `create_time` | String | Output only. When the MembershipFeature resource was created. |
| `spec` | String | Optional. Spec of this membershipFeature. |


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
feature_update_time = feature.update_time
feature_lifecycle_state = feature.lifecycle_state
feature_labels = feature.labels
feature_name = feature.name
feature_state = feature.state
feature_create_time = feature.create_time
feature_spec = feature.spec
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_name = location.name
location_labels = location.labels
location_metadata = location.metadata
location_location_id = location.location_id
location_display_name = location.display_name
```

---


### Feature

Creates membershipFeature under a given parent.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. When the MembershipFeature resource was created. |
| `update_time` | String |  | Output only. When the MembershipFeature resource was last updated. |
| `labels` | HashMap<String, String> |  | GCP labels for this MembershipFeature. |
| `lifecycle_state` | String |  | Output only. Lifecycle information of the resource itself. |
| `name` | String |  | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `spec` | String |  | Optional. Spec of this membershipFeature. |
| `delete_time` | String |  | Output only. When the MembershipFeature resource was deleted. |
| `state` | String |  | Output only. State of the this membershipFeature. |
| `parent` | String | ✅ | Required. The name of parent where the MembershipFeature will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. When the MembershipFeature resource was created. |
| `update_time` | String | Output only. When the MembershipFeature resource was last updated. |
| `labels` | HashMap<String, String> | GCP labels for this MembershipFeature. |
| `lifecycle_state` | String | Output only. Lifecycle information of the resource itself. |
| `name` | String | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `spec` | String | Optional. Spec of this membershipFeature. |
| `delete_time` | String | Output only. When the MembershipFeature resource was deleted. |
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
feature_create_time = feature.create_time
feature_update_time = feature.update_time
feature_labels = feature.labels
feature_lifecycle_state = feature.lifecycle_state
feature_name = feature.name
feature_spec = feature.spec
feature_delete_time = feature.delete_time
feature_state = feature.state
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
```

---


### Membership

Creates a new Membership. **This is currently only supported for GKE clusters on Google Cloud**. To register other clusters, follow the instructions at https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `authority` | String |  | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `description` | String |  | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `create_time` | String |  | Output only. When the Membership was created. |
| `external_id` | String |  | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `membership_type` | String |  | Output only. The type of the membership. |
| `last_connection_time` | String |  | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `name` | String |  | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `state` | String |  | Output only. State of the Membership resource. |
| `monitoring_config` | String |  | Optional. The monitoring config information for this membership. |
| `cluster_tier` | String |  | Output only. The tier of the cluster. |
| `endpoint` | String |  | Optional. Endpoint information to reach this member. |
| `delete_time` | String |  | Output only. When the Membership was deleted. |
| `update_time` | String |  | Output only. When the Membership was last updated. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authority` | String | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `description` | String | Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes. |
| `labels` | HashMap<String, String> | Optional. Labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `create_time` | String | Output only. When the Membership was created. |
| `external_id` | String | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `membership_type` | String | Output only. The type of the membership. |
| `last_connection_time` | String | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `name` | String | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `state` | String | Output only. State of the Membership resource. |
| `monitoring_config` | String | Optional. The monitoring config information for this membership. |
| `cluster_tier` | String | Output only. The tier of the cluster. |
| `endpoint` | String | Optional. Endpoint information to reach this member. |
| `delete_time` | String | Output only. When the Membership was deleted. |
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
membership_authority = membership.authority
membership_description = membership.description
membership_labels = membership.labels
membership_create_time = membership.create_time
membership_external_id = membership.external_id
membership_membership_type = membership.membership_type
membership_last_connection_time = membership.last_connection_time
membership_name = membership.name
membership_unique_id = membership.unique_id
membership_state = membership.state
membership_monitoring_config = membership.monitoring_config
membership_cluster_tier = membership.cluster_tier
membership_endpoint = membership.endpoint
membership_delete_time = membership.delete_time
membership_update_time = membership.update_time
```

---


### Rbacrolebinding

Creates a Scope RBACRoleBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role` | String |  | Required. Role to bind to the principal |
| `update_time` | String |  | Output only. When the rbacrolebinding was last updated. |
| `user` | String |  | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |
| `delete_time` | String |  | Output only. When the rbacrolebinding was deleted. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this RBACRolebinding. |
| `group` | String |  | group is the group, as seen by the kubernetes cluster. |
| `name` | String |  | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `state` | String |  | Output only. State of the rbacrolebinding resource. |
| `create_time` | String |  | Output only. When the rbacrolebinding was created. |
| `parent` | String | ✅ | Required. The parent (project and location) where the RBACRoleBinding will be created. Specified in the format `projects/*/locations/*/scopes/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role` | String | Required. Role to bind to the principal |
| `update_time` | String | Output only. When the rbacrolebinding was last updated. |
| `user` | String | user is the name of the user as seen by the kubernetes cluster, example "alice" or "alice@domain.tld" |
| `delete_time` | String | Output only. When the rbacrolebinding was deleted. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all rbacrolebinding resources. If a rbacrolebinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `labels` | HashMap<String, String> | Optional. Labels for this RBACRolebinding. |
| `group` | String | group is the group, as seen by the kubernetes cluster. |
| `name` | String | The resource name for the rbacrolebinding `projects/{project}/locations/{location}/scopes/{scope}/rbacrolebindings/{rbacrolebinding}` or `projects/{project}/locations/{location}/memberships/{membership}/rbacrolebindings/{rbacrolebinding}` |
| `state` | String | Output only. State of the rbacrolebinding resource. |
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
    parent = "value"  # Required. The parent (project and location) where the RBACRoleBinding will be created. Specified in the format `projects/*/locations/*/scopes/*`.
}

# Access rbacrolebinding outputs
rbacrolebinding_id = rbacrolebinding.id
rbacrolebinding_role = rbacrolebinding.role
rbacrolebinding_update_time = rbacrolebinding.update_time
rbacrolebinding_user = rbacrolebinding.user
rbacrolebinding_delete_time = rbacrolebinding.delete_time
rbacrolebinding_uid = rbacrolebinding.uid
rbacrolebinding_labels = rbacrolebinding.labels
rbacrolebinding_group = rbacrolebinding.group
rbacrolebinding_name = rbacrolebinding.name
rbacrolebinding_state = rbacrolebinding.state
rbacrolebinding_create_time = rbacrolebinding.create_time
```

---


### Namespace

Creates a fleet namespace.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `namespace_labels` | HashMap<String, String> |  | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `state` | String |  | Output only. State of the namespace resource. |
| `delete_time` | String |  | Output only. When the namespace was deleted. |
| `update_time` | String |  | Output only. When the namespace was last updated. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Namespace. |
| `scope` | String |  | Required. Scope associated with the namespace |
| `name` | String |  | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `create_time` | String |  | Output only. When the namespace was created. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Namespace will be created. Specified in the format `projects/*/locations/*/scopes/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `namespace_labels` | HashMap<String, String> | Optional. Namespace-level cluster namespace labels. These labels are applied to the related namespace of the member clusters bound to the parent Scope. Scope-level labels (`namespace_labels` in the Fleet Scope resource) take precedence over Namespace-level labels if they share a key. Keys and values must be Kubernetes-conformant. |
| `state` | String | Output only. State of the namespace resource. |
| `delete_time` | String | Output only. When the namespace was deleted. |
| `update_time` | String | Output only. When the namespace was last updated. |
| `labels` | HashMap<String, String> | Optional. Labels for this Namespace. |
| `scope` | String | Required. Scope associated with the namespace |
| `name` | String | The resource name for the namespace `projects/{project}/locations/{location}/namespaces/{namespace}` |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all namespace resources. If a namespace resource is deleted and another resource with the same name is created, it gets a different uid. |
| `create_time` | String | Output only. When the namespace was created. |


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
namespace_namespace_labels = namespace.namespace_labels
namespace_state = namespace.state
namespace_delete_time = namespace.delete_time
namespace_update_time = namespace.update_time
namespace_labels = namespace.labels
namespace_scope = namespace.scope
namespace_name = namespace.name
namespace_uid = namespace.uid
namespace_create_time = namespace.create_time
```

---


### Feature

Adds a new Feature.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The Fleet-wide Feature state. |
| `update_time` | String |  | Output only. When the Feature resource was last updated. |
| `fleet_default_member_config` | String |  | Optional. Feature configuration applicable to all memberships of the fleet. |
| `delete_time` | String |  | Output only. When the Feature resource was deleted. |
| `labels` | HashMap<String, String> |  | Labels for this Feature. |
| `scope_specs` | HashMap<String, String> |  | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `create_time` | String |  | Output only. When the Feature resource was created. |
| `membership_states` | HashMap<String, String> |  | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `unreachable` | Vec<String> |  | Output only. List of locations that could not be reached while fetching this feature. |
| `name` | String |  | Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`. |
| `membership_specs` | HashMap<String, String> |  | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `scope_states` | HashMap<String, String> |  | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `spec` | String |  | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `resource_state` | String |  | Output only. State of the Feature resource itself. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Feature will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. The Fleet-wide Feature state. |
| `update_time` | String | Output only. When the Feature resource was last updated. |
| `fleet_default_member_config` | String | Optional. Feature configuration applicable to all memberships of the fleet. |
| `delete_time` | String | Output only. When the Feature resource was deleted. |
| `labels` | HashMap<String, String> | Labels for this Feature. |
| `scope_specs` | HashMap<String, String> | Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `create_time` | String | Output only. When the Feature resource was created. |
| `membership_states` | HashMap<String, String> | Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number. |
| `unreachable` | Vec<String> | Output only. List of locations that could not be reached while fetching this feature. |
| `name` | String | Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`. |
| `membership_specs` | HashMap<String, String> | Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature. |
| `scope_states` | HashMap<String, String> | Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. |
| `spec` | String | Optional. Fleet-wide Feature configuration. If this Feature does not support any Fleet-wide configuration, this field may be unused. |
| `resource_state` | String | Output only. State of the Feature resource itself. |


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
feature_state = feature.state
feature_update_time = feature.update_time
feature_fleet_default_member_config = feature.fleet_default_member_config
feature_delete_time = feature.delete_time
feature_labels = feature.labels
feature_scope_specs = feature.scope_specs
feature_create_time = feature.create_time
feature_membership_states = feature.membership_states
feature_unreachable = feature.unreachable
feature_name = feature.name
feature_membership_specs = feature.membership_specs
feature_scope_states = feature.scope_states
feature_spec = feature.spec
feature_resource_state = feature.resource_state
```

---


### Fleet

Creates a fleet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. When the Fleet was created. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Fleet. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `update_time` | String |  | Output only. When the Fleet was last updated. |
| `display_name` | String |  | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `default_cluster_config` | String |  | Optional. The default cluster configurations to apply across the fleet. |
| `delete_time` | String |  | Output only. When the Fleet was deleted. |
| `name` | String |  | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |
| `state` | String |  | Output only. State of the namespace resource. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Fleet will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. When the Fleet was created. |
| `labels` | HashMap<String, String> | Optional. Labels for this Fleet. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all Fleet resources. If a Fleet resource is deleted and another resource with the same name is created, it gets a different uid. |
| `update_time` | String | Output only. When the Fleet was last updated. |
| `display_name` | String | Optional. A user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters. Allowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point. Example: `Production Fleet` |
| `default_cluster_config` | String | Optional. The default cluster configurations to apply across the fleet. |
| `delete_time` | String | Output only. When the Fleet was deleted. |
| `name` | String | Output only. The full, unique resource name of this fleet in the format of `projects/{project}/locations/{location}/fleets/{fleet}`. Each Google Cloud project can have at most one fleet resource, named "default". |
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
fleet_create_time = fleet.create_time
fleet_labels = fleet.labels
fleet_uid = fleet.uid
fleet_update_time = fleet.update_time
fleet_display_name = fleet.display_name
fleet_default_cluster_config = fleet.default_cluster_config
fleet_delete_time = fleet.delete_time
fleet_name = fleet.name
fleet_state = fleet.state
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
operation = provider.gkehub_api.Operation {
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_name = location.name
location_location_id = location.location_id
location_labels = location.labels
location_display_name = location.display_name
```

---


### Scope

Creates a Scope.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this Scope. |
| `delete_time` | String |  | Output only. When the scope was deleted. |
| `create_time` | String |  | Output only. When the scope was created. |
| `namespace_labels` | HashMap<String, String> |  | Optional. Scope-level cluster namespace labels. For the member clusters bound to the Scope, these labels are applied to each namespace under the Scope. Scope-level labels take precedence over Namespace-level labels (`namespace_labels` in the Fleet Namespace resource) if they share a key. Keys and values must be Kubernetes-conformant. |
| `state` | String |  | Output only. State of the scope resource. |
| `name` | String |  | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `update_time` | String |  | Output only. When the scope was last updated. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Scope will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all scope resources. If a scope resource is deleted and another resource with the same name is created, it gets a different uid. |
| `labels` | HashMap<String, String> | Optional. Labels for this Scope. |
| `delete_time` | String | Output only. When the scope was deleted. |
| `create_time` | String | Output only. When the scope was created. |
| `namespace_labels` | HashMap<String, String> | Optional. Scope-level cluster namespace labels. For the member clusters bound to the Scope, these labels are applied to each namespace under the Scope. Scope-level labels take precedence over Namespace-level labels (`namespace_labels` in the Fleet Namespace resource) if they share a key. Keys and values must be Kubernetes-conformant. |
| `state` | String | Output only. State of the scope resource. |
| `name` | String | The resource name for the scope `projects/{project}/locations/{location}/scopes/{scope}` |
| `update_time` | String | Output only. When the scope was last updated. |


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
scope_uid = scope.uid
scope_labels = scope.labels
scope_delete_time = scope.delete_time
scope_create_time = scope.create_time
scope_namespace_labels = scope.namespace_labels
scope_state = scope.state
scope_name = scope.name
scope_update_time = scope.update_time
```

---


### Binding

Creates a MembershipBinding.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `delete_time` | String |  | Output only. When the membership binding was deleted. |
| `update_time` | String |  | Output only. When the membership binding was last updated. |
| `uid` | String |  | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `create_time` | String |  | Output only. When the membership binding was created. |
| `state` | String |  | Output only. State of the membership binding resource. |
| `labels` | HashMap<String, String> |  | Optional. Labels for this MembershipBinding. |
| `name` | String |  | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |
| `scope` | String |  | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |
| `parent` | String | ✅ | Required. The parent (project and location) where the MembershipBinding will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. When the membership binding was deleted. |
| `update_time` | String | Output only. When the membership binding was last updated. |
| `uid` | String | Output only. Google-generated UUID for this resource. This is unique across all membershipbinding resources. If a membershipbinding resource is deleted and another resource with the same name is created, it gets a different uid. |
| `create_time` | String | Output only. When the membership binding was created. |
| `state` | String | Output only. State of the membership binding resource. |
| `labels` | HashMap<String, String> | Optional. Labels for this MembershipBinding. |
| `name` | String | The resource name for the membershipbinding itself `projects/{project}/locations/{location}/memberships/{membership}/bindings/{membershipbinding}` |
| `scope` | String | A Scope resource name in the format `projects/*/locations/*/scopes/*`. |


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
binding_delete_time = binding.delete_time
binding_update_time = binding.update_time
binding_uid = binding.uid
binding_create_time = binding.create_time
binding_state = binding.state
binding_labels = binding.labels
binding_name = binding.name
binding_scope = binding.scope
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
```

---


### Membership

Creates a new Membership. **This is currently only supported for GKE clusters on Google Cloud**. To register other clusters, follow the instructions at https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. State of the Membership resource. |
| `authority` | String |  | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `monitoring_config` | String |  | Optional. The monitoring config information for this membership. |
| `create_time` | String |  | Output only. When the Membership was created. |
| `update_time` | String |  | Output only. When the Membership was last updated. |
| `last_connection_time` | String |  | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `infrastructure_type` | String |  | Optional. The infrastructure type this Membership is running on. |
| `membership_type` | String |  | Output only. The type of the membership. |
| `external_id` | String |  | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. For GKE clusters, external_id is managed by the Hub API and updates will be ignored. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `description` | String |  | Optional. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` |
| `endpoint` | String |  | Optional. Endpoint information to reach this member. |
| `name` | String |  | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `delete_time` | String |  | Output only. When the Membership was deleted. |
| `labels` | HashMap<String, String> |  | Optional. GCP labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `unique_id` | String |  | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |
| `parent` | String | ✅ | Required. The parent (project and location) where the Memberships will be created. Specified in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. State of the Membership resource. |
| `authority` | String | Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity |
| `monitoring_config` | String | Optional. The monitoring config information for this membership. |
| `create_time` | String | Output only. When the Membership was created. |
| `update_time` | String | Output only. When the Membership was last updated. |
| `last_connection_time` | String | Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset. |
| `infrastructure_type` | String | Optional. The infrastructure type this Membership is running on. |
| `membership_type` | String | Output only. The type of the membership. |
| `external_id` | String | Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. For GKE clusters, external_id is managed by the Hub API and updates will be ignored. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object. |
| `description` | String | Optional. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` |
| `endpoint` | String | Optional. Endpoint information to reach this member. |
| `name` | String | Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters. |
| `delete_time` | String | Output only. When the Membership was deleted. |
| `labels` | HashMap<String, String> | Optional. GCP labels for this membership. These labels are not leveraged by multi-cluster features, instead, we prefer cluster labels, which can be set on GKE cluster or other cluster types. |
| `unique_id` | String | Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id. |


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
membership_state = membership.state
membership_authority = membership.authority
membership_monitoring_config = membership.monitoring_config
membership_create_time = membership.create_time
membership_update_time = membership.update_time
membership_last_connection_time = membership.last_connection_time
membership_infrastructure_type = membership.infrastructure_type
membership_membership_type = membership.membership_type
membership_external_id = membership.external_id
membership_description = membership.description
membership_endpoint = membership.endpoint
membership_name = membership.name
membership_delete_time = membership.delete_time
membership_labels = membership.labels
membership_unique_id = membership.unique_id
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_metadata = location.metadata
location_display_name = location.display_name
location_labels = location.labels
location_name = location.name
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
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
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
location_labels = location.labels
location_display_name = location.display_name
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation_error = operation.error
operation_done = operation.done
operation_response = operation.response
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
| `name` | String |  | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `create_time` | String |  | Output only. When the MembershipFeature resource was created. |
| `labels` | HashMap<String, String> |  | GCP labels for this MembershipFeature. |
| `spec` | String |  | Optional. Spec of this membershipFeature. |
| `delete_time` | String |  | Output only. When the MembershipFeature resource was deleted. |
| `update_time` | String |  | Output only. When the MembershipFeature resource was last updated. |
| `state` | String |  | Output only. State of the this membershipFeature. |
| `parent` | String | ✅ | Required. The name of parent where the MembershipFeature will be created. Specified in the format `projects/*/locations/*/memberships/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `lifecycle_state` | String | Output only. Lifecycle information of the resource itself. |
| `name` | String | Output only. The resource name of the membershipFeature, in the format: `projects/{project}/locations/{location}/memberships/{membership}/features/{feature}`. Note that `membershipFeatures` is shortened to `features` in the resource name. (see http://go/aip/122#collection-identifiers) |
| `create_time` | String | Output only. When the MembershipFeature resource was created. |
| `labels` | HashMap<String, String> | GCP labels for this MembershipFeature. |
| `spec` | String | Optional. Spec of this membershipFeature. |
| `delete_time` | String | Output only. When the MembershipFeature resource was deleted. |
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
feature_lifecycle_state = feature.lifecycle_state
feature_name = feature.name
feature_create_time = feature.create_time
feature_labels = feature.labels
feature_spec = feature.spec
feature_delete_time = feature.delete_time
feature_update_time = feature.update_time
feature_state = feature.state
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple membership resources
membership_0 = provider.gkehub_api.Membership {
    parent = "value-0"
}
membership_1 = provider.gkehub_api.Membership {
    parent = "value-1"
}
membership_2 = provider.gkehub_api.Membership {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    membership = provider.gkehub_api.Membership {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Gkehub_api Documentation](https://cloud.google.com/gkehub_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
