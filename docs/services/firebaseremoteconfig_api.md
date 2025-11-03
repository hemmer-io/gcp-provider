# Firebaseremoteconfig_api Service



**Resources**: 1

---

## Overview

The firebaseremoteconfig_api service provides access to 1 resource type:

- [Project](#project) [RU]

---

## Resources


### Project

Get the latest version Remote Configuration for a project.
Returns the RemoteConfig as the payload, and also the eTag as a
response header.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parameters` | HashMap<String, String> |  | Map of parameter keys to their optional default values and optional submap
of (condition name : value). Order doesn't affect semantics, and so is
sorted by the server. The 'key' values of the params must be unique. |
| `conditions` | Vec<String> |  | The list of named conditions. The order *does* affect the semantics.
The condition_name values of these entries must be unique.

The resolved value of a config parameter P is determined as follow:
* Let Y be the set of values from the submap of P that refer to conditions
  that evaluate to <code>true</code>.
* If Y is non empty, the value is taken from the specific submap in Y whose
  condition_name is the earliest in this condition list.
* Else, if P has a default value option (condition_name is empty) then
  the value is taken from that option.
* Else, parameter P has no value and is omitted from the config result.

Example: parameter key "p1", default value "v1", submap specified as
{"c1": v2, "c2": v3} where "c1" and "c2" are names of conditions in the
condition list (where "c1" in this example appears before "c2").  The
value of p1 would be v2 as long as c1 is true.  Otherwise, if c2 is true,
p1 would evaluate to v3, and if c1 and c2 are both false, p1 would evaluate
to v1.  If no default value was specified, and c1 and c2 were both false,
no value for p1 would be generated. |
| `project` | String | ✅ | The GMP project identifier. Required.
See note at the beginning of this file regarding project ids. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `parameters` | HashMap<String, String> | Map of parameter keys to their optional default values and optional submap
of (condition name : value). Order doesn't affect semantics, and so is
sorted by the server. The 'key' values of the params must be unique. |
| `conditions` | Vec<String> | The list of named conditions. The order *does* affect the semantics.
The condition_name values of these entries must be unique.

The resolved value of a config parameter P is determined as follow:
* Let Y be the set of values from the submap of P that refer to conditions
  that evaluate to <code>true</code>.
* If Y is non empty, the value is taken from the specific submap in Y whose
  condition_name is the earliest in this condition list.
* Else, if P has a default value option (condition_name is empty) then
  the value is taken from that option.
* Else, parameter P has no value and is omitted from the config result.

Example: parameter key "p1", default value "v1", submap specified as
{"c1": v2, "c2": v3} where "c1" and "c2" are names of conditions in the
condition list (where "c1" in this example appears before "c2").  The
value of p1 would be v2 as long as c1 is true.  Otherwise, if c2 is true,
p1 would evaluate to v3, and if c1 and c2 are both false, p1 would evaluate
to v1.  If no default value was specified, and c1 and c2 were both false,
no value for p1 would be generated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access project outputs
project_id = project.id
project_parameters = project.parameters
project_conditions = project.conditions
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple project resources
project_0 = provider.firebaseremoteconfig_api.Project {
    project = "value-0"
}
project_1 = provider.firebaseremoteconfig_api.Project {
    project = "value-1"
}
project_2 = provider.firebaseremoteconfig_api.Project {
    project = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    project = provider.firebaseremoteconfig_api.Project {
        project = "production-value"
    }
```

---

## Related Documentation

- [GCP Firebaseremoteconfig_api Documentation](https://cloud.google.com/firebaseremoteconfig_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
