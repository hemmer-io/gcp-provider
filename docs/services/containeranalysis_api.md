# Containeranalysis_api Service



**Resources**: 10

---

## Overview

The containeranalysis_api service provides access to 10 resource types:

- [Note](#note) [CRUD]
- [Resource](#resource) [C]
- [Occurrence](#occurrence) [CRUD]
- [Resource](#resource) [C]
- [Note](#note) [CRUD]
- [Occurrence](#occurrence) [CRUD]
- [Operation](#operation) [CU]
- [Occurrence](#occurrence) [CRUD]
- [Scan_config](#scan_config) [RU]
- [Note](#note) [CRUD]

---

## Resources


### Note

Creates a new note.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `related_note_names` | Vec<String> |  | Other notes related to this note. |
| `dsse_attestation` | String |  | A note describing a dsse attestation note. |
| `package` | String |  | A note describing a package hosted by various package managers. |
| `kind` | String |  | Output only. The type of analysis. This field can be used as a filter in list requests. |
| `create_time` | String |  | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `related_url` | Vec<String> |  | URLs associated with this note. |
| `secret` | String |  | A note describing a secret. |
| `expiration_time` | String |  | Time of expiration for this note. Empty if note does not expire. |
| `discovery` | String |  | A note describing the initial analysis of a resource. |
| `deployment` | String |  | A note describing something that can be deployed. |
| `vulnerability` | String |  | A note describing a package vulnerability. |
| `compliance` | String |  | A note describing a compliance check. |
| `vulnerability_assessment` | String |  | A note describing a vulnerability assessment. |
| `upgrade` | String |  | A note describing available package upgrades. |
| `name` | String |  | Output only. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. |
| `long_description` | String |  | A detailed description of this note. |
| `sbom_reference` | String |  | A note describing an SBOM reference. |
| `short_description` | String |  | A one sentence description of this note. |
| `image` | String |  | A note describing a base image. |
| `build` | String |  | A note describing build provenance for a verifiable build. |
| `update_time` | String |  | Output only. The time this note was last updated. This field can be used as a filter in list requests. |
| `attestation` | String |  | A note describing an attestation role. |
| `parent` | String | ✅ | Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the note is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `related_note_names` | Vec<String> | Other notes related to this note. |
| `dsse_attestation` | String | A note describing a dsse attestation note. |
| `package` | String | A note describing a package hosted by various package managers. |
| `kind` | String | Output only. The type of analysis. This field can be used as a filter in list requests. |
| `create_time` | String | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `related_url` | Vec<String> | URLs associated with this note. |
| `secret` | String | A note describing a secret. |
| `expiration_time` | String | Time of expiration for this note. Empty if note does not expire. |
| `discovery` | String | A note describing the initial analysis of a resource. |
| `deployment` | String | A note describing something that can be deployed. |
| `vulnerability` | String | A note describing a package vulnerability. |
| `compliance` | String | A note describing a compliance check. |
| `vulnerability_assessment` | String | A note describing a vulnerability assessment. |
| `upgrade` | String | A note describing available package upgrades. |
| `name` | String | Output only. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. |
| `long_description` | String | A detailed description of this note. |
| `sbom_reference` | String | A note describing an SBOM reference. |
| `short_description` | String | A one sentence description of this note. |
| `image` | String | A note describing a base image. |
| `build` | String | A note describing build provenance for a verifiable build. |
| `update_time` | String | Output only. The time this note was last updated. This field can be used as a filter in list requests. |
| `attestation` | String | A note describing an attestation role. |


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
note_related_note_names = note.related_note_names
note_dsse_attestation = note.dsse_attestation
note_package = note.package
note_kind = note.kind
note_create_time = note.create_time
note_related_url = note.related_url
note_secret = note.secret
note_expiration_time = note.expiration_time
note_discovery = note.discovery
note_deployment = note.deployment
note_vulnerability = note.vulnerability
note_compliance = note.compliance
note_vulnerability_assessment = note.vulnerability_assessment
note_upgrade = note.upgrade
note_name = note.name
note_long_description = note.long_description
note_sbom_reference = note.sbom_reference
note_short_description = note.short_description
note_image = note.image
note_build = note.build
note_update_time = note.update_time
note_attestation = note.attestation
```

---


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


### Occurrence

Creates a new occurrence.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `build` | String |  | Describes a verifiable build. |
| `image` | String |  | Describes how this resource derives from the basis in the associated note. |
| `envelope` | String |  | https://github.com/secure-systems-lab/dsse |
| `upgrade` | String |  | Describes an available package upgrade on the linked resource. |
| `update_time` | String |  | Output only. The time this occurrence was last updated. |
| `kind` | String |  | Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests. |
| `remediation` | String |  | A description of actions that can be taken to remedy the note. |
| `resource_uri` | String |  | Required. Immutable. A URI that represents the resource for which the occurrence applies. For example, `https://gcr.io/project/image@sha256:123abc` for a Docker image. |
| `compliance` | String |  | Describes a compliance violation on a linked resource. |
| `discovery` | String |  | Describes when a resource was discovered. |
| `vulnerability` | String |  | Describes a security vulnerability. |
| `name` | String |  | Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`. |
| `secret` | String |  | Describes a secret. |
| `dsse_attestation` | String |  | Describes an attestation of an artifact using dsse. |
| `deployment` | String |  | Describes the deployment of an artifact on a runtime. |
| `attestation` | String |  | Describes an attestation of an artifact. |
| `note_name` | String |  | Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests. |
| `package` | String |  | Describes the installation of a package on the linked resource. |
| `sbom_reference` | String |  | Describes a specific SBOM reference occurrences. |
| `create_time` | String |  | Output only. The time this occurrence was created. |
| `parent` | String | ✅ | Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrence is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `build` | String | Describes a verifiable build. |
| `image` | String | Describes how this resource derives from the basis in the associated note. |
| `envelope` | String | https://github.com/secure-systems-lab/dsse |
| `upgrade` | String | Describes an available package upgrade on the linked resource. |
| `update_time` | String | Output only. The time this occurrence was last updated. |
| `kind` | String | Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests. |
| `remediation` | String | A description of actions that can be taken to remedy the note. |
| `resource_uri` | String | Required. Immutable. A URI that represents the resource for which the occurrence applies. For example, `https://gcr.io/project/image@sha256:123abc` for a Docker image. |
| `compliance` | String | Describes a compliance violation on a linked resource. |
| `discovery` | String | Describes when a resource was discovered. |
| `vulnerability` | String | Describes a security vulnerability. |
| `name` | String | Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`. |
| `secret` | String | Describes a secret. |
| `dsse_attestation` | String | Describes an attestation of an artifact using dsse. |
| `deployment` | String | Describes the deployment of an artifact on a runtime. |
| `attestation` | String | Describes an attestation of an artifact. |
| `note_name` | String | Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests. |
| `package` | String | Describes the installation of a package on the linked resource. |
| `sbom_reference` | String | Describes a specific SBOM reference occurrences. |
| `create_time` | String | Output only. The time this occurrence was created. |


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
occurrence_build = occurrence.build
occurrence_image = occurrence.image
occurrence_envelope = occurrence.envelope
occurrence_upgrade = occurrence.upgrade
occurrence_update_time = occurrence.update_time
occurrence_kind = occurrence.kind
occurrence_remediation = occurrence.remediation
occurrence_resource_uri = occurrence.resource_uri
occurrence_compliance = occurrence.compliance
occurrence_discovery = occurrence.discovery
occurrence_vulnerability = occurrence.vulnerability
occurrence_name = occurrence.name
occurrence_secret = occurrence.secret
occurrence_dsse_attestation = occurrence.dsse_attestation
occurrence_deployment = occurrence.deployment
occurrence_attestation = occurrence.attestation
occurrence_note_name = occurrence.note_name
occurrence_package = occurrence.package
occurrence_sbom_reference = occurrence.sbom_reference
occurrence_create_time = occurrence.create_time
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
| `related_url` | Vec<String> |  | URLs associated with this note. |
| `sbom` | String |  | A note describing a software bill of materials. |
| `spdx_package` | String |  | A note describing an SPDX Package. |
| `attestation_authority` | String |  | A note describing an attestation role. |
| `discovery` | String |  | A note describing the initial analysis of a resource. |
| `expiration_time` | String |  | Time of expiration for this note. Empty if note does not expire. |
| `sbom_reference` | String |  | A note describing an SBOM reference. |
| `spdx_file` | String |  | A note describing an SPDX File. |
| `build` | String |  | A note describing build provenance for a verifiable build. |
| `secret` | String |  | A note describing a secret. |
| `long_description` | String |  | A detailed description of this note. |
| `name` | String |  | Output only. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. |
| `update_time` | String |  | Output only. The time this note was last updated. This field can be used as a filter in list requests. |
| `vulnerability` | String |  | A note describing a package vulnerability. |
| `create_time` | String |  | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `spdx_relationship` | String |  | A note describing an SPDX File. |
| `deployable` | String |  | A note describing something that can be deployed. |
| `related_note_names` | Vec<String> |  | Other notes related to this note. |
| `base_image` | String |  | A note describing a base image. |
| `short_description` | String |  | A one sentence description of this note. |
| `kind` | String |  | Output only. The type of analysis. This field can be used as a filter in list requests. |
| `vulnerability_assessment` | String |  | A note describing a vulnerability assessment. |
| `intoto` | String |  | A note describing an in-toto link. |
| `package` | String |  | A note describing a package hosted by various package managers. |
| `parent` | String | ✅ | Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the note is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `related_url` | Vec<String> | URLs associated with this note. |
| `sbom` | String | A note describing a software bill of materials. |
| `spdx_package` | String | A note describing an SPDX Package. |
| `attestation_authority` | String | A note describing an attestation role. |
| `discovery` | String | A note describing the initial analysis of a resource. |
| `expiration_time` | String | Time of expiration for this note. Empty if note does not expire. |
| `sbom_reference` | String | A note describing an SBOM reference. |
| `spdx_file` | String | A note describing an SPDX File. |
| `build` | String | A note describing build provenance for a verifiable build. |
| `secret` | String | A note describing a secret. |
| `long_description` | String | A detailed description of this note. |
| `name` | String | Output only. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. |
| `update_time` | String | Output only. The time this note was last updated. This field can be used as a filter in list requests. |
| `vulnerability` | String | A note describing a package vulnerability. |
| `create_time` | String | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `spdx_relationship` | String | A note describing an SPDX File. |
| `deployable` | String | A note describing something that can be deployed. |
| `related_note_names` | Vec<String> | Other notes related to this note. |
| `base_image` | String | A note describing a base image. |
| `short_description` | String | A one sentence description of this note. |
| `kind` | String | Output only. The type of analysis. This field can be used as a filter in list requests. |
| `vulnerability_assessment` | String | A note describing a vulnerability assessment. |
| `intoto` | String | A note describing an in-toto link. |
| `package` | String | A note describing a package hosted by various package managers. |


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
note_related_url = note.related_url
note_sbom = note.sbom
note_spdx_package = note.spdx_package
note_attestation_authority = note.attestation_authority
note_discovery = note.discovery
note_expiration_time = note.expiration_time
note_sbom_reference = note.sbom_reference
note_spdx_file = note.spdx_file
note_build = note.build
note_secret = note.secret
note_long_description = note.long_description
note_name = note.name
note_update_time = note.update_time
note_vulnerability = note.vulnerability
note_create_time = note.create_time
note_spdx_relationship = note.spdx_relationship
note_deployable = note.deployable
note_related_note_names = note.related_note_names
note_base_image = note.base_image
note_short_description = note.short_description
note_kind = note.kind
note_vulnerability_assessment = note.vulnerability_assessment
note_intoto = note.intoto
note_package = note.package
```

---


### Occurrence

Creates a new occurrence.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `installation` | String |  | Describes the installation of a package on the linked resource. |
| `sbom_reference` | String |  | Describes a specific SBOM reference occurrences. |
| `sbom` | String |  | Describes a specific software bill of materials document. |
| `spdx_file` | String |  | Describes a specific SPDX File. |
| `intoto` | String |  | Describes a specific in-toto link. |
| `note_name` | String |  | Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests. |
| `remediation` | String |  | A description of actions that can be taken to remedy the note. |
| `vulnerability` | String |  | Describes a security vulnerability. |
| `update_time` | String |  | Output only. The time this occurrence was last updated. |
| `kind` | String |  | Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests. |
| `name` | String |  | Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`. |
| `deployment` | String |  | Describes the deployment of an artifact on a runtime. |
| `build` | String |  | Describes a verifiable build. |
| `attestation` | String |  | Describes an attestation of an artifact. |
| `resource` | String |  | Required. Immutable. The resource for which the occurrence applies. |
| `spdx_relationship` | String |  | Describes a specific SPDX Relationship. |
| `discovered` | String |  | Describes when a resource was discovered. |
| `spdx_package` | String |  | Describes a specific SPDX Package. |
| `create_time` | String |  | Output only. The time this occurrence was created. |
| `derived_image` | String |  | Describes how this resource derives from the basis in the associated note. |
| `envelope` | String |  | https://github.com/secure-systems-lab/dsse |
| `secret` | String |  | Describes a secret. |
| `parent` | String | ✅ | Required. The name of the project in the form of `projects/[PROJECT_ID]`, under which the occurrence is to be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `installation` | String | Describes the installation of a package on the linked resource. |
| `sbom_reference` | String | Describes a specific SBOM reference occurrences. |
| `sbom` | String | Describes a specific software bill of materials document. |
| `spdx_file` | String | Describes a specific SPDX File. |
| `intoto` | String | Describes a specific in-toto link. |
| `note_name` | String | Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests. |
| `remediation` | String | A description of actions that can be taken to remedy the note. |
| `vulnerability` | String | Describes a security vulnerability. |
| `update_time` | String | Output only. The time this occurrence was last updated. |
| `kind` | String | Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests. |
| `name` | String | Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`. |
| `deployment` | String | Describes the deployment of an artifact on a runtime. |
| `build` | String | Describes a verifiable build. |
| `attestation` | String | Describes an attestation of an artifact. |
| `resource` | String | Required. Immutable. The resource for which the occurrence applies. |
| `spdx_relationship` | String | Describes a specific SPDX Relationship. |
| `discovered` | String | Describes when a resource was discovered. |
| `spdx_package` | String | Describes a specific SPDX Package. |
| `create_time` | String | Output only. The time this occurrence was created. |
| `derived_image` | String | Describes how this resource derives from the basis in the associated note. |
| `envelope` | String | https://github.com/secure-systems-lab/dsse |
| `secret` | String | Describes a secret. |


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
occurrence_installation = occurrence.installation
occurrence_sbom_reference = occurrence.sbom_reference
occurrence_sbom = occurrence.sbom
occurrence_spdx_file = occurrence.spdx_file
occurrence_intoto = occurrence.intoto
occurrence_note_name = occurrence.note_name
occurrence_remediation = occurrence.remediation
occurrence_vulnerability = occurrence.vulnerability
occurrence_update_time = occurrence.update_time
occurrence_kind = occurrence.kind
occurrence_name = occurrence.name
occurrence_deployment = occurrence.deployment
occurrence_build = occurrence.build
occurrence_attestation = occurrence.attestation
occurrence_resource = occurrence.resource
occurrence_spdx_relationship = occurrence.spdx_relationship
occurrence_discovered = occurrence.discovered
occurrence_spdx_package = occurrence.spdx_package
occurrence_create_time = occurrence.create_time
occurrence_derived_image = occurrence.derived_image
occurrence_envelope = occurrence.envelope
occurrence_secret = occurrence.secret
```

---


### Operation

Creates a new `Operation`.

**Operations**: ✅ Create ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `operation` | String |  | The operation to create. |
| `operation_id` | String |  | The ID to use for this operation. |
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


### Occurrence

Creates a new `Occurrence`. Use this method to create `Occurrences` for a resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dsse_attestation` | String |  | This represents a DSSE attestation occurrence |
| `installation` | String |  | Describes the installation of a package on the linked resource. |
| `spdx_file` | String |  | Describes a specific SPDX File. |
| `kind` | String |  | Output only. This explicitly denotes which of the `Occurrence` details are specified. This field can be used as a filter in list requests. |
| `resource` | String |  |  The resource for which the `Occurrence` applies. |
| `note_name` | String |  | An analysis note associated with this image, in the form "providers/{provider_id}/notes/{NOTE_ID}" This field can be used as a filter in list requests. |
| `discovered` | String |  | Describes the initial scan status for this resource. |
| `create_time` | String |  | Output only. The time this `Occurrence` was created. |
| `deployment` | String |  | Describes the deployment of an artifact on a runtime. |
| `remediation` | String |  | A description of actions that can be taken to remedy the `Note` |
| `sbom` | String |  | Describes a specific software bill of materials document. |
| `spdx_package` | String |  | Describes a specific SPDX Package. |
| `compliance` | String |  | Describes whether or not a resource passes compliance checks. |
| `build_details` | String |  | Build details for a verifiable build. |
| `secret` | String |  | This represents a secret occurrence |
| `derived_image` | String |  | Describes how this resource derives from the basis in the associated note. |
| `resource_url` | String |  | The unique URL of the image or the container for which the `Occurrence` applies. For example, https://gcr.io/project/image@sha256:foo This field can be used as a filter in list requests. |
| `sbom_reference` | String |  | This represents an SBOM reference occurrence |
| `name` | String |  | Output only. The name of the `Occurrence` in the form "projects/{project_id}/occurrences/{OCCURRENCE_ID}" |
| `spdx_relationship` | String |  | Describes a specific relationship between SPDX elements. |
| `vulnerability_details` | String |  | Details of a security vulnerability note. |
| `attestation` | String |  | Describes an attestation of an artifact. |
| `update_time` | String |  | Output only. The time this `Occurrence` was last updated. |
| `upgrade` | String |  | Describes an upgrade. |
| `envelope` | String |  | https://github.com/secure-systems-lab/dsse |
| `parent` | String | ✅ | This field contains the project Id for example: "projects/{project_id}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dsse_attestation` | String | This represents a DSSE attestation occurrence |
| `installation` | String | Describes the installation of a package on the linked resource. |
| `spdx_file` | String | Describes a specific SPDX File. |
| `kind` | String | Output only. This explicitly denotes which of the `Occurrence` details are specified. This field can be used as a filter in list requests. |
| `resource` | String |  The resource for which the `Occurrence` applies. |
| `note_name` | String | An analysis note associated with this image, in the form "providers/{provider_id}/notes/{NOTE_ID}" This field can be used as a filter in list requests. |
| `discovered` | String | Describes the initial scan status for this resource. |
| `create_time` | String | Output only. The time this `Occurrence` was created. |
| `deployment` | String | Describes the deployment of an artifact on a runtime. |
| `remediation` | String | A description of actions that can be taken to remedy the `Note` |
| `sbom` | String | Describes a specific software bill of materials document. |
| `spdx_package` | String | Describes a specific SPDX Package. |
| `compliance` | String | Describes whether or not a resource passes compliance checks. |
| `build_details` | String | Build details for a verifiable build. |
| `secret` | String | This represents a secret occurrence |
| `derived_image` | String | Describes how this resource derives from the basis in the associated note. |
| `resource_url` | String | The unique URL of the image or the container for which the `Occurrence` applies. For example, https://gcr.io/project/image@sha256:foo This field can be used as a filter in list requests. |
| `sbom_reference` | String | This represents an SBOM reference occurrence |
| `name` | String | Output only. The name of the `Occurrence` in the form "projects/{project_id}/occurrences/{OCCURRENCE_ID}" |
| `spdx_relationship` | String | Describes a specific relationship between SPDX elements. |
| `vulnerability_details` | String | Details of a security vulnerability note. |
| `attestation` | String | Describes an attestation of an artifact. |
| `update_time` | String | Output only. The time this `Occurrence` was last updated. |
| `upgrade` | String | Describes an upgrade. |
| `envelope` | String | https://github.com/secure-systems-lab/dsse |


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
occurrence_dsse_attestation = occurrence.dsse_attestation
occurrence_installation = occurrence.installation
occurrence_spdx_file = occurrence.spdx_file
occurrence_kind = occurrence.kind
occurrence_resource = occurrence.resource
occurrence_note_name = occurrence.note_name
occurrence_discovered = occurrence.discovered
occurrence_create_time = occurrence.create_time
occurrence_deployment = occurrence.deployment
occurrence_remediation = occurrence.remediation
occurrence_sbom = occurrence.sbom
occurrence_spdx_package = occurrence.spdx_package
occurrence_compliance = occurrence.compliance
occurrence_build_details = occurrence.build_details
occurrence_secret = occurrence.secret
occurrence_derived_image = occurrence.derived_image
occurrence_resource_url = occurrence.resource_url
occurrence_sbom_reference = occurrence.sbom_reference
occurrence_name = occurrence.name
occurrence_spdx_relationship = occurrence.spdx_relationship
occurrence_vulnerability_details = occurrence.vulnerability_details
occurrence_attestation = occurrence.attestation
occurrence_update_time = occurrence.update_time
occurrence_upgrade = occurrence.upgrade
occurrence_envelope = occurrence.envelope
```

---


### Scan_config

Gets a specific scan configuration for a project.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enabled` | bool |  | Indicates whether the Scan is enabled. |
| `name` | String |  | Output only. The name of the ScanConfig in the form “projects/{project_id}/scanConfigs/{scan_config_id}". |
| `update_time` | String |  | Output only. The time this scan config was last updated. |
| `description` | String |  | Output only. A human-readable description of what the `ScanConfig` does. |
| `create_time` | String |  | Output only. The time this scan config was created. |
| `name` | String | ✅ | The scan config to update of the form projects/{project_id}/scanConfigs/{scan_config_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `enabled` | bool | Indicates whether the Scan is enabled. |
| `name` | String | Output only. The name of the ScanConfig in the form “projects/{project_id}/scanConfigs/{scan_config_id}". |
| `update_time` | String | Output only. The time this scan config was last updated. |
| `description` | String | Output only. A human-readable description of what the `ScanConfig` does. |
| `create_time` | String | Output only. The time this scan config was created. |


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
scan_config_enabled = scan_config.enabled
scan_config_name = scan_config.name
scan_config_update_time = scan_config.update_time
scan_config_description = scan_config.description
scan_config_create_time = scan_config.create_time
```

---


### Note

Creates a new `Note`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the note in the form "projects/{provider_project_id}/notes/{NOTE_ID}" |
| `kind` | String |  | Output only. This explicitly denotes which kind of note is specified. This field can be used as a filter in list requests. |
| `upgrade` | String |  | A note describing an upgrade. |
| `compliance` | String |  | A note describing a compliance check. |
| `deployable` | String |  | A note describing something that can be deployed. |
| `dsse_attestation` | String |  | A note describing a dsse attestation note. |
| `create_time` | String |  | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `secret` | String |  | A note describing a secret. |
| `package` | String |  | A note describing a package hosted by various package managers. |
| `sbom` | String |  | A note describing a software bill of materials. |
| `attestation_authority` | String |  | A note describing an attestation role. |
| `short_description` | String |  | A one sentence description of this `Note`. |
| `sbom_reference` | String |  | A note describing a reference to an SBOM. |
| `spdx_file` | String |  | A note describing an SPDX File. |
| `related_url` | Vec<String> |  | URLs associated with this note |
| `base_image` | String |  | A note describing a base image. |
| `update_time` | String |  | Output only. The time this note was last updated. This field can be used as a filter in list requests. |
| `build_type` | String |  | Build provenance type for a verifiable build. |
| `discovery` | String |  | A note describing a provider/analysis type. |
| `expiration_time` | String |  | Time of expiration for this note, null if note does not expire. |
| `vulnerability_type` | String |  | A package vulnerability type of note. |
| `spdx_package` | String |  | A note describing an SPDX Package. |
| `vulnerability_assessment` | String |  | A note describing a vulnerability assessment. |
| `long_description` | String |  | A detailed description of this `Note`. |
| `spdx_relationship` | String |  | A note describing a relationship between SPDX elements. |
| `parent` | String | ✅ | This field contains the project Id for example: "projects/{project_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the note in the form "projects/{provider_project_id}/notes/{NOTE_ID}" |
| `kind` | String | Output only. This explicitly denotes which kind of note is specified. This field can be used as a filter in list requests. |
| `upgrade` | String | A note describing an upgrade. |
| `compliance` | String | A note describing a compliance check. |
| `deployable` | String | A note describing something that can be deployed. |
| `dsse_attestation` | String | A note describing a dsse attestation note. |
| `create_time` | String | Output only. The time this note was created. This field can be used as a filter in list requests. |
| `secret` | String | A note describing a secret. |
| `package` | String | A note describing a package hosted by various package managers. |
| `sbom` | String | A note describing a software bill of materials. |
| `attestation_authority` | String | A note describing an attestation role. |
| `short_description` | String | A one sentence description of this `Note`. |
| `sbom_reference` | String | A note describing a reference to an SBOM. |
| `spdx_file` | String | A note describing an SPDX File. |
| `related_url` | Vec<String> | URLs associated with this note |
| `base_image` | String | A note describing a base image. |
| `update_time` | String | Output only. The time this note was last updated. This field can be used as a filter in list requests. |
| `build_type` | String | Build provenance type for a verifiable build. |
| `discovery` | String | A note describing a provider/analysis type. |
| `expiration_time` | String | Time of expiration for this note, null if note does not expire. |
| `vulnerability_type` | String | A package vulnerability type of note. |
| `spdx_package` | String | A note describing an SPDX Package. |
| `vulnerability_assessment` | String | A note describing a vulnerability assessment. |
| `long_description` | String | A detailed description of this `Note`. |
| `spdx_relationship` | String | A note describing a relationship between SPDX elements. |


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
    parent = "value"  # This field contains the project Id for example: "projects/{project_id}
}

# Access note outputs
note_id = note.id
note_name = note.name
note_kind = note.kind
note_upgrade = note.upgrade
note_compliance = note.compliance
note_deployable = note.deployable
note_dsse_attestation = note.dsse_attestation
note_create_time = note.create_time
note_secret = note.secret
note_package = note.package
note_sbom = note.sbom
note_attestation_authority = note.attestation_authority
note_short_description = note.short_description
note_sbom_reference = note.sbom_reference
note_spdx_file = note.spdx_file
note_related_url = note.related_url
note_base_image = note.base_image
note_update_time = note.update_time
note_build_type = note.build_type
note_discovery = note.discovery
note_expiration_time = note.expiration_time
note_vulnerability_type = note.vulnerability_type
note_spdx_package = note.spdx_package
note_vulnerability_assessment = note.vulnerability_assessment
note_long_description = note.long_description
note_spdx_relationship = note.spdx_relationship
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple note resources
note_0 = provider.containeranalysis_api.Note {
    parent = "value-0"
}
note_1 = provider.containeranalysis_api.Note {
    parent = "value-1"
}
note_2 = provider.containeranalysis_api.Note {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    note = provider.containeranalysis_api.Note {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Containeranalysis_api Documentation](https://cloud.google.com/containeranalysis_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
