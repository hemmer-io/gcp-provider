# Streetviewpublish_api Service



**Resources**: 2

---

## Overview

The streetviewpublish_api service provides access to 2 resource types:

- [Photo](#photo) [CRUD]
- [Photo_sequence](#photo_sequence) [CRD]

---

## Resources


### Photo

After the client finishes uploading the photo with the returned UploadRef, CreatePhoto publishes the uploaded Photo to Street View on Google Maps. Currently, the only way to set heading, pitch, and roll in CreatePhoto is through the [Photo Sphere XMP metadata](https://developers.google.com/streetview/spherical-metadata) in the photo bytes. CreatePhoto ignores the `pose.heading`, `pose.pitch`, `pose.roll`, `pose.altitude`, and `pose.level` fields in Pose. This method returns the following error codes: * google.rpc.Code.INVALID_ARGUMENT if the request is malformed or if the uploaded photo is not a 360 photo. * google.rpc.Code.NOT_FOUND if the upload reference does not exist. * google.rpc.Code.RESOURCE_EXHAUSTED if the account has reached the storage limit.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `upload_time` | String |  | Output only. Time when the image was uploaded. |
| `capture_time` | String |  | Optional. Absolute time when the photo was captured. When the photo has no exif timestamp, this is used to set a timestamp in the photo metadata. |
| `photo_id` | String |  | Required. Output only. Required when updating a photo. Output only when creating a photo. Identifier for the photo, which is unique among all photos in Google. |
| `share_link` | String |  | Output only. The share link for the photo. |
| `maps_publish_status` | String |  | Output only. Status in Google Maps, whether this photo was published or rejected. |
| `download_url` | String |  | Output only. The download URL for the photo bytes. This field is set only when GetPhotoRequest.view is set to PhotoView.INCLUDE_DOWNLOAD_URL. |
| `upload_reference` | String |  | Input only. Required when creating a photo. Input only. The resource URL where the photo bytes are uploaded to. |
| `connections` | Vec<String> |  | Optional. Connections to other photos. A connection represents the link from this photo to another photo. |
| `thumbnail_url` | String |  | Output only. The thumbnail URL for showing a preview of the given photo. |
| `pose` | String |  | Optional. Pose of the photo. |
| `transfer_status` | String |  | Output only. Status of rights transfer on this photo. |
| `places` | Vec<String> |  | Optional. Places where this photo belongs. |
| `view_count` | String |  | Output only. View count of the photo. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `upload_time` | String | Output only. Time when the image was uploaded. |
| `capture_time` | String | Optional. Absolute time when the photo was captured. When the photo has no exif timestamp, this is used to set a timestamp in the photo metadata. |
| `photo_id` | String | Required. Output only. Required when updating a photo. Output only when creating a photo. Identifier for the photo, which is unique among all photos in Google. |
| `share_link` | String | Output only. The share link for the photo. |
| `maps_publish_status` | String | Output only. Status in Google Maps, whether this photo was published or rejected. |
| `download_url` | String | Output only. The download URL for the photo bytes. This field is set only when GetPhotoRequest.view is set to PhotoView.INCLUDE_DOWNLOAD_URL. |
| `upload_reference` | String | Input only. Required when creating a photo. Input only. The resource URL where the photo bytes are uploaded to. |
| `connections` | Vec<String> | Optional. Connections to other photos. A connection represents the link from this photo to another photo. |
| `thumbnail_url` | String | Output only. The thumbnail URL for showing a preview of the given photo. |
| `pose` | String | Optional. Pose of the photo. |
| `transfer_status` | String | Output only. Status of rights transfer on this photo. |
| `places` | Vec<String> | Optional. Places where this photo belongs. |
| `view_count` | String | Output only. View count of the photo. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create photo
photo = provider.streetviewpublish_api.Photo {
}

# Access photo outputs
photo_id = photo.id
photo_upload_time = photo.upload_time
photo_capture_time = photo.capture_time
photo_photo_id = photo.photo_id
photo_share_link = photo.share_link
photo_maps_publish_status = photo.maps_publish_status
photo_download_url = photo.download_url
photo_upload_reference = photo.upload_reference
photo_connections = photo.connections
photo_thumbnail_url = photo.thumbnail_url
photo_pose = photo.pose
photo_transfer_status = photo.transfer_status
photo_places = photo.places
photo_view_count = photo.view_count
```

---


### Photo_sequence

After the client finishes uploading the PhotoSequence with the returned UploadRef, CreatePhotoSequence extracts a sequence of 360 photos from a video or Extensible Device Metadata (XDM, http://www.xdm.org/) to be published to Street View on Google Maps. `CreatePhotoSequence` returns an Operation, with the PhotoSequence Id set in the `Operation.name` field. This method returns the following error codes: * google.rpc.Code.INVALID_ARGUMENT if the request is malformed. * google.rpc.Code.NOT_FOUND if the upload reference does not exist.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filename` | String |  | Output only. The filename of the upload. Does not include the directory path. Only available if the sequence was uploaded on a platform that provides the filename. |
| `failure_details` | String |  | Output only. If this sequence has `failure_reason` set, this may contain additional details about the failure. |
| `id` | String |  | Output only. Unique identifier for the photo sequence. This also acts as a long running operation ID if uploading is performed asynchronously. |
| `distance_meters` | f64 |  | Output only. The computed distance of the photo sequence in meters. |
| `raw_gps_timeline` | Vec<String> |  | Input only. Raw GPS measurements with increasing timestamps from the device that aren't time synced with each photo. These raw measurements will be used to infer the pose of each frame. Required in input when InputType is VIDEO and raw GPS measurements are not in Camera Motion Metadata Track (CAMM). User can indicate which takes precedence using gps_source if raw GPS measurements are provided in both raw_gps_timeline and Camera Motion Metadata Track (CAMM). |
| `failure_reason` | String |  | Output only. If this sequence has processing_state = FAILED, this will contain the reason why it failed. If the processing_state is any other value, this field will be unset. |
| `gps_source` | String |  | Input only. If both raw_gps_timeline and the Camera Motion Metadata Track (CAMM) contain GPS measurements, indicate which takes precedence. |
| `photos` | Vec<String> |  | Output only. Photos with increasing timestamps. |
| `sequence_bounds` | String |  | Output only. A rectangular box that encapsulates every image in this photo sequence. |
| `processing_state` | String |  | Output only. The processing state of this sequence. |
| `imu` | String |  | Input only. Three axis IMU data for the collection. If this data is too large to put in the request, then it should be put in the CAMM track for the video. This data always takes precedence over the equivalent CAMM data, if it exists. |
| `view_count` | String |  | Output only. The total number of views that all the published images in this PhotoSequence have received. |
| `capture_time_override` | String |  | Optional. Absolute time when the photo sequence starts to be captured. If the photo sequence is a video, this is the start time of the video. If this field is populated in input, it overrides the capture time in the video or XDM file. |
| `upload_reference` | String |  | Input only. Required when creating photo sequence. The resource name where the bytes of the photo sequence (in the form of video) are uploaded. |
| `upload_time` | String |  | Output only. The time this photo sequence was created in uSV Store service. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create photo_sequence
photo_sequence = provider.streetviewpublish_api.Photo_sequence {
}

# Access photo_sequence outputs
photo_sequence_id = photo_sequence.id
photo_sequence_metadata = photo_sequence.metadata
photo_sequence_response = photo_sequence.response
photo_sequence_name = photo_sequence.name
photo_sequence_done = photo_sequence.done
photo_sequence_error = photo_sequence.error
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple photo resources
photo_0 = provider.streetviewpublish_api.Photo {
}
photo_1 = provider.streetviewpublish_api.Photo {
}
photo_2 = provider.streetviewpublish_api.Photo {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    photo = provider.streetviewpublish_api.Photo {
    }
```

---

## Related Documentation

- [GCP Streetviewpublish_api Documentation](https://cloud.google.com/streetviewpublish_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
