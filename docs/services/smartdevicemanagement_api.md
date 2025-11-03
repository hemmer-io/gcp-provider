# Smartdevicemanagement_api Service



**Resources**: 3

---

## Overview

The smartdevicemanagement_api service provides access to 3 resource types:

- [Structure](#structure) [R]
- [Room](#room) [R]
- [Device](#device) [CR]

---

## Resources


### Structure

Gets a structure managed by the enterprise.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name of the structure. For example: "enterprises/XYZ/structures/ABC". |
| `traits` | HashMap<String, String> | Structure traits. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access structure outputs
structure_id = structure.id
structure_name = structure.name
structure_traits = structure.traits
```

---


### Room

Gets a room managed by the enterprise.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name of the room. For example: "enterprises/XYZ/structures/ABC/rooms/123". |
| `traits` | HashMap<String, String> | Room traits. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access room outputs
room_id = room.id
room_name = room.name
room_traits = room.traits
```

---


### Device

Executes a command to device managed by the enterprise.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `command` | String |  | The command name to execute, represented by the fully qualified protobuf message name. |
| `params` | HashMap<String, String> |  | The command message to execute, represented as a Struct. |
| `name` | String | ✅ | The name of the device requested. For example: "enterprises/XYZ/devices/123" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. The resource name of the device. For example: "enterprises/XYZ/devices/123". |
| `traits` | HashMap<String, String> | Output only. Device traits. |
| `type` | String | Output only. Type of the device for general display purposes. For example: "THERMOSTAT". The device type should not be used to deduce or infer functionality of the actual device it is assigned to. Instead, use the returned traits for the device. |
| `parent_relations` | Vec<String> | Assignee details of the device. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device
device = provider.smartdevicemanagement_api.Device {
    name = "value"  # The name of the device requested. For example: "enterprises/XYZ/devices/123"
}

# Access device outputs
device_id = device.id
device_name = device.name
device_traits = device.traits
device_type = device.type
device_parent_relations = device.parent_relations
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple structure resources
structure_0 = provider.smartdevicemanagement_api.Structure {
}
structure_1 = provider.smartdevicemanagement_api.Structure {
}
structure_2 = provider.smartdevicemanagement_api.Structure {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    structure = provider.smartdevicemanagement_api.Structure {
    }
```

---

## Related Documentation

- [GCP Smartdevicemanagement_api Documentation](https://cloud.google.com/smartdevicemanagement_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
