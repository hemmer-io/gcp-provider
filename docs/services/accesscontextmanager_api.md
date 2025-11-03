# Accesscontextmanager_api Service



**Resources**: 11

---

## Overview

The accesscontextmanager_api service provides access to 11 resource types:

- [Operation](#operation) [CRD]
- [Authorized_orgs_desc](#authorized_orgs_desc) [CRUD]
- [Service_perimeter](#service_perimeter) [CRUD]
- [Gcp_user_access_binding](#gcp_user_access_binding) [CRUD]
- [Service](#service) [R]
- [Access_level](#access_level) [CRUD]
- [Access_policie](#access_policie) [CRUD]
- [Access_policie](#access_policie) [CRUD]
- [Service_perimeter](#service_perimeter) [CRUD]
- [Operation](#operation) [R]
- [Access_level](#access_level) [CRUD]

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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.accesscontextmanager_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
operation_error = operation.error
operation_response = operation.response
```

---


### Authorized_orgs_desc

Creates an authorized orgs desc. The long-running operation from this RPC has a successful status after the authorized orgs desc propagates to long-lasting storage. If a authorized orgs desc contains errors, an error response is returned for the first error encountered. The name of this `AuthorizedOrgsDesc` will be assigned during creation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_type` | String |  | The asset type of this authorized orgs desc. Valid values are `ASSET_TYPE_DEVICE`, and `ASSET_TYPE_CREDENTIAL_STRENGTH`. |
| `authorization_direction` | String |  | The direction of the authorization relationship between this organization and the organizations listed in the `orgs` field. The valid values for this field include the following: `AUTHORIZATION_DIRECTION_FROM`: Allows this organization to evaluate traffic in the organizations listed in the `orgs` field. `AUTHORIZATION_DIRECTION_TO`: Allows the organizations listed in the `orgs` field to evaluate the traffic in this organization. For the authorization relationship to take effect, all of the organizations must authorize and specify the appropriate relationship direction. For example, if organization A authorized organization B and C to evaluate its traffic, by specifying `AUTHORIZATION_DIRECTION_TO` as the authorization direction, organizations B and C must specify `AUTHORIZATION_DIRECTION_FROM` as the authorization direction in their `AuthorizedOrgsDesc` resource. |
| `name` | String |  | Identifier. Resource name for the `AuthorizedOrgsDesc`. Format: `accessPolicies/{access_policy}/authorizedOrgsDescs/{authorized_orgs_desc}`. The `authorized_orgs_desc` component must begin with a letter, followed by alphanumeric characters or `_`. After you create an `AuthorizedOrgsDesc`, you cannot change its `name`. |
| `authorization_type` | String |  | A granular control type for authorization levels. Valid value is `AUTHORIZATION_TYPE_TRUST`. |
| `orgs` | Vec<String> |  | The list of organization ids in this AuthorizedOrgsDesc. Format: `organizations/` Example: `organizations/123456` |
| `parent` | String | ✅ | Required. Resource name for the access policy which owns this Authorized Orgs Desc. Format: `accessPolicies/{policy_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `asset_type` | String | The asset type of this authorized orgs desc. Valid values are `ASSET_TYPE_DEVICE`, and `ASSET_TYPE_CREDENTIAL_STRENGTH`. |
| `authorization_direction` | String | The direction of the authorization relationship between this organization and the organizations listed in the `orgs` field. The valid values for this field include the following: `AUTHORIZATION_DIRECTION_FROM`: Allows this organization to evaluate traffic in the organizations listed in the `orgs` field. `AUTHORIZATION_DIRECTION_TO`: Allows the organizations listed in the `orgs` field to evaluate the traffic in this organization. For the authorization relationship to take effect, all of the organizations must authorize and specify the appropriate relationship direction. For example, if organization A authorized organization B and C to evaluate its traffic, by specifying `AUTHORIZATION_DIRECTION_TO` as the authorization direction, organizations B and C must specify `AUTHORIZATION_DIRECTION_FROM` as the authorization direction in their `AuthorizedOrgsDesc` resource. |
| `name` | String | Identifier. Resource name for the `AuthorizedOrgsDesc`. Format: `accessPolicies/{access_policy}/authorizedOrgsDescs/{authorized_orgs_desc}`. The `authorized_orgs_desc` component must begin with a letter, followed by alphanumeric characters or `_`. After you create an `AuthorizedOrgsDesc`, you cannot change its `name`. |
| `authorization_type` | String | A granular control type for authorization levels. Valid value is `AUTHORIZATION_TYPE_TRUST`. |
| `orgs` | Vec<String> | The list of organization ids in this AuthorizedOrgsDesc. Format: `organizations/` Example: `organizations/123456` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authorized_orgs_desc
authorized_orgs_desc = provider.accesscontextmanager_api.Authorized_orgs_desc {
    parent = "value"  # Required. Resource name for the access policy which owns this Authorized Orgs Desc. Format: `accessPolicies/{policy_id}`
}

# Access authorized_orgs_desc outputs
authorized_orgs_desc_id = authorized_orgs_desc.id
authorized_orgs_desc_asset_type = authorized_orgs_desc.asset_type
authorized_orgs_desc_authorization_direction = authorized_orgs_desc.authorization_direction
authorized_orgs_desc_name = authorized_orgs_desc.name
authorized_orgs_desc_authorization_type = authorized_orgs_desc.authorization_type
authorized_orgs_desc_orgs = authorized_orgs_desc.orgs
```

---


### Service_perimeter

Creates a service perimeter. The long-running operation from this RPC has a successful status after the service perimeter propagates to long-lasting storage. If a service perimeter contains errors, an error response is returned for the first error encountered.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Resource name for the `ServicePerimeter`. Format: `accessPolicies/{access_policy}/servicePerimeters/{service_perimeter}`. The `service_perimeter` component must begin with a letter, followed by alphanumeric characters or `_`. After you create a `ServicePerimeter`, you cannot change its `name`. |
| `spec` | String |  | Proposed (or dry run) ServicePerimeter configuration. This configuration allows to specify and test ServicePerimeter configuration without enforcing actual access restrictions. Only allowed to be set when the "use_explicit_dry_run_spec" flag is set. |
| `description` | String |  | Description of the `ServicePerimeter` and its use. Does not affect behavior. |
| `perimeter_type` | String |  | Perimeter type indicator. A single project or VPC network is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty. |
| `use_explicit_dry_run_spec` | bool |  | Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists for all Service Perimeters, and that spec is identical to the status for those Service Perimeters. When this flag is set, it inhibits the generation of the implicit spec, thereby allowing the user to explicitly provide a configuration ("spec") to use in a dry-run version of the Service Perimeter. This allows the user to test changes to the enforced config ("status") without actually enforcing them. This testing is done through analyzing the differences between currently enforced and suggested restrictions. use_explicit_dry_run_spec must bet set to True if any of the fields in the spec are set to non-default values. |
| `status` | String |  | Current ServicePerimeter configuration. Specifies sets of resources, restricted services and access levels that determine perimeter content and boundaries. |
| `title` | String |  | Human readable title. Must be unique within the Policy. |
| `etag` | String |  | Optional. An opaque identifier for the current version of the `ServicePerimeter`. This identifier does not follow any specific format. If an etag is not provided, the operation will be performed as if a valid etag is provided. |
| `parent` | String | ✅ | Required. Resource name for the access policy which owns this Service Perimeter. Format: `accessPolicies/{policy_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Resource name for the `ServicePerimeter`. Format: `accessPolicies/{access_policy}/servicePerimeters/{service_perimeter}`. The `service_perimeter` component must begin with a letter, followed by alphanumeric characters or `_`. After you create a `ServicePerimeter`, you cannot change its `name`. |
| `spec` | String | Proposed (or dry run) ServicePerimeter configuration. This configuration allows to specify and test ServicePerimeter configuration without enforcing actual access restrictions. Only allowed to be set when the "use_explicit_dry_run_spec" flag is set. |
| `description` | String | Description of the `ServicePerimeter` and its use. Does not affect behavior. |
| `perimeter_type` | String | Perimeter type indicator. A single project or VPC network is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, the restricted service list as well as access level lists must be empty. |
| `use_explicit_dry_run_spec` | bool | Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists for all Service Perimeters, and that spec is identical to the status for those Service Perimeters. When this flag is set, it inhibits the generation of the implicit spec, thereby allowing the user to explicitly provide a configuration ("spec") to use in a dry-run version of the Service Perimeter. This allows the user to test changes to the enforced config ("status") without actually enforcing them. This testing is done through analyzing the differences between currently enforced and suggested restrictions. use_explicit_dry_run_spec must bet set to True if any of the fields in the spec are set to non-default values. |
| `status` | String | Current ServicePerimeter configuration. Specifies sets of resources, restricted services and access levels that determine perimeter content and boundaries. |
| `title` | String | Human readable title. Must be unique within the Policy. |
| `etag` | String | Optional. An opaque identifier for the current version of the `ServicePerimeter`. This identifier does not follow any specific format. If an etag is not provided, the operation will be performed as if a valid etag is provided. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_perimeter
service_perimeter = provider.accesscontextmanager_api.Service_perimeter {
    parent = "value"  # Required. Resource name for the access policy which owns this Service Perimeter. Format: `accessPolicies/{policy_id}`
}

# Access service_perimeter outputs
service_perimeter_id = service_perimeter.id
service_perimeter_name = service_perimeter.name
service_perimeter_spec = service_perimeter.spec
service_perimeter_description = service_perimeter.description
service_perimeter_perimeter_type = service_perimeter.perimeter_type
service_perimeter_use_explicit_dry_run_spec = service_perimeter.use_explicit_dry_run_spec
service_perimeter_status = service_perimeter.status
service_perimeter_title = service_perimeter.title
service_perimeter_etag = service_perimeter.etag
```

---


### Gcp_user_access_binding

Creates a GcpUserAccessBinding. If the client specifies a name, the server ignores it. Fails if a resource already exists with the same group_key. Completion of this long-running operation does not necessarily signify that the new binding is deployed onto all affected users, which may take more time.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `access_levels` | Vec<String> |  | Optional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: "accessPolicies/9522/accessLevels/device_trusted" |
| `dry_run_access_levels` | Vec<String> |  | Optional. Dry run access level that will be evaluated but will not be enforced. The access denial based on dry run policy will be logged. Only one access level is supported, not multiple. This list must have exactly one element. Example: "accessPolicies/9522/accessLevels/device_trusted" |
| `name` | String |  | Immutable. Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by [RFC 3986 Section 2.3](https://tools.ietf.org/html/rfc3986#section-2.3)). Should not be specified by the client during creation. Example: "organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N" |
| `restricted_client_applications` | Vec<String> |  | Optional. A list of applications that are subject to this binding's restrictions. If the list is empty, the binding restrictions will universally apply to all applications. |
| `group_key` | String |  | Optional. Immutable. Google Group id whose users are subject to this binding's restrictions. See "id" in the [Google Workspace Directory API's Group Resource] (https://developers.google.com/admin-sdk/directory/v1/reference/groups#resource). If a group's email address/alias is changed, this resource will continue to point at the changed group. This field does not accept group email addresses or aliases. Example: "01d520gv4vjcrht" |
| `scoped_access_settings` | Vec<String> |  | Optional. A list of scoped access settings that set this binding's restrictions on a subset of applications. This field cannot be set if restricted_client_applications is set. |
| `session_settings` | String |  | Optional. The Google Cloud session length (GCSL) policy for the group key. |
| `parent` | String | ✅ | Required. Example: "organizations/256" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_levels` | Vec<String> | Optional. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: "accessPolicies/9522/accessLevels/device_trusted" |
| `dry_run_access_levels` | Vec<String> | Optional. Dry run access level that will be evaluated but will not be enforced. The access denial based on dry run policy will be logged. Only one access level is supported, not multiple. This list must have exactly one element. Example: "accessPolicies/9522/accessLevels/device_trusted" |
| `name` | String | Immutable. Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by [RFC 3986 Section 2.3](https://tools.ietf.org/html/rfc3986#section-2.3)). Should not be specified by the client during creation. Example: "organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N" |
| `restricted_client_applications` | Vec<String> | Optional. A list of applications that are subject to this binding's restrictions. If the list is empty, the binding restrictions will universally apply to all applications. |
| `group_key` | String | Optional. Immutable. Google Group id whose users are subject to this binding's restrictions. See "id" in the [Google Workspace Directory API's Group Resource] (https://developers.google.com/admin-sdk/directory/v1/reference/groups#resource). If a group's email address/alias is changed, this resource will continue to point at the changed group. This field does not accept group email addresses or aliases. Example: "01d520gv4vjcrht" |
| `scoped_access_settings` | Vec<String> | Optional. A list of scoped access settings that set this binding's restrictions on a subset of applications. This field cannot be set if restricted_client_applications is set. |
| `session_settings` | String | Optional. The Google Cloud session length (GCSL) policy for the group key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gcp_user_access_binding
gcp_user_access_binding = provider.accesscontextmanager_api.Gcp_user_access_binding {
    parent = "value"  # Required. Example: "organizations/256"
}

# Access gcp_user_access_binding outputs
gcp_user_access_binding_id = gcp_user_access_binding.id
gcp_user_access_binding_access_levels = gcp_user_access_binding.access_levels
gcp_user_access_binding_dry_run_access_levels = gcp_user_access_binding.dry_run_access_levels
gcp_user_access_binding_name = gcp_user_access_binding.name
gcp_user_access_binding_restricted_client_applications = gcp_user_access_binding.restricted_client_applications
gcp_user_access_binding_group_key = gcp_user_access_binding.group_key
gcp_user_access_binding_scoped_access_settings = gcp_user_access_binding.scoped_access_settings
gcp_user_access_binding_session_settings = gcp_user_access_binding.session_settings
```

---


### Service

Returns a VPC-SC supported service based on the service name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `title` | String | The name of the supported product, such as 'Cloud Product API'. |
| `support_stage` | String | The support stage of the service. |
| `service_support_stage` | String | The support stage of the service. |
| `supported_methods` | Vec<String> | The list of the supported methods. This field exists only in response to GetSupportedService |
| `known_limitations` | bool | True if the service is supported with some limitations. Check [documentation](https://cloud.google.com/vpc-service-controls/docs/supported-products) for details. |
| `available_on_restricted_vip` | bool | True if the service is available on the restricted VIP. Services on the restricted VIP typically either support VPC Service Controls or are core infrastructure services required for the functioning of Google Cloud. |
| `name` | String | The service name or address of the supported service, such as `service.googleapis.com`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access service outputs
service_id = service.id
service_title = service.title
service_support_stage = service.support_stage
service_service_support_stage = service.service_support_stage
service_supported_methods = service.supported_methods
service_known_limitations = service.known_limitations
service_available_on_restricted_vip = service.available_on_restricted_vip
service_name = service.name
```

---


### Access_level

Creates an access level. The long-running operation from this RPC has a successful status after the access level propagates to long-lasting storage. If access levels contain errors, an error response is returned for the first error encountered.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Description of the `AccessLevel` and its use. Does not affect behavior. |
| `custom` | String |  | A `CustomLevel` written in the Common Expression Language. |
| `name` | String |  | Identifier. Resource name for the `AccessLevel`. Format: `accessPolicies/{access_policy}/accessLevels/{access_level}`. The `access_level` component must begin with a letter, followed by alphanumeric characters or `_`. Its maximum length is 50 characters. After you create an `AccessLevel`, you cannot change its `name`. |
| `title` | String |  | Human readable title. Must be unique within the Policy. |
| `basic` | String |  | A `BasicLevel` composed of `Conditions`. |
| `parent` | String | ✅ | Required. Resource name for the access policy which owns this Access Level. Format: `accessPolicies/{policy_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Description of the `AccessLevel` and its use. Does not affect behavior. |
| `custom` | String | A `CustomLevel` written in the Common Expression Language. |
| `name` | String | Identifier. Resource name for the `AccessLevel`. Format: `accessPolicies/{access_policy}/accessLevels/{access_level}`. The `access_level` component must begin with a letter, followed by alphanumeric characters or `_`. Its maximum length is 50 characters. After you create an `AccessLevel`, you cannot change its `name`. |
| `title` | String | Human readable title. Must be unique within the Policy. |
| `basic` | String | A `BasicLevel` composed of `Conditions`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create access_level
access_level = provider.accesscontextmanager_api.Access_level {
    parent = "value"  # Required. Resource name for the access policy which owns this Access Level. Format: `accessPolicies/{policy_id}`
}

# Access access_level outputs
access_level_id = access_level.id
access_level_description = access_level.description
access_level_custom = access_level.custom
access_level_name = access_level.name
access_level_title = access_level.title
access_level_basic = access_level.basic
```

---


### Access_policie

Creates an access policy. This method fails if the organization already has an access policy. The long-running operation has a successful status after the access policy propagates to long-lasting storage. Syntactic and basic semantic errors are returned in `metadata` as a BadRequest proto.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Identifier. Resource name of the `AccessPolicy`. Format: `accessPolicies/{access_policy}` |
| `etag` | String |  | Output only. An opaque identifier for the current version of the `AccessPolicy`. This will always be a strongly validated etag, meaning that two Access Policies will be identical if and only if their etags are identical. Clients should not expect this to be in any specific format. |
| `scopes` | Vec<String> |  | The scopes of the AccessPolicy. Scopes define which resources a policy can restrict and where its resources can be referenced. For example, policy A with `scopes=["folders/123"]` has the following behavior: - ServicePerimeter can only restrict projects within `folders/123`. - ServicePerimeter within policy A can only reference access levels defined within policy A. - Only one policy can include a given scope; thus, attempting to create a second policy which includes `folders/123` will result in an error. If no scopes are provided, then any resource within the organization can be restricted. Scopes cannot be modified after a policy is created. Policies can only have a single scope. Format: list of `folders/{folder_number}` or `projects/{project_number}` |
| `parent` | String |  | Required. The parent of this `AccessPolicy` in the Cloud Resource Hierarchy. Currently immutable once created. Format: `organizations/{organization_id}` |
| `title` | String |  | Required. Human readable title. Does not affect behavior. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Identifier. Resource name of the `AccessPolicy`. Format: `accessPolicies/{access_policy}` |
| `etag` | String | Output only. An opaque identifier for the current version of the `AccessPolicy`. This will always be a strongly validated etag, meaning that two Access Policies will be identical if and only if their etags are identical. Clients should not expect this to be in any specific format. |
| `scopes` | Vec<String> | The scopes of the AccessPolicy. Scopes define which resources a policy can restrict and where its resources can be referenced. For example, policy A with `scopes=["folders/123"]` has the following behavior: - ServicePerimeter can only restrict projects within `folders/123`. - ServicePerimeter within policy A can only reference access levels defined within policy A. - Only one policy can include a given scope; thus, attempting to create a second policy which includes `folders/123` will result in an error. If no scopes are provided, then any resource within the organization can be restricted. Scopes cannot be modified after a policy is created. Policies can only have a single scope. Format: list of `folders/{folder_number}` or `projects/{project_number}` |
| `parent` | String | Required. The parent of this `AccessPolicy` in the Cloud Resource Hierarchy. Currently immutable once created. Format: `organizations/{organization_id}` |
| `title` | String | Required. Human readable title. Does not affect behavior. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create access_policie
access_policie = provider.accesscontextmanager_api.Access_policie {
}

# Access access_policie outputs
access_policie_id = access_policie.id
access_policie_name = access_policie.name
access_policie_etag = access_policie.etag
access_policie_scopes = access_policie.scopes
access_policie_parent = access_policie.parent
access_policie_title = access_policie.title
```

---


### Access_policie

Create an `AccessPolicy`. Fails if this organization already has a `AccessPolicy`. The longrunning Operation will have a successful status once the `AccessPolicy` has propagated to long-lasting storage. Syntactic and basic semantic errors will be returned in `metadata` as a BadRequest proto.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String |  | Required. The parent of this `AccessPolicy` in the Cloud Resource Hierarchy. Currently immutable once created. Format: `organizations/{organization_id}` |
| `name` | String |  | Output only. Resource name of the `AccessPolicy`. Format: `accessPolicies/{policy_id}` |
| `title` | String |  | Required. Human readable title. Does not affect behavior. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parent` | String | Required. The parent of this `AccessPolicy` in the Cloud Resource Hierarchy. Currently immutable once created. Format: `organizations/{organization_id}` |
| `name` | String | Output only. Resource name of the `AccessPolicy`. Format: `accessPolicies/{policy_id}` |
| `title` | String | Required. Human readable title. Does not affect behavior. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create access_policie
access_policie = provider.accesscontextmanager_api.Access_policie {
}

# Access access_policie outputs
access_policie_id = access_policie.id
access_policie_parent = access_policie.parent
access_policie_name = access_policie.name
access_policie_title = access_policie.title
```

---


### Service_perimeter

Create a Service Perimeter. The longrunning operation from this RPC will have a successful status once the Service Perimeter has propagated to long-lasting storage. Service Perimeters containing errors will result in an error response for the first error encountered.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Resource name for the `ServicePerimeter`. Format: `accessPolicies/{access_policy}/servicePerimeters/{service_perimeter}`. The `service_perimeter` component must begin with a letter, followed by alphanumeric characters or `_`. After you create a `ServicePerimeter`, you cannot change its `name`. |
| `description` | String |  | Description of the `ServicePerimeter` and its use. Does not affect behavior. |
| `status` | String |  | Current ServicePerimeter configuration. Specifies sets of resources, restricted/unrestricted services and access levels that determine perimeter content and boundaries. |
| `title` | String |  | Human readable title. Must be unique within the Policy. |
| `perimeter_type` | String |  | Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, restricted/unrestricted service lists as well as access lists must be empty. |
| `parent` | String | ✅ | Required. Resource name for the access policy which owns this Service Perimeter. Format: `accessPolicies/{policy_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the `ServicePerimeter`. Format: `accessPolicies/{access_policy}/servicePerimeters/{service_perimeter}`. The `service_perimeter` component must begin with a letter, followed by alphanumeric characters or `_`. After you create a `ServicePerimeter`, you cannot change its `name`. |
| `description` | String | Description of the `ServicePerimeter` and its use. Does not affect behavior. |
| `status` | String | Current ServicePerimeter configuration. Specifies sets of resources, restricted/unrestricted services and access levels that determine perimeter content and boundaries. |
| `title` | String | Human readable title. Must be unique within the Policy. |
| `perimeter_type` | String | Perimeter type indicator. A single project is allowed to be a member of single regular perimeter, but multiple service perimeter bridges. A project cannot be a included in a perimeter bridge without being included in regular perimeter. For perimeter bridges, restricted/unrestricted service lists as well as access lists must be empty. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_perimeter
service_perimeter = provider.accesscontextmanager_api.Service_perimeter {
    parent = "value"  # Required. Resource name for the access policy which owns this Service Perimeter. Format: `accessPolicies/{policy_id}`
}

# Access service_perimeter outputs
service_perimeter_id = service_perimeter.id
service_perimeter_name = service_perimeter.name
service_perimeter_description = service_perimeter.description
service_perimeter_status = service_perimeter.status
service_perimeter_title = service_perimeter.title
service_perimeter_perimeter_type = service_perimeter.perimeter_type
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


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

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Access_level

Create an Access Level. The longrunning operation from this RPC will have a successful status once the Access Level has propagated to long-lasting storage. Access Levels containing errors will result in an error response for the first error encountered.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String |  | Human readable title. Must be unique within the Policy. |
| `name` | String |  | Resource name for the `AccessLevel`. Format: `accessPolicies/{access_policy}/accessLevels/{access_level}`. The `access_level` component must begin with a letter, followed by alphanumeric characters or `_`. Its maximum length is 50 characters. After you create an `AccessLevel`, you cannot change its `name`. |
| `description` | String |  | Description of the `AccessLevel` and its use. Does not affect behavior. |
| `basic` | String |  | A `BasicLevel` composed of `Conditions`. |
| `custom` | String |  | A `CustomLevel` written in the Common Expression Language. |
| `parent` | String | ✅ | Required. Resource name for the access policy which owns this Access Level. Format: `accessPolicies/{policy_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `title` | String | Human readable title. Must be unique within the Policy. |
| `name` | String | Resource name for the `AccessLevel`. Format: `accessPolicies/{access_policy}/accessLevels/{access_level}`. The `access_level` component must begin with a letter, followed by alphanumeric characters or `_`. Its maximum length is 50 characters. After you create an `AccessLevel`, you cannot change its `name`. |
| `description` | String | Description of the `AccessLevel` and its use. Does not affect behavior. |
| `basic` | String | A `BasicLevel` composed of `Conditions`. |
| `custom` | String | A `CustomLevel` written in the Common Expression Language. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create access_level
access_level = provider.accesscontextmanager_api.Access_level {
    parent = "value"  # Required. Resource name for the access policy which owns this Access Level. Format: `accessPolicies/{policy_id}`
}

# Access access_level outputs
access_level_id = access_level.id
access_level_title = access_level.title
access_level_name = access_level.name
access_level_description = access_level.description
access_level_basic = access_level.basic
access_level_custom = access_level.custom
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
operation_0 = provider.accesscontextmanager_api.Operation {
    name = "value-0"
}
operation_1 = provider.accesscontextmanager_api.Operation {
    name = "value-1"
}
operation_2 = provider.accesscontextmanager_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.accesscontextmanager_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Accesscontextmanager_api Documentation](https://cloud.google.com/accesscontextmanager_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
