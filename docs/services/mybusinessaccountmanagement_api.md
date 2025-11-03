# Mybusinessaccountmanagement_api Service



**Resources**: 4

---

## Overview

The mybusinessaccountmanagement_api service provides access to 4 resource types:

- [Admin](#admin) [CRUD]
- [Account](#account) [CRU]
- [Invitation](#invitation) [CR]
- [Location](#location) [C]

---

## Resources


### Admin

Invites the specified user to become an administrator for the specified location. The invitee must accept the invitation in order to be granted access to the location. See AcceptInvitation to programmatically accept an invitation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. The resource name. For account admins, this is in the form: `accounts/{account_id}/admins/{admin_id}` For location admins, this is in the form: `locations/{location_id}/admins/{admin_id}` This field will be ignored if set during admin creation. |
| `role` | String |  | Required. Specifies the role that this admin uses with the specified Account or Location. |
| `pending_invitation` | bool |  | Output only. Indicates whether this admin has a pending invitation for the specified resource. |
| `account` | String |  | Immutable. The name of the Account resource that this Admin refers to. Used when calling locations.admins.create to invite a LocationGroup as an admin. If both this field and `admin` are set on `CREATE` requests, this field takes precedence and the email address in `admin` will be ignored. Format: `accounts/{account}`. |
| `admin` | String |  | Optional. The name of the admin. When making the initial invitation, this is the invitee's email address. On `GET` calls, the user's email address is returned if the invitation is still pending. Otherwise, it contains the user's first and last names. This field is only needed to be set during admin creation. |
| `parent` | String | ✅ | Required. The resource name of the location this admin is created for. `locations/{location_id}/admins`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_admins` | Vec<String> | A collection of Admin instances. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create admin
admin = provider.mybusinessaccountmanagement_api.Admin {
    parent = "value"  # Required. The resource name of the location this admin is created for. `locations/{location_id}/admins`.
}

# Access admin outputs
admin_id = admin.id
admin_account_admins = admin.account_admins
```

---


### Account

Creates an account with the specified name and type under the given parent. - Personal accounts and Organizations cannot be created. - User Groups cannot be created with a Personal account as primary owner. - Location Groups cannot be created with a primary owner of a Personal account if the Personal account is in an Organization. - Location Groups cannot own Location Groups.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_name` | String |  | Required. The name of the account. For an account of type `PERSONAL`, this is the first and last name of the user account. |
| `organization_info` | String |  | Output only. Additional info for an organization. This is populated only for an organization account. |
| `primary_owner` | String |  | Required. Input only. The resource name of the account which will be the primary owner of the account being created. It should be of the form `accounts/{account_id}`. |
| `account_number` | String |  | Output only. Account reference number if provisioned. |
| `role` | String |  | Output only. Specifies the AccountRole of this account. |
| `verification_state` | String |  | Output only. If verified, future locations that are created are automatically connected to Google Maps, and have Google+ pages created, without requiring moderation. |
| `name` | String |  | Immutable. The resource name, in the format `accounts/{account_id}`. |
| `type` | String |  | Required. Contains the type of account. Accounts of type PERSONAL and ORGANIZATION cannot be created using this API. |
| `vetted_state` | String |  | Output only. Indicates whether the account is vetted by Google. A vetted account is able to verify locations via the VETTED_PARTNER method. |
| `permission_level` | String |  | Output only. Specifies the permission level the user has for this account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_name` | String | Required. The name of the account. For an account of type `PERSONAL`, this is the first and last name of the user account. |
| `organization_info` | String | Output only. Additional info for an organization. This is populated only for an organization account. |
| `primary_owner` | String | Required. Input only. The resource name of the account which will be the primary owner of the account being created. It should be of the form `accounts/{account_id}`. |
| `account_number` | String | Output only. Account reference number if provisioned. |
| `role` | String | Output only. Specifies the AccountRole of this account. |
| `verification_state` | String | Output only. If verified, future locations that are created are automatically connected to Google Maps, and have Google+ pages created, without requiring moderation. |
| `name` | String | Immutable. The resource name, in the format `accounts/{account_id}`. |
| `type` | String | Required. Contains the type of account. Accounts of type PERSONAL and ORGANIZATION cannot be created using this API. |
| `vetted_state` | String | Output only. Indicates whether the account is vetted by Google. A vetted account is able to verify locations via the VETTED_PARTNER method. |
| `permission_level` | String | Output only. Specifies the permission level the user has for this account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.mybusinessaccountmanagement_api.Account {
}

# Access account outputs
account_id = account.id
account_account_name = account.account_name
account_organization_info = account.organization_info
account_primary_owner = account.primary_owner
account_account_number = account.account_number
account_role = account.role
account_verification_state = account.verification_state
account_name = account.name
account_type = account.type
account_vetted_state = account.vetted_state
account_permission_level = account.permission_level
```

---


### Invitation

Accepts the specified invitation.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the invitation that is being accepted. `accounts/{account_id}/invitations/{invitation_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `invitations` | Vec<String> | A collection of invitations that are pending for the account. The number of invitations listed here cannot exceed 1000. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create invitation
invitation = provider.mybusinessaccountmanagement_api.Invitation {
    name = "value"  # Required. The name of the invitation that is being accepted. `accounts/{account_id}/invitations/{invitation_id}`
}

# Access invitation outputs
invitation_id = invitation.id
invitation_invitations = invitation.invitations
```

---


### Location

Moves a location from an account that the user owns to another account that the same user administers. The user must be an owner of the account the location is currently associated with and must also be at least a manager of the destination account.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_account` | String |  | Required. Name of the account resource to transfer the location to (for example, "accounts/{account}"). |
| `name` | String | ✅ | Required. The name of the location to transfer. `locations/{location_id}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create location
location = provider.mybusinessaccountmanagement_api.Location {
    name = "value"  # Required. The name of the location to transfer. `locations/{location_id}`.
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple admin resources
admin_0 = provider.mybusinessaccountmanagement_api.Admin {
    parent = "value-0"
}
admin_1 = provider.mybusinessaccountmanagement_api.Admin {
    parent = "value-1"
}
admin_2 = provider.mybusinessaccountmanagement_api.Admin {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    admin = provider.mybusinessaccountmanagement_api.Admin {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Mybusinessaccountmanagement_api Documentation](https://cloud.google.com/mybusinessaccountmanagement_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
