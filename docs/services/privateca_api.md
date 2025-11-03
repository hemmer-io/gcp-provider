# Privateca_api Service



**Resources**: 12

---

## Overview

The privateca_api service provides access to 12 resource types:

- [Location](#location) [R]
- [Ca_pool](#ca_pool) [CRUD]
- [Certificate_authoritie](#certificate_authoritie) [CRUD]
- [Certificate_template](#certificate_template) [CRUD]
- [Operation](#operation) [CRD]
- [Certificate_revocation_list](#certificate_revocation_list) [CRU]
- [Certificate](#certificate) [CRU]
- [Certificate_authoritie](#certificate_authoritie) [CR]
- [Certificate_revocation_list](#certificate_revocation_list) [CR]
- [Location](#location) [R]
- [Reusable_config](#reusable_config) [CR]
- [Operation](#operation) [CRD]

---

## Resources


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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


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
location_display_name = location.display_name
location_metadata = location.metadata
location_name = location.name
```

---


### Ca_pool

Create a CaPool.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `publishing_options` | String |  | Optional. The PublishingOptions to follow when issuing Certificates from any CertificateAuthority in this CaPool. |
| `encryption_spec` | String |  | Optional. When EncryptionSpec is provided, the Subject, SubjectAltNames, and the PEM-encoded certificate fields will be encrypted at rest. |
| `name` | String |  | Identifier. The resource name for this CaPool in the format `projects/*/locations/*/caPools/*`. |
| `issuance_policy` | String |  | Optional. The IssuancePolicy to control how Certificates will be issued from this CaPool. |
| `labels` | HashMap<String, String> |  | Optional. Labels with user-defined metadata. |
| `tier` | String |  | Required. Immutable. The Tier of this CaPool. |
| `parent` | String | ✅ | Required. The resource name of the location associated with the CaPool, in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `publishing_options` | String | Optional. The PublishingOptions to follow when issuing Certificates from any CertificateAuthority in this CaPool. |
| `encryption_spec` | String | Optional. When EncryptionSpec is provided, the Subject, SubjectAltNames, and the PEM-encoded certificate fields will be encrypted at rest. |
| `name` | String | Identifier. The resource name for this CaPool in the format `projects/*/locations/*/caPools/*`. |
| `issuance_policy` | String | Optional. The IssuancePolicy to control how Certificates will be issued from this CaPool. |
| `labels` | HashMap<String, String> | Optional. Labels with user-defined metadata. |
| `tier` | String | Required. Immutable. The Tier of this CaPool. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ca_pool
ca_pool = provider.privateca_api.Ca_pool {
    parent = "value"  # Required. The resource name of the location associated with the CaPool, in the format `projects/*/locations/*`.
}

# Access ca_pool outputs
ca_pool_id = ca_pool.id
ca_pool_publishing_options = ca_pool.publishing_options
ca_pool_encryption_spec = ca_pool.encryption_spec
ca_pool_name = ca_pool.name
ca_pool_issuance_policy = ca_pool.issuance_policy
ca_pool_labels = ca_pool.labels
ca_pool_tier = ca_pool.tier
```

---


### Certificate_authoritie

Create a new CertificateAuthority in a given Project and Location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`. |
| `update_time` | String |  | Output only. The time at which this CertificateAuthority was last updated. |
| `config` | String |  | Required. Immutable. The config used to create a self-signed X.509 certificate or CSR. |
| `expire_time` | String |  | Output only. The time at which this CertificateAuthority will be permanently purged, if it is in the DELETED state. |
| `gcs_bucket` | String |  | Immutable. The name of a Cloud Storage bucket where this CertificateAuthority will publish content, such as the CA certificate and CRLs. This must be a bucket name, without any prefixes (such as `gs://`) or suffixes (such as `.googleapis.com`). For example, to use a bucket named `my-bucket`, you would simply specify `my-bucket`. If not specified, a managed bucket will be created. |
| `pem_ca_certificates` | Vec<String> |  | Output only. This CertificateAuthority's certificate chain, including the current CertificateAuthority's certificate. Ordered such that the root issuer is the final element (consistent with RFC 5246). For a self-signed CA, this will only list the current CertificateAuthority's certificate. |
| `tier` | String |  | Output only. The CaPool.Tier of the CaPool that includes this CertificateAuthority. |
| `type` | String |  | Required. Immutable. The Type of this CertificateAuthority. |
| `ca_certificate_descriptions` | Vec<String> |  | Output only. A structured description of this CertificateAuthority's CA certificate and its issuers. Ordered as self-to-root. |
| `access_urls` | String |  | Output only. URLs for accessing content published by this CA, such as the CA certificate and CRLs. |
| `delete_time` | String |  | Output only. The time at which this CertificateAuthority was soft deleted, if it is in the DELETED state. |
| `satisfies_pzs` | bool |  | Output only. Reserved for future use. |
| `lifetime` | String |  | Required. Immutable. The desired lifetime of the CA certificate. Used to create the "not_before_time" and "not_after_time" fields inside an X.509 certificate. |
| `subordinate_config` | String |  | Optional. If this is a subordinate CertificateAuthority, this field will be set with the subordinate configuration, which describes its issuers. This may be updated, but this CertificateAuthority must continue to validate. |
| `user_defined_access_urls` | String |  | Optional. User-defined URLs for CA certificate and CRLs. The service does not publish content to these URLs. It is up to the user to mirror content to these URLs. |
| `create_time` | String |  | Output only. The time at which this CertificateAuthority was created. |
| `labels` | HashMap<String, String> |  | Optional. Labels with user-defined metadata. |
| `state` | String |  | Output only. The State for this CertificateAuthority. |
| `key_spec` | String |  | Required. Immutable. Used when issuing certificates for this CertificateAuthority. If this CertificateAuthority is a self-signed CertificateAuthority, this key is also used to sign the self-signed CA certificate. Otherwise, it is used to sign a CSR. |
| `satisfies_pzi` | bool |  | Output only. Reserved for future use. |
| `parent` | String | ✅ | Required. The resource name of the CaPool associated with the CertificateAuthorities, in the format `projects/*/locations/*/caPools/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. The resource name for this CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`. |
| `update_time` | String | Output only. The time at which this CertificateAuthority was last updated. |
| `config` | String | Required. Immutable. The config used to create a self-signed X.509 certificate or CSR. |
| `expire_time` | String | Output only. The time at which this CertificateAuthority will be permanently purged, if it is in the DELETED state. |
| `gcs_bucket` | String | Immutable. The name of a Cloud Storage bucket where this CertificateAuthority will publish content, such as the CA certificate and CRLs. This must be a bucket name, without any prefixes (such as `gs://`) or suffixes (such as `.googleapis.com`). For example, to use a bucket named `my-bucket`, you would simply specify `my-bucket`. If not specified, a managed bucket will be created. |
| `pem_ca_certificates` | Vec<String> | Output only. This CertificateAuthority's certificate chain, including the current CertificateAuthority's certificate. Ordered such that the root issuer is the final element (consistent with RFC 5246). For a self-signed CA, this will only list the current CertificateAuthority's certificate. |
| `tier` | String | Output only. The CaPool.Tier of the CaPool that includes this CertificateAuthority. |
| `type` | String | Required. Immutable. The Type of this CertificateAuthority. |
| `ca_certificate_descriptions` | Vec<String> | Output only. A structured description of this CertificateAuthority's CA certificate and its issuers. Ordered as self-to-root. |
| `access_urls` | String | Output only. URLs for accessing content published by this CA, such as the CA certificate and CRLs. |
| `delete_time` | String | Output only. The time at which this CertificateAuthority was soft deleted, if it is in the DELETED state. |
| `satisfies_pzs` | bool | Output only. Reserved for future use. |
| `lifetime` | String | Required. Immutable. The desired lifetime of the CA certificate. Used to create the "not_before_time" and "not_after_time" fields inside an X.509 certificate. |
| `subordinate_config` | String | Optional. If this is a subordinate CertificateAuthority, this field will be set with the subordinate configuration, which describes its issuers. This may be updated, but this CertificateAuthority must continue to validate. |
| `user_defined_access_urls` | String | Optional. User-defined URLs for CA certificate and CRLs. The service does not publish content to these URLs. It is up to the user to mirror content to these URLs. |
| `create_time` | String | Output only. The time at which this CertificateAuthority was created. |
| `labels` | HashMap<String, String> | Optional. Labels with user-defined metadata. |
| `state` | String | Output only. The State for this CertificateAuthority. |
| `key_spec` | String | Required. Immutable. Used when issuing certificates for this CertificateAuthority. If this CertificateAuthority is a self-signed CertificateAuthority, this key is also used to sign the self-signed CA certificate. Otherwise, it is used to sign a CSR. |
| `satisfies_pzi` | bool | Output only. Reserved for future use. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate_authoritie
certificate_authoritie = provider.privateca_api.Certificate_authoritie {
    parent = "value"  # Required. The resource name of the CaPool associated with the CertificateAuthorities, in the format `projects/*/locations/*/caPools/*`.
}

# Access certificate_authoritie outputs
certificate_authoritie_id = certificate_authoritie.id
certificate_authoritie_name = certificate_authoritie.name
certificate_authoritie_update_time = certificate_authoritie.update_time
certificate_authoritie_config = certificate_authoritie.config
certificate_authoritie_expire_time = certificate_authoritie.expire_time
certificate_authoritie_gcs_bucket = certificate_authoritie.gcs_bucket
certificate_authoritie_pem_ca_certificates = certificate_authoritie.pem_ca_certificates
certificate_authoritie_tier = certificate_authoritie.tier
certificate_authoritie_type = certificate_authoritie.type
certificate_authoritie_ca_certificate_descriptions = certificate_authoritie.ca_certificate_descriptions
certificate_authoritie_access_urls = certificate_authoritie.access_urls
certificate_authoritie_delete_time = certificate_authoritie.delete_time
certificate_authoritie_satisfies_pzs = certificate_authoritie.satisfies_pzs
certificate_authoritie_lifetime = certificate_authoritie.lifetime
certificate_authoritie_subordinate_config = certificate_authoritie.subordinate_config
certificate_authoritie_user_defined_access_urls = certificate_authoritie.user_defined_access_urls
certificate_authoritie_create_time = certificate_authoritie.create_time
certificate_authoritie_labels = certificate_authoritie.labels
certificate_authoritie_state = certificate_authoritie.state
certificate_authoritie_key_spec = certificate_authoritie.key_spec
certificate_authoritie_satisfies_pzi = certificate_authoritie.satisfies_pzi
```

---


### Certificate_template

Create a new CertificateTemplate in a given Project and Location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional. A human-readable description of scenarios this template is intended for. |
| `name` | String |  | Identifier. The resource name for this CertificateTemplate in the format `projects/*/locations/*/certificateTemplates/*`. |
| `identity_constraints` | String |  | Optional. Describes constraints on identities that may be appear in Certificates issued using this template. If this is omitted, then this template will not add restrictions on a certificate's identity. |
| `predefined_values` | String |  | Optional. A set of X.509 values that will be applied to all issued certificates that use this template. If the certificate request includes conflicting values for the same properties, they will be overwritten by the values defined here. If the issuing CaPool's IssuancePolicy defines conflicting baseline_values for the same properties, the certificate issuance request will fail. |
| `labels` | HashMap<String, String> |  | Optional. Labels with user-defined metadata. |
| `update_time` | String |  | Output only. The time at which this CertificateTemplate was updated. |
| `passthrough_extensions` | String |  | Optional. Describes the set of X.509 extensions that may appear in a Certificate issued using this CertificateTemplate. If a certificate request sets extensions that don't appear in the passthrough_extensions, those extensions will be dropped. If the issuing CaPool's IssuancePolicy defines baseline_values that don't appear here, the certificate issuance request will fail. If this is omitted, then this template will not add restrictions on a certificate's X.509 extensions. These constraints do not apply to X.509 extensions set in this CertificateTemplate's predefined_values. |
| `maximum_lifetime` | String |  | Optional. The maximum lifetime allowed for issued Certificates that use this template. If the issuing CaPool resource's IssuancePolicy specifies a maximum_lifetime the minimum of the two durations will be the maximum lifetime for issued Certificates. Note that if the issuing CertificateAuthority expires before a Certificate's requested maximum_lifetime, the effective lifetime will be explicitly truncated to match it. |
| `create_time` | String |  | Output only. The time at which this CertificateTemplate was created. |
| `parent` | String | ✅ | Required. The resource name of the location associated with the CertificateTemplate, in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. A human-readable description of scenarios this template is intended for. |
| `name` | String | Identifier. The resource name for this CertificateTemplate in the format `projects/*/locations/*/certificateTemplates/*`. |
| `identity_constraints` | String | Optional. Describes constraints on identities that may be appear in Certificates issued using this template. If this is omitted, then this template will not add restrictions on a certificate's identity. |
| `predefined_values` | String | Optional. A set of X.509 values that will be applied to all issued certificates that use this template. If the certificate request includes conflicting values for the same properties, they will be overwritten by the values defined here. If the issuing CaPool's IssuancePolicy defines conflicting baseline_values for the same properties, the certificate issuance request will fail. |
| `labels` | HashMap<String, String> | Optional. Labels with user-defined metadata. |
| `update_time` | String | Output only. The time at which this CertificateTemplate was updated. |
| `passthrough_extensions` | String | Optional. Describes the set of X.509 extensions that may appear in a Certificate issued using this CertificateTemplate. If a certificate request sets extensions that don't appear in the passthrough_extensions, those extensions will be dropped. If the issuing CaPool's IssuancePolicy defines baseline_values that don't appear here, the certificate issuance request will fail. If this is omitted, then this template will not add restrictions on a certificate's X.509 extensions. These constraints do not apply to X.509 extensions set in this CertificateTemplate's predefined_values. |
| `maximum_lifetime` | String | Optional. The maximum lifetime allowed for issued Certificates that use this template. If the issuing CaPool resource's IssuancePolicy specifies a maximum_lifetime the minimum of the two durations will be the maximum lifetime for issued Certificates. Note that if the issuing CertificateAuthority expires before a Certificate's requested maximum_lifetime, the effective lifetime will be explicitly truncated to match it. |
| `create_time` | String | Output only. The time at which this CertificateTemplate was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate_template
certificate_template = provider.privateca_api.Certificate_template {
    parent = "value"  # Required. The resource name of the location associated with the CertificateTemplate, in the format `projects/*/locations/*`.
}

# Access certificate_template outputs
certificate_template_id = certificate_template.id
certificate_template_description = certificate_template.description
certificate_template_name = certificate_template.name
certificate_template_identity_constraints = certificate_template.identity_constraints
certificate_template_predefined_values = certificate_template.predefined_values
certificate_template_labels = certificate_template.labels
certificate_template_update_time = certificate_template.update_time
certificate_template_passthrough_extensions = certificate_template.passthrough_extensions
certificate_template_maximum_lifetime = certificate_template.maximum_lifetime
certificate_template_create_time = certificate_template.create_time
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
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
operation = provider.privateca_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
```

---


### Certificate_revocation_list

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `revision_id` | String | Output only. The revision ID of this CertificateRevocationList. A new revision is committed whenever a new CRL is published. The format is an 8-character hexadecimal string. |
| `access_url` | String | Output only. The location where 'pem_crl' can be accessed. |
| `pem_crl` | String | Output only. The PEM-encoded X.509 CRL. |
| `name` | String | Identifier. The resource name for this CertificateRevocationList in the format `projects/*/locations/*/caPools/*certificateAuthorities/*/ certificateRevocationLists/*`. |
| `update_time` | String | Output only. The time at which this CertificateRevocationList was updated. |
| `revoked_certificates` | Vec<String> | Output only. The revoked serial numbers that appear in pem_crl. |
| `create_time` | String | Output only. The time at which this CertificateRevocationList was created. |
| `sequence_number` | String | Output only. The CRL sequence number that appears in pem_crl. |
| `labels` | HashMap<String, String> | Optional. Labels with user-defined metadata. |
| `state` | String | Output only. The State for this CertificateRevocationList. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate_revocation_list
certificate_revocation_list = provider.privateca_api.Certificate_revocation_list {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access certificate_revocation_list outputs
certificate_revocation_list_id = certificate_revocation_list.id
certificate_revocation_list_revision_id = certificate_revocation_list.revision_id
certificate_revocation_list_access_url = certificate_revocation_list.access_url
certificate_revocation_list_pem_crl = certificate_revocation_list.pem_crl
certificate_revocation_list_name = certificate_revocation_list.name
certificate_revocation_list_update_time = certificate_revocation_list.update_time
certificate_revocation_list_revoked_certificates = certificate_revocation_list.revoked_certificates
certificate_revocation_list_create_time = certificate_revocation_list.create_time
certificate_revocation_list_sequence_number = certificate_revocation_list.sequence_number
certificate_revocation_list_labels = certificate_revocation_list.labels
certificate_revocation_list_state = certificate_revocation_list.state
```

---


### Certificate

Create a new Certificate in a given Project, Location from a particular CaPool.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The time at which this Certificate was updated. |
| `labels` | HashMap<String, String> |  | Optional. Labels with user-defined metadata. |
| `issuer_certificate_authority` | String |  | Output only. The resource name of the issuing CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`. |
| `config` | String |  | Immutable. A description of the certificate and key that does not require X.509 or ASN.1. |
| `create_time` | String |  | Output only. The time at which this Certificate was created. |
| `name` | String |  | Identifier. The resource name for this Certificate in the format `projects/*/locations/*/caPools/*/certificates/*`. |
| `certificate_template` | String |  | Immutable. The resource name for a CertificateTemplate used to issue this certificate, in the format `projects/*/locations/*/certificateTemplates/*`. If this is specified, the caller must have the necessary permission to use this template. If this is omitted, no template will be used. This template must be in the same location as the Certificate. |
| `certificate_description` | String |  | Output only. A structured description of the issued X.509 certificate. |
| `lifetime` | String |  | Required. Immutable. The desired lifetime of a certificate. Used to create the "not_before_time" and "not_after_time" fields inside an X.509 certificate. Note that the lifetime may be truncated if it would extend past the life of any certificate authority in the issuing chain. |
| `pem_certificate` | String |  | Output only. The pem-encoded, signed X.509 certificate. |
| `pem_certificate_chain` | Vec<String> |  | Output only. The chain that may be used to verify the X.509 certificate. Expected to be in issuer-to-root order according to RFC 5246. |
| `revocation_details` | String |  | Output only. Details regarding the revocation of this Certificate. This Certificate is considered revoked if and only if this field is present. |
| `subject_mode` | String |  | Immutable. Specifies how the Certificate's identity fields are to be decided. If this is omitted, the `DEFAULT` subject mode will be used. |
| `pem_csr` | String |  | Immutable. A pem-encoded X.509 certificate signing request (CSR). |
| `parent` | String | ✅ | Required. The resource name of the CaPool associated with the Certificate, in the format `projects/*/locations/*/caPools/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time at which this Certificate was updated. |
| `labels` | HashMap<String, String> | Optional. Labels with user-defined metadata. |
| `issuer_certificate_authority` | String | Output only. The resource name of the issuing CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`. |
| `config` | String | Immutable. A description of the certificate and key that does not require X.509 or ASN.1. |
| `create_time` | String | Output only. The time at which this Certificate was created. |
| `name` | String | Identifier. The resource name for this Certificate in the format `projects/*/locations/*/caPools/*/certificates/*`. |
| `certificate_template` | String | Immutable. The resource name for a CertificateTemplate used to issue this certificate, in the format `projects/*/locations/*/certificateTemplates/*`. If this is specified, the caller must have the necessary permission to use this template. If this is omitted, no template will be used. This template must be in the same location as the Certificate. |
| `certificate_description` | String | Output only. A structured description of the issued X.509 certificate. |
| `lifetime` | String | Required. Immutable. The desired lifetime of a certificate. Used to create the "not_before_time" and "not_after_time" fields inside an X.509 certificate. Note that the lifetime may be truncated if it would extend past the life of any certificate authority in the issuing chain. |
| `pem_certificate` | String | Output only. The pem-encoded, signed X.509 certificate. |
| `pem_certificate_chain` | Vec<String> | Output only. The chain that may be used to verify the X.509 certificate. Expected to be in issuer-to-root order according to RFC 5246. |
| `revocation_details` | String | Output only. Details regarding the revocation of this Certificate. This Certificate is considered revoked if and only if this field is present. |
| `subject_mode` | String | Immutable. Specifies how the Certificate's identity fields are to be decided. If this is omitted, the `DEFAULT` subject mode will be used. |
| `pem_csr` | String | Immutable. A pem-encoded X.509 certificate signing request (CSR). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate
certificate = provider.privateca_api.Certificate {
    parent = "value"  # Required. The resource name of the CaPool associated with the Certificate, in the format `projects/*/locations/*/caPools/*`.
}

# Access certificate outputs
certificate_id = certificate.id
certificate_update_time = certificate.update_time
certificate_labels = certificate.labels
certificate_issuer_certificate_authority = certificate.issuer_certificate_authority
certificate_config = certificate.config
certificate_create_time = certificate.create_time
certificate_name = certificate.name
certificate_certificate_template = certificate.certificate_template
certificate_certificate_description = certificate.certificate_description
certificate_lifetime = certificate.lifetime
certificate_pem_certificate = certificate.pem_certificate
certificate_pem_certificate_chain = certificate.pem_certificate_chain
certificate_revocation_details = certificate.revocation_details
certificate_subject_mode = certificate.subject_mode
certificate_pem_csr = certificate.pem_csr
```

---


### Certificate_authoritie

Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may "fail open" without warning.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions). |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate_authoritie
certificate_authoritie = provider.privateca_api.Certificate_authoritie {
    resource = "value"  # REQUIRED: The resource for which the policy detail is being requested. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access certificate_authoritie outputs
certificate_authoritie_id = certificate_authoritie.id
certificate_authoritie_version = certificate_authoritie.version
certificate_authoritie_bindings = certificate_authoritie.bindings
certificate_authoritie_etag = certificate_authoritie.etag
certificate_authoritie_audit_configs = certificate_authoritie.audit_configs
```

---


### Certificate_revocation_list

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create certificate_revocation_list
certificate_revocation_list = provider.privateca_api.Certificate_revocation_list {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access certificate_revocation_list outputs
certificate_revocation_list_id = certificate_revocation_list.id
certificate_revocation_list_version = certificate_revocation_list.version
certificate_revocation_list_bindings = certificate_revocation_list.bindings
certificate_revocation_list_etag = certificate_revocation_list.etag
certificate_revocation_list_audit_configs = certificate_revocation_list.audit_configs
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


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
location_display_name = location.display_name
location_location_id = location.location_id
location_metadata = location.metadata
location_labels = location.labels
location_name = location.name
```

---


### Reusable_config

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reusable_config
reusable_config = provider.privateca_api.Reusable_config {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access reusable_config outputs
reusable_config_id = reusable_config.id
reusable_config_version = reusable_config.version
reusable_config_bindings = reusable_config.bindings
reusable_config_etag = reusable_config.etag
reusable_config_audit_configs = reusable_config.audit_configs
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
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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
operation = provider.privateca_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_name = operation.name
operation_done = operation.done
operation_metadata = operation.metadata
operation_response = operation.response
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple location resources
location_0 = provider.privateca_api.Location {
}
location_1 = provider.privateca_api.Location {
}
location_2 = provider.privateca_api.Location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.privateca_api.Location {
    }
```

---

## Related Documentation

- [GCP Privateca_api Documentation](https://cloud.google.com/privateca_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
