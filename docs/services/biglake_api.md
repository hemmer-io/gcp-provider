# Biglake_api Service



**Resources**: 3

---

## Overview

The biglake_api service provides access to 3 resource types:

- [Database](#database) [CRUD]
- [Catalog](#catalog) [CRD]
- [Table](#table) [CRUD]

---

## Resources


### Database

Creates a new database.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The last modification time of the database. |
| `create_time` | String |  | Output only. The creation time of the database. |
| `type` | String |  | The database type. |
| `expire_time` | String |  | Output only. The time when this database is considered expired. Only set after the database is deleted. |
| `hive_options` | String |  | Options of a Hive database. |
| `name` | String |  | Output only. The resource name. Format: projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id} |
| `delete_time` | String |  | Output only. The deletion time of the database. Only set after the database is deleted. |
| `parent` | String | ✅ | Required. The parent resource where this database will be created. Format: projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The last modification time of the database. |
| `create_time` | String | Output only. The creation time of the database. |
| `type` | String | The database type. |
| `expire_time` | String | Output only. The time when this database is considered expired. Only set after the database is deleted. |
| `hive_options` | String | Options of a Hive database. |
| `name` | String | Output only. The resource name. Format: projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id} |
| `delete_time` | String | Output only. The deletion time of the database. Only set after the database is deleted. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create database
database = provider.biglake_api.Database {
    parent = "value"  # Required. The parent resource where this database will be created. Format: projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}
}

# Access database outputs
database_id = database.id
database_update_time = database.update_time
database_create_time = database.create_time
database_type = database.type
database_expire_time = database.expire_time
database_hive_options = database.hive_options
database_name = database.name
database_delete_time = database.delete_time
```

---


### Catalog

Creates a new catalog.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The creation time of the catalog. |
| `delete_time` | String |  | Output only. The deletion time of the catalog. Only set after the catalog is deleted. |
| `expire_time` | String |  | Output only. The time when this catalog is considered expired. Only set after the catalog is deleted. |
| `name` | String |  | Output only. The resource name. Format: projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id} |
| `update_time` | String |  | Output only. The last modification time of the catalog. |
| `parent` | String | ✅ | Required. The parent resource where this catalog will be created. Format: projects/{project_id_or_number}/locations/{location_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The creation time of the catalog. |
| `delete_time` | String | Output only. The deletion time of the catalog. Only set after the catalog is deleted. |
| `expire_time` | String | Output only. The time when this catalog is considered expired. Only set after the catalog is deleted. |
| `name` | String | Output only. The resource name. Format: projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id} |
| `update_time` | String | Output only. The last modification time of the catalog. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create catalog
catalog = provider.biglake_api.Catalog {
    parent = "value"  # Required. The parent resource where this catalog will be created. Format: projects/{project_id_or_number}/locations/{location_id}
}

# Access catalog outputs
catalog_id = catalog.id
catalog_create_time = catalog.create_time
catalog_delete_time = catalog.delete_time
catalog_expire_time = catalog.expire_time
catalog_name = catalog.name
catalog_update_time = catalog.update_time
```

---


### Table

Creates a new table.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expire_time` | String |  | Output only. The time when this table is considered expired. Only set after the table is deleted. |
| `name` | String |  | Output only. The resource name. Format: projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}/tables/{table_id} |
| `update_time` | String |  | Output only. The last modification time of the table. |
| `hive_options` | String |  | Options of a Hive table. |
| `create_time` | String |  | Output only. The creation time of the table. |
| `etag` | String |  | The checksum of a table object computed by the server based on the value of other fields. It may be sent on update requests to ensure the client has an up-to-date value before proceeding. It is only checked for update table operations. |
| `delete_time` | String |  | Output only. The deletion time of the table. Only set after the table is deleted. |
| `type` | String |  | The table type. |
| `parent` | String | ✅ | Required. The parent resource where this table will be created. Format: projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expire_time` | String | Output only. The time when this table is considered expired. Only set after the table is deleted. |
| `name` | String | Output only. The resource name. Format: projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}/tables/{table_id} |
| `update_time` | String | Output only. The last modification time of the table. |
| `hive_options` | String | Options of a Hive table. |
| `create_time` | String | Output only. The creation time of the table. |
| `etag` | String | The checksum of a table object computed by the server based on the value of other fields. It may be sent on update requests to ensure the client has an up-to-date value before proceeding. It is only checked for update table operations. |
| `delete_time` | String | Output only. The deletion time of the table. Only set after the table is deleted. |
| `type` | String | The table type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create table
table = provider.biglake_api.Table {
    parent = "value"  # Required. The parent resource where this table will be created. Format: projects/{project_id_or_number}/locations/{location_id}/catalogs/{catalog_id}/databases/{database_id}
}

# Access table outputs
table_id = table.id
table_expire_time = table.expire_time
table_name = table.name
table_update_time = table.update_time
table_hive_options = table.hive_options
table_create_time = table.create_time
table_etag = table.etag
table_delete_time = table.delete_time
table_type = table.type
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple database resources
database_0 = provider.biglake_api.Database {
    parent = "value-0"
}
database_1 = provider.biglake_api.Database {
    parent = "value-1"
}
database_2 = provider.biglake_api.Database {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    database = provider.biglake_api.Database {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Biglake_api Documentation](https://cloud.google.com/biglake_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
