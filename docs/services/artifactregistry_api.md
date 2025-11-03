# Artifactregistry_api Service



**Resources**: 37

---

## Overview

The artifactregistry_api service provides access to 37 resource types:

- [Yum_artifact](#yum_artifact) [C]
- [Package](#package) [RUD]
- [Version](#version) [CRUD]
- [Googet_artifact](#googet_artifact) [C]
- [Generic_artifact](#generic_artifact) [C]
- [Kfp_artifact](#kfp_artifact) [C]
- [Rule](#rule) [CRUD]
- [Docker_image](#docker_image) [R]
- [Npm_package](#npm_package) [R]
- [Operation](#operation) [R]
- [Attachment](#attachment) [CRD]
- [Project](#project) [RU]
- [Repositorie](#repositorie) [CRUD]
- [Location](#location) [RU]
- [Python_package](#python_package) [R]
- [Maven_artifact](#maven_artifact) [R]
- [Apt_artifact](#apt_artifact) [C]
- [Tag](#tag) [CRUD]
- [Go_module](#go_module) [C]
- [File](#file) [CRUD]
- [File](#file) [R]
- [Operation](#operation) [R]
- [Location](#location) [R]
- [Apt_artifact](#apt_artifact) [C]
- [Package](#package) [RUD]
- [Project](#project) [RU]
- [Version](#version) [RD]
- [Tag](#tag) [CRUD]
- [Yum_artifact](#yum_artifact) [C]
- [Repositorie](#repositorie) [CRUD]
- [File](#file) [R]
- [Operation](#operation) [R]
- [Repositorie](#repositorie) [CRUD]
- [Version](#version) [RD]
- [Tag](#tag) [CRUD]
- [Package](#package) [RD]
- [Location](#location) [R]

---

## Resources


### Yum_artifact

Directly uploads a Yum artifact. The returned Operation will complete once the resources are uploaded. Package, Version, and File resources are created based on the imported artifact. Imported artifacts that conflict with existing resources are ignored.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | The name of the parent resource where the artifacts will be uploaded. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create yum_artifact
yum_artifact = provider.artifactregistry_api.Yum_artifact {
    parent = "value"  # The name of the parent resource where the artifacts will be uploaded.
}

```

---


### Package

Gets a package.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | Optional. Client specified annotations. |
| `create_time` | String |  | The time when the package was created. |
| `name` | String |  | The name of the package, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1`. If the package ID part contains slashes, the slashes are escaped. |
| `update_time` | String |  | The time when the package was last updated. This includes publishing a new version of the package. |
| `display_name` | String |  | The display name of the package. |
| `name` | String | ✅ | The name of the package, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1`. If the package ID part contains slashes, the slashes are escaped. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Optional. Client specified annotations. |
| `create_time` | String | The time when the package was created. |
| `name` | String | The name of the package, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1`. If the package ID part contains slashes, the slashes are escaped. |
| `update_time` | String | The time when the package was last updated. This includes publishing a new version of the package. |
| `display_name` | String | The display name of the package. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access package outputs
package_id = package.id
package_annotations = package.annotations
package_create_time = package.create_time
package_name = package.name
package_update_time = package.update_time
package_display_name = package.display_name
```

---


### Version

Deletes multiple versions across a repository. The returned operation will complete once the versions have been deleted.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `names` | Vec<String> |  | Required. The names of the versions to delete. The maximum number of versions deleted per batch is determined by the service and is dependent on the available resources in the region. |
| `validate_only` | bool |  | If true, the request is performed without deleting data, following AIP-163. |
| `parent` | String | ✅ | The name of the repository holding all requested versions. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | The time when the version was last updated. |
| `annotations` | HashMap<String, String> | Optional. Client specified annotations. |
| `related_tags` | Vec<String> | Output only. A list of related tags. Will contain up to 100 tags that reference this version. |
| `description` | String | Optional. Description of the version, as specified in its metadata. |
| `name` | String | The name of the version, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/art1`. If the package or version ID parts contain slashes, the slashes are escaped. |
| `create_time` | String | The time when the version was created. |
| `metadata` | HashMap<String, String> | Output only. Repository-specific Metadata stored against this version. The fields returned are defined by the underlying repository-specific resource. Currently, the resources could be: DockerImage MavenArtifact |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.artifactregistry_api.Version {
    parent = "value"  # The name of the repository holding all requested versions.
}

# Access version outputs
version_id = version.id
version_update_time = version.update_time
version_annotations = version.annotations
version_related_tags = version.related_tags
version_description = version.description
version_name = version.name
version_create_time = version.create_time
version_metadata = version.metadata
```

---


### Googet_artifact

Imports GooGet artifacts. The returned Operation will complete once the resources are imported. Package, Version, and File resources are created based on the imported artifacts. Imported artifacts that conflict with existing resources are ignored.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_source` | String |  | Google Cloud Storage location where input content is located. |
| `parent` | String | ✅ | The name of the parent resource where the artifacts will be imported. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create googet_artifact
googet_artifact = provider.artifactregistry_api.Googet_artifact {
    parent = "value"  # The name of the parent resource where the artifacts will be imported.
}

```

---


### Generic_artifact

Directly uploads a Generic artifact. The returned operation will complete once the resources are uploaded. Package, version, and file resources are created based on the uploaded artifact. Uploaded artifacts that conflict with existing resources will raise an `ALREADY_EXISTS` error.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filename` | String |  | The name of the file of the generic artifact to be uploaded. E.g. `example-file.zip` The filename is limited to letters, numbers, and url safe characters, i.e. [a-zA-Z0-9-_.~@]. |
| `version_id` | String |  | The ID of the version of the generic artifact. If the version does not exist, a new version will be created. The version_id must start and end with a letter or number, can only contain lowercase letters, numbers, the following characters [-.+~:], i.e.[a-z0-9-.+~:] and cannot exceed a total of 128 characters. Creating a version called `latest` is not allowed. |
| `package_id` | String |  | The ID of the package of the generic artifact. If the package does not exist, a new package will be created. The `package_id` should start and end with a letter or number, only contain letters, numbers, hyphens, underscores, and periods, and not exceed 256 characters. |
| `parent` | String | ✅ | The resource name of the repository where the generic artifact will be uploaded. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create generic_artifact
generic_artifact = provider.artifactregistry_api.Generic_artifact {
    parent = "value"  # The resource name of the repository where the generic artifact will be uploaded.
}

```

---


### Kfp_artifact

Directly uploads a KFP artifact. The returned Operation will complete once the resource is uploaded. Package, Version, and File resources will be created based on the uploaded artifact. Uploaded artifacts that conflict with existing resources will be overwritten.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Description of the package version. |
| `tags` | Vec<String> |  | Tags to be created with the version. |
| `parent` | String | ✅ | The resource name of the repository where the KFP artifact will be uploaded. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create kfp_artifact
kfp_artifact = provider.artifactregistry_api.Kfp_artifact {
    parent = "value"  # The resource name of the repository where the KFP artifact will be uploaded.
}

```

---


### Rule

Creates a rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `package_id` | String |  | The package ID the rule applies to. If empty, this rule applies to all packages inside the repository. |
| `name` | String |  | The name of the rule, for example: `projects/p1/locations/us-central1/repositories/repo1/rules/rule1`. |
| `condition` | String |  | Optional. A CEL expression for conditions that must be met in order for the rule to apply. If not provided, the rule matches all objects. |
| `action` | String |  | The action this rule takes. |
| `operation` | String |  |  |
| `parent` | String | ✅ | Required. The name of the parent resource where the rule will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `package_id` | String | The package ID the rule applies to. If empty, this rule applies to all packages inside the repository. |
| `name` | String | The name of the rule, for example: `projects/p1/locations/us-central1/repositories/repo1/rules/rule1`. |
| `condition` | String | Optional. A CEL expression for conditions that must be met in order for the rule to apply. If not provided, the rule matches all objects. |
| `action` | String | The action this rule takes. |
| `operation` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rule
rule = provider.artifactregistry_api.Rule {
    parent = "value"  # Required. The name of the parent resource where the rule will be created.
}

# Access rule outputs
rule_id = rule.id
rule_package_id = rule.package_id
rule_name = rule.name
rule_condition = rule.condition
rule_action = rule.action
rule_operation = rule.operation
```

---


### Docker_image

Gets a docker image.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `media_type` | String | Media type of this image, e.g. "application/vnd.docker.distribution.manifest.v2+json". This field is returned as the 'metadata.mediaType' field in the Version resource. |
| `image_size_bytes` | String | Calculated size of the image. This field is returned as the 'metadata.imageSizeBytes' field in the Version resource. |
| `update_time` | String | Output only. The time when the docker image was last updated. |
| `upload_time` | String | Time the image was uploaded. |
| `tags` | Vec<String> | Tags attached to this image. |
| `build_time` | String | The time this image was built. This field is returned as the 'metadata.buildTime' field in the Version resource. The build time is returned to the client as an RFC 3339 string, which can be easily used with the JavaScript Date constructor. |
| `uri` | String | Required. URL to access the image. Example: us-west4-docker.pkg.dev/test-project/test-repo/nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf |
| `name` | String | Required. registry_location, project_id, repository_name and image id forms a unique image name:`projects//locations//repositories//dockerImages/`. For example, "projects/test-project/locations/us-west4/repositories/test-repo/dockerImages/ nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf", where "us-west4" is the registry_location, "test-project" is the project_id, "test-repo" is the repository_name and "nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf" is the image's digest. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access docker_image outputs
docker_image_id = docker_image.id
docker_image_media_type = docker_image.media_type
docker_image_image_size_bytes = docker_image.image_size_bytes
docker_image_update_time = docker_image.update_time
docker_image_upload_time = docker_image.upload_time
docker_image_tags = docker_image.tags
docker_image_build_time = docker_image.build_time
docker_image_uri = docker_image.uri
docker_image_name = docker_image.name
```

---


### Npm_package

Gets a npm package.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. registry_location, project_id, repository_name and npm_package forms a unique package For example, "projects/test-project/locations/us-west4/repositories/test-repo/npmPackages/ npm_test:1.0.0", where "us-west4" is the registry_location, "test-project" is the project_id, "test-repo" is the repository_name and npm_test:1.0.0" is the npm package. |
| `update_time` | String | Output only. Time the package was updated. |
| `version` | String | Version of this package. |
| `create_time` | String | Output only. Time the package was created. |
| `tags` | Vec<String> | Tags attached to this package. |
| `package_name` | String | Package for the artifact. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access npm_package outputs
npm_package_id = npm_package.id
npm_package_name = npm_package.name
npm_package_update_time = npm_package.update_time
npm_package_version = npm_package.version
npm_package_create_time = npm_package.create_time
npm_package_tags = npm_package.tags
npm_package_package_name = npm_package.package_name
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
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
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

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
```

---


### Attachment

Creates an attachment. The returned Operation will finish once the attachment has been created. Its response will be the created attachment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attachment_namespace` | String |  | The namespace this attachment belongs to. E.g. If an attachment is created by artifact analysis, namespace is set to `artifactanalysis.googleapis.com`. |
| `update_time` | String |  | Output only. The time when the attachment was last updated. |
| `create_time` | String |  | Output only. The time when the attachment was created. |
| `files` | Vec<String> |  | Required. The files that belong to this attachment. If the file ID part contains slashes, they are escaped. E.g. `projects/p1/locations/us-central1/repositories/repo1/files/sha:`. |
| `name` | String |  | The name of the attachment. E.g. `projects/p1/locations/us/repositories/repo/attachments/sbom`. |
| `annotations` | HashMap<String, String> |  | Optional. User annotations. These attributes can only be set and used by the user, and not by Artifact Registry. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `type` | String |  | Type of attachment. E.g. `application/vnd.spdx+json` |
| `oci_version_name` | String |  | Output only. The name of the OCI version that this attachment created. Only populated for Docker attachments. E.g. `projects/p1/locations/us-central1/repositories/repo1/packages/p1/versions/v1`. |
| `target` | String |  | Required. The target the attachment is for, can be a Version, Package or Repository. E.g. `projects/p1/locations/us-central1/repositories/repo1/packages/p1/versions/v1`. |
| `parent` | String | ✅ | Required. The name of the parent resource where the attachment will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `attachment_namespace` | String | The namespace this attachment belongs to. E.g. If an attachment is created by artifact analysis, namespace is set to `artifactanalysis.googleapis.com`. |
| `update_time` | String | Output only. The time when the attachment was last updated. |
| `create_time` | String | Output only. The time when the attachment was created. |
| `files` | Vec<String> | Required. The files that belong to this attachment. If the file ID part contains slashes, they are escaped. E.g. `projects/p1/locations/us-central1/repositories/repo1/files/sha:`. |
| `name` | String | The name of the attachment. E.g. `projects/p1/locations/us/repositories/repo/attachments/sbom`. |
| `annotations` | HashMap<String, String> | Optional. User annotations. These attributes can only be set and used by the user, and not by Artifact Registry. See https://google.aip.dev/128#annotations for more details such as format and size limitations. |
| `type` | String | Type of attachment. E.g. `application/vnd.spdx+json` |
| `oci_version_name` | String | Output only. The name of the OCI version that this attachment created. Only populated for Docker attachments. E.g. `projects/p1/locations/us-central1/repositories/repo1/packages/p1/versions/v1`. |
| `target` | String | Required. The target the attachment is for, can be a Version, Package or Repository. E.g. `projects/p1/locations/us-central1/repositories/repo1/packages/p1/versions/v1`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attachment
attachment = provider.artifactregistry_api.Attachment {
    parent = "value"  # Required. The name of the parent resource where the attachment will be created.
}

# Access attachment outputs
attachment_id = attachment.id
attachment_attachment_namespace = attachment.attachment_namespace
attachment_update_time = attachment.update_time
attachment_create_time = attachment.create_time
attachment_files = attachment.files
attachment_name = attachment.name
attachment_annotations = attachment.annotations
attachment_type = attachment.type
attachment_oci_version_name = attachment.oci_version_name
attachment_target = attachment.target
```

---


### Project

Retrieves the Settings for the Project.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the project's settings. Always of the form: projects/{project-id}/projectSettings In update request: never set In response: always set |
| `legacy_redirection_state` | String |  | The redirection state of the legacy repositories in this project. |
| `pull_percent` | i64 |  | The percentage of pull traffic to redirect from GCR to AR when using partial redirection. |
| `name` | String | ✅ | The name of the project's settings. Always of the form: projects/{project-id}/projectSettings In update request: never set In response: always set |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the project's settings. Always of the form: projects/{project-id}/projectSettings In update request: never set In response: always set |
| `legacy_redirection_state` | String | The redirection state of the legacy repositories in this project. |
| `pull_percent` | i64 | The percentage of pull traffic to redirect from GCR to AR when using partial redirection. |


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
project_name = project.name
project_legacy_redirection_state = project.legacy_redirection_state
project_pull_percent = project.pull_percent
```

---


### Repositorie

Creates a repository. The returned Operation will finish once the repository has been created. Its response will be the created Repository.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `satisfies_pzs` | bool |  | Output only. Whether or not this repository satisfies PZS. |
| `maven_config` | String |  | Maven repository config contains repository level configuration for the repositories of maven type. |
| `size_bytes` | String |  | Output only. The size, in bytes, of all artifact storage in this repository. Repositories that are generally available or in public preview use this to calculate storage costs. |
| `create_time` | String |  | Output only. The time when the repository was created. |
| `format` | String |  | Optional. The format of packages that are stored in the repository. |
| `virtual_repository_config` | String |  | Configuration specific for a Virtual Repository. |
| `labels` | HashMap<String, String> |  | Labels with user-defined metadata. This field may contain up to 64 entries. Label keys and values may be no longer than 63 characters. Label keys must begin with a lowercase letter and may only contain lowercase letters, numeric characters, underscores, and dashes. |
| `mode` | String |  | Optional. The mode of the repository. |
| `disallow_unspecified_mode` | bool |  | Optional. If this is true, an unspecified repo type will be treated as error rather than defaulting to standard. |
| `registry_uri` | String |  | Output only. The repository endpoint, for example: `us-docker.pkg.dev/my-proj/my-repo`. |
| `cleanup_policy_dry_run` | bool |  | Optional. If true, the cleanup pipeline is prevented from deleting versions in this repository. |
| `satisfies_pzi` | bool |  | Output only. Whether or not this repository satisfies PZI. |
| `update_time` | String |  | Output only. The time when the repository was last updated. |
| `description` | String |  | The user-provided description of the repository. |
| `remote_repository_config` | String |  | Configuration specific for a Remote Repository. |
| `name` | String |  | The name of the repository, for example: `projects/p1/locations/us-central1/repositories/repo1`. For each location in a project, repository names must be unique. |
| `docker_config` | String |  | Docker repository config contains repository level configuration for the repositories of docker type. |
| `cleanup_policies` | HashMap<String, String> |  | Optional. Cleanup policies for this repository. Cleanup policies indicate when certain package versions can be automatically deleted. Map keys are policy IDs supplied by users during policy creation. They must unique within a repository and be under 128 characters in length. |
| `vulnerability_scanning_config` | String |  | Optional. Config and state for vulnerability scanning of resources within this Repository. |
| `kms_key_name` | String |  | The Cloud KMS resource name of the customer managed encryption key that's used to encrypt the contents of the Repository. Has the form: `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`. This value may not be changed after the Repository has been created. |
| `parent` | String | ✅ | Required. The name of the parent resource where the repository will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `satisfies_pzs` | bool | Output only. Whether or not this repository satisfies PZS. |
| `maven_config` | String | Maven repository config contains repository level configuration for the repositories of maven type. |
| `size_bytes` | String | Output only. The size, in bytes, of all artifact storage in this repository. Repositories that are generally available or in public preview use this to calculate storage costs. |
| `create_time` | String | Output only. The time when the repository was created. |
| `format` | String | Optional. The format of packages that are stored in the repository. |
| `virtual_repository_config` | String | Configuration specific for a Virtual Repository. |
| `labels` | HashMap<String, String> | Labels with user-defined metadata. This field may contain up to 64 entries. Label keys and values may be no longer than 63 characters. Label keys must begin with a lowercase letter and may only contain lowercase letters, numeric characters, underscores, and dashes. |
| `mode` | String | Optional. The mode of the repository. |
| `disallow_unspecified_mode` | bool | Optional. If this is true, an unspecified repo type will be treated as error rather than defaulting to standard. |
| `registry_uri` | String | Output only. The repository endpoint, for example: `us-docker.pkg.dev/my-proj/my-repo`. |
| `cleanup_policy_dry_run` | bool | Optional. If true, the cleanup pipeline is prevented from deleting versions in this repository. |
| `satisfies_pzi` | bool | Output only. Whether or not this repository satisfies PZI. |
| `update_time` | String | Output only. The time when the repository was last updated. |
| `description` | String | The user-provided description of the repository. |
| `remote_repository_config` | String | Configuration specific for a Remote Repository. |
| `name` | String | The name of the repository, for example: `projects/p1/locations/us-central1/repositories/repo1`. For each location in a project, repository names must be unique. |
| `docker_config` | String | Docker repository config contains repository level configuration for the repositories of docker type. |
| `cleanup_policies` | HashMap<String, String> | Optional. Cleanup policies for this repository. Cleanup policies indicate when certain package versions can be automatically deleted. Map keys are policy IDs supplied by users during policy creation. They must unique within a repository and be under 128 characters in length. |
| `vulnerability_scanning_config` | String | Optional. Config and state for vulnerability scanning of resources within this Repository. |
| `kms_key_name` | String | The Cloud KMS resource name of the customer managed encryption key that's used to encrypt the contents of the Repository. Has the form: `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`. This value may not be changed after the Repository has been created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create repositorie
repositorie = provider.artifactregistry_api.Repositorie {
    parent = "value"  # Required. The name of the parent resource where the repository will be created.
}

# Access repositorie outputs
repositorie_id = repositorie.id
repositorie_satisfies_pzs = repositorie.satisfies_pzs
repositorie_maven_config = repositorie.maven_config
repositorie_size_bytes = repositorie.size_bytes
repositorie_create_time = repositorie.create_time
repositorie_format = repositorie.format
repositorie_virtual_repository_config = repositorie.virtual_repository_config
repositorie_labels = repositorie.labels
repositorie_mode = repositorie.mode
repositorie_disallow_unspecified_mode = repositorie.disallow_unspecified_mode
repositorie_registry_uri = repositorie.registry_uri
repositorie_cleanup_policy_dry_run = repositorie.cleanup_policy_dry_run
repositorie_satisfies_pzi = repositorie.satisfies_pzi
repositorie_update_time = repositorie.update_time
repositorie_description = repositorie.description
repositorie_remote_repository_config = repositorie.remote_repository_config
repositorie_name = repositorie.name
repositorie_docker_config = repositorie.docker_config
repositorie_cleanup_policies = repositorie.cleanup_policies
repositorie_vulnerability_scanning_config = repositorie.vulnerability_scanning_config
repositorie_kms_key_name = repositorie.kms_key_name
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the project's VPC SC Config. Always of the form: projects/{projectID}/locations/{location}/vpcscConfig In update request: never set In response: always set |
| `vpcsc_policy` | String |  | The project per location VPC SC policy that defines the VPC SC behavior for the Remote Repository (Allow/Deny). |
| `name` | String | ✅ | The name of the project's VPC SC Config. Always of the form: projects/{projectID}/locations/{location}/vpcscConfig In update request: never set In response: always set |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
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
location_labels = location.labels
location_metadata = location.metadata
location_location_id = location.location_id
location_display_name = location.display_name
```

---


### Python_package

Gets a python package.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time the package was created. |
| `name` | String | Required. registry_location, project_id, repository_name and python_package forms a unique package name:`projects//locations//repository//pythonPackages/`. For example, "projects/test-project/locations/us-west4/repositories/test-repo/pythonPackages/ python_package:1.0.0", where "us-west4" is the registry_location, "test-project" is the project_id, "test-repo" is the repository_name and python_package:1.0.0" is the python package. |
| `package_name` | String | Package for the artifact. |
| `uri` | String | Required. URL to access the package. Example: us-west4-python.pkg.dev/test-project/test-repo/python_package/file-name-1.0.0.tar.gz |
| `version` | String | Version of this package. |
| `update_time` | String | Output only. Time the package was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access python_package outputs
python_package_id = python_package.id
python_package_create_time = python_package.create_time
python_package_name = python_package.name
python_package_package_name = python_package.package_name
python_package_uri = python_package.uri
python_package_version = python_package.version
python_package_update_time = python_package.update_time
```

---


### Maven_artifact

Gets a maven artifact.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Time the artifact was updated. |
| `create_time` | String | Output only. Time the artifact was created. |
| `artifact_id` | String | Artifact ID for the artifact. |
| `pom_uri` | String | Required. URL to access the pom file of the artifact. Example: us-west4-maven.pkg.dev/test-project/test-repo/com/google/guava/guava/31.0/guava-31.0.pom |
| `group_id` | String | Group ID for the artifact. Example: com.google.guava |
| `version` | String | Version of this artifact. |
| `name` | String | Required. registry_location, project_id, repository_name and maven_artifact forms a unique artifact For example, "projects/test-project/locations/us-west4/repositories/test-repo/mavenArtifacts/ com.google.guava:guava:31.0-jre", where "us-west4" is the registry_location, "test-project" is the project_id, "test-repo" is the repository_name and "com.google.guava:guava:31.0-jre" is the maven artifact. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access maven_artifact outputs
maven_artifact_id = maven_artifact.id
maven_artifact_update_time = maven_artifact.update_time
maven_artifact_create_time = maven_artifact.create_time
maven_artifact_artifact_id = maven_artifact.artifact_id
maven_artifact_pom_uri = maven_artifact.pom_uri
maven_artifact_group_id = maven_artifact.group_id
maven_artifact_version = maven_artifact.version
maven_artifact_name = maven_artifact.name
```

---


### Apt_artifact

Imports Apt artifacts. The returned Operation will complete once the resources are imported. Package, Version, and File resources are created based on the imported artifacts. Imported artifacts that conflict with existing resources are ignored.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_source` | String |  | Google Cloud Storage location where input content is located. |
| `parent` | String | ✅ | The name of the parent resource where the artifacts will be imported. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create apt_artifact
apt_artifact = provider.artifactregistry_api.Apt_artifact {
    parent = "value"  # The name of the parent resource where the artifacts will be imported.
}

```

---


### Tag

Creates a tag.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the tag, for example: "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/tags/tag1". If the package part contains slashes, the slashes are escaped. The tag part can only have characters in [a-zA-Z0-9\-._~:@], anything else must be URL encoded. |
| `version` | String |  | The name of the version the tag refers to, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/sha256:5243811` If the package or version ID parts contain slashes, the slashes are escaped. |
| `parent` | String | ✅ | The name of the parent resource where the tag will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the tag, for example: "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/tags/tag1". If the package part contains slashes, the slashes are escaped. The tag part can only have characters in [a-zA-Z0-9\-._~:@], anything else must be URL encoded. |
| `version` | String | The name of the version the tag refers to, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/sha256:5243811` If the package or version ID parts contain slashes, the slashes are escaped. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag
tag = provider.artifactregistry_api.Tag {
    parent = "value"  # The name of the parent resource where the tag will be created.
}

# Access tag outputs
tag_id = tag.id
tag_name = tag.name
tag_version = tag.version
```

---


### Go_module

Directly uploads a Go module. The returned Operation will complete once the Go module is uploaded. Package, Version, and File resources are created based on the uploaded Go module.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | The resource name of the repository where the Go module will be uploaded. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create go_module
go_module = provider.artifactregistry_api.Go_module {
    parent = "value"  # The resource name of the repository where the Go module will be uploaded.
}

```

---


### File

Directly uploads a file to a repository. The returned Operation will complete once the resources are uploaded.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_id` | String |  | Optional. The ID of the file. If left empty will default to sha256 digest of the content uploaded. |
| `parent` | String | ✅ | Required. The resource name of the repository where the file will be uploaded. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Optional. Client specified annotations. |
| `name` | String | The name of the file, for example: `projects/p1/locations/us-central1/repositories/repo1/files/a%2Fb%2Fc.txt`. If the file ID part contains slashes, they are escaped. |
| `size_bytes` | String | The size of the File in bytes. |
| `update_time` | String | Output only. The time when the File was last updated. |
| `owner` | String | The name of the Package or Version that owns this file, if any. |
| `hashes` | Vec<String> | The hashes of the file content. |
| `create_time` | String | Output only. The time when the File was created. |
| `fetch_time` | String | Output only. The time when the last attempt to refresh the file's data was made. Only set when the repository is remote. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create file
file = provider.artifactregistry_api.File {
    parent = "value"  # Required. The resource name of the repository where the file will be uploaded.
}

# Access file outputs
file_id = file.id
file_annotations = file.annotations
file_name = file.name
file_size_bytes = file.size_bytes
file_update_time = file.update_time
file_owner = file.owner
file_hashes = file.hashes
file_create_time = file.create_time
file_fetch_time = file.fetch_time
```

---


### File

Gets a file.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time when the File was last updated. |
| `create_time` | String | Output only. The time when the File was created. |
| `owner` | String | The name of the Package or Version that owns this file, if any. |
| `name` | String | The name of the file, for example: `projects/p1/locations/us-central1/repositories/repo1/files/a%2Fb%2Fc.txt`. If the file ID part contains slashes, they are escaped. |
| `size_bytes` | String | The size of the File in bytes. |
| `hashes` | Vec<String> | The hashes of the file content. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access file outputs
file_id = file.id
file_update_time = file.update_time
file_create_time = file.create_time
file_owner = file.owner
file_name = file.name
file_size_bytes = file.size_bytes
file_hashes = file.hashes
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


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
location_display_name = location.display_name
location_name = location.name
location_metadata = location.metadata
location_labels = location.labels
```

---


### Apt_artifact

Imports Apt artifacts. The returned Operation will complete once the resources are imported. Package, Version, and File resources are created based on the imported artifacts. Imported artifacts that conflict with existing resources are ignored.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gcs_source` | String |  | Google Cloud Storage location where input content is located. |
| `parent` | String | ✅ | The name of the parent resource where the artifacts will be imported. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create apt_artifact
apt_artifact = provider.artifactregistry_api.Apt_artifact {
    parent = "value"  # The name of the parent resource where the artifacts will be imported.
}

```

---


### Package

Gets a package.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | The time when the package was created. |
| `annotations` | HashMap<String, String> |  | Optional. Client specified annotations. |
| `display_name` | String |  | The display name of the package. |
| `name` | String |  | The name of the package, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1`. If the package ID part contains slashes, the slashes are escaped. |
| `update_time` | String |  | The time when the package was last updated. This includes publishing a new version of the package. |
| `name` | String | ✅ | The name of the package, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1`. If the package ID part contains slashes, the slashes are escaped. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | The time when the package was created. |
| `annotations` | HashMap<String, String> | Optional. Client specified annotations. |
| `display_name` | String | The display name of the package. |
| `name` | String | The name of the package, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1`. If the package ID part contains slashes, the slashes are escaped. |
| `update_time` | String | The time when the package was last updated. This includes publishing a new version of the package. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access package outputs
package_id = package.id
package_create_time = package.create_time
package_annotations = package.annotations
package_display_name = package.display_name
package_name = package.name
package_update_time = package.update_time
```

---


### Project

Retrieves the Settings for the Project.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the project's settings. Always of the form: projects/{project-id}/projectSettings In update request: never set In response: always set |
| `legacy_redirection_state` | String |  | The redirection state of the legacy repositories in this project. |
| `pull_percent` | i64 |  | The percentage of pull traffic to redirect from GCR to AR when using partial redirection. |
| `name` | String | ✅ | The name of the project's settings. Always of the form: projects/{project-id}/projectSettings In update request: never set In response: always set |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the project's settings. Always of the form: projects/{project-id}/projectSettings In update request: never set In response: always set |
| `legacy_redirection_state` | String | The redirection state of the legacy repositories in this project. |
| `pull_percent` | i64 | The percentage of pull traffic to redirect from GCR to AR when using partial redirection. |


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
project_name = project.name
project_legacy_redirection_state = project.legacy_redirection_state
project_pull_percent = project.pull_percent
```

---


### Version

Gets a version

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional. Description of the version, as specified in its metadata. |
| `metadata` | HashMap<String, String> | Output only. Repository-specific Metadata stored against this version. The fields returned are defined by the underlying repository-specific resource. Currently, the resources could be: DockerImage MavenArtifact |
| `create_time` | String | The time when the version was created. |
| `name` | String | The name of the version, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/art1`. If the package or version ID parts contain slashes, the slashes are escaped. |
| `related_tags` | Vec<String> | Output only. A list of related tags. Will contain up to 100 tags that reference this version. |
| `update_time` | String | The time when the version was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access version outputs
version_id = version.id
version_description = version.description
version_metadata = version.metadata
version_create_time = version.create_time
version_name = version.name
version_related_tags = version.related_tags
version_update_time = version.update_time
```

---


### Tag

Creates a tag.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | String |  | The name of the version the tag refers to, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/sha256:5243811` If the package or version ID parts contain slashes, the slashes are escaped. |
| `name` | String |  | The name of the tag, for example: "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/tags/tag1". If the package part contains slashes, the slashes are escaped. The tag part can only have characters in [a-zA-Z0-9\-._~:@], anything else must be URL encoded. |
| `parent` | String | ✅ | The name of the parent resource where the tag will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | String | The name of the version the tag refers to, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/sha256:5243811` If the package or version ID parts contain slashes, the slashes are escaped. |
| `name` | String | The name of the tag, for example: "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/tags/tag1". If the package part contains slashes, the slashes are escaped. The tag part can only have characters in [a-zA-Z0-9\-._~:@], anything else must be URL encoded. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag
tag = provider.artifactregistry_api.Tag {
    parent = "value"  # The name of the parent resource where the tag will be created.
}

# Access tag outputs
tag_id = tag.id
tag_version = tag.version
tag_name = tag.name
```

---


### Yum_artifact

Directly uploads a Yum artifact. The returned Operation will complete once the resources are uploaded. Package, Version, and File resources are created based on the imported artifact. Imported artifacts that conflict with existing resources are ignored.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String | ✅ | The name of the parent resource where the artifacts will be uploaded. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create yum_artifact
yum_artifact = provider.artifactregistry_api.Yum_artifact {
    parent = "value"  # The name of the parent resource where the artifacts will be uploaded.
}

```

---


### Repositorie

Creates a repository. The returned Operation will finish once the repository has been created. Its response will be the created Repository.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The time when the repository was last updated. |
| `kms_key_name` | String |  | The Cloud KMS resource name of the customer managed encryption key that's used to encrypt the contents of the Repository. Has the form: `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`. This value may not be changed after the Repository has been created. |
| `size_bytes` | String |  | Output only. The size, in bytes, of all artifact storage in this repository. Repositories that are generally available or in public preview use this to calculate storage costs. |
| `labels` | HashMap<String, String> |  | Labels with user-defined metadata. This field may contain up to 64 entries. Label keys and values may be no longer than 63 characters. Label keys must begin with a lowercase letter and may only contain lowercase letters, numeric characters, underscores, and dashes. |
| `description` | String |  | The user-provided description of the repository. |
| `format` | String |  | Optional. The format of packages that are stored in the repository. |
| `create_time` | String |  | Output only. The time when the repository was created. |
| `maven_config` | String |  | Maven repository config contains repository level configuration for the repositories of maven type. |
| `satisfies_pzi` | bool |  | Output only. Whether or not this repository satisfies PZI. |
| `name` | String |  | The name of the repository, for example: `projects/p1/locations/us-central1/repositories/repo1`. For each location in a project, repository names must be unique. |
| `satisfies_pzs` | bool |  | Output only. Whether or not this repository satisfies PZS. |
| `parent` | String | ✅ | Required. The name of the parent resource where the repository will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time when the repository was last updated. |
| `kms_key_name` | String | The Cloud KMS resource name of the customer managed encryption key that's used to encrypt the contents of the Repository. Has the form: `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`. This value may not be changed after the Repository has been created. |
| `size_bytes` | String | Output only. The size, in bytes, of all artifact storage in this repository. Repositories that are generally available or in public preview use this to calculate storage costs. |
| `labels` | HashMap<String, String> | Labels with user-defined metadata. This field may contain up to 64 entries. Label keys and values may be no longer than 63 characters. Label keys must begin with a lowercase letter and may only contain lowercase letters, numeric characters, underscores, and dashes. |
| `description` | String | The user-provided description of the repository. |
| `format` | String | Optional. The format of packages that are stored in the repository. |
| `create_time` | String | Output only. The time when the repository was created. |
| `maven_config` | String | Maven repository config contains repository level configuration for the repositories of maven type. |
| `satisfies_pzi` | bool | Output only. Whether or not this repository satisfies PZI. |
| `name` | String | The name of the repository, for example: `projects/p1/locations/us-central1/repositories/repo1`. For each location in a project, repository names must be unique. |
| `satisfies_pzs` | bool | Output only. Whether or not this repository satisfies PZS. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create repositorie
repositorie = provider.artifactregistry_api.Repositorie {
    parent = "value"  # Required. The name of the parent resource where the repository will be created.
}

# Access repositorie outputs
repositorie_id = repositorie.id
repositorie_update_time = repositorie.update_time
repositorie_kms_key_name = repositorie.kms_key_name
repositorie_size_bytes = repositorie.size_bytes
repositorie_labels = repositorie.labels
repositorie_description = repositorie.description
repositorie_format = repositorie.format
repositorie_create_time = repositorie.create_time
repositorie_maven_config = repositorie.maven_config
repositorie_satisfies_pzi = repositorie.satisfies_pzi
repositorie_name = repositorie.name
repositorie_satisfies_pzs = repositorie.satisfies_pzs
```

---


### File

Gets a file.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `owner` | String | The name of the Package or Version that owns this file, if any. |
| `hashes` | Vec<String> | The hashes of the file content. |
| `create_time` | String | Output only. The time when the File was created. |
| `size_bytes` | String | The size of the File in bytes. |
| `name` | String | The name of the file, for example: `projects/p1/locations/us-central1/repositories/repo1/files/a%2Fb%2Fc.txt`. If the file ID part contains slashes, they are escaped. |
| `update_time` | String | Output only. The time when the File was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access file outputs
file_id = file.id
file_owner = file.owner
file_hashes = file.hashes
file_create_time = file.create_time
file_size_bytes = file.size_bytes
file_name = file.name
file_update_time = file.update_time
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
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
```

---


### Repositorie

Creates a repository. The returned Operation will finish once the repository has been created. Its response will be the created Repository.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the repository, for example: `projects/p1/locations/us-central1/repositories/repo1`. For each location in a project, repository names must be unique. |
| `create_time` | String |  | Output only. The time when the repository was created. |
| `satisfies_pzs` | bool |  | Output only. Whether or not this repository satisfies PZS. |
| `satisfies_pzi` | bool |  | Output only. Whether or not this repository satisfies PZI. |
| `update_time` | String |  | Output only. The time when the repository was last updated. |
| `labels` | HashMap<String, String> |  | Labels with user-defined metadata. This field may contain up to 64 entries. Label keys and values may be no longer than 63 characters. Label keys must begin with a lowercase letter and may only contain lowercase letters, numeric characters, underscores, and dashes. |
| `size_bytes` | String |  | Output only. The size, in bytes, of all artifact storage in this repository. Repositories that are generally available or in public preview use this to calculate storage costs. |
| `kms_key_name` | String |  | The Cloud KMS resource name of the customer managed encryption key that's used to encrypt the contents of the Repository. Has the form: `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`. This value may not be changed after the Repository has been created. |
| `description` | String |  | The user-provided description of the repository. |
| `format` | String |  | Optional. The format of packages that are stored in the repository. |
| `parent` | String | ✅ | Required. The name of the parent resource where the repository will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the repository, for example: `projects/p1/locations/us-central1/repositories/repo1`. For each location in a project, repository names must be unique. |
| `create_time` | String | Output only. The time when the repository was created. |
| `satisfies_pzs` | bool | Output only. Whether or not this repository satisfies PZS. |
| `satisfies_pzi` | bool | Output only. Whether or not this repository satisfies PZI. |
| `update_time` | String | Output only. The time when the repository was last updated. |
| `labels` | HashMap<String, String> | Labels with user-defined metadata. This field may contain up to 64 entries. Label keys and values may be no longer than 63 characters. Label keys must begin with a lowercase letter and may only contain lowercase letters, numeric characters, underscores, and dashes. |
| `size_bytes` | String | Output only. The size, in bytes, of all artifact storage in this repository. Repositories that are generally available or in public preview use this to calculate storage costs. |
| `kms_key_name` | String | The Cloud KMS resource name of the customer managed encryption key that's used to encrypt the contents of the Repository. Has the form: `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`. This value may not be changed after the Repository has been created. |
| `description` | String | The user-provided description of the repository. |
| `format` | String | Optional. The format of packages that are stored in the repository. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create repositorie
repositorie = provider.artifactregistry_api.Repositorie {
    parent = "value"  # Required. The name of the parent resource where the repository will be created.
}

# Access repositorie outputs
repositorie_id = repositorie.id
repositorie_name = repositorie.name
repositorie_create_time = repositorie.create_time
repositorie_satisfies_pzs = repositorie.satisfies_pzs
repositorie_satisfies_pzi = repositorie.satisfies_pzi
repositorie_update_time = repositorie.update_time
repositorie_labels = repositorie.labels
repositorie_size_bytes = repositorie.size_bytes
repositorie_kms_key_name = repositorie.kms_key_name
repositorie_description = repositorie.description
repositorie_format = repositorie.format
```

---


### Version

Gets a version

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | The time when the version was created. |
| `update_time` | String | The time when the version was last updated. |
| `description` | String | Optional. Description of the version, as specified in its metadata. |
| `related_tags` | Vec<String> | Output only. A list of related tags. Will contain up to 100 tags that reference this version. |
| `name` | String | The name of the version, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/art1`. If the package or version ID parts contain slashes, the slashes are escaped. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access version outputs
version_id = version.id
version_create_time = version.create_time
version_update_time = version.update_time
version_description = version.description
version_related_tags = version.related_tags
version_name = version.name
```

---


### Tag

Creates a tag.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the tag, for example: "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/tags/tag1". If the package part contains slashes, the slashes are escaped. The tag part can only have characters in [a-zA-Z0-9\-._~:@], anything else must be URL encoded. |
| `version` | String |  | The name of the version the tag refers to, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/sha256:5243811` If the package or version ID parts contain slashes, the slashes are escaped. |
| `parent` | String | ✅ | The name of the parent resource where the tag will be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the tag, for example: "projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/tags/tag1". If the package part contains slashes, the slashes are escaped. The tag part can only have characters in [a-zA-Z0-9\-._~:@], anything else must be URL encoded. |
| `version` | String | The name of the version the tag refers to, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1/versions/sha256:5243811` If the package or version ID parts contain slashes, the slashes are escaped. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create tag
tag = provider.artifactregistry_api.Tag {
    parent = "value"  # The name of the parent resource where the tag will be created.
}

# Access tag outputs
tag_id = tag.id
tag_name = tag.name
tag_version = tag.version
```

---


### Package

Gets a package.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | The time when the package was created. |
| `update_time` | String | The time when the package was last updated. This includes publishing a new version of the package. |
| `display_name` | String | The display name of the package. |
| `name` | String | The name of the package, for example: `projects/p1/locations/us-central1/repositories/repo1/packages/pkg1`. If the package ID part contains slashes, the slashes are escaped. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access package outputs
package_id = package.id
package_create_time = package.create_time
package_update_time = package.update_time
package_display_name = package.display_name
package_name = package.name
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
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


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
location_display_name = location.display_name
location_metadata = location.metadata
location_name = location.name
location_labels = location.labels
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple yum_artifact resources
yum_artifact_0 = provider.artifactregistry_api.Yum_artifact {
    parent = "value-0"
}
yum_artifact_1 = provider.artifactregistry_api.Yum_artifact {
    parent = "value-1"
}
yum_artifact_2 = provider.artifactregistry_api.Yum_artifact {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    yum_artifact = provider.artifactregistry_api.Yum_artifact {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Artifactregistry_api Documentation](https://cloud.google.com/artifactregistry_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
