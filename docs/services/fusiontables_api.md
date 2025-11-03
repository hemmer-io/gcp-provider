# Fusiontables_api Service



**Resources**: 12

---

## Overview

The fusiontables_api service provides access to 12 resource types:

- [Template](#template) [CRUD]
- [Query](#query) [CR]
- [Column](#column) [CRUD]
- [Task](#task) [RD]
- [Table](#table) [CRUD]
- [Style](#style) [CRUD]
- [Column](#column) [CRUD]
- [Query](#query) [CR]
- [Template](#template) [CRUD]
- [Table](#table) [CRUD]
- [Style](#style) [CRUD]
- [Task](#task) [RD]

---

## Resources


### Template

Creates a new template for the table.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `body` | String |  | Body of the template. It contains HTML with {column_name} to insert values from a particular column. The body is sanitized to remove certain tags, e.g., script. Only one of body or automaticColumns can be specified. |
| `name` | String |  | Optional name assigned to a template. |
| `kind` | String |  | Type name: a template for the info window contents. The template can either include an HTML body or a list of columns from which the template is computed automatically. |
| `template_id` | i64 |  | Identifier for the template, unique within the context of a particular table. |
| `automatic_column_names` | Vec<String> |  | List of columns from which the template is to be automatically constructed. Only one of body or automaticColumns can be specified. |
| `table_id` | String |  | Identifier for the table for which the template is defined. |
| `table_id` | String | ✅ | Table for which a new template is being created |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `body` | String | Body of the template. It contains HTML with {column_name} to insert values from a particular column. The body is sanitized to remove certain tags, e.g., script. Only one of body or automaticColumns can be specified. |
| `name` | String | Optional name assigned to a template. |
| `kind` | String | Type name: a template for the info window contents. The template can either include an HTML body or a list of columns from which the template is computed automatically. |
| `template_id` | i64 | Identifier for the template, unique within the context of a particular table. |
| `automatic_column_names` | Vec<String> | List of columns from which the template is to be automatically constructed. Only one of body or automaticColumns can be specified. |
| `table_id` | String | Identifier for the table for which the template is defined. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create template
template = provider.fusiontables_api.Template {
    table_id = "value"  # Table for which a new template is being created
}

# Access template outputs
template_id = template.id
template_body = template.body
template_name = template.name
template_kind = template.kind
template_template_id = template.template_id
template_automatic_column_names = template.automatic_column_names
template_table_id = template.table_id
```

---


### Query

Executes an SQL SELECT/INSERT/UPDATE/DELETE/SHOW/DESCRIBE/CREATE statement.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `columns` | Vec<String> | Columns in the table. |
| `kind` | String | Type name: a template for an individual table. |
| `rows` | Vec<Vec<String>> | The rows in the table. For each cell we print out whatever cell value (e.g., numeric, string) exists. Thus it is important that each cell contains only one value. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create query
query = provider.fusiontables_api.Query {
}

# Access query outputs
query_id = query.id
query_columns = query.columns
query_kind = query.kind
query_rows = query.rows
```

---


### Column

Adds a new column to the table.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `column_id` | i64 |  | Identifier for the column. |
| `graph_predicate` | String |  | Optional column predicate. Used to map table to graph data model (subject,predicate,object) See http://www.w3.org/TR/2014/REC-rdf11-concepts-20140225/#data-model |
| `kind` | String |  | Type name: a template for an individual column. |
| `base_column` | String |  | Optional identifier of the base column. If present, this column is derived from the specified base column. |
| `description` | String |  | Optional column description. |
| `name` | String |  | Required name of the column. |
| `type` | String |  | Required type of the column. |
| `table_id` | String | ✅ | Table for which a new column is being added. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `column_id` | i64 | Identifier for the column. |
| `graph_predicate` | String | Optional column predicate. Used to map table to graph data model (subject,predicate,object) See http://www.w3.org/TR/2014/REC-rdf11-concepts-20140225/#data-model |
| `kind` | String | Type name: a template for an individual column. |
| `base_column` | String | Optional identifier of the base column. If present, this column is derived from the specified base column. |
| `description` | String | Optional column description. |
| `name` | String | Required name of the column. |
| `type` | String | Required type of the column. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create column
column = provider.fusiontables_api.Column {
    table_id = "value"  # Table for which a new column is being added.
}

# Access column outputs
column_id = column.id
column_column_id = column.column_id
column_graph_predicate = column.graph_predicate
column_kind = column.kind
column_base_column = column.base_column
column_description = column.description
column_name = column.name
column_type = column.type
```

---


### Task

Retrieves a specific task by its id.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `started` | bool | false while the table is busy with some other task. true if this background task is currently running. |
| `kind` | String | Type of the resource. This is always "fusiontables#task". |
| `progress` | String | An indication of task progress. |
| `type` | String | Type of background task. One of  DELETE_ROWS Deletes one or more rows from the table. ADD_ROWS "Adds one or more rows to a table. Includes importing data into a new table and importing more rows into an existing table. ADD_COLUMN Adds a new column to the table. CHANGE_TYPE Changes the type of a column. |
| `task_id` | String | Identifier for the task. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access task outputs
task_id = task.id
task_started = task.started
task_kind = task.kind
task_progress = task.progress
task_type = task.type
task_task_id = task.task_id
```

---


### Table

Creates a new table.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attribution` | String |  | Optional attribution assigned to the table. |
| `base_table_ids` | Vec<String> |  | Optional base table identifier if this table is a view or merged table. |
| `description` | String |  | Optional description assigned to the table. |
| `table_id` | String |  | Encrypted unique alphanumeric identifier for the table. |
| `attribution_link` | String |  | Optional link for attribution. |
| `is_exportable` | bool |  | Variable for whether table is exportable. |
| `columns` | Vec<String> |  | Columns in the table. |
| `name` | String |  | Name assigned to a table. |
| `sql` | String |  | Optional sql that encodes the table definition for derived tables. |
| `kind` | String |  | Type name: a template for an individual table. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attribution` | String | Optional attribution assigned to the table. |
| `base_table_ids` | Vec<String> | Optional base table identifier if this table is a view or merged table. |
| `description` | String | Optional description assigned to the table. |
| `table_id` | String | Encrypted unique alphanumeric identifier for the table. |
| `attribution_link` | String | Optional link for attribution. |
| `is_exportable` | bool | Variable for whether table is exportable. |
| `columns` | Vec<String> | Columns in the table. |
| `name` | String | Name assigned to a table. |
| `sql` | String | Optional sql that encodes the table definition for derived tables. |
| `kind` | String | Type name: a template for an individual table. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create table
table = provider.fusiontables_api.Table {
}

# Access table outputs
table_id = table.id
table_attribution = table.attribution
table_base_table_ids = table.base_table_ids
table_description = table.description
table_table_id = table.table_id
table_attribution_link = table.attribution_link
table_is_exportable = table.is_exportable
table_columns = table.columns
table_name = table.name
table_sql = table.sql
table_kind = table.kind
```

---


### Style

Adds a new style for the table.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `polygon_options` | String |  | Style definition for polygons in the table. |
| `marker_options` | String |  | Style definition for points in the table. |
| `style_id` | i64 |  | Identifier for the style setting (unique only within tables). |
| `table_id` | String |  | Identifier for the table. |
| `name` | String |  | Optional name for the style setting. |
| `kind` | String |  | Type name: an individual style setting. A StyleSetting contains the style defintions for points, lines, and polygons in a table. Since a table can have any one or all of them, a style definition can have point, line and polygon style definitions. |
| `polyline_options` | String |  | Style definition for lines in the table. |
| `table_id` | String | ✅ | Table for which a new style is being added |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `polygon_options` | String | Style definition for polygons in the table. |
| `marker_options` | String | Style definition for points in the table. |
| `style_id` | i64 | Identifier for the style setting (unique only within tables). |
| `table_id` | String | Identifier for the table. |
| `name` | String | Optional name for the style setting. |
| `kind` | String | Type name: an individual style setting. A StyleSetting contains the style defintions for points, lines, and polygons in a table. Since a table can have any one or all of them, a style definition can have point, line and polygon style definitions. |
| `polyline_options` | String | Style definition for lines in the table. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create style
style = provider.fusiontables_api.Style {
    table_id = "value"  # Table for which a new style is being added
}

# Access style outputs
style_id = style.id
style_polygon_options = style.polygon_options
style_marker_options = style.marker_options
style_style_id = style.style_id
style_table_id = style.table_id
style_name = style.name
style_kind = style.kind
style_polyline_options = style.polyline_options
```

---


### Column

Adds a new column to the table.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `validate_data` | bool |  | If true, data entered via the web application is validated. |
| `graph_predicate` | String |  | Column graph predicate.
Used to map table to graph data model (subject,predicate,object)
See W3C Graph-based Data Model. |
| `column_json_schema` | String |  | JSON schema for interpreting JSON in this column. |
| `column_properties_json` | String |  | JSON object containing custom column properties. |
| `kind` | String |  | The kind of item this is. For a column, this is always fusiontables#column. |
| `valid_values` | Vec<String> |  | List of valid values used to validate data and supply a drop-down list of values in the web application. |
| `name` | String |  | Name of the column. |
| `base_column` | String |  | Identifier of the base column. If present, this column is derived from the specified base column. |
| `type` | String |  | Type of the column. |
| `column_id` | i64 |  | Identifier for the column. |
| `description` | String |  | Column description. |
| `format_pattern` | String |  | Format pattern.
Acceptable values are DT_DATE_MEDIUMe.g Dec 24, 2008 DT_DATE_SHORTfor example 12/24/08 DT_DATE_TIME_MEDIUMfor example Dec 24, 2008 8:30:45 PM DT_DATE_TIME_SHORTfor example 12/24/08 8:30 PM DT_DAY_MONTH_2_DIGIT_YEARfor example 24/12/08 DT_DAY_MONTH_2_DIGIT_YEAR_TIMEfor example 24/12/08 20:30 DT_DAY_MONTH_2_DIGIT_YEAR_TIME_MERIDIANfor example 24/12/08 8:30 PM DT_DAY_MONTH_4_DIGIT_YEARfor example 24/12/2008 DT_DAY_MONTH_4_DIGIT_YEAR_TIMEfor example 24/12/2008 20:30 DT_DAY_MONTH_4_DIGIT_YEAR_TIME_MERIDIANfor example 24/12/2008 8:30 PM DT_ISO_YEAR_MONTH_DAYfor example 2008-12-24 DT_ISO_YEAR_MONTH_DAY_TIMEfor example 2008-12-24 20:30:45 DT_MONTH_DAY_4_DIGIT_YEARfor example 12/24/2008 DT_TIME_LONGfor example 8:30:45 PM UTC-6 DT_TIME_MEDIUMfor example 8:30:45 PM DT_TIME_SHORTfor example 8:30 PM DT_YEAR_ONLYfor example 2008 HIGHLIGHT_UNTYPED_CELLSHighlight cell data that does not match the data type NONENo formatting (default) NUMBER_CURRENCYfor example $1234.56 NUMBER_DEFAULTfor example 1,234.56 NUMBER_INTEGERfor example 1235 NUMBER_NO_SEPARATORfor example 1234.56 NUMBER_PERCENTfor example 123,456% NUMBER_SCIENTIFICfor example 1E3 STRING_EIGHT_LINE_IMAGEDisplays thumbnail images as tall as eight lines of text STRING_FOUR_LINE_IMAGEDisplays thumbnail images as tall as four lines of text STRING_JSON_TEXTAllows editing of text as JSON in UI STRING_JSON_LISTAllows editing of text as a JSON list in UI STRING_LINKTreats cell as a link (must start with http:// or https://) STRING_ONE_LINE_IMAGEDisplays thumbnail images as tall as one line of text STRING_VIDEO_OR_MAPDisplay a video or map thumbnail |
| `table_id` | String | ✅ | Table for which a new column is being added. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `validate_data` | bool | If true, data entered via the web application is validated. |
| `graph_predicate` | String | Column graph predicate.
Used to map table to graph data model (subject,predicate,object)
See W3C Graph-based Data Model. |
| `column_json_schema` | String | JSON schema for interpreting JSON in this column. |
| `column_properties_json` | String | JSON object containing custom column properties. |
| `kind` | String | The kind of item this is. For a column, this is always fusiontables#column. |
| `valid_values` | Vec<String> | List of valid values used to validate data and supply a drop-down list of values in the web application. |
| `name` | String | Name of the column. |
| `base_column` | String | Identifier of the base column. If present, this column is derived from the specified base column. |
| `type` | String | Type of the column. |
| `column_id` | i64 | Identifier for the column. |
| `description` | String | Column description. |
| `format_pattern` | String | Format pattern.
Acceptable values are DT_DATE_MEDIUMe.g Dec 24, 2008 DT_DATE_SHORTfor example 12/24/08 DT_DATE_TIME_MEDIUMfor example Dec 24, 2008 8:30:45 PM DT_DATE_TIME_SHORTfor example 12/24/08 8:30 PM DT_DAY_MONTH_2_DIGIT_YEARfor example 24/12/08 DT_DAY_MONTH_2_DIGIT_YEAR_TIMEfor example 24/12/08 20:30 DT_DAY_MONTH_2_DIGIT_YEAR_TIME_MERIDIANfor example 24/12/08 8:30 PM DT_DAY_MONTH_4_DIGIT_YEARfor example 24/12/2008 DT_DAY_MONTH_4_DIGIT_YEAR_TIMEfor example 24/12/2008 20:30 DT_DAY_MONTH_4_DIGIT_YEAR_TIME_MERIDIANfor example 24/12/2008 8:30 PM DT_ISO_YEAR_MONTH_DAYfor example 2008-12-24 DT_ISO_YEAR_MONTH_DAY_TIMEfor example 2008-12-24 20:30:45 DT_MONTH_DAY_4_DIGIT_YEARfor example 12/24/2008 DT_TIME_LONGfor example 8:30:45 PM UTC-6 DT_TIME_MEDIUMfor example 8:30:45 PM DT_TIME_SHORTfor example 8:30 PM DT_YEAR_ONLYfor example 2008 HIGHLIGHT_UNTYPED_CELLSHighlight cell data that does not match the data type NONENo formatting (default) NUMBER_CURRENCYfor example $1234.56 NUMBER_DEFAULTfor example 1,234.56 NUMBER_INTEGERfor example 1235 NUMBER_NO_SEPARATORfor example 1234.56 NUMBER_PERCENTfor example 123,456% NUMBER_SCIENTIFICfor example 1E3 STRING_EIGHT_LINE_IMAGEDisplays thumbnail images as tall as eight lines of text STRING_FOUR_LINE_IMAGEDisplays thumbnail images as tall as four lines of text STRING_JSON_TEXTAllows editing of text as JSON in UI STRING_JSON_LISTAllows editing of text as a JSON list in UI STRING_LINKTreats cell as a link (must start with http:// or https://) STRING_ONE_LINE_IMAGEDisplays thumbnail images as tall as one line of text STRING_VIDEO_OR_MAPDisplay a video or map thumbnail |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create column
column = provider.fusiontables_api.Column {
    table_id = "value"  # Table for which a new column is being added.
}

# Access column outputs
column_id = column.id
column_validate_data = column.validate_data
column_graph_predicate = column.graph_predicate
column_column_json_schema = column.column_json_schema
column_column_properties_json = column.column_properties_json
column_kind = column.kind
column_valid_values = column.valid_values
column_name = column.name
column_base_column = column.base_column
column_type = column.type
column_column_id = column.column_id
column_description = column.description
column_format_pattern = column.format_pattern
```

---


### Query

Executes a Fusion Tables SQL statement, which can be any of 
- SELECT
- INSERT
- UPDATE
- DELETE
- SHOW
- DESCRIBE
- CREATE statement.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `columns` | Vec<String> | Columns in the table. |
| `kind` | String | The kind of item this is. For responses to SQL queries, this is always fusiontables#sqlresponse. |
| `rows` | Vec<Vec<String>> | The rows in the table. For each cell we print out whatever cell value (e.g., numeric, string) exists. Thus it is important that each cell contains only one value. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create query
query = provider.fusiontables_api.Query {
}

# Access query outputs
query_id = query.id
query_columns = query.columns
query_kind = query.kind
query_rows = query.rows
```

---


### Template

Creates a new template for the table.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Optional name assigned to a template. |
| `automatic_column_names` | Vec<String> |  | List of columns from which the template is to be automatically constructed. Only one of body or automaticColumns can be specified. |
| `body` | String |  | Body of the template. It contains HTML with {column_name} to insert values from a particular column. The body is sanitized to remove certain tags, e.g., script. Only one of body or automaticColumns can be specified. |
| `kind` | String |  | The kind of item this is. For a template, this is always fusiontables#template. |
| `table_id` | String |  | Identifier for the table for which the template is defined. |
| `template_id` | i64 |  | Identifier for the template, unique within the context of a particular table. |
| `table_id` | String | ✅ | Table for which a new template is being created |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Optional name assigned to a template. |
| `automatic_column_names` | Vec<String> | List of columns from which the template is to be automatically constructed. Only one of body or automaticColumns can be specified. |
| `body` | String | Body of the template. It contains HTML with {column_name} to insert values from a particular column. The body is sanitized to remove certain tags, e.g., script. Only one of body or automaticColumns can be specified. |
| `kind` | String | The kind of item this is. For a template, this is always fusiontables#template. |
| `table_id` | String | Identifier for the table for which the template is defined. |
| `template_id` | i64 | Identifier for the template, unique within the context of a particular table. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create template
template = provider.fusiontables_api.Template {
    table_id = "value"  # Table for which a new template is being created
}

# Access template outputs
template_id = template.id
template_name = template.name
template_automatic_column_names = template.automatic_column_names
template_body = template.body
template_kind = template.kind
template_table_id = template.table_id
template_template_id = template.template_id
```

---


### Table

Creates a new table.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attribution_link` | String |  | Optional link for attribution. |
| `name` | String |  | Name assigned to a table. |
| `attribution` | String |  | Attribution assigned to the table. |
| `kind` | String |  | The kind of item this is. For a table, this is always fusiontables#table. |
| `sql` | String |  | SQL that encodes the table definition for derived tables. |
| `is_exportable` | bool |  | Variable for whether table is exportable. |
| `table_properties_json_schema` | String |  | JSON schema for validating the JSON table properties. |
| `base_table_ids` | Vec<String> |  | Base table identifier if this table is a view or merged table. |
| `table_id` | String |  | Encrypted unique alphanumeric identifier for the table. |
| `column_properties_json_schema` | String |  | Default JSON schema for validating all JSON column properties. |
| `columns` | Vec<String> |  | Columns in the table. |
| `description` | String |  | Description assigned to the table. |
| `table_properties_json` | String |  | JSON object containing custom table properties. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attribution_link` | String | Optional link for attribution. |
| `name` | String | Name assigned to a table. |
| `attribution` | String | Attribution assigned to the table. |
| `kind` | String | The kind of item this is. For a table, this is always fusiontables#table. |
| `sql` | String | SQL that encodes the table definition for derived tables. |
| `is_exportable` | bool | Variable for whether table is exportable. |
| `table_properties_json_schema` | String | JSON schema for validating the JSON table properties. |
| `base_table_ids` | Vec<String> | Base table identifier if this table is a view or merged table. |
| `table_id` | String | Encrypted unique alphanumeric identifier for the table. |
| `column_properties_json_schema` | String | Default JSON schema for validating all JSON column properties. |
| `columns` | Vec<String> | Columns in the table. |
| `description` | String | Description assigned to the table. |
| `table_properties_json` | String | JSON object containing custom table properties. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create table
table = provider.fusiontables_api.Table {
}

# Access table outputs
table_id = table.id
table_attribution_link = table.attribution_link
table_name = table.name
table_attribution = table.attribution
table_kind = table.kind
table_sql = table.sql
table_is_exportable = table.is_exportable
table_table_properties_json_schema = table.table_properties_json_schema
table_base_table_ids = table.base_table_ids
table_table_id = table.table_id
table_column_properties_json_schema = table.column_properties_json_schema
table_columns = table.columns
table_description = table.description
table_table_properties_json = table.table_properties_json
```

---


### Style

Adds a new style for the table.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `style_id` | i64 |  | Identifier for the style setting (unique only within tables). |
| `table_id` | String |  | Identifier for the table. |
| `name` | String |  | Optional name for the style setting. |
| `kind` | String |  | The kind of item this is. A StyleSetting contains the style definitions for points, lines, and polygons in a table. Since a table can have any one or all of them, a style definition can have point, line and polygon style definitions. |
| `marker_options` | String |  | Style definition for points in the table. |
| `polygon_options` | String |  | Style definition for polygons in the table. |
| `polyline_options` | String |  | Style definition for lines in the table. |
| `table_id` | String | ✅ | Table for which a new style is being added |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `style_id` | i64 | Identifier for the style setting (unique only within tables). |
| `table_id` | String | Identifier for the table. |
| `name` | String | Optional name for the style setting. |
| `kind` | String | The kind of item this is. A StyleSetting contains the style definitions for points, lines, and polygons in a table. Since a table can have any one or all of them, a style definition can have point, line and polygon style definitions. |
| `marker_options` | String | Style definition for points in the table. |
| `polygon_options` | String | Style definition for polygons in the table. |
| `polyline_options` | String | Style definition for lines in the table. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create style
style = provider.fusiontables_api.Style {
    table_id = "value"  # Table for which a new style is being added
}

# Access style outputs
style_id = style.id
style_style_id = style.style_id
style_table_id = style.table_id
style_name = style.name
style_kind = style.kind
style_marker_options = style.marker_options
style_polygon_options = style.polygon_options
style_polyline_options = style.polyline_options
```

---


### Task

Retrieves a specific task by its ID.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Type of background task. |
| `task_id` | String | Identifier for the task. |
| `kind` | String | Type of the resource. This is always "fusiontables#task". |
| `progress` | String | Task percentage completion. |
| `started` | bool | false while the table is busy with some other task. true if this background task is currently running. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access task outputs
task_id = task.id
task_type = task.type
task_task_id = task.task_id
task_kind = task.kind
task_progress = task.progress
task_started = task.started
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple template resources
template_0 = provider.fusiontables_api.Template {
    table_id = "value-0"
}
template_1 = provider.fusiontables_api.Template {
    table_id = "value-1"
}
template_2 = provider.fusiontables_api.Template {
    table_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    template = provider.fusiontables_api.Template {
        table_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Fusiontables_api Documentation](https://cloud.google.com/fusiontables_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
