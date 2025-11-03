# Sheets_api Service



**Resources**: 4

---

## Overview

The sheets_api service provides access to 4 resource types:

- [Spreadsheet](#spreadsheet) [CR]
- [Sheet](#sheet) [C]
- [Developer_metadata](#developer_metadata) [CR]
- [Value](#value) [CRU]

---

## Resources


### Spreadsheet

Creates a spreadsheet, returning the newly created spreadsheet.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `spreadsheet_url` | String |  | The url of the spreadsheet. This field is read-only. |
| `sheets` | Vec<String> |  | The sheets that are part of a spreadsheet. |
| `developer_metadata` | Vec<String> |  | The developer metadata associated with a spreadsheet. |
| `spreadsheet_id` | String |  | The ID of the spreadsheet. This field is read-only. |
| `data_sources` | Vec<String> |  | A list of external data sources connected with the spreadsheet. |
| `properties` | String |  | Overall properties of a spreadsheet. |
| `named_ranges` | Vec<String> |  | The named ranges defined in a spreadsheet. |
| `data_source_schedules` | Vec<String> |  | Output only. A list of data source refresh schedules. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spreadsheet_url` | String | The url of the spreadsheet. This field is read-only. |
| `sheets` | Vec<String> | The sheets that are part of a spreadsheet. |
| `developer_metadata` | Vec<String> | The developer metadata associated with a spreadsheet. |
| `spreadsheet_id` | String | The ID of the spreadsheet. This field is read-only. |
| `data_sources` | Vec<String> | A list of external data sources connected with the spreadsheet. |
| `properties` | String | Overall properties of a spreadsheet. |
| `named_ranges` | Vec<String> | The named ranges defined in a spreadsheet. |
| `data_source_schedules` | Vec<String> | Output only. A list of data source refresh schedules. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create spreadsheet
spreadsheet = provider.sheets_api.Spreadsheet {
}

# Access spreadsheet outputs
spreadsheet_id = spreadsheet.id
spreadsheet_spreadsheet_url = spreadsheet.spreadsheet_url
spreadsheet_sheets = spreadsheet.sheets
spreadsheet_developer_metadata = spreadsheet.developer_metadata
spreadsheet_spreadsheet_id = spreadsheet.spreadsheet_id
spreadsheet_data_sources = spreadsheet.data_sources
spreadsheet_properties = spreadsheet.properties
spreadsheet_named_ranges = spreadsheet.named_ranges
spreadsheet_data_source_schedules = spreadsheet.data_source_schedules
```

---


### Sheet

Copies a single sheet from a spreadsheet to another spreadsheet. Returns the properties of the newly created sheet.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_spreadsheet_id` | String |  | The ID of the spreadsheet to copy the sheet to. |
| `spreadsheet_id` | String | ✅ | The ID of the spreadsheet containing the sheet to copy. |
| `sheet_id` | i64 | ✅ | The ID of the sheet to copy. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sheet
sheet = provider.sheets_api.Sheet {
    spreadsheet_id = "value"  # The ID of the spreadsheet containing the sheet to copy.
    sheet_id = "value"  # The ID of the sheet to copy.
}

```

---


### Developer_metadata

Returns all developer metadata matching the specified DataFilter. If the provided DataFilter represents a DeveloperMetadataLookup object, this will return all DeveloperMetadata entries selected by it. If the DataFilter represents a location in a spreadsheet, this will return all developer metadata associated with locations intersecting that region.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_filters` | Vec<String> |  | The data filters describing the criteria used to determine which DeveloperMetadata entries to return. DeveloperMetadata matching any of the specified filters are included in the response. |
| `spreadsheet_id` | String | ✅ | The ID of the spreadsheet to retrieve metadata from. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `visibility` | String | The metadata visibility. Developer metadata must always have a visibility specified. |
| `metadata_key` | String | The metadata key. There may be multiple metadata in a spreadsheet with the same key. Developer metadata must always have a key specified. |
| `metadata_id` | i64 | The spreadsheet-scoped unique ID that identifies the metadata. IDs may be specified when metadata is created, otherwise one will be randomly generated and assigned. Must be positive. |
| `location` | String | The location where the metadata is associated. |
| `metadata_value` | String | Data associated with the metadata's key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create developer_metadata
developer_metadata = provider.sheets_api.Developer_metadata {
    spreadsheet_id = "value"  # The ID of the spreadsheet to retrieve metadata from.
}

# Access developer_metadata outputs
developer_metadata_id = developer_metadata.id
developer_metadata_visibility = developer_metadata.visibility
developer_metadata_metadata_key = developer_metadata.metadata_key
developer_metadata_metadata_id = developer_metadata.metadata_id
developer_metadata_location = developer_metadata.location
developer_metadata_metadata_value = developer_metadata.metadata_value
```

---


### Value

Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more DataFilters. Ranges matching any of the specified data filters will be cleared. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `data_filters` | Vec<String> |  | The DataFilters used to determine which ranges to clear. |
| `spreadsheet_id` | String | ✅ | The ID of the spreadsheet to update. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `values` | Vec<Vec<String>> | The data that was read or to be written. This is an array of arrays, the outer array representing all the data and each inner array representing a major dimension. Each item in the inner array corresponds with one cell. For output, empty trailing rows and columns will not be included. For input, supported value types are: bool, string, and double. Null values will be skipped. To set a cell to an empty value, set the string value to an empty string. |
| `major_dimension` | String | The major dimension of the values. For output, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` will return `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` will return `[[1,3],[2,4]]`. For input, with `range=A1:B2,majorDimension=ROWS` then `[[1,2],[3,4]]` will set `A1=1,B1=2,A2=3,B2=4`. With `range=A1:B2,majorDimension=COLUMNS` then `[[1,2],[3,4]]` will set `A1=1,B1=3,A2=2,B2=4`. When writing, if this field is not set, it defaults to ROWS. |
| `range` | String | The range the values cover, in [A1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell). For output, this range indicates the entire requested range, even though the values will exclude trailing rows and columns. When appending values, this field represents the range to search for a table, after which values will be appended. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create value
value = provider.sheets_api.Value {
    spreadsheet_id = "value"  # The ID of the spreadsheet to update.
}

# Access value outputs
value_id = value.id
value_values = value.values
value_major_dimension = value.major_dimension
value_range = value.range
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple spreadsheet resources
spreadsheet_0 = provider.sheets_api.Spreadsheet {
}
spreadsheet_1 = provider.sheets_api.Spreadsheet {
}
spreadsheet_2 = provider.sheets_api.Spreadsheet {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    spreadsheet = provider.sheets_api.Spreadsheet {
    }
```

---

## Related Documentation

- [GCP Sheets_api Documentation](https://cloud.google.com/sheets_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
