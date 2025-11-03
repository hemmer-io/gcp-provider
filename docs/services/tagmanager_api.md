# Tagmanager_api Service



**Resources**: 30

---

## Overview

The tagmanager_api service provides access to 30 resource types:

- [Entitie](#entitie) [R]
- [Move_folder](#move_folder) [U]
- [Variable](#variable) [CRUD]
- [Container](#container) [CRUD]
- [Folder](#folder) [CRUD]
- [Trigger](#trigger) [CRUD]
- [Environment](#environment) [CRUD]
- [Reauthorize_environment](#reauthorize_environment) [U]
- [Version](#version) [CRUD]
- [Permission](#permission) [CRUD]
- [Account](#account) [RU]
- [Tag](#tag) [CRUD]
- [Version](#version) [CRUD]
- [Folder](#folder) [CRUD]
- [Zone](#zone) [CRUD]
- [Variable](#variable) [CRUD]
- [Environment](#environment) [CRUD]
- [Transformation](#transformation) [CRUD]
- [Version_header](#version_header) [R]
- [Container](#container) [CRUD]
- [Gtag_config](#gtag_config) [CRUD]
- [Tag](#tag) [CRUD]
- [Client](#client) [CRUD]
- [Destination](#destination) [CR]
- [Template](#template) [CRUD]
- [Trigger](#trigger) [CRUD]
- [User_permission](#user_permission) [CRUD]
- [Account](#account) [RU]
- [Workspace](#workspace) [CRUD]
- [Built_in_variable](#built_in_variable) [CRD]

---

## Resources


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


### Move_folder



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String |  | GTM Account ID. |
| `folder_id` | String |  | The Folder ID uniquely identifies the GTM Folder. |
| `container_id` | String |  | GTM Container ID. |
| `fingerprint` | String |  | The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified. |
| `name` | String |  | Folder display name. |
| `container_id` | String | ✅ | The GTM Container ID. |
| `folder_id` | String | ✅ | The GTM Folder ID. |
| `account_id` | String | ✅ | The GTM Account ID. |



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


### Variable

Creates a GTM Variable.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | GTM Variable Type. |
| `parameter` | Vec<String> |  | The variable's parameters. |
| `variable_id` | String |  | The Variable ID uniquely identifies the GTM Variable. |
| `disabling_trigger_id` | Vec<String> |  | For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. |
| `container_id` | String |  | GTM Container ID. |
| `parent_folder_id` | String |  | Parent folder id. |
| `fingerprint` | String |  | The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified. |
| `schedule_start_ms` | String |  | The start timestamp in milliseconds to schedule a variable. |
| `enabling_trigger_id` | Vec<String> |  | For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. |
| `schedule_end_ms` | String |  | The end timestamp in milliseconds to schedule a variable. |
| `name` | String |  | Variable display name. |
| `notes` | String |  | User notes on how to apply this variable in the container. |
| `account_id` | String |  | GTM Account ID. |
| `container_id` | String | ✅ | The GTM Container ID. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | GTM Variable Type. |
| `parameter` | Vec<String> | The variable's parameters. |
| `variable_id` | String | The Variable ID uniquely identifies the GTM Variable. |
| `disabling_trigger_id` | Vec<String> | For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. |
| `container_id` | String | GTM Container ID. |
| `parent_folder_id` | String | Parent folder id. |
| `fingerprint` | String | The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified. |
| `schedule_start_ms` | String | The start timestamp in milliseconds to schedule a variable. |
| `enabling_trigger_id` | Vec<String> | For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. |
| `schedule_end_ms` | String | The end timestamp in milliseconds to schedule a variable. |
| `name` | String | Variable display name. |
| `notes` | String | User notes on how to apply this variable in the container. |
| `account_id` | String | GTM Account ID. |


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
    container_id = "value"  # The GTM Container ID.
    account_id = "value"  # The GTM Account ID.
}

# Access variable outputs
variable_id = variable.id
variable_type = variable.type
variable_parameter = variable.parameter
variable_variable_id = variable.variable_id
variable_disabling_trigger_id = variable.disabling_trigger_id
variable_container_id = variable.container_id
variable_parent_folder_id = variable.parent_folder_id
variable_fingerprint = variable.fingerprint
variable_schedule_start_ms = variable.schedule_start_ms
variable_enabling_trigger_id = variable.enabling_trigger_id
variable_schedule_end_ms = variable.schedule_end_ms
variable_name = variable.name
variable_notes = variable.notes
variable_account_id = variable.account_id
```

---


### Container

Creates a Container.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notes` | String |  | Container Notes. |
| `enabled_built_in_variable` | Vec<String> |  | List of enabled built-in variables. Valid values include: pageUrl, pageHostname, pagePath, referrer, event, clickElement, clickClasses, clickId, clickTarget, clickUrl, clickText, formElement, formClasses, formId, formTarget, formUrl, formText, errorMessage, errorUrl, errorLine, newHistoryFragment, oldHistoryFragment, newHistoryState, oldHistoryState, historySource, containerVersion, debugMode, randomNumber, containerId. |
| `time_zone_id` | String |  | Container Time Zone ID. |
| `public_id` | String |  | Container Public ID. |
| `name` | String |  | Container display name. |
| `account_id` | String |  | GTM Account ID. |
| `container_id` | String |  | The Container ID uniquely identifies the GTM Container. |
| `time_zone_country_id` | String |  | Container Country ID. |
| `fingerprint` | String |  | The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified. |
| `usage_context` | Vec<String> |  | List of Usage Contexts for the Container. Valid values include: web, android, ios. |
| `domain_name` | Vec<String> |  | Optional list of domain names associated with the Container. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notes` | String | Container Notes. |
| `enabled_built_in_variable` | Vec<String> | List of enabled built-in variables. Valid values include: pageUrl, pageHostname, pagePath, referrer, event, clickElement, clickClasses, clickId, clickTarget, clickUrl, clickText, formElement, formClasses, formId, formTarget, formUrl, formText, errorMessage, errorUrl, errorLine, newHistoryFragment, oldHistoryFragment, newHistoryState, oldHistoryState, historySource, containerVersion, debugMode, randomNumber, containerId. |
| `time_zone_id` | String | Container Time Zone ID. |
| `public_id` | String | Container Public ID. |
| `name` | String | Container display name. |
| `account_id` | String | GTM Account ID. |
| `container_id` | String | The Container ID uniquely identifies the GTM Container. |
| `time_zone_country_id` | String | Container Country ID. |
| `fingerprint` | String | The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified. |
| `usage_context` | Vec<String> | List of Usage Contexts for the Container. Valid values include: web, android, ios. |
| `domain_name` | Vec<String> | Optional list of domain names associated with the Container. |


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
container_notes = container.notes
container_enabled_built_in_variable = container.enabled_built_in_variable
container_time_zone_id = container.time_zone_id
container_public_id = container.public_id
container_name = container.name
container_account_id = container.account_id
container_container_id = container.container_id
container_time_zone_country_id = container.time_zone_country_id
container_fingerprint = container.fingerprint
container_usage_context = container.usage_context
container_domain_name = container.domain_name
```

---


### Folder

Creates a GTM Folder.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String |  | GTM Account ID. |
| `folder_id` | String |  | The Folder ID uniquely identifies the GTM Folder. |
| `container_id` | String |  | GTM Container ID. |
| `fingerprint` | String |  | The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified. |
| `name` | String |  | Folder display name. |
| `container_id` | String | ✅ | The GTM Container ID. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | GTM Account ID. |
| `folder_id` | String | The Folder ID uniquely identifies the GTM Folder. |
| `container_id` | String | GTM Container ID. |
| `fingerprint` | String | The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified. |
| `name` | String | Folder display name. |


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
    container_id = "value"  # The GTM Container ID.
    account_id = "value"  # The GTM Account ID.
}

# Access folder outputs
folder_id = folder.id
folder_account_id = folder.account_id
folder_folder_id = folder.folder_id
folder_container_id = folder.container_id
folder_fingerprint = folder.fingerprint
folder_name = folder.name
```

---


### Trigger

Creates a GTM Trigger.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `total_time_min_milliseconds` | String |  | A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `visibility_selector` | String |  | A visibility trigger CSS selector (i.e. "#id"). Only valid for AMP Visibility trigger. |
| `interval` | String |  | Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. |
| `fingerprint` | String |  | The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified. |
| `unique_trigger_id` | String |  | Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. |
| `interval_seconds` | String |  | Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. |
| `vertical_scroll_percentage_list` | String |  | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. |
| `wait_for_tags` | String |  | Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. |
| `check_validation` | String |  | Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. |
| `type` | String |  | Defines the data layer event that causes this trigger. |
| `filter` | Vec<String> |  | The trigger will only fire iff all Conditions are true. |
| `wait_for_tags_timeout` | String |  | How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. |
| `auto_event_filter` | Vec<String> |  | Used in the case of auto event tracking. |
| `horizontal_scroll_percentage_list` | String |  | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. |
| `limit` | String |  | Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. |
| `continuous_time_min_milliseconds` | String |  | A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `parameter` | Vec<String> |  | Additional parameters. |
| `trigger_id` | String |  | The Trigger ID uniquely identifies the GTM Trigger. |
| `event_name` | String |  | Name of the GTM event that is fired. Only valid for Timer triggers. |
| `max_timer_length_seconds` | String |  | Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. |
| `selector` | String |  | A click trigger CSS selector (i.e. "a", "button" etc.). Only valid for AMP Click trigger. |
| `container_id` | String |  | GTM Container ID. |
| `account_id` | String |  | GTM Account ID. |
| `parent_folder_id` | String |  | Parent folder id. |
| `custom_event_filter` | Vec<String> |  | Used in the case of custom event, which is fired iff all Conditions are true. |
| `name` | String |  | Trigger display name. |
| `visible_percentage_max` | String |  | A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. |
| `visible_percentage_min` | String |  | A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. |
| `container_id` | String | ✅ | The GTM Container ID. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_time_min_milliseconds` | String | A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `visibility_selector` | String | A visibility trigger CSS selector (i.e. "#id"). Only valid for AMP Visibility trigger. |
| `interval` | String | Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. |
| `fingerprint` | String | The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified. |
| `unique_trigger_id` | String | Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. |
| `interval_seconds` | String | Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. |
| `vertical_scroll_percentage_list` | String | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. |
| `wait_for_tags` | String | Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. |
| `check_validation` | String | Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. |
| `type` | String | Defines the data layer event that causes this trigger. |
| `filter` | Vec<String> | The trigger will only fire iff all Conditions are true. |
| `wait_for_tags_timeout` | String | How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. |
| `auto_event_filter` | Vec<String> | Used in the case of auto event tracking. |
| `horizontal_scroll_percentage_list` | String | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. |
| `limit` | String | Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. |
| `continuous_time_min_milliseconds` | String | A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `parameter` | Vec<String> | Additional parameters. |
| `trigger_id` | String | The Trigger ID uniquely identifies the GTM Trigger. |
| `event_name` | String | Name of the GTM event that is fired. Only valid for Timer triggers. |
| `max_timer_length_seconds` | String | Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. |
| `selector` | String | A click trigger CSS selector (i.e. "a", "button" etc.). Only valid for AMP Click trigger. |
| `container_id` | String | GTM Container ID. |
| `account_id` | String | GTM Account ID. |
| `parent_folder_id` | String | Parent folder id. |
| `custom_event_filter` | Vec<String> | Used in the case of custom event, which is fired iff all Conditions are true. |
| `name` | String | Trigger display name. |
| `visible_percentage_max` | String | A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. |
| `visible_percentage_min` | String | A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. |


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
    container_id = "value"  # The GTM Container ID.
    account_id = "value"  # The GTM Account ID.
}

# Access trigger outputs
trigger_id = trigger.id
trigger_total_time_min_milliseconds = trigger.total_time_min_milliseconds
trigger_visibility_selector = trigger.visibility_selector
trigger_interval = trigger.interval
trigger_fingerprint = trigger.fingerprint
trigger_unique_trigger_id = trigger.unique_trigger_id
trigger_interval_seconds = trigger.interval_seconds
trigger_vertical_scroll_percentage_list = trigger.vertical_scroll_percentage_list
trigger_wait_for_tags = trigger.wait_for_tags
trigger_check_validation = trigger.check_validation
trigger_type = trigger.type
trigger_filter = trigger.filter
trigger_wait_for_tags_timeout = trigger.wait_for_tags_timeout
trigger_auto_event_filter = trigger.auto_event_filter
trigger_horizontal_scroll_percentage_list = trigger.horizontal_scroll_percentage_list
trigger_limit = trigger.limit
trigger_continuous_time_min_milliseconds = trigger.continuous_time_min_milliseconds
trigger_parameter = trigger.parameter
trigger_trigger_id = trigger.trigger_id
trigger_event_name = trigger.event_name
trigger_max_timer_length_seconds = trigger.max_timer_length_seconds
trigger_selector = trigger.selector
trigger_container_id = trigger.container_id
trigger_account_id = trigger.account_id
trigger_parent_folder_id = trigger.parent_folder_id
trigger_custom_event_filter = trigger.custom_event_filter
trigger_name = trigger.name
trigger_visible_percentage_max = trigger.visible_percentage_max
trigger_visible_percentage_min = trigger.visible_percentage_min
```

---


### Environment

Creates a GTM Environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fingerprint` | String |  | The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified. |
| `name` | String |  | The environment display name. Can be set or changed only on USER type environments. |
| `description` | String |  | The environment description. Can be set or changed only on USER type environments. |
| `authorization_timestamp_ms` | String |  | The last update time-stamp for the authorization code. |
| `container_version_id` | String |  |  |
| `account_id` | String |  | GTM Account ID. |
| `authorization_code` | String |  | The environment authorization code. |
| `url` | String |  | Default preview page url for the environment. |
| `environment_id` | String |  | GTM Environment ID uniquely identifies the GTM Environment. |
| `container_id` | String |  | GTM Container ID. |
| `type` | String |  | The type of this environment. |
| `enable_debug` | bool |  | Whether or not to enable debug by default on for the environment. |
| `container_id` | String | ✅ | The GTM Container ID. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fingerprint` | String | The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified. |
| `name` | String | The environment display name. Can be set or changed only on USER type environments. |
| `description` | String | The environment description. Can be set or changed only on USER type environments. |
| `authorization_timestamp_ms` | String | The last update time-stamp for the authorization code. |
| `container_version_id` | String |  |
| `account_id` | String | GTM Account ID. |
| `authorization_code` | String | The environment authorization code. |
| `url` | String | Default preview page url for the environment. |
| `environment_id` | String | GTM Environment ID uniquely identifies the GTM Environment. |
| `container_id` | String | GTM Container ID. |
| `type` | String | The type of this environment. |
| `enable_debug` | bool | Whether or not to enable debug by default on for the environment. |


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
environment_fingerprint = environment.fingerprint
environment_name = environment.name
environment_description = environment.description
environment_authorization_timestamp_ms = environment.authorization_timestamp_ms
environment_container_version_id = environment.container_version_id
environment_account_id = environment.account_id
environment_authorization_code = environment.authorization_code
environment_url = environment.url
environment_environment_id = environment.environment_id
environment_container_id = environment.container_id
environment_type = environment.type
environment_enable_debug = environment.enable_debug
```

---


### Reauthorize_environment



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fingerprint` | String |  | The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified. |
| `name` | String |  | The environment display name. Can be set or changed only on USER type environments. |
| `description` | String |  | The environment description. Can be set or changed only on USER type environments. |
| `authorization_timestamp_ms` | String |  | The last update time-stamp for the authorization code. |
| `container_version_id` | String |  |  |
| `account_id` | String |  | GTM Account ID. |
| `authorization_code` | String |  | The environment authorization code. |
| `url` | String |  | Default preview page url for the environment. |
| `environment_id` | String |  | GTM Environment ID uniquely identifies the GTM Environment. |
| `container_id` | String |  | GTM Container ID. |
| `type` | String |  | The type of this environment. |
| `enable_debug` | bool |  | Whether or not to enable debug by default on for the environment. |
| `container_id` | String | ✅ | The GTM Container ID. |
| `account_id` | String | ✅ | The GTM Account ID. |
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
| `name` | String | Container version display name. |
| `container` | String | The container that this version was taken from. |
| `fingerprint` | String | The fingerprint of the GTM Container Version as computed at storage time. This value is recomputed whenever the container version is modified. |
| `tag` | Vec<String> | The tags in the container that this version was taken from. |
| `trigger` | Vec<String> | The triggers in the container that this version was taken from. |
| `container_version_id` | String | The Container Version ID uniquely identifies the GTM Container Version. |
| `variable` | Vec<String> | The variables in the container that this version was taken from. |
| `account_id` | String | GTM Account ID. |
| `notes` | String | User notes on how to apply this container version in the container. |
| `container_id` | String | GTM Container ID. |
| `folder` | Vec<String> | The folders in the container that this version was taken from. |
| `deleted` | bool | A value of true indicates this container version has been deleted. |


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
version_name = version.name
version_container = version.container
version_fingerprint = version.fingerprint
version_tag = version.tag
version_trigger = version.trigger
version_container_version_id = version.container_version_id
version_variable = version.variable
version_account_id = version.account_id
version_notes = version.notes
version_container_id = version.container_id
version_folder = version.folder
version_deleted = version.deleted
```

---


### Permission

Creates a user's Account & Container Permissions.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String |  | GTM Account ID. |
| `container_access` | Vec<String> |  | GTM Container access permissions. |
| `account_access` | String |  | GTM Account access permissions. |
| `permission_id` | String |  | Account Permission ID. |
| `email_address` | String |  | User's email address. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | GTM Account ID. |
| `container_access` | Vec<String> | GTM Container access permissions. |
| `account_access` | String | GTM Account access permissions. |
| `permission_id` | String | Account Permission ID. |
| `email_address` | String | User's email address. |


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
permission_account_id = permission.account_id
permission_container_access = permission.container_access
permission_account_access = permission.account_access
permission_permission_id = permission.permission_id
permission_email_address = permission.email_address
```

---


### Account

Gets a GTM Account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String |  | The Account ID uniquely identifies the GTM Account. |
| `fingerprint` | String |  | The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified. |
| `name` | String |  | Account display name. |
| `share_data` | bool |  | Whether the account shares data anonymously with Google and others. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | The Account ID uniquely identifies the GTM Account. |
| `fingerprint` | String | The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified. |
| `name` | String | Account display name. |
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
account_account_id = account.account_id
account_fingerprint = account.fingerprint
account_name = account.name
account_share_data = account.share_data
```

---


### Tag

Creates a GTM Tag.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `priority` | String |  | User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. |
| `blocking_trigger_id` | Vec<String> |  | Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. |
| `live_only` | bool |  | If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). |
| `paused` | bool |  | True if the tag is paused. |
| `parent_folder_id` | String |  | Parent folder id. |
| `schedule_end_ms` | String |  | The end timestamp in milliseconds to schedule a tag. |
| `type` | String |  | GTM Tag Type. |
| `schedule_start_ms` | String |  | The start timestamp in milliseconds to schedule a tag. |
| `parameter` | Vec<String> |  | The tag's parameters. |
| `fingerprint` | String |  | The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified. |
| `tag_firing_option` | String |  | Option to fire this tag. |
| `notes` | String |  | User notes on how to apply this tag in the container. |
| `container_id` | String |  | GTM Container ID. |
| `tag_id` | String |  | The Tag ID uniquely identifies the GTM Tag. |
| `teardown_tag` | Vec<String> |  | The list of teardown tags. Currently we only allow one. |
| `name` | String |  | Tag display name. |
| `account_id` | String |  | GTM Account ID. |
| `setup_tag` | Vec<String> |  | The list of setup tags. Currently we only allow one. |
| `firing_trigger_id` | Vec<String> |  | Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. |
| `container_id` | String | ✅ | The GTM Container ID. |
| `account_id` | String | ✅ | The GTM Account ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `priority` | String | User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. |
| `blocking_trigger_id` | Vec<String> | Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. |
| `live_only` | bool | If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). |
| `paused` | bool | True if the tag is paused. |
| `parent_folder_id` | String | Parent folder id. |
| `schedule_end_ms` | String | The end timestamp in milliseconds to schedule a tag. |
| `type` | String | GTM Tag Type. |
| `schedule_start_ms` | String | The start timestamp in milliseconds to schedule a tag. |
| `parameter` | Vec<String> | The tag's parameters. |
| `fingerprint` | String | The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified. |
| `tag_firing_option` | String | Option to fire this tag. |
| `notes` | String | User notes on how to apply this tag in the container. |
| `container_id` | String | GTM Container ID. |
| `tag_id` | String | The Tag ID uniquely identifies the GTM Tag. |
| `teardown_tag` | Vec<String> | The list of teardown tags. Currently we only allow one. |
| `name` | String | Tag display name. |
| `account_id` | String | GTM Account ID. |
| `setup_tag` | Vec<String> | The list of setup tags. Currently we only allow one. |
| `firing_trigger_id` | Vec<String> | Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. |


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
tag_priority = tag.priority
tag_blocking_trigger_id = tag.blocking_trigger_id
tag_live_only = tag.live_only
tag_paused = tag.paused
tag_parent_folder_id = tag.parent_folder_id
tag_schedule_end_ms = tag.schedule_end_ms
tag_type = tag.type
tag_schedule_start_ms = tag.schedule_start_ms
tag_parameter = tag.parameter
tag_fingerprint = tag.fingerprint
tag_tag_firing_option = tag.tag_firing_option
tag_notes = tag.notes
tag_container_id = tag.container_id
tag_tag_id = tag.tag_id
tag_teardown_tag = tag.teardown_tag
tag_name = tag.name
tag_account_id = tag.account_id
tag_setup_tag = tag.setup_tag
tag_firing_trigger_id = tag.firing_trigger_id
```

---


### Version

Undeletes a Container Version.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | String | ✅ | GTM ContainerVersion's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `container_id` | String | GTM Container ID. |
| `transformation` | Vec<String> | The transformations in the container that this version was taken from. |
| `built_in_variable` | Vec<String> | The built-in variables in the container that this version was taken from. |
| `folder` | Vec<String> | The folders in the container that this version was taken from. |
| `client` | Vec<String> | The clients in the container that this version was taken from. |
| `trigger` | Vec<String> | The triggers in the container that this version was taken from. |
| `zone` | Vec<String> | The zones in the container that this version was taken from. |
| `path` | String | GTM Container Version's API relative path. |
| `custom_template` | Vec<String> | The custom templates in the container that this version was taken from. |
| `gtag_config` | Vec<String> | The Google tag configs in the container that this version was taken from. |
| `name` | String | Container version display name. |
| `description` | String | Container version description. |
| `container` | String | The container that this version was taken from. |
| `account_id` | String | GTM Account ID. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `container_version_id` | String | The Container Version ID uniquely identifies the GTM Container Version. |
| `fingerprint` | String | The fingerprint of the GTM Container Version as computed at storage time. This value is recomputed whenever the container version is modified. |
| `tag` | Vec<String> | The tags in the container that this version was taken from. |
| `variable` | Vec<String> | The variables in the container that this version was taken from. |
| `deleted` | bool | A value of true indicates this container version has been deleted. |


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
version_container_id = version.container_id
version_transformation = version.transformation
version_built_in_variable = version.built_in_variable
version_folder = version.folder
version_client = version.client
version_trigger = version.trigger
version_zone = version.zone
version_path = version.path
version_custom_template = version.custom_template
version_gtag_config = version.gtag_config
version_name = version.name
version_description = version.description
version_container = version.container
version_account_id = version.account_id
version_tag_manager_url = version.tag_manager_url
version_container_version_id = version.container_version_id
version_fingerprint = version.fingerprint
version_tag = version.tag
version_variable = version.variable
version_deleted = version.deleted
```

---


### Folder

Creates a GTM Folder.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notes` | String |  | User notes on how to apply this folder in the container. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `workspace_id` | String |  | GTM Workspace ID. |
| `fingerprint` | String |  | The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified. |
| `name` | String |  | Folder display name. |
| `container_id` | String |  | GTM Container ID. |
| `account_id` | String |  | GTM Account ID. |
| `path` | String |  | GTM Folder's API relative path. |
| `folder_id` | String |  | The Folder ID uniquely identifies the GTM Folder. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `notes` | String | User notes on how to apply this folder in the container. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `workspace_id` | String | GTM Workspace ID. |
| `fingerprint` | String | The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified. |
| `name` | String | Folder display name. |
| `container_id` | String | GTM Container ID. |
| `account_id` | String | GTM Account ID. |
| `path` | String | GTM Folder's API relative path. |
| `folder_id` | String | The Folder ID uniquely identifies the GTM Folder. |


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
folder_notes = folder.notes
folder_tag_manager_url = folder.tag_manager_url
folder_workspace_id = folder.workspace_id
folder_fingerprint = folder.fingerprint
folder_name = folder.name
folder_container_id = folder.container_id
folder_account_id = folder.account_id
folder_path = folder.path
folder_folder_id = folder.folder_id
```

---


### Zone

Creates a GTM Zone.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Zone display name. |
| `fingerprint` | String |  | The fingerprint of the GTM Zone as computed at storage time. This value is recomputed whenever the zone is modified. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `child_container` | Vec<String> |  | Containers that are children of this Zone. |
| `notes` | String |  | User notes on how to apply this zone in the container. |
| `container_id` | String |  | GTM Container ID. |
| `boundary` | String |  | This Zone's boundary. |
| `path` | String |  | GTM Zone's API relative path. |
| `workspace_id` | String |  | GTM Workspace ID. |
| `zone_id` | String |  | The Zone ID uniquely identifies the GTM Zone. |
| `account_id` | String |  | GTM Account ID. |
| `type_restriction` | String |  | This Zone's type restrictions. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Zone display name. |
| `fingerprint` | String | The fingerprint of the GTM Zone as computed at storage time. This value is recomputed whenever the zone is modified. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `child_container` | Vec<String> | Containers that are children of this Zone. |
| `notes` | String | User notes on how to apply this zone in the container. |
| `container_id` | String | GTM Container ID. |
| `boundary` | String | This Zone's boundary. |
| `path` | String | GTM Zone's API relative path. |
| `workspace_id` | String | GTM Workspace ID. |
| `zone_id` | String | The Zone ID uniquely identifies the GTM Zone. |
| `account_id` | String | GTM Account ID. |
| `type_restriction` | String | This Zone's type restrictions. |


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
zone_name = zone.name
zone_fingerprint = zone.fingerprint
zone_tag_manager_url = zone.tag_manager_url
zone_child_container = zone.child_container
zone_notes = zone.notes
zone_container_id = zone.container_id
zone_boundary = zone.boundary
zone_path = zone.path
zone_workspace_id = zone.workspace_id
zone_zone_id = zone.zone_id
zone_account_id = zone.account_id
zone_type_restriction = zone.type_restriction
```

---


### Variable

Creates a GTM Variable.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | GTM Variable Type. |
| `container_id` | String |  | GTM Container ID. |
| `parent_folder_id` | String |  | Parent folder id. |
| `variable_id` | String |  | The Variable ID uniquely identifies the GTM Variable. |
| `workspace_id` | String |  | GTM Workspace ID. |
| `enabling_trigger_id` | Vec<String> |  | For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. |
| `name` | String |  | Variable display name. |
| `disabling_trigger_id` | Vec<String> |  | For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. |
| `notes` | String |  | User notes on how to apply this variable in the container. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `format_value` | String |  | Option to convert a variable value to other value. |
| `parameter` | Vec<String> |  | The variable's parameters. |
| `account_id` | String |  | GTM Account ID. |
| `schedule_end_ms` | String |  | The end timestamp in milliseconds to schedule a variable. |
| `path` | String |  | GTM Variable's API relative path. |
| `schedule_start_ms` | String |  | The start timestamp in milliseconds to schedule a variable. |
| `fingerprint` | String |  | The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | GTM Variable Type. |
| `container_id` | String | GTM Container ID. |
| `parent_folder_id` | String | Parent folder id. |
| `variable_id` | String | The Variable ID uniquely identifies the GTM Variable. |
| `workspace_id` | String | GTM Workspace ID. |
| `enabling_trigger_id` | Vec<String> | For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. |
| `name` | String | Variable display name. |
| `disabling_trigger_id` | Vec<String> | For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. |
| `notes` | String | User notes on how to apply this variable in the container. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `format_value` | String | Option to convert a variable value to other value. |
| `parameter` | Vec<String> | The variable's parameters. |
| `account_id` | String | GTM Account ID. |
| `schedule_end_ms` | String | The end timestamp in milliseconds to schedule a variable. |
| `path` | String | GTM Variable's API relative path. |
| `schedule_start_ms` | String | The start timestamp in milliseconds to schedule a variable. |
| `fingerprint` | String | The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified. |


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
variable_type = variable.type
variable_container_id = variable.container_id
variable_parent_folder_id = variable.parent_folder_id
variable_variable_id = variable.variable_id
variable_workspace_id = variable.workspace_id
variable_enabling_trigger_id = variable.enabling_trigger_id
variable_name = variable.name
variable_disabling_trigger_id = variable.disabling_trigger_id
variable_notes = variable.notes
variable_tag_manager_url = variable.tag_manager_url
variable_format_value = variable.format_value
variable_parameter = variable.parameter
variable_account_id = variable.account_id
variable_schedule_end_ms = variable.schedule_end_ms
variable_path = variable.path
variable_schedule_start_ms = variable.schedule_start_ms
variable_fingerprint = variable.fingerprint
```

---


### Environment

Creates a GTM Environment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `authorization_timestamp` | String |  | The last update time-stamp for the authorization code. |
| `container_version_id` | String |  | Represents a link to a container version. |
| `url` | String |  | Default preview page url for the environment. |
| `path` | String |  | GTM Environment's API relative path. |
| `enable_debug` | bool |  | Whether or not to enable debug by default for the environment. |
| `account_id` | String |  | GTM Account ID. |
| `authorization_code` | String |  | The environment authorization code. |
| `container_id` | String |  | GTM Container ID. |
| `description` | String |  | The environment description. Can be set or changed only on USER type environments. |
| `name` | String |  | The environment display name. Can be set or changed only on USER type environments. |
| `type` | String |  | The type of this environment. |
| `workspace_id` | String |  | Represents a link to a quick preview of a workspace. |
| `environment_id` | String |  | GTM Environment ID uniquely identifies the GTM Environment. |
| `fingerprint` | String |  | The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified. |
| `parent` | String | ✅ | GTM Container's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `authorization_timestamp` | String | The last update time-stamp for the authorization code. |
| `container_version_id` | String | Represents a link to a container version. |
| `url` | String | Default preview page url for the environment. |
| `path` | String | GTM Environment's API relative path. |
| `enable_debug` | bool | Whether or not to enable debug by default for the environment. |
| `account_id` | String | GTM Account ID. |
| `authorization_code` | String | The environment authorization code. |
| `container_id` | String | GTM Container ID. |
| `description` | String | The environment description. Can be set or changed only on USER type environments. |
| `name` | String | The environment display name. Can be set or changed only on USER type environments. |
| `type` | String | The type of this environment. |
| `workspace_id` | String | Represents a link to a quick preview of a workspace. |
| `environment_id` | String | GTM Environment ID uniquely identifies the GTM Environment. |
| `fingerprint` | String | The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified. |


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
environment_tag_manager_url = environment.tag_manager_url
environment_authorization_timestamp = environment.authorization_timestamp
environment_container_version_id = environment.container_version_id
environment_url = environment.url
environment_path = environment.path
environment_enable_debug = environment.enable_debug
environment_account_id = environment.account_id
environment_authorization_code = environment.authorization_code
environment_container_id = environment.container_id
environment_description = environment.description
environment_name = environment.name
environment_type = environment.type
environment_workspace_id = environment.workspace_id
environment_environment_id = environment.environment_id
environment_fingerprint = environment.fingerprint
```

---


### Transformation

Creates a GTM Transformation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `workspace_id` | String |  | GTM Workspace ID. |
| `parameter` | Vec<String> |  | The transformation's parameters. |
| `container_id` | String |  | GTM Container ID. |
| `name` | String |  | Transformation display name. |
| `fingerprint` | String |  | The fingerprint of the GTM Transformation as computed at storage time. This value is recomputed whenever the transformation is modified. |
| `path` | String |  | GTM transformation's API relative path. |
| `account_id` | String |  | GTM Account ID. |
| `transformation_id` | String |  | The Transformation ID uniquely identifies the GTM transformation. |
| `type` | String |  | Transformation type. |
| `notes` | String |  | User notes on how to apply this transformation in the container. |
| `parent_folder_id` | String |  | Parent folder id. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `workspace_id` | String | GTM Workspace ID. |
| `parameter` | Vec<String> | The transformation's parameters. |
| `container_id` | String | GTM Container ID. |
| `name` | String | Transformation display name. |
| `fingerprint` | String | The fingerprint of the GTM Transformation as computed at storage time. This value is recomputed whenever the transformation is modified. |
| `path` | String | GTM transformation's API relative path. |
| `account_id` | String | GTM Account ID. |
| `transformation_id` | String | The Transformation ID uniquely identifies the GTM transformation. |
| `type` | String | Transformation type. |
| `notes` | String | User notes on how to apply this transformation in the container. |
| `parent_folder_id` | String | Parent folder id. |


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
transformation_tag_manager_url = transformation.tag_manager_url
transformation_workspace_id = transformation.workspace_id
transformation_parameter = transformation.parameter
transformation_container_id = transformation.container_id
transformation_name = transformation.name
transformation_fingerprint = transformation.fingerprint
transformation_path = transformation.path
transformation_account_id = transformation.account_id
transformation_transformation_id = transformation.transformation_id
transformation_type = transformation.type
transformation_notes = transformation.notes
transformation_parent_folder_id = transformation.parent_folder_id
```

---


### Version_header

Gets the latest container version header

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Container version display name. |
| `num_gtag_configs` | String | Number of Google tag configs in the container version. |
| `container_version_id` | String | The Container Version ID uniquely identifies the GTM Container Version. |
| `num_custom_templates` | String | Number of custom templates in the container version. |
| `account_id` | String | GTM Account ID. |
| `num_tags` | String | Number of tags in the container version. |
| `num_transformations` | String | Number of transformations in the container version. |
| `num_triggers` | String | Number of triggers in the container version. |
| `num_zones` | String | Number of zones in the container version. |
| `path` | String | GTM Container Version's API relative path. |
| `num_variables` | String | Number of variables in the container version. |
| `num_clients` | String | Number of clients in the container version. |
| `container_id` | String | GTM Container ID. |
| `deleted` | bool | A value of true indicates this container version has been deleted. |


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
version_header_name = version_header.name
version_header_num_gtag_configs = version_header.num_gtag_configs
version_header_container_version_id = version_header.container_version_id
version_header_num_custom_templates = version_header.num_custom_templates
version_header_account_id = version_header.account_id
version_header_num_tags = version_header.num_tags
version_header_num_transformations = version_header.num_transformations
version_header_num_triggers = version_header.num_triggers
version_header_num_zones = version_header.num_zones
version_header_path = version_header.path
version_header_num_variables = version_header.num_variables
version_header_num_clients = version_header.num_clients
version_header_container_id = version_header.container_id
version_header_deleted = version_header.deleted
```

---


### Container

Creates a Container.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fingerprint` | String |  | The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified. |
| `tagging_server_urls` | Vec<String> |  | List of server-side container URLs for the Container. If multiple URLs are provided, all URL paths must match. |
| `account_id` | String |  | GTM Account ID. |
| `public_id` | String |  | Container Public ID. |
| `usage_context` | Vec<String> |  | List of Usage Contexts for the Container. Valid values include: web, android, or ios. |
| `notes` | String |  | Container Notes. |
| `features` | String |  | Read-only Container feature set. |
| `container_id` | String |  | The Container ID uniquely identifies the GTM Container. |
| `tag_ids` | Vec<String> |  | All Tag IDs that refer to this Container. |
| `name` | String |  | Container display name. |
| `path` | String |  | GTM Container's API relative path. |
| `domain_name` | Vec<String> |  | List of domain names associated with the Container. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `parent` | String | ✅ | GTM Account's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fingerprint` | String | The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified. |
| `tagging_server_urls` | Vec<String> | List of server-side container URLs for the Container. If multiple URLs are provided, all URL paths must match. |
| `account_id` | String | GTM Account ID. |
| `public_id` | String | Container Public ID. |
| `usage_context` | Vec<String> | List of Usage Contexts for the Container. Valid values include: web, android, or ios. |
| `notes` | String | Container Notes. |
| `features` | String | Read-only Container feature set. |
| `container_id` | String | The Container ID uniquely identifies the GTM Container. |
| `tag_ids` | Vec<String> | All Tag IDs that refer to this Container. |
| `name` | String | Container display name. |
| `path` | String | GTM Container's API relative path. |
| `domain_name` | Vec<String> | List of domain names associated with the Container. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |


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
container_fingerprint = container.fingerprint
container_tagging_server_urls = container.tagging_server_urls
container_account_id = container.account_id
container_public_id = container.public_id
container_usage_context = container.usage_context
container_notes = container.notes
container_features = container.features
container_container_id = container.container_id
container_tag_ids = container.tag_ids
container_name = container.name
container_path = container.path
container_domain_name = container.domain_name
container_tag_manager_url = container.tag_manager_url
```

---


### Gtag_config

Creates a Google tag config.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_id` | String |  | Google tag account ID. |
| `fingerprint` | String |  | The fingerprint of the Google tag config as computed at storage time. This value is recomputed whenever the config is modified. |
| `path` | String |  | Google tag config's API relative path. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `type` | String |  | Google tag config type. |
| `workspace_id` | String |  | Google tag workspace ID. Only used by GTM containers. Set to 0 otherwise. |
| `container_id` | String |  | Google tag container ID. |
| `gtag_config_id` | String |  | The ID uniquely identifies the Google tag config. |
| `parameter` | Vec<String> |  | The Google tag config's parameters. |
| `parent` | String | ✅ | Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_id` | String | Google tag account ID. |
| `fingerprint` | String | The fingerprint of the Google tag config as computed at storage time. This value is recomputed whenever the config is modified. |
| `path` | String | Google tag config's API relative path. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `type` | String | Google tag config type. |
| `workspace_id` | String | Google tag workspace ID. Only used by GTM containers. Set to 0 otherwise. |
| `container_id` | String | Google tag container ID. |
| `gtag_config_id` | String | The ID uniquely identifies the Google tag config. |
| `parameter` | Vec<String> | The Google tag config's parameters. |


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
gtag_config_account_id = gtag_config.account_id
gtag_config_fingerprint = gtag_config.fingerprint
gtag_config_path = gtag_config.path
gtag_config_tag_manager_url = gtag_config.tag_manager_url
gtag_config_type = gtag_config.type
gtag_config_workspace_id = gtag_config.workspace_id
gtag_config_container_id = gtag_config.container_id
gtag_config_gtag_config_id = gtag_config.gtag_config_id
gtag_config_parameter = gtag_config.parameter
```

---


### Tag

Creates a GTM Tag.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `firing_trigger_id` | Vec<String> |  | Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. |
| `schedule_start_ms` | String |  | The start timestamp in milliseconds to schedule a tag. |
| `consent_settings` | String |  | Consent settings of a tag. |
| `parameter` | Vec<String> |  | The tag's parameters. |
| `setup_tag` | Vec<String> |  | The list of setup tags. Currently we only allow one. |
| `parent_folder_id` | String |  | Parent folder id. |
| `account_id` | String |  | GTM Account ID. |
| `priority` | String |  | User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. |
| `tag_firing_option` | String |  | Option to fire this tag. |
| `type` | String |  | GTM Tag Type. |
| `live_only` | bool |  | If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). |
| `container_id` | String |  | GTM Container ID. |
| `fingerprint` | String |  | The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified. |
| `blocking_trigger_id` | Vec<String> |  | Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. |
| `paused` | bool |  | Indicates whether the tag is paused, which prevents the tag from firing. |
| `tag_id` | String |  | The Tag ID uniquely identifies the GTM Tag. |
| `monitoring_metadata_tag_name_key` | String |  | If non-empty, then the tag display name will be included in the monitoring metadata map using the key specified. |
| `path` | String |  | GTM Tag's API relative path. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `notes` | String |  | User notes on how to apply this tag in the container. |
| `teardown_tag` | Vec<String> |  | The list of teardown tags. Currently we only allow one. |
| `workspace_id` | String |  | GTM Workspace ID. |
| `schedule_end_ms` | String |  | The end timestamp in milliseconds to schedule a tag. |
| `monitoring_metadata` | String |  | A map of key-value pairs of tag metadata to be included in the event data for tag monitoring. Notes: - This parameter must be type MAP. - Each parameter in the map are type TEMPLATE, however cannot contain variable references.  |
| `name` | String |  | Tag display name. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `firing_trigger_id` | Vec<String> | Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. |
| `schedule_start_ms` | String | The start timestamp in milliseconds to schedule a tag. |
| `consent_settings` | String | Consent settings of a tag. |
| `parameter` | Vec<String> | The tag's parameters. |
| `setup_tag` | Vec<String> | The list of setup tags. Currently we only allow one. |
| `parent_folder_id` | String | Parent folder id. |
| `account_id` | String | GTM Account ID. |
| `priority` | String | User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. |
| `tag_firing_option` | String | Option to fire this tag. |
| `type` | String | GTM Tag Type. |
| `live_only` | bool | If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). |
| `container_id` | String | GTM Container ID. |
| `fingerprint` | String | The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified. |
| `blocking_trigger_id` | Vec<String> | Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. |
| `paused` | bool | Indicates whether the tag is paused, which prevents the tag from firing. |
| `tag_id` | String | The Tag ID uniquely identifies the GTM Tag. |
| `monitoring_metadata_tag_name_key` | String | If non-empty, then the tag display name will be included in the monitoring metadata map using the key specified. |
| `path` | String | GTM Tag's API relative path. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `notes` | String | User notes on how to apply this tag in the container. |
| `teardown_tag` | Vec<String> | The list of teardown tags. Currently we only allow one. |
| `workspace_id` | String | GTM Workspace ID. |
| `schedule_end_ms` | String | The end timestamp in milliseconds to schedule a tag. |
| `monitoring_metadata` | String | A map of key-value pairs of tag metadata to be included in the event data for tag monitoring. Notes: - This parameter must be type MAP. - Each parameter in the map are type TEMPLATE, however cannot contain variable references.  |
| `name` | String | Tag display name. |


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
tag_firing_trigger_id = tag.firing_trigger_id
tag_schedule_start_ms = tag.schedule_start_ms
tag_consent_settings = tag.consent_settings
tag_parameter = tag.parameter
tag_setup_tag = tag.setup_tag
tag_parent_folder_id = tag.parent_folder_id
tag_account_id = tag.account_id
tag_priority = tag.priority
tag_tag_firing_option = tag.tag_firing_option
tag_type = tag.type
tag_live_only = tag.live_only
tag_container_id = tag.container_id
tag_fingerprint = tag.fingerprint
tag_blocking_trigger_id = tag.blocking_trigger_id
tag_paused = tag.paused
tag_tag_id = tag.tag_id
tag_monitoring_metadata_tag_name_key = tag.monitoring_metadata_tag_name_key
tag_path = tag.path
tag_tag_manager_url = tag.tag_manager_url
tag_notes = tag.notes
tag_teardown_tag = tag.teardown_tag
tag_workspace_id = tag.workspace_id
tag_schedule_end_ms = tag.schedule_end_ms
tag_monitoring_metadata = tag.monitoring_metadata
tag_name = tag.name
```

---


### Client

Creates a GTM Client.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `client_id` | String |  | The Client ID uniquely identifies the GTM client. |
| `parameter` | Vec<String> |  | The client's parameters. |
| `name` | String |  | Client display name. |
| `notes` | String |  | User notes on how to apply this tag in the container. |
| `type` | String |  | Client type. |
| `fingerprint` | String |  | The fingerprint of the GTM Client as computed at storage time. This value is recomputed whenever the client is modified. |
| `workspace_id` | String |  | GTM Workspace ID. |
| `account_id` | String |  | GTM Account ID. |
| `container_id` | String |  | GTM Container ID. |
| `priority` | i64 |  | Priority determines relative firing order. |
| `parent_folder_id` | String |  | Parent folder id. |
| `path` | String |  | GTM client's API relative path. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `client_id` | String | The Client ID uniquely identifies the GTM client. |
| `parameter` | Vec<String> | The client's parameters. |
| `name` | String | Client display name. |
| `notes` | String | User notes on how to apply this tag in the container. |
| `type` | String | Client type. |
| `fingerprint` | String | The fingerprint of the GTM Client as computed at storage time. This value is recomputed whenever the client is modified. |
| `workspace_id` | String | GTM Workspace ID. |
| `account_id` | String | GTM Account ID. |
| `container_id` | String | GTM Container ID. |
| `priority` | i64 | Priority determines relative firing order. |
| `parent_folder_id` | String | Parent folder id. |
| `path` | String | GTM client's API relative path. |


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
client_tag_manager_url = client.tag_manager_url
client_client_id = client.client_id
client_parameter = client.parameter
client_name = client.name
client_notes = client.notes
client_type = client.type
client_fingerprint = client.fingerprint
client_workspace_id = client.workspace_id
client_account_id = client.account_id
client_container_id = client.container_id
client_priority = client.priority
client_parent_folder_id = client.parent_folder_id
client_path = client.path
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
| `account_id` | String | GTM Account ID. |
| `path` | String | Destination's API relative path. |
| `destination_id` | String | Destination ID. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI. |
| `fingerprint` | String | The fingerprint of the Google Tag Destination as computed at storage time. This value is recomputed whenever the destination is modified. |
| `container_id` | String | GTM Container ID. |
| `destination_link_id` | String | The Destination link ID uniquely identifies the Destination. |
| `name` | String | Destination display name. |


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
destination_account_id = destination.account_id
destination_path = destination.path
destination_destination_id = destination.destination_id
destination_tag_manager_url = destination.tag_manager_url
destination_fingerprint = destination.fingerprint
destination_container_id = destination.container_id
destination_destination_link_id = destination.destination_link_id
destination_name = destination.name
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
| `workspace_id` | String |  | GTM Workspace ID. |
| `gallery_reference` | String |  | A reference to the Community Template Gallery entry. |
| `fingerprint` | String |  | The fingerprint of the GTM Custom Template as computed at storage time. This value is recomputed whenever the template is modified. |
| `name` | String |  | Custom Template display name. |
| `account_id` | String |  | GTM Account ID. |
| `template_data` | String |  | The custom template in text format. |
| `container_id` | String |  | GTM Container ID. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `path` | String | GTM Custom Template's API relative path. |
| `template_id` | String | The Custom Template ID uniquely identifies the GTM custom template. |
| `workspace_id` | String | GTM Workspace ID. |
| `gallery_reference` | String | A reference to the Community Template Gallery entry. |
| `fingerprint` | String | The fingerprint of the GTM Custom Template as computed at storage time. This value is recomputed whenever the template is modified. |
| `name` | String | Custom Template display name. |
| `account_id` | String | GTM Account ID. |
| `template_data` | String | The custom template in text format. |
| `container_id` | String | GTM Container ID. |


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
template_workspace_id = template.workspace_id
template_gallery_reference = template.gallery_reference
template_fingerprint = template.fingerprint
template_name = template.name
template_account_id = template.account_id
template_template_data = template.template_data
template_container_id = template.container_id
```

---


### Trigger

Creates a GTM Trigger.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Defines the data layer event that causes this trigger. |
| `total_time_min_milliseconds` | String |  | A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `visible_percentage_max` | String |  | A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. |
| `limit` | String |  | Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. |
| `visibility_selector` | String |  | A visibility trigger CSS selector (i.e. "#id"). Only valid for AMP Visibility trigger. |
| `wait_for_tags_timeout` | String |  | How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. |
| `account_id` | String |  | GTM Account ID. |
| `selector` | String |  | A click trigger CSS selector (i.e. "a", "button" etc.). Only valid for AMP Click trigger. |
| `interval_seconds` | String |  | Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. |
| `continuous_time_min_milliseconds` | String |  | A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `workspace_id` | String |  | GTM Workspace ID. |
| `interval` | String |  | Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. |
| `visible_percentage_min` | String |  | A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. |
| `wait_for_tags` | String |  | Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. |
| `name` | String |  | Trigger display name. |
| `auto_event_filter` | Vec<String> |  | Used in the case of auto event tracking. |
| `check_validation` | String |  | Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. |
| `container_id` | String |  | GTM Container ID. |
| `unique_trigger_id` | String |  | Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. |
| `event_name` | String |  | Name of the GTM event that is fired. Only valid for Timer triggers. |
| `filter` | Vec<String> |  | The trigger will only fire iff all Conditions are true. |
| `fingerprint` | String |  | The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified. |
| `max_timer_length_seconds` | String |  | Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. |
| `notes` | String |  | User notes on how to apply this trigger in the container. |
| `trigger_id` | String |  | The Trigger ID uniquely identifies the GTM Trigger. |
| `vertical_scroll_percentage_list` | String |  | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. |
| `parameter` | Vec<String> |  | Additional parameters. |
| `parent_folder_id` | String |  | Parent folder id. |
| `path` | String |  | GTM Trigger's API relative path. |
| `horizontal_scroll_percentage_list` | String |  | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. |
| `custom_event_filter` | Vec<String> |  | Used in the case of custom event, which is fired iff all Conditions are true. |
| `parent` | String | ✅ | GTM Workspace's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Defines the data layer event that causes this trigger. |
| `total_time_min_milliseconds` | String | A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `visible_percentage_max` | String | A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. |
| `limit` | String | Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. |
| `visibility_selector` | String | A visibility trigger CSS selector (i.e. "#id"). Only valid for AMP Visibility trigger. |
| `wait_for_tags_timeout` | String | How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. |
| `account_id` | String | GTM Account ID. |
| `selector` | String | A click trigger CSS selector (i.e. "a", "button" etc.). Only valid for AMP Click trigger. |
| `interval_seconds` | String | Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. |
| `continuous_time_min_milliseconds` | String | A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `workspace_id` | String | GTM Workspace ID. |
| `interval` | String | Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. |
| `visible_percentage_min` | String | A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. |
| `wait_for_tags` | String | Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. |
| `name` | String | Trigger display name. |
| `auto_event_filter` | Vec<String> | Used in the case of auto event tracking. |
| `check_validation` | String | Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. |
| `container_id` | String | GTM Container ID. |
| `unique_trigger_id` | String | Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. |
| `event_name` | String | Name of the GTM event that is fired. Only valid for Timer triggers. |
| `filter` | Vec<String> | The trigger will only fire iff all Conditions are true. |
| `fingerprint` | String | The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified. |
| `max_timer_length_seconds` | String | Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. |
| `notes` | String | User notes on how to apply this trigger in the container. |
| `trigger_id` | String | The Trigger ID uniquely identifies the GTM Trigger. |
| `vertical_scroll_percentage_list` | String | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. |
| `parameter` | Vec<String> | Additional parameters. |
| `parent_folder_id` | String | Parent folder id. |
| `path` | String | GTM Trigger's API relative path. |
| `horizontal_scroll_percentage_list` | String | List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. |
| `custom_event_filter` | Vec<String> | Used in the case of custom event, which is fired iff all Conditions are true. |


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
trigger_type = trigger.type
trigger_total_time_min_milliseconds = trigger.total_time_min_milliseconds
trigger_visible_percentage_max = trigger.visible_percentage_max
trigger_limit = trigger.limit
trigger_visibility_selector = trigger.visibility_selector
trigger_wait_for_tags_timeout = trigger.wait_for_tags_timeout
trigger_account_id = trigger.account_id
trigger_selector = trigger.selector
trigger_interval_seconds = trigger.interval_seconds
trigger_continuous_time_min_milliseconds = trigger.continuous_time_min_milliseconds
trigger_tag_manager_url = trigger.tag_manager_url
trigger_workspace_id = trigger.workspace_id
trigger_interval = trigger.interval
trigger_visible_percentage_min = trigger.visible_percentage_min
trigger_wait_for_tags = trigger.wait_for_tags
trigger_name = trigger.name
trigger_auto_event_filter = trigger.auto_event_filter
trigger_check_validation = trigger.check_validation
trigger_container_id = trigger.container_id
trigger_unique_trigger_id = trigger.unique_trigger_id
trigger_event_name = trigger.event_name
trigger_filter = trigger.filter
trigger_fingerprint = trigger.fingerprint
trigger_max_timer_length_seconds = trigger.max_timer_length_seconds
trigger_notes = trigger.notes
trigger_trigger_id = trigger.trigger_id
trigger_vertical_scroll_percentage_list = trigger.vertical_scroll_percentage_list
trigger_parameter = trigger.parameter
trigger_parent_folder_id = trigger.parent_folder_id
trigger_path = trigger.path
trigger_horizontal_scroll_percentage_list = trigger.horizontal_scroll_percentage_list
trigger_custom_event_filter = trigger.custom_event_filter
```

---


### User_permission

Creates a user's Account & Container access.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_access` | String |  | GTM Account access permissions. |
| `email_address` | String |  | User's email address. |
| `account_id` | String |  | The Account ID uniquely identifies the GTM Account. |
| `container_access` | Vec<String> |  | GTM Container access permissions. |
| `path` | String |  | GTM UserPermission's API relative path. |
| `parent` | String | ✅ | GTM Account's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_access` | String | GTM Account access permissions. |
| `email_address` | String | User's email address. |
| `account_id` | String | The Account ID uniquely identifies the GTM Account. |
| `container_access` | Vec<String> | GTM Container access permissions. |
| `path` | String | GTM UserPermission's API relative path. |


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
user_permission_email_address = user_permission.email_address
user_permission_account_id = user_permission.account_id
user_permission_container_access = user_permission.container_access
user_permission_path = user_permission.path
```

---


### Account

Gets a GTM Account.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `features` | String |  | Read-only Account feature set |
| `account_id` | String |  | The Account ID uniquely identifies the GTM Account. |
| `path` | String |  | GTM Account's API relative path. |
| `share_data` | bool |  | Whether the account shares data anonymously with Google and others. This flag enables benchmarking by sharing your data in an anonymous form. Google will remove all identifiable information about your website, combine the data with hundreds of other anonymous sites and report aggregate trends in the benchmarking service. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `fingerprint` | String |  | The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified. |
| `name` | String |  | Account display name. |
| `path` | String | ✅ | GTM Account's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `features` | String | Read-only Account feature set |
| `account_id` | String | The Account ID uniquely identifies the GTM Account. |
| `path` | String | GTM Account's API relative path. |
| `share_data` | bool | Whether the account shares data anonymously with Google and others. This flag enables benchmarking by sharing your data in an anonymous form. Google will remove all identifiable information about your website, combine the data with hundreds of other anonymous sites and report aggregate trends in the benchmarking service. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `fingerprint` | String | The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified. |
| `name` | String | Account display name. |


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
account_features = account.features
account_account_id = account.account_id
account_path = account.path
account_share_data = account.share_data
account_tag_manager_url = account.tag_manager_url
account_fingerprint = account.fingerprint
account_name = account.name
```

---


### Workspace

Creates a Workspace.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | String |  | GTM Workspace's API relative path. |
| `account_id` | String |  | GTM Account ID. |
| `description` | String |  | Workspace description. |
| `container_id` | String |  | GTM Container ID. |
| `workspace_id` | String |  | The Workspace ID uniquely identifies the GTM Workspace. |
| `fingerprint` | String |  | The fingerprint of the GTM Workspace as computed at storage time. This value is recomputed whenever the workspace is modified. |
| `tag_manager_url` | String |  | Auto generated link to the tag manager UI |
| `name` | String |  | Workspace display name. |
| `parent` | String | ✅ | GTM parent Container's API relative path. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `path` | String | GTM Workspace's API relative path. |
| `account_id` | String | GTM Account ID. |
| `description` | String | Workspace description. |
| `container_id` | String | GTM Container ID. |
| `workspace_id` | String | The Workspace ID uniquely identifies the GTM Workspace. |
| `fingerprint` | String | The fingerprint of the GTM Workspace as computed at storage time. This value is recomputed whenever the workspace is modified. |
| `tag_manager_url` | String | Auto generated link to the tag manager UI |
| `name` | String | Workspace display name. |


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
workspace_path = workspace.path
workspace_account_id = workspace.account_id
workspace_description = workspace.description
workspace_container_id = workspace.container_id
workspace_workspace_id = workspace.workspace_id
workspace_fingerprint = workspace.fingerprint
workspace_tag_manager_url = workspace.tag_manager_url
workspace_name = workspace.name
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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple entitie resources
entitie_0 = provider.tagmanager_api.Entitie {
}
entitie_1 = provider.tagmanager_api.Entitie {
}
entitie_2 = provider.tagmanager_api.Entitie {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    entitie = provider.tagmanager_api.Entitie {
    }
```

---

## Related Documentation

- [GCP Tagmanager_api Documentation](https://cloud.google.com/tagmanager_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
