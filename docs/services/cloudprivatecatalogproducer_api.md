# Cloudprivatecatalogproducer_api Service



**Resources**: 6

---

## Overview

The cloudprivatecatalogproducer_api service provides access to 6 resource types:

- [Operation](#operation) [CRD]
- [Icon](#icon) [C]
- [Catalog](#catalog) [CRUD]
- [Product](#product) [CRUD]
- [Version](#version) [CRUD]
- [Association](#association) [CRD]

---

## Resources


### Operation

Starts asynchronous cancellation on a long-running operation.  The server
makes a best effort to cancel the operation, but success is not
guaranteed.  If the server doesn't support this method, it returns
`google.rpc.Code.UNIMPLEMENTED`.  Clients can use
Operations.GetOperation or
other methods to check whether the cancellation succeeded or whether the
operation completed despite cancellation. On successful cancellation,
the operation is not deleted; instead, it becomes an operation with
an Operation.error value with a google.rpc.Status.code of 1,
corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal response of the operation in case of success.  If the original
method returns no data on success, such as `Delete`, the response is
`google.protobuf.Empty`.  If the original method is standard
`Get`/`Create`/`Update`, the response should be the resource.  For other
methods, the response should have the type `XxxResponse`, where `Xxx`
is the original method name.  For example, if the original method name
is `TakeSnapshot()`, the inferred response type is
`TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation.  It typically
contains progress information and common metadata such as create time.
Some services might not provide such metadata.  Any method that returns a
long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that
originally returns it. If you use the default HTTP mapping, the
`name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress.
If `true`, the operation is completed, and either `error` or `response` is
available. |
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
operation = provider.cloudprivatecatalogproducer_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_metadata = operation.metadata
operation_name = operation.name
operation_done = operation.done
operation_error = operation.error
```

---


### Icon

Creates an Icon instance under a given Product.
If Product only has a default icon, a new Icon
instance is created and associated with the given Product.
If Product already has a non-default icon, the action creates
a new Icon instance, associates the newly created
Icon with the given Product and deletes the old icon.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `icon` | String |  | The raw icon bytes user-supplied to be uploaded to the product. The format
of the icon can only be PNG or JPEG. The minimum allowed dimensions are
130x130 pixels and the maximum dimensions are 10000x10000 pixels.
Required. |
| `product` | String | ✅ | The resource name of the product. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create icon
icon = provider.cloudprivatecatalogproducer_api.Icon {
    product = "value"  # The resource name of the product.
}

```

---


### Catalog

Creates a new Catalog resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Required. The user-supplied description of the catalog. Maximum of 512
characters. |
| `display_name` | String |  | Required. The user-supplied descriptive name of the catalog as it appears
in UIs. Maximum 256 characters in length. |
| `name` | String |  | Output only. The resource name of the catalog, in the format
`catalogs/{catalog_id}'.

A unique identifier for the catalog, which is generated
by catalog service. |
| `create_time` | String |  | Output only. The time when the catalog was created. |
| `update_time` | String |  | Output only. The time when the catalog was last updated. |
| `parent` | String |  | Required. The parent resource name of the catalog, which can't be changed
after a catalog is created. It can only be an organization. Values are of
the form `//cloudresourcemanager.googleapis.com/organizations/<id>`.
Maximum 256 characters in length. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Required. The user-supplied description of the catalog. Maximum of 512
characters. |
| `display_name` | String | Required. The user-supplied descriptive name of the catalog as it appears
in UIs. Maximum 256 characters in length. |
| `name` | String | Output only. The resource name of the catalog, in the format
`catalogs/{catalog_id}'.

A unique identifier for the catalog, which is generated
by catalog service. |
| `create_time` | String | Output only. The time when the catalog was created. |
| `update_time` | String | Output only. The time when the catalog was last updated. |
| `parent` | String | Required. The parent resource name of the catalog, which can't be changed
after a catalog is created. It can only be an organization. Values are of
the form `//cloudresourcemanager.googleapis.com/organizations/<id>`.
Maximum 256 characters in length. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create catalog
catalog = provider.cloudprivatecatalogproducer_api.Catalog {
}

# Access catalog outputs
catalog_id = catalog.id
catalog_description = catalog.description
catalog_display_name = catalog.display_name
catalog_name = catalog.name
catalog_create_time = catalog.create_time
catalog_update_time = catalog.update_time
catalog_parent = catalog.parent
```

---


### Product

Creates a Product instance under a given Catalog.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset_type` | String |  | Required. The type of the product asset, which cannot be changed after the
product is created. It supports the values
`google.deploymentmanager.Template` and
`google.cloudprivatecatalog.ListingOnly`. Other values will be
rejected by the server. Read only after creation.

The following fields or resource types have different validation rules
under each `asset_type` values:

* Product.display_metadata has different validation schema for each
asset type value. See its comment for details.
* Version resource isn't allowed to be added under the
`google.cloudprivatecatalog.ListingOnly` type. |
| `display_metadata` | HashMap<String, String> |  | The user-supplied display metadata to describe the product.
The JSON schema of the metadata differs by Product.asset_type.
When the type is `google.deploymentmanager.Template`, the schema is as
follows:

```
"$schema": http://json-schema.org/draft-04/schema#
type: object
properties:
  name:
    type: string
    minLength: 1
    maxLength: 64
  description:
    type: string
    minLength: 1
    maxLength: 2048
  tagline:
    type: string
    minLength: 1
    maxLength: 100
  support_info:
    type: string
    minLength: 1
    maxLength: 2048
  creator:
    type: string
    minLength: 1
    maxLength: 100
  documentation:
    type: array
    items:
      type: object
      properties:
        url:
          type: string
          pattern:
          "^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]"
        title:
          type: string
          minLength: 1
          maxLength: 64
        description:
          type: string
          minLength: 1
          maxLength: 2048
required:
- name
- description
additionalProperties: false

```

When the asset type is `google.cloudprivatecatalog.ListingOnly`, the schema
is as follows:

```
"$schema": http://json-schema.org/draft-04/schema#
type: object
properties:
  name:
    type: string
    minLength: 1
    maxLength: 64
  description:
    type: string
    minLength: 1
    maxLength: 2048
  tagline:
    type: string
    minLength: 1
    maxLength: 100
  support_info:
    type: string
    minLength: 1
    maxLength: 2048
  creator:
    type: string
    minLength: 1
    maxLength: 100
  documentation:
    type: array
    items:
      type: object
      properties:
        url:
          type: string
          pattern:
          "^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]"
        title:
          type: string
          minLength: 1
          maxLength: 64
        description:
          type: string
          minLength: 1
          maxLength: 2048
  signup_url:
    type: string
    pattern:
    "^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]"
required:
- name
- description
- signup_url
additionalProperties: false
``` |
| `create_time` | String |  | Output only. The time when the product was created. |
| `icon_uri` | String |  | Output only. The public accessible URI of the icon uploaded by
PrivateCatalogProducer.UploadIcon.

If no icon is uploaded, it will be the default icon's URI. |
| `name` | String |  | Required. The resource name of the product in the format
`catalogs/{catalog_id}/products/a-z*[a-z0-9]'.

A unique identifier for the product under a catalog, which cannot
be changed after the product is created. The final
segment of the name must between 1 and 256 characters in length. |
| `update_time` | String |  | Output only. The time when the product was last updated. |
| `parent` | String | ✅ | The catalog name of the new product's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `asset_type` | String | Required. The type of the product asset, which cannot be changed after the
product is created. It supports the values
`google.deploymentmanager.Template` and
`google.cloudprivatecatalog.ListingOnly`. Other values will be
rejected by the server. Read only after creation.

The following fields or resource types have different validation rules
under each `asset_type` values:

* Product.display_metadata has different validation schema for each
asset type value. See its comment for details.
* Version resource isn't allowed to be added under the
`google.cloudprivatecatalog.ListingOnly` type. |
| `display_metadata` | HashMap<String, String> | The user-supplied display metadata to describe the product.
The JSON schema of the metadata differs by Product.asset_type.
When the type is `google.deploymentmanager.Template`, the schema is as
follows:

```
"$schema": http://json-schema.org/draft-04/schema#
type: object
properties:
  name:
    type: string
    minLength: 1
    maxLength: 64
  description:
    type: string
    minLength: 1
    maxLength: 2048
  tagline:
    type: string
    minLength: 1
    maxLength: 100
  support_info:
    type: string
    minLength: 1
    maxLength: 2048
  creator:
    type: string
    minLength: 1
    maxLength: 100
  documentation:
    type: array
    items:
      type: object
      properties:
        url:
          type: string
          pattern:
          "^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]"
        title:
          type: string
          minLength: 1
          maxLength: 64
        description:
          type: string
          minLength: 1
          maxLength: 2048
required:
- name
- description
additionalProperties: false

```

When the asset type is `google.cloudprivatecatalog.ListingOnly`, the schema
is as follows:

```
"$schema": http://json-schema.org/draft-04/schema#
type: object
properties:
  name:
    type: string
    minLength: 1
    maxLength: 64
  description:
    type: string
    minLength: 1
    maxLength: 2048
  tagline:
    type: string
    minLength: 1
    maxLength: 100
  support_info:
    type: string
    minLength: 1
    maxLength: 2048
  creator:
    type: string
    minLength: 1
    maxLength: 100
  documentation:
    type: array
    items:
      type: object
      properties:
        url:
          type: string
          pattern:
          "^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]"
        title:
          type: string
          minLength: 1
          maxLength: 64
        description:
          type: string
          minLength: 1
          maxLength: 2048
  signup_url:
    type: string
    pattern:
    "^(https?)://[-a-zA-Z0-9+&@#/%?=~_|!:,.;]*[-a-zA-Z0-9+&@#/%=~_|]"
required:
- name
- description
- signup_url
additionalProperties: false
``` |
| `create_time` | String | Output only. The time when the product was created. |
| `icon_uri` | String | Output only. The public accessible URI of the icon uploaded by
PrivateCatalogProducer.UploadIcon.

If no icon is uploaded, it will be the default icon's URI. |
| `name` | String | Required. The resource name of the product in the format
`catalogs/{catalog_id}/products/a-z*[a-z0-9]'.

A unique identifier for the product under a catalog, which cannot
be changed after the product is created. The final
segment of the name must between 1 and 256 characters in length. |
| `update_time` | String | Output only. The time when the product was last updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product
product = provider.cloudprivatecatalogproducer_api.Product {
    parent = "value"  # The catalog name of the new product's parent.
}

# Access product outputs
product_id = product.id
product_asset_type = product.asset_type
product_display_metadata = product.display_metadata
product_create_time = product.create_time
product_icon_uri = product.icon_uri
product_name = product.name
product_update_time = product.update_time
```

---


### Version

Creates a Version instance under a given Product.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `original_asset` | HashMap<String, String> |  | The user-supplied asset payload. The maximum size of the payload is 2MB.
The JSON schema of the payload is defined as:

```
type: object
properties:
  mainTemplate:
    type: string
    description: The file name of the main template and name prefix of
    schema file. The content of the main template should be set in the
    imports list. The schema file name is expected to be
    <mainTemplate>.schema in the imports list. required: true
  imports:
    type: array
    description: Import template and schema file contents. Required to have
    both <mainTemplate> and <mainTemplate>.schema files. required: true
    minItems: 2
    items:
      type: object
      properties:
        name:
          type: string
        content:
          type: string
``` |
| `update_time` | String |  | Output only. The time when the version was last updated. |
| `asset` | HashMap<String, String> |  | Output only. The asset which has been validated and is ready to be
provisioned. See Version.original_asset for the schema. |
| `create_time` | String |  | Output only. The time when the version was created. |
| `name` | String |  | Required. The resource name of the version, in the format
`catalogs/{catalog_id}/products/{product_id}/versions/a-z*[a-z0-9]'.

A unique identifier for the version under a product, which can't
be changed after the version is created. The final segment of the name must
between 1 and 63 characters in length. |
| `description` | String |  | The user-supplied description of the version. Maximum of 256 characters. |
| `parent` | String | ✅ | The product name of the new version's parent. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `original_asset` | HashMap<String, String> | The user-supplied asset payload. The maximum size of the payload is 2MB.
The JSON schema of the payload is defined as:

```
type: object
properties:
  mainTemplate:
    type: string
    description: The file name of the main template and name prefix of
    schema file. The content of the main template should be set in the
    imports list. The schema file name is expected to be
    <mainTemplate>.schema in the imports list. required: true
  imports:
    type: array
    description: Import template and schema file contents. Required to have
    both <mainTemplate> and <mainTemplate>.schema files. required: true
    minItems: 2
    items:
      type: object
      properties:
        name:
          type: string
        content:
          type: string
``` |
| `update_time` | String | Output only. The time when the version was last updated. |
| `asset` | HashMap<String, String> | Output only. The asset which has been validated and is ready to be
provisioned. See Version.original_asset for the schema. |
| `create_time` | String | Output only. The time when the version was created. |
| `name` | String | Required. The resource name of the version, in the format
`catalogs/{catalog_id}/products/{product_id}/versions/a-z*[a-z0-9]'.

A unique identifier for the version under a product, which can't
be changed after the version is created. The final segment of the name must
between 1 and 63 characters in length. |
| `description` | String | The user-supplied description of the version. Maximum of 256 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.cloudprivatecatalogproducer_api.Version {
    parent = "value"  # The product name of the new version's parent.
}

# Access version outputs
version_id = version.id
version_original_asset = version.original_asset
version_update_time = version.update_time
version_asset = version.asset
version_create_time = version.create_time
version_name = version.name
version_description = version.description
```

---


### Association

Creates an Association instance under a given Catalog.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `association` | String |  | The user-supplied `Association` that is going to be created.

If the `resource` field is set, the
`privatecatalogproducer.catalogTargets.associate` permission is checked on
the target resource. |
| `parent` | String | ✅ | The `Catalog` resource's name. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource` | String | Required. The user-supplied fully qualified name of the `Resource`
associated to the `Catalog`. It supports `Organization`, `Folder`,
and `Project`. Values are of the form

* `//cloudresourcemanager.googleapis.com/projects/{product_number}`
* `//cloudresourcemanager.googleapis.com/folders/{folder_id}`
* `//cloudresourcemanager.googleapis.com/organizations/{organization_id}` |
| `create_time` | String | The creation time of the association. |
| `name` | String | Output only. The resource name of the catalog association, in the format
`catalogs/{catalog_id}/associations/{association_id}'.

A unique identifier for the catalog association, which is
generated by catalog service. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create association
association = provider.cloudprivatecatalogproducer_api.Association {
    parent = "value"  # The `Catalog` resource's name.
}

# Access association outputs
association_id = association.id
association_resource = association.resource
association_create_time = association.create_time
association_name = association.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple operation resources
operation_0 = provider.cloudprivatecatalogproducer_api.Operation {
    name = "value-0"
}
operation_1 = provider.cloudprivatecatalogproducer_api.Operation {
    name = "value-1"
}
operation_2 = provider.cloudprivatecatalogproducer_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.cloudprivatecatalogproducer_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudprivatecatalogproducer_api Documentation](https://cloud.google.com/cloudprivatecatalogproducer_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
