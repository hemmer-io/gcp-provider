# Clouddebugger_api Service



**Resources**: 2

---

## Overview

The clouddebugger_api service provides access to 2 resource types:

- [Breakpoint](#breakpoint) [CRUD]
- [Debuggee](#debuggee) [CR]

---

## Resources


### Breakpoint

Sets the breakpoint to the debuggee.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `variable_table` | Vec<String> |  | The `variable_table` exists to aid with computation, memory and network traffic optimization. It enables storing a variable once and reference it from multiple variables, including variables stored in the `variable_table` itself. For example, the same `this` object, which may appear at many levels of the stack, can have all of its data stored once in this table. The stack frame variables then would hold only a reference to it. The variable `var_table_index` field is an index into this repeated field. The stored objects are nameless and get their name from the referencing variable. The effective variable is a merge of the referencing variable and the referenced variable. |
| `evaluated_expressions` | Vec<String> |  | Values of evaluated expressions at breakpoint time. The evaluated expressions appear in exactly the same order they are listed in the `expressions` field. The `name` field holds the original expression text, the `value` or `members` field holds the result of the evaluated expression. If the expression cannot be evaluated, the `status` inside the `Variable` will indicate an error and contain the error text. |
| `expressions` | Vec<String> |  | List of read-only expressions to evaluate at the breakpoint location. The expressions are composed using expressions in the programming language at the source location. If the breakpoint action is `LOG`, the evaluated expressions are included in log statements. |
| `stack_frames` | Vec<String> |  | The stack at breakpoint time, where stack_frames[0] represents the most recently entered function. |
| `state` | String |  | The current state of the breakpoint. |
| `log_level` | String |  | Indicates the severity of the log. Only relevant when action is `LOG`. |
| `condition` | String |  | Condition that triggers the breakpoint. The condition is a compound boolean expression composed using expressions in a programming language at the source location. |
| `canary_expire_time` | String |  | The deadline for the breakpoint to stay in CANARY_ACTIVE state. The value is meaningless when the breakpoint is not in CANARY_ACTIVE state. |
| `location` | String |  | Breakpoint source location. |
| `is_final_state` | bool |  | When true, indicates that this is a final result and the breakpoint state will not change from here on. |
| `status` | String |  | Breakpoint status. The status includes an error flag and a human readable message. This field is usually unset. The message can be either informational or an error message. Regardless, clients should always display the text message back to the user. Error status indicates complete failure of the breakpoint. Example (non-final state): `Still loading symbols...` Examples (final state): * `Invalid line number` referring to location * `Field f not found in class C` referring to condition |
| `create_time` | String |  | Time this breakpoint was created by the server in seconds resolution. |
| `labels` | HashMap<String, String> |  | A set of custom breakpoint properties, populated by the agent, to be displayed to the user. |
| `final_time` | String |  | Time this breakpoint was finalized as seen by the server in seconds resolution. |
| `action` | String |  | Action that the agent should perform when the code at the breakpoint location is hit. |
| `id` | String |  | Breakpoint identifier, unique in the scope of the debuggee. |
| `log_message_format` | String |  | Only relevant when action is `LOG`. Defines the message to log when the breakpoint hits. The message may include parameter placeholders `$0`, `$1`, etc. These placeholders are replaced with the evaluated value of the appropriate expression. Expressions not referenced in `log_message_format` are not logged. Example: `Message received, id = $0, count = $1` with `expressions` = `[ message.id, message.count ]`. |
| `user_email` | String |  | E-mail address of the user that created this breakpoint |
| `debuggee_id` | String | ✅ | Required. ID of the debuggee where the breakpoint is to be set. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `breakpoint` | String | Complete breakpoint state. The fields `id` and `location` are guaranteed to be set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create breakpoint
breakpoint = provider.clouddebugger_api.Breakpoint {
    debuggee_id = "value"  # Required. ID of the debuggee where the breakpoint is to be set.
}

# Access breakpoint outputs
breakpoint_id = breakpoint.id
breakpoint_breakpoint = breakpoint.breakpoint
```

---


### Debuggee

Registers the debuggee with the controller service. All agents attached to the same application must call this method with exactly the same request content to get back the same stable `debuggee_id`. Agents should call this method again whenever `google.rpc.Code.NOT_FOUND` is returned from any controller method. This protocol allows the controller service to disable debuggees, recover from data loss, or change the `debuggee_id` format. Agents must handle `debuggee_id` value changing upon re-registration.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `debuggee` | String |  | Required. Debuggee information to register. The fields `project`, `uniquifier`, `description` and `agent_version` of the debuggee must be set. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `debuggees` | Vec<String> | List of debuggees accessible to the calling user. The fields `debuggee.id` and `description` are guaranteed to be set. The `description` field is a human readable field provided by agents and can be displayed to users. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create debuggee
debuggee = provider.clouddebugger_api.Debuggee {
}

# Access debuggee outputs
debuggee_id = debuggee.id
debuggee_debuggees = debuggee.debuggees
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple breakpoint resources
breakpoint_0 = provider.clouddebugger_api.Breakpoint {
    debuggee_id = "value-0"
}
breakpoint_1 = provider.clouddebugger_api.Breakpoint {
    debuggee_id = "value-1"
}
breakpoint_2 = provider.clouddebugger_api.Breakpoint {
    debuggee_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    breakpoint = provider.clouddebugger_api.Breakpoint {
        debuggee_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Clouddebugger_api Documentation](https://cloud.google.com/clouddebugger_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
