# Sheets_api Service



**Resources**: 4

---

## Overview

The sheets_api service provides access to 4 resource types:

- [Spreadsheet](#spreadsheet) [CR]
- [Value](#value) [CRU]
- [Developer_metadata](#developer_metadata) [CR]
- [Sheet](#sheet) [C]

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
| `named_ranges` | Vec<String> |  | The named ranges defined in a spreadsheet. |
| `spreadsheet_id` | String |  | The ID of the spreadsheet. This field is read-only. |
| `properties` | String |  | Overall properties of a spreadsheet. |
| `data_source_schedules` | Vec<String> |  | Output only. A list of data source refresh schedules. |
| `developer_metadata` | Vec<String> |  | The developer metadata associated with a spreadsheet. |
| `data_sources` | Vec<String> |  | A list of external data sources connected with the spreadsheet. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spreadsheet_url` | String | The url of the spreadsheet. This field is read-only. |
| `sheets` | Vec<String> | The sheets that are part of a spreadsheet. |
| `named_ranges` | Vec<String> | The named ranges defined in a spreadsheet. |
| `spreadsheet_id` | String | The ID of the spreadsheet. This field is read-only. |
| `properties` | String | Overall properties of a spreadsheet. |
| `data_source_schedules` | Vec<String> | Output only. A list of data source refresh schedules. |
| `developer_metadata` | Vec<String> | The developer metadata associated with a spreadsheet. |
| `data_sources` | Vec<String> | A list of external data sources connected with the spreadsheet. |


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
spreadsheet_named_ranges = spreadsheet.named_ranges
spreadsheet_spreadsheet_id = spreadsheet.spreadsheet_id
spreadsheet_properties = spreadsheet.properties
spreadsheet_data_source_schedules = spreadsheet.data_source_schedules
spreadsheet_developer_metadata = spreadsheet.developer_metadata
spreadsheet_data_sources = spreadsheet.data_sources
```

---


### Value

Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more DataFilterValueRanges.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `response_value_render_option` | String |  | Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE. |
| `data` | Vec<String> |  | The new values to apply to the spreadsheet. If more than one range is matched by the specified DataFilter the specified values are applied to all of those ranges. |
| `response_date_time_render_option` | String |  | Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER. |
| `value_input_option` | String |  | How the input data should be interpreted. |
| `include_values_in_response` | bool |  | Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. The `updatedData` field within each of the BatchUpdateValuesResponse.responses contains the updated values. If the range to write was larger than the range actually written, the response includes all values in the requested range (excluding trailing empty rows and columns). |
| `spreadsheet_id` | String | ✅ | The ID of the spreadsheet to update. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `range` | String | The range the values cover, in [A1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell). For output, this range indicates the entire requested range, even though the values will exclude trailing rows and columns. When appending values, this field represents the range to search for a table, after which values will be appended. |
| `values` | Vec<Vec<String>> | The data that was read or to be written. This is an array of arrays, the outer array representing all the data and each inner array representing a major dimension. Each item in the inner array corresponds with one cell. For output, empty trailing rows and columns will not be included. For input, supported value types are: bool, string, and double. Null values will be skipped. To set a cell to an empty value, set the string value to an empty string. |
| `major_dimension` | String | The major dimension of the values. For output, if the spreadsheet data is: `A1=1,B1=2,A2=3,B2=4`, then requesting `range=A1:B2,majorDimension=ROWS` will return `[[1,2],[3,4]]`, whereas requesting `range=A1:B2,majorDimension=COLUMNS` will return `[[1,3],[2,4]]`. For input, with `range=A1:B2,majorDimension=ROWS` then `[[1,2],[3,4]]` will set `A1=1,B1=2,A2=3,B2=4`. With `range=A1:B2,majorDimension=COLUMNS` then `[[1,2],[3,4]]` will set `A1=1,B1=3,A2=2,B2=4`. When writing, if this field is not set, it defaults to ROWS. |


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
value_range = value.range
value_values = value.values
value_major_dimension = value.major_dimension
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
| `location` | String | The location where the metadata is associated. |
| `metadata_value` | String | Data associated with the metadata's key. |
| `metadata_key` | String | The metadata key. There may be multiple metadata in a spreadsheet with the same key. Developer metadata must always have a key specified. |
| `metadata_id` | i64 | The spreadsheet-scoped unique ID that identifies the metadata. IDs may be specified when metadata is created, otherwise one will be randomly generated and assigned. Must be positive. |
| `visibility` | String | The metadata visibility. Developer metadata must always have a visibility specified. |


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
developer_metadata_location = developer_metadata.location
developer_metadata_metadata_value = developer_metadata.metadata_value
developer_metadata_metadata_key = developer_metadata.metadata_key
developer_metadata_metadata_id = developer_metadata.metadata_id
developer_metadata_visibility = developer_metadata.visibility
```

---


### Sheet

Copies a single sheet from a spreadsheet to another spreadsheet. Returns the properties of the newly created sheet.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination_spreadsheet_id` | String |  | The ID of the spreadsheet to copy the sheet to. |
| `sheet_id` | i64 | ✅ | The ID of the sheet to copy. |
| `spreadsheet_id` | String | ✅ | The ID of the spreadsheet containing the sheet to copy. |



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
    sheet_id = "value"  # The ID of the sheet to copy.
    spreadsheet_id = "value"  # The ID of the spreadsheet containing the sheet to copy.
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
