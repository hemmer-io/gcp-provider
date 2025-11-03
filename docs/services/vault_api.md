# Vault_api Service



**Resources**: 6

---

## Overview

The vault_api service provides access to 6 resource types:

- [Account](#account) [CRD]
- [Export](#export) [CRD]
- [Operation](#operation) [CRD]
- [Matter](#matter) [CRUD]
- [Saved_querie](#saved_querie) [CRD]
- [Hold](#hold) [CRUD]

---

## Resources


### Account

Adds an account to a hold. Accounts can be added only to a hold that does not have an organizational unit set. If you try to add an account to an organizational unit-based hold, an error is returned.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email` | String |  | The primary email address of the account. If used as an input, this takes precedence over **accountId**. |
| `last_name` | String |  | Output only. The last name of the account holder. |
| `account_id` | String |  | The account ID, as provided by the [Admin SDK](https://developers.google.com/admin-sdk/). |
| `first_name` | String |  | Output only. The first name of the account holder. |
| `hold_time` | String |  | Output only. When the account was put on hold. |
| `hold_id` | String | ✅ | The hold ID. |
| `matter_id` | String | ✅ | The matter ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `accounts` | Vec<String> | The held accounts on a hold. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.vault_api.Account {
    hold_id = "value"  # The hold ID.
    matter_id = "value"  # The matter ID.
}

# Access account outputs
account_id = account.id
account_accounts = account.accounts
```

---


### Export

Creates an export.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent_export_id` | String |  | Output only. Identifies the parent export that spawned this child export. This is only set on child exports. |
| `name` | String |  | The export name. Don't use special characters (~!$'(),;@:/?) in the name, they can prevent you from downloading exports. |
| `export_options` | String |  | Additional export options. |
| `requester` | String |  | Output only. The requester of the export. |
| `cloud_storage_sink` | String |  | Output only. The sink for export files in Cloud Storage. |
| `stats` | String |  | Output only. Details about the export progress and size. |
| `create_time` | String |  | Output only. The time when the export was created. |
| `id` | String |  | Output only. The generated export ID. |
| `query` | String |  | The query parameters used to create the export. |
| `matter_id` | String |  | Output only. The matter ID. |
| `status` | String |  | Output only. The status of the export. |
| `matter_id` | String | ✅ | The matter ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parent_export_id` | String | Output only. Identifies the parent export that spawned this child export. This is only set on child exports. |
| `name` | String | The export name. Don't use special characters (~!$'(),;@:/?) in the name, they can prevent you from downloading exports. |
| `export_options` | String | Additional export options. |
| `requester` | String | Output only. The requester of the export. |
| `cloud_storage_sink` | String | Output only. The sink for export files in Cloud Storage. |
| `stats` | String | Output only. Details about the export progress and size. |
| `create_time` | String | Output only. The time when the export was created. |
| `id` | String | Output only. The generated export ID. |
| `query` | String | The query parameters used to create the export. |
| `matter_id` | String | Output only. The matter ID. |
| `status` | String | Output only. The status of the export. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create export
export = provider.vault_api.Export {
    matter_id = "value"  # The matter ID.
}

# Access export outputs
export_id = export.id
export_parent_export_id = export.parent_export_id
export_name = export.name
export_export_options = export.export_options
export_requester = export.requester
export_cloud_storage_sink = export.cloud_storage_sink
export_stats = export.stats
export_create_time = export.create_time
export_id = export.id
export_query = export.query
export_matter_id = export.matter_id
export_status = export.status
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.vault_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
```

---


### Matter

Creates a matter with the given name and description. The initial state is open, and the owner is the method caller. Returns the created matter with default view.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | The state of the matter. |
| `matter_permissions` | Vec<String> |  | Lists the users and their permission for the matter. Currently there is no programmer defined limit on the number of permissions a matter can have. |
| `matter_id` | String |  | The matter ID, which is generated by the server. Leave blank when creating a matter. |
| `description` | String |  | An optional description for the matter. |
| `matter_region` | String |  | Optional. The requested data region for the matter. |
| `name` | String |  | The name of the matter. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | The state of the matter. |
| `matter_permissions` | Vec<String> | Lists the users and their permission for the matter. Currently there is no programmer defined limit on the number of permissions a matter can have. |
| `matter_id` | String | The matter ID, which is generated by the server. Leave blank when creating a matter. |
| `description` | String | An optional description for the matter. |
| `matter_region` | String | Optional. The requested data region for the matter. |
| `name` | String | The name of the matter. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create matter
matter = provider.vault_api.Matter {
}

# Access matter outputs
matter_id = matter.id
matter_state = matter.state
matter_matter_permissions = matter.matter_permissions
matter_matter_id = matter.matter_id
matter_description = matter.description
matter_matter_region = matter.matter_region
matter_name = matter.name
```

---


### Saved_querie

Creates a saved query.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `matter_id` | String |  | Output only. The matter ID of the matter the saved query is saved in. The server does not use this field during create and always uses matter ID in the URL. |
| `create_time` | String |  | Output only. The server-generated timestamp when the saved query was created. |
| `query` | String |  | The search parameters of the saved query. |
| `saved_query_id` | String |  | A unique identifier for the saved query. |
| `display_name` | String |  | The name of the saved query. |
| `matter_id` | String | ✅ | The ID of the matter to create the saved query in. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `matter_id` | String | Output only. The matter ID of the matter the saved query is saved in. The server does not use this field during create and always uses matter ID in the URL. |
| `create_time` | String | Output only. The server-generated timestamp when the saved query was created. |
| `query` | String | The search parameters of the saved query. |
| `saved_query_id` | String | A unique identifier for the saved query. |
| `display_name` | String | The name of the saved query. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create saved_querie
saved_querie = provider.vault_api.Saved_querie {
    matter_id = "value"  # The ID of the matter to create the saved query in.
}

# Access saved_querie outputs
saved_querie_id = saved_querie.id
saved_querie_matter_id = saved_querie.matter_id
saved_querie_create_time = saved_querie.create_time
saved_querie_query = saved_querie.query
saved_querie_saved_query_id = saved_querie.saved_query_id
saved_querie_display_name = saved_querie.display_name
```

---


### Hold

Creates a hold in the specified matter.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `accounts` | Vec<String> |  | If set, the hold applies to the specified accounts and **orgUnit** must be empty. |
| `hold_id` | String |  | The unique immutable ID of the hold. Assigned during creation. |
| `org_unit` | String |  | If set, the hold applies to all members of the organizational unit and **accounts** must be empty. This property is mutable. For Groups holds, set **accounts**. |
| `query` | String |  | Service-specific options. If set, **CorpusQuery** must match **CorpusType**. |
| `update_time` | String |  | The last time this hold was modified. |
| `name` | String |  | The name of the hold. |
| `corpus` | String |  | The service to be searched. |
| `matter_id` | String | ✅ | The matter ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `accounts` | Vec<String> | If set, the hold applies to the specified accounts and **orgUnit** must be empty. |
| `hold_id` | String | The unique immutable ID of the hold. Assigned during creation. |
| `org_unit` | String | If set, the hold applies to all members of the organizational unit and **accounts** must be empty. This property is mutable. For Groups holds, set **accounts**. |
| `query` | String | Service-specific options. If set, **CorpusQuery** must match **CorpusType**. |
| `update_time` | String | The last time this hold was modified. |
| `name` | String | The name of the hold. |
| `corpus` | String | The service to be searched. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create hold
hold = provider.vault_api.Hold {
    matter_id = "value"  # The matter ID.
}

# Access hold outputs
hold_id = hold.id
hold_accounts = hold.accounts
hold_hold_id = hold.hold_id
hold_org_unit = hold.org_unit
hold_query = hold.query
hold_update_time = hold.update_time
hold_name = hold.name
hold_corpus = hold.corpus
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple account resources
account_0 = provider.vault_api.Account {
    hold_id = "value-0"
    matter_id = "value-0"
}
account_1 = provider.vault_api.Account {
    hold_id = "value-1"
    matter_id = "value-1"
}
account_2 = provider.vault_api.Account {
    hold_id = "value-2"
    matter_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account = provider.vault_api.Account {
        hold_id = "production-value"
        matter_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Vault_api Documentation](https://cloud.google.com/vault_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
