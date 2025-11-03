# Tagmanager_api Service



**Resources**: 30

---

## Overview

The tagmanager_api service provides access to 30 resource types:

- [Tag](#tag) [CRUD]
- [Version](#version) [CRUD]
- [Permission](#permission) [CRUD]
- [Account](#account) [RU]
- [Container](#container) [CRUD]
- [Environment](#environment) [CRUD]
- [Variable](#variable) [CRUD]
- [Reauthorize_environment](#reauthorize_environment) [U]
- [Move_folder](#move_folder) [U]
- [Trigger](#trigger) [CRUD]
- [Entitie](#entitie) [R]
- [Folder](#folder) [CRUD]
- [Destination](#destination) [CR]
- [Template](#template) [CRUD]
- [Variable](#variable) [CRUD]
- [User_permission](#user_permission) [CRUD]
- [Account](#account) [RU]
- [Version](#version) [CRUD]
- [Tag](#tag) [CRUD]
- [Version_header](#version_header) [R]
- [Transformation](#transformation) [CRUD]
- [Trigger](#trigger) [CRUD]
- [Zone](#zone) [CRUD]
- [Folder](#folder) [CRUD]
- [Container](#container) [CRUD]
- [Gtag_config](#gtag_config) [CRUD]
- [Environment](#environment) [CRUD]
- [Workspace](#workspace) [CRUD]
- [Built_in_variable](#built_in_variable) [CRD]
- [Client](#client) [CRUD]

---

## Resources


### Tag

Creates a GTM Tag.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `firing_trigger_id` | Vec<String> |  | Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. |
| `account_id` | String |  | GTM Account ID. |
| `priority` | String |  | User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. |
| `tag_firing_option` | String |  | Option to fire this tag. |
| `blocking_trigger_id` | Vec<String> |  | Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. |
| `type` | String |  | GTM Tag Type. |
| `name` | String |  | Tag display name. |
| `schedule_start_ms` | String |  | The start timestamp in milliseconds to schedule a tag. |
| `schedule_end_ms` | String |  | The end timestamp in milliseconds to schedule a tag. |
| `notes` | String |  | User notes on how to apply this tag in the container. |
| `setup_tag` | Vec<String> |  | The list of setup tags. Currently we only allow one. |
| `container_id` | String |  | GTM Container ID. |
| `parameter` | Vec<String> |  | The tag's parameters. |
| `parent_folder_id` | String |  | Parent folder id. |
| `tag_id` | String |  | The Tag ID uniquely identifies the GTM Tag. |
| `teardown_tag` | Vec<String> |  | The list of teardown tags. Currently we only allow one. |
| `paused` | bool |  | True if the tag is paused. |
| `live_only` | bool |  | If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). |
| `fingerprint` | String |  | The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified. |
| `container_id` | String | ✅ | The GTM Container ID. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `firing_trigger_id` | Vec<String> | Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. |
| `account_id` | String | GTM Account ID. |
| `priority` | String | User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. |
| `tag_firing_option` | String | Option to fire this tag. |
| `blocking_trigger_id` | Vec<String> | Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. |
| `type` | String | GTM Tag Type. |
| `name` | String | Tag display name. |
| `schedule_start_ms` | String | The start timestamp in milliseconds to schedule a tag. |
| `schedule_end_ms` | String | The end timestamp in milliseconds to schedule a tag. |
| `notes` | String | User notes on how to apply this tag in the container. |
| `setup_tag` | Vec<String> | The list of setup tags. Currently we only allow one. |
| `container_id` | String | GTM Container ID. |
| `parameter` | Vec<String> | The tag's parameters. |
| `parent_folder_id` | String | Parent folder id. |
| `tag_id` | String | The Tag ID uniquely identifies the GTM Tag. |
| `teardown_tag` | Vec<String> | The list of teardown tags. Currently we only allow one. |
| `paused` | bool | True if the tag is paused. |
| `live_only` | bool | If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). |
| `fingerprint` | String | The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag
tag = provider.tagmanager_api.Tag {
    container_id = "value"  # The GTM Container ID.
    account_id = "value"  # The GTM Account ID.
}

# Access tag outputs
tag_id = tag.id
tag_firing_trigger_id = tag.firing_trigger_id
tag_account_id = tag.account_id
tag_priority = tag.priority
tag_tag_firing_option = tag.tag_firing_option
tag_blocking_trigger_id = tag.blocking_trigger_id
tag_type = tag.type
tag_name = tag.name
tag_schedule_start_ms = tag.schedule_start_ms
tag_schedule_end_ms = tag.schedule_end_ms
tag_notes = tag.notes
tag_setup_tag = tag.setup_tag
tag_container_id = tag.container_id
tag_parameter = tag.parameter
tag_parent_folder_id = tag.parent_folder_id
tag_tag_id = tag.tag_id
tag_teardown_tag = tag.teardown_tag
tag_paused = tag.paused
tag_live_only = tag.live_only
tag_fingerprint = tag.fingerprint
```

---


### Version

Creates a Container Version.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the container version to be created. |
| `notes` | String |  | The notes of the container version to be created. |
| `quick_preview` | bool |  | The creation of this version may be for quick preview and shouldn't be saved. |
| `account_id` | String | ✅ | The GTM Account ID. |
| `container_id` | String | ✅ | The GTM Container ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `container_version_id` | String | The Container Version ID uniquely identifies the GTM Container Version. |
| `container` | String | The container that this version was taken from. |
| `name` | String | Container version display name. |
| `fingerprint` | String | The fingerprint of the GTM Container Version as computed at storage time. This value is recomputed whenever the container version is modified. |
| `folder` | Vec<String> | The folders in the container that this version was taken from. |
| `notes` | String | User notes on how to apply this container version in the container. |
| `tag` | Vec<String> | The tags in the container that this version was taken from. |
| `trigger` | Vec<String> | The triggers in the container that this version was taken from. |
| `variable` | Vec<String> | The variables in the container that this version was taken from. |
| `account_id` | String | GTM Account ID. |
| `deleted` | bool | A value of true indicates this container version has been deleted. |
| `container_id` | String | GTM Container ID. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.tagmanager_api.Version {
    account_id = "value"  # The GTM Account ID.
    container_id = "value"  # The GTM Container ID.
}

# Access version outputs
version_id = version.id
version_container_version_id = version.container_version_id
version_container = version.container
version_name = version.name
version_fingerprint = version.fingerprint
version_folder = version.folder
version_notes = version.notes
version_tag = version.tag
version_trigger = version.trigger
version_variable = version.variable
version_account_id = version.account_id
version_deleted = version.deleted
version_container_id = version.container_id
```

---


### Permission

Creates a user's Account & Container Permissions.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email_address` | String |  | User's email address. |
| `permission_id` | String |  | Account Permission ID. |
| `account_access` | String |  | GTM Account access permissions. |
| `account_id` | String |  | GTM Account ID. |
| `container_access` | Vec<String> |  | GTM Container access permissions. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `email_address` | String | User's email address. |
| `permission_id` | String | Account Permission ID. |
| `account_access` | String | GTM Account access permissions. |
| `account_id` | String | GTM Account ID. |
| `container_access` | Vec<String> | GTM Container access permissions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create permission
permission = provider.tagmanager_api.Permission {
    account_id = "value"  # The GTM Account ID.
}

# Access permission outputs
permission_id = permission.id
permission_email_address = permission.email_address
permission_permission_id = permission.permission_id
permission_account_access = permission.account_access
permission_account_id = permission.account_id
permission_container_access = permission.container_access
```

---


### Account

Gets a GTM Account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fingerprint` | String |  | The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified. |
| `name` | String |  | Account display name. |
| `account_id` | String |  | The Account ID uniquely identifies the GTM Account. |
| `share_data` | bool |  | Whether the account shares data anonymously with Google and others. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fingerprint` | String | The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified. |
| `name` | String | Account display name. |
| `account_id` | String | The Account ID uniquely identifies the GTM Account. |
| `share_data` | bool | Whether the account shares data anonymously with Google and others. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account outputs
account_id = account.id
account_fingerprint = account.fingerprint
account_name = account.name
account_account_id = account.account_id
account_share_data = account.share_data
```

---


### Container

Creates a Container.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `public_id` | String |  | Container Public ID. |
| `name` | String |  | Container display name. |
| `container_id` | String |  | The Container ID uniquely identifies the GTM Container. |
| `account_id` | String |  | GTM Account ID. |
| `enabled_built_in_variable` | Vec<String> |  | List of enabled built-in variables. Valid values include: pageUrl, pageHostname, pagePath, referrer, event, clickElement, clickClasses, clickId, clickTarget, clickUrl, clickText, formElement, formClasses, formId, formTarget, formUrl, formText, errorMessage, errorUrl, errorLine, newHistoryFragment, oldHistoryFragment, newHistoryState, oldHistoryState, historySource, containerVersion, debugMode, randomNumber, containerId. |
| `fingerprint` | String |  | The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified. |
| `time_zone_country_id` | String |  | Container Country ID. |
| `time_zone_id` | String |  | Container Time Zone ID. |
| `usage_context` | Vec<String> |  | List of Usage Contexts for the Container. Valid values include: web, android, ios. |
| `domain_name` | Vec<String> |  | Optional list of domain names associated with the Container. |
| `notes` | String |  | Container Notes. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `public_id` | String | Container Public ID. |
| `name` | String | Container display name. |
| `container_id` | String | The Container ID uniquely identifies the GTM Container. |
| `account_id` | String | GTM Account ID. |
| `enabled_built_in_variable` | Vec<String> | List of enabled built-in variables. Valid values include: pageUrl, pageHostname, pagePath, referrer, event, clickElement, clickClasses, clickId, clickTarget, clickUrl, clickText, formElement, formClasses, formId, formTarget, formUrl, formText, errorMessage, errorUrl, errorLine, newHistoryFragment, oldHistoryFragment, newHistoryState, oldHistoryState, historySource, containerVersion, debugMode, randomNumber, containerId. |
| `fingerprint` | String | The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified. |
| `time_zone_country_id` | String | Container Country ID. |
| `time_zone_id` | String | Container Time Zone ID. |
| `usage_context` | Vec<String> | List of Usage Contexts for the Container. Valid values include: web, android, ios. |
| `domain_name` | Vec<String> | Optional list of domain names associated with the Container. |
| `notes` | String | Container Notes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create container
container = provider.tagmanager_api.Container {
    account_id = "value"  # The GTM Account ID.
}

# Access container outputs
container_id = container.id
container_public_id = container.public_id
container_name = container.name
container_container_id = container.container_id
container_account_id = container.account_id
container_enabled_built_in_variable = container.enabled_built_in_variable
container_fingerprint = container.fingerprint
container_time_zone_country_id = container.time_zone_country_id
container_time_zone_id = container.time_zone_id
container_usage_context = container.usage_context
container_domain_name = container.domain_name
container_notes = container.notes
```

---


### Environment

Creates a GTM Environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The environment display name. Can be set or changed only on USER type environments. |
| `container_version_id` | String |  |  |
| `account_id` | String |  | GTM Account ID. |
| `environment_id` | String |  | GTM Environment ID uniquely identifies the GTM Environment. |
| `url` | String |  | Default preview page url for the environment. |
| `authorization_timestamp_ms` | String |  | The last update time-stamp for the authorization code. |
| `enable_debug` | bool |  | Whether or not to enable debug by default on for the environment. |
| `authorization_code` | String |  | The environment authorization code. |
| `fingerprint` | String |  | The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified. |
| `type` | String |  | The type of this environment. |
| `description` | String |  | The environment description. Can be set or changed only on USER type environments. |
| `container_id` | String |  | GTM Container ID. |
| `container_id` | String | ✅ | The GTM Container ID. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The environment display name. Can be set or changed only on USER type environments. |
| `container_version_id` | String |  |
| `account_id` | String | GTM Account ID. |
| `environment_id` | String | GTM Environment ID uniquely identifies the GTM Environment. |
| `url` | String | Default preview page url for the environment. |
| `authorization_timestamp_ms` | String | The last update time-stamp for the authorization code. |
| `enable_debug` | bool | Whether or not to enable debug by default on for the environment. |
| `authorization_code` | String | The environment authorization code. |
| `fingerprint` | String | The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified. |
| `type` | String | The type of this environment. |
| `description` | String | The environment description. Can be set or changed only on USER type environments. |
| `container_id` | String | GTM Container ID. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.tagmanager_api.Environment {
    container_id = "value"  # The GTM Container ID.
    account_id = "value"  # The GTM Account ID.
}

# Access environment outputs
environment_id = environment.id
environment_name = environment.name
environment_container_version_id = environment.container_version_id
environment_account_id = environment.account_id
environment_environment_id = environment.environment_id
environment_url = environment.url
environment_authorization_timestamp_ms = environment.authorization_timestamp_ms
environment_enable_debug = environment.enable_debug
environment_authorization_code = environment.authorization_code
environment_fingerprint = environment.fingerprint
environment_type = environment.type
environment_description = environment.description
environment_container_id = environment.container_id
```

---


### Variable

Creates a GTM Variable.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `container_id` | String |  | GTM Container ID. |
| `parameter` | Vec<String> |  | The variable's parameters. |
| `schedule_start_ms` | String |  | The start timestamp in milliseconds to schedule a variable. |
| `fingerprint` | String |  | The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified. |
| `enabling_trigger_id` | Vec<String> |  | For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. |
| `notes` | String |  | User notes on how to apply this variable in the container. |
| `disabling_trigger_id` | Vec<String> |  | For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. |
| `account_id` | String |  | GTM Account ID. |
| `parent_folder_id` | String |  | Parent folder id. |
| `name` | String |  | Variable display name. |
| `schedule_end_ms` | String |  | The end timestamp in milliseconds to schedule a variable. |
| `type` | String |  | GTM Variable Type. |
| `variable_id` | String |  | The Variable ID uniquely identifies the GTM Variable. |
| `account_id` | String | ✅ | The GTM Account ID. |
| `container_id` | String | ✅ | The GTM Container ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `container_id` | String | GTM Container ID. |
| `parameter` | Vec<String> | The variable's parameters. |
| `schedule_start_ms` | String | The start timestamp in milliseconds to schedule a variable. |
| `fingerprint` | String | The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified. |
| `enabling_trigger_id` | Vec<String> | For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. |
| `notes` | String | User notes on how to apply this variable in the container. |
| `disabling_trigger_id` | Vec<String> | For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. |
| `account_id` | String | GTM Account ID. |
| `parent_folder_id` | String | Parent folder id. |
| `name` | String | Variable display name. |
| `schedule_end_ms` | String | The end timestamp in milliseconds to schedule a variable. |
| `type` | String | GTM Variable Type. |
| `variable_id` | String | The Variable ID uniquely identifies the GTM Variable. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create variable
variable = provider.tagmanager_api.Variable {
    account_id = "value"  # The GTM Account ID.
    container_id = "value"  # The GTM Container ID.
}

# Access variable outputs
variable_id = variable.id
variable_container_id = variable.container_id
variable_parameter = variable.parameter
variable_schedule_start_ms = variable.schedule_start_ms
variable_fingerprint = variable.fingerprint
variable_enabling_trigger_id = variable.enabling_trigger_id
variable_notes = variable.notes
variable_disabling_trigger_id = variable.disabling_trigger_id
variable_account_id = variable.account_id
variable_parent_folder_id = variable.parent_folder_id
variable_name = variable.name
variable_schedule_end_ms = variable.schedule_end_ms
variable_type = variable.type
variable_variable_id = variable.variable_id
```

---


### Reauthorize_environment



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The environment display name. Can be set or changed only on USER type environments. |
| `container_version_id` | String |  |  |
| `account_id` | String |  | GTM Account ID. |
| `environment_id` | String |  | GTM Environment ID uniquely identifies the GTM Environment. |
| `url` | String |  | Default preview page url for the environment. |
| `authorization_timestamp_ms` | String |  | The last update time-stamp for the authorization code. |
| `enable_debug` | bool |  | Whether or not to enable debug by default on for the environment. |
| `authorization_code` | String |  | The environment authorization code. |
| `fingerprint` | String |  | The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified. |
| `type` | String |  | The type of this environment. |
| `description` | String |  | The environment description. Can be set or changed only on USER type environments. |
| `container_id` | String |  | GTM Container ID. |
| `account_id` | String | ✅ | The GTM Account ID. |
| `container_id` | String | ✅ | The GTM Container ID. |
| `environment_id` | String | ✅ | The GTM Environment ID. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

```

---


### Move_folder



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fingerprint` | String |  | The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified. |
| `name` | String |  | Folder display name. |
| `account_id` | String |  | GTM Account ID. |
| `folder_id` | String |  | The Folder ID uniquely identifies the GTM Folder. |
| `container_id` | String |  | GTM Container ID. |
| `account_id` | String | ✅ | The GTM Account ID. |
| `container_id` | String | ✅ | The GTM Container ID. |
| `folder_id` | String | ✅ | The GTM Folder ID. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

```

---


### Trigger

Creates a GTM Trigger.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auto_event_filter` | Vec<String> |  | Used in the case of auto event tracking. |
| `parent_folder_id` | String |  | Parent folder id. |
| `wait_for_tags` | String |  | Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. |
| `unique_trigger_id` | String |  | Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. |
| `wait_for_tags_timeout` | String |  | How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. |
| `visibility_selector` | String |  | A visibility trigger CSS selector (i.e. "#id"). Only valid for AMP Visibility trigger. |
| `filter` | Vec<String> |  | The trigger will only fire iff all Conditions are true. |
| `account_id` | String |  | GTM Account ID. |
| `horizontal_scroll_percentage_list` | String |  | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. |
| `continuous_time_min_milliseconds` | String |  | A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `type` | String |  | Defines the data layer event that causes this trigger. |
| `event_name` | String |  | Name of the GTM event that is fired. Only valid for Timer triggers. |
| `trigger_id` | String |  | The Trigger ID uniquely identifies the GTM Trigger. |
| `custom_event_filter` | Vec<String> |  | Used in the case of custom event, which is fired iff all Conditions are true. |
| `vertical_scroll_percentage_list` | String |  | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. |
| `visible_percentage_min` | String |  | A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. |
| `fingerprint` | String |  | The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified. |
| `parameter` | Vec<String> |  | Additional parameters. |
| `interval_seconds` | String |  | Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. |
| `check_validation` | String |  | Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. |
| `limit` | String |  | Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. |
| `max_timer_length_seconds` | String |  | Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. |
| `total_time_min_milliseconds` | String |  | A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `container_id` | String |  | GTM Container ID. |
| `name` | String |  | Trigger display name. |
| `selector` | String |  | A click trigger CSS selector (i.e. "a", "button" etc.). Only valid for AMP Click trigger. |
| `interval` | String |  | Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. |
| `visible_percentage_max` | String |  | A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. |
| `account_id` | String | ✅ | The GTM Account ID. |
| `container_id` | String | ✅ | The GTM Container ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `auto_event_filter` | Vec<String> | Used in the case of auto event tracking. |
| `parent_folder_id` | String | Parent folder id. |
| `wait_for_tags` | String | Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. |
| `unique_trigger_id` | String | Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. |
| `wait_for_tags_timeout` | String | How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. |
| `visibility_selector` | String | A visibility trigger CSS selector (i.e. "#id"). Only valid for AMP Visibility trigger. |
| `filter` | Vec<String> | The trigger will only fire iff all Conditions are true. |
| `account_id` | String | GTM Account ID. |
| `horizontal_scroll_percentage_list` | String | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. |
| `continuous_time_min_milliseconds` | String | A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `type` | String | Defines the data layer event that causes this trigger. |
| `event_name` | String | Name of the GTM event that is fired. Only valid for Timer triggers. |
| `trigger_id` | String | The Trigger ID uniquely identifies the GTM Trigger. |
| `custom_event_filter` | Vec<String> | Used in the case of custom event, which is fired iff all Conditions are true. |
| `vertical_scroll_percentage_list` | String | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. |
| `visible_percentage_min` | String | A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. |
| `fingerprint` | String | The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified. |
| `parameter` | Vec<String> | Additional parameters. |
| `interval_seconds` | String | Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. |
| `check_validation` | String | Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. |
| `limit` | String | Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. |
| `max_timer_length_seconds` | String | Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. |
| `total_time_min_milliseconds` | String | A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `container_id` | String | GTM Container ID. |
| `name` | String | Trigger display name. |
| `selector` | String | A click trigger CSS selector (i.e. "a", "button" etc.). Only valid for AMP Click trigger. |
| `interval` | String | Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. |
| `visible_percentage_max` | String | A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trigger
trigger = provider.tagmanager_api.Trigger {
    account_id = "value"  # The GTM Account ID.
    container_id = "value"  # The GTM Container ID.
}

# Access trigger outputs
trigger_id = trigger.id
trigger_auto_event_filter = trigger.auto_event_filter
trigger_parent_folder_id = trigger.parent_folder_id
trigger_wait_for_tags = trigger.wait_for_tags
trigger_unique_trigger_id = trigger.unique_trigger_id
trigger_wait_for_tags_timeout = trigger.wait_for_tags_timeout
trigger_visibility_selector = trigger.visibility_selector
trigger_filter = trigger.filter
trigger_account_id = trigger.account_id
trigger_horizontal_scroll_percentage_list = trigger.horizontal_scroll_percentage_list
trigger_continuous_time_min_milliseconds = trigger.continuous_time_min_milliseconds
trigger_type = trigger.type
trigger_event_name = trigger.event_name
trigger_trigger_id = trigger.trigger_id
trigger_custom_event_filter = trigger.custom_event_filter
trigger_vertical_scroll_percentage_list = trigger.vertical_scroll_percentage_list
trigger_visible_percentage_min = trigger.visible_percentage_min
trigger_fingerprint = trigger.fingerprint
trigger_parameter = trigger.parameter
trigger_interval_seconds = trigger.interval_seconds
trigger_check_validation = trigger.check_validation
trigger_limit = trigger.limit
trigger_max_timer_length_seconds = trigger.max_timer_length_seconds
trigger_total_time_min_milliseconds = trigger.total_time_min_milliseconds
trigger_container_id = trigger.container_id
trigger_name = trigger.name
trigger_selector = trigger.selector
trigger_interval = trigger.interval
trigger_visible_percentage_max = trigger.visible_percentage_max
```

---


### Entitie

List all entities in a GTM Folder.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag` | Vec<String> | The list of tags inside the folder. |
| `trigger` | Vec<String> | The list of triggers inside the folder. |
| `variable` | Vec<String> | The list of variables inside the folder. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access entitie outputs
entitie_id = entitie.id
entitie_tag = entitie.tag
entitie_trigger = entitie.trigger
entitie_variable = entitie.variable
```

---


### Folder

Creates a GTM Folder.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fingerprint` | String |  | The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified. |
| `name` | String |  | Folder display name. |
| `account_id` | String |  | GTM Account ID. |
| `folder_id` | String |  | The Folder ID uniquely identifies the GTM Folder. |
| `container_id` | String |  | GTM Container ID. |
| `account_id` | String | ✅ | The GTM Account ID. |
| `container_id` | String | ✅ | The GTM Container ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fingerprint` | String | The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified. |
| `name` | String | Folder display name. |
| `account_id` | String | GTM Account ID. |
| `folder_id` | String | The Folder ID uniquely identifies the GTM Folder. |
| `container_id` | String | GTM Container ID. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create folder
folder = provider.tagmanager_api.Folder {
    account_id = "value"  # The GTM Account ID.
    container_id = "value"  # The GTM Container ID.
}

# Access folder outputs
folder_id = folder.id
folder_fingerprint = folder.fingerprint
folder_name = folder.name
folder_account_id = folder.account_id
folder_folder_id = folder.folder_id
folder_container_id = folder.container_id
```

---


### Destination

Adds a Destination to this Container and removes it from the Container to which it is currently linked.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | GTM parent Container's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination_link_id` | String | The Destination link ID uniquely identifies the Destination. |
| `name` | String | Destination display name. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI. |
| `container_id` | String | GTM Container ID. |
| `destination_id` | String | Destination ID. |
| `fingerprint` | String | The fingerprint of the Google Tag Destination as computed at storage time. This value is recomputed whenever the destination is modified. |
| `path` | String | Destination's API relative path. |
| `account_id` | String | GTM Account ID. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create destination
destination = provider.tagmanager_api.Destination {
    parent = "value"  # GTM parent Container's API relative path.
}

# Access destination outputs
destination_id = destination.id
destination_destination_link_id = destination.destination_link_id
destination_name = destination.name
destination_tag_manager_url = destination.tag_manager_url
destination_container_id = destination.container_id
destination_destination_id = destination.destination_id
destination_fingerprint = destination.fingerprint
destination_path = destination.path
destination_account_id = destination.account_id
```

---


### Template

Creates a GTM Custom Template.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `path` | String |  | GTM Custom Template's API relative path. |
| `template_id` | String |  | The Custom Template ID uniquely identifies the GTM custom template. |
| `fingerprint` | String |  | The fingerprint of the GTM Custom Template as computed at storage time. This value is recomputed whenever the template is modified. |
| `template_data` | String |  | The custom template in text format. |
| `workspace_id` | String |  | GTM Workspace ID. |
| `account_id` | String |  | GTM Account ID. |
| `name` | String |  | Custom Template display name. |
| `container_id` | String |  | GTM Container ID. |
| `gallery_reference` | String |  | A reference to the Community Template Gallery entry. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `path` | String | GTM Custom Template's API relative path. |
| `template_id` | String | The Custom Template ID uniquely identifies the GTM custom template. |
| `fingerprint` | String | The fingerprint of the GTM Custom Template as computed at storage time. This value is recomputed whenever the template is modified. |
| `template_data` | String | The custom template in text format. |
| `workspace_id` | String | GTM Workspace ID. |
| `account_id` | String | GTM Account ID. |
| `name` | String | Custom Template display name. |
| `container_id` | String | GTM Container ID. |
| `gallery_reference` | String | A reference to the Community Template Gallery entry. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create template
template = provider.tagmanager_api.Template {
    parent = "value"  # GTM Workspace's API relative path.
}

# Access template outputs
template_id = template.id
template_tag_manager_url = template.tag_manager_url
template_path = template.path
template_template_id = template.template_id
template_fingerprint = template.fingerprint
template_template_data = template.template_data
template_workspace_id = template.workspace_id
template_account_id = template.account_id
template_name = template.name
template_container_id = template.container_id
template_gallery_reference = template.gallery_reference
```

---


### Variable

Creates a GTM Variable.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent_folder_id` | String |  | Parent folder id. |
| `parameter` | Vec<String> |  | The variable's parameters. |
| `format_value` | String |  | Option to convert a variable value to other value. |
| `name` | String |  | Variable display name. |
| `enabling_trigger_id` | Vec<String> |  | For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. |
| `account_id` | String |  | GTM Account ID. |
| `path` | String |  | GTM Variable's API relative path. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `container_id` | String |  | GTM Container ID. |
| `schedule_start_ms` | String |  | The start timestamp in milliseconds to schedule a variable. |
| `schedule_end_ms` | String |  | The end timestamp in milliseconds to schedule a variable. |
| `notes` | String |  | User notes on how to apply this variable in the container. |
| `disabling_trigger_id` | Vec<String> |  | For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. |
| `type` | String |  | GTM Variable Type. |
| `fingerprint` | String |  | The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified. |
| `variable_id` | String |  | The Variable ID uniquely identifies the GTM Variable. |
| `workspace_id` | String |  | GTM Workspace ID. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parent_folder_id` | String | Parent folder id. |
| `parameter` | Vec<String> | The variable's parameters. |
| `format_value` | String | Option to convert a variable value to other value. |
| `name` | String | Variable display name. |
| `enabling_trigger_id` | Vec<String> | For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. |
| `account_id` | String | GTM Account ID. |
| `path` | String | GTM Variable's API relative path. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `container_id` | String | GTM Container ID. |
| `schedule_start_ms` | String | The start timestamp in milliseconds to schedule a variable. |
| `schedule_end_ms` | String | The end timestamp in milliseconds to schedule a variable. |
| `notes` | String | User notes on how to apply this variable in the container. |
| `disabling_trigger_id` | Vec<String> | For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. |
| `type` | String | GTM Variable Type. |
| `fingerprint` | String | The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified. |
| `variable_id` | String | The Variable ID uniquely identifies the GTM Variable. |
| `workspace_id` | String | GTM Workspace ID. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create variable
variable = provider.tagmanager_api.Variable {
    parent = "value"  # GTM Workspace's API relative path.
}

# Access variable outputs
variable_id = variable.id
variable_parent_folder_id = variable.parent_folder_id
variable_parameter = variable.parameter
variable_format_value = variable.format_value
variable_name = variable.name
variable_enabling_trigger_id = variable.enabling_trigger_id
variable_account_id = variable.account_id
variable_path = variable.path
variable_tag_manager_url = variable.tag_manager_url
variable_container_id = variable.container_id
variable_schedule_start_ms = variable.schedule_start_ms
variable_schedule_end_ms = variable.schedule_end_ms
variable_notes = variable.notes
variable_disabling_trigger_id = variable.disabling_trigger_id
variable_type = variable.type
variable_fingerprint = variable.fingerprint
variable_variable_id = variable.variable_id
variable_workspace_id = variable.workspace_id
```

---


### User_permission

Creates a user's Account & Container access.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_access` | String |  | GTM Account access permissions. |
| `container_access` | Vec<String> |  | GTM Container access permissions. |
| `account_id` | String |  | The Account ID uniquely identifies the GTM Account. |
| `path` | String |  | GTM UserPermission's API relative path. |
| `email_address` | String |  | User's email address. |
| `parent` | String | ✅ | GTM Account's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_access` | String | GTM Account access permissions. |
| `container_access` | Vec<String> | GTM Container access permissions. |
| `account_id` | String | The Account ID uniquely identifies the GTM Account. |
| `path` | String | GTM UserPermission's API relative path. |
| `email_address` | String | User's email address. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_permission
user_permission = provider.tagmanager_api.User_permission {
    parent = "value"  # GTM Account's API relative path.
}

# Access user_permission outputs
user_permission_id = user_permission.id
user_permission_account_access = user_permission.account_access
user_permission_container_access = user_permission.container_access
user_permission_account_id = user_permission.account_id
user_permission_path = user_permission.path
user_permission_email_address = user_permission.email_address
```

---


### Account

Gets a GTM Account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String |  | The Account ID uniquely identifies the GTM Account. |
| `features` | String |  | Read-only Account feature set |
| `name` | String |  | Account display name. |
| `fingerprint` | String |  | The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified. |
| `path` | String |  | GTM Account's API relative path. |
| `share_data` | bool |  | Whether the account shares data anonymously with Google and others. This flag enables benchmarking by sharing your data in an anonymous form. Google will remove all identifiable information about your website, combine the data with hundreds of other anonymous sites and report aggregate trends in the benchmarking service. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `path` | String | ✅ | GTM Account's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | The Account ID uniquely identifies the GTM Account. |
| `features` | String | Read-only Account feature set |
| `name` | String | Account display name. |
| `fingerprint` | String | The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified. |
| `path` | String | GTM Account's API relative path. |
| `share_data` | bool | Whether the account shares data anonymously with Google and others. This flag enables benchmarking by sharing your data in an anonymous form. Google will remove all identifiable information about your website, combine the data with hundreds of other anonymous sites and report aggregate trends in the benchmarking service. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account outputs
account_id = account.id
account_account_id = account.account_id
account_features = account.features
account_name = account.name
account_fingerprint = account.fingerprint
account_path = account.path
account_share_data = account.share_data
account_tag_manager_url = account.tag_manager_url
```

---


### Version

Publishes a Container Version.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | String | ✅ | GTM ContainerVersion's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `zone` | Vec<String> | The zones in the container that this version was taken from. |
| `container_version_id` | String | The Container Version ID uniquely identifies the GTM Container Version. |
| `transformation` | Vec<String> | The transformations in the container that this version was taken from. |
| `path` | String | GTM Container Version's API relative path. |
| `container_id` | String | GTM Container ID. |
| `deleted` | bool | A value of true indicates this container version has been deleted. |
| `name` | String | Container version display name. |
| `variable` | Vec<String> | The variables in the container that this version was taken from. |
| `account_id` | String | GTM Account ID. |
| `custom_template` | Vec<String> | The custom templates in the container that this version was taken from. |
| `folder` | Vec<String> | The folders in the container that this version was taken from. |
| `gtag_config` | Vec<String> | The Google tag configs in the container that this version was taken from. |
| `built_in_variable` | Vec<String> | The built-in variables in the container that this version was taken from. |
| `description` | String | Container version description. |
| `fingerprint` | String | The fingerprint of the GTM Container Version as computed at storage time. This value is recomputed whenever the container version is modified. |
| `tag` | Vec<String> | The tags in the container that this version was taken from. |
| `trigger` | Vec<String> | The triggers in the container that this version was taken from. |
| `client` | Vec<String> | The clients in the container that this version was taken from. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `container` | String | The container that this version was taken from. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.tagmanager_api.Version {
    path = "value"  # GTM ContainerVersion's API relative path.
}

# Access version outputs
version_id = version.id
version_zone = version.zone
version_container_version_id = version.container_version_id
version_transformation = version.transformation
version_path = version.path
version_container_id = version.container_id
version_deleted = version.deleted
version_name = version.name
version_variable = version.variable
version_account_id = version.account_id
version_custom_template = version.custom_template
version_folder = version.folder
version_gtag_config = version.gtag_config
version_built_in_variable = version.built_in_variable
version_description = version.description
version_fingerprint = version.fingerprint
version_tag = version.tag
version_trigger = version.trigger
version_client = version.client
version_tag_manager_url = version.tag_manager_url
version_container = version.container
```

---


### Tag

Creates a GTM Tag.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `workspace_id` | String |  | GTM Workspace ID. |
| `firing_trigger_id` | Vec<String> |  | Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. |
| `account_id` | String |  | GTM Account ID. |
| `live_only` | bool |  | If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). |
| `setup_tag` | Vec<String> |  | The list of setup tags. Currently we only allow one. |
| `tag_id` | String |  | The Tag ID uniquely identifies the GTM Tag. |
| `name` | String |  | Tag display name. |
| `blocking_trigger_id` | Vec<String> |  | Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. |
| `notes` | String |  | User notes on how to apply this tag in the container. |
| `fingerprint` | String |  | The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified. |
| `container_id` | String |  | GTM Container ID. |
| `monitoring_metadata` | String |  | A map of key-value pairs of tag metadata to be included in the event data for tag monitoring. Notes: - This parameter must be type MAP. - Each parameter in the map are type TEMPLATE, however cannot contain variable references.  |
| `schedule_end_ms` | String |  | The end timestamp in milliseconds to schedule a tag. |
| `teardown_tag` | Vec<String> |  | The list of teardown tags. Currently we only allow one. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `path` | String |  | GTM Tag's API relative path. |
| `type` | String |  | GTM Tag Type. |
| `parameter` | Vec<String> |  | The tag's parameters. |
| `parent_folder_id` | String |  | Parent folder id. |
| `priority` | String |  | User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. |
| `schedule_start_ms` | String |  | The start timestamp in milliseconds to schedule a tag. |
| `consent_settings` | String |  | Consent settings of a tag. |
| `paused` | bool |  | Indicates whether the tag is paused, which prevents the tag from firing. |
| `tag_firing_option` | String |  | Option to fire this tag. |
| `monitoring_metadata_tag_name_key` | String |  | If non-empty, then the tag display name will be included in the monitoring metadata map using the key specified. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `workspace_id` | String | GTM Workspace ID. |
| `firing_trigger_id` | Vec<String> | Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. |
| `account_id` | String | GTM Account ID. |
| `live_only` | bool | If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). |
| `setup_tag` | Vec<String> | The list of setup tags. Currently we only allow one. |
| `tag_id` | String | The Tag ID uniquely identifies the GTM Tag. |
| `name` | String | Tag display name. |
| `blocking_trigger_id` | Vec<String> | Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. |
| `notes` | String | User notes on how to apply this tag in the container. |
| `fingerprint` | String | The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified. |
| `container_id` | String | GTM Container ID. |
| `monitoring_metadata` | String | A map of key-value pairs of tag metadata to be included in the event data for tag monitoring. Notes: - This parameter must be type MAP. - Each parameter in the map are type TEMPLATE, however cannot contain variable references.  |
| `schedule_end_ms` | String | The end timestamp in milliseconds to schedule a tag. |
| `teardown_tag` | Vec<String> | The list of teardown tags. Currently we only allow one. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `path` | String | GTM Tag's API relative path. |
| `type` | String | GTM Tag Type. |
| `parameter` | Vec<String> | The tag's parameters. |
| `parent_folder_id` | String | Parent folder id. |
| `priority` | String | User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. |
| `schedule_start_ms` | String | The start timestamp in milliseconds to schedule a tag. |
| `consent_settings` | String | Consent settings of a tag. |
| `paused` | bool | Indicates whether the tag is paused, which prevents the tag from firing. |
| `tag_firing_option` | String | Option to fire this tag. |
| `monitoring_metadata_tag_name_key` | String | If non-empty, then the tag display name will be included in the monitoring metadata map using the key specified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag
tag = provider.tagmanager_api.Tag {
    parent = "value"  # GTM Workspace's API relative path.
}

# Access tag outputs
tag_id = tag.id
tag_workspace_id = tag.workspace_id
tag_firing_trigger_id = tag.firing_trigger_id
tag_account_id = tag.account_id
tag_live_only = tag.live_only
tag_setup_tag = tag.setup_tag
tag_tag_id = tag.tag_id
tag_name = tag.name
tag_blocking_trigger_id = tag.blocking_trigger_id
tag_notes = tag.notes
tag_fingerprint = tag.fingerprint
tag_container_id = tag.container_id
tag_monitoring_metadata = tag.monitoring_metadata
tag_schedule_end_ms = tag.schedule_end_ms
tag_teardown_tag = tag.teardown_tag
tag_tag_manager_url = tag.tag_manager_url
tag_path = tag.path
tag_type = tag.type
tag_parameter = tag.parameter
tag_parent_folder_id = tag.parent_folder_id
tag_priority = tag.priority
tag_schedule_start_ms = tag.schedule_start_ms
tag_consent_settings = tag.consent_settings
tag_paused = tag.paused
tag_tag_firing_option = tag.tag_firing_option
tag_monitoring_metadata_tag_name_key = tag.monitoring_metadata_tag_name_key
```

---


### Version_header

Lists all Container Versions of a GTM Container.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `container_version_header` | Vec<String> | All container version headers of a GTM Container. |
| `next_page_token` | String | Continuation token for fetching the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access version_header outputs
version_header_id = version_header.id
version_header_container_version_header = version_header.container_version_header
version_header_next_page_token = version_header.next_page_token
```

---


### Transformation

Creates a GTM Transformation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Transformation display name. |
| `workspace_id` | String |  | GTM Workspace ID. |
| `fingerprint` | String |  | The fingerprint of the GTM Transformation as computed at storage time. This value is recomputed whenever the transformation is modified. |
| `transformation_id` | String |  | The Transformation ID uniquely identifies the GTM transformation. |
| `type` | String |  | Transformation type. |
| `account_id` | String |  | GTM Account ID. |
| `container_id` | String |  | GTM Container ID. |
| `notes` | String |  | User notes on how to apply this transformation in the container. |
| `parameter` | Vec<String> |  | The transformation's parameters. |
| `parent_folder_id` | String |  | Parent folder id. |
| `path` | String |  | GTM transformation's API relative path. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Transformation display name. |
| `workspace_id` | String | GTM Workspace ID. |
| `fingerprint` | String | The fingerprint of the GTM Transformation as computed at storage time. This value is recomputed whenever the transformation is modified. |
| `transformation_id` | String | The Transformation ID uniquely identifies the GTM transformation. |
| `type` | String | Transformation type. |
| `account_id` | String | GTM Account ID. |
| `container_id` | String | GTM Container ID. |
| `notes` | String | User notes on how to apply this transformation in the container. |
| `parameter` | Vec<String> | The transformation's parameters. |
| `parent_folder_id` | String | Parent folder id. |
| `path` | String | GTM transformation's API relative path. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create transformation
transformation = provider.tagmanager_api.Transformation {
    parent = "value"  # GTM Workspace's API relative path.
}

# Access transformation outputs
transformation_id = transformation.id
transformation_name = transformation.name
transformation_workspace_id = transformation.workspace_id
transformation_fingerprint = transformation.fingerprint
transformation_transformation_id = transformation.transformation_id
transformation_type = transformation.type
transformation_account_id = transformation.account_id
transformation_container_id = transformation.container_id
transformation_notes = transformation.notes
transformation_parameter = transformation.parameter
transformation_parent_folder_id = transformation.parent_folder_id
transformation_path = transformation.path
transformation_tag_manager_url = transformation.tag_manager_url
```

---


### Trigger

Creates a GTM Trigger.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `wait_for_tags` | String |  | Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. |
| `selector` | String |  | A click trigger CSS selector (i.e. "a", "button" etc.). Only valid for AMP Click trigger. |
| `type` | String |  | Defines the data layer event that causes this trigger. |
| `visible_percentage_max` | String |  | A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. |
| `unique_trigger_id` | String |  | Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. |
| `visible_percentage_min` | String |  | A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `trigger_id` | String |  | The Trigger ID uniquely identifies the GTM Trigger. |
| `interval` | String |  | Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. |
| `container_id` | String |  | GTM Container ID. |
| `custom_event_filter` | Vec<String> |  | Used in the case of custom event, which is fired iff all Conditions are true. |
| `total_time_min_milliseconds` | String |  | A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `notes` | String |  | User notes on how to apply this trigger in the container. |
| `vertical_scroll_percentage_list` | String |  | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. |
| `visibility_selector` | String |  | A visibility trigger CSS selector (i.e. "#id"). Only valid for AMP Visibility trigger. |
| `parameter` | Vec<String> |  | Additional parameters. |
| `interval_seconds` | String |  | Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. |
| `horizontal_scroll_percentage_list` | String |  | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. |
| `wait_for_tags_timeout` | String |  | How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. |
| `auto_event_filter` | Vec<String> |  | Used in the case of auto event tracking. |
| `limit` | String |  | Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. |
| `parent_folder_id` | String |  | Parent folder id. |
| `event_name` | String |  | Name of the GTM event that is fired. Only valid for Timer triggers. |
| `check_validation` | String |  | Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. |
| `continuous_time_min_milliseconds` | String |  | A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `workspace_id` | String |  | GTM Workspace ID. |
| `max_timer_length_seconds` | String |  | Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. |
| `name` | String |  | Trigger display name. |
| `filter` | Vec<String> |  | The trigger will only fire iff all Conditions are true. |
| `account_id` | String |  | GTM Account ID. |
| `fingerprint` | String |  | The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified. |
| `path` | String |  | GTM Trigger's API relative path. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `wait_for_tags` | String | Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. |
| `selector` | String | A click trigger CSS selector (i.e. "a", "button" etc.). Only valid for AMP Click trigger. |
| `type` | String | Defines the data layer event that causes this trigger. |
| `visible_percentage_max` | String | A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. |
| `unique_trigger_id` | String | Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. |
| `visible_percentage_min` | String | A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `trigger_id` | String | The Trigger ID uniquely identifies the GTM Trigger. |
| `interval` | String | Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. |
| `container_id` | String | GTM Container ID. |
| `custom_event_filter` | Vec<String> | Used in the case of custom event, which is fired iff all Conditions are true. |
| `total_time_min_milliseconds` | String | A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `notes` | String | User notes on how to apply this trigger in the container. |
| `vertical_scroll_percentage_list` | String | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. |
| `visibility_selector` | String | A visibility trigger CSS selector (i.e. "#id"). Only valid for AMP Visibility trigger. |
| `parameter` | Vec<String> | Additional parameters. |
| `interval_seconds` | String | Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. |
| `horizontal_scroll_percentage_list` | String | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. |
| `wait_for_tags_timeout` | String | How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. |
| `auto_event_filter` | Vec<String> | Used in the case of auto event tracking. |
| `limit` | String | Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. |
| `parent_folder_id` | String | Parent folder id. |
| `event_name` | String | Name of the GTM event that is fired. Only valid for Timer triggers. |
| `check_validation` | String | Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. |
| `continuous_time_min_milliseconds` | String | A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `workspace_id` | String | GTM Workspace ID. |
| `max_timer_length_seconds` | String | Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. |
| `name` | String | Trigger display name. |
| `filter` | Vec<String> | The trigger will only fire iff all Conditions are true. |
| `account_id` | String | GTM Account ID. |
| `fingerprint` | String | The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified. |
| `path` | String | GTM Trigger's API relative path. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trigger
trigger = provider.tagmanager_api.Trigger {
    parent = "value"  # GTM Workspace's API relative path.
}

# Access trigger outputs
trigger_id = trigger.id
trigger_wait_for_tags = trigger.wait_for_tags
trigger_selector = trigger.selector
trigger_type = trigger.type
trigger_visible_percentage_max = trigger.visible_percentage_max
trigger_unique_trigger_id = trigger.unique_trigger_id
trigger_visible_percentage_min = trigger.visible_percentage_min
trigger_tag_manager_url = trigger.tag_manager_url
trigger_trigger_id = trigger.trigger_id
trigger_interval = trigger.interval
trigger_container_id = trigger.container_id
trigger_custom_event_filter = trigger.custom_event_filter
trigger_total_time_min_milliseconds = trigger.total_time_min_milliseconds
trigger_notes = trigger.notes
trigger_vertical_scroll_percentage_list = trigger.vertical_scroll_percentage_list
trigger_visibility_selector = trigger.visibility_selector
trigger_parameter = trigger.parameter
trigger_interval_seconds = trigger.interval_seconds
trigger_horizontal_scroll_percentage_list = trigger.horizontal_scroll_percentage_list
trigger_wait_for_tags_timeout = trigger.wait_for_tags_timeout
trigger_auto_event_filter = trigger.auto_event_filter
trigger_limit = trigger.limit
trigger_parent_folder_id = trigger.parent_folder_id
trigger_event_name = trigger.event_name
trigger_check_validation = trigger.check_validation
trigger_continuous_time_min_milliseconds = trigger.continuous_time_min_milliseconds
trigger_workspace_id = trigger.workspace_id
trigger_max_timer_length_seconds = trigger.max_timer_length_seconds
trigger_name = trigger.name
trigger_filter = trigger.filter
trigger_account_id = trigger.account_id
trigger_fingerprint = trigger.fingerprint
trigger_path = trigger.path
```

---


### Zone

Creates a GTM Zone.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String |  | GTM Account ID. |
| `name` | String |  | Zone display name. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `fingerprint` | String |  | The fingerprint of the GTM Zone as computed at storage time. This value is recomputed whenever the zone is modified. |
| `path` | String |  | GTM Zone's API relative path. |
| `type_restriction` | String |  | This Zone's type restrictions. |
| `workspace_id` | String |  | GTM Workspace ID. |
| `child_container` | Vec<String> |  | Containers that are children of this Zone. |
| `zone_id` | String |  | The Zone ID uniquely identifies the GTM Zone. |
| `notes` | String |  | User notes on how to apply this zone in the container. |
| `boundary` | String |  | This Zone's boundary. |
| `container_id` | String |  | GTM Container ID. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | GTM Account ID. |
| `name` | String | Zone display name. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `fingerprint` | String | The fingerprint of the GTM Zone as computed at storage time. This value is recomputed whenever the zone is modified. |
| `path` | String | GTM Zone's API relative path. |
| `type_restriction` | String | This Zone's type restrictions. |
| `workspace_id` | String | GTM Workspace ID. |
| `child_container` | Vec<String> | Containers that are children of this Zone. |
| `zone_id` | String | The Zone ID uniquely identifies the GTM Zone. |
| `notes` | String | User notes on how to apply this zone in the container. |
| `boundary` | String | This Zone's boundary. |
| `container_id` | String | GTM Container ID. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create zone
zone = provider.tagmanager_api.Zone {
    parent = "value"  # GTM Workspace's API relative path.
}

# Access zone outputs
zone_id = zone.id
zone_account_id = zone.account_id
zone_name = zone.name
zone_tag_manager_url = zone.tag_manager_url
zone_fingerprint = zone.fingerprint
zone_path = zone.path
zone_type_restriction = zone.type_restriction
zone_workspace_id = zone.workspace_id
zone_child_container = zone.child_container
zone_zone_id = zone.zone_id
zone_notes = zone.notes
zone_boundary = zone.boundary
zone_container_id = zone.container_id
```

---


### Folder

Creates a GTM Folder.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `folder_id` | String |  | The Folder ID uniquely identifies the GTM Folder. |
| `workspace_id` | String |  | GTM Workspace ID. |
| `notes` | String |  | User notes on how to apply this folder in the container. |
| `name` | String |  | Folder display name. |
| `account_id` | String |  | GTM Account ID. |
| `path` | String |  | GTM Folder's API relative path. |
| `container_id` | String |  | GTM Container ID. |
| `fingerprint` | String |  | The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `folder_id` | String | The Folder ID uniquely identifies the GTM Folder. |
| `workspace_id` | String | GTM Workspace ID. |
| `notes` | String | User notes on how to apply this folder in the container. |
| `name` | String | Folder display name. |
| `account_id` | String | GTM Account ID. |
| `path` | String | GTM Folder's API relative path. |
| `container_id` | String | GTM Container ID. |
| `fingerprint` | String | The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create folder
folder = provider.tagmanager_api.Folder {
    parent = "value"  # GTM Workspace's API relative path.
}

# Access folder outputs
folder_id = folder.id
folder_folder_id = folder.folder_id
folder_workspace_id = folder.workspace_id
folder_notes = folder.notes
folder_name = folder.name
folder_account_id = folder.account_id
folder_path = folder.path
folder_container_id = folder.container_id
folder_fingerprint = folder.fingerprint
folder_tag_manager_url = folder.tag_manager_url
```

---


### Container

Creates a Container.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_ids` | Vec<String> |  | All Tag IDs that refer to this Container. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `tagging_server_urls` | Vec<String> |  | List of server-side container URLs for the Container. If multiple URLs are provided, all URL paths must match. |
| `name` | String |  | Container display name. |
| `usage_context` | Vec<String> |  | List of Usage Contexts for the Container. Valid values include: web, android, or ios. |
| `container_id` | String |  | The Container ID uniquely identifies the GTM Container. |
| `public_id` | String |  | Container Public ID. |
| `account_id` | String |  | GTM Account ID. |
| `fingerprint` | String |  | The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified. |
| `path` | String |  | GTM Container's API relative path. |
| `domain_name` | Vec<String> |  | List of domain names associated with the Container. |
| `notes` | String |  | Container Notes. |
| `features` | String |  | Read-only Container feature set. |
| `parent` | String | ✅ | GTM Account's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag_ids` | Vec<String> | All Tag IDs that refer to this Container. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `tagging_server_urls` | Vec<String> | List of server-side container URLs for the Container. If multiple URLs are provided, all URL paths must match. |
| `name` | String | Container display name. |
| `usage_context` | Vec<String> | List of Usage Contexts for the Container. Valid values include: web, android, or ios. |
| `container_id` | String | The Container ID uniquely identifies the GTM Container. |
| `public_id` | String | Container Public ID. |
| `account_id` | String | GTM Account ID. |
| `fingerprint` | String | The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified. |
| `path` | String | GTM Container's API relative path. |
| `domain_name` | Vec<String> | List of domain names associated with the Container. |
| `notes` | String | Container Notes. |
| `features` | String | Read-only Container feature set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create container
container = provider.tagmanager_api.Container {
    parent = "value"  # GTM Account's API relative path.
}

# Access container outputs
container_id = container.id
container_tag_ids = container.tag_ids
container_tag_manager_url = container.tag_manager_url
container_tagging_server_urls = container.tagging_server_urls
container_name = container.name
container_usage_context = container.usage_context
container_container_id = container.container_id
container_public_id = container.public_id
container_account_id = container.account_id
container_fingerprint = container.fingerprint
container_path = container.path
container_domain_name = container.domain_name
container_notes = container.notes
container_features = container.features
```

---


### Gtag_config

Creates a Google tag config.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fingerprint` | String |  | The fingerprint of the Google tag config as computed at storage time. This value is recomputed whenever the config is modified. |
| `gtag_config_id` | String |  | The ID uniquely identifies the Google tag config. |
| `path` | String |  | Google tag config's API relative path. |
| `account_id` | String |  | Google tag account ID. |
| `type` | String |  | Google tag config type. |
| `parameter` | Vec<String> |  | The Google tag config's parameters. |
| `workspace_id` | String |  | Google tag workspace ID. Only used by GTM containers. Set to 0 otherwise. |
| `container_id` | String |  | Google tag container ID. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `parent` | String | ✅ | Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fingerprint` | String | The fingerprint of the Google tag config as computed at storage time. This value is recomputed whenever the config is modified. |
| `gtag_config_id` | String | The ID uniquely identifies the Google tag config. |
| `path` | String | Google tag config's API relative path. |
| `account_id` | String | Google tag account ID. |
| `type` | String | Google tag config type. |
| `parameter` | Vec<String> | The Google tag config's parameters. |
| `workspace_id` | String | Google tag workspace ID. Only used by GTM containers. Set to 0 otherwise. |
| `container_id` | String | Google tag container ID. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gtag_config
gtag_config = provider.tagmanager_api.Gtag_config {
    parent = "value"  # Workspace's API relative path.
}

# Access gtag_config outputs
gtag_config_id = gtag_config.id
gtag_config_fingerprint = gtag_config.fingerprint
gtag_config_gtag_config_id = gtag_config.gtag_config_id
gtag_config_path = gtag_config.path
gtag_config_account_id = gtag_config.account_id
gtag_config_type = gtag_config.type
gtag_config_parameter = gtag_config.parameter
gtag_config_workspace_id = gtag_config.workspace_id
gtag_config_container_id = gtag_config.container_id
gtag_config_tag_manager_url = gtag_config.tag_manager_url
```

---


### Environment

Creates a GTM Environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The environment display name. Can be set or changed only on USER type environments. |
| `fingerprint` | String |  | The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `path` | String |  | GTM Environment's API relative path. |
| `environment_id` | String |  | GTM Environment ID uniquely identifies the GTM Environment. |
| `account_id` | String |  | GTM Account ID. |
| `description` | String |  | The environment description. Can be set or changed only on USER type environments. |
| `container_version_id` | String |  | Represents a link to a container version. |
| `authorization_code` | String |  | The environment authorization code. |
| `authorization_timestamp` | String |  | The last update time-stamp for the authorization code. |
| `type` | String |  | The type of this environment. |
| `workspace_id` | String |  | Represents a link to a quick preview of a workspace. |
| `url` | String |  | Default preview page url for the environment. |
| `container_id` | String |  | GTM Container ID. |
| `enable_debug` | bool |  | Whether or not to enable debug by default for the environment. |
| `parent` | String | ✅ | GTM Container's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The environment display name. Can be set or changed only on USER type environments. |
| `fingerprint` | String | The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `path` | String | GTM Environment's API relative path. |
| `environment_id` | String | GTM Environment ID uniquely identifies the GTM Environment. |
| `account_id` | String | GTM Account ID. |
| `description` | String | The environment description. Can be set or changed only on USER type environments. |
| `container_version_id` | String | Represents a link to a container version. |
| `authorization_code` | String | The environment authorization code. |
| `authorization_timestamp` | String | The last update time-stamp for the authorization code. |
| `type` | String | The type of this environment. |
| `workspace_id` | String | Represents a link to a quick preview of a workspace. |
| `url` | String | Default preview page url for the environment. |
| `container_id` | String | GTM Container ID. |
| `enable_debug` | bool | Whether or not to enable debug by default for the environment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create environment
environment = provider.tagmanager_api.Environment {
    parent = "value"  # GTM Container's API relative path.
}

# Access environment outputs
environment_id = environment.id
environment_name = environment.name
environment_fingerprint = environment.fingerprint
environment_tag_manager_url = environment.tag_manager_url
environment_path = environment.path
environment_environment_id = environment.environment_id
environment_account_id = environment.account_id
environment_description = environment.description
environment_container_version_id = environment.container_version_id
environment_authorization_code = environment.authorization_code
environment_authorization_timestamp = environment.authorization_timestamp
environment_type = environment.type
environment_workspace_id = environment.workspace_id
environment_url = environment.url
environment_container_id = environment.container_id
environment_enable_debug = environment.enable_debug
```

---


### Workspace

Creates a Workspace.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fingerprint` | String |  | The fingerprint of the GTM Workspace as computed at storage time. This value is recomputed whenever the workspace is modified. |
| `account_id` | String |  | GTM Account ID. |
| `container_id` | String |  | GTM Container ID. |
| `description` | String |  | Workspace description. |
| `path` | String |  | GTM Workspace's API relative path. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `name` | String |  | Workspace display name. |
| `workspace_id` | String |  | The Workspace ID uniquely identifies the GTM Workspace. |
| `parent` | String | ✅ | GTM parent Container's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fingerprint` | String | The fingerprint of the GTM Workspace as computed at storage time. This value is recomputed whenever the workspace is modified. |
| `account_id` | String | GTM Account ID. |
| `container_id` | String | GTM Container ID. |
| `description` | String | Workspace description. |
| `path` | String | GTM Workspace's API relative path. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `name` | String | Workspace display name. |
| `workspace_id` | String | The Workspace ID uniquely identifies the GTM Workspace. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create workspace
workspace = provider.tagmanager_api.Workspace {
    parent = "value"  # GTM parent Container's API relative path.
}

# Access workspace outputs
workspace_id = workspace.id
workspace_fingerprint = workspace.fingerprint
workspace_account_id = workspace.account_id
workspace_container_id = workspace.container_id
workspace_description = workspace.description
workspace_path = workspace.path
workspace_tag_manager_url = workspace.tag_manager_url
workspace_name = workspace.name
workspace_workspace_id = workspace.workspace_id
```

---


### Built_in_variable

Creates one or more GTM Built-In Variables.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `built_in_variable` | Vec<String> | All GTM BuiltInVariables of a GTM container. |
| `next_page_token` | String | Continuation token for fetching the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create built_in_variable
built_in_variable = provider.tagmanager_api.Built_in_variable {
    parent = "value"  # GTM Workspace's API relative path.
}

# Access built_in_variable outputs
built_in_variable_id = built_in_variable.id
built_in_variable_built_in_variable = built_in_variable.built_in_variable
built_in_variable_next_page_token = built_in_variable.next_page_token
```

---


### Client

Creates a GTM Client.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameter` | Vec<String> |  | The client's parameters. |
| `path` | String |  | GTM client's API relative path. |
| `parent_folder_id` | String |  | Parent folder id. |
| `notes` | String |  | User notes on how to apply this tag in the container. |
| `name` | String |  | Client display name. |
| `client_id` | String |  | The Client ID uniquely identifies the GTM client. |
| `container_id` | String |  | GTM Container ID. |
| `fingerprint` | String |  | The fingerprint of the GTM Client as computed at storage time. This value is recomputed whenever the client is modified. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `workspace_id` | String |  | GTM Workspace ID. |
| `account_id` | String |  | GTM Account ID. |
| `type` | String |  | Client type. |
| `priority` | i64 |  | Priority determines relative firing order. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameter` | Vec<String> | The client's parameters. |
| `path` | String | GTM client's API relative path. |
| `parent_folder_id` | String | Parent folder id. |
| `notes` | String | User notes on how to apply this tag in the container. |
| `name` | String | Client display name. |
| `client_id` | String | The Client ID uniquely identifies the GTM client. |
| `container_id` | String | GTM Container ID. |
| `fingerprint` | String | The fingerprint of the GTM Client as computed at storage time. This value is recomputed whenever the client is modified. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `workspace_id` | String | GTM Workspace ID. |
| `account_id` | String | GTM Account ID. |
| `type` | String | Client type. |
| `priority` | i64 | Priority determines relative firing order. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create client
client = provider.tagmanager_api.Client {
    parent = "value"  # GTM Workspace's API relative path.
}

# Access client outputs
client_id = client.id
client_parameter = client.parameter
client_path = client.path
client_parent_folder_id = client.parent_folder_id
client_notes = client.notes
client_name = client.name
client_client_id = client.client_id
client_container_id = client.container_id
client_fingerprint = client.fingerprint
client_tag_manager_url = client.tag_manager_url
client_workspace_id = client.workspace_id
client_account_id = client.account_id
client_type = client.type
client_priority = client.priority
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple tag resources
tag_0 = provider.tagmanager_api.Tag {
    container_id = "value-0"
    account_id = "value-0"
}
tag_1 = provider.tagmanager_api.Tag {
    container_id = "value-1"
    account_id = "value-1"
}
tag_2 = provider.tagmanager_api.Tag {
    container_id = "value-2"
    account_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    tag = provider.tagmanager_api.Tag {
        container_id = "production-value"
        account_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Tagmanager_api Documentation](https://cloud.google.com/tagmanager_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
