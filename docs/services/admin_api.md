# Admin_api Service



**Resources**: 34

---

## Overview

The admin_api service provides access to 34 resource types:

- [Privilege](#privilege) [R]
- [Schema](#schema) [CRUD]
- [Group](#group) [CRUD]
- [Role_assignment](#role_assignment) [CRD]
- [Aliase](#aliase) [CRD]
- [Domain](#domain) [CRD]
- [Orgunit](#orgunit) [CRUD]
- [Channel](#channel) [C]
- [Mobiledevice](#mobiledevice) [CRD]
- [Photo](#photo) [RUD]
- [Token](#token) [RD]
- [Role](#role) [CRUD]
- [Building](#building) [CRUD]
- [Chromeosdevice](#chromeosdevice) [CRU]
- [Asp](#asp) [RD]
- [Domain_aliase](#domain_aliase) [CRD]
- [Feature](#feature) [CRUD]
- [Print_server](#print_server) [CRUD]
- [Command](#command) [R]
- [Calendar](#calendar) [CRUD]
- [Two_step_verification](#two_step_verification) [C]
- [Verification_code](#verification_code) [CR]
- [Customer](#customer) [RU]
- [Printer](#printer) [CRUD]
- [User](#user) [CRUD]
- [Member](#member) [CRUD]
- [Chromeo](#chromeo) [C]
- [Application](#application) [R]
- [Transfer](#transfer) [CR]
- [Activitie](#activitie) [CR]
- [User_usage_report](#user_usage_report) [R]
- [Entity_usage_report](#entity_usage_report) [R]
- [Channel](#channel) [C]
- [Customer_usage_report](#customer_usage_report) [R]

---

## Resources


### Privilege

Retrieves a paginated list of all privileges for a customer.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of Privilege resources. |
| `etag` | String | ETag of the resource. |
| `kind` | String | The type of the API resource. This is always `admin#directory#privileges`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access privilege outputs
privilege_id = privilege.id
privilege_items = privilege.items
privilege_etag = privilege.etag
privilege_kind = privilege.kind
```

---


### Schema

Creates a schema.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `schema_name` | String |  | The schema's name. Each `schema_name` must be unique within a customer. Reusing a name results in a `409: Entity already exists` error. |
| `kind` | String |  | Kind of resource this is. |
| `schema_id` | String |  | The unique identifier of the schema (Read-only) |
| `display_name` | String |  | Display name for the schema. |
| `etag` | String |  | The ETag of the resource. |
| `fields` | Vec<String> |  | A list of fields in the schema. |
| `customer_id` | String | ✅ | Immutable ID of the Google Workspace account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `schema_name` | String | The schema's name. Each `schema_name` must be unique within a customer. Reusing a name results in a `409: Entity already exists` error. |
| `kind` | String | Kind of resource this is. |
| `schema_id` | String | The unique identifier of the schema (Read-only) |
| `display_name` | String | Display name for the schema. |
| `etag` | String | The ETag of the resource. |
| `fields` | Vec<String> | A list of fields in the schema. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create schema
schema = provider.admin_api.Schema {
    customer_id = "value"  # Immutable ID of the Google Workspace account.
}

# Access schema outputs
schema_id = schema.id
schema_schema_name = schema.schema_name
schema_kind = schema.kind
schema_schema_id = schema.schema_id
schema_display_name = schema.display_name
schema_etag = schema.etag
schema_fields = schema.fields
```

---


### Group

Creates a group.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | An extended description to help users determine the purpose of a group. For example, you can include information about who should join the group, the types of messages to send to the group, links to FAQs about the group, or related groups. Maximum length is `4,096` characters. |
| `email` | String |  | The group's email address. If your account has multiple domains, select the appropriate domain for the email address. The `email` must be unique. This property is required when creating a group. Group email addresses are subject to the same character usage rules as usernames, see the [help center](https://support.google.com/a/answer/9193374) for details. |
| `kind` | String |  | The type of the API resource. For Groups resources, the value is `admin#directory#group`. |
| `aliases` | Vec<String> |  | Read-only. The list of a group's alias email addresses. To add, update, or remove a group's aliases, use the `groups.aliases` methods. If edited in a group's POST or PUT request, the edit is ignored. |
| `etag` | String |  | ETag of the resource. |
| `direct_members_count` | String |  | The number of users that are direct members of the group. If a group is a member (child) of this group (the parent), members of the child group are not counted in the `directMembersCount` property of the parent group. |
| `admin_created` | bool |  | Read-only. Value is `true` if this group was created by an administrator rather than a user. |
| `name` | String |  | The group's display name. |
| `non_editable_aliases` | Vec<String> |  | Read-only. The list of the group's non-editable alias email addresses that are outside of the account's primary domain or subdomains. These are functioning email addresses used by the group. This is a read-only property returned in the API's response for a group. If edited in a group's POST or PUT request, the edit is ignored. |
| `id` | String |  | Read-only. The unique ID of a group. A group `id` can be used as a group request URI's `groupKey`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | An extended description to help users determine the purpose of a group. For example, you can include information about who should join the group, the types of messages to send to the group, links to FAQs about the group, or related groups. Maximum length is `4,096` characters. |
| `email` | String | The group's email address. If your account has multiple domains, select the appropriate domain for the email address. The `email` must be unique. This property is required when creating a group. Group email addresses are subject to the same character usage rules as usernames, see the [help center](https://support.google.com/a/answer/9193374) for details. |
| `kind` | String | The type of the API resource. For Groups resources, the value is `admin#directory#group`. |
| `aliases` | Vec<String> | Read-only. The list of a group's alias email addresses. To add, update, or remove a group's aliases, use the `groups.aliases` methods. If edited in a group's POST or PUT request, the edit is ignored. |
| `etag` | String | ETag of the resource. |
| `direct_members_count` | String | The number of users that are direct members of the group. If a group is a member (child) of this group (the parent), members of the child group are not counted in the `directMembersCount` property of the parent group. |
| `admin_created` | bool | Read-only. Value is `true` if this group was created by an administrator rather than a user. |
| `name` | String | The group's display name. |
| `non_editable_aliases` | Vec<String> | Read-only. The list of the group's non-editable alias email addresses that are outside of the account's primary domain or subdomains. These are functioning email addresses used by the group. This is a read-only property returned in the API's response for a group. If edited in a group's POST or PUT request, the edit is ignored. |
| `id` | String | Read-only. The unique ID of a group. A group `id` can be used as a group request URI's `groupKey`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.admin_api.Group {
}

# Access group outputs
group_id = group.id
group_description = group.description
group_email = group.email
group_kind = group.kind
group_aliases = group.aliases
group_etag = group.etag
group_direct_members_count = group.direct_members_count
group_admin_created = group.admin_created
group_name = group.name
group_non_editable_aliases = group.non_editable_aliases
group_id = group.id
```

---


### Role_assignment

Creates a role assignment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role_assignment_id` | String |  | ID of this roleAssignment. |
| `condition` | String |  | Optional. The condition associated with this role assignment. Note: Feature is available to Enterprise Standard, Enterprise Plus, Google Workspace for Education Plus and Cloud Identity Premium customers. A `RoleAssignment` with the `condition` field set will only take effect when the resource being accessed meets the condition. If `condition` is empty, the role (`role_id`) is applied to the actor (`assigned_to`) at the scope (`scope_type`) unconditionally. Currently, the following conditions are supported: - To make the `RoleAssignment` only applicable to [Security Groups](https://cloud.google.com/identity/docs/groups#group_types): `api.getAttribute('cloudidentity.googleapis.com/groups.labels', []).hasAny(['groups.security']) && resource.type == 'cloudidentity.googleapis.com/Group'` - To make the `RoleAssignment` not applicable to [Security Groups](https://cloud.google.com/identity/docs/groups#group_types): `!api.getAttribute('cloudidentity.googleapis.com/groups.labels', []).hasAny(['groups.security']) && resource.type == 'cloudidentity.googleapis.com/Group'` Currently, the condition strings have to be verbatim and they only work with the following [pre-built administrator roles](https://support.google.com/a/answer/2405986): - Groups Editor - Groups Reader The condition follows [Cloud IAM condition syntax](https://cloud.google.com/iam/docs/conditions-overview). - To make the `RoleAssignment` not applicable to [Locked Groups](https://cloud.google.com/identity/docs/groups#group_types): `!api.getAttribute('cloudidentity.googleapis.com/groups.labels', []).hasAny(['groups.locked']) && resource.type == 'cloudidentity.googleapis.com/Group'` This condition can also be used in conjunction with a Security-related condition. |
| `org_unit_id` | String |  | If the role is restricted to an organization unit, this contains the ID for the organization unit the exercise of this role is restricted to. |
| `role_id` | String |  | The ID of the role that is assigned. |
| `assigned_to` | String |  | The unique ID of the entity this role is assigned to—either the `user_id` of a user, the `group_id` of a group, or the `uniqueId` of a service account as defined in [Identity and Access Management (IAM)](https://cloud.google.com/iam/docs/reference/rest/v1/projects.serviceAccounts). |
| `scope_type` | String |  | The scope in which this role is assigned. |
| `etag` | String |  | ETag of the resource. |
| `assignee_type` | String |  | Output only. The type of the assignee (`USER` or `GROUP`). |
| `kind` | String |  | The type of the API resource. This is always `admin#directory#roleAssignment`. |
| `customer` | String | ✅ | Immutable ID of the Google Workspace account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role_assignment_id` | String | ID of this roleAssignment. |
| `condition` | String | Optional. The condition associated with this role assignment. Note: Feature is available to Enterprise Standard, Enterprise Plus, Google Workspace for Education Plus and Cloud Identity Premium customers. A `RoleAssignment` with the `condition` field set will only take effect when the resource being accessed meets the condition. If `condition` is empty, the role (`role_id`) is applied to the actor (`assigned_to`) at the scope (`scope_type`) unconditionally. Currently, the following conditions are supported: - To make the `RoleAssignment` only applicable to [Security Groups](https://cloud.google.com/identity/docs/groups#group_types): `api.getAttribute('cloudidentity.googleapis.com/groups.labels', []).hasAny(['groups.security']) && resource.type == 'cloudidentity.googleapis.com/Group'` - To make the `RoleAssignment` not applicable to [Security Groups](https://cloud.google.com/identity/docs/groups#group_types): `!api.getAttribute('cloudidentity.googleapis.com/groups.labels', []).hasAny(['groups.security']) && resource.type == 'cloudidentity.googleapis.com/Group'` Currently, the condition strings have to be verbatim and they only work with the following [pre-built administrator roles](https://support.google.com/a/answer/2405986): - Groups Editor - Groups Reader The condition follows [Cloud IAM condition syntax](https://cloud.google.com/iam/docs/conditions-overview). - To make the `RoleAssignment` not applicable to [Locked Groups](https://cloud.google.com/identity/docs/groups#group_types): `!api.getAttribute('cloudidentity.googleapis.com/groups.labels', []).hasAny(['groups.locked']) && resource.type == 'cloudidentity.googleapis.com/Group'` This condition can also be used in conjunction with a Security-related condition. |
| `org_unit_id` | String | If the role is restricted to an organization unit, this contains the ID for the organization unit the exercise of this role is restricted to. |
| `role_id` | String | The ID of the role that is assigned. |
| `assigned_to` | String | The unique ID of the entity this role is assigned to—either the `user_id` of a user, the `group_id` of a group, or the `uniqueId` of a service account as defined in [Identity and Access Management (IAM)](https://cloud.google.com/iam/docs/reference/rest/v1/projects.serviceAccounts). |
| `scope_type` | String | The scope in which this role is assigned. |
| `etag` | String | ETag of the resource. |
| `assignee_type` | String | Output only. The type of the assignee (`USER` or `GROUP`). |
| `kind` | String | The type of the API resource. This is always `admin#directory#roleAssignment`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create role_assignment
role_assignment = provider.admin_api.Role_assignment {
    customer = "value"  # Immutable ID of the Google Workspace account.
}

# Access role_assignment outputs
role_assignment_id = role_assignment.id
role_assignment_role_assignment_id = role_assignment.role_assignment_id
role_assignment_condition = role_assignment.condition
role_assignment_org_unit_id = role_assignment.org_unit_id
role_assignment_role_id = role_assignment.role_id
role_assignment_assigned_to = role_assignment.assigned_to
role_assignment_scope_type = role_assignment.scope_type
role_assignment_etag = role_assignment.etag
role_assignment_assignee_type = role_assignment.assignee_type
role_assignment_kind = role_assignment.kind
```

---


### Aliase

Adds an alias for the group.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alias` | String |  |  |
| `etag` | String |  |  |
| `primary_email` | String |  |  |
| `id` | String |  |  |
| `kind` | String |  |  |
| `group_key` | String | ✅ | Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String |  |
| `kind` | String |  |
| `aliases` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create aliase
aliase = provider.admin_api.Aliase {
    group_key = "value"  # Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
}

# Access aliase outputs
aliase_id = aliase.id
aliase_etag = aliase.etag
aliase_kind = aliase.kind
aliase_aliases = aliase.aliases
```

---


### Domain

Inserts a domain of the customer.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creation_time` | String |  | Creation time of the domain. Expressed in [Unix time](https://en.wikipedia.org/wiki/Epoch_time) format. (Read-only). |
| `is_primary` | bool |  | Indicates if the domain is a primary domain (Read-only). |
| `kind` | String |  | Kind of resource this is. |
| `verified` | bool |  | Indicates the verification state of a domain. (Read-only). |
| `domain_name` | String |  | The domain name of the customer. |
| `etag` | String |  | ETag of the resource. |
| `domain_aliases` | Vec<String> |  | A list of domain alias objects. (Read-only) |
| `customer` | String | ✅ | Immutable ID of the Google Workspace account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | Creation time of the domain. Expressed in [Unix time](https://en.wikipedia.org/wiki/Epoch_time) format. (Read-only). |
| `is_primary` | bool | Indicates if the domain is a primary domain (Read-only). |
| `kind` | String | Kind of resource this is. |
| `verified` | bool | Indicates the verification state of a domain. (Read-only). |
| `domain_name` | String | The domain name of the customer. |
| `etag` | String | ETag of the resource. |
| `domain_aliases` | Vec<String> | A list of domain alias objects. (Read-only) |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domain
domain = provider.admin_api.Domain {
    customer = "value"  # Immutable ID of the Google Workspace account.
}

# Access domain outputs
domain_id = domain.id
domain_creation_time = domain.creation_time
domain_is_primary = domain.is_primary
domain_kind = domain.kind
domain_verified = domain.verified
domain_domain_name = domain.domain_name
domain_etag = domain.etag
domain_domain_aliases = domain.domain_aliases
```

---


### Orgunit

Adds an organizational unit.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Description of the organizational unit. |
| `kind` | String |  | The type of the API resource. For Orgunits resources, the value is `admin#directory#orgUnit`. |
| `block_inheritance` | bool |  | This field is deprecated and setting its value has no effect. |
| `org_unit_path` | String |  | The full path to the organizational unit. The `orgUnitPath` is a derived property. When listed, it is derived from `parentOrgunitPath` and organizational unit's `name`. For example, for an organizational unit named 'apps' under parent organization '/engineering', the orgUnitPath is '/engineering/apps'. In order to edit an `orgUnitPath`, either update the name of the organization or the `parentOrgunitPath`. A user's organizational unit determines which Google Workspace services the user has access to. If the user is moved to a new organization, the user's access changes. For more information about organization structures, see the [administration help center](https://support.google.com/a/answer/4352075). For more information about moving a user to a different organization, see [Update a user](https://developers.google.com/workspace/admin/directory/v1/guides/manage-users.html#update_user). |
| `etag` | String |  | ETag of the resource. |
| `parent_org_unit_id` | String |  | The unique ID of the parent organizational unit. Required, unless `parentOrgUnitPath` is set. |
| `name` | String |  | The organizational unit's path name. For example, an organizational unit's name within the /corp/support/sales_support parent path is sales_support. Required. |
| `org_unit_id` | String |  | The unique ID of the organizational unit. |
| `parent_org_unit_path` | String |  | The organizational unit's parent path. For example, /corp/sales is the parent path for /corp/sales/sales_support organizational unit. Required, unless `parentOrgUnitId` is set. |
| `customer_id` | String | ✅ | The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](https://developers.google.com/workspace/admin/directory/v1/reference/users). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Description of the organizational unit. |
| `kind` | String | The type of the API resource. For Orgunits resources, the value is `admin#directory#orgUnit`. |
| `block_inheritance` | bool | This field is deprecated and setting its value has no effect. |
| `org_unit_path` | String | The full path to the organizational unit. The `orgUnitPath` is a derived property. When listed, it is derived from `parentOrgunitPath` and organizational unit's `name`. For example, for an organizational unit named 'apps' under parent organization '/engineering', the orgUnitPath is '/engineering/apps'. In order to edit an `orgUnitPath`, either update the name of the organization or the `parentOrgunitPath`. A user's organizational unit determines which Google Workspace services the user has access to. If the user is moved to a new organization, the user's access changes. For more information about organization structures, see the [administration help center](https://support.google.com/a/answer/4352075). For more information about moving a user to a different organization, see [Update a user](https://developers.google.com/workspace/admin/directory/v1/guides/manage-users.html#update_user). |
| `etag` | String | ETag of the resource. |
| `parent_org_unit_id` | String | The unique ID of the parent organizational unit. Required, unless `parentOrgUnitPath` is set. |
| `name` | String | The organizational unit's path name. For example, an organizational unit's name within the /corp/support/sales_support parent path is sales_support. Required. |
| `org_unit_id` | String | The unique ID of the organizational unit. |
| `parent_org_unit_path` | String | The organizational unit's parent path. For example, /corp/sales is the parent path for /corp/sales/sales_support organizational unit. Required, unless `parentOrgUnitId` is set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create orgunit
orgunit = provider.admin_api.Orgunit {
    customer_id = "value"  # The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](https://developers.google.com/workspace/admin/directory/v1/reference/users).
}

# Access orgunit outputs
orgunit_id = orgunit.id
orgunit_description = orgunit.description
orgunit_kind = orgunit.kind
orgunit_block_inheritance = orgunit.block_inheritance
orgunit_org_unit_path = orgunit.org_unit_path
orgunit_etag = orgunit.etag
orgunit_parent_org_unit_id = orgunit.parent_org_unit_id
orgunit_name = orgunit.name
orgunit_org_unit_id = orgunit.org_unit_id
orgunit_parent_org_unit_path = orgunit.parent_org_unit_path
```

---


### Channel

Stops watching resources through this channel.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. For example, `params.ttl` specifies the time-to-live in seconds for the notification channel, where the default is 2 hours and the maximum TTL is 2 days. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | String |  | The type of delivery mechanism used for this channel. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel
channel = provider.admin_api.Channel {
}

```

---


### Mobiledevice

Takes an action that affects a mobile device. For example, remotely wiping a device.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String |  | The action to be performed on the device. |
| `customer_id` | String | ✅ | The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](https://developers.google.com/workspace/admin/directory/v1/reference/users). |
| `resource_id` | String | ✅ | The unique ID the API service uses to identify the mobile device. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | The type of the API resource. For Mobiledevices resources, the value is `admin#directory#mobiledevice`. |
| `resource_id` | String | The unique ID the API service uses to identify the mobile device. |
| `security_patch_level` | String | Mobile Device Security patch level (Read-only) |
| `etag` | String | ETag of the resource. |
| `hardware` | String | Mobile Device Hardware (Read-only) |
| `applications` | Vec<String> | The list of applications installed on an Android mobile device. It is not applicable to Google Sync and iOS devices. The list includes any Android applications that access Google Workspace data. When updating an applications list, it is important to note that updates replace the existing list. If the Android device has two existing applications and the API updates the list with five applications, the is now the updated list of five applications. |
| `device_password_status` | String | DevicePasswordStatus (Read-only) |
| `email` | Vec<String> | The list of the owner's email addresses. If your application needs the current list of user emails, use the [get](https://developers.google.com/workspace/admin/directory/v1/reference/mobiledevices/get.html) method. For additional information, see the [retrieve a user](https://developers.google.com/workspace/admin/directory/v1/guides/manage-users#get_user) method. |
| `device_id` | String | The serial number for a Google Sync mobile device. For Android and iOS devices, this is a software generated unique identifier. |
| `device_compromised_status` | String | The compromised device status. |
| `model` | String | The mobile device's model name, for example Nexus S. This property can be [updated](https://developers.google.com/workspace/admin/directory/v1/reference/mobiledevices/update.html). For more information, see the [Developer's Guide](https://developers.google.com/workspace/admin/directory/v1/guides/manage-mobile=devices#update_mobile_device). |
| `kernel_version` | String | The device's kernel version. |
| `supports_work_profile` | bool | Work profile supported on device (Read-only) |
| `default_language` | String | The default locale used on the device. |
| `other_accounts_info` | Vec<String> | The list of accounts added on device (Read-only) |
| `first_sync` | String | Date and time the device was first synchronized with the policy settings in the G Suite administrator control panel (Read-only) |
| `developer_options_status` | bool | Developer options enabled or disabled on device (Read-only) |
| `type` | String | The type of mobile device. |
| `last_sync` | String | Date and time the device was last synchronized with the policy settings in the G Suite administrator control panel (Read-only) |
| `os` | String | The mobile device's operating system, for example IOS 4.3 or Android 2.3.5. This property can be [updated](https://developers.google.com/workspace/admin/directory/v1/reference/mobiledevices/update.html). For more information, see the [Developer's Guide](https://developers.google.com/workspace/admin/directory/v1/guides/manage-mobile-devices#update_mobile_device). |
| `baseband_version` | String | The device's baseband version. |
| `meid` | String | The device's MEID number. |
| `status` | String | The device's status. |
| `encryption_status` | String | Mobile Device Encryption Status (Read-only) |
| `serial_number` | String | The device's serial number. |
| `hardware_id` | String | The IMEI/MEID unique identifier for Android hardware. It is not applicable to Google Sync devices. When adding an Android mobile device, this is an optional property. When updating one of these devices, this is a read-only property. |
| `build_number` | String | The device's operating system build number. |
| `imei` | String | The device's IMEI number. |
| `privilege` | String | DMAgentPermission (Read-only) |
| `adb_status` | bool | Adb (USB debugging) enabled or disabled on device (Read-only) |
| `unknown_sources_status` | bool | Unknown sources enabled or disabled on device (Read-only) |
| `network_operator` | String | Mobile Device mobile or network operator (if available) (Read-only) |
| `managed_account_is_on_owner_profile` | bool | Boolean indicating if this account is on owner/primary profile or not. |
| `release_version` | String | Mobile Device release version version (Read-only) |
| `user_agent` | String | Gives information about the device such as `os` version. This property can be [updated](https://developers.google.com/workspace/admin/directory/v1/reference/mobiledevices/update.html). For more information, see the [Developer's Guide](https://developers.google.com/workspace/admin/directory/v1/guides/manage-mobile-devices#update_mobile_device). |
| `wifi_mac_address` | String | The device's MAC address on Wi-Fi networks. |
| `bootloader_version` | String | Mobile Device Bootloader version (Read-only) |
| `name` | Vec<String> | The list of the owner's user names. If your application needs the current list of device owner names, use the [get](https://developers.google.com/workspace/admin/directory/v1/reference/mobiledevices/get.html) method. For more information about retrieving mobile device user information, see the [Developer's Guide](https://developers.google.com/workspace/admin/directory/v1/guides/manage-users#get_user). |
| `brand` | String | Mobile Device Brand (Read-only) |
| `manufacturer` | String | Mobile Device manufacturer (Read-only) |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mobiledevice
mobiledevice = provider.admin_api.Mobiledevice {
    customer_id = "value"  # The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](https://developers.google.com/workspace/admin/directory/v1/reference/users).
    resource_id = "value"  # The unique ID the API service uses to identify the mobile device.
}

# Access mobiledevice outputs
mobiledevice_id = mobiledevice.id
mobiledevice_kind = mobiledevice.kind
mobiledevice_resource_id = mobiledevice.resource_id
mobiledevice_security_patch_level = mobiledevice.security_patch_level
mobiledevice_etag = mobiledevice.etag
mobiledevice_hardware = mobiledevice.hardware
mobiledevice_applications = mobiledevice.applications
mobiledevice_device_password_status = mobiledevice.device_password_status
mobiledevice_email = mobiledevice.email
mobiledevice_device_id = mobiledevice.device_id
mobiledevice_device_compromised_status = mobiledevice.device_compromised_status
mobiledevice_model = mobiledevice.model
mobiledevice_kernel_version = mobiledevice.kernel_version
mobiledevice_supports_work_profile = mobiledevice.supports_work_profile
mobiledevice_default_language = mobiledevice.default_language
mobiledevice_other_accounts_info = mobiledevice.other_accounts_info
mobiledevice_first_sync = mobiledevice.first_sync
mobiledevice_developer_options_status = mobiledevice.developer_options_status
mobiledevice_type = mobiledevice.type
mobiledevice_last_sync = mobiledevice.last_sync
mobiledevice_os = mobiledevice.os
mobiledevice_baseband_version = mobiledevice.baseband_version
mobiledevice_meid = mobiledevice.meid
mobiledevice_status = mobiledevice.status
mobiledevice_encryption_status = mobiledevice.encryption_status
mobiledevice_serial_number = mobiledevice.serial_number
mobiledevice_hardware_id = mobiledevice.hardware_id
mobiledevice_build_number = mobiledevice.build_number
mobiledevice_imei = mobiledevice.imei
mobiledevice_privilege = mobiledevice.privilege
mobiledevice_adb_status = mobiledevice.adb_status
mobiledevice_unknown_sources_status = mobiledevice.unknown_sources_status
mobiledevice_network_operator = mobiledevice.network_operator
mobiledevice_managed_account_is_on_owner_profile = mobiledevice.managed_account_is_on_owner_profile
mobiledevice_release_version = mobiledevice.release_version
mobiledevice_user_agent = mobiledevice.user_agent
mobiledevice_wifi_mac_address = mobiledevice.wifi_mac_address
mobiledevice_bootloader_version = mobiledevice.bootloader_version
mobiledevice_name = mobiledevice.name
mobiledevice_brand = mobiledevice.brand
mobiledevice_manufacturer = mobiledevice.manufacturer
```

---


### Photo

Retrieves the user's photo.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `photo_data` | String |  | The user photo's upload data in [web-safe Base64](https://en.wikipedia.org/wiki/Base64#URL_applications) format in bytes. This means: * The slash (/) character is replaced with the underscore (_) character. * The plus sign (+) character is replaced with the hyphen (-) character. * The equals sign (=) character is replaced with the asterisk (*). * For padding, the period (.) character is used instead of the RFC-4648 baseURL definition which uses the equals sign (=) for padding. This is done to simplify URL-parsing. * Whatever the size of the photo being uploaded, the API downsizes it to 96x96 pixels. |
| `etag` | String |  | ETag of the resource. |
| `kind` | String |  | The type of the API resource. For Photo resources, this is `admin#directory#user#photo`. |
| `primary_email` | String |  | The user's primary email address. |
| `height` | i64 |  | Height of the photo in pixels. |
| `id` | String |  | The ID the API uses to uniquely identify the user. |
| `mime_type` | String |  | The MIME type of the photo. Allowed values are `JPEG`, `PNG`, `GIF`, `BMP`, `TIFF`, and web-safe base64 encoding. |
| `width` | i64 |  | Width of the photo in pixels. |
| `user_key` | String | ✅ | Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `photo_data` | String | The user photo's upload data in [web-safe Base64](https://en.wikipedia.org/wiki/Base64#URL_applications) format in bytes. This means: * The slash (/) character is replaced with the underscore (_) character. * The plus sign (+) character is replaced with the hyphen (-) character. * The equals sign (=) character is replaced with the asterisk (*). * For padding, the period (.) character is used instead of the RFC-4648 baseURL definition which uses the equals sign (=) for padding. This is done to simplify URL-parsing. * Whatever the size of the photo being uploaded, the API downsizes it to 96x96 pixels. |
| `etag` | String | ETag of the resource. |
| `kind` | String | The type of the API resource. For Photo resources, this is `admin#directory#user#photo`. |
| `primary_email` | String | The user's primary email address. |
| `height` | i64 | Height of the photo in pixels. |
| `id` | String | The ID the API uses to uniquely identify the user. |
| `mime_type` | String | The MIME type of the photo. Allowed values are `JPEG`, `PNG`, `GIF`, `BMP`, `TIFF`, and web-safe base64 encoding. |
| `width` | i64 | Width of the photo in pixels. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access photo outputs
photo_id = photo.id
photo_photo_data = photo.photo_data
photo_etag = photo.etag
photo_kind = photo.kind
photo_primary_email = photo.primary_email
photo_height = photo.height
photo_id = photo.id
photo_mime_type = photo.mime_type
photo_width = photo.width
```

---


### Token

Gets information about an access token issued by a user.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `anonymous` | bool | Whether the application is registered with Google. The value is `true` if the application has an anonymous Client ID. |
| `display_text` | String | The displayable name of the application the token is issued to. |
| `etag` | String | ETag of the resource. |
| `native_app` | bool | Whether the token is issued to an installed application. The value is `true` if the application is installed to a desktop or mobile device. |
| `scopes` | Vec<String> | A list of authorization scopes the application is granted. |
| `user_key` | String | The unique ID of the user that issued the token. |
| `client_id` | String | The Client ID of the application the token is issued to. |
| `kind` | String | The type of the API resource. This is always `admin#directory#token`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access token outputs
token_id = token.id
token_anonymous = token.anonymous
token_display_text = token.display_text
token_etag = token.etag
token_native_app = token.native_app
token_scopes = token.scopes
token_user_key = token.user_key
token_client_id = token.client_id
token_kind = token.kind
```

---


### Role

Creates a role.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `is_system_role` | bool |  | Returns `true` if this is a pre-defined system role. |
| `etag` | String |  | ETag of the resource. |
| `role_description` | String |  | A short description of the role. |
| `is_super_admin_role` | bool |  | Returns `true` if the role is a super admin role. |
| `role_privileges` | Vec<String> |  | The set of privileges that are granted to this role. |
| `kind` | String |  | The type of the API resource. This is always `admin#directory#role`. |
| `role_id` | String |  | ID of the role. |
| `role_name` | String |  | Name of the role. |
| `customer` | String | ✅ | Immutable ID of the Google Workspace account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_system_role` | bool | Returns `true` if this is a pre-defined system role. |
| `etag` | String | ETag of the resource. |
| `role_description` | String | A short description of the role. |
| `is_super_admin_role` | bool | Returns `true` if the role is a super admin role. |
| `role_privileges` | Vec<String> | The set of privileges that are granted to this role. |
| `kind` | String | The type of the API resource. This is always `admin#directory#role`. |
| `role_id` | String | ID of the role. |
| `role_name` | String | Name of the role. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create role
role = provider.admin_api.Role {
    customer = "value"  # Immutable ID of the Google Workspace account.
}

# Access role outputs
role_id = role.id
role_is_system_role = role.is_system_role
role_etag = role.etag
role_role_description = role.role_description
role_is_super_admin_role = role.is_super_admin_role
role_role_privileges = role.role_privileges
role_kind = role.kind
role_role_id = role.role_id
role_role_name = role.role_name
```

---


### Building

Inserts a building.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `floor_names` | Vec<String> |  | The display names for all floors in this building. The floors are expected to be sorted in ascending order, from lowest floor to highest floor. For example, ["B2", "B1", "L", "1", "2", "2M", "3", "PH"] Must contain at least one entry. |
| `address` | String |  | The postal address of the building. See [`PostalAddress`](/my-business/reference/rest/v4/PostalAddress) for details. Note that only a single address line and region code are required. |
| `building_id` | String |  | Unique identifier for the building. The maximum length is 100 characters. |
| `coordinates` | String |  | The geographic coordinates of the center of the building, expressed as latitude and longitude in decimal degrees. |
| `building_name` | String |  | The building name as seen by users in Calendar. Must be unique for the customer. For example, "NYC-CHEL". The maximum length is 100 characters. |
| `description` | String |  | A brief description of the building. For example, "Chelsea Market". |
| `etags` | String |  | ETag of the resource. |
| `kind` | String |  | Kind of resource this is. |
| `customer` | String | ✅ | The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `floor_names` | Vec<String> | The display names for all floors in this building. The floors are expected to be sorted in ascending order, from lowest floor to highest floor. For example, ["B2", "B1", "L", "1", "2", "2M", "3", "PH"] Must contain at least one entry. |
| `address` | String | The postal address of the building. See [`PostalAddress`](/my-business/reference/rest/v4/PostalAddress) for details. Note that only a single address line and region code are required. |
| `building_id` | String | Unique identifier for the building. The maximum length is 100 characters. |
| `coordinates` | String | The geographic coordinates of the center of the building, expressed as latitude and longitude in decimal degrees. |
| `building_name` | String | The building name as seen by users in Calendar. Must be unique for the customer. For example, "NYC-CHEL". The maximum length is 100 characters. |
| `description` | String | A brief description of the building. For example, "Chelsea Market". |
| `etags` | String | ETag of the resource. |
| `kind` | String | Kind of resource this is. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create building
building = provider.admin_api.Building {
    customer = "value"  # The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
}

# Access building outputs
building_id = building.id
building_floor_names = building.floor_names
building_address = building.address
building_building_id = building.building_id
building_coordinates = building.coordinates
building_building_name = building.building_name
building_description = building.description
building_etags = building.etags
building_kind = building.kind
```

---


### Chromeosdevice

Use [BatchChangeChromeOsDeviceStatus](https://developers.google.com/workspace/admin/directory/reference/rest/v1/customer.devices.chromeos/batchChangeStatus) instead. Takes an action that affects a Chrome OS Device. This includes deprovisioning, disabling, and re-enabling devices. *Warning:* * Deprovisioning a device will stop device policy syncing and remove device-level printers. After a device is deprovisioned, it must be wiped before it can be re-enrolled. * Lost or stolen devices should use the disable action. * Re-enabling a disabled device will consume a device license. If you do not have sufficient licenses available when completing the re-enable action, you will receive an error. For more information about deprovisioning and disabling devices, visit the [help center](https://support.google.com/chrome/a/answer/3523633).

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deprovision_reason` | String |  | Only used when the action is `deprovision`. With the `deprovision` action, this field is required. *Note*: The deprovision reason is audited because it might have implications on licenses for perpetual subscription customers. |
| `action` | String |  | Action to be taken on the Chrome OS device. |
| `resource_id` | String | ✅ | The unique ID of the device. The `resourceId`s are returned in the response from the [chromeosdevices.list](https://developers.google.com/workspace/admin/directory/v1/reference/chromeosdevices/list) method. |
| `customer_id` | String | ✅ | The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](https://developers.google.com/workspace/admin/directory/v1/reference/users). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `os_version` | String | The Chrome device's operating system version. |
| `kind` | String | The type of resource. For the Chromeosdevices resource, the value is `admin#directory#chromeosdevice`. |
| `will_auto_renew` | bool | Determines if the device will auto renew its support after the support end date. This is a read-only property. |
| `last_sync` | String | Date and time the device was last synchronized with the policy settings in the G Suite administrator control panel (Read-only) |
| `device_license_type` | String | Output only. Device license type. |
| `extended_support_start` | String | Output only. Date of the device when extended support policy for automatic updates starts. |
| `fan_info` | Vec<String> | Output only. Fan information for the device. |
| `cpu_status_reports` | Vec<String> | Reports of CPU utilization and temperature (Read-only) |
| `disk_volume_reports` | Vec<String> | Reports of disk space and other info about mounted/connected volumes. |
| `dock_mac_address` | String | (Read-only) Built-in MAC address for the docking station that the device connected to. Factory sets Media access control address (MAC address) assigned for use by a dock. It is reserved specifically for MAC pass through device policy. The format is twelve (12) hexadecimal digits without any delimiter (uppercase letters). This is only relevant for some devices. |
| `etag` | String | ETag of the resource. |
| `last_known_network` | Vec<String> | Contains last known network (Read-only) |
| `extended_support_eligible` | bool | Output only. Whether or not the device requires the extended support opt in. |
| `annotated_user` | String | The user of the device as noted by the administrator. Maximum length is 100 characters. Empty values are allowed. |
| `manufacture_date` | String | (Read-only) The date the device was manufactured in yyyy-mm-dd format. |
| `os_update_status` | String | The status of the OS updates for the device. |
| `auto_update_through` | String | Output only. The timestamp after which the device will stop receiving Chrome updates or support. |
| `device_files` | Vec<String> | A list of device files to download (Read-only) |
| `cpu_info` | Vec<String> | Information regarding CPU specs in the device. |
| `first_enrollment_time` | String | Date and time for the first time the device was enrolled. |
| `model` | String | The device's model information. If the device does not have this information, this property is not included in the response. |
| `ethernet_mac_address0` | String | (Read-only) MAC address used by the Chromebook’s internal ethernet port, and for onboard network (ethernet) interface. The format is twelve (12) hexadecimal digits without any delimiter (uppercase letters). This is only relevant for some devices. |
| `screenshot_files` | Vec<String> | A list of screenshot files to download. Type is always "SCREENSHOT_FILE". (Read-only) |
| `status` | String | The status of the device. |
| `org_unit_id` | String | The unique ID of the organizational unit. orgUnitPath is the human readable version of orgUnitId. While orgUnitPath may change by renaming an organizational unit within the path, orgUnitId is unchangeable for one organizational unit. This property can be [updated](https://developers.google.com/workspace/admin/directory/v1/guides/manage-chrome-devices#move_chrome_devices_to_ou) using the API. For more information about how to create an organizational structure for your device, see the [administration help center](https://support.google.com/a/answer/182433). |
| `annotated_asset_id` | String | The asset identifier as noted by an administrator or specified during enrollment. |
| `annotated_location` | String | The address or location of the device as noted by the administrator. Maximum length is `200` characters. Empty values are allowed. |
| `platform_version` | String | The Chrome device's platform version. |
| `recent_users` | Vec<String> | A list of recent device users, in descending order, by last login time. |
| `support_end_date` | String | Final date the device will be supported (Read-only) |
| `ethernet_mac_address` | String | The device's MAC address on the ethernet network interface. |
| `tpm_version_info` | String | Trusted Platform Module (TPM) (Read-only) |
| `serial_number` | String | The Chrome device serial number entered when the device was enabled. This value is the same as the Admin console's *Serial Number* in the *Chrome OS Devices* tab. |
| `order_number` | String | The device's order number. Only devices directly purchased from Google have an order number. |
| `deprovision_reason` | String | (Read-only) Deprovision reason. |
| `backlight_info` | Vec<String> | Output only. Contains backlight information for the device. |
| `active_time_ranges` | Vec<String> | A list of active time ranges (Read-only). |
| `disk_space_usage` | String | Output only. How much disk space the device has available and is currently using. |
| `meid` | String | The Mobile Equipment Identifier (MEID) or the International Mobile Equipment Identity (IMEI) for the 3G mobile card in a mobile device. A MEID/IMEI is typically used when adding a device to a wireless carrier's post-pay service plan. If the device does not have this information, this property is not included in the response. For more information on how to export a MEID/IMEI list, see the [Developer's Guide](https://developers.google.com/workspace/admin/directory/v1/guides/manage-chrome-devices.html#export_meid). |
| `auto_update_expiration` | String | (Read-only) The timestamp after which the device will stop receiving Chrome updates or support. Please use "autoUpdateThrough" instead. |
| `mac_address` | String | The device's wireless MAC address. If the device does not have this information, it is not included in the response. |
| `notes` | String | Notes about this device added by the administrator. This property can be [searched](https://support.google.com/chrome/a/answer/1698333) with the [list](https://developers.google.com/workspace/admin/directory/v1/reference/chromeosdevices/list) method's `query` parameter. Maximum length is 500 characters. Empty values are allowed. |
| `bluetooth_adapter_info` | Vec<String> | Output only. Information about bluetooth adapters of the device. |
| `device_id` | String | The unique ID of the Chrome device. |
| `extended_support_enabled` | bool | Output only. Whether extended support policy is enabled on the device. |
| `last_enrollment_time` | String | Date and time the device was last enrolled (Read-only) |
| `os_version_compliance` | String | Output only. Device policy compliance status of the OS version. |
| `system_ram_total` | String | Total RAM on the device [in bytes] (Read-only) |
| `firmware_version` | String | The Chrome device's firmware version. |
| `chrome_os_type` | String | Output only. Chrome OS type of the device. |
| `last_deprovision_timestamp` | String | (Read-only) Date and time for the last deprovision of the device. |
| `org_unit_path` | String | The full parent path with the organizational unit's name associated with the device. Path names are case insensitive. If the parent organizational unit is the top-level organization, it is represented as a forward slash, `/`. This property can be [updated](https://developers.google.com/workspace/admin/directory/v1/guides/manage-chrome-devices#move_chrome_devices_to_ou) using the API. For more information about how to create an organizational structure for your device, see the [administration help center](https://support.google.com/a/answer/182433). |
| `system_ram_free_reports` | Vec<String> | Reports of amounts of available RAM memory (Read-only) |
| `boot_mode` | String | The boot mode for the device. The possible values are: * `Verified`: The device is running a valid version of the Chrome OS. * `Dev`: The devices's developer hardware switch is enabled. When booted, the device has a command line shell. For an example of a developer switch, see the [Chromebook developer information](https://www.chromium.org/chromium-os/developer-information-for-chrome-os-devices/samsung-series-5-chromebook#TOC-Developer-switch). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create chromeosdevice
chromeosdevice = provider.admin_api.Chromeosdevice {
    resource_id = "value"  # The unique ID of the device. The `resourceId`s are returned in the response from the [chromeosdevices.list](https://developers.google.com/workspace/admin/directory/v1/reference/chromeosdevices/list) method.
    customer_id = "value"  # The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](https://developers.google.com/workspace/admin/directory/v1/reference/users).
}

# Access chromeosdevice outputs
chromeosdevice_id = chromeosdevice.id
chromeosdevice_os_version = chromeosdevice.os_version
chromeosdevice_kind = chromeosdevice.kind
chromeosdevice_will_auto_renew = chromeosdevice.will_auto_renew
chromeosdevice_last_sync = chromeosdevice.last_sync
chromeosdevice_device_license_type = chromeosdevice.device_license_type
chromeosdevice_extended_support_start = chromeosdevice.extended_support_start
chromeosdevice_fan_info = chromeosdevice.fan_info
chromeosdevice_cpu_status_reports = chromeosdevice.cpu_status_reports
chromeosdevice_disk_volume_reports = chromeosdevice.disk_volume_reports
chromeosdevice_dock_mac_address = chromeosdevice.dock_mac_address
chromeosdevice_etag = chromeosdevice.etag
chromeosdevice_last_known_network = chromeosdevice.last_known_network
chromeosdevice_extended_support_eligible = chromeosdevice.extended_support_eligible
chromeosdevice_annotated_user = chromeosdevice.annotated_user
chromeosdevice_manufacture_date = chromeosdevice.manufacture_date
chromeosdevice_os_update_status = chromeosdevice.os_update_status
chromeosdevice_auto_update_through = chromeosdevice.auto_update_through
chromeosdevice_device_files = chromeosdevice.device_files
chromeosdevice_cpu_info = chromeosdevice.cpu_info
chromeosdevice_first_enrollment_time = chromeosdevice.first_enrollment_time
chromeosdevice_model = chromeosdevice.model
chromeosdevice_ethernet_mac_address0 = chromeosdevice.ethernet_mac_address0
chromeosdevice_screenshot_files = chromeosdevice.screenshot_files
chromeosdevice_status = chromeosdevice.status
chromeosdevice_org_unit_id = chromeosdevice.org_unit_id
chromeosdevice_annotated_asset_id = chromeosdevice.annotated_asset_id
chromeosdevice_annotated_location = chromeosdevice.annotated_location
chromeosdevice_platform_version = chromeosdevice.platform_version
chromeosdevice_recent_users = chromeosdevice.recent_users
chromeosdevice_support_end_date = chromeosdevice.support_end_date
chromeosdevice_ethernet_mac_address = chromeosdevice.ethernet_mac_address
chromeosdevice_tpm_version_info = chromeosdevice.tpm_version_info
chromeosdevice_serial_number = chromeosdevice.serial_number
chromeosdevice_order_number = chromeosdevice.order_number
chromeosdevice_deprovision_reason = chromeosdevice.deprovision_reason
chromeosdevice_backlight_info = chromeosdevice.backlight_info
chromeosdevice_active_time_ranges = chromeosdevice.active_time_ranges
chromeosdevice_disk_space_usage = chromeosdevice.disk_space_usage
chromeosdevice_meid = chromeosdevice.meid
chromeosdevice_auto_update_expiration = chromeosdevice.auto_update_expiration
chromeosdevice_mac_address = chromeosdevice.mac_address
chromeosdevice_notes = chromeosdevice.notes
chromeosdevice_bluetooth_adapter_info = chromeosdevice.bluetooth_adapter_info
chromeosdevice_device_id = chromeosdevice.device_id
chromeosdevice_extended_support_enabled = chromeosdevice.extended_support_enabled
chromeosdevice_last_enrollment_time = chromeosdevice.last_enrollment_time
chromeosdevice_os_version_compliance = chromeosdevice.os_version_compliance
chromeosdevice_system_ram_total = chromeosdevice.system_ram_total
chromeosdevice_firmware_version = chromeosdevice.firmware_version
chromeosdevice_chrome_os_type = chromeosdevice.chrome_os_type
chromeosdevice_last_deprovision_timestamp = chromeosdevice.last_deprovision_timestamp
chromeosdevice_org_unit_path = chromeosdevice.org_unit_path
chromeosdevice_system_ram_free_reports = chromeosdevice.system_ram_free_reports
chromeosdevice_boot_mode = chromeosdevice.boot_mode
```

---


### Asp

Gets information about an ASP issued by a user.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | The time when the ASP was created. Expressed in [Unix time](https://en.wikipedia.org/wiki/Epoch_time) format. |
| `name` | String | The name of the application that the user, represented by their `userId`, entered when the ASP was created. |
| `user_key` | String | The unique ID of the user who issued the ASP. |
| `kind` | String | The type of the API resource. This is always `admin#directory#asp`. |
| `code_id` | i64 | The unique ID of the ASP. |
| `etag` | String | ETag of the ASP. |
| `last_time_used` | String | The time when the ASP was last used. Expressed in [Unix time](https://en.wikipedia.org/wiki/Epoch_time) format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access asp outputs
asp_id = asp.id
asp_creation_time = asp.creation_time
asp_name = asp.name
asp_user_key = asp.user_key
asp_kind = asp.kind
asp_code_id = asp.code_id
asp_etag = asp.etag
asp_last_time_used = asp.last_time_used
```

---


### Domain_aliase

Inserts a domain alias of the customer.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Kind of resource this is. |
| `domain_alias_name` | String |  | The domain alias name. |
| `parent_domain_name` | String |  | The parent domain name that the domain alias is associated with. This can either be a primary or secondary domain name within a customer. |
| `verified` | bool |  | Indicates the verification state of a domain alias. (Read-only) |
| `etag` | String |  | ETag of the resource. |
| `creation_time` | String |  | The creation time of the domain alias. (Read-only). |
| `customer` | String | ✅ | Immutable ID of the Google Workspace account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of resource this is. |
| `domain_alias_name` | String | The domain alias name. |
| `parent_domain_name` | String | The parent domain name that the domain alias is associated with. This can either be a primary or secondary domain name within a customer. |
| `verified` | bool | Indicates the verification state of a domain alias. (Read-only) |
| `etag` | String | ETag of the resource. |
| `creation_time` | String | The creation time of the domain alias. (Read-only). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domain_aliase
domain_aliase = provider.admin_api.Domain_aliase {
    customer = "value"  # Immutable ID of the Google Workspace account.
}

# Access domain_aliase outputs
domain_aliase_id = domain_aliase.id
domain_aliase_kind = domain_aliase.kind
domain_aliase_domain_alias_name = domain_aliase.domain_alias_name
domain_aliase_parent_domain_name = domain_aliase.parent_domain_name
domain_aliase_verified = domain_aliase.verified
domain_aliase_etag = domain_aliase.etag
domain_aliase_creation_time = domain_aliase.creation_time
```

---


### Feature

Inserts a feature.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Kind of resource this is. |
| `name` | String |  | The name of the feature. |
| `etags` | String |  | ETag of the resource. |
| `customer` | String | ✅ | The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Kind of resource this is. |
| `name` | String | The name of the feature. |
| `etags` | String | ETag of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feature
feature = provider.admin_api.Feature {
    customer = "value"  # The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
}

# Access feature outputs
feature_id = feature.id
feature_kind = feature.kind
feature_name = feature.name
feature_etags = feature.etags
```

---


### Print_server

Creates a print server.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `org_unit_id` | String |  | ID of the organization unit (OU) that owns this print server. This value can only be set when the print server is initially created. If it's not populated, the print server is placed under the root OU. The `org_unit_id` can be retrieved using the [Directory API](https://developers.google.com/workspace/admin/directory/reference/rest/v1/orgunits). |
| `create_time` | String |  | Output only. Time when the print server was created. |
| `name` | String |  | Identifier. Resource name of the print server. Leave empty when creating. Format: `customers/{customer.id}/printServers/{print_server.id}` |
| `uri` | String |  | Editable. Print server URI. |
| `display_name` | String |  | Editable. Display name of the print server (as shown in the Admin console). |
| `id` | String |  | Immutable. ID of the print server. Leave empty when creating. |
| `description` | String |  | Editable. Description of the print server (as shown in the Admin console). |
| `parent` | String | ✅ | Required. The [unique ID](https://developers.google.com/workspace/admin/directory/reference/rest/v1/customers) of the customer's Google Workspace account. Format: `customers/{id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `org_unit_id` | String | ID of the organization unit (OU) that owns this print server. This value can only be set when the print server is initially created. If it's not populated, the print server is placed under the root OU. The `org_unit_id` can be retrieved using the [Directory API](https://developers.google.com/workspace/admin/directory/reference/rest/v1/orgunits). |
| `create_time` | String | Output only. Time when the print server was created. |
| `name` | String | Identifier. Resource name of the print server. Leave empty when creating. Format: `customers/{customer.id}/printServers/{print_server.id}` |
| `uri` | String | Editable. Print server URI. |
| `display_name` | String | Editable. Display name of the print server (as shown in the Admin console). |
| `id` | String | Immutable. ID of the print server. Leave empty when creating. |
| `description` | String | Editable. Description of the print server (as shown in the Admin console). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create print_server
print_server = provider.admin_api.Print_server {
    parent = "value"  # Required. The [unique ID](https://developers.google.com/workspace/admin/directory/reference/rest/v1/customers) of the customer's Google Workspace account. Format: `customers/{id}`
}

# Access print_server outputs
print_server_id = print_server.id
print_server_org_unit_id = print_server.org_unit_id
print_server_create_time = print_server.create_time
print_server_name = print_server.name
print_server_uri = print_server.uri
print_server_display_name = print_server.display_name
print_server_id = print_server.id
print_server_description = print_server.description
```

---


### Command

Gets command data a specific command issued to the device.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `command_expire_time` | String | The time at which the command will expire. If the device doesn't execute the command within this time the command will become expired. |
| `payload` | String | The payload that the command specified, if any. |
| `issue_time` | String | The timestamp when the command was issued by the admin. |
| `type` | String | The type of the command. |
| `command_id` | String | Unique ID of a device command. |
| `state` | String | Indicates the command state. |
| `command_result` | String | The result of the command execution. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access command outputs
command_id = command.id
command_command_expire_time = command.command_expire_time
command_payload = command.payload
command_issue_time = command.issue_time
command_type = command.type
command_command_id = command.command_id
command_state = command.state
command_command_result = command.command_result
```

---


### Calendar

Inserts a calendar resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etags` | String |  | ETag of the resource. |
| `floor_name` | String |  | Name of the floor a resource is located on. |
| `resource_name` | String |  | The name of the calendar resource. For example, "Training Room 1A". |
| `feature_instances` | String |  | Instances of features for the calendar resource. |
| `generated_resource_name` | String |  | The read-only auto-generated name of the calendar resource which includes metadata about the resource such as building name, floor, capacity, etc. For example, "NYC-2-Training Room 1A (16)". |
| `resource_description` | String |  | Description of the resource, visible only to admins. |
| `user_visible_description` | String |  | Description of the resource, visible to users and admins. |
| `resource_email` | String |  | The read-only email for the calendar resource. Generated as part of creating a new calendar resource. |
| `resource_category` | String |  | The category of the calendar resource. Either CONFERENCE_ROOM or OTHER. Legacy data is set to CATEGORY_UNKNOWN. |
| `floor_section` | String |  | Name of the section within a floor a resource is located in. |
| `kind` | String |  | The type of the resource. For calendar resources, the value is `admin#directory#resources#calendars#CalendarResource`. |
| `building_id` | String |  | Unique ID for the building a resource is located in. |
| `resource_id` | String |  | The unique ID for the calendar resource. |
| `resource_type` | String |  | The type of the calendar resource, intended for non-room resources. |
| `capacity` | i64 |  | Capacity of a resource, number of seats in a room. |
| `customer` | String | ✅ | The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etags` | String | ETag of the resource. |
| `floor_name` | String | Name of the floor a resource is located on. |
| `resource_name` | String | The name of the calendar resource. For example, "Training Room 1A". |
| `feature_instances` | String | Instances of features for the calendar resource. |
| `generated_resource_name` | String | The read-only auto-generated name of the calendar resource which includes metadata about the resource such as building name, floor, capacity, etc. For example, "NYC-2-Training Room 1A (16)". |
| `resource_description` | String | Description of the resource, visible only to admins. |
| `user_visible_description` | String | Description of the resource, visible to users and admins. |
| `resource_email` | String | The read-only email for the calendar resource. Generated as part of creating a new calendar resource. |
| `resource_category` | String | The category of the calendar resource. Either CONFERENCE_ROOM or OTHER. Legacy data is set to CATEGORY_UNKNOWN. |
| `floor_section` | String | Name of the section within a floor a resource is located in. |
| `kind` | String | The type of the resource. For calendar resources, the value is `admin#directory#resources#calendars#CalendarResource`. |
| `building_id` | String | Unique ID for the building a resource is located in. |
| `resource_id` | String | The unique ID for the calendar resource. |
| `resource_type` | String | The type of the calendar resource, intended for non-room resources. |
| `capacity` | i64 | Capacity of a resource, number of seats in a room. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create calendar
calendar = provider.admin_api.Calendar {
    customer = "value"  # The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's customer ID.
}

# Access calendar outputs
calendar_id = calendar.id
calendar_etags = calendar.etags
calendar_floor_name = calendar.floor_name
calendar_resource_name = calendar.resource_name
calendar_feature_instances = calendar.feature_instances
calendar_generated_resource_name = calendar.generated_resource_name
calendar_resource_description = calendar.resource_description
calendar_user_visible_description = calendar.user_visible_description
calendar_resource_email = calendar.resource_email
calendar_resource_category = calendar.resource_category
calendar_floor_section = calendar.floor_section
calendar_kind = calendar.kind
calendar_building_id = calendar.building_id
calendar_resource_id = calendar.resource_id
calendar_resource_type = calendar.resource_type
calendar_capacity = calendar.capacity
```

---


### Two_step_verification

Turns off 2-Step Verification for user.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_key` | String | ✅ | Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create two_step_verification
two_step_verification = provider.admin_api.Two_step_verification {
    user_key = "value"  # Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
}

```

---


### Verification_code

Generates new backup verification codes for the user.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_key` | String | ✅ | Email or immutable ID of the user |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of verification code resources. |
| `etag` | String | ETag of the resource. |
| `kind` | String | The type of the resource. This is always `admin#directory#verificationCodesList`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create verification_code
verification_code = provider.admin_api.Verification_code {
    user_key = "value"  # Email or immutable ID of the user
}

# Access verification_code outputs
verification_code_id = verification_code.id
verification_code_items = verification_code.items
verification_code_etag = verification_code.etag
verification_code_kind = verification_code.kind
```

---


### Customer

Retrieves a customer.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The unique ID for the customer's Google Workspace account. (Readonly) |
| `language` | String |  | The customer's ISO 639-2 language code. See the [Language Codes](https://developers.google.com/workspace/admin/directory/v1/languages) page for the list of supported codes. Valid language codes outside the supported set will be accepted by the API but may lead to unexpected behavior. The default value is `en`. |
| `alternate_email` | String |  | The customer's secondary contact email address. This email address cannot be on the same domain as the `customerDomain` |
| `etag` | String |  | ETag of the resource. |
| `customer_creation_time` | String |  | The customer's creation time (Readonly) |
| `customer_domain` | String |  | The customer's primary domain name string. Do not include the `www` prefix when creating a new customer. |
| `kind` | String |  | Identifies the resource as a customer. Value: `admin#directory#customer` |
| `postal_address` | String |  | The customer's postal address information. |
| `phone_number` | String |  | The customer's contact phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. |
| `customer_key` | String | ✅ | Id of the customer to be updated |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The unique ID for the customer's Google Workspace account. (Readonly) |
| `language` | String | The customer's ISO 639-2 language code. See the [Language Codes](https://developers.google.com/workspace/admin/directory/v1/languages) page for the list of supported codes. Valid language codes outside the supported set will be accepted by the API but may lead to unexpected behavior. The default value is `en`. |
| `alternate_email` | String | The customer's secondary contact email address. This email address cannot be on the same domain as the `customerDomain` |
| `etag` | String | ETag of the resource. |
| `customer_creation_time` | String | The customer's creation time (Readonly) |
| `customer_domain` | String | The customer's primary domain name string. Do not include the `www` prefix when creating a new customer. |
| `kind` | String | Identifies the resource as a customer. Value: `admin#directory#customer` |
| `postal_address` | String | The customer's postal address information. |
| `phone_number` | String | The customer's contact phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access customer outputs
customer_id = customer.id
customer_id = customer.id
customer_language = customer.language
customer_alternate_email = customer.alternate_email
customer_etag = customer.etag
customer_customer_creation_time = customer.customer_creation_time
customer_customer_domain = customer.customer_domain
customer_kind = customer.kind
customer_postal_address = customer.postal_address
customer_phone_number = customer.phone_number
```

---


### Printer

Creates a printer under given Organization Unit.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `org_unit_id` | String |  | Organization Unit that owns this printer (Only can be set during Printer creation) |
| `uri` | String |  | Editable. Printer URI. |
| `use_driverless_config` | bool |  | Editable. flag to use driverless configuration or not. If it's set to be true, make_and_model can be ignored |
| `name` | String |  | Identifier. The resource name of the Printer object, in the format customers/{customer-id}/printers/{printer-id} (During printer creation leave empty) |
| `create_time` | String |  | Output only. Time when printer was created. |
| `make_and_model` | String |  | Editable. Make and model of printer. e.g. Lexmark MS610de Value must be in format as seen in ListPrinterModels response. |
| `id` | String |  | Id of the printer. (During printer creation leave empty) |
| `description` | String |  | Editable. Description of printer. |
| `display_name` | String |  | Editable. Name of printer. |
| `auxiliary_messages` | Vec<String> |  | Output only. Auxiliary messages about issues with the printer configuration if any. |
| `parent` | String | ✅ | Required. The name of the customer. Format: customers/{customer_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `org_unit_id` | String | Organization Unit that owns this printer (Only can be set during Printer creation) |
| `uri` | String | Editable. Printer URI. |
| `use_driverless_config` | bool | Editable. flag to use driverless configuration or not. If it's set to be true, make_and_model can be ignored |
| `name` | String | Identifier. The resource name of the Printer object, in the format customers/{customer-id}/printers/{printer-id} (During printer creation leave empty) |
| `create_time` | String | Output only. Time when printer was created. |
| `make_and_model` | String | Editable. Make and model of printer. e.g. Lexmark MS610de Value must be in format as seen in ListPrinterModels response. |
| `id` | String | Id of the printer. (During printer creation leave empty) |
| `description` | String | Editable. Description of printer. |
| `display_name` | String | Editable. Name of printer. |
| `auxiliary_messages` | Vec<String> | Output only. Auxiliary messages about issues with the printer configuration if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create printer
printer = provider.admin_api.Printer {
    parent = "value"  # Required. The name of the customer. Format: customers/{customer_id}
}

# Access printer outputs
printer_id = printer.id
printer_org_unit_id = printer.org_unit_id
printer_uri = printer.uri
printer_use_driverless_config = printer.use_driverless_config
printer_name = printer.name
printer_create_time = printer.create_time
printer_make_and_model = printer.make_and_model
printer_id = printer.id
printer_description = printer.description
printer_display_name = printer.display_name
printer_auxiliary_messages = printer.auxiliary_messages
```

---


### User

Creates a user. Mutate calls immediately following user creation might sometimes fail as the user isn't fully created due to propagation delay in our backends. Check the error details for the "User creation is not complete" message to see if this is the case. Retrying the calls after some time can help in this case. If `resolveConflictAccount` is set to `true`, a `202` response code means that a conflicting unmanaged account exists and was invited to join the organization. A `409` response code means that a conflicting account exists so the user wasn't created based on the [handling unmanaged user accounts](https://support.google.com/a/answer/11112794) option selected.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer_id` | String |  | Output only. The customer ID to [retrieve all account users](https://developers.google.com/workspace/admin/directory/v1/guides/manage-users.html#get_all_users). You can use the alias `my_customer` to represent your account's `customerId`. As a reseller administrator, you can use the resold customer account's `customerId`. To get a `customerId`, use the account's primary domain in the `domain` parameter of a [users.list](https://developers.google.com/workspace/admin/directory/v1/reference/users/list) request. |
| `emails` | String |  | The list of the user's email addresses. The maximum allowed data size for this field is 10KB. This excludes `publicKeyEncryptionCertificates`. |
| `locations` | String |  | The user's locations. The maximum allowed data size for this field is 10KB. |
| `external_ids` | String |  | The list of external IDs for the user, such as an employee or network ID. The maximum allowed data size for this field is 2KB. |
| `recovery_email` | String |  | Recovery email of the user. |
| `suspended` | bool |  | Indicates if user is suspended. |
| `last_login_time` | String |  | User's last login time. (Read-only) |
| `websites` | String |  | The user's websites. The maximum allowed data size for this field is 2KB. |
| `ip_whitelisted` | bool |  | If `true`, the user's IP address is subject to a deprecated IP address [`allowlist`](https://support.google.com/a/answer/60752) configuration. |
| `agreed_to_terms` | bool |  | Output only. This property is `true` if the user has completed an initial login and accepted the Terms of Service agreement. |
| `keywords` | String |  | The list of the user's keywords. The maximum allowed data size for this field is 1KB. |
| `kind` | String |  | Output only. The type of the API resource. For Users resources, the value is `admin#directory#user`. |
| `is_enrolled_in2_sv` | bool |  | Output only. Is enrolled in 2-step verification (Read-only) |
| `is_enforced_in2_sv` | bool |  | Output only. Is 2-step verification enforced (Read-only) |
| `notes` | String |  | Notes for the user. |
| `archived` | bool |  | Indicates if user is archived. |
| `org_unit_path` | String |  | The full path of the parent organization associated with the user. If the parent organization is the top-level, it is represented as a forward slash (`/`). |
| `aliases` | Vec<String> |  | Output only. The list of the user's alias email addresses. |
| `languages` | String |  | The user's languages. The maximum allowed data size for this field is 1KB. |
| `ims` | String |  | The list of the user's Instant Messenger (IM) accounts. A user account can have multiple ims properties. But, only one of these ims properties can be the primary IM contact. The maximum allowed data size for this field is 2KB. |
| `deletion_time` | String |  |  |
| `organizations` | String |  | The list of organizations the user belongs to. The maximum allowed data size for this field is 10KB. |
| `password` | String |  | User's password |
| `posix_accounts` | String |  | The list of [POSIX](https://www.opengroup.org/austin/papers/posix_faq.html) account information for the user. |
| `recovery_phone` | String |  | Recovery phone of the user. The phone number must be in the E.164 format, starting with the plus sign (+). Example: *+16506661212*. |
| `is_mailbox_setup` | bool |  | Output only. Indicates if the user's Google mailbox is created. This property is only applicable if the user has been assigned a Gmail license. |
| `etag` | String |  | Output only. ETag of the resource. |
| `relations` | String |  | The list of the user's relationships to other users. The maximum allowed data size for this field is 2KB. |
| `phones` | String |  | The list of the user's phone numbers. The maximum allowed data size for this field is 1KB. |
| `thumbnail_photo_etag` | String |  | Output only. ETag of the user's photo (Read-only) |
| `include_in_global_address_list` | bool |  | Indicates if the user's profile is visible in the Google Workspace global address list when the contact sharing feature is enabled for the domain. For more information about excluding user profiles, see the [administration help center](https://support.google.com/a/answer/1285988). |
| `id` | String |  | The unique ID for the user. A user `id` can be used as a user request URI's `userKey`. |
| `is_admin` | bool |  | Output only. Indicates a user with super administrator privileges. The `isAdmin` property can only be edited in the [Make a user an administrator](https://developers.google.com/workspace/admin/directory/v1/guides/manage-users.html#make_admin) operation ( [makeAdmin](https://developers.google.com/workspace/admin/directory/v1/reference/users/makeAdmin.html) method). If edited in the user [insert](https://developers.google.com/workspace/admin/directory/v1/reference/users/insert.html) or [update](https://developers.google.com/workspace/admin/directory/v1/reference/users/update.html) methods, the edit is ignored by the API service. |
| `ssh_public_keys` | String |  | A list of SSH public keys. |
| `creation_time` | String |  | User's G Suite account creation time. (Read-only) |
| `custom_schemas` | HashMap<String, HashMap<String, String>> |  | Custom fields of the user. The key is a `schema_name` and its values are `'field_name': 'field_value'`. |
| `name` | String |  | Holds the given and family names of the user, and the read-only `fullName` value. The maximum number of characters in the `givenName` and in the `familyName` values is 60. In addition, name values support unicode/UTF-8 characters, and can contain spaces, letters (a-z), numbers (0-9), dashes (-), forward slashes (/), and periods (.). For more information about character usage rules, see the [administration help center](https://support.google.com/a/answer/9193374). Maximum allowed data size for this field is 1KB. |
| `addresses` | String |  | The list of the user's addresses. The maximum allowed data size for this field is 10KB. |
| `primary_email` | String |  | The user's primary email address. This property is required in a request to create a user account. The `primaryEmail` must be unique and cannot be an alias of another user. |
| `change_password_at_next_login` | bool |  | Indicates if the user is forced to change their password at next login. This setting doesn't apply when [the user signs in via a third-party identity provider](https://support.google.com/a/answer/60224). |
| `thumbnail_photo_url` | String |  | Output only. The URL of the user's profile photo. The URL might be temporary or private. |
| `hash_function` | String |  | Stores the hash format of the `password` property. The following `hashFunction` values are allowed: * `MD5` - Accepts simple hex-encoded values. * `SHA-1` - Accepts simple hex-encoded values. * `crypt` - Compliant with the [C crypt library](https://en.wikipedia.org/wiki/Crypt_%28C%29). Supports the DES, MD5 (hash prefix `$1$`), SHA-256 (hash prefix `$5$`), and SHA-512 (hash prefix `$6$`) hash algorithms. If rounds are specified as part of the prefix, they must be 10,000 or fewer. |
| `non_editable_aliases` | Vec<String> |  | Output only. The list of the user's non-editable alias email addresses. These are typically outside the account's primary domain or sub-domain. |
| `is_delegated_admin` | bool |  | Output only. Indicates if the user is a delegated administrator. Delegated administrators are supported by the API but cannot create or undelete users, or make users administrators. These requests are ignored by the API service. Roles and privileges for administrators are assigned using the [Admin console](https://support.google.com/a/answer/33325). |
| `suspension_reason` | String |  | Output only. Has the reason a user account is suspended either by the administrator or by Google at the time of suspension. The property is returned only if the `suspended` property is `true`. |
| `gender` | String |  | The user's gender. The maximum allowed data size for this field is 1KB. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer_id` | String | Output only. The customer ID to [retrieve all account users](https://developers.google.com/workspace/admin/directory/v1/guides/manage-users.html#get_all_users). You can use the alias `my_customer` to represent your account's `customerId`. As a reseller administrator, you can use the resold customer account's `customerId`. To get a `customerId`, use the account's primary domain in the `domain` parameter of a [users.list](https://developers.google.com/workspace/admin/directory/v1/reference/users/list) request. |
| `emails` | String | The list of the user's email addresses. The maximum allowed data size for this field is 10KB. This excludes `publicKeyEncryptionCertificates`. |
| `locations` | String | The user's locations. The maximum allowed data size for this field is 10KB. |
| `external_ids` | String | The list of external IDs for the user, such as an employee or network ID. The maximum allowed data size for this field is 2KB. |
| `recovery_email` | String | Recovery email of the user. |
| `suspended` | bool | Indicates if user is suspended. |
| `last_login_time` | String | User's last login time. (Read-only) |
| `websites` | String | The user's websites. The maximum allowed data size for this field is 2KB. |
| `ip_whitelisted` | bool | If `true`, the user's IP address is subject to a deprecated IP address [`allowlist`](https://support.google.com/a/answer/60752) configuration. |
| `agreed_to_terms` | bool | Output only. This property is `true` if the user has completed an initial login and accepted the Terms of Service agreement. |
| `keywords` | String | The list of the user's keywords. The maximum allowed data size for this field is 1KB. |
| `kind` | String | Output only. The type of the API resource. For Users resources, the value is `admin#directory#user`. |
| `is_enrolled_in2_sv` | bool | Output only. Is enrolled in 2-step verification (Read-only) |
| `is_enforced_in2_sv` | bool | Output only. Is 2-step verification enforced (Read-only) |
| `notes` | String | Notes for the user. |
| `archived` | bool | Indicates if user is archived. |
| `org_unit_path` | String | The full path of the parent organization associated with the user. If the parent organization is the top-level, it is represented as a forward slash (`/`). |
| `aliases` | Vec<String> | Output only. The list of the user's alias email addresses. |
| `languages` | String | The user's languages. The maximum allowed data size for this field is 1KB. |
| `ims` | String | The list of the user's Instant Messenger (IM) accounts. A user account can have multiple ims properties. But, only one of these ims properties can be the primary IM contact. The maximum allowed data size for this field is 2KB. |
| `deletion_time` | String |  |
| `organizations` | String | The list of organizations the user belongs to. The maximum allowed data size for this field is 10KB. |
| `password` | String | User's password |
| `posix_accounts` | String | The list of [POSIX](https://www.opengroup.org/austin/papers/posix_faq.html) account information for the user. |
| `recovery_phone` | String | Recovery phone of the user. The phone number must be in the E.164 format, starting with the plus sign (+). Example: *+16506661212*. |
| `is_mailbox_setup` | bool | Output only. Indicates if the user's Google mailbox is created. This property is only applicable if the user has been assigned a Gmail license. |
| `etag` | String | Output only. ETag of the resource. |
| `relations` | String | The list of the user's relationships to other users. The maximum allowed data size for this field is 2KB. |
| `phones` | String | The list of the user's phone numbers. The maximum allowed data size for this field is 1KB. |
| `thumbnail_photo_etag` | String | Output only. ETag of the user's photo (Read-only) |
| `include_in_global_address_list` | bool | Indicates if the user's profile is visible in the Google Workspace global address list when the contact sharing feature is enabled for the domain. For more information about excluding user profiles, see the [administration help center](https://support.google.com/a/answer/1285988). |
| `id` | String | The unique ID for the user. A user `id` can be used as a user request URI's `userKey`. |
| `is_admin` | bool | Output only. Indicates a user with super administrator privileges. The `isAdmin` property can only be edited in the [Make a user an administrator](https://developers.google.com/workspace/admin/directory/v1/guides/manage-users.html#make_admin) operation ( [makeAdmin](https://developers.google.com/workspace/admin/directory/v1/reference/users/makeAdmin.html) method). If edited in the user [insert](https://developers.google.com/workspace/admin/directory/v1/reference/users/insert.html) or [update](https://developers.google.com/workspace/admin/directory/v1/reference/users/update.html) methods, the edit is ignored by the API service. |
| `ssh_public_keys` | String | A list of SSH public keys. |
| `creation_time` | String | User's G Suite account creation time. (Read-only) |
| `custom_schemas` | HashMap<String, HashMap<String, String>> | Custom fields of the user. The key is a `schema_name` and its values are `'field_name': 'field_value'`. |
| `name` | String | Holds the given and family names of the user, and the read-only `fullName` value. The maximum number of characters in the `givenName` and in the `familyName` values is 60. In addition, name values support unicode/UTF-8 characters, and can contain spaces, letters (a-z), numbers (0-9), dashes (-), forward slashes (/), and periods (.). For more information about character usage rules, see the [administration help center](https://support.google.com/a/answer/9193374). Maximum allowed data size for this field is 1KB. |
| `addresses` | String | The list of the user's addresses. The maximum allowed data size for this field is 10KB. |
| `primary_email` | String | The user's primary email address. This property is required in a request to create a user account. The `primaryEmail` must be unique and cannot be an alias of another user. |
| `change_password_at_next_login` | bool | Indicates if the user is forced to change their password at next login. This setting doesn't apply when [the user signs in via a third-party identity provider](https://support.google.com/a/answer/60224). |
| `thumbnail_photo_url` | String | Output only. The URL of the user's profile photo. The URL might be temporary or private. |
| `hash_function` | String | Stores the hash format of the `password` property. The following `hashFunction` values are allowed: * `MD5` - Accepts simple hex-encoded values. * `SHA-1` - Accepts simple hex-encoded values. * `crypt` - Compliant with the [C crypt library](https://en.wikipedia.org/wiki/Crypt_%28C%29). Supports the DES, MD5 (hash prefix `$1$`), SHA-256 (hash prefix `$5$`), and SHA-512 (hash prefix `$6$`) hash algorithms. If rounds are specified as part of the prefix, they must be 10,000 or fewer. |
| `non_editable_aliases` | Vec<String> | Output only. The list of the user's non-editable alias email addresses. These are typically outside the account's primary domain or sub-domain. |
| `is_delegated_admin` | bool | Output only. Indicates if the user is a delegated administrator. Delegated administrators are supported by the API but cannot create or undelete users, or make users administrators. These requests are ignored by the API service. Roles and privileges for administrators are assigned using the [Admin console](https://support.google.com/a/answer/33325). |
| `suspension_reason` | String | Output only. Has the reason a user account is suspended either by the administrator or by Google at the time of suspension. The property is returned only if the `suspended` property is `true`. |
| `gender` | String | The user's gender. The maximum allowed data size for this field is 1KB. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.admin_api.User {
}

# Access user outputs
user_id = user.id
user_customer_id = user.customer_id
user_emails = user.emails
user_locations = user.locations
user_external_ids = user.external_ids
user_recovery_email = user.recovery_email
user_suspended = user.suspended
user_last_login_time = user.last_login_time
user_websites = user.websites
user_ip_whitelisted = user.ip_whitelisted
user_agreed_to_terms = user.agreed_to_terms
user_keywords = user.keywords
user_kind = user.kind
user_is_enrolled_in2_sv = user.is_enrolled_in2_sv
user_is_enforced_in2_sv = user.is_enforced_in2_sv
user_notes = user.notes
user_archived = user.archived
user_org_unit_path = user.org_unit_path
user_aliases = user.aliases
user_languages = user.languages
user_ims = user.ims
user_deletion_time = user.deletion_time
user_organizations = user.organizations
user_password = user.password
user_posix_accounts = user.posix_accounts
user_recovery_phone = user.recovery_phone
user_is_mailbox_setup = user.is_mailbox_setup
user_etag = user.etag
user_relations = user.relations
user_phones = user.phones
user_thumbnail_photo_etag = user.thumbnail_photo_etag
user_include_in_global_address_list = user.include_in_global_address_list
user_id = user.id
user_is_admin = user.is_admin
user_ssh_public_keys = user.ssh_public_keys
user_creation_time = user.creation_time
user_custom_schemas = user.custom_schemas
user_name = user.name
user_addresses = user.addresses
user_primary_email = user.primary_email
user_change_password_at_next_login = user.change_password_at_next_login
user_thumbnail_photo_url = user.thumbnail_photo_url
user_hash_function = user.hash_function
user_non_editable_aliases = user.non_editable_aliases
user_is_delegated_admin = user.is_delegated_admin
user_suspension_reason = user.suspension_reason
user_gender = user.gender
```

---


### Member

Adds a user to the specified group.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | ETag of the resource. |
| `delivery_settings` | String |  | Defines mail delivery preferences of member. This field is only supported by `insert`, `update`, and `get` methods. |
| `role` | String |  | The member's role in a group. The API returns an error for cycles in group memberships. For example, if `group1` is a member of `group2`, `group2` cannot be a member of `group1`. For more information about a member's role, see the [administration help center](https://support.google.com/a/answer/167094). |
| `status` | String |  | Status of member (Immutable) |
| `type` | String |  | The type of group member. |
| `kind` | String |  | The type of the API resource. For Members resources, the value is `admin#directory#member`. |
| `id` | String |  | The unique ID of the group member. A member `id` can be used as a member request URI's `memberKey`. |
| `email` | String |  | The member's email address. A member can be a user or another group. This property is required when adding a member to a group. The `email` must be unique and cannot be an alias of another group. If the email address is changed, the API automatically reflects the email address changes. |
| `group_key` | String | ✅ | Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | ETag of the resource. |
| `delivery_settings` | String | Defines mail delivery preferences of member. This field is only supported by `insert`, `update`, and `get` methods. |
| `role` | String | The member's role in a group. The API returns an error for cycles in group memberships. For example, if `group1` is a member of `group2`, `group2` cannot be a member of `group1`. For more information about a member's role, see the [administration help center](https://support.google.com/a/answer/167094). |
| `status` | String | Status of member (Immutable) |
| `type` | String | The type of group member. |
| `kind` | String | The type of the API resource. For Members resources, the value is `admin#directory#member`. |
| `id` | String | The unique ID of the group member. A member `id` can be used as a member request URI's `memberKey`. |
| `email` | String | The member's email address. A member can be a user or another group. This property is required when adding a member to a group. The `email` must be unique and cannot be an alias of another group. If the email address is changed, the API automatically reflects the email address changes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create member
member = provider.admin_api.Member {
    group_key = "value"  # Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
}

# Access member outputs
member_id = member.id
member_etag = member.etag
member_delivery_settings = member.delivery_settings
member_role = member.role
member_status = member.status
member_type = member.type
member_kind = member.kind
member_id = member.id
member_email = member.email
```

---


### Chromeo

Changes the status of a batch of ChromeOS devices. For more information about changing a ChromeOS device state [Repair, repurpose, or retire ChromeOS devices](https://support.google.com/chrome/a/answer/3523633).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deprovision_reason` | String |  | Optional. The reason behind a device deprovision. Must be provided if 'changeChromeOsDeviceStatusAction' is set to 'CHANGE_CHROME_OS_DEVICE_STATUS_ACTION_DEPROVISION'. Otherwise, omit this field. |
| `device_ids` | Vec<String> |  | Required. List of the IDs of the ChromeOS devices to change. Maximum 50. |
| `change_chrome_os_device_status_action` | String |  | Required. The action to take on the ChromeOS device in order to change its status. |
| `customer_id` | String | ✅ | Required. Immutable ID of the Google Workspace account. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create chromeo
chromeo = provider.admin_api.Chromeo {
    customer_id = "value"  # Required. Immutable ID of the Google Workspace account.
}

```

---


### Application

Retrieves information about an application for the given application ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The application's name. |
| `transfer_params` | Vec<String> | The list of all possible transfer parameters for this application. These parameters select which categories of the user's data to transfer. |
| `id` | String | The application's ID. Retrievable by using the [`applications.list()`](https://developers.google.com/workspace/admin/data-transfer/reference/rest/v1/applications/list) method. |
| `etag` | String | Etag of the resource. |
| `kind` | String | Identifies the resource as a DataTransfer Application Resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access application outputs
application_id = application.id
application_name = application.name
application_transfer_params = application.transfer_params
application_id = application.id
application_etag = application.etag
application_kind = application.kind
```

---


### Transfer

Inserts a data transfer request. See the [Transfer parameters](https://developers.google.com/workspace/admin/data-transfer/v1/parameters) reference for specific application requirements.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `old_owner_user_id` | String |  | ID of the user whose data is being transferred. |
| `request_time` | String |  | Read-only. The time at which the data transfer was requested. |
| `application_data_transfers` | Vec<String> |  | The list of per-application data transfer resources. It contains details of the applications associated with this transfer resource, and also specifies the applications for which data transfer has to be done at the time of the transfer resource creation. |
| `kind` | String |  | Identifies the resource as a DataTransfer request. |
| `etag` | String |  | ETag of the resource. |
| `id` | String |  | Read-only. The transfer's ID. |
| `new_owner_user_id` | String |  | ID of the user to whom the data is being transferred. |
| `overall_transfer_status_code` | String |  | Read-only. Overall transfer status. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `old_owner_user_id` | String | ID of the user whose data is being transferred. |
| `request_time` | String | Read-only. The time at which the data transfer was requested. |
| `application_data_transfers` | Vec<String> | The list of per-application data transfer resources. It contains details of the applications associated with this transfer resource, and also specifies the applications for which data transfer has to be done at the time of the transfer resource creation. |
| `kind` | String | Identifies the resource as a DataTransfer request. |
| `etag` | String | ETag of the resource. |
| `id` | String | Read-only. The transfer's ID. |
| `new_owner_user_id` | String | ID of the user to whom the data is being transferred. |
| `overall_transfer_status_code` | String | Read-only. Overall transfer status. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create transfer
transfer = provider.admin_api.Transfer {
}

# Access transfer outputs
transfer_id = transfer.id
transfer_old_owner_user_id = transfer.old_owner_user_id
transfer_request_time = transfer.request_time
transfer_application_data_transfers = transfer.application_data_transfers
transfer_kind = transfer.kind
transfer_etag = transfer.etag
transfer_id = transfer.id
transfer_new_owner_user_id = transfer.new_owner_user_id
transfer_overall_transfer_status_code = transfer.overall_transfer_status_code
```

---


### Activitie

Start receiving notifications for account activities. For more information, see Receiving Push Notifications.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. A payload is data that is sent in the body of an HTTP POST, PUT, or PATCH message and contains important information about the request. Optional. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is "`api#channel`". |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | String |  | The type of delivery mechanism used for this channel. The value should be set to `"web_hook"`. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `application_name` | String | ✅ | Application name for which the events are to be retrieved. |
| `user_key` | String | ✅ | Represents the profile ID or the user email for which the data should be filtered. Can be `all` for all information, or `userKey` for a user's unique Google Workspace profile ID or their primary email address. Must not be a deleted user. For a deleted user, call `users.list` in Directory API with `showDeleted=true`, then use the returned `ID` as the `userKey`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | Each activity record in the response. |
| `kind` | String | The type of API resource. For an activity report, the value is `reports#activities`. |
| `etag` | String | ETag of the resource. |
| `next_page_token` | String | Token for retrieving the follow-on next page of the report. The `nextPageToken` value is used in the request's `pageToken` query string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create activitie
activitie = provider.admin_api.Activitie {
    application_name = "value"  # Application name for which the events are to be retrieved.
    user_key = "value"  # Represents the profile ID or the user email for which the data should be filtered. Can be `all` for all information, or `userKey` for a user's unique Google Workspace profile ID or their primary email address. Must not be a deleted user. For a deleted user, call `users.list` in Directory API with `showDeleted=true`, then use the returned `ID` as the `userKey`.
}

# Access activitie outputs
activitie_id = activitie.id
activitie_items = activitie.items
activitie_kind = activitie.kind
activitie_etag = activitie.etag
activitie_next_page_token = activitie.next_page_token
```

---


### User_usage_report

Retrieves a report which is a collection of properties and statistics for a set of users with the account. For more information, see the User Usage Report guide. For more information about the user report's parameters, see the Users Usage parameters reference guides.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `warnings` | Vec<String> | Warnings, if any. |
| `kind` | String | The type of API resource. For a usage report, the value is `admin#reports#usageReports`. |
| `etag` | String | ETag of the resource. |
| `usage_reports` | Vec<String> | Various application parameter records. |
| `next_page_token` | String | Token to specify next page. A report with multiple pages has a `nextPageToken` property in the response. For your follow-on requests getting all of the report's pages, enter the `nextPageToken` value in the `pageToken` query string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user_usage_report outputs
user_usage_report_id = user_usage_report.id
user_usage_report_warnings = user_usage_report.warnings
user_usage_report_kind = user_usage_report.kind
user_usage_report_etag = user_usage_report.etag
user_usage_report_usage_reports = user_usage_report.usage_reports
user_usage_report_next_page_token = user_usage_report.next_page_token
```

---


### Entity_usage_report

Retrieves a report which is a collection of properties and statistics for entities used by users within the account. For more information, see the Entities Usage Report guide. For more information about the entities report's parameters, see the Entities Usage parameters reference guides.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `warnings` | Vec<String> | Warnings, if any. |
| `kind` | String | The type of API resource. For a usage report, the value is `admin#reports#usageReports`. |
| `etag` | String | ETag of the resource. |
| `usage_reports` | Vec<String> | Various application parameter records. |
| `next_page_token` | String | Token to specify next page. A report with multiple pages has a `nextPageToken` property in the response. For your follow-on requests getting all of the report's pages, enter the `nextPageToken` value in the `pageToken` query string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access entity_usage_report outputs
entity_usage_report_id = entity_usage_report.id
entity_usage_report_warnings = entity_usage_report.warnings
entity_usage_report_kind = entity_usage_report.kind
entity_usage_report_etag = entity_usage_report.etag
entity_usage_report_usage_reports = entity_usage_report.usage_reports
entity_usage_report_next_page_token = entity_usage_report.next_page_token
```

---


### Channel

Stop watching resources through this channel.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. A payload is data that is sent in the body of an HTTP POST, PUT, or PATCH message and contains important information about the request. Optional. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is "`api#channel`". |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | String |  | The type of delivery mechanism used for this channel. The value should be set to `"web_hook"`. |
| `address` | String |  | The address where notifications are delivered for this channel. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel
channel = provider.admin_api.Channel {
}

```

---


### Customer_usage_report

Retrieves a report which is a collection of properties and statistics for a specific customer's account. For more information, see the Customers Usage Report guide. For more information about the customer report's parameters, see the Customers Usage parameters reference guides. 

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `warnings` | Vec<String> | Warnings, if any. |
| `kind` | String | The type of API resource. For a usage report, the value is `admin#reports#usageReports`. |
| `etag` | String | ETag of the resource. |
| `usage_reports` | Vec<String> | Various application parameter records. |
| `next_page_token` | String | Token to specify next page. A report with multiple pages has a `nextPageToken` property in the response. For your follow-on requests getting all of the report's pages, enter the `nextPageToken` value in the `pageToken` query string. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access customer_usage_report outputs
customer_usage_report_id = customer_usage_report.id
customer_usage_report_warnings = customer_usage_report.warnings
customer_usage_report_kind = customer_usage_report.kind
customer_usage_report_etag = customer_usage_report.etag
customer_usage_report_usage_reports = customer_usage_report.usage_reports
customer_usage_report_next_page_token = customer_usage_report.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple privilege resources
privilege_0 = provider.admin_api.Privilege {
}
privilege_1 = provider.admin_api.Privilege {
}
privilege_2 = provider.admin_api.Privilege {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    privilege = provider.admin_api.Privilege {
    }
```

---

## Related Documentation

- [GCP Admin_api Documentation](https://cloud.google.com/admin_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
