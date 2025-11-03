# Tasks_api Service



**Resources**: 2

---

## Overview

The tasks_api service provides access to 2 resource types:

- [Tasklist](#tasklist) [CRUD]
- [Task](#task) [CRUD]

---

## Resources


### Tasklist

Creates a new task list and adds it to the authenticated user's task lists. A user can have up to 2000 lists at a time.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Output only. Type of the resource. This is always "tasks#taskList". |
| `id` | String |  | Task list identifier. |
| `etag` | String |  | ETag of the resource. |
| `updated` | String |  | Output only. Last modification time of the task list (as a RFC 3339 timestamp). |
| `self_link` | String |  | Output only. URL pointing to this task list. Used to retrieve, update, or delete this task list. |
| `title` | String |  | Title of the task list. Maximum length allowed: 1024 characters. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Output only. Type of the resource. This is always "tasks#taskList". |
| `id` | String | Task list identifier. |
| `etag` | String | ETag of the resource. |
| `updated` | String | Output only. Last modification time of the task list (as a RFC 3339 timestamp). |
| `self_link` | String | Output only. URL pointing to this task list. Used to retrieve, update, or delete this task list. |
| `title` | String | Title of the task list. Maximum length allowed: 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tasklist
tasklist = provider.tasks_api.Tasklist {
}

# Access tasklist outputs
tasklist_id = tasklist.id
tasklist_kind = tasklist.kind
tasklist_id = tasklist.id
tasklist_etag = tasklist.etag
tasklist_updated = tasklist.updated
tasklist_self_link = tasklist.self_link
tasklist_title = tasklist.title
```

---


### Task

Creates a new task on the specified task list. Tasks assigned from Docs or Chat Spaces cannot be inserted from Tasks Public API; they can only be created by assigning them from Docs or Chat Spaces. A user can have up to 20,000 non-hidden tasks per list and up to 100,000 tasks in total at a time.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `assignment_info` | String |  | Output only. Context information for assigned tasks. A task can be assigned to a user, currently possible from surfaces like Docs and Chat Spaces. This field is populated for tasks assigned to the current user and identifies where the task was assigned from. This field is read-only. |
| `deleted` | bool |  | Flag indicating whether the task has been deleted. For assigned tasks this field is read-only. They can only be deleted by calling tasks.delete, in which case both the assigned task and the original task (in Docs or Chat Spaces) are deleted. To delete the assigned task only, navigate to the assignment surface and unassign the task from there. The default is False. |
| `due` | String |  | Due date of the task (as a RFC 3339 timestamp). Optional. The due date only records date information; the time portion of the timestamp is discarded when setting the due date. It isn't possible to read or write the time that a task is due via the API. |
| `self_link` | String |  | Output only. URL pointing to this task. Used to retrieve, update, or delete this task. |
| `web_view_link` | String |  | Output only. An absolute link to the task in the Google Tasks Web UI. |
| `id` | String |  | Task identifier. |
| `position` | String |  | Output only. String indicating the position of the task among its sibling tasks under the same parent task or at the top level. If this string is greater than another task's corresponding position string according to lexicographical ordering, the task is positioned after the other task under the same parent task (or at the top level). Use the "move" method to move the task to another position. |
| `notes` | String |  | Notes describing the task. Tasks assigned from Google Docs cannot have notes. Optional. Maximum length allowed: 8192 characters. |
| `links` | Vec<String> |  | Output only. Collection of links. This collection is read-only. |
| `hidden` | bool |  | Flag indicating whether the task is hidden. This is the case if the task had been marked completed when the task list was last cleared. The default is False. This field is read-only. |
| `updated` | String |  | Output only. Last modification time of the task (as a RFC 3339 timestamp). |
| `parent` | String |  | Output only. Parent task identifier. This field is omitted if it is a top-level task. Use the "move" method to move the task under a different parent or to the top level. A parent task can never be an assigned task (from Chat Spaces, Docs). This field is read-only. |
| `completed` | String |  | Completion date of the task (as a RFC 3339 timestamp). This field is omitted if the task has not been completed. |
| `etag` | String |  | ETag of the resource. |
| `kind` | String |  | Output only. Type of the resource. This is always "tasks#task". |
| `status` | String |  | Status of the task. This is either "needsAction" or "completed". |
| `title` | String |  | Title of the task. Maximum length allowed: 1024 characters. |
| `tasklist` | String | ✅ | Task list identifier. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `assignment_info` | String | Output only. Context information for assigned tasks. A task can be assigned to a user, currently possible from surfaces like Docs and Chat Spaces. This field is populated for tasks assigned to the current user and identifies where the task was assigned from. This field is read-only. |
| `deleted` | bool | Flag indicating whether the task has been deleted. For assigned tasks this field is read-only. They can only be deleted by calling tasks.delete, in which case both the assigned task and the original task (in Docs or Chat Spaces) are deleted. To delete the assigned task only, navigate to the assignment surface and unassign the task from there. The default is False. |
| `due` | String | Due date of the task (as a RFC 3339 timestamp). Optional. The due date only records date information; the time portion of the timestamp is discarded when setting the due date. It isn't possible to read or write the time that a task is due via the API. |
| `self_link` | String | Output only. URL pointing to this task. Used to retrieve, update, or delete this task. |
| `web_view_link` | String | Output only. An absolute link to the task in the Google Tasks Web UI. |
| `id` | String | Task identifier. |
| `position` | String | Output only. String indicating the position of the task among its sibling tasks under the same parent task or at the top level. If this string is greater than another task's corresponding position string according to lexicographical ordering, the task is positioned after the other task under the same parent task (or at the top level). Use the "move" method to move the task to another position. |
| `notes` | String | Notes describing the task. Tasks assigned from Google Docs cannot have notes. Optional. Maximum length allowed: 8192 characters. |
| `links` | Vec<String> | Output only. Collection of links. This collection is read-only. |
| `hidden` | bool | Flag indicating whether the task is hidden. This is the case if the task had been marked completed when the task list was last cleared. The default is False. This field is read-only. |
| `updated` | String | Output only. Last modification time of the task (as a RFC 3339 timestamp). |
| `parent` | String | Output only. Parent task identifier. This field is omitted if it is a top-level task. Use the "move" method to move the task under a different parent or to the top level. A parent task can never be an assigned task (from Chat Spaces, Docs). This field is read-only. |
| `completed` | String | Completion date of the task (as a RFC 3339 timestamp). This field is omitted if the task has not been completed. |
| `etag` | String | ETag of the resource. |
| `kind` | String | Output only. Type of the resource. This is always "tasks#task". |
| `status` | String | Status of the task. This is either "needsAction" or "completed". |
| `title` | String | Title of the task. Maximum length allowed: 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create task
task = provider.tasks_api.Task {
    tasklist = "value"  # Task list identifier.
}

# Access task outputs
task_id = task.id
task_assignment_info = task.assignment_info
task_deleted = task.deleted
task_due = task.due
task_self_link = task.self_link
task_web_view_link = task.web_view_link
task_id = task.id
task_position = task.position
task_notes = task.notes
task_links = task.links
task_hidden = task.hidden
task_updated = task.updated
task_parent = task.parent
task_completed = task.completed
task_etag = task.etag
task_kind = task.kind
task_status = task.status
task_title = task.title
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple tasklist resources
tasklist_0 = provider.tasks_api.Tasklist {
}
tasklist_1 = provider.tasks_api.Tasklist {
}
tasklist_2 = provider.tasks_api.Tasklist {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    tasklist = provider.tasks_api.Tasklist {
    }
```

---

## Related Documentation

- [GCP Tasks_api Documentation](https://cloud.google.com/tasks_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
