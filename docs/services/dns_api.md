# Dns_api Service



**Resources**: 34

---

## Overview

The dns_api service provides access to 34 resource types:

- [Project](#project) [R]
- [Change](#change) [CR]
- [Response_policy_rule](#response_policy_rule) [CRUD]
- [Managed_zone_operation](#managed_zone_operation) [R]
- [Policie](#policie) [CRUD]
- [Response_policie](#response_policie) [CRUD]
- [Resource_record_set](#resource_record_set) [CRUD]
- [Managed_zone](#managed_zone) [CRUD]
- [Dns_key](#dns_key) [R]
- [Managed_zone_operation](#managed_zone_operation) [R]
- [Response_policie](#response_policie) [CRUD]
- [Policie](#policie) [CRUD]
- [Response_policy_rule](#response_policy_rule) [CRUD]
- [Resource_record_set](#resource_record_set) [CRUD]
- [Dns_key](#dns_key) [R]
- [Change](#change) [CR]
- [Project](#project) [R]
- [Managed_zone](#managed_zone) [CRUD]
- [Resource_record_set](#resource_record_set) [R]
- [Project](#project) [R]
- [Managed_zone_operation](#managed_zone_operation) [R]
- [Managed_zone](#managed_zone) [CRUD]
- [Change](#change) [CR]
- [Policie](#policie) [CRUD]
- [Dns_key](#dns_key) [R]
- [Change](#change) [CR]
- [Dns_key](#dns_key) [R]
- [Managed_zone_operation](#managed_zone_operation) [R]
- [Resource_record_set](#resource_record_set) [CRUD]
- [Response_policy_rule](#response_policy_rule) [CRUD]
- [Response_policie](#response_policie) [CRUD]
- [Managed_zone](#managed_zone) [CRUD]
- [Project](#project) [R]
- [Policie](#policie) [CRUD]

---

## Resources


### Project

Fetches the representation of an existing Project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String |  |
| `id` | String | User assigned unique identifier for the resource (output only). |
| `number` | String | Unique numeric identifier for the resource; defined by the server (output only). |
| `quota` | String | Quotas assigned to this project (output only). |


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
project_kind = project.kind
project_id = project.id
project_number = project.number
project_quota = project.quota
```

---


### Change

Atomically updates the ResourceRecordSet collection.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  |  |
| `status` | String |  | Status of the operation (output only). A status of "done" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet. |
| `deletions` | Vec<String> |  | Which ResourceRecordSets to remove? Must match existing data exactly. |
| `additions` | Vec<String> |  | Which ResourceRecordSets to add? |
| `is_serving` | bool |  | If the DNS queries for the zone will be served. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only). |
| `start_time` | String |  | The time that this operation was started by the server (output only). This is in RFC3339 text format. |
| `managed_zone` | String | ✅ | Identifies the managed zone addressed by this request. Can be the managed zone name or ID. |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String |  |
| `status` | String | Status of the operation (output only). A status of "done" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet. |
| `deletions` | Vec<String> | Which ResourceRecordSets to remove? Must match existing data exactly. |
| `additions` | Vec<String> | Which ResourceRecordSets to add? |
| `is_serving` | bool | If the DNS queries for the zone will be served. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `start_time` | String | The time that this operation was started by the server (output only). This is in RFC3339 text format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create change
change = provider.dns_api.Change {
    managed_zone = "value"  # Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    project = "value"  # Identifies the project addressed by this request.
}

# Access change outputs
change_id = change.id
change_kind = change.kind
change_status = change.status
change_deletions = change.deletions
change_additions = change.additions
change_is_serving = change.is_serving
change_id = change.id
change_start_time = change.start_time
```

---


### Response_policy_rule

Creates a new Response Policy Rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  |  |
| `dns_name` | String |  | The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule. |
| `rule_name` | String |  | An identifier for this rule. Must be unique with the ResponsePolicy. |
| `local_data` | String |  | Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name; in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed. |
| `behavior` | String |  | Answer this query with a behavior rather than DNS data. |
| `response_policy` | String | ✅ | User assigned name of the Response Policy containing the Response Policy Rule. |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String |  |
| `dns_name` | String | The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule. |
| `rule_name` | String | An identifier for this rule. Must be unique with the ResponsePolicy. |
| `local_data` | String | Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name; in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed. |
| `behavior` | String | Answer this query with a behavior rather than DNS data. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create response_policy_rule
response_policy_rule = provider.dns_api.Response_policy_rule {
    response_policy = "value"  # User assigned name of the Response Policy containing the Response Policy Rule.
    project = "value"  # Identifies the project addressed by this request.
}

# Access response_policy_rule outputs
response_policy_rule_id = response_policy_rule.id
response_policy_rule_kind = response_policy_rule.kind
response_policy_rule_dns_name = response_policy_rule.dns_name
response_policy_rule_rule_name = response_policy_rule.rule_name
response_policy_rule_local_data = response_policy_rule.local_data
response_policy_rule_behavior = response_policy_rule.behavior
```

---


### Managed_zone_operation

Fetches the representation of an existing Operation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Unique identifier for the resource. This is the client_operation_id if the client specified it when the mutation was initiated, otherwise, it is generated by the server. The name must be 1-63 characters long and match the regular expression [-a-z0-9]? (output only) |
| `dns_key_context` | String | Only populated if the operation targeted a DnsKey (output only). |
| `type` | String | Type of the operation. Operations include insert, update, and delete (output only). |
| `zone_context` | String | Only populated if the operation targeted a ManagedZone (output only). |
| `start_time` | String | The time that this operation was started by the server. This is in RFC3339 text format (output only). |
| `user` | String | User who requested the operation, for example: user@example.com. cloud-dns-system for operations automatically done by the system. (output only) |
| `status` | String | Status of the operation. Can be one of the following: "PENDING" or "DONE" (output only). A status of "DONE" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet. |
| `kind` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access managed_zone_operation outputs
managed_zone_operation_id = managed_zone_operation.id
managed_zone_operation_id = managed_zone_operation.id
managed_zone_operation_dns_key_context = managed_zone_operation.dns_key_context
managed_zone_operation_type = managed_zone_operation.type
managed_zone_operation_zone_context = managed_zone_operation.zone_context
managed_zone_operation_start_time = managed_zone_operation.start_time
managed_zone_operation_user = managed_zone_operation.user
managed_zone_operation_status = managed_zone_operation.status
managed_zone_operation_kind = managed_zone_operation.kind
```

---


### Policie

Creates a new policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_logging` | bool |  | Controls whether logging is enabled for the networks bound to this policy. Defaults to no logging if not set. |
| `kind` | String |  |  |
| `name` | String |  | User-assigned name for this policy. |
| `alternative_name_server_config` | String |  | Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified. |
| `networks` | Vec<String> |  | List of network names specifying networks to which this policy is applied. |
| `description` | String |  | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the policy's function. |
| `enable_inbound_forwarding` | bool |  | Allows networks bound to this policy to receive DNS queries sent by VMs or applications over VPN connections. When enabled, a virtual IP address is allocated from each of the subnetworks that are bound to this policy. |
| `dns64_config` | String |  | Configurations related to DNS64 for this policy. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only). |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_logging` | bool | Controls whether logging is enabled for the networks bound to this policy. Defaults to no logging if not set. |
| `kind` | String |  |
| `name` | String | User-assigned name for this policy. |
| `alternative_name_server_config` | String | Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified. |
| `networks` | Vec<String> | List of network names specifying networks to which this policy is applied. |
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the policy's function. |
| `enable_inbound_forwarding` | bool | Allows networks bound to this policy to receive DNS queries sent by VMs or applications over VPN connections. When enabled, a virtual IP address is allocated from each of the subnetworks that are bound to this policy. |
| `dns64_config` | String | Configurations related to DNS64 for this policy. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policie
policie = provider.dns_api.Policie {
    project = "value"  # Identifies the project addressed by this request.
}

# Access policie outputs
policie_id = policie.id
policie_enable_logging = policie.enable_logging
policie_kind = policie.kind
policie_name = policie.name
policie_alternative_name_server_config = policie.alternative_name_server_config
policie_networks = policie.networks
policie_description = policie.description
policie_enable_inbound_forwarding = policie.enable_inbound_forwarding
policie_dns64_config = policie.dns64_config
policie_id = policie.id
```

---


### Response_policie

Creates a new Response Policy

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gke_clusters` | Vec<String> |  | The list of Google Kubernetes Engine clusters to which this response policy is applied. |
| `kind` | String |  |  |
| `labels` | HashMap<String, String> |  | User labels. |
| `networks` | Vec<String> |  | List of network names specifying networks to which this policy is applied. |
| `description` | String |  | User-provided description for this Response Policy. |
| `response_policy_name` | String |  | User assigned name for this Response Policy. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only). |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `gke_clusters` | Vec<String> | The list of Google Kubernetes Engine clusters to which this response policy is applied. |
| `kind` | String |  |
| `labels` | HashMap<String, String> | User labels. |
| `networks` | Vec<String> | List of network names specifying networks to which this policy is applied. |
| `description` | String | User-provided description for this Response Policy. |
| `response_policy_name` | String | User assigned name for this Response Policy. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create response_policie
response_policie = provider.dns_api.Response_policie {
    project = "value"  # Identifies the project addressed by this request.
}

# Access response_policie outputs
response_policie_id = response_policie.id
response_policie_gke_clusters = response_policie.gke_clusters
response_policie_kind = response_policie.kind
response_policie_labels = response_policie.labels
response_policie_networks = response_policie.networks
response_policie_description = response_policie.description
response_policie_response_policy_name = response_policie.response_policy_name
response_policie_id = response_policie.id
```

---


### Resource_record_set

Creates a new ResourceRecordSet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | For example, www.example.com. |
| `routing_policy` | String |  | Configures dynamic query responses based on either the geo location of the querying user or a weighted round robin based routing policy. A valid `ResourceRecordSet` contains only `rrdata` (for static resolution) or a `routing_policy` (for dynamic resolution). |
| `rrdatas` | Vec<String> |  | As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) -- see examples. |
| `kind` | String |  |  |
| `ttl` | i64 |  | Number of seconds that this `ResourceRecordSet` can be cached by resolvers. |
| `type` | String |  | The identifier of a supported record type. See the list of Supported DNS record types. |
| `signature_rrdatas` | Vec<String> |  | As defined in RFC 4034 (section 3.2). |
| `project` | String | ✅ | Identifies the project addressed by this request. |
| `managed_zone` | String | ✅ | Identifies the managed zone addressed by this request. Can be the managed zone name or ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | For example, www.example.com. |
| `routing_policy` | String | Configures dynamic query responses based on either the geo location of the querying user or a weighted round robin based routing policy. A valid `ResourceRecordSet` contains only `rrdata` (for static resolution) or a `routing_policy` (for dynamic resolution). |
| `rrdatas` | Vec<String> | As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) -- see examples. |
| `kind` | String |  |
| `ttl` | i64 | Number of seconds that this `ResourceRecordSet` can be cached by resolvers. |
| `type` | String | The identifier of a supported record type. See the list of Supported DNS record types. |
| `signature_rrdatas` | Vec<String> | As defined in RFC 4034 (section 3.2). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create resource_record_set
resource_record_set = provider.dns_api.Resource_record_set {
    project = "value"  # Identifies the project addressed by this request.
    managed_zone = "value"  # Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
}

# Access resource_record_set outputs
resource_record_set_id = resource_record_set.id
resource_record_set_name = resource_record_set.name
resource_record_set_routing_policy = resource_record_set.routing_policy
resource_record_set_rrdatas = resource_record_set.rrdatas
resource_record_set_kind = resource_record_set.kind
resource_record_set_ttl = resource_record_set.ttl
resource_record_set_type = resource_record_set.type
resource_record_set_signature_rrdatas = resource_record_set.signature_rrdatas
```

---


### Managed_zone

Creates a new ManagedZone.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes. |
| `service_directory_config` | String |  | This field links to the associated service directory namespace. Do not set this field for public zones or forwarding zones. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only) |
| `forwarding_config` | String |  | The presence for this field indicates that outbound forwarding is enabled for this zone. The value of this field contains the set of destinations to forward to. |
| `kind` | String |  |  |
| `creation_time` | String |  | The time that this resource was created on the server. This is in RFC3339 text format. Output only. |
| `dns_name` | String |  | The DNS name of this managed zone, for instance "example.com.". |
| `dnssec_config` | String |  | DNSSEC configuration. |
| `name_servers` | Vec<String> |  | Delegate your managed_zone to these virtual name servers; defined by the server (output only) |
| `labels` | HashMap<String, String> |  | User labels. |
| `visibility` | String |  | The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources. |
| `private_visibility_config` | String |  | For privately visible zones, the set of Virtual Private Cloud resources that the zone is visible from. |
| `reverse_lookup_config` | String |  | The presence of this field indicates that this is a managed reverse lookup zone and Cloud DNS resolves reverse lookup queries using automatically configured records for VPC resources. This only applies to networks listed under private_visibility_config. |
| `description` | String |  | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function. |
| `name_server_set` | String |  | Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users leave this field unset. If you need to use this field, contact your account team. |
| `peering_config` | String |  | The presence of this field indicates that DNS Peering is enabled for this zone. The value of this field contains the network to peer with. |
| `cloud_logging_config` | String |  |  |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes. |
| `service_directory_config` | String | This field links to the associated service directory namespace. Do not set this field for public zones or forwarding zones. |
| `id` | String | Unique identifier for the resource; defined by the server (output only) |
| `forwarding_config` | String | The presence for this field indicates that outbound forwarding is enabled for this zone. The value of this field contains the set of destinations to forward to. |
| `kind` | String |  |
| `creation_time` | String | The time that this resource was created on the server. This is in RFC3339 text format. Output only. |
| `dns_name` | String | The DNS name of this managed zone, for instance "example.com.". |
| `dnssec_config` | String | DNSSEC configuration. |
| `name_servers` | Vec<String> | Delegate your managed_zone to these virtual name servers; defined by the server (output only) |
| `labels` | HashMap<String, String> | User labels. |
| `visibility` | String | The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources. |
| `private_visibility_config` | String | For privately visible zones, the set of Virtual Private Cloud resources that the zone is visible from. |
| `reverse_lookup_config` | String | The presence of this field indicates that this is a managed reverse lookup zone and Cloud DNS resolves reverse lookup queries using automatically configured records for VPC resources. This only applies to networks listed under private_visibility_config. |
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function. |
| `name_server_set` | String | Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users leave this field unset. If you need to use this field, contact your account team. |
| `peering_config` | String | The presence of this field indicates that DNS Peering is enabled for this zone. The value of this field contains the network to peer with. |
| `cloud_logging_config` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create managed_zone
managed_zone = provider.dns_api.Managed_zone {
    project = "value"  # Identifies the project addressed by this request.
}

# Access managed_zone outputs
managed_zone_id = managed_zone.id
managed_zone_name = managed_zone.name
managed_zone_service_directory_config = managed_zone.service_directory_config
managed_zone_id = managed_zone.id
managed_zone_forwarding_config = managed_zone.forwarding_config
managed_zone_kind = managed_zone.kind
managed_zone_creation_time = managed_zone.creation_time
managed_zone_dns_name = managed_zone.dns_name
managed_zone_dnssec_config = managed_zone.dnssec_config
managed_zone_name_servers = managed_zone.name_servers
managed_zone_labels = managed_zone.labels
managed_zone_visibility = managed_zone.visibility
managed_zone_private_visibility_config = managed_zone.private_visibility_config
managed_zone_reverse_lookup_config = managed_zone.reverse_lookup_config
managed_zone_description = managed_zone.description
managed_zone_name_server_set = managed_zone.name_server_set
managed_zone_peering_config = managed_zone.peering_config
managed_zone_cloud_logging_config = managed_zone.cloud_logging_config
```

---


### Dns_key

Fetches the representation of an existing DnsKey.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `key_tag` | i64 | The key tag is a non-cryptographic hash of the a DNSKEY resource record associated with this DnsKey. The key tag can be used to identify a DNSKEY more quickly (but it is not a unique identifier). In particular, the key tag is used in a parent zone's DS record to point at the DNSKEY in this child ManagedZone. The key tag is a number in the range [0, 65535] and the algorithm to calculate it is specified in RFC4034 Appendix B. Output only. |
| `public_key` | String | Base64 encoded public half of this key. Output only. |
| `type` | String | One of "KEY_SIGNING" or "ZONE_SIGNING". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, are used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag is cleared, and this key is used to sign only resource record sets of other types. Immutable after creation time. |
| `is_active` | bool | Active keys are used to sign subsequent changes to the ManagedZone. Inactive keys are still present as DNSKEY Resource Records for the use of resolvers validating existing signatures. |
| `algorithm` | String | String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time. |
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the resource's function. |
| `creation_time` | String | The time that this resource was created in the control plane. This is in RFC3339 text format. Output only. |
| `key_length` | i64 | Length of the key in bits. Specified at creation time, and then immutable. |
| `kind` | String |  |
| `digests` | Vec<String> | Cryptographic hashes of the DNSKEY resource record associated with this DnsKey. These digests are needed to construct a DS record that points at this DNS key. Output only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access dns_key outputs
dns_key_id = dns_key.id
dns_key_id = dns_key.id
dns_key_key_tag = dns_key.key_tag
dns_key_public_key = dns_key.public_key
dns_key_type = dns_key.type
dns_key_is_active = dns_key.is_active
dns_key_algorithm = dns_key.algorithm
dns_key_description = dns_key.description
dns_key_creation_time = dns_key.creation_time
dns_key_key_length = dns_key.key_length
dns_key_kind = dns_key.kind
dns_key_digests = dns_key.digests
```

---


### Managed_zone_operation

Fetches the representation of an existing Operation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String |  |
| `user` | String | User who requested the operation, for example: user@example.com. cloud-dns-system for operations automatically done by the system. (output only) |
| `zone_context` | String | Only populated if the operation targeted a ManagedZone (output only). |
| `dns_key_context` | String | Only populated if the operation targeted a DnsKey (output only). |
| `id` | String | Unique identifier for the resource. This is the client_operation_id if the client specified it when the mutation was initiated, otherwise, it is generated by the server. The name must be 1-63 characters long and match the regular expression [-a-z0-9]? (output only) |
| `status` | String | Status of the operation. Can be one of the following: "PENDING" or "DONE" (output only). A status of "DONE" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet. |
| `start_time` | String | The time that this operation was started by the server. This is in RFC3339 text format (output only). |
| `type` | String | Type of the operation. Operations include insert, update, and delete (output only). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access managed_zone_operation outputs
managed_zone_operation_id = managed_zone_operation.id
managed_zone_operation_kind = managed_zone_operation.kind
managed_zone_operation_user = managed_zone_operation.user
managed_zone_operation_zone_context = managed_zone_operation.zone_context
managed_zone_operation_dns_key_context = managed_zone_operation.dns_key_context
managed_zone_operation_id = managed_zone_operation.id
managed_zone_operation_status = managed_zone_operation.status
managed_zone_operation_start_time = managed_zone_operation.start_time
managed_zone_operation_type = managed_zone_operation.type
```

---


### Response_policie

Creates a new Response Policy

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `networks` | Vec<String> |  | List of network names specifying networks to which this policy is applied. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only). |
| `kind` | String |  |  |
| `response_policy_name` | String |  | User assigned name for this Response Policy. |
| `labels` | HashMap<String, String> |  | User labels. |
| `description` | String |  | User-provided description for this Response Policy. |
| `gke_clusters` | Vec<String> |  | The list of Google Kubernetes Engine clusters to which this response policy is applied. |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `networks` | Vec<String> | List of network names specifying networks to which this policy is applied. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `kind` | String |  |
| `response_policy_name` | String | User assigned name for this Response Policy. |
| `labels` | HashMap<String, String> | User labels. |
| `description` | String | User-provided description for this Response Policy. |
| `gke_clusters` | Vec<String> | The list of Google Kubernetes Engine clusters to which this response policy is applied. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create response_policie
response_policie = provider.dns_api.Response_policie {
    project = "value"  # Identifies the project addressed by this request.
}

# Access response_policie outputs
response_policie_id = response_policie.id
response_policie_networks = response_policie.networks
response_policie_id = response_policie.id
response_policie_kind = response_policie.kind
response_policie_response_policy_name = response_policie.response_policy_name
response_policie_labels = response_policie.labels
response_policie_description = response_policie.description
response_policie_gke_clusters = response_policie.gke_clusters
```

---


### Policie

Creates a new policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Unique identifier for the resource; defined by the server (output only). |
| `description` | String |  | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the policy's function. |
| `dns64_config` | String |  | Configurations related to DNS64 for this policy. |
| `networks` | Vec<String> |  | List of network names specifying networks to which this policy is applied. |
| `kind` | String |  |  |
| `enable_inbound_forwarding` | bool |  | Allows networks bound to this policy to receive DNS queries sent by VMs or applications over VPN connections. When enabled, a virtual IP address is allocated from each of the subnetworks that are bound to this policy. |
| `name` | String |  | User-assigned name for this policy. |
| `alternative_name_server_config` | String |  | Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified. |
| `enable_logging` | bool |  | Controls whether logging is enabled for the networks bound to this policy. Defaults to no logging if not set. |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the policy's function. |
| `dns64_config` | String | Configurations related to DNS64 for this policy. |
| `networks` | Vec<String> | List of network names specifying networks to which this policy is applied. |
| `kind` | String |  |
| `enable_inbound_forwarding` | bool | Allows networks bound to this policy to receive DNS queries sent by VMs or applications over VPN connections. When enabled, a virtual IP address is allocated from each of the subnetworks that are bound to this policy. |
| `name` | String | User-assigned name for this policy. |
| `alternative_name_server_config` | String | Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified. |
| `enable_logging` | bool | Controls whether logging is enabled for the networks bound to this policy. Defaults to no logging if not set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policie
policie = provider.dns_api.Policie {
    project = "value"  # Identifies the project addressed by this request.
}

# Access policie outputs
policie_id = policie.id
policie_id = policie.id
policie_description = policie.description
policie_dns64_config = policie.dns64_config
policie_networks = policie.networks
policie_kind = policie.kind
policie_enable_inbound_forwarding = policie.enable_inbound_forwarding
policie_name = policie.name
policie_alternative_name_server_config = policie.alternative_name_server_config
policie_enable_logging = policie.enable_logging
```

---


### Response_policy_rule

Creates a new Response Policy Rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `local_data` | String |  | Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name; in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed. |
| `behavior` | String |  | Answer this query with a behavior rather than DNS data. |
| `dns_name` | String |  | The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule. |
| `rule_name` | String |  | An identifier for this rule. Must be unique with the ResponsePolicy. |
| `kind` | String |  |  |
| `project` | String | ✅ | Identifies the project addressed by this request. |
| `response_policy` | String | ✅ | User assigned name of the Response Policy containing the Response Policy Rule. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `local_data` | String | Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name; in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed. |
| `behavior` | String | Answer this query with a behavior rather than DNS data. |
| `dns_name` | String | The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule. |
| `rule_name` | String | An identifier for this rule. Must be unique with the ResponsePolicy. |
| `kind` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create response_policy_rule
response_policy_rule = provider.dns_api.Response_policy_rule {
    project = "value"  # Identifies the project addressed by this request.
    response_policy = "value"  # User assigned name of the Response Policy containing the Response Policy Rule.
}

# Access response_policy_rule outputs
response_policy_rule_id = response_policy_rule.id
response_policy_rule_local_data = response_policy_rule.local_data
response_policy_rule_behavior = response_policy_rule.behavior
response_policy_rule_dns_name = response_policy_rule.dns_name
response_policy_rule_rule_name = response_policy_rule.rule_name
response_policy_rule_kind = response_policy_rule.kind
```

---


### Resource_record_set

Creates a new ResourceRecordSet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  |  |
| `rrdatas` | Vec<String> |  | As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) -- see examples. |
| `signature_rrdatas` | Vec<String> |  | As defined in RFC 4034 (section 3.2). |
| `ttl` | i64 |  | Number of seconds that this `ResourceRecordSet` can be cached by resolvers. |
| `type` | String |  | The identifier of a supported record type. See the list of Supported DNS record types. |
| `name` | String |  | For example, www.example.com. |
| `routing_policy` | String |  | Configures dynamic query responses based on either the geo location of the querying user or a weighted round robin based routing policy. A valid `ResourceRecordSet` contains only `rrdata` (for static resolution) or a `routing_policy` (for dynamic resolution). |
| `project` | String | ✅ | Identifies the project addressed by this request. |
| `managed_zone` | String | ✅ | Identifies the managed zone addressed by this request. Can be the managed zone name or ID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String |  |
| `rrdatas` | Vec<String> | As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) -- see examples. |
| `signature_rrdatas` | Vec<String> | As defined in RFC 4034 (section 3.2). |
| `ttl` | i64 | Number of seconds that this `ResourceRecordSet` can be cached by resolvers. |
| `type` | String | The identifier of a supported record type. See the list of Supported DNS record types. |
| `name` | String | For example, www.example.com. |
| `routing_policy` | String | Configures dynamic query responses based on either the geo location of the querying user or a weighted round robin based routing policy. A valid `ResourceRecordSet` contains only `rrdata` (for static resolution) or a `routing_policy` (for dynamic resolution). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create resource_record_set
resource_record_set = provider.dns_api.Resource_record_set {
    project = "value"  # Identifies the project addressed by this request.
    managed_zone = "value"  # Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
}

# Access resource_record_set outputs
resource_record_set_id = resource_record_set.id
resource_record_set_kind = resource_record_set.kind
resource_record_set_rrdatas = resource_record_set.rrdatas
resource_record_set_signature_rrdatas = resource_record_set.signature_rrdatas
resource_record_set_ttl = resource_record_set.ttl
resource_record_set_type = resource_record_set.type
resource_record_set_name = resource_record_set.name
resource_record_set_routing_policy = resource_record_set.routing_policy
```

---


### Dns_key

Fetches the representation of an existing DnsKey.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `key_tag` | i64 | The key tag is a non-cryptographic hash of the a DNSKEY resource record associated with this DnsKey. The key tag can be used to identify a DNSKEY more quickly (but it is not a unique identifier). In particular, the key tag is used in a parent zone's DS record to point at the DNSKEY in this child ManagedZone. The key tag is a number in the range [0, 65535] and the algorithm to calculate it is specified in RFC4034 Appendix B. Output only. |
| `digests` | Vec<String> | Cryptographic hashes of the DNSKEY resource record associated with this DnsKey. These digests are needed to construct a DS record that points at this DNS key. Output only. |
| `key_length` | i64 | Length of the key in bits. Specified at creation time, and then immutable. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `public_key` | String | Base64 encoded public half of this key. Output only. |
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the resource's function. |
| `is_active` | bool | Active keys are used to sign subsequent changes to the ManagedZone. Inactive keys are still present as DNSKEY Resource Records for the use of resolvers validating existing signatures. |
| `creation_time` | String | The time that this resource was created in the control plane. This is in RFC3339 text format. Output only. |
| `kind` | String |  |
| `algorithm` | String | String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time. |
| `type` | String | One of "KEY_SIGNING" or "ZONE_SIGNING". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, are used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag is cleared, and this key is used to sign only resource record sets of other types. Immutable after creation time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access dns_key outputs
dns_key_id = dns_key.id
dns_key_key_tag = dns_key.key_tag
dns_key_digests = dns_key.digests
dns_key_key_length = dns_key.key_length
dns_key_id = dns_key.id
dns_key_public_key = dns_key.public_key
dns_key_description = dns_key.description
dns_key_is_active = dns_key.is_active
dns_key_creation_time = dns_key.creation_time
dns_key_kind = dns_key.kind
dns_key_algorithm = dns_key.algorithm
dns_key_type = dns_key.type
```

---


### Change

Atomically updates the ResourceRecordSet collection.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `is_serving` | bool |  | If the DNS queries for the zone will be served. |
| `start_time` | String |  | The time that this operation was started by the server (output only). This is in RFC3339 text format. |
| `status` | String |  | Status of the operation (output only). A status of "done" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet. |
| `additions` | Vec<String> |  | Which ResourceRecordSets to add? |
| `deletions` | Vec<String> |  | Which ResourceRecordSets to remove? Must match existing data exactly. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only). |
| `kind` | String |  |  |
| `managed_zone` | String | ✅ | Identifies the managed zone addressed by this request. Can be the managed zone name or ID. |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_serving` | bool | If the DNS queries for the zone will be served. |
| `start_time` | String | The time that this operation was started by the server (output only). This is in RFC3339 text format. |
| `status` | String | Status of the operation (output only). A status of "done" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet. |
| `additions` | Vec<String> | Which ResourceRecordSets to add? |
| `deletions` | Vec<String> | Which ResourceRecordSets to remove? Must match existing data exactly. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `kind` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create change
change = provider.dns_api.Change {
    managed_zone = "value"  # Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    project = "value"  # Identifies the project addressed by this request.
}

# Access change outputs
change_id = change.id
change_is_serving = change.is_serving
change_start_time = change.start_time
change_status = change.status
change_additions = change.additions
change_deletions = change.deletions
change_id = change.id
change_kind = change.kind
```

---


### Project

Fetches the representation of an existing Project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `number` | String | Unique numeric identifier for the resource; defined by the server (output only). |
| `quota` | String | Quotas assigned to this project (output only). |
| `kind` | String |  |
| `id` | String | User assigned unique identifier for the resource (output only). |


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
project_number = project.number
project_quota = project.quota
project_kind = project.kind
project_id = project.id
```

---


### Managed_zone

Creates a new ManagedZone.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reverse_lookup_config` | String |  | The presence of this field indicates that this is a managed reverse lookup zone and Cloud DNS resolves reverse lookup queries using automatically configured records for VPC resources. This only applies to networks listed under private_visibility_config. |
| `creation_time` | String |  | The time that this resource was created on the server. This is in RFC3339 text format. Output only. |
| `kind` | String |  |  |
| `service_directory_config` | String |  | This field links to the associated service directory namespace. Do not set this field for public zones or forwarding zones. |
| `labels` | HashMap<String, String> |  | User labels. |
| `name` | String |  | User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes. |
| `description` | String |  | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function. |
| `private_visibility_config` | String |  | For privately visible zones, the set of Virtual Private Cloud resources that the zone is visible from. |
| `dns_name` | String |  | The DNS name of this managed zone, for instance "example.com.". |
| `name_server_set` | String |  | Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users leave this field unset. If you need to use this field, contact your account team. |
| `visibility` | String |  | The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources. |
| `cloud_logging_config` | String |  |  |
| `dnssec_config` | String |  | DNSSEC configuration. |
| `name_servers` | Vec<String> |  | Delegate your managed_zone to these virtual name servers; defined by the server (output only) |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only) |
| `forwarding_config` | String |  | The presence for this field indicates that outbound forwarding is enabled for this zone. The value of this field contains the set of destinations to forward to. |
| `peering_config` | String |  | The presence of this field indicates that DNS Peering is enabled for this zone. The value of this field contains the network to peer with. |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `reverse_lookup_config` | String | The presence of this field indicates that this is a managed reverse lookup zone and Cloud DNS resolves reverse lookup queries using automatically configured records for VPC resources. This only applies to networks listed under private_visibility_config. |
| `creation_time` | String | The time that this resource was created on the server. This is in RFC3339 text format. Output only. |
| `kind` | String |  |
| `service_directory_config` | String | This field links to the associated service directory namespace. Do not set this field for public zones or forwarding zones. |
| `labels` | HashMap<String, String> | User labels. |
| `name` | String | User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes. |
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function. |
| `private_visibility_config` | String | For privately visible zones, the set of Virtual Private Cloud resources that the zone is visible from. |
| `dns_name` | String | The DNS name of this managed zone, for instance "example.com.". |
| `name_server_set` | String | Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users leave this field unset. If you need to use this field, contact your account team. |
| `visibility` | String | The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources. |
| `cloud_logging_config` | String |  |
| `dnssec_config` | String | DNSSEC configuration. |
| `name_servers` | Vec<String> | Delegate your managed_zone to these virtual name servers; defined by the server (output only) |
| `id` | String | Unique identifier for the resource; defined by the server (output only) |
| `forwarding_config` | String | The presence for this field indicates that outbound forwarding is enabled for this zone. The value of this field contains the set of destinations to forward to. |
| `peering_config` | String | The presence of this field indicates that DNS Peering is enabled for this zone. The value of this field contains the network to peer with. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create managed_zone
managed_zone = provider.dns_api.Managed_zone {
    project = "value"  # Identifies the project addressed by this request.
}

# Access managed_zone outputs
managed_zone_id = managed_zone.id
managed_zone_reverse_lookup_config = managed_zone.reverse_lookup_config
managed_zone_creation_time = managed_zone.creation_time
managed_zone_kind = managed_zone.kind
managed_zone_service_directory_config = managed_zone.service_directory_config
managed_zone_labels = managed_zone.labels
managed_zone_name = managed_zone.name
managed_zone_description = managed_zone.description
managed_zone_private_visibility_config = managed_zone.private_visibility_config
managed_zone_dns_name = managed_zone.dns_name
managed_zone_name_server_set = managed_zone.name_server_set
managed_zone_visibility = managed_zone.visibility
managed_zone_cloud_logging_config = managed_zone.cloud_logging_config
managed_zone_dnssec_config = managed_zone.dnssec_config
managed_zone_name_servers = managed_zone.name_servers
managed_zone_id = managed_zone.id
managed_zone_forwarding_config = managed_zone.forwarding_config
managed_zone_peering_config = managed_zone.peering_config
```

---


### Resource_record_set

Enumerate ResourceRecordSets that have been created but not yet deleted.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your pagination token. In this way you can retrieve the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned will be an inconsistent view of the collection. There is no way to retrieve a consistent snapshot of a collection larger than the maximum page size. |
| `header` | String |  |
| `kind` | String | Type of resource. |
| `rrsets` | Vec<String> | The resource record set resources. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access resource_record_set outputs
resource_record_set_id = resource_record_set.id
resource_record_set_next_page_token = resource_record_set.next_page_token
resource_record_set_header = resource_record_set.header
resource_record_set_kind = resource_record_set.kind
resource_record_set_rrsets = resource_record_set.rrsets
```

---


### Project

Fetch the representation of an existing Project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String |  |
| `id` | String | User assigned unique identifier for the resource (output only). |
| `number` | String | Unique numeric identifier for the resource; defined by the server (output only). |
| `quota` | String | Quotas assigned to this project (output only). |


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
project_kind = project.kind
project_id = project.id
project_number = project.number
project_quota = project.quota
```

---


### Managed_zone_operation

Fetch the representation of an existing Operation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dns_key_context` | String | Only populated if the operation targeted a DnsKey (output only). |
| `status` | String | Status of the operation. Can be one of the following: "PENDING" or "DONE" (output only). A status of "DONE" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet. |
| `start_time` | String | The time that this operation was started by the server. This is in RFC3339 text format (output only). |
| `zone_context` | String | Only populated if the operation targeted a ManagedZone (output only). |
| `id` | String | Unique identifier for the resource. This is the client_operation_id if the client specified it when the mutation was initiated, otherwise, it is generated by the server. The name must be 1-63 characters long and match the regular expression [-a-z0-9]? (output only) |
| `kind` | String |  |
| `type` | String | Type of the operation. Operations include insert, update, and delete (output only). |
| `user` | String | User who requested the operation, for example: user@example.com. cloud-dns-system for operations automatically done by the system. (output only) |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access managed_zone_operation outputs
managed_zone_operation_id = managed_zone_operation.id
managed_zone_operation_dns_key_context = managed_zone_operation.dns_key_context
managed_zone_operation_status = managed_zone_operation.status
managed_zone_operation_start_time = managed_zone_operation.start_time
managed_zone_operation_zone_context = managed_zone_operation.zone_context
managed_zone_operation_id = managed_zone_operation.id
managed_zone_operation_kind = managed_zone_operation.kind
managed_zone_operation_type = managed_zone_operation.type
managed_zone_operation_user = managed_zone_operation.user
```

---


### Managed_zone

Create a new ManagedZone.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dns_name` | String |  | The DNS name of this managed zone, for instance "example.com.". |
| `name_servers` | Vec<String> |  | Delegate your managed_zone to these virtual name servers; defined by the server (output only) |
| `reverse_lookup_config` | String |  | The presence of this field indicates that this is a managed reverse lookup zone and Cloud DNS will resolve reverse lookup queries using automatically configured records for VPC resources. This only applies to networks listed under private_visibility_config. |
| `creation_time` | String |  | The time that this resource was created on the server. This is in RFC3339 text format. Output only. |
| `description` | String |  | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function. |
| `name` | String |  | User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes. |
| `peering_config` | String |  | The presence of this field indicates that DNS Peering is enabled for this zone. The value of this field contains the network to peer with. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only) |
| `private_visibility_config` | String |  | For privately visible zones, the set of Virtual Private Cloud resources that the zone is visible from. |
| `name_server_set` | String |  | Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users will leave this field unset. |
| `forwarding_config` | String |  | The presence for this field indicates that outbound forwarding is enabled for this zone. The value of this field contains the set of destinations to forward to. |
| `labels` | HashMap<String, String> |  | User labels. |
| `dnssec_config` | String |  | DNSSEC configuration. |
| `kind` | String |  |  |
| `visibility` | String |  | The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources. |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dns_name` | String | The DNS name of this managed zone, for instance "example.com.". |
| `name_servers` | Vec<String> | Delegate your managed_zone to these virtual name servers; defined by the server (output only) |
| `reverse_lookup_config` | String | The presence of this field indicates that this is a managed reverse lookup zone and Cloud DNS will resolve reverse lookup queries using automatically configured records for VPC resources. This only applies to networks listed under private_visibility_config. |
| `creation_time` | String | The time that this resource was created on the server. This is in RFC3339 text format. Output only. |
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function. |
| `name` | String | User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes. |
| `peering_config` | String | The presence of this field indicates that DNS Peering is enabled for this zone. The value of this field contains the network to peer with. |
| `id` | String | Unique identifier for the resource; defined by the server (output only) |
| `private_visibility_config` | String | For privately visible zones, the set of Virtual Private Cloud resources that the zone is visible from. |
| `name_server_set` | String | Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users will leave this field unset. |
| `forwarding_config` | String | The presence for this field indicates that outbound forwarding is enabled for this zone. The value of this field contains the set of destinations to forward to. |
| `labels` | HashMap<String, String> | User labels. |
| `dnssec_config` | String | DNSSEC configuration. |
| `kind` | String |  |
| `visibility` | String | The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create managed_zone
managed_zone = provider.dns_api.Managed_zone {
    project = "value"  # Identifies the project addressed by this request.
}

# Access managed_zone outputs
managed_zone_id = managed_zone.id
managed_zone_dns_name = managed_zone.dns_name
managed_zone_name_servers = managed_zone.name_servers
managed_zone_reverse_lookup_config = managed_zone.reverse_lookup_config
managed_zone_creation_time = managed_zone.creation_time
managed_zone_description = managed_zone.description
managed_zone_name = managed_zone.name
managed_zone_peering_config = managed_zone.peering_config
managed_zone_id = managed_zone.id
managed_zone_private_visibility_config = managed_zone.private_visibility_config
managed_zone_name_server_set = managed_zone.name_server_set
managed_zone_forwarding_config = managed_zone.forwarding_config
managed_zone_labels = managed_zone.labels
managed_zone_dnssec_config = managed_zone.dnssec_config
managed_zone_kind = managed_zone.kind
managed_zone_visibility = managed_zone.visibility
```

---


### Change

Atomically update the ResourceRecordSet collection.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  |  |
| `additions` | Vec<String> |  | Which ResourceRecordSets to add? |
| `deletions` | Vec<String> |  | Which ResourceRecordSets to remove? Must match existing data exactly. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only). |
| `is_serving` | bool |  | If the DNS queries for the zone will be served. |
| `start_time` | String |  | The time that this operation was started by the server (output only). This is in RFC3339 text format. |
| `status` | String |  | Status of the operation (output only). A status of "done" means that the request to update the authoritative servers has been sent but the servers might not be updated yet. |
| `project` | String | ✅ | Identifies the project addressed by this request. |
| `managed_zone` | String | ✅ | Identifies the managed zone addressed by this request. Can be the managed zone name or id. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String |  |
| `additions` | Vec<String> | Which ResourceRecordSets to add? |
| `deletions` | Vec<String> | Which ResourceRecordSets to remove? Must match existing data exactly. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `is_serving` | bool | If the DNS queries for the zone will be served. |
| `start_time` | String | The time that this operation was started by the server (output only). This is in RFC3339 text format. |
| `status` | String | Status of the operation (output only). A status of "done" means that the request to update the authoritative servers has been sent but the servers might not be updated yet. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create change
change = provider.dns_api.Change {
    project = "value"  # Identifies the project addressed by this request.
    managed_zone = "value"  # Identifies the managed zone addressed by this request. Can be the managed zone name or id.
}

# Access change outputs
change_id = change.id
change_kind = change.kind
change_additions = change.additions
change_deletions = change.deletions
change_id = change.id
change_is_serving = change.is_serving
change_start_time = change.start_time
change_status = change.status
```

---


### Policie

Create a new Policy

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enable_logging` | bool |  | Controls whether logging is enabled for the networks bound to this policy. Defaults to no logging if not set. |
| `name` | String |  | User assigned name for this policy. |
| `enable_inbound_forwarding` | bool |  | Allows networks bound to this policy to receive DNS queries sent by VMs or applications over VPN connections. When enabled, a virtual IP address will be allocated from each of the sub-networks that are bound to this policy. |
| `alternative_name_server_config` | String |  | Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified. |
| `description` | String |  | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the policy's function. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only). |
| `kind` | String |  |  |
| `networks` | Vec<String> |  | List of network names specifying networks to which this policy is applied. |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enable_logging` | bool | Controls whether logging is enabled for the networks bound to this policy. Defaults to no logging if not set. |
| `name` | String | User assigned name for this policy. |
| `enable_inbound_forwarding` | bool | Allows networks bound to this policy to receive DNS queries sent by VMs or applications over VPN connections. When enabled, a virtual IP address will be allocated from each of the sub-networks that are bound to this policy. |
| `alternative_name_server_config` | String | Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified. |
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the policy's function. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `kind` | String |  |
| `networks` | Vec<String> | List of network names specifying networks to which this policy is applied. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policie
policie = provider.dns_api.Policie {
    project = "value"  # Identifies the project addressed by this request.
}

# Access policie outputs
policie_id = policie.id
policie_enable_logging = policie.enable_logging
policie_name = policie.name
policie_enable_inbound_forwarding = policie.enable_inbound_forwarding
policie_alternative_name_server_config = policie.alternative_name_server_config
policie_description = policie.description
policie_id = policie.id
policie_kind = policie.kind
policie_networks = policie.networks
```

---


### Dns_key

Fetch the representation of an existing DnsKey.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | The time that this resource was created in the control plane. This is in RFC3339 text format. Output only. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `key_length` | i64 | Length of the key in bits. Specified at creation time then immutable. |
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the resource's function. |
| `key_tag` | i64 | The key tag is a non-cryptographic hash of the a DNSKEY resource record associated with this DnsKey. The key tag can be used to identify a DNSKEY more quickly (but it is not a unique identifier). In particular, the key tag is used in a parent zone's DS record to point at the DNSKEY in this child ManagedZone. The key tag is a number in the range [0, 65535] and the algorithm to calculate it is specified in RFC4034 Appendix B. Output only. |
| `is_active` | bool | Active keys will be used to sign subsequent changes to the ManagedZone. Inactive keys will still be present as DNSKEY Resource Records for the use of resolvers validating existing signatures. |
| `type` | String | One of "KEY_SIGNING" or "ZONE_SIGNING". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, will be used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag will be cleared and this key will be used to sign only resource record sets of other types. Immutable after creation time. |
| `kind` | String |  |
| `public_key` | String | Base64 encoded public half of this key. Output only. |
| `digests` | Vec<String> | Cryptographic hashes of the DNSKEY resource record associated with this DnsKey. These digests are needed to construct a DS record that points at this DNS key. Output only. |
| `algorithm` | String | String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access dns_key outputs
dns_key_id = dns_key.id
dns_key_creation_time = dns_key.creation_time
dns_key_id = dns_key.id
dns_key_key_length = dns_key.key_length
dns_key_description = dns_key.description
dns_key_key_tag = dns_key.key_tag
dns_key_is_active = dns_key.is_active
dns_key_type = dns_key.type
dns_key_kind = dns_key.kind
dns_key_public_key = dns_key.public_key
dns_key_digests = dns_key.digests
dns_key_algorithm = dns_key.algorithm
```

---


### Change

Atomically updates the ResourceRecordSet collection.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Unique identifier for the resource; defined by the server (output only). |
| `start_time` | String |  | The time that this operation was started by the server (output only). This is in RFC3339 text format. |
| `is_serving` | bool |  | If the DNS queries for the zone will be served. |
| `status` | String |  | Status of the operation (output only). A status of "done" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet. |
| `deletions` | Vec<String> |  | Which ResourceRecordSets to remove? Must match existing data exactly. |
| `additions` | Vec<String> |  | Which ResourceRecordSets to add? |
| `kind` | String |  |  |
| `managed_zone` | String | ✅ | Identifies the managed zone addressed by this request. Can be the managed zone name or ID. |
| `project` | String | ✅ | Identifies the project addressed by this request. |
| `location` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `start_time` | String | The time that this operation was started by the server (output only). This is in RFC3339 text format. |
| `is_serving` | bool | If the DNS queries for the zone will be served. |
| `status` | String | Status of the operation (output only). A status of "done" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet. |
| `deletions` | Vec<String> | Which ResourceRecordSets to remove? Must match existing data exactly. |
| `additions` | Vec<String> | Which ResourceRecordSets to add? |
| `kind` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create change
change = provider.dns_api.Change {
    managed_zone = "value"  # Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    project = "value"  # Identifies the project addressed by this request.
    location = "value"  # Required field
}

# Access change outputs
change_id = change.id
change_id = change.id
change_start_time = change.start_time
change_is_serving = change.is_serving
change_status = change.status
change_deletions = change.deletions
change_additions = change.additions
change_kind = change.kind
```

---


### Dns_key

Fetches the representation of an existing DnsKey.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | The time that this resource was created in the control plane. This is in RFC3339 text format. Output only. |
| `is_active` | bool | Active keys are used to sign subsequent changes to the ManagedZone. Inactive keys are still present as DNSKEY Resource Records for the use of resolvers validating existing signatures. |
| `algorithm` | String | String mnemonic specifying the DNSSEC algorithm of this key. Immutable after creation time. |
| `key_tag` | i64 | The key tag is a non-cryptographic hash of the a DNSKEY resource record associated with this DnsKey. The key tag can be used to identify a DNSKEY more quickly (but it is not a unique identifier). In particular, the key tag is used in a parent zone's DS record to point at the DNSKEY in this child ManagedZone. The key tag is a number in the range [0, 65535] and the algorithm to calculate it is specified in RFC4034 Appendix B. Output only. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `type` | String | One of "KEY_SIGNING" or "ZONE_SIGNING". Keys of type KEY_SIGNING have the Secure Entry Point flag set and, when active, are used to sign only resource record sets of type DNSKEY. Otherwise, the Secure Entry Point flag is cleared, and this key is used to sign only resource record sets of other types. Immutable after creation time. |
| `digests` | Vec<String> | Cryptographic hashes of the DNSKEY resource record associated with this DnsKey. These digests are needed to construct a DS record that points at this DNS key. Output only. |
| `public_key` | String | Base64 encoded public half of this key. Output only. |
| `key_length` | i64 | Length of the key in bits. Specified at creation time, and then immutable. |
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the resource's function. |
| `kind` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access dns_key outputs
dns_key_id = dns_key.id
dns_key_creation_time = dns_key.creation_time
dns_key_is_active = dns_key.is_active
dns_key_algorithm = dns_key.algorithm
dns_key_key_tag = dns_key.key_tag
dns_key_id = dns_key.id
dns_key_type = dns_key.type
dns_key_digests = dns_key.digests
dns_key_public_key = dns_key.public_key
dns_key_key_length = dns_key.key_length
dns_key_description = dns_key.description
dns_key_kind = dns_key.kind
```

---


### Managed_zone_operation

Fetches the representation of an existing Operation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user` | String | User who requested the operation, for example: user@example.com. cloud-dns-system for operations automatically done by the system. (output only) |
| `kind` | String |  |
| `status` | String | Status of the operation. Can be one of the following: "PENDING" or "DONE" (output only). A status of "DONE" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet. |
| `id` | String | Unique identifier for the resource. This is the client_operation_id if the client specified it when the mutation was initiated, otherwise, it is generated by the server. The name must be 1-63 characters long and match the regular expression [-a-z0-9]? (output only) |
| `dns_key_context` | String | Only populated if the operation targeted a DnsKey (output only). |
| `start_time` | String | The time that this operation was started by the server. This is in RFC3339 text format (output only). |
| `zone_context` | String | Only populated if the operation targeted a ManagedZone (output only). |
| `type` | String | Type of the operation. Operations include insert, update, and delete (output only). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access managed_zone_operation outputs
managed_zone_operation_id = managed_zone_operation.id
managed_zone_operation_user = managed_zone_operation.user
managed_zone_operation_kind = managed_zone_operation.kind
managed_zone_operation_status = managed_zone_operation.status
managed_zone_operation_id = managed_zone_operation.id
managed_zone_operation_dns_key_context = managed_zone_operation.dns_key_context
managed_zone_operation_start_time = managed_zone_operation.start_time
managed_zone_operation_zone_context = managed_zone_operation.zone_context
managed_zone_operation_type = managed_zone_operation.type
```

---


### Resource_record_set

Creates a new ResourceRecordSet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rrdatas` | Vec<String> |  | As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) -- see examples. |
| `type` | String |  | The identifier of a supported record type. See the list of Supported DNS record types. |
| `routing_policy` | String |  | Configures dynamic query responses based on geo location of querying user or a weighted round robin based routing policy. A ResourceRecordSet should only have either rrdata (static) or routing_policy (dynamic). An error is returned otherwise. |
| `kind` | String |  |  |
| `signature_rrdatas` | Vec<String> |  | As defined in RFC 4034 (section 3.2). |
| `name` | String |  | For example, www.example.com. |
| `ttl` | i64 |  | Number of seconds that this ResourceRecordSet can be cached by resolvers. |
| `managed_zone` | String | ✅ | Identifies the managed zone addressed by this request. Can be the managed zone name or ID. |
| `project` | String | ✅ | Identifies the project addressed by this request. |
| `location` | String | ✅ | Specifies the location of the resource. This information will be used for routing and will be part of the resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rrdatas` | Vec<String> | As defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1) -- see examples. |
| `type` | String | The identifier of a supported record type. See the list of Supported DNS record types. |
| `routing_policy` | String | Configures dynamic query responses based on geo location of querying user or a weighted round robin based routing policy. A ResourceRecordSet should only have either rrdata (static) or routing_policy (dynamic). An error is returned otherwise. |
| `kind` | String |  |
| `signature_rrdatas` | Vec<String> | As defined in RFC 4034 (section 3.2). |
| `name` | String | For example, www.example.com. |
| `ttl` | i64 | Number of seconds that this ResourceRecordSet can be cached by resolvers. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create resource_record_set
resource_record_set = provider.dns_api.Resource_record_set {
    managed_zone = "value"  # Identifies the managed zone addressed by this request. Can be the managed zone name or ID.
    project = "value"  # Identifies the project addressed by this request.
    location = "value"  # Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
}

# Access resource_record_set outputs
resource_record_set_id = resource_record_set.id
resource_record_set_rrdatas = resource_record_set.rrdatas
resource_record_set_type = resource_record_set.type
resource_record_set_routing_policy = resource_record_set.routing_policy
resource_record_set_kind = resource_record_set.kind
resource_record_set_signature_rrdatas = resource_record_set.signature_rrdatas
resource_record_set_name = resource_record_set.name
resource_record_set_ttl = resource_record_set.ttl
```

---


### Response_policy_rule

Creates a new Response Policy Rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `behavior` | String |  | Answer this query with a behavior rather than DNS data. |
| `dns_name` | String |  | The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule. |
| `rule_name` | String |  | An identifier for this rule. Must be unique with the ResponsePolicy. |
| `local_data` | String |  | Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name; in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed. |
| `kind` | String |  |  |
| `response_policy` | String | ✅ | User assigned name of the Response Policy containing the Response Policy Rule. |
| `location` | String | ✅ | Specifies the location of the resource. This information will be used for routing and will be part of the resource name. |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `behavior` | String | Answer this query with a behavior rather than DNS data. |
| `dns_name` | String | The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule. |
| `rule_name` | String | An identifier for this rule. Must be unique with the ResponsePolicy. |
| `local_data` | String | Answer this query directly with DNS data. These ResourceRecordSets override any other DNS behavior for the matched name; in particular they override private zones, the public internet, and GCP internal DNS. No SOA nor NS types are allowed. |
| `kind` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create response_policy_rule
response_policy_rule = provider.dns_api.Response_policy_rule {
    response_policy = "value"  # User assigned name of the Response Policy containing the Response Policy Rule.
    location = "value"  # Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
    project = "value"  # Identifies the project addressed by this request.
}

# Access response_policy_rule outputs
response_policy_rule_id = response_policy_rule.id
response_policy_rule_behavior = response_policy_rule.behavior
response_policy_rule_dns_name = response_policy_rule.dns_name
response_policy_rule_rule_name = response_policy_rule.rule_name
response_policy_rule_local_data = response_policy_rule.local_data
response_policy_rule_kind = response_policy_rule.kind
```

---


### Response_policie

Creates a new Response Policy

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  |  |
| `response_policy_name` | String |  | User assigned name for this Response Policy. |
| `networks` | Vec<String> |  | List of network names specifying networks to which this policy is applied. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only). |
| `description` | String |  | User-provided description for this Response Policy. |
| `gke_clusters` | Vec<String> |  | The list of Google Kubernetes Engine clusters to which this response policy is applied. |
| `location` | String | ✅ | Specifies the location of the resource, only applicable in the v APIs. This information will be used for routing and will be part of the resource name. |
| `project` | String | ✅ | Identifies the project addressed by this request. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String |  |
| `response_policy_name` | String | User assigned name for this Response Policy. |
| `networks` | Vec<String> | List of network names specifying networks to which this policy is applied. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `description` | String | User-provided description for this Response Policy. |
| `gke_clusters` | Vec<String> | The list of Google Kubernetes Engine clusters to which this response policy is applied. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create response_policie
response_policie = provider.dns_api.Response_policie {
    location = "value"  # Specifies the location of the resource, only applicable in the v APIs. This information will be used for routing and will be part of the resource name.
    project = "value"  # Identifies the project addressed by this request.
}

# Access response_policie outputs
response_policie_id = response_policie.id
response_policie_kind = response_policie.kind
response_policie_response_policy_name = response_policie.response_policy_name
response_policie_networks = response_policie.networks
response_policie_id = response_policie.id
response_policie_description = response_policie.description
response_policie_gke_clusters = response_policie.gke_clusters
```

---


### Managed_zone

Creates a new ManagedZone.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dns_name` | String |  | The DNS name of this managed zone, for instance "example.com.". |
| `dnssec_config` | String |  | DNSSEC configuration. |
| `private_visibility_config` | String |  | For privately visible zones, the set of Virtual Private Cloud resources that the zone is visible from. |
| `name_server_set` | String |  | Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users leave this field unset. If you need to use this field, contact your account team. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only) |
| `name_servers` | Vec<String> |  | Delegate your managed_zone to these virtual name servers; defined by the server (output only) |
| `visibility` | String |  | The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources. |
| `creation_time` | String |  | The time that this resource was created on the server. This is in RFC3339 text format. Output only. |
| `labels` | HashMap<String, String> |  | User labels. |
| `description` | String |  | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function. |
| `cloud_logging_config` | String |  |  |
| `kind` | String |  |  |
| `reverse_lookup_config` | String |  | The presence of this field indicates that this is a managed reverse lookup zone and Cloud DNS resolves reverse lookup queries using automatically configured records for VPC resources. This only applies to networks listed under private_visibility_config. |
| `peering_config` | String |  | The presence of this field indicates that DNS Peering is enabled for this zone. The value of this field contains the network to peer with. |
| `name` | String |  | User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes. |
| `service_directory_config` | String |  | This field links to the associated service directory namespace. Do not set this field for public zones or forwarding zones. |
| `forwarding_config` | String |  | The presence for this field indicates that outbound forwarding is enabled for this zone. The value of this field contains the set of destinations to forward to. |
| `project` | String | ✅ | Identifies the project addressed by this request. |
| `location` | String | ✅ | Specifies the location of the resource. This information will be used for routing and will be part of the resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dns_name` | String | The DNS name of this managed zone, for instance "example.com.". |
| `dnssec_config` | String | DNSSEC configuration. |
| `private_visibility_config` | String | For privately visible zones, the set of Virtual Private Cloud resources that the zone is visible from. |
| `name_server_set` | String | Optionally specifies the NameServerSet for this ManagedZone. A NameServerSet is a set of DNS name servers that all host the same ManagedZones. Most users leave this field unset. If you need to use this field, contact your account team. |
| `id` | String | Unique identifier for the resource; defined by the server (output only) |
| `name_servers` | Vec<String> | Delegate your managed_zone to these virtual name servers; defined by the server (output only) |
| `visibility` | String | The zone's visibility: public zones are exposed to the Internet, while private zones are visible only to Virtual Private Cloud resources. |
| `creation_time` | String | The time that this resource was created on the server. This is in RFC3339 text format. Output only. |
| `labels` | HashMap<String, String> | User labels. |
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the managed zone's function. |
| `cloud_logging_config` | String |  |
| `kind` | String |  |
| `reverse_lookup_config` | String | The presence of this field indicates that this is a managed reverse lookup zone and Cloud DNS resolves reverse lookup queries using automatically configured records for VPC resources. This only applies to networks listed under private_visibility_config. |
| `peering_config` | String | The presence of this field indicates that DNS Peering is enabled for this zone. The value of this field contains the network to peer with. |
| `name` | String | User assigned name for this resource. Must be unique within the project. The name must be 1-63 characters long, must begin with a letter, end with a letter or digit, and only contain lowercase letters, digits or dashes. |
| `service_directory_config` | String | This field links to the associated service directory namespace. Do not set this field for public zones or forwarding zones. |
| `forwarding_config` | String | The presence for this field indicates that outbound forwarding is enabled for this zone. The value of this field contains the set of destinations to forward to. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create managed_zone
managed_zone = provider.dns_api.Managed_zone {
    project = "value"  # Identifies the project addressed by this request.
    location = "value"  # Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
}

# Access managed_zone outputs
managed_zone_id = managed_zone.id
managed_zone_dns_name = managed_zone.dns_name
managed_zone_dnssec_config = managed_zone.dnssec_config
managed_zone_private_visibility_config = managed_zone.private_visibility_config
managed_zone_name_server_set = managed_zone.name_server_set
managed_zone_id = managed_zone.id
managed_zone_name_servers = managed_zone.name_servers
managed_zone_visibility = managed_zone.visibility
managed_zone_creation_time = managed_zone.creation_time
managed_zone_labels = managed_zone.labels
managed_zone_description = managed_zone.description
managed_zone_cloud_logging_config = managed_zone.cloud_logging_config
managed_zone_kind = managed_zone.kind
managed_zone_reverse_lookup_config = managed_zone.reverse_lookup_config
managed_zone_peering_config = managed_zone.peering_config
managed_zone_name = managed_zone.name
managed_zone_service_directory_config = managed_zone.service_directory_config
managed_zone_forwarding_config = managed_zone.forwarding_config
```

---


### Project

Fetches the representation of an existing Project.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | User assigned unique identifier for the resource (output only). |
| `kind` | String |  |
| `number` | String | Unique numeric identifier for the resource; defined by the server (output only). |
| `quota` | String | Quotas assigned to this project (output only). |


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
project_id = project.id
project_kind = project.kind
project_number = project.number
project_quota = project.quota
```

---


### Policie

Creates a new Policy.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the policy's function. |
| `kind` | String |  |  |
| `enable_logging` | bool |  | Controls whether logging is enabled for the networks bound to this policy. Defaults to no logging if not set. |
| `name` | String |  | User-assigned name for this policy. |
| `enable_inbound_forwarding` | bool |  | Allows networks bound to this policy to receive DNS queries sent by VMs or applications over VPN connections. When enabled, a virtual IP address is allocated from each of the subnetworks that are bound to this policy. |
| `id` | String |  | Unique identifier for the resource; defined by the server (output only). |
| `networks` | Vec<String> |  | List of network names specifying networks to which this policy is applied. |
| `alternative_name_server_config` | String |  | Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified. |
| `project` | String | ✅ | Identifies the project addressed by this request. |
| `location` | String | ✅ | Specifies the location of the resource. This information will be used for routing and will be part of the resource name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | A mutable string of at most 1024 characters associated with this resource for the user's convenience. Has no effect on the policy's function. |
| `kind` | String |  |
| `enable_logging` | bool | Controls whether logging is enabled for the networks bound to this policy. Defaults to no logging if not set. |
| `name` | String | User-assigned name for this policy. |
| `enable_inbound_forwarding` | bool | Allows networks bound to this policy to receive DNS queries sent by VMs or applications over VPN connections. When enabled, a virtual IP address is allocated from each of the subnetworks that are bound to this policy. |
| `id` | String | Unique identifier for the resource; defined by the server (output only). |
| `networks` | Vec<String> | List of network names specifying networks to which this policy is applied. |
| `alternative_name_server_config` | String | Sets an alternative name server for the associated networks. When specified, all DNS queries are forwarded to a name server that you choose. Names such as .internal are not available when an alternative name server is specified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create policie
policie = provider.dns_api.Policie {
    project = "value"  # Identifies the project addressed by this request.
    location = "value"  # Specifies the location of the resource. This information will be used for routing and will be part of the resource name.
}

# Access policie outputs
policie_id = policie.id
policie_description = policie.description
policie_kind = policie.kind
policie_enable_logging = policie.enable_logging
policie_name = policie.name
policie_enable_inbound_forwarding = policie.enable_inbound_forwarding
policie_id = policie.id
policie_networks = policie.networks
policie_alternative_name_server_config = policie.alternative_name_server_config
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
project_0 = provider.dns_api.Project {
}
project_1 = provider.dns_api.Project {
}
project_2 = provider.dns_api.Project {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    project = provider.dns_api.Project {
    }
```

---

## Related Documentation

- [GCP Dns_api Documentation](https://cloud.google.com/dns_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
