# Iam_api Service



**Resources**: 20

---

## Overview

The iam_api service provides access to 20 resource types:

- [Namespace](#namespace) [CRUD]
- [Permission](#permission) [C]
- [Scim_tenant](#scim_tenant) [CRUD]
- [Operation](#operation) [R]
- [Token](#token) [CRUD]
- [Service_account](#service_account) [CRUD]
- [Oauth_client](#oauth_client) [CRUD]
- [Managed_identitie](#managed_identitie) [CRUD]
- [Provider](#provider) [CRUD]
- [Key](#key) [CRD]
- [Iam_policie](#iam_policie) [C]
- [Subject](#subject) [CD]
- [Workload_identity_pool](#workload_identity_pool) [CRUD]
- [Credential](#credential) [CRUD]
- [Role](#role) [CRUD]
- [Workforce_pool](#workforce_pool) [CRUD]
- [Operation](#operation) [R]
- [Policie](#policie) [CRUD]
- [Operation](#operation) [R]
- [Policie](#policie) [CRUD]

---

## Resources


### Namespace

Creates a new WorkloadIdentityPoolNamespace in a WorkloadIdentityPool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disabled` | bool |  | Optional. Whether the namespace is disabled. If disabled, credentials may no longer be issued for identities within this namespace, however existing credentials will still be accepted until they expire. |
| `owner_service` | String |  | Output only. The Google Cloud service that owns this namespace. |
| `state` | String |  | Output only. The state of the namespace. |
| `name` | String |  | Output only. The resource name of the namespace. |
| `description` | String |  | Optional. A description of the namespace. Cannot exceed 256 characters. |
| `expire_time` | String |  | Output only. Time after which the namespace will be permanently purged and cannot be recovered. |
| `parent` | String | ✅ | Required. The parent resource to create the namespace in. The only supported location is `global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disabled` | bool | Optional. Whether the namespace is disabled. If disabled, credentials may no longer be issued for identities within this namespace, however existing credentials will still be accepted until they expire. |
| `owner_service` | String | Output only. The Google Cloud service that owns this namespace. |
| `state` | String | Output only. The state of the namespace. |
| `name` | String | Output only. The resource name of the namespace. |
| `description` | String | Optional. A description of the namespace. Cannot exceed 256 characters. |
| `expire_time` | String | Output only. Time after which the namespace will be permanently purged and cannot be recovered. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create namespace
namespace = provider.iam_api.Namespace {
    parent = "value"  # Required. The parent resource to create the namespace in. The only supported location is `global`.
}

# Access namespace outputs
namespace_id = namespace.id
namespace_disabled = namespace.disabled
namespace_owner_service = namespace.owner_service
namespace_state = namespace.state
namespace_name = namespace.name
namespace_description = namespace.description
namespace_expire_time = namespace.expire_time
```

---


### Permission

Lists every permission that you can test on a resource. A permission is testable if you can check whether a principal has that permission on the resource.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_size` | i64 |  | Optional limit on the number of permissions to include in the response. The default is 100, and the maximum is 1,000. |
| `full_resource_name` | String |  | Required. The full resource name to query from the list of testable permissions. The name follows the Google Cloud Platform resource format. For example, a Cloud Platform project with id `my-project` will be named `//cloudresourcemanager.googleapis.com/projects/my-project`. |
| `page_token` | String |  | Optional pagination token returned in an earlier QueryTestablePermissionsRequest. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create permission
permission = provider.iam_api.Permission {
}

```

---


### Scim_tenant

Agentspace only. Creates a new WorkforcePoolProviderScimTenant in a WorkforcePoolProvider. You cannot reuse the name of a deleted SCIM tenant until 30 days after deletion.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Agentspace only. The resource name of the SCIM Tenant. Format: `locations/{location}/workforcePools/{workforce_pool}/providers/ {workforce_pool_provider}/scimTenants/{scim_tenant}` |
| `description` | String |  | Optional. Agentspace only. The description of the SCIM tenant. Cannot exceed 256 characters. |
| `purge_time` | String |  | Output only. Agentspace only. The timestamp that represents the time when the SCIM tenant is purged. |
| `claim_mapping` | HashMap<String, String> |  | Optional. Agentspace only. Maps BYOID claims to SCIM claims. |
| `service_agent` | String |  | Output only. Service Agent created by SCIM Tenant API. SCIM tokens created under this tenant will be attached to this service agent. |
| `base_uri` | String |  | Output only. Agentspace only. Represents the base URI as defined in [RFC 7644, Section 1.3](https://datatracker.ietf.org/doc/html/rfc7644#section-1.3). Clients must use this as the root address for managing resources under the tenant. Format: https://iamscim.googleapis.com/{version}/{tenant_id}/ |
| `state` | String |  | Output only. Agentspace only. The state of the tenant. |
| `display_name` | String |  | Optional. Agentspace only. The display name of the SCIM tenant. Cannot exceed 32 characters. |
| `parent` | String | ✅ | Required. Agentspace only. The parent to create SCIM tenant. Format: 'locations/{location}/workforcePools/{workforce_pool}/providers/{provider}' |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Agentspace only. The resource name of the SCIM Tenant. Format: `locations/{location}/workforcePools/{workforce_pool}/providers/ {workforce_pool_provider}/scimTenants/{scim_tenant}` |
| `description` | String | Optional. Agentspace only. The description of the SCIM tenant. Cannot exceed 256 characters. |
| `purge_time` | String | Output only. Agentspace only. The timestamp that represents the time when the SCIM tenant is purged. |
| `claim_mapping` | HashMap<String, String> | Optional. Agentspace only. Maps BYOID claims to SCIM claims. |
| `service_agent` | String | Output only. Service Agent created by SCIM Tenant API. SCIM tokens created under this tenant will be attached to this service agent. |
| `base_uri` | String | Output only. Agentspace only. Represents the base URI as defined in [RFC 7644, Section 1.3](https://datatracker.ietf.org/doc/html/rfc7644#section-1.3). Clients must use this as the root address for managing resources under the tenant. Format: https://iamscim.googleapis.com/{version}/{tenant_id}/ |
| `state` | String | Output only. Agentspace only. The state of the tenant. |
| `display_name` | String | Optional. Agentspace only. The display name of the SCIM tenant. Cannot exceed 32 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create scim_tenant
scim_tenant = provider.iam_api.Scim_tenant {
    parent = "value"  # Required. Agentspace only. The parent to create SCIM tenant. Format: 'locations/{location}/workforcePools/{workforce_pool}/providers/{provider}'
}

# Access scim_tenant outputs
scim_tenant_id = scim_tenant.id
scim_tenant_name = scim_tenant.name
scim_tenant_description = scim_tenant.description
scim_tenant_purge_time = scim_tenant.purge_time
scim_tenant_claim_mapping = scim_tenant.claim_mapping
scim_tenant_service_agent = scim_tenant.service_agent
scim_tenant_base_uri = scim_tenant.base_uri
scim_tenant_state = scim_tenant.state
scim_tenant_display_name = scim_tenant.display_name
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |


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
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
```

---


### Token

Agentspace only. Creates a new WorkforcePoolProviderScimToken in a WorkforcePoolProviderScimTenant. You cannot reuse the name of a deleted SCIM token until 30 days after deletion.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `security_token` | String |  | Output only. Agentspace only. The token string. Provide this to the IdP for authentication. Will be set only during creation. |
| `display_name` | String |  | Optional. Agentspace only. The display name of the SCIM token. Cannot exceed 32 characters. |
| `name` | String |  | Identifier. Agentspace only. The resource name of the SCIM Token. Format: `locations/{location}/workforcePools/{workforce_pool}/providers/ {workforce_pool_provider}/scimTenants/{scim_tenant}/tokens/{token}` |
| `state` | String |  | Output only. Agentspace only. The state of the token. |
| `parent` | String | ✅ | Required. Agentspace only. The parent tenant to create SCIM token. Format: 'locations/{location}/workforcePools/{workforce_pool}/providers/{provider}/scimTenants/{scim_tenant}' |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `security_token` | String | Output only. Agentspace only. The token string. Provide this to the IdP for authentication. Will be set only during creation. |
| `display_name` | String | Optional. Agentspace only. The display name of the SCIM token. Cannot exceed 32 characters. |
| `name` | String | Identifier. Agentspace only. The resource name of the SCIM Token. Format: `locations/{location}/workforcePools/{workforce_pool}/providers/ {workforce_pool_provider}/scimTenants/{scim_tenant}/tokens/{token}` |
| `state` | String | Output only. Agentspace only. The state of the token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create token
token = provider.iam_api.Token {
    parent = "value"  # Required. Agentspace only. The parent tenant to create SCIM token. Format: 'locations/{location}/workforcePools/{workforce_pool}/providers/{provider}/scimTenants/{scim_tenant}'
}

# Access token outputs
token_id = token.id
token_security_token = token.security_token
token_display_name = token.display_name
token_name = token.name
token_state = token.state
```

---


### Service_account

Creates a ServiceAccount.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service_account` | String |  | The ServiceAccount resource to create. Currently, only the following values are user assignable: `display_name` and `description`. |
| `account_id` | String |  | Required. The account id that is used to generate the service account email address and a stable unique id. It is unique within a project, must be 6-30 characters long, and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])` to comply with RFC1035. |
| `name` | String | ✅ | Required. The resource name of the project associated with the service accounts, such as `projects/my-project-123`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `oauth2_client_id` | String | Output only. The OAuth 2.0 client ID for the service account. |
| `disabled` | bool | Output only. Whether the service account is disabled. |
| `etag` | String | Deprecated. Do not use. |
| `display_name` | String | Optional. A user-specified, human-readable name for the service account. The maximum length is 100 UTF-8 bytes. |
| `unique_id` | String | Output only. The unique, stable numeric ID for the service account. Each service account retains its unique ID even if you delete the service account. For example, if you delete a service account, then create a new service account with the same name, the new service account has a different unique ID than the deleted service account. |
| `email` | String | Output only. The email address of the service account. |
| `name` | String | The resource name of the service account. Use one of the following formats: * `projects/{PROJECT_ID}/serviceAccounts/{EMAIL_ADDRESS}` * `projects/{PROJECT_ID}/serviceAccounts/{UNIQUE_ID}` As an alternative, you can use the `-` wildcard character instead of the project ID: * `projects/-/serviceAccounts/{EMAIL_ADDRESS}` * `projects/-/serviceAccounts/{UNIQUE_ID}` When possible, avoid using the `-` wildcard character, because it can cause response messages to contain misleading error codes. For example, if you try to access the service account `projects/-/serviceAccounts/fake@example.com`, which does not exist, the response contains an HTTP `403 Forbidden` error instead of a `404 Not Found` error. |
| `project_id` | String | Output only. The ID of the project that owns the service account. |
| `description` | String | Optional. A user-specified, human-readable description of the service account. The maximum length is 256 UTF-8 bytes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_account
service_account = provider.iam_api.Service_account {
    name = "value"  # Required. The resource name of the project associated with the service accounts, such as `projects/my-project-123`.
}

# Access service_account outputs
service_account_id = service_account.id
service_account_oauth2_client_id = service_account.oauth2_client_id
service_account_disabled = service_account.disabled
service_account_etag = service_account.etag
service_account_display_name = service_account.display_name
service_account_unique_id = service_account.unique_id
service_account_email = service_account.email
service_account_name = service_account.name
service_account_project_id = service_account.project_id
service_account_description = service_account.description
```

---


### Oauth_client

Creates a new OauthClient. You cannot reuse the name of a deleted OauthClient until 30 days after deletion.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. A user-specified description of the OauthClient. Cannot exceed 256 characters. |
| `disabled` | bool |  | Optional. Whether the OauthClient is disabled. You cannot use a disabled OAuth client. |
| `allowed_redirect_uris` | Vec<String> |  | Required. The list of redirect uris that is allowed to redirect back when authorization process is completed. |
| `display_name` | String |  | Optional. A user-specified display name of the OauthClient. Cannot exceed 32 characters. |
| `expire_time` | String |  | Output only. Time after which the OauthClient will be permanently purged and cannot be recovered. |
| `name` | String |  | Immutable. Identifier. The resource name of the OauthClient. Format:`projects/{project}/locations/{location}/oauthClients/{oauth_client}`. |
| `client_type` | String |  | Immutable. The type of OauthClient. Either public or private. For private clients, the client secret can be managed using the dedicated OauthClientCredential resource. |
| `state` | String |  | Output only. The state of the OauthClient. |
| `allowed_scopes` | Vec<String> |  | Required. The list of scopes that the OauthClient is allowed to request during OAuth flows. The following scopes are supported: * `https://www.googleapis.com/auth/cloud-platform`: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account. * `openid`: The OAuth client can associate you with your personal information on Google Cloud. * `email`: The OAuth client can read a federated identity's email address. * `groups`: The OAuth client can read a federated identity's groups. |
| `allowed_grant_types` | Vec<String> |  | Required. The list of OAuth grant types is allowed for the OauthClient. |
| `client_id` | String |  | Output only. The system-generated OauthClient id. |
| `parent` | String | ✅ | Required. The parent resource to create the OauthClient in. The only supported location is `global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. A user-specified description of the OauthClient. Cannot exceed 256 characters. |
| `disabled` | bool | Optional. Whether the OauthClient is disabled. You cannot use a disabled OAuth client. |
| `allowed_redirect_uris` | Vec<String> | Required. The list of redirect uris that is allowed to redirect back when authorization process is completed. |
| `display_name` | String | Optional. A user-specified display name of the OauthClient. Cannot exceed 32 characters. |
| `expire_time` | String | Output only. Time after which the OauthClient will be permanently purged and cannot be recovered. |
| `name` | String | Immutable. Identifier. The resource name of the OauthClient. Format:`projects/{project}/locations/{location}/oauthClients/{oauth_client}`. |
| `client_type` | String | Immutable. The type of OauthClient. Either public or private. For private clients, the client secret can be managed using the dedicated OauthClientCredential resource. |
| `state` | String | Output only. The state of the OauthClient. |
| `allowed_scopes` | Vec<String> | Required. The list of scopes that the OauthClient is allowed to request during OAuth flows. The following scopes are supported: * `https://www.googleapis.com/auth/cloud-platform`: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account. * `openid`: The OAuth client can associate you with your personal information on Google Cloud. * `email`: The OAuth client can read a federated identity's email address. * `groups`: The OAuth client can read a federated identity's groups. |
| `allowed_grant_types` | Vec<String> | Required. The list of OAuth grant types is allowed for the OauthClient. |
| `client_id` | String | Output only. The system-generated OauthClient id. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create oauth_client
oauth_client = provider.iam_api.Oauth_client {
    parent = "value"  # Required. The parent resource to create the OauthClient in. The only supported location is `global`.
}

# Access oauth_client outputs
oauth_client_id = oauth_client.id
oauth_client_description = oauth_client.description
oauth_client_disabled = oauth_client.disabled
oauth_client_allowed_redirect_uris = oauth_client.allowed_redirect_uris
oauth_client_display_name = oauth_client.display_name
oauth_client_expire_time = oauth_client.expire_time
oauth_client_name = oauth_client.name
oauth_client_client_type = oauth_client.client_type
oauth_client_state = oauth_client.state
oauth_client_allowed_scopes = oauth_client.allowed_scopes
oauth_client_allowed_grant_types = oauth_client.allowed_grant_types
oauth_client_client_id = oauth_client.client_id
```

---


### Managed_identitie

Creates a new WorkloadIdentityPoolManagedIdentity in a WorkloadIdentityPoolNamespace.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disabled` | bool |  | Optional. Whether the managed identity is disabled. If disabled, credentials may no longer be issued for the identity, however existing credentials will still be accepted until they expire. |
| `name` | String |  | Output only. The resource name of the managed identity. |
| `description` | String |  | Optional. A description of the managed identity. Cannot exceed 256 characters. |
| `state` | String |  | Output only. The state of the managed identity. |
| `expire_time` | String |  | Output only. Time after which the managed identity will be permanently purged and cannot be recovered. |
| `parent` | String | ✅ | Required. The parent resource to create the manage identity in. The only supported location is `global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disabled` | bool | Optional. Whether the managed identity is disabled. If disabled, credentials may no longer be issued for the identity, however existing credentials will still be accepted until they expire. |
| `name` | String | Output only. The resource name of the managed identity. |
| `description` | String | Optional. A description of the managed identity. Cannot exceed 256 characters. |
| `state` | String | Output only. The state of the managed identity. |
| `expire_time` | String | Output only. Time after which the managed identity will be permanently purged and cannot be recovered. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create managed_identitie
managed_identitie = provider.iam_api.Managed_identitie {
    parent = "value"  # Required. The parent resource to create the manage identity in. The only supported location is `global`.
}

# Access managed_identitie outputs
managed_identitie_id = managed_identitie.id
managed_identitie_disabled = managed_identitie.disabled
managed_identitie_name = managed_identitie.name
managed_identitie_description = managed_identitie.description
managed_identitie_state = managed_identitie.state
managed_identitie_expire_time = managed_identitie.expire_time
```

---


### Provider

Creates a new WorkforcePoolProvider in a WorkforcePool. You cannot reuse the name of a deleted provider until 30 days after deletion.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attribute_mapping` | HashMap<String, String> |  | Required. Maps attributes from the authentication credentials issued by an external identity provider to Google Cloud attributes, such as `subject` and `segment`. Each key must be a string specifying the Google Cloud IAM attribute to map to. The following keys are supported: * `google.subject`: The principal IAM is authenticating. You can reference this value in IAM bindings. This is also the subject that appears in Cloud Logging logs. This is a required field and the mapped subject cannot exceed 127 bytes. * `google.groups`: Groups the authenticating user belongs to. You can grant groups access to resources using an IAM `principalSet` binding; access applies to all members of the group. * `google.display_name`: The name of the authenticated user. This is an optional field and the mapped display name cannot exceed 100 bytes. If not set, `google.subject` will be displayed instead. This attribute cannot be referenced in IAM bindings. * `google.profile_photo`: The URL that specifies the authenticated user's thumbnail photo. This is an optional field. When set, the image will be visible as the user's profile picture. If not set, a generic user icon will be displayed instead. This attribute cannot be referenced in IAM bindings. * `google.posix_username`: The Linux username used by OS Login. This is an optional field and the mapped POSIX username cannot exceed 32 characters, The key must match the regex "^a-zA-Z0-9._{0,31}$". This attribute cannot be referenced in IAM bindings. You can also provide custom attributes by specifying `attribute.{custom_attribute}`, where {custom_attribute} is the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes. The maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_]. You can reference these attributes in IAM policies to define fine-grained access for a workforce pool to Google Cloud resources. For example: * `google.subject`: `principal://iam.googleapis.com/locations/global/workforcePools/{pool}/subject/{value}` * `google.groups`: `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool}/group/{value}` * `attribute.{custom_attribute}`: `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool}/attribute.{custom_attribute}/{value}` Each value must be a [Common Expression Language] (https://opensource.google/projects/cel) function that maps an identity provider credential to the normalized attribute specified by the corresponding map key. You can use the `assertion` keyword in the expression to access a JSON representation of the authentication credential issued by the provider. The maximum length of an attribute mapping expression is 2048 characters. When evaluated, the total size of all mapped attributes must not exceed 16 KB. For OIDC providers, you must supply a custom mapping that includes the `google.subject` attribute. For example, the following maps the `sub` claim of the incoming credential to the `subject` attribute on a Google token: ``` {"google.subject": "assertion.sub"} ``` |
| `attribute_condition` | String |  | Optional. A [Common Expression Language](https://opensource.google/projects/cel) expression, in plain text, to restrict what otherwise valid authentication credentials issued by the provider should not be accepted. The expression must output a boolean representing whether to allow the federation. The following keywords may be referenced in the expressions: * `assertion`: JSON representing the authentication credential issued by the provider. * `google`: The Google attributes mapped from the assertion in the `attribute_mappings`. `google.profile_photo`, `google.display_name` and `google.posix_username` are not supported. * `attribute`: The custom attributes mapped from the assertion in the `attribute_mappings`. The maximum length of the attribute condition expression is 4096 characters. If unspecified, all valid authentication credentials will be accepted. The following example shows how to only allow credentials with a mapped `google.groups` value of `admins`: ``` "'admins' in google.groups" ``` |
| `extra_attributes_oauth2_client` | String |  | Optional. The configuration for OAuth 2.0 client used to get the additional user attributes. This should be used when users can't get the desired claims in authentication credentials. Currently this configuration is only supported with OIDC protocol. |
| `name` | String |  | Identifier. The resource name of the provider. Format: `locations/{location}/workforcePools/{workforce_pool_id}/providers/{provider_id}` |
| `scim_usage` | String |  | Optional. Agentspace only. Specifies whether the workforce identity pool provider uses SCIM-managed groups instead of the `google.groups` attribute mapping for authorization checks. The `scim_usage` and `extended_attributes_oauth2_client` fields are mutually exclusive. A request that enables both fields on the same workforce identity pool provider will produce an error. |
| `description` | String |  | Optional. A description of the provider. Cannot exceed 256 characters. |
| `oidc` | String |  | An OpenId Connect 1.0 identity provider configuration. |
| `extended_attributes_oauth2_client` | String |  | Optional. The configuration for OAuth 2.0 client used to get the extended group memberships for user identities. Only the `AZURE_AD_GROUPS_ID` attribute type is supported. Extended groups supports a subset of Google Cloud services. When the user accesses these services, extended group memberships override the mapped `google.groups` attribute. Extended group memberships cannot be used in attribute mapping or attribute condition expressions. To keep extended group memberships up to date, extended groups are retrieved when the user signs in and at regular intervals during the user's active session. Each user identity in the workforce identity pool must map to a unique Microsoft Entra ID user. |
| `saml` | String |  | A SAML identity provider configuration. |
| `state` | String |  | Output only. The state of the provider. |
| `disabled` | bool |  | Optional. Disables the workforce pool provider. You cannot use a disabled provider to exchange tokens. However, existing tokens still grant access. |
| `expire_time` | String |  | Output only. Time after which the workforce identity pool provider will be permanently purged and cannot be recovered. |
| `detailed_audit_logging` | bool |  | Optional. If true, populates additional debug information in Cloud Audit Logs for this provider. Logged attribute mappings and values can be found in `sts.googleapis.com` data access logs. Default value is false. |
| `display_name` | String |  | Optional. A display name for the provider. Cannot exceed 32 characters. |
| `parent` | String | ✅ | Required. The pool to create this provider in. Format: `locations/{location}/workforcePools/{workforce_pool_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attribute_mapping` | HashMap<String, String> | Required. Maps attributes from the authentication credentials issued by an external identity provider to Google Cloud attributes, such as `subject` and `segment`. Each key must be a string specifying the Google Cloud IAM attribute to map to. The following keys are supported: * `google.subject`: The principal IAM is authenticating. You can reference this value in IAM bindings. This is also the subject that appears in Cloud Logging logs. This is a required field and the mapped subject cannot exceed 127 bytes. * `google.groups`: Groups the authenticating user belongs to. You can grant groups access to resources using an IAM `principalSet` binding; access applies to all members of the group. * `google.display_name`: The name of the authenticated user. This is an optional field and the mapped display name cannot exceed 100 bytes. If not set, `google.subject` will be displayed instead. This attribute cannot be referenced in IAM bindings. * `google.profile_photo`: The URL that specifies the authenticated user's thumbnail photo. This is an optional field. When set, the image will be visible as the user's profile picture. If not set, a generic user icon will be displayed instead. This attribute cannot be referenced in IAM bindings. * `google.posix_username`: The Linux username used by OS Login. This is an optional field and the mapped POSIX username cannot exceed 32 characters, The key must match the regex "^a-zA-Z0-9._{0,31}$". This attribute cannot be referenced in IAM bindings. You can also provide custom attributes by specifying `attribute.{custom_attribute}`, where {custom_attribute} is the name of the custom attribute to be mapped. You can define a maximum of 50 custom attributes. The maximum length of a mapped attribute key is 100 characters, and the key may only contain the characters [a-z0-9_]. You can reference these attributes in IAM policies to define fine-grained access for a workforce pool to Google Cloud resources. For example: * `google.subject`: `principal://iam.googleapis.com/locations/global/workforcePools/{pool}/subject/{value}` * `google.groups`: `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool}/group/{value}` * `attribute.{custom_attribute}`: `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool}/attribute.{custom_attribute}/{value}` Each value must be a [Common Expression Language] (https://opensource.google/projects/cel) function that maps an identity provider credential to the normalized attribute specified by the corresponding map key. You can use the `assertion` keyword in the expression to access a JSON representation of the authentication credential issued by the provider. The maximum length of an attribute mapping expression is 2048 characters. When evaluated, the total size of all mapped attributes must not exceed 16 KB. For OIDC providers, you must supply a custom mapping that includes the `google.subject` attribute. For example, the following maps the `sub` claim of the incoming credential to the `subject` attribute on a Google token: ``` {"google.subject": "assertion.sub"} ``` |
| `attribute_condition` | String | Optional. A [Common Expression Language](https://opensource.google/projects/cel) expression, in plain text, to restrict what otherwise valid authentication credentials issued by the provider should not be accepted. The expression must output a boolean representing whether to allow the federation. The following keywords may be referenced in the expressions: * `assertion`: JSON representing the authentication credential issued by the provider. * `google`: The Google attributes mapped from the assertion in the `attribute_mappings`. `google.profile_photo`, `google.display_name` and `google.posix_username` are not supported. * `attribute`: The custom attributes mapped from the assertion in the `attribute_mappings`. The maximum length of the attribute condition expression is 4096 characters. If unspecified, all valid authentication credentials will be accepted. The following example shows how to only allow credentials with a mapped `google.groups` value of `admins`: ``` "'admins' in google.groups" ``` |
| `extra_attributes_oauth2_client` | String | Optional. The configuration for OAuth 2.0 client used to get the additional user attributes. This should be used when users can't get the desired claims in authentication credentials. Currently this configuration is only supported with OIDC protocol. |
| `name` | String | Identifier. The resource name of the provider. Format: `locations/{location}/workforcePools/{workforce_pool_id}/providers/{provider_id}` |
| `scim_usage` | String | Optional. Agentspace only. Specifies whether the workforce identity pool provider uses SCIM-managed groups instead of the `google.groups` attribute mapping for authorization checks. The `scim_usage` and `extended_attributes_oauth2_client` fields are mutually exclusive. A request that enables both fields on the same workforce identity pool provider will produce an error. |
| `description` | String | Optional. A description of the provider. Cannot exceed 256 characters. |
| `oidc` | String | An OpenId Connect 1.0 identity provider configuration. |
| `extended_attributes_oauth2_client` | String | Optional. The configuration for OAuth 2.0 client used to get the extended group memberships for user identities. Only the `AZURE_AD_GROUPS_ID` attribute type is supported. Extended groups supports a subset of Google Cloud services. When the user accesses these services, extended group memberships override the mapped `google.groups` attribute. Extended group memberships cannot be used in attribute mapping or attribute condition expressions. To keep extended group memberships up to date, extended groups are retrieved when the user signs in and at regular intervals during the user's active session. Each user identity in the workforce identity pool must map to a unique Microsoft Entra ID user. |
| `saml` | String | A SAML identity provider configuration. |
| `state` | String | Output only. The state of the provider. |
| `disabled` | bool | Optional. Disables the workforce pool provider. You cannot use a disabled provider to exchange tokens. However, existing tokens still grant access. |
| `expire_time` | String | Output only. Time after which the workforce identity pool provider will be permanently purged and cannot be recovered. |
| `detailed_audit_logging` | bool | Optional. If true, populates additional debug information in Cloud Audit Logs for this provider. Logged attribute mappings and values can be found in `sts.googleapis.com` data access logs. Default value is false. |
| `display_name` | String | Optional. A display name for the provider. Cannot exceed 32 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create provider
provider = provider.iam_api.Provider {
    parent = "value"  # Required. The pool to create this provider in. Format: `locations/{location}/workforcePools/{workforce_pool_id}`
}

# Access provider outputs
provider_id = provider.id
provider_attribute_mapping = provider.attribute_mapping
provider_attribute_condition = provider.attribute_condition
provider_extra_attributes_oauth2_client = provider.extra_attributes_oauth2_client
provider_name = provider.name
provider_scim_usage = provider.scim_usage
provider_description = provider.description
provider_oidc = provider.oidc
provider_extended_attributes_oauth2_client = provider.extended_attributes_oauth2_client
provider_saml = provider.saml
provider_state = provider.state
provider_disabled = provider.disabled
provider_expire_time = provider.expire_time
provider_detailed_audit_logging = provider.detailed_audit_logging
provider_display_name = provider.display_name
```

---


### Key

Creates a new WorkforcePoolProviderKey in a WorkforcePoolProvider.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name of the key. Format: `locations/{location}/workforcePools/{workforce_pool_id}/providers/{provider_id}/keys/{key_id}` |
| `state` | String |  | Output only. The state of the key. |
| `key_data` | String |  | Immutable. Public half of the asymmetric key. |
| `use` | String |  | Required. The purpose of the key. |
| `expire_time` | String |  | Output only. The time after which the key will be permanently deleted and cannot be recovered. Note that the key may get purged before this time if the total limit of keys per provider is exceeded. |
| `parent` | String | ✅ | Required. The provider to create this key in. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name of the key. Format: `locations/{location}/workforcePools/{workforce_pool_id}/providers/{provider_id}/keys/{key_id}` |
| `state` | String | Output only. The state of the key. |
| `key_data` | String | Immutable. Public half of the asymmetric key. |
| `use` | String | Required. The purpose of the key. |
| `expire_time` | String | Output only. The time after which the key will be permanently deleted and cannot be recovered. Note that the key may get purged before this time if the total limit of keys per provider is exceeded. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create key
key = provider.iam_api.Key {
    parent = "value"  # Required. The provider to create this key in.
}

# Access key outputs
key_id = key.id
key_name = key.name
key_state = key.state
key_key_data = key.key_data
key_use = key.use
key_expire_time = key.expire_time
```

---


### Iam_policie

Returns a list of services that allow you to opt into audit logs that are not generated by default. To learn more about audit logs, see the [Logging documentation](https://cloud.google.com/logging/docs/audit).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `full_resource_name` | String |  | Required. The full resource name to query from the list of auditable services. The name follows the Google Cloud Platform resource format. For example, a Cloud Platform project with id `my-project` will be named `//cloudresourcemanager.googleapis.com/projects/my-project`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create iam_policie
iam_policie = provider.iam_api.Iam_policie {
}

```

---


### Subject

Undeletes a WorkforcePoolSubject, as long as it was deleted fewer than 30 days ago.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The resource name of the WorkforcePoolSubject. Special characters, like `/` and `:`, must be escaped, because all URLs need to conform to the "When to Escape and Unescape" section of [RFC3986](https://www.ietf.org/rfc/rfc2396.txt). Format: `locations/{location}/workforcePools/{workforce_pool_id}/subjects/{subject_id}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subject
subject = provider.iam_api.Subject {
    name = "value"  # Required. The resource name of the WorkforcePoolSubject. Special characters, like `/` and `:`, must be escaped, because all URLs need to conform to the "When to Escape and Unescape" section of [RFC3986](https://www.ietf.org/rfc/rfc2396.txt). Format: `locations/{location}/workforcePools/{workforce_pool_id}/subjects/{subject_id}`
}

```

---


### Workload_identity_pool

Creates a new WorkloadIdentityPool. You cannot reuse the name of a deleted pool until 30 days after deletion.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `inline_certificate_issuance_config` | String |  | Optional. Defines the Certificate Authority (CA) pool resources and configurations required for issuance and rotation of mTLS workload certificates. |
| `state` | String |  | Output only. The state of the pool. |
| `inline_trust_config` | String |  | Optional. Represents config to add additional trusted trust domains. |
| `mode` | String |  | Immutable. The mode the pool is operating in. |
| `description` | String |  | Optional. A description of the pool. Cannot exceed 256 characters. |
| `name` | String |  | Output only. The resource name of the pool. |
| `disabled` | bool |  | Optional. Whether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again. |
| `expire_time` | String |  | Output only. Time after which the workload identity pool will be permanently purged and cannot be recovered. |
| `display_name` | String |  | Optional. A display name for the pool. Cannot exceed 32 characters. |
| `parent` | String | ✅ | Required. The parent resource to create the pool in. The only supported location is `global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `inline_certificate_issuance_config` | String | Optional. Defines the Certificate Authority (CA) pool resources and configurations required for issuance and rotation of mTLS workload certificates. |
| `state` | String | Output only. The state of the pool. |
| `inline_trust_config` | String | Optional. Represents config to add additional trusted trust domains. |
| `mode` | String | Immutable. The mode the pool is operating in. |
| `description` | String | Optional. A description of the pool. Cannot exceed 256 characters. |
| `name` | String | Output only. The resource name of the pool. |
| `disabled` | bool | Optional. Whether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again. |
| `expire_time` | String | Output only. Time after which the workload identity pool will be permanently purged and cannot be recovered. |
| `display_name` | String | Optional. A display name for the pool. Cannot exceed 32 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workload_identity_pool
workload_identity_pool = provider.iam_api.Workload_identity_pool {
    parent = "value"  # Required. The parent resource to create the pool in. The only supported location is `global`.
}

# Access workload_identity_pool outputs
workload_identity_pool_id = workload_identity_pool.id
workload_identity_pool_inline_certificate_issuance_config = workload_identity_pool.inline_certificate_issuance_config
workload_identity_pool_state = workload_identity_pool.state
workload_identity_pool_inline_trust_config = workload_identity_pool.inline_trust_config
workload_identity_pool_mode = workload_identity_pool.mode
workload_identity_pool_description = workload_identity_pool.description
workload_identity_pool_name = workload_identity_pool.name
workload_identity_pool_disabled = workload_identity_pool.disabled
workload_identity_pool_expire_time = workload_identity_pool.expire_time
workload_identity_pool_display_name = workload_identity_pool.display_name
```

---


### Credential

Creates a new OauthClientCredential.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_secret` | String |  | Output only. The system-generated OAuth client secret. The client secret must be stored securely. If the client secret is leaked, you must delete and re-create the client credential. To learn more, see [OAuth client and credential security risks and mitigations](https://cloud.google.com/iam/docs/workforce-oauth-app#security) |
| `display_name` | String |  | Optional. A user-specified display name of the OauthClientCredential. Cannot exceed 32 characters. |
| `disabled` | bool |  | Optional. Whether the OauthClientCredential is disabled. You cannot use a disabled OauthClientCredential. |
| `name` | String |  | Immutable. Identifier. The resource name of the OauthClientCredential. Format: `projects/{project}/locations/{location}/oauthClients/{oauth_client}/credentials/{credential}` |
| `parent` | String | ✅ | Required. The parent resource to create the OauthClientCredential in. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client_secret` | String | Output only. The system-generated OAuth client secret. The client secret must be stored securely. If the client secret is leaked, you must delete and re-create the client credential. To learn more, see [OAuth client and credential security risks and mitigations](https://cloud.google.com/iam/docs/workforce-oauth-app#security) |
| `display_name` | String | Optional. A user-specified display name of the OauthClientCredential. Cannot exceed 32 characters. |
| `disabled` | bool | Optional. Whether the OauthClientCredential is disabled. You cannot use a disabled OauthClientCredential. |
| `name` | String | Immutable. Identifier. The resource name of the OauthClientCredential. Format: `projects/{project}/locations/{location}/oauthClients/{oauth_client}/credentials/{credential}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create credential
credential = provider.iam_api.Credential {
    parent = "value"  # Required. The parent resource to create the OauthClientCredential in.
}

# Access credential outputs
credential_id = credential.id
credential_client_secret = credential.client_secret
credential_display_name = credential.display_name
credential_disabled = credential.disabled
credential_name = credential.name
```

---


### Role

Creates a new custom Role.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role` | String |  | The Role resource to create. |
| `role_id` | String |  | The role ID to use for this role. A role ID may contain alphanumeric characters, underscores (`_`), and periods (`.`). It must contain a minimum of 3 characters and a maximum of 64 characters. |
| `parent` | String | ✅ | The `parent` parameter's value depends on the target resource for the request, namely [projects](https://cloud.google.com/iam/docs/reference/rest/v1/projects.roles) or [organizations](https://cloud.google.com/iam/docs/reference/rest/v1/organizations.roles). Each resource type's `parent` value format is described below: * [projects.roles.create](https://cloud.google.com/iam/docs/reference/rest/v1/projects.roles/create): `projects/{PROJECT_ID}`. This method creates project-level [custom roles](https://cloud.google.com/iam/docs/understanding-custom-roles). Example request URL: `https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles` * [organizations.roles.create](https://cloud.google.com/iam/docs/reference/rest/v1/organizations.roles/create): `organizations/{ORGANIZATION_ID}`. This method creates organization-level [custom roles](https://cloud.google.com/iam/docs/understanding-custom-roles). Example request URL: `https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles` Note: Wildcard (*) values are invalid; you must specify a complete project ID or organization ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. A human-readable description for the role. |
| `included_permissions` | Vec<String> | The names of the permissions this role grants when bound in an IAM policy. |
| `stage` | String | The current launch stage of the role. If the `ALPHA` launch stage has been selected for a role, the `stage` field will not be included in the returned definition for the role. |
| `title` | String | Optional. A human-readable title for the role. Typically this is limited to 100 UTF-8 bytes. |
| `deleted` | bool | The current deleted state of the role. This field is read only. It will be ignored in calls to CreateRole and UpdateRole. |
| `name` | String | The name of the role. When `Role` is used in `CreateRole`, the role name must not be set. When `Role` is used in output and other input such as `UpdateRole`, the role name is the complete path. For example, `roles/logging.viewer` for predefined roles, `organizations/{ORGANIZATION_ID}/roles/myRole` for organization-level custom roles, and `projects/{PROJECT_ID}/roles/myRole` for project-level custom roles. |
| `etag` | String | Used to perform a consistent read-modify-write. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create role
role = provider.iam_api.Role {
    parent = "value"  # The `parent` parameter's value depends on the target resource for the request, namely [projects](https://cloud.google.com/iam/docs/reference/rest/v1/projects.roles) or [organizations](https://cloud.google.com/iam/docs/reference/rest/v1/organizations.roles). Each resource type's `parent` value format is described below: * [projects.roles.create](https://cloud.google.com/iam/docs/reference/rest/v1/projects.roles/create): `projects/{PROJECT_ID}`. This method creates project-level [custom roles](https://cloud.google.com/iam/docs/understanding-custom-roles). Example request URL: `https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles` * [organizations.roles.create](https://cloud.google.com/iam/docs/reference/rest/v1/organizations.roles/create): `organizations/{ORGANIZATION_ID}`. This method creates organization-level [custom roles](https://cloud.google.com/iam/docs/understanding-custom-roles). Example request URL: `https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles` Note: Wildcard (*) values are invalid; you must specify a complete project ID or organization ID.
}

# Access role outputs
role_id = role.id
role_description = role.description
role_included_permissions = role.included_permissions
role_stage = role.stage
role_title = role.title
role_deleted = role.deleted
role_name = role.name
role_etag = role.etag
```

---


### Workforce_pool

Creates a new WorkforcePool. You cannot reuse the name of a deleted pool until 30 days after deletion.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. A description of the pool. Cannot exceed 256 characters. |
| `parent` | String |  | Immutable. The resource name of the parent. Format: `organizations/{org-id}`. |
| `disabled` | bool |  | Optional. Disables the workforce pool. You cannot use a disabled pool to exchange tokens, or use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again. |
| `display_name` | String |  | Optional. A display name for the pool. Cannot exceed 32 characters. |
| `state` | String |  | Output only. The state of the pool. |
| `session_duration` | String |  | Optional. Duration that the Google Cloud access tokens, console sign-in sessions, and `gcloud` sign-in sessions from this pool are valid. Must be greater than 15 minutes (900s) and less than 12 hours (43200s). If `session_duration` is not configured, minted credentials have a default duration of one hour (3600s). For SAML providers, the lifetime of the token is the minimum of the `session_duration` and the `SessionNotOnOrAfter` claim in the SAML assertion. |
| `access_restrictions` | String |  | Optional. Configure access restrictions on the workforce pool users. This is an optional field. If specified web sign-in can be restricted to given set of services or programmatic sign-in can be disabled for pool users. |
| `name` | String |  | Identifier. The resource name of the pool. Format: `locations/{location}/workforcePools/{workforce_pool_id}` |
| `expire_time` | String |  | Output only. Time after which the workforce pool will be permanently purged and cannot be recovered. |
| `location` | String | ✅ | Optional. The location of the pool to create. Format: `locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. A description of the pool. Cannot exceed 256 characters. |
| `parent` | String | Immutable. The resource name of the parent. Format: `organizations/{org-id}`. |
| `disabled` | bool | Optional. Disables the workforce pool. You cannot use a disabled pool to exchange tokens, or use existing tokens to access resources. If the pool is re-enabled, existing tokens grant access again. |
| `display_name` | String | Optional. A display name for the pool. Cannot exceed 32 characters. |
| `state` | String | Output only. The state of the pool. |
| `session_duration` | String | Optional. Duration that the Google Cloud access tokens, console sign-in sessions, and `gcloud` sign-in sessions from this pool are valid. Must be greater than 15 minutes (900s) and less than 12 hours (43200s). If `session_duration` is not configured, minted credentials have a default duration of one hour (3600s). For SAML providers, the lifetime of the token is the minimum of the `session_duration` and the `SessionNotOnOrAfter` claim in the SAML assertion. |
| `access_restrictions` | String | Optional. Configure access restrictions on the workforce pool users. This is an optional field. If specified web sign-in can be restricted to given set of services or programmatic sign-in can be disabled for pool users. |
| `name` | String | Identifier. The resource name of the pool. Format: `locations/{location}/workforcePools/{workforce_pool_id}` |
| `expire_time` | String | Output only. Time after which the workforce pool will be permanently purged and cannot be recovered. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workforce_pool
workforce_pool = provider.iam_api.Workforce_pool {
    location = "value"  # Optional. The location of the pool to create. Format: `locations/{location}`.
}

# Access workforce_pool outputs
workforce_pool_id = workforce_pool.id
workforce_pool_description = workforce_pool.description
workforce_pool_parent = workforce_pool.parent
workforce_pool_disabled = workforce_pool.disabled
workforce_pool_display_name = workforce_pool.display_name
workforce_pool_state = workforce_pool.state
workforce_pool_session_duration = workforce_pool.session_duration
workforce_pool_access_restrictions = workforce_pool.access_restrictions
workforce_pool_name = workforce_pool.name
workforce_pool_expire_time = workforce_pool.expire_time
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


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
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
```

---


### Policie

Creates a policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uid` | String |  | Immutable. The globally unique ID of the `Policy`. Assigned automatically when the `Policy` is created. |
| `etag` | String |  | An opaque tag that identifies the current version of the `Policy`. IAM uses this value to help manage concurrent updates, so they do not cause one update to be overwritten by another. If this field is present in a CreatePolicyRequest, the value is ignored. |
| `name` | String |  | Immutable. The resource name of the `Policy`, which must be unique. Format: `policies/{attachment_point}/denypolicies/{policy_id}` The attachment point is identified by its URL-encoded full resource name, which means that the forward-slash character, `/`, must be written as `%2F`. For example, `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies/my-deny-policy`. For organizations and folders, use the numeric ID in the full resource name. For projects, requests can use the alphanumeric or the numeric ID. Responses always contain the numeric ID. |
| `annotations` | HashMap<String, String> |  | A key-value map to store arbitrary metadata for the `Policy`. Keys can be up to 63 characters. Values can be up to 255 characters. |
| `display_name` | String |  | A user-specified description of the `Policy`. This value can be up to 63 characters. |
| `create_time` | String |  | Output only. The time when the `Policy` was created. |
| `update_time` | String |  | Output only. The time when the `Policy` was last updated. |
| `rules` | Vec<String> |  | A list of rules that specify the behavior of the `Policy`. All of the rules should be of the `kind` specified in the `Policy`. |
| `kind` | String |  | Output only. The kind of the `Policy`. Always contains the value `DenyPolicy`. |
| `delete_time` | String |  | Output only. The time when the `Policy` was deleted. Empty if the policy is not deleted. |
| `parent` | String | ✅ | Required. The resource that the policy is attached to, along with the kind of policy to create. Format: `policies/{attachment_point}/denypolicies` The attachment point is identified by its URL-encoded full resource name, which means that the forward-slash character, `/`, must be written as `%2F`. For example, `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies`. For organizations and folders, use the numeric ID in the full resource name. For projects, you can use the alphanumeric or the numeric ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uid` | String | Immutable. The globally unique ID of the `Policy`. Assigned automatically when the `Policy` is created. |
| `etag` | String | An opaque tag that identifies the current version of the `Policy`. IAM uses this value to help manage concurrent updates, so they do not cause one update to be overwritten by another. If this field is present in a CreatePolicyRequest, the value is ignored. |
| `name` | String | Immutable. The resource name of the `Policy`, which must be unique. Format: `policies/{attachment_point}/denypolicies/{policy_id}` The attachment point is identified by its URL-encoded full resource name, which means that the forward-slash character, `/`, must be written as `%2F`. For example, `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies/my-deny-policy`. For organizations and folders, use the numeric ID in the full resource name. For projects, requests can use the alphanumeric or the numeric ID. Responses always contain the numeric ID. |
| `annotations` | HashMap<String, String> | A key-value map to store arbitrary metadata for the `Policy`. Keys can be up to 63 characters. Values can be up to 255 characters. |
| `display_name` | String | A user-specified description of the `Policy`. This value can be up to 63 characters. |
| `create_time` | String | Output only. The time when the `Policy` was created. |
| `update_time` | String | Output only. The time when the `Policy` was last updated. |
| `rules` | Vec<String> | A list of rules that specify the behavior of the `Policy`. All of the rules should be of the `kind` specified in the `Policy`. |
| `kind` | String | Output only. The kind of the `Policy`. Always contains the value `DenyPolicy`. |
| `delete_time` | String | Output only. The time when the `Policy` was deleted. Empty if the policy is not deleted. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policie
policie = provider.iam_api.Policie {
    parent = "value"  # Required. The resource that the policy is attached to, along with the kind of policy to create. Format: `policies/{attachment_point}/denypolicies` The attachment point is identified by its URL-encoded full resource name, which means that the forward-slash character, `/`, must be written as `%2F`. For example, `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies`. For organizations and folders, use the numeric ID in the full resource name. For projects, you can use the alphanumeric or the numeric ID.
}

# Access policie outputs
policie_id = policie.id
policie_uid = policie.uid
policie_etag = policie.etag
policie_name = policie.name
policie_annotations = policie.annotations
policie_display_name = policie.display_name
policie_create_time = policie.create_time
policie_update_time = policie.update_time
policie_rules = policie.rules
policie_kind = policie.kind
policie_delete_time = policie.delete_time
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


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
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
```

---


### Policie

Creates a policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Output only. The kind of the `Policy`. Always contains the value `DenyPolicy`. |
| `name` | String |  | Immutable. The resource name of the `Policy`, which must be unique. Format: `policies/{attachment_point}/denypolicies/{policy_id}` The attachment point is identified by its URL-encoded full resource name, which means that the forward-slash character, `/`, must be written as `%2F`. For example, `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies/my-deny-policy`. For organizations and folders, use the numeric ID in the full resource name. For projects, requests can use the alphanumeric or the numeric ID. Responses always contain the numeric ID. |
| `rules` | Vec<String> |  | A list of rules that specify the behavior of the `Policy`. All of the rules should be of the `kind` specified in the `Policy`. |
| `uid` | String |  | Immutable. The globally unique ID of the `Policy`. Assigned automatically when the `Policy` is created. |
| `update_time` | String |  | Output only. The time when the `Policy` was last updated. |
| `create_time` | String |  | Output only. The time when the `Policy` was created. |
| `annotations` | HashMap<String, String> |  | A key-value map to store arbitrary metadata for the `Policy`. Keys can be up to 63 characters. Values can be up to 255 characters. |
| `delete_time` | String |  | Output only. The time when the `Policy` was deleted. Empty if the policy is not deleted. |
| `etag` | String |  | An opaque tag that identifies the current version of the `Policy`. IAM uses this value to help manage concurrent updates, so they do not cause one update to be overwritten by another. If this field is present in a CreatePolicyRequest, the value is ignored. |
| `display_name` | String |  | A user-specified description of the `Policy`. This value can be up to 63 characters. |
| `parent` | String | ✅ | Required. The resource that the policy is attached to, along with the kind of policy to create. Format: `policies/{attachment_point}/denypolicies` The attachment point is identified by its URL-encoded full resource name, which means that the forward-slash character, `/`, must be written as `%2F`. For example, `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies`. For organizations and folders, use the numeric ID in the full resource name. For projects, you can use the alphanumeric or the numeric ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Output only. The kind of the `Policy`. Always contains the value `DenyPolicy`. |
| `name` | String | Immutable. The resource name of the `Policy`, which must be unique. Format: `policies/{attachment_point}/denypolicies/{policy_id}` The attachment point is identified by its URL-encoded full resource name, which means that the forward-slash character, `/`, must be written as `%2F`. For example, `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies/my-deny-policy`. For organizations and folders, use the numeric ID in the full resource name. For projects, requests can use the alphanumeric or the numeric ID. Responses always contain the numeric ID. |
| `rules` | Vec<String> | A list of rules that specify the behavior of the `Policy`. All of the rules should be of the `kind` specified in the `Policy`. |
| `uid` | String | Immutable. The globally unique ID of the `Policy`. Assigned automatically when the `Policy` is created. |
| `update_time` | String | Output only. The time when the `Policy` was last updated. |
| `create_time` | String | Output only. The time when the `Policy` was created. |
| `annotations` | HashMap<String, String> | A key-value map to store arbitrary metadata for the `Policy`. Keys can be up to 63 characters. Values can be up to 255 characters. |
| `delete_time` | String | Output only. The time when the `Policy` was deleted. Empty if the policy is not deleted. |
| `etag` | String | An opaque tag that identifies the current version of the `Policy`. IAM uses this value to help manage concurrent updates, so they do not cause one update to be overwritten by another. If this field is present in a CreatePolicyRequest, the value is ignored. |
| `display_name` | String | A user-specified description of the `Policy`. This value can be up to 63 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policie
policie = provider.iam_api.Policie {
    parent = "value"  # Required. The resource that the policy is attached to, along with the kind of policy to create. Format: `policies/{attachment_point}/denypolicies` The attachment point is identified by its URL-encoded full resource name, which means that the forward-slash character, `/`, must be written as `%2F`. For example, `policies/cloudresourcemanager.googleapis.com%2Fprojects%2Fmy-project/denypolicies`. For organizations and folders, use the numeric ID in the full resource name. For projects, you can use the alphanumeric or the numeric ID.
}

# Access policie outputs
policie_id = policie.id
policie_kind = policie.kind
policie_name = policie.name
policie_rules = policie.rules
policie_uid = policie.uid
policie_update_time = policie.update_time
policie_create_time = policie.create_time
policie_annotations = policie.annotations
policie_delete_time = policie.delete_time
policie_etag = policie.etag
policie_display_name = policie.display_name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple namespace resources
namespace_0 = provider.iam_api.Namespace {
    parent = "value-0"
}
namespace_1 = provider.iam_api.Namespace {
    parent = "value-1"
}
namespace_2 = provider.iam_api.Namespace {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    namespace = provider.iam_api.Namespace {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Iam_api Documentation](https://cloud.google.com/iam_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
