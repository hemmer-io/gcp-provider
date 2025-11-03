# Script_api Service



**Resources**: 5

---

## Overview

The script_api service provides access to 5 resource types:

- [Version](#version) [CR]
- [Project](#project) [CRU]
- [Script](#script) [C]
- [Deployment](#deployment) [CRUD]
- [Processe](#processe) [R]

---

## Resources


### Version

Creates a new immutable version using the current code, with a unique version number.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description for this version. |
| `script_id` | String |  | The script project's Drive ID. |
| `create_time` | String |  | When the version was created. |
| `version_number` | i64 |  | The incremental ID that is created by Apps Script when a version is created. This is system assigned number and is immutable once created. |
| `script_id` | String | ✅ | The script project's Drive ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | The description for this version. |
| `script_id` | String | The script project's Drive ID. |
| `create_time` | String | When the version was created. |
| `version_number` | i64 | The incremental ID that is created by Apps Script when a version is created. This is system assigned number and is immutable once created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.script_api.Version {
    script_id = "value"  # The script project's Drive ID.
}

# Access version outputs
version_id = version.id
version_description = version.description
version_script_id = version.script_id
version_create_time = version.create_time
version_version_number = version.version_number
```

---


### Project

Creates a new, empty script project with no script files and a base manifest file.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent_id` | String |  | The Drive ID of a parent file that the created script project is bound to. This is usually the ID of a Google Doc, Google Sheet, Google Form, or Google Slides file. If not set, a standalone script project is created. |
| `title` | String |  | The title for the project. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | When the script was last updated. |
| `creator` | String | User who originally created the script. |
| `last_modify_user` | String | User who last modified the script. |
| `create_time` | String | When the script was created. |
| `parent_id` | String | The parent's Drive ID that the script will be attached to. This is usually the ID of a Google Document or Google Sheet. This field is optional, and if not set, a stand-alone script will be created. |
| `script_id` | String | The script project's Drive ID. |
| `title` | String | The title for the project. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.script_api.Project {
}

# Access project outputs
project_id = project.id
project_update_time = project.update_time
project_creator = project.creator
project_last_modify_user = project.last_modify_user
project_create_time = project.create_time
project_parent_id = project.parent_id
project_script_id = project.script_id
project_title = project.title
```

---


### Script



**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `function` | String |  | The name of the function to execute in the given script. The name does not include parentheses or parameters. It can reference a function in an included library such as `Library.libFunction1`. |
| `parameters` | Vec<String> |  | The parameters to be passed to the function being executed. The object type for each parameter should match the expected type in Apps Script. Parameters cannot be Apps Script-specific object types (such as a `Document` or a `Calendar`); they can only be primitive types such as `string`, `number`, `array`, `object`, or `boolean`. Optional. |
| `session_state` | String |  | *Deprecated*. For use with Android add-ons only. An ID that represents the user's current session in the Android app for Google Docs or Sheets, included as extra data in the [Intent](https://developer.android.com/guide/components/intents-filters.html) that launches the add-on. When an Android add-on is run with a session state, it gains the privileges of a [bound](https://developers.google.com/apps-script/guides/bound) script—that is, it can access information like the user's current cursor position (in Docs) or selected cell (in Sheets). To retrieve the state, call `Intent.getStringExtra("com.google.android.apps.docs.addons.SessionState")`. Optional. |
| `dev_mode` | bool |  | If `true` and the user is an owner of the script, the script runs at the most recently saved version rather than the version deployed for use with the Apps Script API. Optional; default is `false`. |
| `script_id` | String | ✅ | The script ID of the script to be executed. Find the script ID on the **Project settings** page under "IDs." As multiple executable APIs can be deployed in new IDE for same script, this field should be populated with DeploymentID generated while deploying in new IDE instead of script ID. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create script
script = provider.script_api.Script {
    script_id = "value"  # The script ID of the script to be executed. Find the script ID on the **Project settings** page under "IDs." As multiple executable APIs can be deployed in new IDE for same script, this field should be populated with DeploymentID generated while deploying in new IDE instead of script ID.
}

```

---


### Deployment

Creates a deployment of an Apps Script project.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | The description for this deployment. |
| `manifest_file_name` | String |  | The manifest file name for this deployment. |
| `script_id` | String |  | The script project's Drive ID. |
| `version_number` | i64 |  | The version number on which this deployment is based. |
| `script_id` | String | ✅ | The script project's Drive ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Last modified date time stamp. |
| `deployment_id` | String | The deployment ID for this deployment. |
| `entry_points` | Vec<String> | The deployment's entry points. |
| `deployment_config` | String | The deployment configuration. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create deployment
deployment = provider.script_api.Deployment {
    script_id = "value"  # The script project's Drive ID.
}

# Access deployment outputs
deployment_id = deployment.id
deployment_update_time = deployment.update_time
deployment_deployment_id = deployment.deployment_id
deployment_entry_points = deployment.entry_points
deployment_deployment_config = deployment.deployment_config
```

---


### Processe

List information about a script's executed processes, such as process type and current status.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `processes` | Vec<String> | List of processes matching request parameters. |
| `next_page_token` | String | Token for the next page of results. If empty, there are no more pages remaining. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access processe outputs
processe_id = processe.id
processe_processes = processe.processes
processe_next_page_token = processe.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple version resources
version_0 = provider.script_api.Version {
    script_id = "value-0"
}
version_1 = provider.script_api.Version {
    script_id = "value-1"
}
version_2 = provider.script_api.Version {
    script_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    version = provider.script_api.Version {
        script_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Script_api Documentation](https://cloud.google.com/script_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
