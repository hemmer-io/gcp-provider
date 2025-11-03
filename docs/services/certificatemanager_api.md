# Certificatemanager_api Service



**Resources**: 8

---

## Overview

The certificatemanager_api service provides access to 8 resource types:

- [Certificate](#certificate) [CRUD]
- [Certificate_issuance_config](#certificate_issuance_config) [CRUD]
- [Trust_config](#trust_config) [CRUD]
- [Operation](#operation) [CRD]
- [Certificate_map](#certificate_map) [CRUD]
- [Certificate_map_entrie](#certificate_map_entrie) [CRUD]
- [Dns_authorization](#dns_authorization) [CRUD]
- [Location](#location) [R]

---

## Resources


### Certificate

Creates a new Certificate in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `managed` | String |  | If set, contains configuration and state of a managed certificate. |
| `create_time` | String |  | Output only. The creation timestamp of a Certificate. |
| `description` | String |  | Optional. One or more paragraphs of text description of a certificate. |
| `name` | String |  | Identifier. A user-defined name of the certificate. Certificate names must be unique globally and match pattern `projects/*/locations/*/certificates/*`. |
| `scope` | String |  | Optional. Immutable. The scope of the certificate. |
| `expire_time` | String |  | Output only. The expiry timestamp of a Certificate. |
| `update_time` | String |  | Output only. The last update timestamp of a Certificate. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with a Certificate. |
| `san_dnsnames` | Vec<String> |  | Output only. The list of Subject Alternative Names of dnsName type defined in the certificate (see RFC 5280 4.2.1.6). Managed certificates that haven't been provisioned yet have this field populated with a value of the managed.domains field. |
| `used_by` | Vec<String> |  | Output only. The list of resources that use this Certificate. |
| `self_managed` | String |  | If set, defines data of a self-managed certificate. |
| `pem_certificate` | String |  | Output only. The PEM-encoded certificate chain. |
| `parent` | String | ✅ | Required. The parent resource of the certificate. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed` | String | If set, contains configuration and state of a managed certificate. |
| `create_time` | String | Output only. The creation timestamp of a Certificate. |
| `description` | String | Optional. One or more paragraphs of text description of a certificate. |
| `name` | String | Identifier. A user-defined name of the certificate. Certificate names must be unique globally and match pattern `projects/*/locations/*/certificates/*`. |
| `scope` | String | Optional. Immutable. The scope of the certificate. |
| `expire_time` | String | Output only. The expiry timestamp of a Certificate. |
| `update_time` | String | Output only. The last update timestamp of a Certificate. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with a Certificate. |
| `san_dnsnames` | Vec<String> | Output only. The list of Subject Alternative Names of dnsName type defined in the certificate (see RFC 5280 4.2.1.6). Managed certificates that haven't been provisioned yet have this field populated with a value of the managed.domains field. |
| `used_by` | Vec<String> | Output only. The list of resources that use this Certificate. |
| `self_managed` | String | If set, defines data of a self-managed certificate. |
| `pem_certificate` | String | Output only. The PEM-encoded certificate chain. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate
certificate = provider.certificatemanager_api.Certificate {
    parent = "value"  # Required. The parent resource of the certificate. Must be in the format `projects/*/locations/*`.
}

# Access certificate outputs
certificate_id = certificate.id
certificate_managed = certificate.managed
certificate_create_time = certificate.create_time
certificate_description = certificate.description
certificate_name = certificate.name
certificate_scope = certificate.scope
certificate_expire_time = certificate.expire_time
certificate_update_time = certificate.update_time
certificate_labels = certificate.labels
certificate_san_dnsnames = certificate.san_dnsnames
certificate_used_by = certificate.used_by
certificate_self_managed = certificate.self_managed
certificate_pem_certificate = certificate.pem_certificate
```

---


### Certificate_issuance_config

Creates a new CertificateIssuanceConfig in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with a CertificateIssuanceConfig. |
| `rotation_window_percentage` | i64 |  | Required. Specifies the percentage of elapsed time of the certificate lifetime to wait before renewing the certificate. Must be a number between 1-99, inclusive. |
| `lifetime` | String |  | Required. Workload certificate lifetime requested. |
| `name` | String |  | Identifier. A user-defined name of the certificate issuance config. CertificateIssuanceConfig names must be unique globally and match pattern `projects/*/locations/*/certificateIssuanceConfigs/*`. |
| `description` | String |  | Optional. One or more paragraphs of text description of a CertificateIssuanceConfig. |
| `certificate_authority_config` | String |  | Required. The CA that issues the workload certificate. It includes the CA address, type, authentication to CA service, etc. |
| `create_time` | String |  | Output only. The creation timestamp of a CertificateIssuanceConfig. |
| `key_algorithm` | String |  | Required. The key algorithm to use when generating the private key. |
| `update_time` | String |  | Output only. The last update timestamp of a CertificateIssuanceConfig. |
| `parent` | String | ✅ | Required. The parent resource of the certificate issuance config. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Set of labels associated with a CertificateIssuanceConfig. |
| `rotation_window_percentage` | i64 | Required. Specifies the percentage of elapsed time of the certificate lifetime to wait before renewing the certificate. Must be a number between 1-99, inclusive. |
| `lifetime` | String | Required. Workload certificate lifetime requested. |
| `name` | String | Identifier. A user-defined name of the certificate issuance config. CertificateIssuanceConfig names must be unique globally and match pattern `projects/*/locations/*/certificateIssuanceConfigs/*`. |
| `description` | String | Optional. One or more paragraphs of text description of a CertificateIssuanceConfig. |
| `certificate_authority_config` | String | Required. The CA that issues the workload certificate. It includes the CA address, type, authentication to CA service, etc. |
| `create_time` | String | Output only. The creation timestamp of a CertificateIssuanceConfig. |
| `key_algorithm` | String | Required. The key algorithm to use when generating the private key. |
| `update_time` | String | Output only. The last update timestamp of a CertificateIssuanceConfig. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate_issuance_config
certificate_issuance_config = provider.certificatemanager_api.Certificate_issuance_config {
    parent = "value"  # Required. The parent resource of the certificate issuance config. Must be in the format `projects/*/locations/*`.
}

# Access certificate_issuance_config outputs
certificate_issuance_config_id = certificate_issuance_config.id
certificate_issuance_config_labels = certificate_issuance_config.labels
certificate_issuance_config_rotation_window_percentage = certificate_issuance_config.rotation_window_percentage
certificate_issuance_config_lifetime = certificate_issuance_config.lifetime
certificate_issuance_config_name = certificate_issuance_config.name
certificate_issuance_config_description = certificate_issuance_config.description
certificate_issuance_config_certificate_authority_config = certificate_issuance_config.certificate_authority_config
certificate_issuance_config_create_time = certificate_issuance_config.create_time
certificate_issuance_config_key_algorithm = certificate_issuance_config.key_algorithm
certificate_issuance_config_update_time = certificate_issuance_config.update_time
```

---


### Trust_config

Creates a new TrustConfig in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `allowlisted_certificates` | Vec<String> |  | Optional. A certificate matching an allowlisted certificate is always considered valid as long as the certificate is parseable, proof of private key possession is established, and constraints on the certificate's SAN field are met. |
| `create_time` | String |  | Output only. The creation timestamp of a TrustConfig. |
| `trust_stores` | Vec<String> |  | Optional. Set of trust stores to perform validation against. This field is supported when TrustConfig is configured with Load Balancers, currently not supported for SPIFFE certificate validation. Only one TrustStore specified is currently allowed. |
| `update_time` | String |  | Output only. The last update timestamp of a TrustConfig. |
| `name` | String |  | Identifier. A user-defined name of the trust config. TrustConfig names must be unique globally and match pattern `projects/*/locations/*/trustConfigs/*`. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with a TrustConfig. |
| `description` | String |  | Optional. One or more paragraphs of text description of a TrustConfig. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |
| `parent` | String | ✅ | Required. The parent resource of the TrustConfig. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `allowlisted_certificates` | Vec<String> | Optional. A certificate matching an allowlisted certificate is always considered valid as long as the certificate is parseable, proof of private key possession is established, and constraints on the certificate's SAN field are met. |
| `create_time` | String | Output only. The creation timestamp of a TrustConfig. |
| `trust_stores` | Vec<String> | Optional. Set of trust stores to perform validation against. This field is supported when TrustConfig is configured with Load Balancers, currently not supported for SPIFFE certificate validation. Only one TrustStore specified is currently allowed. |
| `update_time` | String | Output only. The last update timestamp of a TrustConfig. |
| `name` | String | Identifier. A user-defined name of the trust config. TrustConfig names must be unique globally and match pattern `projects/*/locations/*/trustConfigs/*`. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with a TrustConfig. |
| `description` | String | Optional. One or more paragraphs of text description of a TrustConfig. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trust_config
trust_config = provider.certificatemanager_api.Trust_config {
    parent = "value"  # Required. The parent resource of the TrustConfig. Must be in the format `projects/*/locations/*`.
}

# Access trust_config outputs
trust_config_id = trust_config.id
trust_config_allowlisted_certificates = trust_config.allowlisted_certificates
trust_config_create_time = trust_config.create_time
trust_config_trust_stores = trust_config.trust_stores
trust_config_update_time = trust_config.update_time
trust_config_name = trust_config.name
trust_config_labels = trust_config.labels
trust_config_description = trust_config.description
trust_config_etag = trust_config.etag
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.certificatemanager_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
```

---


### Certificate_map

Creates a new CertificateMap in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. One or more paragraphs of text description of a certificate map. |
| `create_time` | String |  | Output only. The creation timestamp of a Certificate Map. |
| `gclb_targets` | Vec<String> |  | Output only. A list of GCLB targets that use this Certificate Map. A Target Proxy is only present on this list if it's attached to a Forwarding Rule. |
| `name` | String |  | Identifier. A user-defined name of the Certificate Map. Certificate Map names must be unique globally and match pattern `projects/*/locations/*/certificateMaps/*`. |
| `update_time` | String |  | Output only. The update timestamp of a Certificate Map. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with a Certificate Map. |
| `parent` | String | ✅ | Required. The parent resource of the certificate map. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. One or more paragraphs of text description of a certificate map. |
| `create_time` | String | Output only. The creation timestamp of a Certificate Map. |
| `gclb_targets` | Vec<String> | Output only. A list of GCLB targets that use this Certificate Map. A Target Proxy is only present on this list if it's attached to a Forwarding Rule. |
| `name` | String | Identifier. A user-defined name of the Certificate Map. Certificate Map names must be unique globally and match pattern `projects/*/locations/*/certificateMaps/*`. |
| `update_time` | String | Output only. The update timestamp of a Certificate Map. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with a Certificate Map. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate_map
certificate_map = provider.certificatemanager_api.Certificate_map {
    parent = "value"  # Required. The parent resource of the certificate map. Must be in the format `projects/*/locations/*`.
}

# Access certificate_map outputs
certificate_map_id = certificate_map.id
certificate_map_description = certificate_map.description
certificate_map_create_time = certificate_map.create_time
certificate_map_gclb_targets = certificate_map.gclb_targets
certificate_map_name = certificate_map.name
certificate_map_update_time = certificate_map.update_time
certificate_map_labels = certificate_map.labels
```

---


### Certificate_map_entrie

Creates a new CertificateMapEntry in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. A serving state of this Certificate Map Entry. |
| `hostname` | String |  | A Hostname (FQDN, e.g. `example.com`) or a wildcard hostname expression (`*.example.com`) for a set of hostnames with common suffix. Used as Server Name Indication (SNI) for selecting a proper certificate. |
| `matcher` | String |  | A predefined matcher for particular cases, other than SNI selection. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with a Certificate Map Entry. |
| `create_time` | String |  | Output only. The creation timestamp of a Certificate Map Entry. |
| `description` | String |  | Optional. One or more paragraphs of text description of a certificate map entry. |
| `name` | String |  | Identifier. A user-defined name of the Certificate Map Entry. Certificate Map Entry names must be unique globally and match pattern `projects/*/locations/*/certificateMaps/*/certificateMapEntries/*`. |
| `update_time` | String |  | Output only. The update timestamp of a Certificate Map Entry. |
| `certificates` | Vec<String> |  | Optional. A set of Certificates defines for the given `hostname`. There can be defined up to four certificates in each Certificate Map Entry. Each certificate must match pattern `projects/*/locations/*/certificates/*`. |
| `parent` | String | ✅ | Required. The parent resource of the certificate map entry. Must be in the format `projects/*/locations/*/certificateMaps/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. A serving state of this Certificate Map Entry. |
| `hostname` | String | A Hostname (FQDN, e.g. `example.com`) or a wildcard hostname expression (`*.example.com`) for a set of hostnames with common suffix. Used as Server Name Indication (SNI) for selecting a proper certificate. |
| `matcher` | String | A predefined matcher for particular cases, other than SNI selection. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with a Certificate Map Entry. |
| `create_time` | String | Output only. The creation timestamp of a Certificate Map Entry. |
| `description` | String | Optional. One or more paragraphs of text description of a certificate map entry. |
| `name` | String | Identifier. A user-defined name of the Certificate Map Entry. Certificate Map Entry names must be unique globally and match pattern `projects/*/locations/*/certificateMaps/*/certificateMapEntries/*`. |
| `update_time` | String | Output only. The update timestamp of a Certificate Map Entry. |
| `certificates` | Vec<String> | Optional. A set of Certificates defines for the given `hostname`. There can be defined up to four certificates in each Certificate Map Entry. Each certificate must match pattern `projects/*/locations/*/certificates/*`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate_map_entrie
certificate_map_entrie = provider.certificatemanager_api.Certificate_map_entrie {
    parent = "value"  # Required. The parent resource of the certificate map entry. Must be in the format `projects/*/locations/*/certificateMaps/*`.
}

# Access certificate_map_entrie outputs
certificate_map_entrie_id = certificate_map_entrie.id
certificate_map_entrie_state = certificate_map_entrie.state
certificate_map_entrie_hostname = certificate_map_entrie.hostname
certificate_map_entrie_matcher = certificate_map_entrie.matcher
certificate_map_entrie_labels = certificate_map_entrie.labels
certificate_map_entrie_create_time = certificate_map_entrie.create_time
certificate_map_entrie_description = certificate_map_entrie.description
certificate_map_entrie_name = certificate_map_entrie.name
certificate_map_entrie_update_time = certificate_map_entrie.update_time
certificate_map_entrie_certificates = certificate_map_entrie.certificates
```

---


### Dns_authorization

Creates a new DnsAuthorization in a given project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Optional. Immutable. Type of DnsAuthorization. If unset during resource creation the following default will be used: - in location `global`: FIXED_RECORD, - in other locations: PER_PROJECT_RECORD. |
| `create_time` | String |  | Output only. The creation timestamp of a DnsAuthorization. |
| `name` | String |  | Identifier. A user-defined name of the dns authorization. DnsAuthorization names must be unique globally and match pattern `projects/*/locations/*/dnsAuthorizations/*`. |
| `update_time` | String |  | Output only. The last update timestamp of a DnsAuthorization. |
| `dns_resource_record` | String |  | Output only. DNS Resource Record that needs to be added to DNS configuration. |
| `labels` | HashMap<String, String> |  | Optional. Set of labels associated with a DnsAuthorization. |
| `description` | String |  | Optional. One or more paragraphs of text description of a DnsAuthorization. |
| `domain` | String |  | Required. Immutable. A domain that is being authorized. A DnsAuthorization resource covers a single domain and its wildcard, e.g. authorization for `example.com` can be used to issue certificates for `example.com` and `*.example.com`. |
| `parent` | String | ✅ | Required. The parent resource of the dns authorization. Must be in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Optional. Immutable. Type of DnsAuthorization. If unset during resource creation the following default will be used: - in location `global`: FIXED_RECORD, - in other locations: PER_PROJECT_RECORD. |
| `create_time` | String | Output only. The creation timestamp of a DnsAuthorization. |
| `name` | String | Identifier. A user-defined name of the dns authorization. DnsAuthorization names must be unique globally and match pattern `projects/*/locations/*/dnsAuthorizations/*`. |
| `update_time` | String | Output only. The last update timestamp of a DnsAuthorization. |
| `dns_resource_record` | String | Output only. DNS Resource Record that needs to be added to DNS configuration. |
| `labels` | HashMap<String, String> | Optional. Set of labels associated with a DnsAuthorization. |
| `description` | String | Optional. One or more paragraphs of text description of a DnsAuthorization. |
| `domain` | String | Required. Immutable. A domain that is being authorized. A DnsAuthorization resource covers a single domain and its wildcard, e.g. authorization for `example.com` can be used to issue certificates for `example.com` and `*.example.com`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dns_authorization
dns_authorization = provider.certificatemanager_api.Dns_authorization {
    parent = "value"  # Required. The parent resource of the dns authorization. Must be in the format `projects/*/locations/*`.
}

# Access dns_authorization outputs
dns_authorization_id = dns_authorization.id
dns_authorization_type = dns_authorization.type
dns_authorization_create_time = dns_authorization.create_time
dns_authorization_name = dns_authorization.name
dns_authorization_update_time = dns_authorization.update_time
dns_authorization_dns_resource_record = dns_authorization.dns_resource_record
dns_authorization_labels = dns_authorization.labels
dns_authorization_description = dns_authorization.description
dns_authorization_domain = dns_authorization.domain
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
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
location_location_id = location.location_id
location_display_name = location.display_name
location_labels = location.labels
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

# Create multiple certificate resources
certificate_0 = provider.certificatemanager_api.Certificate {
    parent = "value-0"
}
certificate_1 = provider.certificatemanager_api.Certificate {
    parent = "value-1"
}
certificate_2 = provider.certificatemanager_api.Certificate {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    certificate = provider.certificatemanager_api.Certificate {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Certificatemanager_api Documentation](https://cloud.google.com/certificatemanager_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
