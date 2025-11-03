# Networkmanagement_api Service



**Resources**: 8

---

## Overview

The networkmanagement_api service provides access to 8 resource types:

- [Vpc_flow_logs_config](#vpc_flow_logs_config) [CRUD]
- [Operation](#operation) [CRD]
- [Location](#location) [R]
- [Connectivity_test](#connectivity_test) [CRUD]
- [Vpc_flow_logs_config](#vpc_flow_logs_config) [CRUD]
- [Operation](#operation) [CRD]
- [Connectivity_test](#connectivity_test) [CRUD]
- [Location](#location) [R]

---

## Resources


### Vpc_flow_logs_config

Creates a new `VpcFlowLogsConfig`. If a configuration with the exact same settings already exists (even if the ID is different), the creation fails. Notes: 1. Creating a configuration with `state=DISABLED` will fail 2. The following fields are not considered as settings for the purpose of the check mentioned above, therefore - creating another configuration with the same fields but different values for the following fields will fail as well: * name * create_time * update_time * labels * description

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time the config was created. |
| `network` | String |  | Traffic will be logged from VMs, VPN tunnels and Interconnect Attachments within the network. Format: projects/{project_id}/global/networks/{name} |
| `aggregation_interval` | String |  | Optional. The aggregation interval for the logs. Default value is INTERVAL_5_SEC. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. |
| `target_resource_state` | String |  | Output only. Describes the state of the configured target resource for diagnostic purposes. |
| `state` | String |  | Optional. The state of the VPC Flow Log configuration. Default value is ENABLED. When creating a new configuration, it must be enabled. Setting state=DISABLED will pause the log generation for this config. |
| `name` | String |  | Identifier. Unique name of the configuration. The name can have one of the following forms: - For project-level configurations: `projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` - For organization-level configurations: `organizations/{organization_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` |
| `update_time` | String |  | Output only. The time the config was updated. |
| `description` | String |  | Optional. The user-supplied description of the VPC Flow Logs configuration. Maximum of 512 characters. |
| `interconnect_attachment` | String |  | Traffic will be logged from the Interconnect Attachment. Format: projects/{project_id}/regions/{region}/interconnectAttachments/{name} |
| `flow_sampling` | f64 |  | Optional. The value of the field must be in (0, 1]. The sampling rate of VPC Flow Logs where 1.0 means all collected logs are reported. Setting the sampling rate to 0.0 is not allowed. If you want to disable VPC Flow Logs, use the state field instead. Default value is 1.0. |
| `vpn_tunnel` | String |  | Traffic will be logged from the VPN Tunnel. Format: projects/{project_id}/regions/{region}/vpnTunnels/{name} |
| `subnet` | String |  | Traffic will be logged from VMs within the subnetwork. Format: projects/{project_id}/regions/{region}/subnetworks/{name} |
| `cross_project_metadata` | String |  | Optional. Determines whether to include cross project annotations in the logs. This field is available only for organization configurations. If not specified in org configs will be set to CROSS_PROJECT_METADATA_ENABLED. |
| `metadata` | String |  | Optional. Configures whether all, none or a subset of metadata fields should be added to the reported VPC flow logs. Default value is INCLUDE_ALL_METADATA. |
| `metadata_fields` | Vec<String> |  | Optional. Custom metadata fields to include in the reported VPC flow logs. Can only be specified if "metadata" was set to CUSTOM_METADATA. |
| `filter_expr` | String |  | Optional. Export filter used to define which VPC Flow Logs should be logged. |
| `parent` | String | ✅ | Required. The parent resource of the VpcFlowLogsConfig to create, in one of the following formats: - For project-level resources: `projects/{project_id}/locations/global` - For organization-level resources: `organizations/{organization_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time the config was created. |
| `network` | String | Traffic will be logged from VMs, VPN tunnels and Interconnect Attachments within the network. Format: projects/{project_id}/global/networks/{name} |
| `aggregation_interval` | String | Optional. The aggregation interval for the logs. Default value is INTERVAL_5_SEC. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. |
| `target_resource_state` | String | Output only. Describes the state of the configured target resource for diagnostic purposes. |
| `state` | String | Optional. The state of the VPC Flow Log configuration. Default value is ENABLED. When creating a new configuration, it must be enabled. Setting state=DISABLED will pause the log generation for this config. |
| `name` | String | Identifier. Unique name of the configuration. The name can have one of the following forms: - For project-level configurations: `projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` - For organization-level configurations: `organizations/{organization_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` |
| `update_time` | String | Output only. The time the config was updated. |
| `description` | String | Optional. The user-supplied description of the VPC Flow Logs configuration. Maximum of 512 characters. |
| `interconnect_attachment` | String | Traffic will be logged from the Interconnect Attachment. Format: projects/{project_id}/regions/{region}/interconnectAttachments/{name} |
| `flow_sampling` | f64 | Optional. The value of the field must be in (0, 1]. The sampling rate of VPC Flow Logs where 1.0 means all collected logs are reported. Setting the sampling rate to 0.0 is not allowed. If you want to disable VPC Flow Logs, use the state field instead. Default value is 1.0. |
| `vpn_tunnel` | String | Traffic will be logged from the VPN Tunnel. Format: projects/{project_id}/regions/{region}/vpnTunnels/{name} |
| `subnet` | String | Traffic will be logged from VMs within the subnetwork. Format: projects/{project_id}/regions/{region}/subnetworks/{name} |
| `cross_project_metadata` | String | Optional. Determines whether to include cross project annotations in the logs. This field is available only for organization configurations. If not specified in org configs will be set to CROSS_PROJECT_METADATA_ENABLED. |
| `metadata` | String | Optional. Configures whether all, none or a subset of metadata fields should be added to the reported VPC flow logs. Default value is INCLUDE_ALL_METADATA. |
| `metadata_fields` | Vec<String> | Optional. Custom metadata fields to include in the reported VPC flow logs. Can only be specified if "metadata" was set to CUSTOM_METADATA. |
| `filter_expr` | String | Optional. Export filter used to define which VPC Flow Logs should be logged. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create vpc_flow_logs_config
vpc_flow_logs_config = provider.networkmanagement_api.Vpc_flow_logs_config {
    parent = "value"  # Required. The parent resource of the VpcFlowLogsConfig to create, in one of the following formats: - For project-level resources: `projects/{project_id}/locations/global` - For organization-level resources: `organizations/{organization_id}/locations/global`
}

# Access vpc_flow_logs_config outputs
vpc_flow_logs_config_id = vpc_flow_logs_config.id
vpc_flow_logs_config_create_time = vpc_flow_logs_config.create_time
vpc_flow_logs_config_network = vpc_flow_logs_config.network
vpc_flow_logs_config_aggregation_interval = vpc_flow_logs_config.aggregation_interval
vpc_flow_logs_config_labels = vpc_flow_logs_config.labels
vpc_flow_logs_config_target_resource_state = vpc_flow_logs_config.target_resource_state
vpc_flow_logs_config_state = vpc_flow_logs_config.state
vpc_flow_logs_config_name = vpc_flow_logs_config.name
vpc_flow_logs_config_update_time = vpc_flow_logs_config.update_time
vpc_flow_logs_config_description = vpc_flow_logs_config.description
vpc_flow_logs_config_interconnect_attachment = vpc_flow_logs_config.interconnect_attachment
vpc_flow_logs_config_flow_sampling = vpc_flow_logs_config.flow_sampling
vpc_flow_logs_config_vpn_tunnel = vpc_flow_logs_config.vpn_tunnel
vpc_flow_logs_config_subnet = vpc_flow_logs_config.subnet
vpc_flow_logs_config_cross_project_metadata = vpc_flow_logs_config.cross_project_metadata
vpc_flow_logs_config_metadata = vpc_flow_logs_config.metadata
vpc_flow_logs_config_metadata_fields = vpc_flow_logs_config.metadata_fields
vpc_flow_logs_config_filter_expr = vpc_flow_logs_config.filter_expr
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.networkmanagement_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_done = operation.done
operation_response = operation.response
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_location_id = location.location_id
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
location_display_name = location.display_name
```

---


### Connectivity_test

Creates a new Connectivity Test. After you create a test, the reachability analysis is performed as part of the long running operation, which completes when the analysis completes. If the endpoint specifications in `ConnectivityTest` are invalid (for example, containing non-existent resources in the network, or you don't have read permissions to the network configurations of listed projects), then the reachability result returns a value of `UNKNOWN`. If the endpoint specifications in `ConnectivityTest` are incomplete, the reachability result returns a value of AMBIGUOUS. For more information, see the Connectivity Test documentation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | Output only. The display name of a Connectivity Test. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user-provided metadata. |
| `description` | String |  | The user-supplied description of the Connectivity Test. Maximum of 512 characters. |
| `create_time` | String |  | Output only. The time the test was created. |
| `protocol` | String |  | IP Protocol of the test. When not provided, "TCP" is assumed. |
| `return_reachability_details` | String |  | Output only. The reachability details of this test from the latest run for the return path. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |
| `reachability_details` | String |  | Output only. The reachability details of this test from the latest run. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |
| `bypass_firewall_checks` | bool |  | Whether the analysis should skip firewall checking. Default value is false. |
| `name` | String |  | Identifier. Unique name of the resource using the form: `projects/{project_id}/locations/global/connectivityTests/{test_id}` |
| `update_time` | String |  | Output only. The time the test's configuration was updated. |
| `round_trip` | bool |  | Whether run analysis for the return path from destination to source. Default value is false. |
| `destination` | String |  | Required. Destination specification of the Connectivity Test. You can use a combination of destination IP address, URI of a supported endpoint, project ID, or VPC network to identify the destination location. Reachability analysis proceeds even if the destination location is ambiguous. However, the test result might include endpoints or use a destination that you don't intend to test. |
| `source` | String |  | Required. Source specification of the Connectivity Test. You can use a combination of source IP address, URI of a supported endpoint, project ID, or VPC network to identify the source location. Reachability analysis might proceed even if the source location is ambiguous. However, the test result might include endpoints or use a source that you don't intend to test. |
| `related_projects` | Vec<String> |  | Other projects that may be relevant for reachability analysis. This is applicable to scenarios where a test can cross project boundaries. |
| `probing_details` | String |  | Output only. The probing details of this test from the latest run, present for applicable tests only. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |
| `parent` | String | ✅ | Required. The parent resource of the Connectivity Test to create: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Output only. The display name of a Connectivity Test. |
| `labels` | HashMap<String, String> | Resource labels to represent user-provided metadata. |
| `description` | String | The user-supplied description of the Connectivity Test. Maximum of 512 characters. |
| `create_time` | String | Output only. The time the test was created. |
| `protocol` | String | IP Protocol of the test. When not provided, "TCP" is assumed. |
| `return_reachability_details` | String | Output only. The reachability details of this test from the latest run for the return path. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |
| `reachability_details` | String | Output only. The reachability details of this test from the latest run. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |
| `bypass_firewall_checks` | bool | Whether the analysis should skip firewall checking. Default value is false. |
| `name` | String | Identifier. Unique name of the resource using the form: `projects/{project_id}/locations/global/connectivityTests/{test_id}` |
| `update_time` | String | Output only. The time the test's configuration was updated. |
| `round_trip` | bool | Whether run analysis for the return path from destination to source. Default value is false. |
| `destination` | String | Required. Destination specification of the Connectivity Test. You can use a combination of destination IP address, URI of a supported endpoint, project ID, or VPC network to identify the destination location. Reachability analysis proceeds even if the destination location is ambiguous. However, the test result might include endpoints or use a destination that you don't intend to test. |
| `source` | String | Required. Source specification of the Connectivity Test. You can use a combination of source IP address, URI of a supported endpoint, project ID, or VPC network to identify the source location. Reachability analysis might proceed even if the source location is ambiguous. However, the test result might include endpoints or use a source that you don't intend to test. |
| `related_projects` | Vec<String> | Other projects that may be relevant for reachability analysis. This is applicable to scenarios where a test can cross project boundaries. |
| `probing_details` | String | Output only. The probing details of this test from the latest run, present for applicable tests only. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connectivity_test
connectivity_test = provider.networkmanagement_api.Connectivity_test {
    parent = "value"  # Required. The parent resource of the Connectivity Test to create: `projects/{project_id}/locations/global`
}

# Access connectivity_test outputs
connectivity_test_id = connectivity_test.id
connectivity_test_display_name = connectivity_test.display_name
connectivity_test_labels = connectivity_test.labels
connectivity_test_description = connectivity_test.description
connectivity_test_create_time = connectivity_test.create_time
connectivity_test_protocol = connectivity_test.protocol
connectivity_test_return_reachability_details = connectivity_test.return_reachability_details
connectivity_test_reachability_details = connectivity_test.reachability_details
connectivity_test_bypass_firewall_checks = connectivity_test.bypass_firewall_checks
connectivity_test_name = connectivity_test.name
connectivity_test_update_time = connectivity_test.update_time
connectivity_test_round_trip = connectivity_test.round_trip
connectivity_test_destination = connectivity_test.destination
connectivity_test_source = connectivity_test.source
connectivity_test_related_projects = connectivity_test.related_projects
connectivity_test_probing_details = connectivity_test.probing_details
```

---


### Vpc_flow_logs_config

Creates a new `VpcFlowLogsConfig`. If a configuration with the exact same settings already exists (even if the ID is different), the creation fails. Notes: 1. Creating a configuration with `state=DISABLED` will fail 2. The following fields are not considered as settings for the purpose of the check mentioned above, therefore - creating another configuration with the same fields but different values for the following fields will fail as well: * name * create_time * update_time * labels * description

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Unique name of the configuration. The name can have one of the following forms: - For project-level configurations: `projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` - For organization-level configurations: `organizations/{organization_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` |
| `vpn_tunnel` | String |  | Traffic will be logged from the VPN Tunnel. Format: projects/{project_id}/regions/{region}/vpnTunnels/{name} |
| `metadata_fields` | Vec<String> |  | Optional. Custom metadata fields to include in the reported VPC flow logs. Can only be specified if "metadata" was set to CUSTOM_METADATA. |
| `create_time` | String |  | Output only. The time the config was created. |
| `description` | String |  | Optional. The user-supplied description of the VPC Flow Logs configuration. Maximum of 512 characters. |
| `network` | String |  | Traffic will be logged from VMs, VPN tunnels and Interconnect Attachments within the network. Format: projects/{project_id}/global/networks/{name} |
| `subnet` | String |  | Traffic will be logged from VMs within the subnetwork. Format: projects/{project_id}/regions/{region}/subnetworks/{name} |
| `target_resource_state` | String |  | Output only. Describes the state of the configured target resource for diagnostic purposes. |
| `aggregation_interval` | String |  | Optional. The aggregation interval for the logs. Default value is INTERVAL_5_SEC. |
| `state` | String |  | Optional. The state of the VPC Flow Log configuration. Default value is ENABLED. When creating a new configuration, it must be enabled. Setting state=DISABLED will pause the log generation for this config. |
| `flow_sampling` | f64 |  | Optional. The value of the field must be in (0, 1]. The sampling rate of VPC Flow Logs where 1.0 means all collected logs are reported. Setting the sampling rate to 0.0 is not allowed. If you want to disable VPC Flow Logs, use the state field instead. Default value is 1.0. |
| `cross_project_metadata` | String |  | Optional. Determines whether to include cross project annotations in the logs. This field is available only for organization configurations. If not specified in org configs will be set to CROSS_PROJECT_METADATA_ENABLED. |
| `interconnect_attachment` | String |  | Traffic will be logged from the Interconnect Attachment. Format: projects/{project_id}/regions/{region}/interconnectAttachments/{name} |
| `metadata` | String |  | Optional. Configures whether all, none or a subset of metadata fields should be added to the reported VPC flow logs. Default value is INCLUDE_ALL_METADATA. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels to represent user-provided metadata. |
| `filter_expr` | String |  | Optional. Export filter used to define which VPC Flow Logs should be logged. |
| `update_time` | String |  | Output only. The time the config was updated. |
| `parent` | String | ✅ | Required. The parent resource of the VpcFlowLogsConfig to create, in one of the following formats: - For project-level resources: `projects/{project_id}/locations/global` - For organization-level resources: `organizations/{organization_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Unique name of the configuration. The name can have one of the following forms: - For project-level configurations: `projects/{project_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` - For organization-level configurations: `organizations/{organization_id}/locations/global/vpcFlowLogsConfigs/{vpc_flow_logs_config_id}` |
| `vpn_tunnel` | String | Traffic will be logged from the VPN Tunnel. Format: projects/{project_id}/regions/{region}/vpnTunnels/{name} |
| `metadata_fields` | Vec<String> | Optional. Custom metadata fields to include in the reported VPC flow logs. Can only be specified if "metadata" was set to CUSTOM_METADATA. |
| `create_time` | String | Output only. The time the config was created. |
| `description` | String | Optional. The user-supplied description of the VPC Flow Logs configuration. Maximum of 512 characters. |
| `network` | String | Traffic will be logged from VMs, VPN tunnels and Interconnect Attachments within the network. Format: projects/{project_id}/global/networks/{name} |
| `subnet` | String | Traffic will be logged from VMs within the subnetwork. Format: projects/{project_id}/regions/{region}/subnetworks/{name} |
| `target_resource_state` | String | Output only. Describes the state of the configured target resource for diagnostic purposes. |
| `aggregation_interval` | String | Optional. The aggregation interval for the logs. Default value is INTERVAL_5_SEC. |
| `state` | String | Optional. The state of the VPC Flow Log configuration. Default value is ENABLED. When creating a new configuration, it must be enabled. Setting state=DISABLED will pause the log generation for this config. |
| `flow_sampling` | f64 | Optional. The value of the field must be in (0, 1]. The sampling rate of VPC Flow Logs where 1.0 means all collected logs are reported. Setting the sampling rate to 0.0 is not allowed. If you want to disable VPC Flow Logs, use the state field instead. Default value is 1.0. |
| `cross_project_metadata` | String | Optional. Determines whether to include cross project annotations in the logs. This field is available only for organization configurations. If not specified in org configs will be set to CROSS_PROJECT_METADATA_ENABLED. |
| `interconnect_attachment` | String | Traffic will be logged from the Interconnect Attachment. Format: projects/{project_id}/regions/{region}/interconnectAttachments/{name} |
| `metadata` | String | Optional. Configures whether all, none or a subset of metadata fields should be added to the reported VPC flow logs. Default value is INCLUDE_ALL_METADATA. |
| `labels` | HashMap<String, String> | Optional. Resource labels to represent user-provided metadata. |
| `filter_expr` | String | Optional. Export filter used to define which VPC Flow Logs should be logged. |
| `update_time` | String | Output only. The time the config was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create vpc_flow_logs_config
vpc_flow_logs_config = provider.networkmanagement_api.Vpc_flow_logs_config {
    parent = "value"  # Required. The parent resource of the VpcFlowLogsConfig to create, in one of the following formats: - For project-level resources: `projects/{project_id}/locations/global` - For organization-level resources: `organizations/{organization_id}/locations/global`
}

# Access vpc_flow_logs_config outputs
vpc_flow_logs_config_id = vpc_flow_logs_config.id
vpc_flow_logs_config_name = vpc_flow_logs_config.name
vpc_flow_logs_config_vpn_tunnel = vpc_flow_logs_config.vpn_tunnel
vpc_flow_logs_config_metadata_fields = vpc_flow_logs_config.metadata_fields
vpc_flow_logs_config_create_time = vpc_flow_logs_config.create_time
vpc_flow_logs_config_description = vpc_flow_logs_config.description
vpc_flow_logs_config_network = vpc_flow_logs_config.network
vpc_flow_logs_config_subnet = vpc_flow_logs_config.subnet
vpc_flow_logs_config_target_resource_state = vpc_flow_logs_config.target_resource_state
vpc_flow_logs_config_aggregation_interval = vpc_flow_logs_config.aggregation_interval
vpc_flow_logs_config_state = vpc_flow_logs_config.state
vpc_flow_logs_config_flow_sampling = vpc_flow_logs_config.flow_sampling
vpc_flow_logs_config_cross_project_metadata = vpc_flow_logs_config.cross_project_metadata
vpc_flow_logs_config_interconnect_attachment = vpc_flow_logs_config.interconnect_attachment
vpc_flow_logs_config_metadata = vpc_flow_logs_config.metadata
vpc_flow_logs_config_labels = vpc_flow_logs_config.labels
vpc_flow_logs_config_filter_expr = vpc_flow_logs_config.filter_expr
vpc_flow_logs_config_update_time = vpc_flow_logs_config.update_time
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.networkmanagement_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_done = operation.done
operation_name = operation.name
operation_metadata = operation.metadata
operation_error = operation.error
```

---


### Connectivity_test

Creates a new Connectivity Test. After you create a test, the reachability analysis is performed as part of the long running operation, which completes when the analysis completes. If the endpoint specifications in `ConnectivityTest` are invalid (for example, containing non-existent resources in the network, or you don't have read permissions to the network configurations of listed projects), then the reachability result returns a value of `UNKNOWN`. If the endpoint specifications in `ConnectivityTest` are incomplete, the reachability result returns a value of AMBIGUOUS. For more information, see the Connectivity Test documentation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `protocol` | String |  | IP Protocol of the test. When not provided, "TCP" is assumed. |
| `reachability_details` | String |  | Output only. The reachability details of this test from the latest run. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |
| `name` | String |  | Identifier. Unique name of the resource using the form: `projects/{project_id}/locations/global/connectivityTests/{test}` |
| `source` | String |  | Required. Source specification of the Connectivity Test. You can use a combination of source IP address, URI of a supported endpoint, project ID, or VPC network to identify the source location. Reachability analysis might proceed even if the source location is ambiguous. However, the test result might include endpoints or use a source that you don't intend to test. |
| `related_projects` | Vec<String> |  | Other projects that may be relevant for reachability analysis. This is applicable to scenarios where a test can cross project boundaries. |
| `update_time` | String |  | Output only. The time the test's configuration was updated. |
| `return_reachability_details` | String |  | Output only. The reachability details of this test from the latest run for the return path. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |
| `create_time` | String |  | Output only. The time the test was created. |
| `bypass_firewall_checks` | bool |  | Whether the analysis should skip firewall checking. Default value is false. |
| `description` | String |  | The user-supplied description of the Connectivity Test. Maximum of 512 characters. |
| `destination` | String |  | Required. Destination specification of the Connectivity Test. You can use a combination of destination IP address, URI of a supported endpoint, project ID, or VPC network to identify the destination location. Reachability analysis proceeds even if the destination location is ambiguous. However, the test result might include endpoints or use a destination that you don't intend to test. |
| `round_trip` | bool |  | Whether run analysis for the return path from destination to source. Default value is false. |
| `labels` | HashMap<String, String> |  | Resource labels to represent user-provided metadata. |
| `display_name` | String |  | Output only. The display name of a Connectivity Test. |
| `probing_details` | String |  | Output only. The probing details of this test from the latest run, present for applicable tests only. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |
| `parent` | String | ✅ | Required. The parent resource of the Connectivity Test to create: `projects/{project_id}/locations/global` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `protocol` | String | IP Protocol of the test. When not provided, "TCP" is assumed. |
| `reachability_details` | String | Output only. The reachability details of this test from the latest run. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |
| `name` | String | Identifier. Unique name of the resource using the form: `projects/{project_id}/locations/global/connectivityTests/{test}` |
| `source` | String | Required. Source specification of the Connectivity Test. You can use a combination of source IP address, URI of a supported endpoint, project ID, or VPC network to identify the source location. Reachability analysis might proceed even if the source location is ambiguous. However, the test result might include endpoints or use a source that you don't intend to test. |
| `related_projects` | Vec<String> | Other projects that may be relevant for reachability analysis. This is applicable to scenarios where a test can cross project boundaries. |
| `update_time` | String | Output only. The time the test's configuration was updated. |
| `return_reachability_details` | String | Output only. The reachability details of this test from the latest run for the return path. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |
| `create_time` | String | Output only. The time the test was created. |
| `bypass_firewall_checks` | bool | Whether the analysis should skip firewall checking. Default value is false. |
| `description` | String | The user-supplied description of the Connectivity Test. Maximum of 512 characters. |
| `destination` | String | Required. Destination specification of the Connectivity Test. You can use a combination of destination IP address, URI of a supported endpoint, project ID, or VPC network to identify the destination location. Reachability analysis proceeds even if the destination location is ambiguous. However, the test result might include endpoints or use a destination that you don't intend to test. |
| `round_trip` | bool | Whether run analysis for the return path from destination to source. Default value is false. |
| `labels` | HashMap<String, String> | Resource labels to represent user-provided metadata. |
| `display_name` | String | Output only. The display name of a Connectivity Test. |
| `probing_details` | String | Output only. The probing details of this test from the latest run, present for applicable tests only. The details are updated when creating a new test, updating an existing test, or triggering a one-time rerun of an existing test. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create connectivity_test
connectivity_test = provider.networkmanagement_api.Connectivity_test {
    parent = "value"  # Required. The parent resource of the Connectivity Test to create: `projects/{project_id}/locations/global`
}

# Access connectivity_test outputs
connectivity_test_id = connectivity_test.id
connectivity_test_protocol = connectivity_test.protocol
connectivity_test_reachability_details = connectivity_test.reachability_details
connectivity_test_name = connectivity_test.name
connectivity_test_source = connectivity_test.source
connectivity_test_related_projects = connectivity_test.related_projects
connectivity_test_update_time = connectivity_test.update_time
connectivity_test_return_reachability_details = connectivity_test.return_reachability_details
connectivity_test_create_time = connectivity_test.create_time
connectivity_test_bypass_firewall_checks = connectivity_test.bypass_firewall_checks
connectivity_test_description = connectivity_test.description
connectivity_test_destination = connectivity_test.destination
connectivity_test_round_trip = connectivity_test.round_trip
connectivity_test_labels = connectivity_test.labels
connectivity_test_display_name = connectivity_test.display_name
connectivity_test_probing_details = connectivity_test.probing_details
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_name = location.name
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple vpc_flow_logs_config resources
vpc_flow_logs_config_0 = provider.networkmanagement_api.Vpc_flow_logs_config {
    parent = "value-0"
}
vpc_flow_logs_config_1 = provider.networkmanagement_api.Vpc_flow_logs_config {
    parent = "value-1"
}
vpc_flow_logs_config_2 = provider.networkmanagement_api.Vpc_flow_logs_config {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    vpc_flow_logs_config = provider.networkmanagement_api.Vpc_flow_logs_config {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Networkmanagement_api Documentation](https://cloud.google.com/networkmanagement_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
