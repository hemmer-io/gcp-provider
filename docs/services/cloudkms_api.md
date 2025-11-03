# Cloudkms_api Service



**Resources**: 12

---

## Overview

The cloudkms_api service provides access to 12 resource types:

- [Import_job](#import_job) [CR]
- [Ekm_connection](#ekm_connection) [CRU]
- [Crypto_key_version](#crypto_key_version) [CRU]
- [Key_ring](#key_ring) [CR]
- [Crypto_key](#crypto_key) [CRU]
- [Organization](#organization) [RU]
- [Project](#project) [RU]
- [Key_handle](#key_handle) [CR]
- [Operation](#operation) [R]
- [Ekm_config](#ekm_config) [CR]
- [Folder](#folder) [RU]
- [Location](#location) [CRU]

---

## Resources


### Import_job

Create a new ImportJob within a KeyRing. ImportJob.import_method is required.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expire_event_time` | String |  | Output only. The time this ImportJob expired. Only present if state is EXPIRED. |
| `import_method` | String |  | Required. Immutable. The wrapping method to be used for incoming key material. |
| `state` | String |  | Output only. The current state of the ImportJob, indicating if it can be used. |
| `expire_time` | String |  | Output only. The time at which this ImportJob is scheduled for expiration and can no longer be used to import key material. |
| `protection_level` | String |  | Required. Immutable. The protection level of the ImportJob. This must match the protection_level of the version_template on the CryptoKey you attempt to import into. |
| `generate_time` | String |  | Output only. The time this ImportJob's key material was generated. |
| `attestation` | String |  | Output only. Statement that was generated and signed by the key creator (for example, an HSM) at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google. Only present if the chosen ImportMethod is one with a protection level of HSM. |
| `create_time` | String |  | Output only. The time at which this ImportJob was created. |
| `public_key` | String |  | Output only. The public key with which to wrap key material prior to import. Only returned if state is ACTIVE. |
| `name` | String |  | Output only. The resource name for this ImportJob in the format `projects/*/locations/*/keyRings/*/importJobs/*`. |
| `parent` | String | ✅ | Required. The name of the KeyRing associated with the ImportJobs. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expire_event_time` | String | Output only. The time this ImportJob expired. Only present if state is EXPIRED. |
| `import_method` | String | Required. Immutable. The wrapping method to be used for incoming key material. |
| `state` | String | Output only. The current state of the ImportJob, indicating if it can be used. |
| `expire_time` | String | Output only. The time at which this ImportJob is scheduled for expiration and can no longer be used to import key material. |
| `protection_level` | String | Required. Immutable. The protection level of the ImportJob. This must match the protection_level of the version_template on the CryptoKey you attempt to import into. |
| `generate_time` | String | Output only. The time this ImportJob's key material was generated. |
| `attestation` | String | Output only. Statement that was generated and signed by the key creator (for example, an HSM) at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google. Only present if the chosen ImportMethod is one with a protection level of HSM. |
| `create_time` | String | Output only. The time at which this ImportJob was created. |
| `public_key` | String | Output only. The public key with which to wrap key material prior to import. Only returned if state is ACTIVE. |
| `name` | String | Output only. The resource name for this ImportJob in the format `projects/*/locations/*/keyRings/*/importJobs/*`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create import_job
import_job = provider.cloudkms_api.Import_job {
    parent = "value"  # Required. The name of the KeyRing associated with the ImportJobs.
}

# Access import_job outputs
import_job_id = import_job.id
import_job_expire_event_time = import_job.expire_event_time
import_job_import_method = import_job.import_method
import_job_state = import_job.state
import_job_expire_time = import_job.expire_time
import_job_protection_level = import_job.protection_level
import_job_generate_time = import_job.generate_time
import_job_attestation = import_job.attestation
import_job_create_time = import_job.create_time
import_job_public_key = import_job.public_key
import_job_name = import_job.name
```

---


### Ekm_connection

Creates a new EkmConnection in a given Project and Location.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `crypto_space_path` | String |  | Optional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if KeyManagementMode is CLOUD_KMS. |
| `etag` | String |  | Optional. Etag of the currently stored EkmConnection. |
| `key_management_mode` | String |  | Optional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL. |
| `name` | String |  | Output only. The resource name for the EkmConnection in the format `projects/*/locations/*/ekmConnections/*`. |
| `service_resolvers` | Vec<String> |  | Optional. A list of ServiceResolvers where the EKM can be reached. There should be one ServiceResolver per EKM replica. Currently, only a single ServiceResolver is supported. |
| `create_time` | String |  | Output only. The time at which the EkmConnection was created. |
| `parent` | String | ✅ | Required. The resource name of the location associated with the EkmConnection, in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `crypto_space_path` | String | Optional. Identifies the EKM Crypto Space that this EkmConnection maps to. Note: This field is required if KeyManagementMode is CLOUD_KMS. |
| `etag` | String | Optional. Etag of the currently stored EkmConnection. |
| `key_management_mode` | String | Optional. Describes who can perform control plane operations on the EKM. If unset, this defaults to MANUAL. |
| `name` | String | Output only. The resource name for the EkmConnection in the format `projects/*/locations/*/ekmConnections/*`. |
| `service_resolvers` | Vec<String> | Optional. A list of ServiceResolvers where the EKM can be reached. There should be one ServiceResolver per EKM replica. Currently, only a single ServiceResolver is supported. |
| `create_time` | String | Output only. The time at which the EkmConnection was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ekm_connection
ekm_connection = provider.cloudkms_api.Ekm_connection {
    parent = "value"  # Required. The resource name of the location associated with the EkmConnection, in the format `projects/*/locations/*`.
}

# Access ekm_connection outputs
ekm_connection_id = ekm_connection.id
ekm_connection_crypto_space_path = ekm_connection.crypto_space_path
ekm_connection_etag = ekm_connection.etag
ekm_connection_key_management_mode = ekm_connection.key_management_mode
ekm_connection_name = ekm_connection.name
ekm_connection_service_resolvers = ekm_connection.service_resolvers
ekm_connection_create_time = ekm_connection.create_time
```

---


### Crypto_key_version

Create a new CryptoKeyVersion in a CryptoKey. The server will assign the next sequential id. If unset, state will be set to ENABLED.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `external_protection_level_options` | String |  | ExternalProtectionLevelOptions stores a group of additional fields for configuring a CryptoKeyVersion that are specific to the EXTERNAL protection level and EXTERNAL_VPC protection levels. |
| `protection_level` | String |  | Output only. The ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion. |
| `state` | String |  | The current state of the CryptoKeyVersion. |
| `generation_failure_reason` | String |  | Output only. The root cause of the most recent generation failure. Only present if state is GENERATION_FAILED. |
| `algorithm` | String |  | Output only. The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports. |
| `external_destruction_failure_reason` | String |  | Output only. The root cause of the most recent external destruction failure. Only present if state is EXTERNAL_DESTRUCTION_FAILED. |
| `destroy_time` | String |  | Output only. The time this CryptoKeyVersion's key material is scheduled for destruction. Only present if state is DESTROY_SCHEDULED. |
| `import_failure_reason` | String |  | Output only. The root cause of the most recent import failure. Only present if state is IMPORT_FAILED. |
| `attestation` | String |  | Output only. Statement that was generated and signed by the HSM at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google. Only provided for key versions with protection_level HSM. |
| `destroy_event_time` | String |  | Output only. The time this CryptoKeyVersion's key material was destroyed. Only present if state is DESTROYED. |
| `generate_time` | String |  | Output only. The time this CryptoKeyVersion's key material was generated. |
| `import_job` | String |  | Output only. The name of the ImportJob used in the most recent import of this CryptoKeyVersion. Only present if the underlying key material was imported. |
| `import_time` | String |  | Output only. The time at which this CryptoKeyVersion's key material was most recently imported. |
| `name` | String |  | Output only. The resource name for this CryptoKeyVersion in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`. |
| `reimport_eligible` | bool |  | Output only. Whether or not this key version is eligible for reimport, by being specified as a target in ImportCryptoKeyVersionRequest.crypto_key_version. |
| `create_time` | String |  | Output only. The time at which this CryptoKeyVersion was created. |
| `parent` | String | ✅ | Required. The name of the CryptoKey associated with the CryptoKeyVersions. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `external_protection_level_options` | String | ExternalProtectionLevelOptions stores a group of additional fields for configuring a CryptoKeyVersion that are specific to the EXTERNAL protection level and EXTERNAL_VPC protection levels. |
| `protection_level` | String | Output only. The ProtectionLevel describing how crypto operations are performed with this CryptoKeyVersion. |
| `state` | String | The current state of the CryptoKeyVersion. |
| `generation_failure_reason` | String | Output only. The root cause of the most recent generation failure. Only present if state is GENERATION_FAILED. |
| `algorithm` | String | Output only. The CryptoKeyVersionAlgorithm that this CryptoKeyVersion supports. |
| `external_destruction_failure_reason` | String | Output only. The root cause of the most recent external destruction failure. Only present if state is EXTERNAL_DESTRUCTION_FAILED. |
| `destroy_time` | String | Output only. The time this CryptoKeyVersion's key material is scheduled for destruction. Only present if state is DESTROY_SCHEDULED. |
| `import_failure_reason` | String | Output only. The root cause of the most recent import failure. Only present if state is IMPORT_FAILED. |
| `attestation` | String | Output only. Statement that was generated and signed by the HSM at key creation time. Use this statement to verify attributes of the key as stored on the HSM, independently of Google. Only provided for key versions with protection_level HSM. |
| `destroy_event_time` | String | Output only. The time this CryptoKeyVersion's key material was destroyed. Only present if state is DESTROYED. |
| `generate_time` | String | Output only. The time this CryptoKeyVersion's key material was generated. |
| `import_job` | String | Output only. The name of the ImportJob used in the most recent import of this CryptoKeyVersion. Only present if the underlying key material was imported. |
| `import_time` | String | Output only. The time at which this CryptoKeyVersion's key material was most recently imported. |
| `name` | String | Output only. The resource name for this CryptoKeyVersion in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`. |
| `reimport_eligible` | bool | Output only. Whether or not this key version is eligible for reimport, by being specified as a target in ImportCryptoKeyVersionRequest.crypto_key_version. |
| `create_time` | String | Output only. The time at which this CryptoKeyVersion was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create crypto_key_version
crypto_key_version = provider.cloudkms_api.Crypto_key_version {
    parent = "value"  # Required. The name of the CryptoKey associated with the CryptoKeyVersions.
}

# Access crypto_key_version outputs
crypto_key_version_id = crypto_key_version.id
crypto_key_version_external_protection_level_options = crypto_key_version.external_protection_level_options
crypto_key_version_protection_level = crypto_key_version.protection_level
crypto_key_version_state = crypto_key_version.state
crypto_key_version_generation_failure_reason = crypto_key_version.generation_failure_reason
crypto_key_version_algorithm = crypto_key_version.algorithm
crypto_key_version_external_destruction_failure_reason = crypto_key_version.external_destruction_failure_reason
crypto_key_version_destroy_time = crypto_key_version.destroy_time
crypto_key_version_import_failure_reason = crypto_key_version.import_failure_reason
crypto_key_version_attestation = crypto_key_version.attestation
crypto_key_version_destroy_event_time = crypto_key_version.destroy_event_time
crypto_key_version_generate_time = crypto_key_version.generate_time
crypto_key_version_import_job = crypto_key_version.import_job
crypto_key_version_import_time = crypto_key_version.import_time
crypto_key_version_name = crypto_key_version.name
crypto_key_version_reimport_eligible = crypto_key_version.reimport_eligible
crypto_key_version_create_time = crypto_key_version.create_time
```

---


### Key_ring

Create a new KeyRing in a given Project and Location.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time at which this KeyRing was created. |
| `name` | String |  | Output only. The resource name for the KeyRing in the format `projects/*/locations/*/keyRings/*`. |
| `parent` | String | ✅ | Required. The resource name of the location associated with the KeyRings, in the format `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time at which this KeyRing was created. |
| `name` | String | Output only. The resource name for the KeyRing in the format `projects/*/locations/*/keyRings/*`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create key_ring
key_ring = provider.cloudkms_api.Key_ring {
    parent = "value"  # Required. The resource name of the location associated with the KeyRings, in the format `projects/*/locations/*`.
}

# Access key_ring outputs
key_ring_id = key_ring.id
key_ring_create_time = key_ring.create_time
key_ring_name = key_ring.name
```

---


### Crypto_key

Create a new CryptoKey within a KeyRing. CryptoKey.purpose and CryptoKey.version_template.algorithm are required.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `next_rotation_time` | String |  | At next_rotation_time, the Key Management Service will automatically: 1. Create a new version of this CryptoKey. 2. Mark the new version as primary. Key rotations performed manually via CreateCryptoKeyVersion and UpdateCryptoKeyPrimaryVersion do not affect next_rotation_time. Keys with purpose ENCRYPT_DECRYPT support automatic rotation. For other keys, this field must be omitted. |
| `destroy_scheduled_duration` | String |  | Immutable. The period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED. If not specified at creation time, the default duration is 30 days. |
| `import_only` | bool |  | Immutable. Whether this key may contain imported versions only. |
| `name` | String |  | Output only. The resource name for this CryptoKey in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `create_time` | String |  | Output only. The time at which this CryptoKey was created. |
| `rotation_period` | String |  | next_rotation_time will be advanced by this period when the service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours. If rotation_period is set, next_rotation_time must also be set. Keys with purpose ENCRYPT_DECRYPT support automatic rotation. For other keys, this field must be omitted. |
| `labels` | HashMap<String, String> |  | Labels with user-defined metadata. For more information, see [Labeling Keys](https://cloud.google.com/kms/docs/labeling-keys). |
| `primary` | String |  | Output only. A copy of the "primary" CryptoKeyVersion that will be used by Encrypt when this CryptoKey is given in EncryptRequest.name. The CryptoKey's primary version can be updated via UpdateCryptoKeyPrimaryVersion. Keys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be omitted. |
| `purpose` | String |  | Immutable. The immutable purpose of this CryptoKey. |
| `version_template` | String |  | A template describing settings for new CryptoKeyVersion instances. The properties of new CryptoKeyVersion instances created by either CreateCryptoKeyVersion or auto-rotation are controlled by this template. |
| `key_access_justifications_policy` | String |  | Optional. The policy used for Key Access Justifications Policy Enforcement. If this field is present and this key is enrolled in Key Access Justifications Policy Enforcement, the policy will be evaluated in encrypt, decrypt, and sign operations, and the operation will fail if rejected by the policy. The policy is defined by specifying zero or more allowed justification codes. https://cloud.google.com/assured-workloads/key-access-justifications/docs/justification-codes By default, this field is absent, and all justification codes are allowed. |
| `crypto_key_backend` | String |  | Immutable. The resource name of the backend environment where the key material for all CryptoKeyVersions associated with this CryptoKey reside and where all related cryptographic operations are performed. Only applicable if CryptoKeyVersions have a ProtectionLevel of EXTERNAL_VPC, with the resource name in the format `projects/*/locations/*/ekmConnections/*`. Note, this list is non-exhaustive and may apply to additional ProtectionLevels in the future. |
| `parent` | String | ✅ | Required. The name of the KeyRing associated with the CryptoKeys. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_rotation_time` | String | At next_rotation_time, the Key Management Service will automatically: 1. Create a new version of this CryptoKey. 2. Mark the new version as primary. Key rotations performed manually via CreateCryptoKeyVersion and UpdateCryptoKeyPrimaryVersion do not affect next_rotation_time. Keys with purpose ENCRYPT_DECRYPT support automatic rotation. For other keys, this field must be omitted. |
| `destroy_scheduled_duration` | String | Immutable. The period of time that versions of this key spend in the DESTROY_SCHEDULED state before transitioning to DESTROYED. If not specified at creation time, the default duration is 30 days. |
| `import_only` | bool | Immutable. Whether this key may contain imported versions only. |
| `name` | String | Output only. The resource name for this CryptoKey in the format `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `create_time` | String | Output only. The time at which this CryptoKey was created. |
| `rotation_period` | String | next_rotation_time will be advanced by this period when the service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours. If rotation_period is set, next_rotation_time must also be set. Keys with purpose ENCRYPT_DECRYPT support automatic rotation. For other keys, this field must be omitted. |
| `labels` | HashMap<String, String> | Labels with user-defined metadata. For more information, see [Labeling Keys](https://cloud.google.com/kms/docs/labeling-keys). |
| `primary` | String | Output only. A copy of the "primary" CryptoKeyVersion that will be used by Encrypt when this CryptoKey is given in EncryptRequest.name. The CryptoKey's primary version can be updated via UpdateCryptoKeyPrimaryVersion. Keys with purpose ENCRYPT_DECRYPT may have a primary. For other keys, this field will be omitted. |
| `purpose` | String | Immutable. The immutable purpose of this CryptoKey. |
| `version_template` | String | A template describing settings for new CryptoKeyVersion instances. The properties of new CryptoKeyVersion instances created by either CreateCryptoKeyVersion or auto-rotation are controlled by this template. |
| `key_access_justifications_policy` | String | Optional. The policy used for Key Access Justifications Policy Enforcement. If this field is present and this key is enrolled in Key Access Justifications Policy Enforcement, the policy will be evaluated in encrypt, decrypt, and sign operations, and the operation will fail if rejected by the policy. The policy is defined by specifying zero or more allowed justification codes. https://cloud.google.com/assured-workloads/key-access-justifications/docs/justification-codes By default, this field is absent, and all justification codes are allowed. |
| `crypto_key_backend` | String | Immutable. The resource name of the backend environment where the key material for all CryptoKeyVersions associated with this CryptoKey reside and where all related cryptographic operations are performed. Only applicable if CryptoKeyVersions have a ProtectionLevel of EXTERNAL_VPC, with the resource name in the format `projects/*/locations/*/ekmConnections/*`. Note, this list is non-exhaustive and may apply to additional ProtectionLevels in the future. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create crypto_key
crypto_key = provider.cloudkms_api.Crypto_key {
    parent = "value"  # Required. The name of the KeyRing associated with the CryptoKeys.
}

# Access crypto_key outputs
crypto_key_id = crypto_key.id
crypto_key_next_rotation_time = crypto_key.next_rotation_time
crypto_key_destroy_scheduled_duration = crypto_key.destroy_scheduled_duration
crypto_key_import_only = crypto_key.import_only
crypto_key_name = crypto_key.name
crypto_key_create_time = crypto_key.create_time
crypto_key_rotation_period = crypto_key.rotation_period
crypto_key_labels = crypto_key.labels
crypto_key_primary = crypto_key.primary
crypto_key_purpose = crypto_key.purpose
crypto_key_version_template = crypto_key.version_template
crypto_key_key_access_justifications_policy = crypto_key.key_access_justifications_policy
crypto_key_crypto_key_backend = crypto_key.crypto_key_backend
```

---


### Organization

Gets the KeyAccessJustificationsPolicyConfig for a given organization, folder, or project.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_key_access_justification_policy` | String |  | Optional. The default key access justification policy used when a CryptoKey is created in this folder. This is only used when a Key Access Justifications policy is not provided in the CreateCryptoKeyRequest. This overrides any default policies in its ancestry. |
| `name` | String |  | Identifier. The resource name for this KeyAccessJustificationsPolicyConfig in the format of "{organizations|folders|projects}/*/kajPolicyConfig". |
| `name` | String | ✅ | Identifier. The resource name for this KeyAccessJustificationsPolicyConfig in the format of "{organizations|folders|projects}/*/kajPolicyConfig". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_key_access_justification_policy` | String | Optional. The default key access justification policy used when a CryptoKey is created in this folder. This is only used when a Key Access Justifications policy is not provided in the CreateCryptoKeyRequest. This overrides any default policies in its ancestry. |
| `name` | String | Identifier. The resource name for this KeyAccessJustificationsPolicyConfig in the format of "{organizations|folders|projects}/*/kajPolicyConfig". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access organization outputs
organization_id = organization.id
organization_default_key_access_justification_policy = organization.default_key_access_justification_policy
organization_name = organization.name
```

---


### Project

Returns the KeyAccessJustificationsPolicyConfig of the resource closest to the given project in hierarchy.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `default_key_access_justification_policy` | String |  | Optional. The default key access justification policy used when a CryptoKey is created in this folder. This is only used when a Key Access Justifications policy is not provided in the CreateCryptoKeyRequest. This overrides any default policies in its ancestry. |
| `name` | String |  | Identifier. The resource name for this KeyAccessJustificationsPolicyConfig in the format of "{organizations|folders|projects}/*/kajPolicyConfig". |
| `name` | String | ✅ | Identifier. The resource name for this KeyAccessJustificationsPolicyConfig in the format of "{organizations|folders|projects}/*/kajPolicyConfig". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `effective_kaj_policy` | String | The effective KeyAccessJustificationsPolicyConfig. |


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
project_effective_kaj_policy = project.effective_kaj_policy
```

---


### Key_handle

Creates a new KeyHandle, triggering the provisioning of a new CryptoKey for CMEK use with the given resource type in the configured key project and the same location. GetOperation should be used to resolve the resulting long-running operation and get the resulting KeyHandle and CryptoKey.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kms_key` | String |  | Output only. Name of a CryptoKey that has been provisioned for Customer Managed Encryption Key (CMEK) use in the KeyHandle project and location for the requested resource type. The CryptoKey project will reflect the value configured in the AutokeyConfig on the resource project's ancestor folder at the time of the KeyHandle creation. If more than one ancestor folder has a configured AutokeyConfig, the nearest of these configurations is used. |
| `name` | String |  | Identifier. Name of the KeyHandle resource, e.g. `projects/{PROJECT_ID}/locations/{LOCATION}/keyHandles/{KEY_HANDLE_ID}`. |
| `resource_type_selector` | String |  | Required. Indicates the resource type that the resulting CryptoKey is meant to protect, e.g. `{SERVICE}.googleapis.com/{TYPE}`. See documentation for supported resource types. |
| `parent` | String | ✅ | Required. Name of the resource project and location to create the KeyHandle in, e.g. `projects/{PROJECT_ID}/locations/{LOCATION}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kms_key` | String | Output only. Name of a CryptoKey that has been provisioned for Customer Managed Encryption Key (CMEK) use in the KeyHandle project and location for the requested resource type. The CryptoKey project will reflect the value configured in the AutokeyConfig on the resource project's ancestor folder at the time of the KeyHandle creation. If more than one ancestor folder has a configured AutokeyConfig, the nearest of these configurations is used. |
| `name` | String | Identifier. Name of the KeyHandle resource, e.g. `projects/{PROJECT_ID}/locations/{LOCATION}/keyHandles/{KEY_HANDLE_ID}`. |
| `resource_type_selector` | String | Required. Indicates the resource type that the resulting CryptoKey is meant to protect, e.g. `{SERVICE}.googleapis.com/{TYPE}`. See documentation for supported resource types. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create key_handle
key_handle = provider.cloudkms_api.Key_handle {
    parent = "value"  # Required. Name of the resource project and location to create the KeyHandle in, e.g. `projects/{PROJECT_ID}/locations/{LOCATION}`.
}

# Access key_handle outputs
key_handle_id = key_handle.id
key_handle_kms_key = key_handle.kms_key
key_handle_name = key_handle.name
key_handle_resource_type_selector = key_handle.resource_type_selector
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_response = operation.response
```

---


### Ekm_config

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
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
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

# Create ekm_config
ekm_config = provider.cloudkms_api.Ekm_config {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access ekm_config outputs
ekm_config_id = ekm_config.id
ekm_config_version = ekm_config.version
ekm_config_bindings = ekm_config.bindings
ekm_config_audit_configs = ekm_config.audit_configs
ekm_config_etag = ekm_config.etag
```

---


### Folder

Gets the KeyAccessJustificationsPolicyConfig for a given organization, folder, or project.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | Output only. The state for the AutokeyConfig. |
| `etag` | String |  | Optional. A checksum computed by the server based on the value of other fields. This may be sent on update requests to ensure that the client has an up-to-date value before proceeding. The request will be rejected with an ABORTED error on a mismatched etag. |
| `name` | String |  | Identifier. Name of the AutokeyConfig resource, e.g. `folders/{FOLDER_NUMBER}/autokeyConfig` `projects/{PROJECT_NUMBER}/autokeyConfig`. |
| `key_project` | String |  | Optional. Name of the key project, e.g. `projects/{PROJECT_ID}` or `projects/{PROJECT_NUMBER}`, where Cloud KMS Autokey will provision a new CryptoKey when a KeyHandle is created. On UpdateAutokeyConfig, the caller will require `cloudkms.cryptoKeys.setIamPolicy` permission on this key project. Once configured, for Cloud KMS Autokey to function properly, this key project must have the Cloud KMS API activated and the Cloud KMS Service Agent for this key project must be granted the `cloudkms.admin` role (or pertinent permissions). A request with an empty key project field will clear the configuration. |
| `name` | String | ✅ | Identifier. Name of the AutokeyConfig resource, e.g. `folders/{FOLDER_NUMBER}/autokeyConfig` `projects/{PROJECT_NUMBER}/autokeyConfig`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `default_key_access_justification_policy` | String | Optional. The default key access justification policy used when a CryptoKey is created in this folder. This is only used when a Key Access Justifications policy is not provided in the CreateCryptoKeyRequest. This overrides any default policies in its ancestry. |
| `name` | String | Identifier. The resource name for this KeyAccessJustificationsPolicyConfig in the format of "{organizations|folders|projects}/*/kajPolicyConfig". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access folder outputs
folder_id = folder.id
folder_default_key_access_justification_policy = folder.default_key_access_justification_policy
folder_name = folder.name
```

---


### Location

Generate random bytes using the Cloud KMS randomness source in the provided location.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `length_bytes` | i64 |  | The length in bytes of the amount of randomness to retrieve. Minimum 8 bytes, maximum 1024 bytes. |
| `protection_level` | String |  | The ProtectionLevel to use when generating the random data. Currently, only HSM protection level is supported. |
| `location` | String | ✅ | The project-specific location in which to generate random bytes. For example, "projects/my-project/locations/us-central1". |


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

# Create location
location = provider.cloudkms_api.Location {
    location = "value"  # The project-specific location in which to generate random bytes. For example, "projects/my-project/locations/us-central1".
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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple import_job resources
import_job_0 = provider.cloudkms_api.Import_job {
    parent = "value-0"
}
import_job_1 = provider.cloudkms_api.Import_job {
    parent = "value-1"
}
import_job_2 = provider.cloudkms_api.Import_job {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    import_job = provider.cloudkms_api.Import_job {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudkms_api Documentation](https://cloud.google.com/cloudkms_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
