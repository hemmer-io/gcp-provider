# Vision_api Service



**Resources**: 10

---

## Overview

The vision_api service provides access to 10 resource types:

- [Image](#image) [C]
- [Operation](#operation) [CRD]
- [Product_set](#product_set) [CRUD]
- [Product](#product) [CRUD]
- [File](#file) [C]
- [Reference_image](#reference_image) [CRD]
- [Image](#image) [C]
- [File](#file) [C]
- [Image](#image) [C]
- [File](#file) [C]

---

## Resources


### Image

Run image detection and annotation for a batch of images.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. |
| `parent` | String |  | Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`. |
| `requests` | Vec<String> |  | Required. Individual image annotation requests for this batch. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create image
image = provider.vision_api.Image {
}

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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
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
operation = provider.vision_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Product_set

Creates and returns a new ProductSet resource. Possible errors: * Returns INVALID_ARGUMENT if display_name is missing, or is longer than 4096 characters.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `index_error` | String |  | Output only. If there was an error with indexing the product set, the field is populated. This field is ignored when creating a ProductSet. |
| `display_name` | String |  | The user-provided name for this ProductSet. Must not be empty. Must be at most 4096 characters long. |
| `index_time` | String |  | Output only. The time at which this ProductSet was last indexed. Query results will reflect all updates before this time. If this ProductSet has never been indexed, this timestamp is the default value "1970-01-01T00:00:00Z". This field is ignored when creating a ProductSet. |
| `name` | String |  | The resource name of the ProductSet. Format is: `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`. This field is ignored when creating a ProductSet. |
| `parent` | String | ✅ | Required. The project in which the ProductSet should be created. Format is `projects/PROJECT_ID/locations/LOC_ID`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `index_error` | String | Output only. If there was an error with indexing the product set, the field is populated. This field is ignored when creating a ProductSet. |
| `display_name` | String | The user-provided name for this ProductSet. Must not be empty. Must be at most 4096 characters long. |
| `index_time` | String | Output only. The time at which this ProductSet was last indexed. Query results will reflect all updates before this time. If this ProductSet has never been indexed, this timestamp is the default value "1970-01-01T00:00:00Z". This field is ignored when creating a ProductSet. |
| `name` | String | The resource name of the ProductSet. Format is: `projects/PROJECT_ID/locations/LOC_ID/productSets/PRODUCT_SET_ID`. This field is ignored when creating a ProductSet. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product_set
product_set = provider.vision_api.Product_set {
    parent = "value"  # Required. The project in which the ProductSet should be created. Format is `projects/PROJECT_ID/locations/LOC_ID`.
}

# Access product_set outputs
product_set_id = product_set.id
product_set_index_error = product_set.index_error
product_set_display_name = product_set.display_name
product_set_index_time = product_set.index_time
product_set_name = product_set.name
```

---


### Product

Creates and returns a new product resource. Possible errors: * Returns INVALID_ARGUMENT if display_name is missing or longer than 4096 characters. * Returns INVALID_ARGUMENT if description is longer than 4096 characters. * Returns INVALID_ARGUMENT if product_category is missing or invalid.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The user-provided name for this Product. Must not be empty. Must be at most 4096 characters long. |
| `name` | String |  | The resource name of the product. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`. This field is ignored when creating a product. |
| `description` | String |  | User-provided metadata to be stored with this product. Must be at most 4096 characters long. |
| `product_labels` | Vec<String> |  | Key-value pairs that can be attached to a product. At query time, constraints can be specified based on the product_labels. Note that integer values can be provided as strings, e.g. "1199". Only strings with integer values can match a range-based restriction which is to be supported soon. Multiple values can be assigned to the same key. One product may have up to 500 product_labels. Notice that the total number of distinct product_labels over all products in one ProductSet cannot exceed 1M, otherwise the product search pipeline will refuse to work for that ProductSet. |
| `product_category` | String |  | Immutable. The category for the product identified by the reference image. This should be one of "homegoods-v2", "apparel-v2", "toys-v2", "packagedgoods-v1" or "general-v1". The legacy categories "homegoods", "apparel", and "toys" are still supported, but these should not be used for new products. |
| `parent` | String | ✅ | Required. The project in which the Product should be created. Format is `projects/PROJECT_ID/locations/LOC_ID`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The user-provided name for this Product. Must not be empty. Must be at most 4096 characters long. |
| `name` | String | The resource name of the product. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`. This field is ignored when creating a product. |
| `description` | String | User-provided metadata to be stored with this product. Must be at most 4096 characters long. |
| `product_labels` | Vec<String> | Key-value pairs that can be attached to a product. At query time, constraints can be specified based on the product_labels. Note that integer values can be provided as strings, e.g. "1199". Only strings with integer values can match a range-based restriction which is to be supported soon. Multiple values can be assigned to the same key. One product may have up to 500 product_labels. Notice that the total number of distinct product_labels over all products in one ProductSet cannot exceed 1M, otherwise the product search pipeline will refuse to work for that ProductSet. |
| `product_category` | String | Immutable. The category for the product identified by the reference image. This should be one of "homegoods-v2", "apparel-v2", "toys-v2", "packagedgoods-v1" or "general-v1". The legacy categories "homegoods", "apparel", and "toys" are still supported, but these should not be used for new products. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product
product = provider.vision_api.Product {
    parent = "value"  # Required. The project in which the Product should be created. Format is `projects/PROJECT_ID/locations/LOC_ID`.
}

# Access product outputs
product_id = product.id
product_display_name = product.display_name
product_name = product.name
product_description = product.description
product_product_labels = product.product_labels
product_product_category = product.product_category
```

---


### File

Run asynchronous image detection and annotation for a list of generic files, such as PDF files, which may contain multiple pages and multiple images per page. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. |
| `parent` | String |  | Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`. |
| `requests` | Vec<String> |  | Required. Individual async file annotation requests for this batch. |
| `parent` | String | ✅ | Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create file
file = provider.vision_api.File {
    parent = "value"  # Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`.
}

```

---


### Reference_image

Creates and returns a new ReferenceImage resource. The `bounding_poly` field is optional. If `bounding_poly` is not specified, the system will try to detect regions of interest in the image that are compatible with the product_category on the parent product. If it is specified, detection is ALWAYS skipped. The system converts polygons into non-rotated rectangles. Note that the pipeline will resize the image if the image resolution is too large to process (above 50MP). Possible errors: * Returns INVALID_ARGUMENT if the image_uri is missing or longer than 4096 characters. * Returns INVALID_ARGUMENT if the product does not exist. * Returns INVALID_ARGUMENT if bounding_poly is not provided, and nothing compatible with the parent product's product_category is detected. * Returns INVALID_ARGUMENT if bounding_poly contains more than 10 polygons.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uri` | String |  | Required. The Google Cloud Storage URI of the reference image. The URI must start with `gs://`. |
| `bounding_polys` | Vec<String> |  | Optional. Bounding polygons around the areas of interest in the reference image. If this field is empty, the system will try to detect regions of interest. At most 10 bounding polygons will be used. The provided shape is converted into a non-rotated rectangle. Once converted, the small edge of the rectangle must be greater than or equal to 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5 is not). |
| `name` | String |  | The resource name of the reference image. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`. This field is ignored when creating a reference image. |
| `parent` | String | ✅ | Required. Resource name of the product in which to create the reference image. Format is `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `uri` | String | Required. The Google Cloud Storage URI of the reference image. The URI must start with `gs://`. |
| `bounding_polys` | Vec<String> | Optional. Bounding polygons around the areas of interest in the reference image. If this field is empty, the system will try to detect regions of interest. At most 10 bounding polygons will be used. The provided shape is converted into a non-rotated rectangle. Once converted, the small edge of the rectangle must be greater than or equal to 300 pixels. The aspect ratio must be 1:4 or less (i.e. 1:3 is ok; 1:5 is not). |
| `name` | String | The resource name of the reference image. Format is: `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID/referenceImages/IMAGE_ID`. This field is ignored when creating a reference image. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reference_image
reference_image = provider.vision_api.Reference_image {
    parent = "value"  # Required. Resource name of the product in which to create the reference image. Format is `projects/PROJECT_ID/locations/LOC_ID/products/PRODUCT_ID`.
}

# Access reference_image outputs
reference_image_id = reference_image.id
reference_image_uri = reference_image.uri
reference_image_bounding_polys = reference_image.bounding_polys
reference_image_name = reference_image.name
```

---


### Image

Run image detection and annotation for a batch of images.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. Individual image annotation requests for this batch. |
| `parent` | String |  | Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create image
image = provider.vision_api.Image {
}

```

---


### File

Run asynchronous image detection and annotation for a list of generic files, such as PDF files, which may contain multiple pages and multiple images per page. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateFilesResponse` (results).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. Individual async file annotation requests for this batch. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. |
| `parent` | String |  | Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`. |
| `parent` | String | ✅ | Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create file
file = provider.vision_api.File {
    parent = "value"  # Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`.
}

```

---


### Image

Run asynchronous image detection and annotation for a list of images. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `OperationMetadata` (metadata). `Operation.response` contains `AsyncBatchAnnotateImagesResponse` (results). This service will write image annotation outputs to json files in customer GCS bucket, each json file containing BatchAnnotateImagesResponse proto.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String |  | Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. |
| `output_config` | String |  | Required. The desired output location and metadata (e.g. format). |
| `requests` | Vec<String> |  | Required. Individual image annotation requests for this batch. |
| `parent` | String | ✅ | Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create image
image = provider.vision_api.Image {
    parent = "value"  # Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`.
}

```

---


### File

Service that performs image detection and annotation for a batch of files. Now only "application/pdf", "image/tiff" and "image/gif" are supported. This service will extract at most 5 (customers can specify which 5 in AnnotateFileRequest.pages) frames (gif) or pages (pdf or tiff) from each file provided and perform detection and annotation for each image extracted.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. The list of file annotation requests. Right now we support only one AnnotateFileRequest in BatchAnnotateFilesRequest. |
| `labels` | HashMap<String, String> |  | Optional. The labels with user-defined metadata for the request. Label keys and values can be no longer than 63 characters (Unicode codepoints), can only contain lowercase letters, numeric characters, underscores and dashes. International characters are allowed. Label values are optional. Label keys must start with a letter. |
| `parent` | String |  | Optional. Target project and location to make a call. Format: `projects/{project-id}/locations/{location-id}`. If no parent is specified, a region will be chosen automatically. Supported location-ids: `us`: USA country only, `asia`: East asia areas, like Japan, Taiwan, `eu`: The European Union. Example: `projects/project-A/locations/eu`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create file
file = provider.vision_api.File {
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

# Create multiple image resources
image_0 = provider.vision_api.Image {
}
image_1 = provider.vision_api.Image {
}
image_2 = provider.vision_api.Image {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    image = provider.vision_api.Image {
    }
```

---

## Related Documentation

- [GCP Vision_api Documentation](https://cloud.google.com/vision_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
