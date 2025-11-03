# Networkservices_api Service



**Resources**: 40

---

## Overview

The networkservices_api service provides access to 40 resource types:

- [Gateway](#gateway) [CRUD]
- [Endpoint_policie](#endpoint_policie) [CRUD]
- [Service_lb_policie](#service_lb_policie) [CRUD]
- [Lb_traffic_extension](#lb_traffic_extension) [CRUD]
- [Tls_route](#tls_route) [CRUD]
- [Wasm_plugin](#wasm_plugin) [CRUD]
- [Version](#version) [CRD]
- [Edge_cache_service](#edge_cache_service) [CR]
- [Lb_edge_extension](#lb_edge_extension) [CRUD]
- [Authz_extension](#authz_extension) [CRUD]
- [Edge_cache_origin](#edge_cache_origin) [CR]
- [Location](#location) [R]
- [Edge_cache_keyset](#edge_cache_keyset) [CR]
- [Meshe](#meshe) [CRUD]
- [Operation](#operation) [CRD]
- [Grpc_route](#grpc_route) [CRUD]
- [Http_route](#http_route) [CRUD]
- [Lb_route_extension](#lb_route_extension) [CRUD]
- [Service_binding](#service_binding) [CRUD]
- [Tcp_route](#tcp_route) [CRUD]
- [Route_view](#route_view) [R]
- [Gateway](#gateway) [CRUD]
- [Lb_traffic_extension](#lb_traffic_extension) [CRUD]
- [Endpoint_policie](#endpoint_policie) [CRUD]
- [Location](#location) [R]
- [Service_binding](#service_binding) [CRUD]
- [Lb_route_extension](#lb_route_extension) [CRUD]
- [Tls_route](#tls_route) [CRUD]
- [Lb_tcp_extension](#lb_tcp_extension) [CRUD]
- [Tcp_route](#tcp_route) [CRUD]
- [Http_route](#http_route) [CRUD]
- [Meshe](#meshe) [CRUD]
- [Authz_extension](#authz_extension) [CRUD]
- [Route_view](#route_view) [R]
- [Service_lb_policie](#service_lb_policie) [CRUD]
- [Lb_edge_extension](#lb_edge_extension) [CRUD]
- [Operation](#operation) [CRD]
- [Wasm_plugin](#wasm_plugin) [CRUD]
- [Grpc_route](#grpc_route) [CRUD]
- [Version](#version) [CRD]

---

## Resources


### Gateway

Creates a new Gateway in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `envoy_headers` | String |  | Optional. Determines if envoy will insert internal debug headers into upstream requests. Other Envoy headers may still be injected. By default, envoy will not insert any debug headers. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `network` | String |  | Optional. The relative resource name identifying the VPC network that is using this configuration. For example: `projects/*/global/networks/network-1`. Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'. |
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `addresses` | Vec<String> |  | Optional. Zero or one IPv4 or IPv6 address on which the Gateway will receive the traffic. When no address is provided, an IP from the subnetwork is allocated This field only applies to gateways of type 'SECURE_WEB_GATEWAY'. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `ip_version` | String |  | Optional. The IP Version that will be used by this gateway. Valid options are IPV4 or IPV6. Default is IPV4. |
| `subnetwork` | String |  | Optional. The relative resource name identifying the subnetwork in which this SWG is allocated. For example: `projects/*/regions/us-central1/subnetworks/network-1` Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY". |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the Gateway resource. |
| `name` | String |  | Identifier. Name of the Gateway resource. It matches pattern `projects/*/locations/*/gateways/`. |
| `routing_mode` | String |  | Optional. The routing mode of the Gateway. This field is configurable only for gateways of type SECURE_WEB_GATEWAY. This field is required for gateways of type SECURE_WEB_GATEWAY. |
| `scope` | String |  | Optional. Scope determines how configuration across multiple Gateway instances are merged. The configuration for multiple Gateway instances with the same scope will be merged as presented as a single configuration to the proxy/load balancer. Max length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens. |
| `server_tls_policy` | String |  | Optional. A fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated. If empty, TLS termination is disabled. |
| `type` | String |  | Immutable. The type of the customer managed gateway. This field is required. If unspecified, an error is returned. |
| `certificate_urls` | Vec<String> |  | Optional. A fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection. This feature only applies to gateways of type 'SECURE_WEB_GATEWAY'. |
| `ports` | Vec<i64> |  | Required. One or more port numbers (1-65535), on which the Gateway will receive traffic. The proxy binds to the specified ports. Gateways of type 'SECURE_WEB_GATEWAY' are limited to 5 ports. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6 and support multiple ports. |
| `gateway_security_policy` | String |  | Optional. A fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections. For example: `projects/*/locations/*/gatewaySecurityPolicies/swg-policy`. This policy is specific to gateways of type 'SECURE_WEB_GATEWAY'. |
| `parent` | String | ✅ | Required. The parent resource of the Gateway. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `envoy_headers` | String | Optional. Determines if envoy will insert internal debug headers into upstream requests. Other Envoy headers may still be injected. By default, envoy will not insert any debug headers. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `network` | String | Optional. The relative resource name identifying the VPC network that is using this configuration. For example: `projects/*/global/networks/network-1`. Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'. |
| `self_link` | String | Output only. Server-defined URL of this resource |
| `addresses` | Vec<String> | Optional. Zero or one IPv4 or IPv6 address on which the Gateway will receive the traffic. When no address is provided, an IP from the subnetwork is allocated This field only applies to gateways of type 'SECURE_WEB_GATEWAY'. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `ip_version` | String | Optional. The IP Version that will be used by this gateway. Valid options are IPV4 or IPV6. Default is IPV4. |
| `subnetwork` | String | Optional. The relative resource name identifying the subnetwork in which this SWG is allocated. For example: `projects/*/regions/us-central1/subnetworks/network-1` Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY". |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the Gateway resource. |
| `name` | String | Identifier. Name of the Gateway resource. It matches pattern `projects/*/locations/*/gateways/`. |
| `routing_mode` | String | Optional. The routing mode of the Gateway. This field is configurable only for gateways of type SECURE_WEB_GATEWAY. This field is required for gateways of type SECURE_WEB_GATEWAY. |
| `scope` | String | Optional. Scope determines how configuration across multiple Gateway instances are merged. The configuration for multiple Gateway instances with the same scope will be merged as presented as a single configuration to the proxy/load balancer. Max length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens. |
| `server_tls_policy` | String | Optional. A fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated. If empty, TLS termination is disabled. |
| `type` | String | Immutable. The type of the customer managed gateway. This field is required. If unspecified, an error is returned. |
| `certificate_urls` | Vec<String> | Optional. A fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection. This feature only applies to gateways of type 'SECURE_WEB_GATEWAY'. |
| `ports` | Vec<i64> | Required. One or more port numbers (1-65535), on which the Gateway will receive traffic. The proxy binds to the specified ports. Gateways of type 'SECURE_WEB_GATEWAY' are limited to 5 ports. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6 and support multiple ports. |
| `gateway_security_policy` | String | Optional. A fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections. For example: `projects/*/locations/*/gatewaySecurityPolicies/swg-policy`. This policy is specific to gateways of type 'SECURE_WEB_GATEWAY'. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gateway
gateway = provider.networkservices_api.Gateway {
    parent = "value"  # Required. The parent resource of the Gateway. Must be in the format `projects/*/locations/*`.
}

# Access gateway outputs
gateway_id = gateway.id
gateway_envoy_headers = gateway.envoy_headers
gateway_update_time = gateway.update_time
gateway_network = gateway.network
gateway_self_link = gateway.self_link
gateway_addresses = gateway.addresses
gateway_create_time = gateway.create_time
gateway_ip_version = gateway.ip_version
gateway_subnetwork = gateway.subnetwork
gateway_description = gateway.description
gateway_labels = gateway.labels
gateway_name = gateway.name
gateway_routing_mode = gateway.routing_mode
gateway_scope = gateway.scope
gateway_server_tls_policy = gateway.server_tls_policy
gateway_type = gateway.type
gateway_certificate_urls = gateway.certificate_urls
gateway_ports = gateway.ports
gateway_gateway_security_policy = gateway.gateway_security_policy
```

---


### Endpoint_policie

Creates a new EndpointPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the EndpointPolicy resource. |
| `type` | String |  | Required. The type of endpoint policy. This is primarily used to validate the configuration. |
| `server_tls_policy` | String |  | Optional. A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be applied to terminate the inbound traffic at the identified backends. If this field is not set, authentication is disabled(open) for this endpoint. |
| `authorization_policy` | String |  | Optional. This field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic at the matched endpoints. Refer to Authorization. If this field is not specified, authorization is disabled(no authz checks) for this endpoint. |
| `endpoint_matcher` | String |  | Required. A matcher that selects endpoints to which the policies should be applied. |
| `name` | String |  | Identifier. Name of the EndpointPolicy resource. It matches pattern `projects/{project}/locations/global/endpointPolicies/{endpoint_policy}`. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `client_tls_policy` | String |  | Optional. A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from the proxy to the actual endpoints. More specifically, it is applied to the outgoing traffic from the proxy to the endpoint. This is typically used for sidecar model where the proxy identifies itself as endpoint to the control plane, with the connection between sidecar and endpoint requiring authentication. If this field is not set, authentication is disabled(open). Applicable only when EndpointPolicyType is SIDECAR_PROXY. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `traffic_port_selector` | String |  | Optional. Port selector for the (matched) endpoints. If no port selector is provided, the matched config is applied to all ports. |
| `parent` | String | ✅ | Required. The parent resource of the EndpointPolicy. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the EndpointPolicy resource. |
| `type` | String | Required. The type of endpoint policy. This is primarily used to validate the configuration. |
| `server_tls_policy` | String | Optional. A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be applied to terminate the inbound traffic at the identified backends. If this field is not set, authentication is disabled(open) for this endpoint. |
| `authorization_policy` | String | Optional. This field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic at the matched endpoints. Refer to Authorization. If this field is not specified, authorization is disabled(no authz checks) for this endpoint. |
| `endpoint_matcher` | String | Required. A matcher that selects endpoints to which the policies should be applied. |
| `name` | String | Identifier. Name of the EndpointPolicy resource. It matches pattern `projects/{project}/locations/global/endpointPolicies/{endpoint_policy}`. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `client_tls_policy` | String | Optional. A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from the proxy to the actual endpoints. More specifically, it is applied to the outgoing traffic from the proxy to the endpoint. This is typically used for sidecar model where the proxy identifies itself as endpoint to the control plane, with the connection between sidecar and endpoint requiring authentication. If this field is not set, authentication is disabled(open). Applicable only when EndpointPolicyType is SIDECAR_PROXY. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `traffic_port_selector` | String | Optional. Port selector for the (matched) endpoints. If no port selector is provided, the matched config is applied to all ports. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create endpoint_policie
endpoint_policie = provider.networkservices_api.Endpoint_policie {
    parent = "value"  # Required. The parent resource of the EndpointPolicy. Must be in the format `projects/*/locations/global`.
}

# Access endpoint_policie outputs
endpoint_policie_id = endpoint_policie.id
endpoint_policie_labels = endpoint_policie.labels
endpoint_policie_type = endpoint_policie.type
endpoint_policie_server_tls_policy = endpoint_policie.server_tls_policy
endpoint_policie_authorization_policy = endpoint_policie.authorization_policy
endpoint_policie_endpoint_matcher = endpoint_policie.endpoint_matcher
endpoint_policie_name = endpoint_policie.name
endpoint_policie_update_time = endpoint_policie.update_time
endpoint_policie_create_time = endpoint_policie.create_time
endpoint_policie_client_tls_policy = endpoint_policie.client_tls_policy
endpoint_policie_description = endpoint_policie.description
endpoint_policie_traffic_port_selector = endpoint_policie.traffic_port_selector
```

---


### Service_lb_policie

Creates a new ServiceLbPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Name of the ServiceLbPolicy resource. It matches pattern `projects/{project}/locations/{location}/serviceLbPolicies/{service_lb_policy_name}`. |
| `auto_capacity_drain` | String |  | Optional. Configuration to automatically move traffic away for unhealthy IG/NEG for the associated Backend Service. |
| `isolation_config` | String |  | Optional. Configuration to provide isolation support for the associated Backend Service. |
| `update_time` | String |  | Output only. The timestamp when this resource was last updated. |
| `create_time` | String |  | Output only. The timestamp when this resource was created. |
| `load_balancing_algorithm` | String |  | Optional. The type of load balancing algorithm to be used. The default behavior is WATERFALL_BY_REGION. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the ServiceLbPolicy resource. |
| `failover_config` | String |  | Optional. Configuration related to health based failover. |
| `parent` | String | ✅ | Required. The parent resource of the ServiceLbPolicy. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Name of the ServiceLbPolicy resource. It matches pattern `projects/{project}/locations/{location}/serviceLbPolicies/{service_lb_policy_name}`. |
| `auto_capacity_drain` | String | Optional. Configuration to automatically move traffic away for unhealthy IG/NEG for the associated Backend Service. |
| `isolation_config` | String | Optional. Configuration to provide isolation support for the associated Backend Service. |
| `update_time` | String | Output only. The timestamp when this resource was last updated. |
| `create_time` | String | Output only. The timestamp when this resource was created. |
| `load_balancing_algorithm` | String | Optional. The type of load balancing algorithm to be used. The default behavior is WATERFALL_BY_REGION. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the ServiceLbPolicy resource. |
| `failover_config` | String | Optional. Configuration related to health based failover. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_lb_policie
service_lb_policie = provider.networkservices_api.Service_lb_policie {
    parent = "value"  # Required. The parent resource of the ServiceLbPolicy. Must be in the format `projects/{project}/locations/{location}`.
}

# Access service_lb_policie outputs
service_lb_policie_id = service_lb_policie.id
service_lb_policie_name = service_lb_policie.name
service_lb_policie_auto_capacity_drain = service_lb_policie.auto_capacity_drain
service_lb_policie_isolation_config = service_lb_policie.isolation_config
service_lb_policie_update_time = service_lb_policie.update_time
service_lb_policie_create_time = service_lb_policie.create_time
service_lb_policie_load_balancing_algorithm = service_lb_policie.load_balancing_algorithm
service_lb_policie_description = service_lb_policie.description
service_lb_policie_labels = service_lb_policie.labels
service_lb_policie_failover_config = service_lb_policie.failover_config
```

---


### Lb_traffic_extension

Creates a new `LbTrafficExtension` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. Identifier. Name of the `LbTrafficExtension` resource in the following format: `projects/{project}/locations/{location}/lbTrafficExtensions/{lb_traffic_extension}`. |
| `forwarding_rules` | Vec<String> |  | Optional. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbTrafficExtension` resource can be associated with a forwarding rule. |
| `extension_chains` | Vec<String> |  | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `LbTrafficExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `load_balancing_scheme` | String |  | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED` and `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `metadata` | HashMap<String, String> |  | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata applies to all extensions in all extensions chains in this resource. The metadata is available under the key `com.google.lb_traffic_extension.`. The following variables are supported in the metadata: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. This field must not be set if at least one of the extension chains contains plugin extensions. Setting it results in a validation error. You can set metadata at either the resource level or the extension level. The extension level metadata is recommended because you can pass a different set of metadata through each extension to the backend. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `parent` | String | ✅ | Required. The parent resource of the `LbTrafficExtension` resource. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. Identifier. Name of the `LbTrafficExtension` resource in the following format: `projects/{project}/locations/{location}/lbTrafficExtensions/{lb_traffic_extension}`. |
| `forwarding_rules` | Vec<String> | Optional. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbTrafficExtension` resource can be associated with a forwarding rule. |
| `extension_chains` | Vec<String> | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `LbTrafficExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `load_balancing_scheme` | String | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED` and `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `metadata` | HashMap<String, String> | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata applies to all extensions in all extensions chains in this resource. The metadata is available under the key `com.google.lb_traffic_extension.`. The following variables are supported in the metadata: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. This field must not be set if at least one of the extension chains contains plugin extensions. Setting it results in a validation error. You can set metadata at either the resource level or the extension level. The extension level metadata is recommended because you can pass a different set of metadata through each extension to the backend. |
| `description` | String | Optional. A human-readable description of the resource. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lb_traffic_extension
lb_traffic_extension = provider.networkservices_api.Lb_traffic_extension {
    parent = "value"  # Required. The parent resource of the `LbTrafficExtension` resource. Must be in the format `projects/{project}/locations/{location}`.
}

# Access lb_traffic_extension outputs
lb_traffic_extension_id = lb_traffic_extension.id
lb_traffic_extension_name = lb_traffic_extension.name
lb_traffic_extension_forwarding_rules = lb_traffic_extension.forwarding_rules
lb_traffic_extension_extension_chains = lb_traffic_extension.extension_chains
lb_traffic_extension_labels = lb_traffic_extension.labels
lb_traffic_extension_load_balancing_scheme = lb_traffic_extension.load_balancing_scheme
lb_traffic_extension_metadata = lb_traffic_extension.metadata
lb_traffic_extension_description = lb_traffic_extension.description
lb_traffic_extension_create_time = lb_traffic_extension.create_time
lb_traffic_extension_update_time = lb_traffic_extension.update_time
```

---


### Tls_route

Creates a new TlsRoute in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gateways` | Vec<String> |  | Optional. Gateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `name` | String |  | Identifier. Name of the TlsRoute resource. It matches pattern `projects/*/locations/global/tlsRoutes/tls_route_name>`. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the TlsRoute resource. |
| `rules` | Vec<String> |  | Required. Rules that define how traffic is routed and handled. At least one RouteRule must be supplied. If there are multiple rules then the action taken will be the first rule to match. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `meshes` | Vec<String> |  | Optional. Meshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |
| `parent` | String | ✅ | Required. The parent resource of the TlsRoute. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gateways` | Vec<String> | Optional. Gateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `self_link` | String | Output only. Server-defined URL of this resource |
| `name` | String | Identifier. Name of the TlsRoute resource. It matches pattern `projects/*/locations/global/tlsRoutes/tls_route_name>`. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the TlsRoute resource. |
| `rules` | Vec<String> | Required. Rules that define how traffic is routed and handled. At least one RouteRule must be supplied. If there are multiple rules then the action taken will be the first rule to match. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `meshes` | Vec<String> | Optional. Meshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tls_route
tls_route = provider.networkservices_api.Tls_route {
    parent = "value"  # Required. The parent resource of the TlsRoute. Must be in the format `projects/*/locations/global`.
}

# Access tls_route outputs
tls_route_id = tls_route.id
tls_route_gateways = tls_route.gateways
tls_route_self_link = tls_route.self_link
tls_route_name = tls_route.name
tls_route_create_time = tls_route.create_time
tls_route_description = tls_route.description
tls_route_labels = tls_route.labels
tls_route_rules = tls_route.rules
tls_route_update_time = tls_route.update_time
tls_route_meshes = tls_route.meshes
```

---


### Wasm_plugin

Creates a new `WasmPlugin` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `main_version_id` | String |  | Optional. The ID of the `WasmPluginVersion` resource that is the currently serving one. The version referred to must be a child of this `WasmPlugin` resource. |
| `log_config` | String |  | Optional. Specifies the logging options for the activity performed by this plugin. If logging is enabled, plugin logs are exported to Cloud Logging. Note that the settings relate to the logs generated by using logging statements in your Wasm code. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `name` | String |  | Identifier. Name of the `WasmPlugin` resource in the following format: `projects/{project}/locations/{location}/wasmPlugins/{wasm_plugin}`. |
| `versions` | HashMap<String, String> |  | Optional. All versions of this `WasmPlugin` resource in the key-value format. The key is the resource ID, and the value is the `VersionDetails` object. Lets you create or update a `WasmPlugin` resource and its versions in a single request. When the `main_version_id` field is not empty, it must point to one of the `VersionDetails` objects in the map. If provided in a `PATCH` request, the new versions replace the previous set. Any version omitted from the `versions` field is removed. Because the `WasmPluginVersion` resource is immutable, if a `WasmPluginVersion` resource with the same name already exists and differs, the request fails. Note: In a `GET` request, this field is populated only if the field `GetWasmPluginRequest.view` is set to `WASM_PLUGIN_VIEW_FULL`. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `WasmPlugin` resource. The format must comply with [the following requirements](/compute/docs/labeling-resources#requirements). |
| `used_by` | Vec<String> |  | Output only. List of all [extensions](https://cloud.google.com/service-extensions/docs/overview) that use this `WasmPlugin` resource. |
| `parent` | String | ✅ | Required. The parent resource of the `WasmPlugin` resource. Must be in the format `projects/{project}/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `main_version_id` | String | Optional. The ID of the `WasmPluginVersion` resource that is the currently serving one. The version referred to must be a child of this `WasmPlugin` resource. |
| `log_config` | String | Optional. Specifies the logging options for the activity performed by this plugin. If logging is enabled, plugin logs are exported to Cloud Logging. Note that the settings relate to the logs generated by using logging statements in your Wasm code. |
| `description` | String | Optional. A human-readable description of the resource. |
| `name` | String | Identifier. Name of the `WasmPlugin` resource in the following format: `projects/{project}/locations/{location}/wasmPlugins/{wasm_plugin}`. |
| `versions` | HashMap<String, String> | Optional. All versions of this `WasmPlugin` resource in the key-value format. The key is the resource ID, and the value is the `VersionDetails` object. Lets you create or update a `WasmPlugin` resource and its versions in a single request. When the `main_version_id` field is not empty, it must point to one of the `VersionDetails` objects in the map. If provided in a `PATCH` request, the new versions replace the previous set. Any version omitted from the `versions` field is removed. Because the `WasmPluginVersion` resource is immutable, if a `WasmPluginVersion` resource with the same name already exists and differs, the request fails. Note: In a `GET` request, this field is populated only if the field `GetWasmPluginRequest.view` is set to `WASM_PLUGIN_VIEW_FULL`. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `WasmPlugin` resource. The format must comply with [the following requirements](/compute/docs/labeling-resources#requirements). |
| `used_by` | Vec<String> | Output only. List of all [extensions](https://cloud.google.com/service-extensions/docs/overview) that use this `WasmPlugin` resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create wasm_plugin
wasm_plugin = provider.networkservices_api.Wasm_plugin {
    parent = "value"  # Required. The parent resource of the `WasmPlugin` resource. Must be in the format `projects/{project}/locations/global`.
}

# Access wasm_plugin outputs
wasm_plugin_id = wasm_plugin.id
wasm_plugin_create_time = wasm_plugin.create_time
wasm_plugin_main_version_id = wasm_plugin.main_version_id
wasm_plugin_log_config = wasm_plugin.log_config
wasm_plugin_description = wasm_plugin.description
wasm_plugin_name = wasm_plugin.name
wasm_plugin_versions = wasm_plugin.versions
wasm_plugin_update_time = wasm_plugin.update_time
wasm_plugin_labels = wasm_plugin.labels
wasm_plugin_used_by = wasm_plugin.used_by
```

---


### Version

Creates a new `WasmPluginVersion` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. A human-readable description of the resource. |
| `image_digest` | String |  | Output only. This field holds the digest (usually checksum) value for the plugin image. The value is calculated based on the `image_uri` field. If the `image_uri` field refers to a container image, the digest value is obtained from the container image. If the `image_uri` field refers to a generic artifact, the digest value is calculated based on the contents of the file. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `WasmPluginVersion` resource. |
| `plugin_config_data` | String |  | Configuration for the plugin. The configuration is provided to the plugin at runtime through the `ON_CONFIGURE` callback. When a new `WasmPluginVersion` resource is created, the digest of the contents is saved in the `plugin_config_digest` field. |
| `plugin_config_uri` | String |  | URI of the plugin configuration stored in the Artifact Registry. The configuration is provided to the plugin at runtime through the `ON_CONFIGURE` callback. The URI can refer to one of the following repository formats: * Container images: the `plugin_config_uri` must point to a container that contains a single file with the name `plugin.config`. When a new `WasmPluginVersion` resource is created, the digest of the image is saved in the `plugin_config_digest` field. When pulling a container image from Artifact Registry, the digest value is used instead of an image tag. * Generic artifacts: the `plugin_config_uri` must be in this format: `projects/{project}/locations/{location}/repositories/{repository}/ genericArtifacts/{package}:{version}`. The specified package and version must contain a file with the name `plugin.config`. When a new `WasmPluginVersion` resource is created, the checksum of the contents of the file is saved in the `plugin_config_digest` field. |
| `name` | String |  | Identifier. Name of the `WasmPluginVersion` resource in the following format: `projects/{project}/locations/{location}/wasmPlugins/{wasm_plugin}/ versions/{wasm_plugin_version}`. |
| `image_uri` | String |  | Optional. URI of the image containing the Wasm module, stored in Artifact Registry. The URI can refer to one of the following repository formats: * Container images: the `image_uri` must point to a container that contains a single file with the name `plugin.wasm`. When a new `WasmPluginVersion` resource is created, the digest of the image is saved in the `image_digest` field. When pulling a container image from Artifact Registry, the digest value is used instead of an image tag. * Generic artifacts: the `image_uri` must be in this format: `projects/{project}/locations/{location}/repositories/{repository}/ genericArtifacts/{package}:{version}`. The specified package and version must contain a file with the name `plugin.wasm`. When a new `WasmPluginVersion` resource is created, the checksum of the contents of the file is saved in the `image_digest` field. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `plugin_config_digest` | String |  | Output only. This field holds the digest (usually checksum) value for the plugin configuration. The value is calculated based on the contents of `plugin_config_data` field or the image defined by the `plugin_config_uri` field. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `parent` | String | ✅ | Required. The parent resource of the `WasmPluginVersion` resource. Must be in the format `projects/{project}/locations/global/wasmPlugins/{wasm_plugin}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. A human-readable description of the resource. |
| `image_digest` | String | Output only. This field holds the digest (usually checksum) value for the plugin image. The value is calculated based on the `image_uri` field. If the `image_uri` field refers to a container image, the digest value is obtained from the container image. If the `image_uri` field refers to a generic artifact, the digest value is calculated based on the contents of the file. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `WasmPluginVersion` resource. |
| `plugin_config_data` | String | Configuration for the plugin. The configuration is provided to the plugin at runtime through the `ON_CONFIGURE` callback. When a new `WasmPluginVersion` resource is created, the digest of the contents is saved in the `plugin_config_digest` field. |
| `plugin_config_uri` | String | URI of the plugin configuration stored in the Artifact Registry. The configuration is provided to the plugin at runtime through the `ON_CONFIGURE` callback. The URI can refer to one of the following repository formats: * Container images: the `plugin_config_uri` must point to a container that contains a single file with the name `plugin.config`. When a new `WasmPluginVersion` resource is created, the digest of the image is saved in the `plugin_config_digest` field. When pulling a container image from Artifact Registry, the digest value is used instead of an image tag. * Generic artifacts: the `plugin_config_uri` must be in this format: `projects/{project}/locations/{location}/repositories/{repository}/ genericArtifacts/{package}:{version}`. The specified package and version must contain a file with the name `plugin.config`. When a new `WasmPluginVersion` resource is created, the checksum of the contents of the file is saved in the `plugin_config_digest` field. |
| `name` | String | Identifier. Name of the `WasmPluginVersion` resource in the following format: `projects/{project}/locations/{location}/wasmPlugins/{wasm_plugin}/ versions/{wasm_plugin_version}`. |
| `image_uri` | String | Optional. URI of the image containing the Wasm module, stored in Artifact Registry. The URI can refer to one of the following repository formats: * Container images: the `image_uri` must point to a container that contains a single file with the name `plugin.wasm`. When a new `WasmPluginVersion` resource is created, the digest of the image is saved in the `image_digest` field. When pulling a container image from Artifact Registry, the digest value is used instead of an image tag. * Generic artifacts: the `image_uri` must be in this format: `projects/{project}/locations/{location}/repositories/{repository}/ genericArtifacts/{package}:{version}`. The specified package and version must contain a file with the name `plugin.wasm`. When a new `WasmPluginVersion` resource is created, the checksum of the contents of the file is saved in the `image_digest` field. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `plugin_config_digest` | String | Output only. This field holds the digest (usually checksum) value for the plugin configuration. The value is calculated based on the contents of `plugin_config_data` field or the image defined by the `plugin_config_uri` field. |
| `create_time` | String | Output only. The timestamp when the resource was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.networkservices_api.Version {
    parent = "value"  # Required. The parent resource of the `WasmPluginVersion` resource. Must be in the format `projects/{project}/locations/global/wasmPlugins/{wasm_plugin}`.
}

# Access version outputs
version_id = version.id
version_description = version.description
version_image_digest = version.image_digest
version_labels = version.labels
version_plugin_config_data = version.plugin_config_data
version_plugin_config_uri = version.plugin_config_uri
version_name = version.name
version_image_uri = version.image_uri
version_update_time = version.update_time
version_plugin_config_digest = version.plugin_config_digest
version_create_time = version.create_time
```

---


### Edge_cache_service

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create edge_cache_service
edge_cache_service = provider.networkservices_api.Edge_cache_service {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access edge_cache_service outputs
edge_cache_service_id = edge_cache_service.id
edge_cache_service_bindings = edge_cache_service.bindings
edge_cache_service_version = edge_cache_service.version
edge_cache_service_audit_configs = edge_cache_service.audit_configs
edge_cache_service_etag = edge_cache_service.etag
```

---


### Lb_edge_extension

Creates a new `LbEdgeExtension` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `extension_chains` | Vec<String> |  | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `forwarding_rules` | Vec<String> |  | Required. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbEdgeExtension` resource can be associated with a forwarding rule. |
| `name` | String |  | Required. Identifier. Name of the `LbEdgeExtension` resource in the following format: `projects/{project}/locations/{location}/lbEdgeExtensions/{lb_edge_extension}`. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `LbEdgeExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `load_balancing_scheme` | String |  | Required. All forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `EXTERNAL_MANAGED`. |
| `parent` | String | ✅ | Required. The parent resource of the `LbEdgeExtension` resource. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `extension_chains` | Vec<String> | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `forwarding_rules` | Vec<String> | Required. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbEdgeExtension` resource can be associated with a forwarding rule. |
| `name` | String | Required. Identifier. Name of the `LbEdgeExtension` resource in the following format: `projects/{project}/locations/{location}/lbEdgeExtensions/{lb_edge_extension}`. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `LbEdgeExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `description` | String | Optional. A human-readable description of the resource. |
| `load_balancing_scheme` | String | Required. All forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `EXTERNAL_MANAGED`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lb_edge_extension
lb_edge_extension = provider.networkservices_api.Lb_edge_extension {
    parent = "value"  # Required. The parent resource of the `LbEdgeExtension` resource. Must be in the format `projects/{project}/locations/{location}`.
}

# Access lb_edge_extension outputs
lb_edge_extension_id = lb_edge_extension.id
lb_edge_extension_update_time = lb_edge_extension.update_time
lb_edge_extension_create_time = lb_edge_extension.create_time
lb_edge_extension_extension_chains = lb_edge_extension.extension_chains
lb_edge_extension_forwarding_rules = lb_edge_extension.forwarding_rules
lb_edge_extension_name = lb_edge_extension.name
lb_edge_extension_labels = lb_edge_extension.labels
lb_edge_extension_description = lb_edge_extension.description
lb_edge_extension_load_balancing_scheme = lb_edge_extension.load_balancing_scheme
```

---


### Authz_extension

Creates a new `AuthzExtension` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `fail_open` | bool |  | Optional. Determines how the proxy behaves if the call to the extension fails or times out. When set to `TRUE`, request or response processing continues without error. Any subsequent extensions in the extension chain are also executed. When set to `FALSE` or the default setting of `FALSE` is used, one of the following happens: * If response headers have not been delivered to the downstream client, a generic 500 error is returned to the client. The error response can be tailored by configuring a custom error response in the load balancer. * If response headers have been delivered, then the HTTP stream to the downstream client is reset. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `timeout` | String |  | Required. Specifies the timeout for each individual message on the stream. The timeout must be between 10-10000 milliseconds. |
| `authority` | String |  | Required. The `:authority` header in the gRPC request sent from Envoy to the extension service. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `name` | String |  | Required. Identifier. Name of the `AuthzExtension` resource in the following format: `projects/{project}/locations/{location}/authzExtensions/{authz_extension}`. |
| `forward_headers` | Vec<String> |  | Optional. List of the HTTP headers to forward to the extension (from the client). If omitted, all headers are sent. Each element is a string indicating the header name. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `load_balancing_scheme` | String |  | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `AuthzExtension` resource. The format must comply with [the requirements for labels](/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `metadata` | HashMap<String, String> |  | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata is available under the namespace `com.google.authz_extension.`. The following variables are supported in the metadata Struct: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. |
| `service` | String |  | Required. The reference to the service that runs the extension. To configure a callout extension, `service` must be a fully-qualified reference to a [backend service](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices) in the format: `https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/backendServices/{backendService}` or `https://www.googleapis.com/compute/v1/projects/{project}/global/backendServices/{backendService}`. |
| `wire_format` | String |  | Optional. The format of communication supported by the callout extension. If not specified, the default value `EXT_PROC_GRPC` is used. |
| `parent` | String | ✅ | Required. The parent resource of the `AuthzExtension` resource. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `fail_open` | bool | Optional. Determines how the proxy behaves if the call to the extension fails or times out. When set to `TRUE`, request or response processing continues without error. Any subsequent extensions in the extension chain are also executed. When set to `FALSE` or the default setting of `FALSE` is used, one of the following happens: * If response headers have not been delivered to the downstream client, a generic 500 error is returned to the client. The error response can be tailored by configuring a custom error response in the load balancer. * If response headers have been delivered, then the HTTP stream to the downstream client is reset. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `timeout` | String | Required. Specifies the timeout for each individual message on the stream. The timeout must be between 10-10000 milliseconds. |
| `authority` | String | Required. The `:authority` header in the gRPC request sent from Envoy to the extension service. |
| `description` | String | Optional. A human-readable description of the resource. |
| `name` | String | Required. Identifier. Name of the `AuthzExtension` resource in the following format: `projects/{project}/locations/{location}/authzExtensions/{authz_extension}`. |
| `forward_headers` | Vec<String> | Optional. List of the HTTP headers to forward to the extension (from the client). If omitted, all headers are sent. Each element is a string indicating the header name. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `load_balancing_scheme` | String | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `AuthzExtension` resource. The format must comply with [the requirements for labels](/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `metadata` | HashMap<String, String> | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata is available under the namespace `com.google.authz_extension.`. The following variables are supported in the metadata Struct: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. |
| `service` | String | Required. The reference to the service that runs the extension. To configure a callout extension, `service` must be a fully-qualified reference to a [backend service](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices) in the format: `https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/backendServices/{backendService}` or `https://www.googleapis.com/compute/v1/projects/{project}/global/backendServices/{backendService}`. |
| `wire_format` | String | Optional. The format of communication supported by the callout extension. If not specified, the default value `EXT_PROC_GRPC` is used. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authz_extension
authz_extension = provider.networkservices_api.Authz_extension {
    parent = "value"  # Required. The parent resource of the `AuthzExtension` resource. Must be in the format `projects/{project}/locations/{location}`.
}

# Access authz_extension outputs
authz_extension_id = authz_extension.id
authz_extension_fail_open = authz_extension.fail_open
authz_extension_create_time = authz_extension.create_time
authz_extension_timeout = authz_extension.timeout
authz_extension_authority = authz_extension.authority
authz_extension_description = authz_extension.description
authz_extension_name = authz_extension.name
authz_extension_forward_headers = authz_extension.forward_headers
authz_extension_update_time = authz_extension.update_time
authz_extension_load_balancing_scheme = authz_extension.load_balancing_scheme
authz_extension_labels = authz_extension.labels
authz_extension_metadata = authz_extension.metadata
authz_extension_service = authz_extension.service
authz_extension_wire_format = authz_extension.wire_format
```

---


### Edge_cache_origin

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create edge_cache_origin
edge_cache_origin = provider.networkservices_api.Edge_cache_origin {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access edge_cache_origin outputs
edge_cache_origin_id = edge_cache_origin.id
edge_cache_origin_bindings = edge_cache_origin.bindings
edge_cache_origin_version = edge_cache_origin.version
edge_cache_origin_audit_configs = edge_cache_origin.audit_configs
edge_cache_origin_etag = edge_cache_origin.etag
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
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_name = location.name
location_location_id = location.location_id
location_metadata = location.metadata
location_labels = location.labels
location_display_name = location.display_name
```

---


### Edge_cache_keyset

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create edge_cache_keyset
edge_cache_keyset = provider.networkservices_api.Edge_cache_keyset {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access edge_cache_keyset outputs
edge_cache_keyset_id = edge_cache_keyset.id
edge_cache_keyset_bindings = edge_cache_keyset.bindings
edge_cache_keyset_version = edge_cache_keyset.version
edge_cache_keyset_audit_configs = edge_cache_keyset.audit_configs
edge_cache_keyset_etag = edge_cache_keyset.etag
```

---


### Meshe

Creates a new Mesh in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the Mesh resource. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `name` | String |  | Identifier. Name of the Mesh resource. It matches pattern `projects/*/locations/global/meshes/`. |
| `envoy_headers` | String |  | Optional. Determines if envoy will insert internal debug headers into upstream requests. Other Envoy headers may still be injected. By default, envoy will not insert any debug headers. |
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `interception_port` | i64 |  | Optional. If set to a valid TCP port (1-65535), instructs the SIDECAR proxy to listen on the specified port of localhost (127.0.0.1) address. The SIDECAR proxy will expect all traffic to be redirected to this port regardless of its actual ip:port destination. If unset, a port '15001' is used as the interception port. This is applicable only for sidecar proxy deployments. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `parent` | String | ✅ | Required. The parent resource of the Mesh. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the Mesh resource. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `name` | String | Identifier. Name of the Mesh resource. It matches pattern `projects/*/locations/global/meshes/`. |
| `envoy_headers` | String | Optional. Determines if envoy will insert internal debug headers into upstream requests. Other Envoy headers may still be injected. By default, envoy will not insert any debug headers. |
| `self_link` | String | Output only. Server-defined URL of this resource |
| `interception_port` | i64 | Optional. If set to a valid TCP port (1-65535), instructs the SIDECAR proxy to listen on the specified port of localhost (127.0.0.1) address. The SIDECAR proxy will expect all traffic to be redirected to this port regardless of its actual ip:port destination. If unset, a port '15001' is used as the interception port. This is applicable only for sidecar proxy deployments. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create meshe
meshe = provider.networkservices_api.Meshe {
    parent = "value"  # Required. The parent resource of the Mesh. Must be in the format `projects/*/locations/global`.
}

# Access meshe outputs
meshe_id = meshe.id
meshe_labels = meshe.labels
meshe_create_time = meshe.create_time
meshe_description = meshe.description
meshe_name = meshe.name
meshe_envoy_headers = meshe.envoy_headers
meshe_self_link = meshe.self_link
meshe_interception_port = meshe.interception_port
meshe_update_time = meshe.update_time
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
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
operation = provider.networkservices_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_done = operation.done
operation_name = operation.name
operation_metadata = operation.metadata
operation_response = operation.response
```

---


### Grpc_route

Creates a new GrpcRoute in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Name of the GrpcRoute resource. It matches pattern `projects/*/locations/global/grpcRoutes/` |
| `gateways` | Vec<String> |  | Optional. Gateways defines a list of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the GrpcRoute resource. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `rules` | Vec<String> |  | Required. A list of detailed rules defining how to route traffic. Within a single GrpcRoute, the GrpcRoute.RouteAction associated with the first matching GrpcRoute.RouteRule will be executed. At least one rule must be supplied. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `hostnames` | Vec<String> |  | Required. Service hostnames with an optional port for which this route describes traffic. Format: [:] Hostname is the fully qualified domain name of a network host. This matches the RFC 1123 definition of a hostname with 2 notable exceptions: - IPs are not allowed. - A hostname may be prefixed with a wildcard label (`*.`). The wildcard label must appear by itself as the first label. Hostname can be "precise" which is a domain name without the terminating dot of a network host (e.g. `foo.example.com`) or "wildcard", which is a domain name prefixed with a single wildcard label (e.g. `*.example.com`). Note that as per RFC1035 and RFC1123, a label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character. No other punctuation is allowed. The routes associated with a Mesh or Gateway must have unique hostnames. If you attempt to attach multiple routes with conflicting hostnames, the configuration will be rejected. For example, while it is acceptable for routes for the hostnames `*.foo.bar.com` and `*.bar.com` to be associated with the same route, it is not possible to associate two routes both with `*.bar.com` or both with `bar.com`. If a port is specified, then gRPC clients must use the channel URI with the port to match this rule (i.e. "xds:///service:123"), otherwise they must supply the URI without a port (i.e. "xds:///service"). |
| `meshes` | Vec<String> |  | Optional. Meshes defines a list of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` |
| `parent` | String | ✅ | Required. The parent resource of the GrpcRoute. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Name of the GrpcRoute resource. It matches pattern `projects/*/locations/global/grpcRoutes/` |
| `gateways` | Vec<String> | Optional. Gateways defines a list of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `self_link` | String | Output only. Server-defined URL of this resource |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the GrpcRoute resource. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `rules` | Vec<String> | Required. A list of detailed rules defining how to route traffic. Within a single GrpcRoute, the GrpcRoute.RouteAction associated with the first matching GrpcRoute.RouteRule will be executed. At least one rule must be supplied. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `hostnames` | Vec<String> | Required. Service hostnames with an optional port for which this route describes traffic. Format: [:] Hostname is the fully qualified domain name of a network host. This matches the RFC 1123 definition of a hostname with 2 notable exceptions: - IPs are not allowed. - A hostname may be prefixed with a wildcard label (`*.`). The wildcard label must appear by itself as the first label. Hostname can be "precise" which is a domain name without the terminating dot of a network host (e.g. `foo.example.com`) or "wildcard", which is a domain name prefixed with a single wildcard label (e.g. `*.example.com`). Note that as per RFC1035 and RFC1123, a label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character. No other punctuation is allowed. The routes associated with a Mesh or Gateway must have unique hostnames. If you attempt to attach multiple routes with conflicting hostnames, the configuration will be rejected. For example, while it is acceptable for routes for the hostnames `*.foo.bar.com` and `*.bar.com` to be associated with the same route, it is not possible to associate two routes both with `*.bar.com` or both with `bar.com`. If a port is specified, then gRPC clients must use the channel URI with the port to match this rule (i.e. "xds:///service:123"), otherwise they must supply the URI without a port (i.e. "xds:///service"). |
| `meshes` | Vec<String> | Optional. Meshes defines a list of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create grpc_route
grpc_route = provider.networkservices_api.Grpc_route {
    parent = "value"  # Required. The parent resource of the GrpcRoute. Must be in the format `projects/*/locations/global`.
}

# Access grpc_route outputs
grpc_route_id = grpc_route.id
grpc_route_name = grpc_route.name
grpc_route_gateways = grpc_route.gateways
grpc_route_self_link = grpc_route.self_link
grpc_route_create_time = grpc_route.create_time
grpc_route_labels = grpc_route.labels
grpc_route_description = grpc_route.description
grpc_route_rules = grpc_route.rules
grpc_route_update_time = grpc_route.update_time
grpc_route_hostnames = grpc_route.hostnames
grpc_route_meshes = grpc_route.meshes
```

---


### Http_route

Creates a new HttpRoute in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `rules` | Vec<String> |  | Required. Rules that define how traffic is routed and handled. Rules will be matched sequentially based on the RouteMatch specified for the rule. |
| `meshes` | Vec<String> |  | Optional. Meshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the HttpRoute resource. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `hostnames` | Vec<String> |  | Required. Hostnames define a set of hosts that should match against the HTTP host header to select a HttpRoute to process the request. Hostname is the fully qualified domain name of a network host, as defined by RFC 1123 with the exception that: - IPs are not allowed. - A hostname may be prefixed with a wildcard label (`*.`). The wildcard label must appear by itself as the first label. Hostname can be "precise" which is a domain name without the terminating dot of a network host (e.g. `foo.example.com`) or "wildcard", which is a domain name prefixed with a single wildcard label (e.g. `*.example.com`). Note that as per RFC1035 and RFC1123, a label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character. No other punctuation is allowed. The routes associated with a Mesh or Gateways must have unique hostnames. If you attempt to attach multiple routes with conflicting hostnames, the configuration will be rejected. For example, while it is acceptable for routes for the hostnames `*.foo.bar.com` and `*.bar.com` to be associated with the same Mesh (or Gateways under the same scope), it is not possible to associate two routes both with `*.bar.com` or both with `bar.com`. |
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `gateways` | Vec<String> |  | Optional. Gateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `name` | String |  | Identifier. Name of the HttpRoute resource. It matches pattern `projects/*/locations/global/httpRoutes/http_route_name>`. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `parent` | String | ✅ | Required. The parent resource of the HttpRoute. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `rules` | Vec<String> | Required. Rules that define how traffic is routed and handled. Rules will be matched sequentially based on the RouteMatch specified for the rule. |
| `meshes` | Vec<String> | Optional. Meshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the HttpRoute resource. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `hostnames` | Vec<String> | Required. Hostnames define a set of hosts that should match against the HTTP host header to select a HttpRoute to process the request. Hostname is the fully qualified domain name of a network host, as defined by RFC 1123 with the exception that: - IPs are not allowed. - A hostname may be prefixed with a wildcard label (`*.`). The wildcard label must appear by itself as the first label. Hostname can be "precise" which is a domain name without the terminating dot of a network host (e.g. `foo.example.com`) or "wildcard", which is a domain name prefixed with a single wildcard label (e.g. `*.example.com`). Note that as per RFC1035 and RFC1123, a label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character. No other punctuation is allowed. The routes associated with a Mesh or Gateways must have unique hostnames. If you attempt to attach multiple routes with conflicting hostnames, the configuration will be rejected. For example, while it is acceptable for routes for the hostnames `*.foo.bar.com` and `*.bar.com` to be associated with the same Mesh (or Gateways under the same scope), it is not possible to associate two routes both with `*.bar.com` or both with `bar.com`. |
| `self_link` | String | Output only. Server-defined URL of this resource |
| `gateways` | Vec<String> | Optional. Gateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `name` | String | Identifier. Name of the HttpRoute resource. It matches pattern `projects/*/locations/global/httpRoutes/http_route_name>`. |
| `create_time` | String | Output only. The timestamp when the resource was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create http_route
http_route = provider.networkservices_api.Http_route {
    parent = "value"  # Required. The parent resource of the HttpRoute. Must be in the format `projects/*/locations/global`.
}

# Access http_route outputs
http_route_id = http_route.id
http_route_update_time = http_route.update_time
http_route_rules = http_route.rules
http_route_meshes = http_route.meshes
http_route_labels = http_route.labels
http_route_description = http_route.description
http_route_hostnames = http_route.hostnames
http_route_self_link = http_route.self_link
http_route_gateways = http_route.gateways
http_route_name = http_route.name
http_route_create_time = http_route.create_time
```

---


### Lb_route_extension

Creates a new `LbRouteExtension` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `LbRouteExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `metadata` | HashMap<String, String> |  | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata applies to all extensions in all extensions chains in this resource. The metadata is available under the key `com.google.lb_route_extension.`. The following variables are supported in the metadata: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. This field must not be set if at least one of the extension chains contains plugin extensions. Setting it results in a validation error. You can set metadata at either the resource level or the extension level. The extension level metadata is recommended because you can pass a different set of metadata through each extension to the backend. |
| `name` | String |  | Required. Identifier. Name of the `LbRouteExtension` resource in the following format: `projects/{project}/locations/{location}/lbRouteExtensions/{lb_route_extension}`. |
| `extension_chains` | Vec<String> |  | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `forwarding_rules` | Vec<String> |  | Required. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbRouteExtension` resource can be associated with a forwarding rule. |
| `load_balancing_scheme` | String |  | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `parent` | String | ✅ | Required. The parent resource of the `LbRouteExtension` resource. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `LbRouteExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `description` | String | Optional. A human-readable description of the resource. |
| `metadata` | HashMap<String, String> | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata applies to all extensions in all extensions chains in this resource. The metadata is available under the key `com.google.lb_route_extension.`. The following variables are supported in the metadata: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. This field must not be set if at least one of the extension chains contains plugin extensions. Setting it results in a validation error. You can set metadata at either the resource level or the extension level. The extension level metadata is recommended because you can pass a different set of metadata through each extension to the backend. |
| `name` | String | Required. Identifier. Name of the `LbRouteExtension` resource in the following format: `projects/{project}/locations/{location}/lbRouteExtensions/{lb_route_extension}`. |
| `extension_chains` | Vec<String> | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `forwarding_rules` | Vec<String> | Required. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbRouteExtension` resource can be associated with a forwarding rule. |
| `load_balancing_scheme` | String | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lb_route_extension
lb_route_extension = provider.networkservices_api.Lb_route_extension {
    parent = "value"  # Required. The parent resource of the `LbRouteExtension` resource. Must be in the format `projects/{project}/locations/{location}`.
}

# Access lb_route_extension outputs
lb_route_extension_id = lb_route_extension.id
lb_route_extension_labels = lb_route_extension.labels
lb_route_extension_update_time = lb_route_extension.update_time
lb_route_extension_create_time = lb_route_extension.create_time
lb_route_extension_description = lb_route_extension.description
lb_route_extension_metadata = lb_route_extension.metadata
lb_route_extension_name = lb_route_extension.name
lb_route_extension_extension_chains = lb_route_extension.extension_chains
lb_route_extension_forwarding_rules = lb_route_extension.forwarding_rules
lb_route_extension_load_balancing_scheme = lb_route_extension.load_balancing_scheme
```

---


### Service_binding

Creates a new ServiceBinding in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `service` | String |  | Optional. The full Service Directory Service name of the format `projects/*/locations/*/namespaces/*/services/*`. This field is for Service Directory integration which will be deprecated soon. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the ServiceBinding resource. |
| `name` | String |  | Identifier. Name of the ServiceBinding resource. It matches pattern `projects/*/locations/*/serviceBindings/`. |
| `service_id` | String |  | Output only. The unique identifier of the Service Directory Service against which the ServiceBinding resource is validated. This is populated when the Service Binding resource is used in another resource (like Backend Service). This is of the UUID4 format. This field is for Service Directory integration which will be deprecated soon. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `parent` | String | ✅ | Required. The parent resource of the ServiceBinding. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service` | String | Optional. The full Service Directory Service name of the format `projects/*/locations/*/namespaces/*/services/*`. This field is for Service Directory integration which will be deprecated soon. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the ServiceBinding resource. |
| `name` | String | Identifier. Name of the ServiceBinding resource. It matches pattern `projects/*/locations/*/serviceBindings/`. |
| `service_id` | String | Output only. The unique identifier of the Service Directory Service against which the ServiceBinding resource is validated. This is populated when the Service Binding resource is used in another resource (like Backend Service). This is of the UUID4 format. This field is for Service Directory integration which will be deprecated soon. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_binding
service_binding = provider.networkservices_api.Service_binding {
    parent = "value"  # Required. The parent resource of the ServiceBinding. Must be in the format `projects/*/locations/*`.
}

# Access service_binding outputs
service_binding_id = service_binding.id
service_binding_service = service_binding.service
service_binding_update_time = service_binding.update_time
service_binding_labels = service_binding.labels
service_binding_name = service_binding.name
service_binding_service_id = service_binding.service_id
service_binding_create_time = service_binding.create_time
service_binding_description = service_binding.description
```

---


### Tcp_route

Creates a new TcpRoute in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `rules` | Vec<String> |  | Required. Rules that define how traffic is routed and handled. At least one RouteRule must be supplied. If there are multiple rules then the action taken will be the first rule to match. |
| `gateways` | Vec<String> |  | Optional. Gateways defines a list of gateways this TcpRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the TcpRoute resource. |
| `name` | String |  | Identifier. Name of the TcpRoute resource. It matches pattern `projects/*/locations/global/tcpRoutes/tcp_route_name>`. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `meshes` | Vec<String> |  | Optional. Meshes defines a list of meshes this TcpRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `parent` | String | ✅ | Required. The parent resource of the TcpRoute. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | Output only. Server-defined URL of this resource |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `rules` | Vec<String> | Required. Rules that define how traffic is routed and handled. At least one RouteRule must be supplied. If there are multiple rules then the action taken will be the first rule to match. |
| `gateways` | Vec<String> | Optional. Gateways defines a list of gateways this TcpRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the TcpRoute resource. |
| `name` | String | Identifier. Name of the TcpRoute resource. It matches pattern `projects/*/locations/global/tcpRoutes/tcp_route_name>`. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `meshes` | Vec<String> | Optional. Meshes defines a list of meshes this TcpRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tcp_route
tcp_route = provider.networkservices_api.Tcp_route {
    parent = "value"  # Required. The parent resource of the TcpRoute. Must be in the format `projects/*/locations/global`.
}

# Access tcp_route outputs
tcp_route_id = tcp_route.id
tcp_route_self_link = tcp_route.self_link
tcp_route_create_time = tcp_route.create_time
tcp_route_rules = tcp_route.rules
tcp_route_gateways = tcp_route.gateways
tcp_route_labels = tcp_route.labels
tcp_route_name = tcp_route.name
tcp_route_update_time = tcp_route.update_time
tcp_route_meshes = tcp_route.meshes
tcp_route_description = tcp_route.description
```

---


### Route_view

Get a single RouteView of a Mesh.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `route_id` | String | Output only. The resource id for the route. |
| `route_type` | String | Output only. Type of the route: HttpRoute,GrpcRoute,TcpRoute, or TlsRoute |
| `route_project_number` | String | Output only. Project number where the route exists. |
| `name` | String | Output only. Identifier. Full path name of the MeshRouteView resource. Format: projects/{project_number}/locations/{location}/meshes/{mesh}/routeViews/{route_view} |
| `route_location` | String | Output only. Location where the route exists. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access route_view outputs
route_view_id = route_view.id
route_view_route_id = route_view.route_id
route_view_route_type = route_view.route_type
route_view_route_project_number = route_view.route_project_number
route_view_name = route_view.name
route_view_route_location = route_view.route_location
```

---


### Gateway

Creates a new Gateway in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `ip_version` | String |  | Optional. The IP Version that will be used by this gateway. Valid options are IPV4 or IPV6. Default is IPV4. |
| `scope` | String |  | Optional. Scope determines how configuration across multiple Gateway instances are merged. The configuration for multiple Gateway instances with the same scope will be merged as presented as a single configuration to the proxy/load balancer. Max length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the Gateway resource. |
| `ports` | Vec<i64> |  | Required. One or more port numbers (1-65535), on which the Gateway will receive traffic. The proxy binds to the specified ports. Gateways of type 'SECURE_WEB_GATEWAY' are limited to 5 ports. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6 and support multiple ports. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `network` | String |  | Optional. The relative resource name identifying the VPC network that is using this configuration. For example: `projects/*/global/networks/network-1`. Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'. |
| `subnetwork` | String |  | Optional. The relative resource name identifying the subnetwork in which this SWG is allocated. For example: `projects/*/regions/us-central1/subnetworks/network-1` Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY". |
| `addresses` | Vec<String> |  | Optional. Zero or one IPv4 or IPv6 address on which the Gateway will receive the traffic. When no address is provided, an IP from the subnetwork is allocated This field only applies to gateways of type 'SECURE_WEB_GATEWAY'. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `routing_mode` | String |  | Optional. The routing mode of the Gateway. This field is configurable only for gateways of type SECURE_WEB_GATEWAY. This field is required for gateways of type SECURE_WEB_GATEWAY. |
| `envoy_headers` | String |  | Optional. Determines if envoy will insert internal debug headers into upstream requests. Other Envoy headers may still be injected. By default, envoy will not insert any debug headers. |
| `type` | String |  | Immutable. The type of the customer managed gateway. This field is required. If unspecified, an error is returned. |
| `name` | String |  | Identifier. Name of the Gateway resource. It matches pattern `projects/*/locations/*/gateways/`. |
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `certificate_urls` | Vec<String> |  | Optional. A fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection. This feature only applies to gateways of type 'SECURE_WEB_GATEWAY'. |
| `server_tls_policy` | String |  | Optional. A fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated. If empty, TLS termination is disabled. |
| `gateway_security_policy` | String |  | Optional. A fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections. For example: `projects/*/locations/*/gatewaySecurityPolicies/swg-policy`. This policy is specific to gateways of type 'SECURE_WEB_GATEWAY'. |
| `parent` | String | ✅ | Required. The parent resource of the Gateway. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `ip_version` | String | Optional. The IP Version that will be used by this gateway. Valid options are IPV4 or IPV6. Default is IPV4. |
| `scope` | String | Optional. Scope determines how configuration across multiple Gateway instances are merged. The configuration for multiple Gateway instances with the same scope will be merged as presented as a single configuration to the proxy/load balancer. Max length 64 characters. Scope should start with a letter and can only have letters, numbers, hyphens. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the Gateway resource. |
| `ports` | Vec<i64> | Required. One or more port numbers (1-65535), on which the Gateway will receive traffic. The proxy binds to the specified ports. Gateways of type 'SECURE_WEB_GATEWAY' are limited to 5 ports. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6 and support multiple ports. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `network` | String | Optional. The relative resource name identifying the VPC network that is using this configuration. For example: `projects/*/global/networks/network-1`. Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY'. |
| `subnetwork` | String | Optional. The relative resource name identifying the subnetwork in which this SWG is allocated. For example: `projects/*/regions/us-central1/subnetworks/network-1` Currently, this field is specific to gateways of type 'SECURE_WEB_GATEWAY". |
| `addresses` | Vec<String> | Optional. Zero or one IPv4 or IPv6 address on which the Gateway will receive the traffic. When no address is provided, an IP from the subnetwork is allocated This field only applies to gateways of type 'SECURE_WEB_GATEWAY'. Gateways of type 'OPEN_MESH' listen on 0.0.0.0 for IPv4 and :: for IPv6. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `routing_mode` | String | Optional. The routing mode of the Gateway. This field is configurable only for gateways of type SECURE_WEB_GATEWAY. This field is required for gateways of type SECURE_WEB_GATEWAY. |
| `envoy_headers` | String | Optional. Determines if envoy will insert internal debug headers into upstream requests. Other Envoy headers may still be injected. By default, envoy will not insert any debug headers. |
| `type` | String | Immutable. The type of the customer managed gateway. This field is required. If unspecified, an error is returned. |
| `name` | String | Identifier. Name of the Gateway resource. It matches pattern `projects/*/locations/*/gateways/`. |
| `self_link` | String | Output only. Server-defined URL of this resource |
| `certificate_urls` | Vec<String> | Optional. A fully-qualified Certificates URL reference. The proxy presents a Certificate (selected based on SNI) when establishing a TLS connection. This feature only applies to gateways of type 'SECURE_WEB_GATEWAY'. |
| `server_tls_policy` | String | Optional. A fully-qualified ServerTLSPolicy URL reference. Specifies how TLS traffic is terminated. If empty, TLS termination is disabled. |
| `gateway_security_policy` | String | Optional. A fully-qualified GatewaySecurityPolicy URL reference. Defines how a server should apply security policy to inbound (VM to Proxy) initiated connections. For example: `projects/*/locations/*/gatewaySecurityPolicies/swg-policy`. This policy is specific to gateways of type 'SECURE_WEB_GATEWAY'. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create gateway
gateway = provider.networkservices_api.Gateway {
    parent = "value"  # Required. The parent resource of the Gateway. Must be in the format `projects/*/locations/*`.
}

# Access gateway outputs
gateway_id = gateway.id
gateway_create_time = gateway.create_time
gateway_ip_version = gateway.ip_version
gateway_scope = gateway.scope
gateway_labels = gateway.labels
gateway_ports = gateway.ports
gateway_description = gateway.description
gateway_network = gateway.network
gateway_subnetwork = gateway.subnetwork
gateway_addresses = gateway.addresses
gateway_update_time = gateway.update_time
gateway_routing_mode = gateway.routing_mode
gateway_envoy_headers = gateway.envoy_headers
gateway_type = gateway.type
gateway_name = gateway.name
gateway_self_link = gateway.self_link
gateway_certificate_urls = gateway.certificate_urls
gateway_server_tls_policy = gateway.server_tls_policy
gateway_gateway_security_policy = gateway.gateway_security_policy
```

---


### Lb_traffic_extension

Creates a new `LbTrafficExtension` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `extension_chains` | Vec<String> |  | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `name` | String |  | Required. Identifier. Name of the `LbTrafficExtension` resource in the following format: `projects/{project}/locations/{location}/lbTrafficExtensions/{lb_traffic_extension}`. |
| `metadata` | HashMap<String, String> |  | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata applies to all extensions in all extensions chains in this resource. The metadata is available under the key `com.google.lb_traffic_extension.`. The following variables are supported in the metadata: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. This field must not be set if at least one of the extension chains contains plugin extensions. Setting it results in a validation error. You can set metadata at either the resource level or the extension level. The extension level metadata is recommended because you can pass a different set of metadata through each extension to the backend. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `load_balancing_scheme` | String |  | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED` and `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `forwarding_rules` | Vec<String> |  | Optional. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbTrafficExtension` resource can be associated with a forwarding rule. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `LbTrafficExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `parent` | String | ✅ | Required. The parent resource of the `LbTrafficExtension` resource. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `extension_chains` | Vec<String> | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `name` | String | Required. Identifier. Name of the `LbTrafficExtension` resource in the following format: `projects/{project}/locations/{location}/lbTrafficExtensions/{lb_traffic_extension}`. |
| `metadata` | HashMap<String, String> | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata applies to all extensions in all extensions chains in this resource. The metadata is available under the key `com.google.lb_traffic_extension.`. The following variables are supported in the metadata: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. This field must not be set if at least one of the extension chains contains plugin extensions. Setting it results in a validation error. You can set metadata at either the resource level or the extension level. The extension level metadata is recommended because you can pass a different set of metadata through each extension to the backend. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `load_balancing_scheme` | String | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED` and `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `forwarding_rules` | Vec<String> | Optional. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbTrafficExtension` resource can be associated with a forwarding rule. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `LbTrafficExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `description` | String | Optional. A human-readable description of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lb_traffic_extension
lb_traffic_extension = provider.networkservices_api.Lb_traffic_extension {
    parent = "value"  # Required. The parent resource of the `LbTrafficExtension` resource. Must be in the format `projects/{project}/locations/{location}`.
}

# Access lb_traffic_extension outputs
lb_traffic_extension_id = lb_traffic_extension.id
lb_traffic_extension_extension_chains = lb_traffic_extension.extension_chains
lb_traffic_extension_name = lb_traffic_extension.name
lb_traffic_extension_metadata = lb_traffic_extension.metadata
lb_traffic_extension_update_time = lb_traffic_extension.update_time
lb_traffic_extension_load_balancing_scheme = lb_traffic_extension.load_balancing_scheme
lb_traffic_extension_forwarding_rules = lb_traffic_extension.forwarding_rules
lb_traffic_extension_create_time = lb_traffic_extension.create_time
lb_traffic_extension_labels = lb_traffic_extension.labels
lb_traffic_extension_description = lb_traffic_extension.description
```

---


### Endpoint_policie

Creates a new EndpointPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `authorization_policy` | String |  | Optional. This field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic at the matched endpoints. Refer to Authorization. If this field is not specified, authorization is disabled(no authz checks) for this endpoint. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `endpoint_matcher` | String |  | Required. A matcher that selects endpoints to which the policies should be applied. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the EndpointPolicy resource. |
| `traffic_port_selector` | String |  | Optional. Port selector for the (matched) endpoints. If no port selector is provided, the matched config is applied to all ports. |
| `client_tls_policy` | String |  | Optional. A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from the proxy to the actual endpoints. More specifically, it is applied to the outgoing traffic from the proxy to the endpoint. This is typically used for sidecar model where the proxy identifies itself as endpoint to the control plane, with the connection between sidecar and endpoint requiring authentication. If this field is not set, authentication is disabled(open). Applicable only when EndpointPolicyType is SIDECAR_PROXY. |
| `name` | String |  | Identifier. Name of the EndpointPolicy resource. It matches pattern `projects/{project}/locations/global/endpointPolicies/{endpoint_policy}`. |
| `type` | String |  | Required. The type of endpoint policy. This is primarily used to validate the configuration. |
| `server_tls_policy` | String |  | Optional. A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be applied to terminate the inbound traffic at the identified backends. If this field is not set, authentication is disabled(open) for this endpoint. |
| `security_policy` | String |  | Optional. A URL referring to a SecurityPolicy resource. SecurityPolicy is used to enforce rate limiting policy on the inbound traffic at the identified backends. If this field is not set, rate limiting is disabled for this endpoint. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `parent` | String | ✅ | Required. The parent resource of the EndpointPolicy. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `authorization_policy` | String | Optional. This field specifies the URL of AuthorizationPolicy resource that applies authorization policies to the inbound traffic at the matched endpoints. Refer to Authorization. If this field is not specified, authorization is disabled(no authz checks) for this endpoint. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `endpoint_matcher` | String | Required. A matcher that selects endpoints to which the policies should be applied. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the EndpointPolicy resource. |
| `traffic_port_selector` | String | Optional. Port selector for the (matched) endpoints. If no port selector is provided, the matched config is applied to all ports. |
| `client_tls_policy` | String | Optional. A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set to specify the authentication for traffic from the proxy to the actual endpoints. More specifically, it is applied to the outgoing traffic from the proxy to the endpoint. This is typically used for sidecar model where the proxy identifies itself as endpoint to the control plane, with the connection between sidecar and endpoint requiring authentication. If this field is not set, authentication is disabled(open). Applicable only when EndpointPolicyType is SIDECAR_PROXY. |
| `name` | String | Identifier. Name of the EndpointPolicy resource. It matches pattern `projects/{project}/locations/global/endpointPolicies/{endpoint_policy}`. |
| `type` | String | Required. The type of endpoint policy. This is primarily used to validate the configuration. |
| `server_tls_policy` | String | Optional. A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to determine the authentication policy to be applied to terminate the inbound traffic at the identified backends. If this field is not set, authentication is disabled(open) for this endpoint. |
| `security_policy` | String | Optional. A URL referring to a SecurityPolicy resource. SecurityPolicy is used to enforce rate limiting policy on the inbound traffic at the identified backends. If this field is not set, rate limiting is disabled for this endpoint. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create endpoint_policie
endpoint_policie = provider.networkservices_api.Endpoint_policie {
    parent = "value"  # Required. The parent resource of the EndpointPolicy. Must be in the format `projects/*/locations/global`.
}

# Access endpoint_policie outputs
endpoint_policie_id = endpoint_policie.id
endpoint_policie_create_time = endpoint_policie.create_time
endpoint_policie_authorization_policy = endpoint_policie.authorization_policy
endpoint_policie_description = endpoint_policie.description
endpoint_policie_endpoint_matcher = endpoint_policie.endpoint_matcher
endpoint_policie_labels = endpoint_policie.labels
endpoint_policie_traffic_port_selector = endpoint_policie.traffic_port_selector
endpoint_policie_client_tls_policy = endpoint_policie.client_tls_policy
endpoint_policie_name = endpoint_policie.name
endpoint_policie_type = endpoint_policie.type
endpoint_policie_server_tls_policy = endpoint_policie.server_tls_policy
endpoint_policie_security_policy = endpoint_policie.security_policy
endpoint_policie_update_time = endpoint_policie.update_time
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
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |


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
location_metadata = location.metadata
location_labels = location.labels
location_display_name = location.display_name
location_location_id = location.location_id
```

---


### Service_binding

Creates a new ServiceBinding in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the ServiceBinding resource. |
| `service` | String |  | Optional. The full Service Directory Service name of the format `projects/*/locations/*/namespaces/*/services/*`. This field is for Service Directory integration which will be deprecated soon. |
| `service_id` | String |  | Output only. The unique identifier of the Service Directory Service against which the ServiceBinding resource is validated. This is populated when the Service Binding resource is used in another resource (like Backend Service). This is of the UUID4 format. This field is for Service Directory integration which will be deprecated soon. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `name` | String |  | Identifier. Name of the ServiceBinding resource. It matches pattern `projects/*/locations/*/serviceBindings/`. |
| `parent` | String | ✅ | Required. The parent resource of the ServiceBinding. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the ServiceBinding resource. |
| `service` | String | Optional. The full Service Directory Service name of the format `projects/*/locations/*/namespaces/*/services/*`. This field is for Service Directory integration which will be deprecated soon. |
| `service_id` | String | Output only. The unique identifier of the Service Directory Service against which the ServiceBinding resource is validated. This is populated when the Service Binding resource is used in another resource (like Backend Service). This is of the UUID4 format. This field is for Service Directory integration which will be deprecated soon. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `name` | String | Identifier. Name of the ServiceBinding resource. It matches pattern `projects/*/locations/*/serviceBindings/`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_binding
service_binding = provider.networkservices_api.Service_binding {
    parent = "value"  # Required. The parent resource of the ServiceBinding. Must be in the format `projects/*/locations/*`.
}

# Access service_binding outputs
service_binding_id = service_binding.id
service_binding_create_time = service_binding.create_time
service_binding_labels = service_binding.labels
service_binding_service = service_binding.service
service_binding_service_id = service_binding.service_id
service_binding_update_time = service_binding.update_time
service_binding_description = service_binding.description
service_binding_name = service_binding.name
```

---


### Lb_route_extension

Creates a new `LbRouteExtension` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. A human-readable description of the resource. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `LbRouteExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `load_balancing_scheme` | String |  | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `extension_chains` | Vec<String> |  | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `metadata` | HashMap<String, String> |  | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata applies to all extensions in all extensions chains in this resource. The metadata is available under the key `com.google.lb_route_extension.`. The following variables are supported in the metadata: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. This field must not be set if at least one of the extension chains contains plugin extensions. Setting it results in a validation error. You can set metadata at either the resource level or the extension level. The extension level metadata is recommended because you can pass a different set of metadata through each extension to the backend. |
| `forwarding_rules` | Vec<String> |  | Required. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbRouteExtension` resource can be associated with a forwarding rule. |
| `name` | String |  | Required. Identifier. Name of the `LbRouteExtension` resource in the following format: `projects/{project}/locations/{location}/lbRouteExtensions/{lb_route_extension}`. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `parent` | String | ✅ | Required. The parent resource of the `LbRouteExtension` resource. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. A human-readable description of the resource. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `LbRouteExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `load_balancing_scheme` | String | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `extension_chains` | Vec<String> | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `metadata` | HashMap<String, String> | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata applies to all extensions in all extensions chains in this resource. The metadata is available under the key `com.google.lb_route_extension.`. The following variables are supported in the metadata: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. This field must not be set if at least one of the extension chains contains plugin extensions. Setting it results in a validation error. You can set metadata at either the resource level or the extension level. The extension level metadata is recommended because you can pass a different set of metadata through each extension to the backend. |
| `forwarding_rules` | Vec<String> | Required. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbRouteExtension` resource can be associated with a forwarding rule. |
| `name` | String | Required. Identifier. Name of the `LbRouteExtension` resource in the following format: `projects/{project}/locations/{location}/lbRouteExtensions/{lb_route_extension}`. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `create_time` | String | Output only. The timestamp when the resource was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lb_route_extension
lb_route_extension = provider.networkservices_api.Lb_route_extension {
    parent = "value"  # Required. The parent resource of the `LbRouteExtension` resource. Must be in the format `projects/{project}/locations/{location}`.
}

# Access lb_route_extension outputs
lb_route_extension_id = lb_route_extension.id
lb_route_extension_description = lb_route_extension.description
lb_route_extension_labels = lb_route_extension.labels
lb_route_extension_load_balancing_scheme = lb_route_extension.load_balancing_scheme
lb_route_extension_extension_chains = lb_route_extension.extension_chains
lb_route_extension_metadata = lb_route_extension.metadata
lb_route_extension_forwarding_rules = lb_route_extension.forwarding_rules
lb_route_extension_name = lb_route_extension.name
lb_route_extension_update_time = lb_route_extension.update_time
lb_route_extension_create_time = lb_route_extension.create_time
```

---


### Tls_route

Creates a new TlsRoute in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the TlsRoute resource. |
| `name` | String |  | Identifier. Name of the TlsRoute resource. It matches pattern `projects/*/locations/global/tlsRoutes/tls_route_name>`. |
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `meshes` | Vec<String> |  | Optional. Meshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |
| `rules` | Vec<String> |  | Required. Rules that define how traffic is routed and handled. At least one RouteRule must be supplied. If there are multiple rules then the action taken will be the first rule to match. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `gateways` | Vec<String> |  | Optional. Gateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `parent` | String | ✅ | Required. The parent resource of the TlsRoute. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the TlsRoute resource. |
| `name` | String | Identifier. Name of the TlsRoute resource. It matches pattern `projects/*/locations/global/tlsRoutes/tls_route_name>`. |
| `self_link` | String | Output only. Server-defined URL of this resource |
| `meshes` | Vec<String> | Optional. Meshes defines a list of meshes this TlsRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |
| `rules` | Vec<String> | Required. Rules that define how traffic is routed and handled. At least one RouteRule must be supplied. If there are multiple rules then the action taken will be the first rule to match. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `gateways` | Vec<String> | Optional. Gateways defines a list of gateways this TlsRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tls_route
tls_route = provider.networkservices_api.Tls_route {
    parent = "value"  # Required. The parent resource of the TlsRoute. Must be in the format `projects/*/locations/global`.
}

# Access tls_route outputs
tls_route_id = tls_route.id
tls_route_create_time = tls_route.create_time
tls_route_description = tls_route.description
tls_route_labels = tls_route.labels
tls_route_name = tls_route.name
tls_route_self_link = tls_route.self_link
tls_route_meshes = tls_route.meshes
tls_route_rules = tls_route.rules
tls_route_update_time = tls_route.update_time
tls_route_gateways = tls_route.gateways
```

---


### Lb_tcp_extension

Creates a new `LbTcpExtension` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `LbTcpExtension` resource. The format must comply with [the requirements for labels](/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `load_balancing_scheme` | String |  | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `networks` | Vec<String> |  | Optional. If set, this `LbTcpExtension` resource applies to all `ForwardingRule` resources in these VPC networks. Values should be relative resource names identifying VPC networks, for example `projects/*/global/networks/network-1`. Currently limited to 1 network per resource. Limited to 1 network per resource. |
| `name` | String |  | Required. Identifier. Name of the `LbTcpExtension` resource in the following format: `projects/{project}/locations/{location}/LbTcpExtension/{lb_tcp_extension}` |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `extension_chains` | Vec<String> |  | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `parent` | String | ✅ | Required. The parent resource of the `LbTcpExtension` resource. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `description` | String | Optional. A human-readable description of the resource. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `LbTcpExtension` resource. The format must comply with [the requirements for labels](/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `load_balancing_scheme` | String | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `networks` | Vec<String> | Optional. If set, this `LbTcpExtension` resource applies to all `ForwardingRule` resources in these VPC networks. Values should be relative resource names identifying VPC networks, for example `projects/*/global/networks/network-1`. Currently limited to 1 network per resource. Limited to 1 network per resource. |
| `name` | String | Required. Identifier. Name of the `LbTcpExtension` resource in the following format: `projects/{project}/locations/{location}/LbTcpExtension/{lb_tcp_extension}` |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `extension_chains` | Vec<String> | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lb_tcp_extension
lb_tcp_extension = provider.networkservices_api.Lb_tcp_extension {
    parent = "value"  # Required. The parent resource of the `LbTcpExtension` resource. Must be in the format `projects/{project}/locations/{location}`.
}

# Access lb_tcp_extension outputs
lb_tcp_extension_id = lb_tcp_extension.id
lb_tcp_extension_create_time = lb_tcp_extension.create_time
lb_tcp_extension_description = lb_tcp_extension.description
lb_tcp_extension_labels = lb_tcp_extension.labels
lb_tcp_extension_load_balancing_scheme = lb_tcp_extension.load_balancing_scheme
lb_tcp_extension_networks = lb_tcp_extension.networks
lb_tcp_extension_name = lb_tcp_extension.name
lb_tcp_extension_update_time = lb_tcp_extension.update_time
lb_tcp_extension_extension_chains = lb_tcp_extension.extension_chains
```

---


### Tcp_route

Creates a new TcpRoute in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the TcpRoute resource. |
| `rules` | Vec<String> |  | Required. Rules that define how traffic is routed and handled. At least one RouteRule must be supplied. If there are multiple rules then the action taken will be the first rule to match. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `gateways` | Vec<String> |  | Optional. Gateways defines a list of gateways this TcpRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `meshes` | Vec<String> |  | Optional. Meshes defines a list of meshes this TcpRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |
| `name` | String |  | Identifier. Name of the TcpRoute resource. It matches pattern `projects/*/locations/global/tcpRoutes/tcp_route_name>`. |
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `parent` | String | ✅ | Required. The parent resource of the TcpRoute. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the TcpRoute resource. |
| `rules` | Vec<String> | Required. Rules that define how traffic is routed and handled. At least one RouteRule must be supplied. If there are multiple rules then the action taken will be the first rule to match. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `gateways` | Vec<String> | Optional. Gateways defines a list of gateways this TcpRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `meshes` | Vec<String> | Optional. Meshes defines a list of meshes this TcpRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |
| `name` | String | Identifier. Name of the TcpRoute resource. It matches pattern `projects/*/locations/global/tcpRoutes/tcp_route_name>`. |
| `self_link` | String | Output only. Server-defined URL of this resource |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tcp_route
tcp_route = provider.networkservices_api.Tcp_route {
    parent = "value"  # Required. The parent resource of the TcpRoute. Must be in the format `projects/*/locations/global`.
}

# Access tcp_route outputs
tcp_route_id = tcp_route.id
tcp_route_labels = tcp_route.labels
tcp_route_rules = tcp_route.rules
tcp_route_update_time = tcp_route.update_time
tcp_route_gateways = tcp_route.gateways
tcp_route_create_time = tcp_route.create_time
tcp_route_description = tcp_route.description
tcp_route_meshes = tcp_route.meshes
tcp_route_name = tcp_route.name
tcp_route_self_link = tcp_route.self_link
```

---


### Http_route

Creates a new HttpRoute in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `meshes` | Vec<String> |  | Optional. Meshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |
| `gateways` | Vec<String> |  | Optional. Gateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the HttpRoute resource. |
| `name` | String |  | Identifier. Name of the HttpRoute resource. It matches pattern `projects/*/locations/global/httpRoutes/http_route_name>`. |
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `rules` | Vec<String> |  | Required. Rules that define how traffic is routed and handled. Rules will be matched sequentially based on the RouteMatch specified for the rule. |
| `hostnames` | Vec<String> |  | Required. Hostnames define a set of hosts that should match against the HTTP host header to select a HttpRoute to process the request. Hostname is the fully qualified domain name of a network host, as defined by RFC 1123 with the exception that: - IPs are not allowed. - A hostname may be prefixed with a wildcard label (`*.`). The wildcard label must appear by itself as the first label. Hostname can be "precise" which is a domain name without the terminating dot of a network host (e.g. `foo.example.com`) or "wildcard", which is a domain name prefixed with a single wildcard label (e.g. `*.example.com`). Note that as per RFC1035 and RFC1123, a label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character. No other punctuation is allowed. The routes associated with a Mesh or Gateways must have unique hostnames. If you attempt to attach multiple routes with conflicting hostnames, the configuration will be rejected. For example, while it is acceptable for routes for the hostnames `*.foo.bar.com` and `*.bar.com` to be associated with the same Mesh (or Gateways under the same scope), it is not possible to associate two routes both with `*.bar.com` or both with `bar.com`. |
| `parent` | String | ✅ | Required. The parent resource of the HttpRoute. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `meshes` | Vec<String> | Optional. Meshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` The attached Mesh should be of a type SIDECAR |
| `gateways` | Vec<String> | Optional. Gateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the HttpRoute resource. |
| `name` | String | Identifier. Name of the HttpRoute resource. It matches pattern `projects/*/locations/global/httpRoutes/http_route_name>`. |
| `self_link` | String | Output only. Server-defined URL of this resource |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `rules` | Vec<String> | Required. Rules that define how traffic is routed and handled. Rules will be matched sequentially based on the RouteMatch specified for the rule. |
| `hostnames` | Vec<String> | Required. Hostnames define a set of hosts that should match against the HTTP host header to select a HttpRoute to process the request. Hostname is the fully qualified domain name of a network host, as defined by RFC 1123 with the exception that: - IPs are not allowed. - A hostname may be prefixed with a wildcard label (`*.`). The wildcard label must appear by itself as the first label. Hostname can be "precise" which is a domain name without the terminating dot of a network host (e.g. `foo.example.com`) or "wildcard", which is a domain name prefixed with a single wildcard label (e.g. `*.example.com`). Note that as per RFC1035 and RFC1123, a label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character. No other punctuation is allowed. The routes associated with a Mesh or Gateways must have unique hostnames. If you attempt to attach multiple routes with conflicting hostnames, the configuration will be rejected. For example, while it is acceptable for routes for the hostnames `*.foo.bar.com` and `*.bar.com` to be associated with the same Mesh (or Gateways under the same scope), it is not possible to associate two routes both with `*.bar.com` or both with `bar.com`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create http_route
http_route = provider.networkservices_api.Http_route {
    parent = "value"  # Required. The parent resource of the HttpRoute. Must be in the format `projects/*/locations/global`.
}

# Access http_route outputs
http_route_id = http_route.id
http_route_meshes = http_route.meshes
http_route_gateways = http_route.gateways
http_route_labels = http_route.labels
http_route_name = http_route.name
http_route_self_link = http_route.self_link
http_route_description = http_route.description
http_route_update_time = http_route.update_time
http_route_create_time = http_route.create_time
http_route_rules = http_route.rules
http_route_hostnames = http_route.hostnames
```

---


### Meshe

Creates a new Mesh in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `interception_port` | i64 |  | Optional. If set to a valid TCP port (1-65535), instructs the SIDECAR proxy to listen on the specified port of localhost (127.0.0.1) address. The SIDECAR proxy will expect all traffic to be redirected to this port regardless of its actual ip:port destination. If unset, a port '15001' is used as the interception port. This is applicable only for sidecar proxy deployments. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `name` | String |  | Identifier. Name of the Mesh resource. It matches pattern `projects/*/locations/global/meshes/`. |
| `envoy_headers` | String |  | Optional. Determines if envoy will insert internal debug headers into upstream requests. Other Envoy headers may still be injected. By default, envoy will not insert any debug headers. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the Mesh resource. |
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `parent` | String | ✅ | Required. The parent resource of the Mesh. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `interception_port` | i64 | Optional. If set to a valid TCP port (1-65535), instructs the SIDECAR proxy to listen on the specified port of localhost (127.0.0.1) address. The SIDECAR proxy will expect all traffic to be redirected to this port regardless of its actual ip:port destination. If unset, a port '15001' is used as the interception port. This is applicable only for sidecar proxy deployments. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `name` | String | Identifier. Name of the Mesh resource. It matches pattern `projects/*/locations/global/meshes/`. |
| `envoy_headers` | String | Optional. Determines if envoy will insert internal debug headers into upstream requests. Other Envoy headers may still be injected. By default, envoy will not insert any debug headers. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the Mesh resource. |
| `self_link` | String | Output only. Server-defined URL of this resource |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create meshe
meshe = provider.networkservices_api.Meshe {
    parent = "value"  # Required. The parent resource of the Mesh. Must be in the format `projects/*/locations/global`.
}

# Access meshe outputs
meshe_id = meshe.id
meshe_interception_port = meshe.interception_port
meshe_create_time = meshe.create_time
meshe_name = meshe.name
meshe_envoy_headers = meshe.envoy_headers
meshe_labels = meshe.labels
meshe_self_link = meshe.self_link
meshe_description = meshe.description
meshe_update_time = meshe.update_time
```

---


### Authz_extension

Creates a new `AuthzExtension` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. A human-readable description of the resource. |
| `fail_open` | bool |  | Optional. Determines how the proxy behaves if the call to the extension fails or times out. When set to `TRUE`, request or response processing continues without error. Any subsequent extensions in the extension chain are also executed. When set to `FALSE` or the default setting of `FALSE` is used, one of the following happens: * If response headers have not been delivered to the downstream client, a generic 500 error is returned to the client. The error response can be tailored by configuring a custom error response in the load balancer. * If response headers have been delivered, then the HTTP stream to the downstream client is reset. |
| `forward_headers` | Vec<String> |  | Optional. List of the HTTP headers to forward to the extension (from the client). If omitted, all headers are sent. Each element is a string indicating the header name. |
| `metadata` | HashMap<String, String> |  | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata is available under the namespace `com.google.authz_extension.`. The following variables are supported in the metadata Struct: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `AuthzExtension` resource. The format must comply with [the requirements for labels](/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `name` | String |  | Required. Identifier. Name of the `AuthzExtension` resource in the following format: `projects/{project}/locations/{location}/authzExtensions/{authz_extension}`. |
| `service` | String |  | Required. The reference to the service that runs the extension. To configure a callout extension, `service` must be a fully-qualified reference to a [backend service](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices) in the format: `https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/backendServices/{backendService}` or `https://www.googleapis.com/compute/v1/projects/{project}/global/backendServices/{backendService}`. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `wire_format` | String |  | Optional. The format of communication supported by the callout extension. If not specified, the default value `EXT_PROC_GRPC` is used. |
| `load_balancing_scheme` | String |  | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `authority` | String |  | Required. The `:authority` header in the gRPC request sent from Envoy to the extension service. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `timeout` | String |  | Required. Specifies the timeout for each individual message on the stream. The timeout must be between 10-10000 milliseconds. |
| `parent` | String | ✅ | Required. The parent resource of the `AuthzExtension` resource. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. A human-readable description of the resource. |
| `fail_open` | bool | Optional. Determines how the proxy behaves if the call to the extension fails or times out. When set to `TRUE`, request or response processing continues without error. Any subsequent extensions in the extension chain are also executed. When set to `FALSE` or the default setting of `FALSE` is used, one of the following happens: * If response headers have not been delivered to the downstream client, a generic 500 error is returned to the client. The error response can be tailored by configuring a custom error response in the load balancer. * If response headers have been delivered, then the HTTP stream to the downstream client is reset. |
| `forward_headers` | Vec<String> | Optional. List of the HTTP headers to forward to the extension (from the client). If omitted, all headers are sent. Each element is a string indicating the header name. |
| `metadata` | HashMap<String, String> | Optional. The metadata provided here is included as part of the `metadata_context` (of type `google.protobuf.Struct`) in the `ProcessingRequest` message sent to the extension server. The metadata is available under the namespace `com.google.authz_extension.`. The following variables are supported in the metadata Struct: `{forwarding_rule_id}` - substituted with the forwarding rule's fully qualified resource name. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `AuthzExtension` resource. The format must comply with [the requirements for labels](/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `name` | String | Required. Identifier. Name of the `AuthzExtension` resource in the following format: `projects/{project}/locations/{location}/authzExtensions/{authz_extension}`. |
| `service` | String | Required. The reference to the service that runs the extension. To configure a callout extension, `service` must be a fully-qualified reference to a [backend service](https://cloud.google.com/compute/docs/reference/rest/v1/backendServices) in the format: `https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/backendServices/{backendService}` or `https://www.googleapis.com/compute/v1/projects/{project}/global/backendServices/{backendService}`. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `wire_format` | String | Optional. The format of communication supported by the callout extension. If not specified, the default value `EXT_PROC_GRPC` is used. |
| `load_balancing_scheme` | String | Required. All backend services and forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`. For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service). |
| `authority` | String | Required. The `:authority` header in the gRPC request sent from Envoy to the extension service. |
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `timeout` | String | Required. Specifies the timeout for each individual message on the stream. The timeout must be between 10-10000 milliseconds. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create authz_extension
authz_extension = provider.networkservices_api.Authz_extension {
    parent = "value"  # Required. The parent resource of the `AuthzExtension` resource. Must be in the format `projects/{project}/locations/{location}`.
}

# Access authz_extension outputs
authz_extension_id = authz_extension.id
authz_extension_description = authz_extension.description
authz_extension_fail_open = authz_extension.fail_open
authz_extension_forward_headers = authz_extension.forward_headers
authz_extension_metadata = authz_extension.metadata
authz_extension_labels = authz_extension.labels
authz_extension_name = authz_extension.name
authz_extension_service = authz_extension.service
authz_extension_update_time = authz_extension.update_time
authz_extension_wire_format = authz_extension.wire_format
authz_extension_load_balancing_scheme = authz_extension.load_balancing_scheme
authz_extension_authority = authz_extension.authority
authz_extension_create_time = authz_extension.create_time
authz_extension_timeout = authz_extension.timeout
```

---


### Route_view

Get a single RouteView of a Mesh.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Identifier. Full path name of the MeshRouteView resource. Format: projects/{project_number}/locations/{location}/meshes/{mesh}/routeViews/{route_view} |
| `route_location` | String | Output only. Location where the route exists. |
| `route_project_number` | String | Output only. Project number where the route exists. |
| `route_id` | String | Output only. The resource id for the route. |
| `route_type` | String | Output only. Type of the route: HttpRoute,GrpcRoute,TcpRoute, or TlsRoute |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access route_view outputs
route_view_id = route_view.id
route_view_name = route_view.name
route_view_route_location = route_view.route_location
route_view_route_project_number = route_view.route_project_number
route_view_route_id = route_view.route_id
route_view_route_type = route_view.route_type
```

---


### Service_lb_policie

Creates a new ServiceLbPolicy in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `failover_config` | String |  | Optional. Configuration related to health based failover. |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `isolation_config` | String |  | Optional. Configuration to provide isolation support for the associated Backend Service. |
| `load_balancing_algorithm` | String |  | Optional. The type of load balancing algorithm to be used. The default behavior is WATERFALL_BY_REGION. |
| `auto_capacity_drain` | String |  | Optional. Configuration to automatically move traffic away for unhealthy IG/NEG for the associated Backend Service. |
| `create_time` | String |  | Output only. The timestamp when this resource was created. |
| `name` | String |  | Identifier. Name of the ServiceLbPolicy resource. It matches pattern `projects/{project}/locations/{location}/serviceLbPolicies/{service_lb_policy_name}`. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the ServiceLbPolicy resource. |
| `update_time` | String |  | Output only. The timestamp when this resource was last updated. |
| `parent` | String | ✅ | Required. The parent resource of the ServiceLbPolicy. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `failover_config` | String | Optional. Configuration related to health based failover. |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `isolation_config` | String | Optional. Configuration to provide isolation support for the associated Backend Service. |
| `load_balancing_algorithm` | String | Optional. The type of load balancing algorithm to be used. The default behavior is WATERFALL_BY_REGION. |
| `auto_capacity_drain` | String | Optional. Configuration to automatically move traffic away for unhealthy IG/NEG for the associated Backend Service. |
| `create_time` | String | Output only. The timestamp when this resource was created. |
| `name` | String | Identifier. Name of the ServiceLbPolicy resource. It matches pattern `projects/{project}/locations/{location}/serviceLbPolicies/{service_lb_policy_name}`. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the ServiceLbPolicy resource. |
| `update_time` | String | Output only. The timestamp when this resource was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create service_lb_policie
service_lb_policie = provider.networkservices_api.Service_lb_policie {
    parent = "value"  # Required. The parent resource of the ServiceLbPolicy. Must be in the format `projects/{project}/locations/{location}`.
}

# Access service_lb_policie outputs
service_lb_policie_id = service_lb_policie.id
service_lb_policie_failover_config = service_lb_policie.failover_config
service_lb_policie_description = service_lb_policie.description
service_lb_policie_isolation_config = service_lb_policie.isolation_config
service_lb_policie_load_balancing_algorithm = service_lb_policie.load_balancing_algorithm
service_lb_policie_auto_capacity_drain = service_lb_policie.auto_capacity_drain
service_lb_policie_create_time = service_lb_policie.create_time
service_lb_policie_name = service_lb_policie.name
service_lb_policie_labels = service_lb_policie.labels
service_lb_policie_update_time = service_lb_policie.update_time
```

---


### Lb_edge_extension

Creates a new `LbEdgeExtension` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `extension_chains` | Vec<String> |  | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `forwarding_rules` | Vec<String> |  | Required. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbEdgeExtension` resource can be associated with a forwarding rule. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `LbEdgeExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `name` | String |  | Required. Identifier. Name of the `LbEdgeExtension` resource in the following format: `projects/{project}/locations/{location}/lbEdgeExtensions/{lb_edge_extension}`. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `load_balancing_scheme` | String |  | Required. All forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `EXTERNAL_MANAGED`. |
| `parent` | String | ✅ | Required. The parent resource of the `LbEdgeExtension` resource. Must be in the format `projects/{project}/locations/{location}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `extension_chains` | Vec<String> | Required. A set of ordered extension chains that contain the match conditions and extensions to execute. Match conditions for each extension chain are evaluated in sequence for a given request. The first extension chain that has a condition that matches the request is executed. Any subsequent extension chains do not execute. Limited to 5 extension chains per resource. |
| `forwarding_rules` | Vec<String> | Required. A list of references to the forwarding rules to which this service extension is attached. At least one forwarding rule is required. Only one `LbEdgeExtension` resource can be associated with a forwarding rule. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `LbEdgeExtension` resource. The format must comply with [the requirements for labels](https://cloud.google.com/compute/docs/labeling-resources#requirements) for Google Cloud resources. |
| `description` | String | Optional. A human-readable description of the resource. |
| `name` | String | Required. Identifier. Name of the `LbEdgeExtension` resource in the following format: `projects/{project}/locations/{location}/lbEdgeExtensions/{lb_edge_extension}`. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `load_balancing_scheme` | String | Required. All forwarding rules referenced by this extension must share the same load balancing scheme. Supported values: `EXTERNAL_MANAGED`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lb_edge_extension
lb_edge_extension = provider.networkservices_api.Lb_edge_extension {
    parent = "value"  # Required. The parent resource of the `LbEdgeExtension` resource. Must be in the format `projects/{project}/locations/{location}`.
}

# Access lb_edge_extension outputs
lb_edge_extension_id = lb_edge_extension.id
lb_edge_extension_create_time = lb_edge_extension.create_time
lb_edge_extension_extension_chains = lb_edge_extension.extension_chains
lb_edge_extension_forwarding_rules = lb_edge_extension.forwarding_rules
lb_edge_extension_labels = lb_edge_extension.labels
lb_edge_extension_description = lb_edge_extension.description
lb_edge_extension_name = lb_edge_extension.name
lb_edge_extension_update_time = lb_edge_extension.update_time
lb_edge_extension_load_balancing_scheme = lb_edge_extension.load_balancing_scheme
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.networkservices_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_error = operation.error
operation_metadata = operation.metadata
operation_response = operation.response
operation_done = operation.done
```

---


### Wasm_plugin

Creates a new `WasmPlugin` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `log_config` | String |  | Optional. Specifies the logging options for the activity performed by this plugin. If logging is enabled, plugin logs are exported to Cloud Logging. Note that the settings relate to the logs generated by using logging statements in your Wasm code. |
| `name` | String |  | Identifier. Name of the `WasmPlugin` resource in the following format: `projects/{project}/locations/{location}/wasmPlugins/{wasm_plugin}`. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `WasmPlugin` resource. The format must comply with [the following requirements](/compute/docs/labeling-resources#requirements). |
| `used_by` | Vec<String> |  | Output only. List of all [extensions](https://cloud.google.com/service-extensions/docs/overview) that use this `WasmPlugin` resource. |
| `versions` | HashMap<String, String> |  | Optional. All versions of this `WasmPlugin` resource in the key-value format. The key is the resource ID, and the value is the `VersionDetails` object. Lets you create or update a `WasmPlugin` resource and its versions in a single request. When the `main_version_id` field is not empty, it must point to one of the `VersionDetails` objects in the map. If provided in a `PATCH` request, the new versions replace the previous set. Any version omitted from the `versions` field is removed. Because the `WasmPluginVersion` resource is immutable, if a `WasmPluginVersion` resource with the same name already exists and differs, the request fails. Note: In a `GET` request, this field is populated only if the field `GetWasmPluginRequest.view` is set to `WASM_PLUGIN_VIEW_FULL`. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `main_version_id` | String |  | Optional. The ID of the `WasmPluginVersion` resource that is the currently serving one. The version referred to must be a child of this `WasmPlugin` resource. |
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `parent` | String | ✅ | Required. The parent resource of the `WasmPlugin` resource. Must be in the format `projects/{project}/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `log_config` | String | Optional. Specifies the logging options for the activity performed by this plugin. If logging is enabled, plugin logs are exported to Cloud Logging. Note that the settings relate to the logs generated by using logging statements in your Wasm code. |
| `name` | String | Identifier. Name of the `WasmPlugin` resource in the following format: `projects/{project}/locations/{location}/wasmPlugins/{wasm_plugin}`. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `WasmPlugin` resource. The format must comply with [the following requirements](/compute/docs/labeling-resources#requirements). |
| `used_by` | Vec<String> | Output only. List of all [extensions](https://cloud.google.com/service-extensions/docs/overview) that use this `WasmPlugin` resource. |
| `versions` | HashMap<String, String> | Optional. All versions of this `WasmPlugin` resource in the key-value format. The key is the resource ID, and the value is the `VersionDetails` object. Lets you create or update a `WasmPlugin` resource and its versions in a single request. When the `main_version_id` field is not empty, it must point to one of the `VersionDetails` objects in the map. If provided in a `PATCH` request, the new versions replace the previous set. Any version omitted from the `versions` field is removed. Because the `WasmPluginVersion` resource is immutable, if a `WasmPluginVersion` resource with the same name already exists and differs, the request fails. Note: In a `GET` request, this field is populated only if the field `GetWasmPluginRequest.view` is set to `WASM_PLUGIN_VIEW_FULL`. |
| `description` | String | Optional. A human-readable description of the resource. |
| `main_version_id` | String | Optional. The ID of the `WasmPluginVersion` resource that is the currently serving one. The version referred to must be a child of this `WasmPlugin` resource. |
| `create_time` | String | Output only. The timestamp when the resource was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create wasm_plugin
wasm_plugin = provider.networkservices_api.Wasm_plugin {
    parent = "value"  # Required. The parent resource of the `WasmPlugin` resource. Must be in the format `projects/{project}/locations/global`.
}

# Access wasm_plugin outputs
wasm_plugin_id = wasm_plugin.id
wasm_plugin_log_config = wasm_plugin.log_config
wasm_plugin_name = wasm_plugin.name
wasm_plugin_update_time = wasm_plugin.update_time
wasm_plugin_labels = wasm_plugin.labels
wasm_plugin_used_by = wasm_plugin.used_by
wasm_plugin_versions = wasm_plugin.versions
wasm_plugin_description = wasm_plugin.description
wasm_plugin_main_version_id = wasm_plugin.main_version_id
wasm_plugin_create_time = wasm_plugin.create_time
```

---


### Grpc_route

Creates a new GrpcRoute in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `labels` | HashMap<String, String> |  | Optional. Set of label tags associated with the GrpcRoute resource. |
| `rules` | Vec<String> |  | Required. A list of detailed rules defining how to route traffic. Within a single GrpcRoute, the GrpcRoute.RouteAction associated with the first matching GrpcRoute.RouteRule will be executed. At least one rule must be supplied. |
| `self_link` | String |  | Output only. Server-defined URL of this resource |
| `gateways` | Vec<String> |  | Optional. Gateways defines a list of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `meshes` | Vec<String> |  | Optional. Meshes defines a list of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` |
| `hostnames` | Vec<String> |  | Required. Service hostnames with an optional port for which this route describes traffic. Format: [:] Hostname is the fully qualified domain name of a network host. This matches the RFC 1123 definition of a hostname with 2 notable exceptions: - IPs are not allowed. - A hostname may be prefixed with a wildcard label (`*.`). The wildcard label must appear by itself as the first label. Hostname can be "precise" which is a domain name without the terminating dot of a network host (e.g. `foo.example.com`) or "wildcard", which is a domain name prefixed with a single wildcard label (e.g. `*.example.com`). Note that as per RFC1035 and RFC1123, a label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character. No other punctuation is allowed. The routes associated with a Mesh or Gateway must have unique hostnames. If you attempt to attach multiple routes with conflicting hostnames, the configuration will be rejected. For example, while it is acceptable for routes for the hostnames `*.foo.bar.com` and `*.bar.com` to be associated with the same route, it is not possible to associate two routes both with `*.bar.com` or both with `bar.com`. If a port is specified, then gRPC clients must use the channel URI with the port to match this rule (i.e. "xds:///service:123"), otherwise they must supply the URI without a port (i.e. "xds:///service"). |
| `description` | String |  | Optional. A free-text description of the resource. Max length 1024 characters. |
| `name` | String |  | Identifier. Name of the GrpcRoute resource. It matches pattern `projects/*/locations/global/grpcRoutes/` |
| `parent` | String | ✅ | Required. The parent resource of the GrpcRoute. Must be in the format `projects/*/locations/global`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `labels` | HashMap<String, String> | Optional. Set of label tags associated with the GrpcRoute resource. |
| `rules` | Vec<String> | Required. A list of detailed rules defining how to route traffic. Within a single GrpcRoute, the GrpcRoute.RouteAction associated with the first matching GrpcRoute.RouteRule will be executed. At least one rule must be supplied. |
| `self_link` | String | Output only. Server-defined URL of this resource |
| `gateways` | Vec<String> | Optional. Gateways defines a list of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway. Each gateway reference should match the pattern: `projects/*/locations/global/gateways/` |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `meshes` | Vec<String> | Optional. Meshes defines a list of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh. Each mesh reference should match the pattern: `projects/*/locations/global/meshes/` |
| `hostnames` | Vec<String> | Required. Service hostnames with an optional port for which this route describes traffic. Format: [:] Hostname is the fully qualified domain name of a network host. This matches the RFC 1123 definition of a hostname with 2 notable exceptions: - IPs are not allowed. - A hostname may be prefixed with a wildcard label (`*.`). The wildcard label must appear by itself as the first label. Hostname can be "precise" which is a domain name without the terminating dot of a network host (e.g. `foo.example.com`) or "wildcard", which is a domain name prefixed with a single wildcard label (e.g. `*.example.com`). Note that as per RFC1035 and RFC1123, a label must consist of lower case alphanumeric characters or '-', and must start and end with an alphanumeric character. No other punctuation is allowed. The routes associated with a Mesh or Gateway must have unique hostnames. If you attempt to attach multiple routes with conflicting hostnames, the configuration will be rejected. For example, while it is acceptable for routes for the hostnames `*.foo.bar.com` and `*.bar.com` to be associated with the same route, it is not possible to associate two routes both with `*.bar.com` or both with `bar.com`. If a port is specified, then gRPC clients must use the channel URI with the port to match this rule (i.e. "xds:///service:123"), otherwise they must supply the URI without a port (i.e. "xds:///service"). |
| `description` | String | Optional. A free-text description of the resource. Max length 1024 characters. |
| `name` | String | Identifier. Name of the GrpcRoute resource. It matches pattern `projects/*/locations/global/grpcRoutes/` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create grpc_route
grpc_route = provider.networkservices_api.Grpc_route {
    parent = "value"  # Required. The parent resource of the GrpcRoute. Must be in the format `projects/*/locations/global`.
}

# Access grpc_route outputs
grpc_route_id = grpc_route.id
grpc_route_create_time = grpc_route.create_time
grpc_route_labels = grpc_route.labels
grpc_route_rules = grpc_route.rules
grpc_route_self_link = grpc_route.self_link
grpc_route_gateways = grpc_route.gateways
grpc_route_update_time = grpc_route.update_time
grpc_route_meshes = grpc_route.meshes
grpc_route_hostnames = grpc_route.hostnames
grpc_route_description = grpc_route.description
grpc_route_name = grpc_route.name
```

---


### Version

Creates a new `WasmPluginVersion` resource in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The timestamp when the resource was created. |
| `plugin_config_data` | String |  | Configuration for the plugin. The configuration is provided to the plugin at runtime through the `ON_CONFIGURE` callback. When a new `WasmPluginVersion` resource is created, the digest of the contents is saved in the `plugin_config_digest` field. |
| `plugin_config_digest` | String |  | Output only. This field holds the digest (usually checksum) value for the plugin configuration. The value is calculated based on the contents of `plugin_config_data` field or the image defined by the `plugin_config_uri` field. |
| `name` | String |  | Identifier. Name of the `WasmPluginVersion` resource in the following format: `projects/{project}/locations/{location}/wasmPlugins/{wasm_plugin}/ versions/{wasm_plugin_version}`. |
| `image_digest` | String |  | Output only. This field holds the digest (usually checksum) value for the plugin image. The value is calculated based on the `image_uri` field. If the `image_uri` field refers to a container image, the digest value is obtained from the container image. If the `image_uri` field refers to a generic artifact, the digest value is calculated based on the contents of the file. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with the `WasmPluginVersion` resource. |
| `update_time` | String |  | Output only. The timestamp when the resource was updated. |
| `image_uri` | String |  | Optional. URI of the image containing the Wasm module, stored in Artifact Registry. The URI can refer to one of the following repository formats: * Container images: the `image_uri` must point to a container that contains a single file with the name `plugin.wasm`. When a new `WasmPluginVersion` resource is created, the digest of the image is saved in the `image_digest` field. When pulling a container image from Artifact Registry, the digest value is used instead of an image tag. * Generic artifacts: the `image_uri` must be in this format: `projects/{project}/locations/{location}/repositories/{repository}/ genericArtifacts/{package}:{version}`. The specified package and version must contain a file with the name `plugin.wasm`. When a new `WasmPluginVersion` resource is created, the checksum of the contents of the file is saved in the `image_digest` field. |
| `plugin_config_uri` | String |  | URI of the plugin configuration stored in the Artifact Registry. The configuration is provided to the plugin at runtime through the `ON_CONFIGURE` callback. The URI can refer to one of the following repository formats: * Container images: the `plugin_config_uri` must point to a container that contains a single file with the name `plugin.config`. When a new `WasmPluginVersion` resource is created, the digest of the image is saved in the `plugin_config_digest` field. When pulling a container image from Artifact Registry, the digest value is used instead of an image tag. * Generic artifacts: the `plugin_config_uri` must be in this format: `projects/{project}/locations/{location}/repositories/{repository}/ genericArtifacts/{package}:{version}`. The specified package and version must contain a file with the name `plugin.config`. When a new `WasmPluginVersion` resource is created, the checksum of the contents of the file is saved in the `plugin_config_digest` field. |
| `description` | String |  | Optional. A human-readable description of the resource. |
| `parent` | String | ✅ | Required. The parent resource of the `WasmPluginVersion` resource. Must be in the format `projects/{project}/locations/global/wasmPlugins/{wasm_plugin}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The timestamp when the resource was created. |
| `plugin_config_data` | String | Configuration for the plugin. The configuration is provided to the plugin at runtime through the `ON_CONFIGURE` callback. When a new `WasmPluginVersion` resource is created, the digest of the contents is saved in the `plugin_config_digest` field. |
| `plugin_config_digest` | String | Output only. This field holds the digest (usually checksum) value for the plugin configuration. The value is calculated based on the contents of `plugin_config_data` field or the image defined by the `plugin_config_uri` field. |
| `name` | String | Identifier. Name of the `WasmPluginVersion` resource in the following format: `projects/{project}/locations/{location}/wasmPlugins/{wasm_plugin}/ versions/{wasm_plugin_version}`. |
| `image_digest` | String | Output only. This field holds the digest (usually checksum) value for the plugin image. The value is calculated based on the `image_uri` field. If the `image_uri` field refers to a container image, the digest value is obtained from the container image. If the `image_uri` field refers to a generic artifact, the digest value is calculated based on the contents of the file. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with the `WasmPluginVersion` resource. |
| `update_time` | String | Output only. The timestamp when the resource was updated. |
| `image_uri` | String | Optional. URI of the image containing the Wasm module, stored in Artifact Registry. The URI can refer to one of the following repository formats: * Container images: the `image_uri` must point to a container that contains a single file with the name `plugin.wasm`. When a new `WasmPluginVersion` resource is created, the digest of the image is saved in the `image_digest` field. When pulling a container image from Artifact Registry, the digest value is used instead of an image tag. * Generic artifacts: the `image_uri` must be in this format: `projects/{project}/locations/{location}/repositories/{repository}/ genericArtifacts/{package}:{version}`. The specified package and version must contain a file with the name `plugin.wasm`. When a new `WasmPluginVersion` resource is created, the checksum of the contents of the file is saved in the `image_digest` field. |
| `plugin_config_uri` | String | URI of the plugin configuration stored in the Artifact Registry. The configuration is provided to the plugin at runtime through the `ON_CONFIGURE` callback. The URI can refer to one of the following repository formats: * Container images: the `plugin_config_uri` must point to a container that contains a single file with the name `plugin.config`. When a new `WasmPluginVersion` resource is created, the digest of the image is saved in the `plugin_config_digest` field. When pulling a container image from Artifact Registry, the digest value is used instead of an image tag. * Generic artifacts: the `plugin_config_uri` must be in this format: `projects/{project}/locations/{location}/repositories/{repository}/ genericArtifacts/{package}:{version}`. The specified package and version must contain a file with the name `plugin.config`. When a new `WasmPluginVersion` resource is created, the checksum of the contents of the file is saved in the `plugin_config_digest` field. |
| `description` | String | Optional. A human-readable description of the resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.networkservices_api.Version {
    parent = "value"  # Required. The parent resource of the `WasmPluginVersion` resource. Must be in the format `projects/{project}/locations/global/wasmPlugins/{wasm_plugin}`.
}

# Access version outputs
version_id = version.id
version_create_time = version.create_time
version_plugin_config_data = version.plugin_config_data
version_plugin_config_digest = version.plugin_config_digest
version_name = version.name
version_image_digest = version.image_digest
version_labels = version.labels
version_update_time = version.update_time
version_image_uri = version.image_uri
version_plugin_config_uri = version.plugin_config_uri
version_description = version.description
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple gateway resources
gateway_0 = provider.networkservices_api.Gateway {
    parent = "value-0"
}
gateway_1 = provider.networkservices_api.Gateway {
    parent = "value-1"
}
gateway_2 = provider.networkservices_api.Gateway {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    gateway = provider.networkservices_api.Gateway {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Networkservices_api Documentation](https://cloud.google.com/networkservices_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
