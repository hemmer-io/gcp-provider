# Area120tables_api Service



**Resources**: 3

---

## Overview

The area120tables_api service provides access to 3 resource types:

- [Workspace](#workspace) [R]
- [Table](#table) [R]
- [Row](#row) [CRUD]

---

## Resources


### Workspace

Gets a workspace. Returns NOT_FOUND if the workspace does not exist.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Time when the workspace was last updated. |
| `tables` | Vec<String> | The list of tables in the workspace. |
| `display_name` | String | The human readable title of the workspace. |
| `create_time` | String | Time when the workspace was created. |
| `name` | String | The resource name of the workspace. Workspace names have the form `workspaces/{workspace}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access workspace outputs
workspace_id = workspace.id
workspace_update_time = workspace.update_time
workspace_tables = workspace.tables
workspace_display_name = workspace.display_name
workspace_create_time = workspace.create_time
workspace_name = workspace.name
```

---


### Table

Gets a table. Returns NOT_FOUND if the table does not exist.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `time_zone` | String | The time zone of the table. IANA Time Zone Database time zone, e.g. "America/New_York". |
| `update_time` | String | Time when the table was last updated excluding updates to individual rows |
| `columns` | Vec<String> | List of columns in this table. Order of columns matches the display order. |
| `name` | String | The resource name of the table. Table names have the form `tables/{table}`. |
| `saved_views` | Vec<String> | Saved views for this table. |
| `display_name` | String | The human readable title of the table. |
| `create_time` | String | Time when the table was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access table outputs
table_id = table.id
table_time_zone = table.time_zone
table_update_time = table.update_time
table_columns = table.columns
table_name = table.name
table_saved_views = table.saved_views
table_display_name = table.display_name
table_create_time = table.create_time
```

---


### Row

Creates a row.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `values` | HashMap<String, String> |  | The values of the row. This is a map of column key to value. Key is user entered name(default) or the internal column id based on the view in the request. |
| `update_time` | String |  | Time when the row was last updated. |
| `name` | String |  | The resource name of the row. Row names have the form `tables/{table}/rows/{row}`. The name is ignored when creating a row. |
| `create_time` | String |  | Time when the row was created. |
| `parent` | String | ✅ | Required. The parent table where this row will be created. Format: tables/{table} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `values` | HashMap<String, String> | The values of the row. This is a map of column key to value. Key is user entered name(default) or the internal column id based on the view in the request. |
| `update_time` | String | Time when the row was last updated. |
| `name` | String | The resource name of the row. Row names have the form `tables/{table}/rows/{row}`. The name is ignored when creating a row. |
| `create_time` | String | Time when the row was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create row
row = provider.area120tables_api.Row {
    parent = "value"  # Required. The parent table where this row will be created. Format: tables/{table}
}

# Access row outputs
row_id = row.id
row_values = row.values
row_update_time = row.update_time
row_name = row.name
row_create_time = row.create_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple workspace resources
workspace_0 = provider.area120tables_api.Workspace {
}
workspace_1 = provider.area120tables_api.Workspace {
}
workspace_2 = provider.area120tables_api.Workspace {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    workspace = provider.area120tables_api.Workspace {
    }
```

---

## Related Documentation

- [GCP Area120tables_api Documentation](https://cloud.google.com/area120tables_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
