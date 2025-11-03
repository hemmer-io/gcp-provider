# Cloudiot_api Service



**Resources**: 5

---

## Overview

The cloudiot_api service provides access to 5 resource types:

- [Group](#group) [C]
- [Config_version](#config_version) [R]
- [Registrie](#registrie) [CRUD]
- [State](#state) [R]
- [Device](#device) [CRUD]

---

## Resources


### Group

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create group
group = provider.cloudiot_api.Group {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

```

---


### Config_version

Lists the last few versions of the device configuration in descending order (i.e.: newest first).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `device_configs` | Vec<String> | The device configuration for the last few versions. Versions are listed in decreasing order, starting from the most recent one. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access config_version outputs
config_version_id = config_version.id
config_version_device_configs = config_version.device_configs
```

---


### Registrie

Creates a device registry that contains devices.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_notification_configs` | Vec<String> |  | The configuration for notification of telemetry events received from the device. All telemetry events that were successfully published by the device and acknowledged by Cloud IoT Core are guaranteed to be delivered to Cloud Pub/Sub. If multiple configurations match a message, only the first matching configuration is used. If you try to publish a device telemetry event using MQTT without specifying a Cloud Pub/Sub topic for the device's registry, the connection closes automatically. If you try to do so using an HTTP connection, an error is returned. Up to 10 configurations may be provided. |
| `http_config` | String |  | The DeviceService (HTTP) configuration for this device registry. |
| `credentials` | Vec<String> |  | The credentials used to verify the device credentials. No more than 10 credentials can be bound to a single registry at a time. The verification process occurs at the time of device creation or update. If this field is empty, no verification is performed. Otherwise, the credentials of a newly created device or added credentials of an updated device should be signed with one of these registry credentials. Note, however, that existing devices will never be affected by modifications to this list of credentials: after a device has been successfully created in a registry, it should be able to connect even if its registry credentials are revoked, deleted, or modified. |
| `mqtt_config` | String |  | The MQTT configuration for this device registry. |
| `name` | String |  | The resource path name. For example, `projects/example-project/locations/us-central1/registries/my-registry`. |
| `id` | String |  | The identifier of this device registry. For example, `myRegistry`. |
| `state_notification_config` | String |  | The configuration for notification of new states received from the device. State updates are guaranteed to be stored in the state history, but notifications to Cloud Pub/Sub are not guaranteed. For example, if permissions are misconfigured or the specified topic doesn't exist, no notification will be published but the state will still be stored in Cloud IoT Core. |
| `log_level` | String |  | **Beta Feature** The default logging verbosity for activity from devices in this registry. The verbosity level can be overridden by Device.log_level. |
| `parent` | String | ✅ | Required. The project and cloud region where this device registry must be created. For example, `projects/example-project/locations/us-central1`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_notification_configs` | Vec<String> | The configuration for notification of telemetry events received from the device. All telemetry events that were successfully published by the device and acknowledged by Cloud IoT Core are guaranteed to be delivered to Cloud Pub/Sub. If multiple configurations match a message, only the first matching configuration is used. If you try to publish a device telemetry event using MQTT without specifying a Cloud Pub/Sub topic for the device's registry, the connection closes automatically. If you try to do so using an HTTP connection, an error is returned. Up to 10 configurations may be provided. |
| `http_config` | String | The DeviceService (HTTP) configuration for this device registry. |
| `credentials` | Vec<String> | The credentials used to verify the device credentials. No more than 10 credentials can be bound to a single registry at a time. The verification process occurs at the time of device creation or update. If this field is empty, no verification is performed. Otherwise, the credentials of a newly created device or added credentials of an updated device should be signed with one of these registry credentials. Note, however, that existing devices will never be affected by modifications to this list of credentials: after a device has been successfully created in a registry, it should be able to connect even if its registry credentials are revoked, deleted, or modified. |
| `mqtt_config` | String | The MQTT configuration for this device registry. |
| `name` | String | The resource path name. For example, `projects/example-project/locations/us-central1/registries/my-registry`. |
| `id` | String | The identifier of this device registry. For example, `myRegistry`. |
| `state_notification_config` | String | The configuration for notification of new states received from the device. State updates are guaranteed to be stored in the state history, but notifications to Cloud Pub/Sub are not guaranteed. For example, if permissions are misconfigured or the specified topic doesn't exist, no notification will be published but the state will still be stored in Cloud IoT Core. |
| `log_level` | String | **Beta Feature** The default logging verbosity for activity from devices in this registry. The verbosity level can be overridden by Device.log_level. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create registrie
registrie = provider.cloudiot_api.Registrie {
    parent = "value"  # Required. The project and cloud region where this device registry must be created. For example, `projects/example-project/locations/us-central1`.
}

# Access registrie outputs
registrie_id = registrie.id
registrie_event_notification_configs = registrie.event_notification_configs
registrie_http_config = registrie.http_config
registrie_credentials = registrie.credentials
registrie_mqtt_config = registrie.mqtt_config
registrie_name = registrie.name
registrie_id = registrie.id
registrie_state_notification_config = registrie.state_notification_config
registrie_log_level = registrie.log_level
```

---


### State

Lists the last few versions of the device state in descending order (i.e.: newest first).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `device_states` | Vec<String> | The last few device states. States are listed in descending order of server update time, starting from the most recent one. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access state outputs
state_id = state.id
state_device_states = state.device_states
```

---


### Device

Creates a device in a device registry.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `last_heartbeat_time` | String |  | [Output only] The last time an MQTT `PINGREQ` was received. This field applies only to devices connecting through MQTT. MQTT clients usually only send `PINGREQ` messages if the connection is idle, and no other messages have been sent. Timestamps are periodically collected and written to storage; they may be stale by a few minutes. |
| `metadata` | HashMap<String, String> |  | The metadata key-value pairs assigned to the device. This metadata is not interpreted or indexed by Cloud IoT Core. It can be used to add contextual information for the device. Keys must conform to the regular expression a-zA-Z+ and be less than 128 bytes in length. Values are free-form strings. Each value must be less than or equal to 32 KB in size. The total size of all keys and values must be less than 256 KB, and the maximum number of key-value pairs is 500. |
| `last_config_send_time` | String |  | [Output only] The last time a cloud-to-device config version was sent to the device. |
| `last_error_time` | String |  | [Output only] The time the most recent error occurred, such as a failure to publish to Cloud Pub/Sub. This field is the timestamp of 'last_error_status'. |
| `last_config_ack_time` | String |  | [Output only] The last time a cloud-to-device config version acknowledgment was received from the device. This field is only for configurations sent through MQTT. |
| `last_error_status` | String |  | [Output only] The error message of the most recent error, such as a failure to publish to Cloud Pub/Sub. 'last_error_time' is the timestamp of this field. If no errors have occurred, this field has an empty message and the status code 0 == OK. Otherwise, this field is expected to have a status code other than OK. |
| `config` | String |  | The most recent device configuration, which is eventually sent from Cloud IoT Core to the device. If not present on creation, the configuration will be initialized with an empty payload and version value of `1`. To update this field after creation, use the `DeviceManager.ModifyCloudToDeviceConfig` method. |
| `last_state_time` | String |  | [Output only] The last time a state event was received. Timestamps are periodically collected and written to storage; they may be stale by a few minutes. |
| `blocked` | bool |  | If a device is blocked, connections or requests from this device will fail. Can be used to temporarily prevent the device from connecting if, for example, the sensor is generating bad data and needs maintenance. |
| `id` | String |  | The user-defined device identifier. The device ID must be unique within a device registry. |
| `credentials` | Vec<String> |  | The credentials used to authenticate this device. To allow credential rotation without interruption, multiple device credentials can be bound to this device. No more than 3 credentials can be bound to a single device at a time. When new credentials are added to a device, they are verified against the registry credentials. For details, see the description of the `DeviceRegistry.credentials` field. |
| `log_level` | String |  | **Beta Feature** The logging verbosity for device activity. If unspecified, DeviceRegistry.log_level will be used. |
| `state` | String |  | [Output only] The state most recently received from the device. If no state has been reported, this field is not present. |
| `last_event_time` | String |  | [Output only] The last time a telemetry event was received. Timestamps are periodically collected and written to storage; they may be stale by a few minutes. |
| `gateway_config` | String |  | Gateway-related configuration and state. |
| `name` | String |  | The resource path name. For example, `projects/p1/locations/us-central1/registries/registry0/devices/dev0` or `projects/p1/locations/us-central1/registries/registry0/devices/{num_id}`. When `name` is populated as a response from the service, it always ends in the device numeric ID. |
| `num_id` | String |  | [Output only] A server-defined unique numeric ID for the device. This is a more compact way to identify devices, and it is globally unique. |
| `parent` | String | ✅ | Required. The name of the device registry where this device should be created. For example, `projects/example-project/locations/us-central1/registries/my-registry`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_heartbeat_time` | String | [Output only] The last time an MQTT `PINGREQ` was received. This field applies only to devices connecting through MQTT. MQTT clients usually only send `PINGREQ` messages if the connection is idle, and no other messages have been sent. Timestamps are periodically collected and written to storage; they may be stale by a few minutes. |
| `metadata` | HashMap<String, String> | The metadata key-value pairs assigned to the device. This metadata is not interpreted or indexed by Cloud IoT Core. It can be used to add contextual information for the device. Keys must conform to the regular expression a-zA-Z+ and be less than 128 bytes in length. Values are free-form strings. Each value must be less than or equal to 32 KB in size. The total size of all keys and values must be less than 256 KB, and the maximum number of key-value pairs is 500. |
| `last_config_send_time` | String | [Output only] The last time a cloud-to-device config version was sent to the device. |
| `last_error_time` | String | [Output only] The time the most recent error occurred, such as a failure to publish to Cloud Pub/Sub. This field is the timestamp of 'last_error_status'. |
| `last_config_ack_time` | String | [Output only] The last time a cloud-to-device config version acknowledgment was received from the device. This field is only for configurations sent through MQTT. |
| `last_error_status` | String | [Output only] The error message of the most recent error, such as a failure to publish to Cloud Pub/Sub. 'last_error_time' is the timestamp of this field. If no errors have occurred, this field has an empty message and the status code 0 == OK. Otherwise, this field is expected to have a status code other than OK. |
| `config` | String | The most recent device configuration, which is eventually sent from Cloud IoT Core to the device. If not present on creation, the configuration will be initialized with an empty payload and version value of `1`. To update this field after creation, use the `DeviceManager.ModifyCloudToDeviceConfig` method. |
| `last_state_time` | String | [Output only] The last time a state event was received. Timestamps are periodically collected and written to storage; they may be stale by a few minutes. |
| `blocked` | bool | If a device is blocked, connections or requests from this device will fail. Can be used to temporarily prevent the device from connecting if, for example, the sensor is generating bad data and needs maintenance. |
| `id` | String | The user-defined device identifier. The device ID must be unique within a device registry. |
| `credentials` | Vec<String> | The credentials used to authenticate this device. To allow credential rotation without interruption, multiple device credentials can be bound to this device. No more than 3 credentials can be bound to a single device at a time. When new credentials are added to a device, they are verified against the registry credentials. For details, see the description of the `DeviceRegistry.credentials` field. |
| `log_level` | String | **Beta Feature** The logging verbosity for device activity. If unspecified, DeviceRegistry.log_level will be used. |
| `state` | String | [Output only] The state most recently received from the device. If no state has been reported, this field is not present. |
| `last_event_time` | String | [Output only] The last time a telemetry event was received. Timestamps are periodically collected and written to storage; they may be stale by a few minutes. |
| `gateway_config` | String | Gateway-related configuration and state. |
| `name` | String | The resource path name. For example, `projects/p1/locations/us-central1/registries/registry0/devices/dev0` or `projects/p1/locations/us-central1/registries/registry0/devices/{num_id}`. When `name` is populated as a response from the service, it always ends in the device numeric ID. |
| `num_id` | String | [Output only] A server-defined unique numeric ID for the device. This is a more compact way to identify devices, and it is globally unique. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device
device = provider.cloudiot_api.Device {
    parent = "value"  # Required. The name of the device registry where this device should be created. For example, `projects/example-project/locations/us-central1/registries/my-registry`.
}

# Access device outputs
device_id = device.id
device_last_heartbeat_time = device.last_heartbeat_time
device_metadata = device.metadata
device_last_config_send_time = device.last_config_send_time
device_last_error_time = device.last_error_time
device_last_config_ack_time = device.last_config_ack_time
device_last_error_status = device.last_error_status
device_config = device.config
device_last_state_time = device.last_state_time
device_blocked = device.blocked
device_id = device.id
device_credentials = device.credentials
device_log_level = device.log_level
device_state = device.state
device_last_event_time = device.last_event_time
device_gateway_config = device.gateway_config
device_name = device.name
device_num_id = device.num_id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple group resources
group_0 = provider.cloudiot_api.Group {
    resource = "value-0"
}
group_1 = provider.cloudiot_api.Group {
    resource = "value-1"
}
group_2 = provider.cloudiot_api.Group {
    resource = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    group = provider.cloudiot_api.Group {
        resource = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudiot_api Documentation](https://cloud.google.com/cloudiot_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
