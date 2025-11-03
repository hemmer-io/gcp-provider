# Containeranalysis_api Service



**Resources**: 10

---

## Overview

The containeranalysis_api service provides access to 10 resource types:

- [Resource](#resource) [C]
- [Note](#note) [CRUD]
- [Occurrence](#occurrence) [CRUD]
- [Occurrence](#occurrence) [CRUD]
- [Resource](#resource) [C]
- [Note](#note) [CRUD]
- [Note](#note) [CRUD]
- [Scan_config](#scan_config) [RU]
- [Occurrence](#occurrence) [CRUD]
- [Operation](#operation) [CU]

---

## Resources


### Resource

Generates an SBOM for the given resource.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cloud_storage_location` | String |  | Optional. Empty placeholder to denote that this is a Google Cloud Storage export request. |
| `name` | String | ✅ | Required. The name of the resource in the form of `projects/[PROJECT_ID]/resources/[RESOURCE_URL]`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create resource
resource = provider.containeranalysis_api.Resource {
    name = "value"  # Required. The name of the resource in the form of `projects/[PROJECT_ID]/resources/[RESOURCE_URL]`.
}

```

---


### Note

Creates a new note.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. |
| `expiration_time` | String |  | Time of expiration for this note. Empty if note does not expire. |
| `image` | String |  | A note describing a base image. |
| `create_time` | String |  | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `vulnerability` | String |  | A note describing a package vulnerability. |
| `kind` | String |  | Output only. The type of analysis. This field can be used as a filter in list requests. |
| `attestation` | String |  | A note describing an attestation role. |
| `package` | String |  | A note describing a package hosted by various package managers. |
| `deployment` | String |  | A note describing something that can be deployed. |
| `build` | String |  | A note describing build provenance for a verifiable build. |
| `sbom_reference` | String |  | A note describing an SBOM reference. |
| `update_time` | String |  | Output only. The time this note was last updated. This field can be used as a filter in list requests. |
| `upgrade` | String |  | A note describing available package upgrades. |
| `secret` | String |  | A note describing a secret. |
| `long_description` | String |  | A detailed description of this note. |
| `vulnerability_assessment` | String |  | A note describing a vulnerability assessment. |
| `compliance` | String |  | A note describing a compliance check. |
| `discovery` | String |  | A note describing the initial analysis of a resource. |
| `related_note_names` | Vec<String> |  | Other notes related to this note. |
| `related_url` | Vec<String> |  | URLs associated with this note. |
| `short_description` | String |  | A one sentence description of this note. |
| `dsse_attestation` | String |  | A note describing a dsse attestation note. |
| `parent` | String | ✅ | Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the note is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. |
| `expiration_time` | String | Time of expiration for this note. Empty if note does not expire. |
| `image` | String | A note describing a base image. |
| `create_time` | String | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `vulnerability` | String | A note describing a package vulnerability. |
| `kind` | String | Output only. The type of analysis. This field can be used as a filter in list requests. |
| `attestation` | String | A note describing an attestation role. |
| `package` | String | A note describing a package hosted by various package managers. |
| `deployment` | String | A note describing something that can be deployed. |
| `build` | String | A note describing build provenance for a verifiable build. |
| `sbom_reference` | String | A note describing an SBOM reference. |
| `update_time` | String | Output only. The time this note was last updated. This field can be used as a filter in list requests. |
| `upgrade` | String | A note describing available package upgrades. |
| `secret` | String | A note describing a secret. |
| `long_description` | String | A detailed description of this note. |
| `vulnerability_assessment` | String | A note describing a vulnerability assessment. |
| `compliance` | String | A note describing a compliance check. |
| `discovery` | String | A note describing the initial analysis of a resource. |
| `related_note_names` | Vec<String> | Other notes related to this note. |
| `related_url` | Vec<String> | URLs associated with this note. |
| `short_description` | String | A one sentence description of this note. |
| `dsse_attestation` | String | A note describing a dsse attestation note. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create note
note = provider.containeranalysis_api.Note {
    parent = "value"  # Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the note is to be created.
}

# Access note outputs
note_id = note.id
note_name = note.name
note_expiration_time = note.expiration_time
note_image = note.image
note_create_time = note.create_time
note_vulnerability = note.vulnerability
note_kind = note.kind
note_attestation = note.attestation
note_package = note.package
note_deployment = note.deployment
note_build = note.build
note_sbom_reference = note.sbom_reference
note_update_time = note.update_time
note_upgrade = note.upgrade
note_secret = note.secret
note_long_description = note.long_description
note_vulnerability_assessment = note.vulnerability_assessment
note_compliance = note.compliance
note_discovery = note.discovery
note_related_note_names = note.related_note_names
note_related_url = note.related_url
note_short_description = note.short_description
note_dsse_attestation = note.dsse_attestation
```

---


### Occurrence

Creates a new occurrence.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `secret` | String |  | Describes a secret. |
| `build` | String |  | Describes a verifiable build. |
| `compliance` | String |  | Describes a compliance violation on a linked resource. |
| `name` | String |  | Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`. |
| `note_name` | String |  | Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests. |
| `kind` | String |  | Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests. |
| `dsse_attestation` | String |  | Describes an attestation of an artifact using dsse. |
| `envelope` | String |  | https://github.com/secure-systems-lab/dsse |
| `package` | String |  | Describes the installation of a package on the linked resource. |
| `update_time` | String |  | Output only. The time this occurrence was last updated. |
| `image` | String |  | Describes how this resource derives from the basis in the associated note. |
| `vulnerability` | String |  | Describes a security vulnerability. |
| `deployment` | String |  | Describes the deployment of an artifact on a runtime. |
| `resource_uri` | String |  | Required. Immutable. A URI that represents the resource for which the occurrence applies. For example, `https://gcr.io/project/image@sha256:123abc` for a Docker image. |
| `remediation` | String |  | A description of actions that can be taken to remedy the note. |
| `sbom_reference` | String |  | Describes a specific SBOM reference occurrences. |
| `upgrade` | String |  | Describes an available package upgrade on the linked resource. |
| `create_time` | String |  | Output only. The time this occurrence was created. |
| `attestation` | String |  | Describes an attestation of an artifact. |
| `discovery` | String |  | Describes when a resource was discovered. |
| `parent` | String | ✅ | Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrence is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `secret` | String | Describes a secret. |
| `build` | String | Describes a verifiable build. |
| `compliance` | String | Describes a compliance violation on a linked resource. |
| `name` | String | Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`. |
| `note_name` | String | Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests. |
| `kind` | String | Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests. |
| `dsse_attestation` | String | Describes an attestation of an artifact using dsse. |
| `envelope` | String | https://github.com/secure-systems-lab/dsse |
| `package` | String | Describes the installation of a package on the linked resource. |
| `update_time` | String | Output only. The time this occurrence was last updated. |
| `image` | String | Describes how this resource derives from the basis in the associated note. |
| `vulnerability` | String | Describes a security vulnerability. |
| `deployment` | String | Describes the deployment of an artifact on a runtime. |
| `resource_uri` | String | Required. Immutable. A URI that represents the resource for which the occurrence applies. For example, `https://gcr.io/project/image@sha256:123abc` for a Docker image. |
| `remediation` | String | A description of actions that can be taken to remedy the note. |
| `sbom_reference` | String | Describes a specific SBOM reference occurrences. |
| `upgrade` | String | Describes an available package upgrade on the linked resource. |
| `create_time` | String | Output only. The time this occurrence was created. |
| `attestation` | String | Describes an attestation of an artifact. |
| `discovery` | String | Describes when a resource was discovered. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create occurrence
occurrence = provider.containeranalysis_api.Occurrence {
    parent = "value"  # Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrence is to be created.
}

# Access occurrence outputs
occurrence_id = occurrence.id
occurrence_secret = occurrence.secret
occurrence_build = occurrence.build
occurrence_compliance = occurrence.compliance
occurrence_name = occurrence.name
occurrence_note_name = occurrence.note_name
occurrence_kind = occurrence.kind
occurrence_dsse_attestation = occurrence.dsse_attestation
occurrence_envelope = occurrence.envelope
occurrence_package = occurrence.package
occurrence_update_time = occurrence.update_time
occurrence_image = occurrence.image
occurrence_vulnerability = occurrence.vulnerability
occurrence_deployment = occurrence.deployment
occurrence_resource_uri = occurrence.resource_uri
occurrence_remediation = occurrence.remediation
occurrence_sbom_reference = occurrence.sbom_reference
occurrence_upgrade = occurrence.upgrade
occurrence_create_time = occurrence.create_time
occurrence_attestation = occurrence.attestation
occurrence_discovery = occurrence.discovery
```

---


### Occurrence

Creates a new occurrence.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sbom_reference` | String |  | Describes a specific SBOM reference occurrences. |
| `update_time` | String |  | Output only. The time this occurrence was last updated. |
| `note_name` | String |  | Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests. |
| `resource` | String |  | Required. Immutable. The resource for which the occurrence applies. |
| `derived_image` | String |  | Describes how this resource derives from the basis in the associated note. |
| `remediation` | String |  | A description of actions that can be taken to remedy the note. |
| `sbom` | String |  | Describes a specific software bill of materials document. |
| `envelope` | String |  | https://github.com/secure-systems-lab/dsse |
| `secret` | String |  | Describes a secret. |
| `spdx_package` | String |  | Describes a specific SPDX Package. |
| `spdx_relationship` | String |  | Describes a specific SPDX Relationship. |
| `vulnerability` | String |  | Describes a security vulnerability. |
| `deployment` | String |  | Describes the deployment of an artifact on a runtime. |
| `attestation` | String |  | Describes an attestation of an artifact. |
| `build` | String |  | Describes a verifiable build. |
| `intoto` | String |  | Describes a specific in-toto link. |
| `kind` | String |  | Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests. |
| `create_time` | String |  | Output only. The time this occurrence was created. |
| `installation` | String |  | Describes the installation of a package on the linked resource. |
| `name` | String |  | Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`. |
| `spdx_file` | String |  | Describes a specific SPDX File. |
| `discovered` | String |  | Describes when a resource was discovered. |
| `parent` | String | ✅ | Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrence is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sbom_reference` | String | Describes a specific SBOM reference occurrences. |
| `update_time` | String | Output only. The time this occurrence was last updated. |
| `note_name` | String | Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests. |
| `resource` | String | Required. Immutable. The resource for which the occurrence applies. |
| `derived_image` | String | Describes how this resource derives from the basis in the associated note. |
| `remediation` | String | A description of actions that can be taken to remedy the note. |
| `sbom` | String | Describes a specific software bill of materials document. |
| `envelope` | String | https://github.com/secure-systems-lab/dsse |
| `secret` | String | Describes a secret. |
| `spdx_package` | String | Describes a specific SPDX Package. |
| `spdx_relationship` | String | Describes a specific SPDX Relationship. |
| `vulnerability` | String | Describes a security vulnerability. |
| `deployment` | String | Describes the deployment of an artifact on a runtime. |
| `attestation` | String | Describes an attestation of an artifact. |
| `build` | String | Describes a verifiable build. |
| `intoto` | String | Describes a specific in-toto link. |
| `kind` | String | Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests. |
| `create_time` | String | Output only. The time this occurrence was created. |
| `installation` | String | Describes the installation of a package on the linked resource. |
| `name` | String | Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`. |
| `spdx_file` | String | Describes a specific SPDX File. |
| `discovered` | String | Describes when a resource was discovered. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create occurrence
occurrence = provider.containeranalysis_api.Occurrence {
    parent = "value"  # Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrence is to be created.
}

# Access occurrence outputs
occurrence_id = occurrence.id
occurrence_sbom_reference = occurrence.sbom_reference
occurrence_update_time = occurrence.update_time
occurrence_note_name = occurrence.note_name
occurrence_resource = occurrence.resource
occurrence_derived_image = occurrence.derived_image
occurrence_remediation = occurrence.remediation
occurrence_sbom = occurrence.sbom
occurrence_envelope = occurrence.envelope
occurrence_secret = occurrence.secret
occurrence_spdx_package = occurrence.spdx_package
occurrence_spdx_relationship = occurrence.spdx_relationship
occurrence_vulnerability = occurrence.vulnerability
occurrence_deployment = occurrence.deployment
occurrence_attestation = occurrence.attestation
occurrence_build = occurrence.build
occurrence_intoto = occurrence.intoto
occurrence_kind = occurrence.kind
occurrence_create_time = occurrence.create_time
occurrence_installation = occurrence.installation
occurrence_name = occurrence.name
occurrence_spdx_file = occurrence.spdx_file
occurrence_discovered = occurrence.discovered
```

---


### Resource

Generates an SBOM and other dependency information for the given resource.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the resource in the form of `projects/[PROJECT_ID]/resources/[RESOURCE_URL]`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create resource
resource = provider.containeranalysis_api.Resource {
    name = "value"  # Required. The name of the resource in the form of `projects/[PROJECT_ID]/resources/[RESOURCE_URL]`.
}

```

---


### Note

Creates a new note.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Output only. The type of analysis. This field can be used as a filter in list requests. |
| `package` | String |  | A note describing a package hosted by various package managers. |
| `build` | String |  | A note describing build provenance for a verifiable build. |
| `intoto` | String |  | A note describing an in-toto link. |
| `expiration_time` | String |  | Time of expiration for this note. Empty if note does not expire. |
| `deployable` | String |  | A note describing something that can be deployed. |
| `name` | String |  | Output only. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. |
| `short_description` | String |  | A one sentence description of this note. |
| `spdx_file` | String |  | A note describing an SPDX File. |
| `spdx_relationship` | String |  | A note describing an SPDX File. |
| `vulnerability` | String |  | A note describing a package vulnerability. |
| `create_time` | String |  | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `vulnerability_assessment` | String |  | A note describing a vulnerability assessment. |
| `related_note_names` | Vec<String> |  | Other notes related to this note. |
| `sbom_reference` | String |  | A note describing an SBOM reference. |
| `long_description` | String |  | A detailed description of this note. |
| `secret` | String |  | A note describing a secret. |
| `attestation_authority` | String |  | A note describing an attestation role. |
| `spdx_package` | String |  | A note describing an SPDX Package. |
| `related_url` | Vec<String> |  | URLs associated with this note. |
| `sbom` | String |  | A note describing a software bill of materials. |
| `base_image` | String |  | A note describing a base image. |
| `discovery` | String |  | A note describing the initial analysis of a resource. |
| `update_time` | String |  | Output only. The time this note was last updated. This field can be used as a filter in list requests. |
| `parent` | String | ✅ | Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the note is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Output only. The type of analysis. This field can be used as a filter in list requests. |
| `package` | String | A note describing a package hosted by various package managers. |
| `build` | String | A note describing build provenance for a verifiable build. |
| `intoto` | String | A note describing an in-toto link. |
| `expiration_time` | String | Time of expiration for this note. Empty if note does not expire. |
| `deployable` | String | A note describing something that can be deployed. |
| `name` | String | Output only. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. |
| `short_description` | String | A one sentence description of this note. |
| `spdx_file` | String | A note describing an SPDX File. |
| `spdx_relationship` | String | A note describing an SPDX File. |
| `vulnerability` | String | A note describing a package vulnerability. |
| `create_time` | String | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `vulnerability_assessment` | String | A note describing a vulnerability assessment. |
| `related_note_names` | Vec<String> | Other notes related to this note. |
| `sbom_reference` | String | A note describing an SBOM reference. |
| `long_description` | String | A detailed description of this note. |
| `secret` | String | A note describing a secret. |
| `attestation_authority` | String | A note describing an attestation role. |
| `spdx_package` | String | A note describing an SPDX Package. |
| `related_url` | Vec<String> | URLs associated with this note. |
| `sbom` | String | A note describing a software bill of materials. |
| `base_image` | String | A note describing a base image. |
| `discovery` | String | A note describing the initial analysis of a resource. |
| `update_time` | String | Output only. The time this note was last updated. This field can be used as a filter in list requests. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create note
note = provider.containeranalysis_api.Note {
    parent = "value"  # Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the note is to be created.
}

# Access note outputs
note_id = note.id
note_kind = note.kind
note_package = note.package
note_build = note.build
note_intoto = note.intoto
note_expiration_time = note.expiration_time
note_deployable = note.deployable
note_name = note.name
note_short_description = note.short_description
note_spdx_file = note.spdx_file
note_spdx_relationship = note.spdx_relationship
note_vulnerability = note.vulnerability
note_create_time = note.create_time
note_vulnerability_assessment = note.vulnerability_assessment
note_related_note_names = note.related_note_names
note_sbom_reference = note.sbom_reference
note_long_description = note.long_description
note_secret = note.secret
note_attestation_authority = note.attestation_authority
note_spdx_package = note.spdx_package
note_related_url = note.related_url
note_sbom = note.sbom
note_base_image = note.base_image
note_discovery = note.discovery
note_update_time = note.update_time
```

---


### Note

Creates a new `Note`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `spdx_relationship` | String |  | A note describing a relationship between SPDX elements. |
| `update_time` | String |  | Output only. The time this note was last updated. This field can be used as a filter in list requests. |
| `base_image` | String |  | A note describing a base image. |
| `discovery` | String |  | A note describing a provider/analysis type. |
| `name` | String |  | The name of the note in the form "projects/{provider_project_id}/notes/{NOTE_ID}" |
| `create_time` | String |  | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `package` | String |  | A note describing a package hosted by various package managers. |
| `secret` | String |  | A note describing a secret. |
| `attestation_authority` | String |  | A note describing an attestation role. |
| `related_url` | Vec<String> |  | URLs associated with this note |
| `upgrade` | String |  | A note describing an upgrade. |
| `sbom_reference` | String |  | A note describing a reference to an SBOM. |
| `expiration_time` | String |  | Time of expiration for this note, null if note does not expire. |
| `vulnerability_type` | String |  | A package vulnerability type of note. |
| `kind` | String |  | Output only. This explicitly denotes which kind of note is specified. This field can be used as a filter in list requests. |
| `spdx_file` | String |  | A note describing an SPDX File. |
| `long_description` | String |  | A detailed description of this `Note`. |
| `deployable` | String |  | A note describing something that can be deployed. |
| `compliance` | String |  | A note describing a compliance check. |
| `sbom` | String |  | A note describing a software bill of materials. |
| `short_description` | String |  | A one sentence description of this `Note`. |
| `vulnerability_assessment` | String |  | A note describing a vulnerability assessment. |
| `build_type` | String |  | Build provenance type for a verifiable build. |
| `dsse_attestation` | String |  | A note describing a dsse attestation note. |
| `spdx_package` | String |  | A note describing an SPDX Package. |
| `name` | String | ✅ | The name of the project. Should be of the form "providers/{provider_id}". @Deprecated |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `spdx_relationship` | String | A note describing a relationship between SPDX elements. |
| `update_time` | String | Output only. The time this note was last updated. This field can be used as a filter in list requests. |
| `base_image` | String | A note describing a base image. |
| `discovery` | String | A note describing a provider/analysis type. |
| `name` | String | The name of the note in the form "projects/{provider_project_id}/notes/{NOTE_ID}" |
| `create_time` | String | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `package` | String | A note describing a package hosted by various package managers. |
| `secret` | String | A note describing a secret. |
| `attestation_authority` | String | A note describing an attestation role. |
| `related_url` | Vec<String> | URLs associated with this note |
| `upgrade` | String | A note describing an upgrade. |
| `sbom_reference` | String | A note describing a reference to an SBOM. |
| `expiration_time` | String | Time of expiration for this note, null if note does not expire. |
| `vulnerability_type` | String | A package vulnerability type of note. |
| `kind` | String | Output only. This explicitly denotes which kind of note is specified. This field can be used as a filter in list requests. |
| `spdx_file` | String | A note describing an SPDX File. |
| `long_description` | String | A detailed description of this `Note`. |
| `deployable` | String | A note describing something that can be deployed. |
| `compliance` | String | A note describing a compliance check. |
| `sbom` | String | A note describing a software bill of materials. |
| `short_description` | String | A one sentence description of this `Note`. |
| `vulnerability_assessment` | String | A note describing a vulnerability assessment. |
| `build_type` | String | Build provenance type for a verifiable build. |
| `dsse_attestation` | String | A note describing a dsse attestation note. |
| `spdx_package` | String | A note describing an SPDX Package. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create note
note = provider.containeranalysis_api.Note {
    name = "value"  # The name of the project. Should be of the form "providers/{provider_id}". @Deprecated
}

# Access note outputs
note_id = note.id
note_spdx_relationship = note.spdx_relationship
note_update_time = note.update_time
note_base_image = note.base_image
note_discovery = note.discovery
note_name = note.name
note_create_time = note.create_time
note_package = note.package
note_secret = note.secret
note_attestation_authority = note.attestation_authority
note_related_url = note.related_url
note_upgrade = note.upgrade
note_sbom_reference = note.sbom_reference
note_expiration_time = note.expiration_time
note_vulnerability_type = note.vulnerability_type
note_kind = note.kind
note_spdx_file = note.spdx_file
note_long_description = note.long_description
note_deployable = note.deployable
note_compliance = note.compliance
note_sbom = note.sbom
note_short_description = note.short_description
note_vulnerability_assessment = note.vulnerability_assessment
note_build_type = note.build_type
note_dsse_attestation = note.dsse_attestation
note_spdx_package = note.spdx_package
```

---


### Scan_config

Gets a specific scan configuration for a project.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The name of the ScanConfig in the form “projects/{project_id}/scanConfigs/{scan_config_id}". |
| `enabled` | bool |  | Indicates whether the Scan is enabled. |
| `update_time` | String |  | Output only. The time this scan config was last updated. |
| `create_time` | String |  | Output only. The time this scan config was created. |
| `description` | String |  | Output only. A human-readable description of what the `ScanConfig` does. |
| `name` | String | ✅ | The scan config to update of the form projects/{project_id}/scanConfigs/{scan_config_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The name of the ScanConfig in the form “projects/{project_id}/scanConfigs/{scan_config_id}". |
| `enabled` | bool | Indicates whether the Scan is enabled. |
| `update_time` | String | Output only. The time this scan config was last updated. |
| `create_time` | String | Output only. The time this scan config was created. |
| `description` | String | Output only. A human-readable description of what the `ScanConfig` does. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access scan_config outputs
scan_config_id = scan_config.id
scan_config_name = scan_config.name
scan_config_enabled = scan_config.enabled
scan_config_update_time = scan_config.update_time
scan_config_create_time = scan_config.create_time
scan_config_description = scan_config.description
```

---


### Occurrence

Creates a new `Occurrence`. Use this method to create `Occurrences` for a resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Output only. This explicitly denotes which of the `Occurrence` details are specified. This field can be used as a filter in list requests. |
| `name` | String |  | Output only. The name of the `Occurrence` in the form "projects/{project_id}/occurrences/{OCCURRENCE_ID}" |
| `vulnerability_details` | String |  | Details of a security vulnerability note. |
| `installation` | String |  | Describes the installation of a package on the linked resource. |
| `sbom` | String |  | Describes a specific software bill of materials document. |
| `deployment` | String |  | Describes the deployment of an artifact on a runtime. |
| `dsse_attestation` | String |  | This represents a DSSE attestation occurrence |
| `discovered` | String |  | Describes the initial scan status for this resource. |
| `remediation` | String |  | A description of actions that can be taken to remedy the `Note` |
| `resource` | String |  |  The resource for which the `Occurrence` applies. |
| `sbom_reference` | String |  | This represents an SBOM reference occurrence |
| `create_time` | String |  | Output only. The time this `Occurrence` was created. |
| `compliance` | String |  | Describes whether or not a resource passes compliance checks. |
| `note_name` | String |  | An analysis note associated with this image, in the form "providers/{provider_id}/notes/{NOTE_ID}" This field can be used as a filter in list requests. |
| `build_details` | String |  | Build details for a verifiable build. |
| `envelope` | String |  | https://github.com/secure-systems-lab/dsse |
| `derived_image` | String |  | Describes how this resource derives from the basis in the associated note. |
| `resource_url` | String |  | The unique URL of the image or the container for which the `Occurrence` applies. For example, https://gcr.io/project/image@sha256:foo This field can be used as a filter in list requests. |
| `secret` | String |  | This represents a secret occurrence |
| `spdx_file` | String |  | Describes a specific SPDX File. |
| `spdx_package` | String |  | Describes a specific SPDX Package. |
| `attestation` | String |  | Describes an attestation of an artifact. |
| `spdx_relationship` | String |  | Describes a specific relationship between SPDX elements. |
| `update_time` | String |  | Output only. The time this `Occurrence` was last updated. |
| `upgrade` | String |  | Describes an upgrade. |
| `parent` | String | ✅ | This field contains the project Id for example: "projects/{project_id}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Output only. This explicitly denotes which of the `Occurrence` details are specified. This field can be used as a filter in list requests. |
| `name` | String | Output only. The name of the `Occurrence` in the form "projects/{project_id}/occurrences/{OCCURRENCE_ID}" |
| `vulnerability_details` | String | Details of a security vulnerability note. |
| `installation` | String | Describes the installation of a package on the linked resource. |
| `sbom` | String | Describes a specific software bill of materials document. |
| `deployment` | String | Describes the deployment of an artifact on a runtime. |
| `dsse_attestation` | String | This represents a DSSE attestation occurrence |
| `discovered` | String | Describes the initial scan status for this resource. |
| `remediation` | String | A description of actions that can be taken to remedy the `Note` |
| `resource` | String |  The resource for which the `Occurrence` applies. |
| `sbom_reference` | String | This represents an SBOM reference occurrence |
| `create_time` | String | Output only. The time this `Occurrence` was created. |
| `compliance` | String | Describes whether or not a resource passes compliance checks. |
| `note_name` | String | An analysis note associated with this image, in the form "providers/{provider_id}/notes/{NOTE_ID}" This field can be used as a filter in list requests. |
| `build_details` | String | Build details for a verifiable build. |
| `envelope` | String | https://github.com/secure-systems-lab/dsse |
| `derived_image` | String | Describes how this resource derives from the basis in the associated note. |
| `resource_url` | String | The unique URL of the image or the container for which the `Occurrence` applies. For example, https://gcr.io/project/image@sha256:foo This field can be used as a filter in list requests. |
| `secret` | String | This represents a secret occurrence |
| `spdx_file` | String | Describes a specific SPDX File. |
| `spdx_package` | String | Describes a specific SPDX Package. |
| `attestation` | String | Describes an attestation of an artifact. |
| `spdx_relationship` | String | Describes a specific relationship between SPDX elements. |
| `update_time` | String | Output only. The time this `Occurrence` was last updated. |
| `upgrade` | String | Describes an upgrade. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create occurrence
occurrence = provider.containeranalysis_api.Occurrence {
    parent = "value"  # This field contains the project Id for example: "projects/{project_id}"
}

# Access occurrence outputs
occurrence_id = occurrence.id
occurrence_kind = occurrence.kind
occurrence_name = occurrence.name
occurrence_vulnerability_details = occurrence.vulnerability_details
occurrence_installation = occurrence.installation
occurrence_sbom = occurrence.sbom
occurrence_deployment = occurrence.deployment
occurrence_dsse_attestation = occurrence.dsse_attestation
occurrence_discovered = occurrence.discovered
occurrence_remediation = occurrence.remediation
occurrence_resource = occurrence.resource
occurrence_sbom_reference = occurrence.sbom_reference
occurrence_create_time = occurrence.create_time
occurrence_compliance = occurrence.compliance
occurrence_note_name = occurrence.note_name
occurrence_build_details = occurrence.build_details
occurrence_envelope = occurrence.envelope
occurrence_derived_image = occurrence.derived_image
occurrence_resource_url = occurrence.resource_url
occurrence_secret = occurrence.secret
occurrence_spdx_file = occurrence.spdx_file
occurrence_spdx_package = occurrence.spdx_package
occurrence_attestation = occurrence.attestation
occurrence_spdx_relationship = occurrence.spdx_relationship
occurrence_update_time = occurrence.update_time
occurrence_upgrade = occurrence.upgrade
```

---


### Operation

Creates a new `Operation`.

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `operation_id` | String |  | The ID to use for this operation. |
| `operation` | String |  | The operation to create. |
| `parent` | String | ✅ | The project Id that this operation should be created under. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.containeranalysis_api.Operation {
    parent = "value"  # The project Id that this operation should be created under.
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple resource resources
resource_0 = provider.containeranalysis_api.Resource {
    name = "value-0"
}
resource_1 = provider.containeranalysis_api.Resource {
    name = "value-1"
}
resource_2 = provider.containeranalysis_api.Resource {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    resource = provider.containeranalysis_api.Resource {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Containeranalysis_api Documentation](https://cloud.google.com/containeranalysis_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
