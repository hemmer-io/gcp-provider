# Datalabeling_api Service



**Resources**: 15

---

## Overview

The datalabeling_api service provides access to 15 resource types:

- [Annotated_dataset](#annotated_dataset) [RD]
- [Image](#image) [C]
- [Text](#text) [C]
- [Operation](#operation) [RD]
- [Evaluation](#evaluation) [R]
- [Example_comparison](#example_comparison) [C]
- [Example](#example) [R]
- [Data_item](#data_item) [R]
- [Feedback_message](#feedback_message) [CRD]
- [Instruction](#instruction) [CRD]
- [Feedback_thread](#feedback_thread) [RD]
- [Annotation_spec_set](#annotation_spec_set) [CRD]
- [Video](#video) [C]
- [Evaluation_job](#evaluation_job) [CRUD]
- [Dataset](#dataset) [CRD]

---

## Resources


### Annotated_dataset

Gets an annotated dataset by resource name.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | String | Output only. Additional information about AnnotatedDataset. |
| `example_count` | String | Output only. Number of examples in the annotated dataset. |
| `annotation_source` | String | Output only. Source of the annotation. |
| `create_time` | String | Output only. Time the AnnotatedDataset was created. |
| `name` | String | Output only. AnnotatedDataset resource name in format of: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/ {annotated_dataset_id} |
| `annotation_type` | String | Output only. Type of the annotation. It is specified when starting labeling task. |
| `display_name` | String | Output only. The display name of the AnnotatedDataset. It is specified in HumanAnnotationConfig when user starts a labeling task. Maximum of 64 characters. |
| `completed_example_count` | String | Output only. Number of examples that have annotation in the annotated dataset. |
| `label_stats` | String | Output only. Per label statistics. |
| `blocking_resources` | Vec<String> | Output only. The names of any related resources that are blocking changes to the annotated dataset. |
| `description` | String | Output only. The description of the AnnotatedDataset. It is specified in HumanAnnotationConfig when user starts a labeling task. Maximum of 10000 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access annotated_dataset outputs
annotated_dataset_id = annotated_dataset.id
annotated_dataset_metadata = annotated_dataset.metadata
annotated_dataset_example_count = annotated_dataset.example_count
annotated_dataset_annotation_source = annotated_dataset.annotation_source
annotated_dataset_create_time = annotated_dataset.create_time
annotated_dataset_name = annotated_dataset.name
annotated_dataset_annotation_type = annotated_dataset.annotation_type
annotated_dataset_display_name = annotated_dataset.display_name
annotated_dataset_completed_example_count = annotated_dataset.completed_example_count
annotated_dataset_label_stats = annotated_dataset.label_stats
annotated_dataset_blocking_resources = annotated_dataset.blocking_resources
annotated_dataset_description = annotated_dataset.description
```

---


### Image

 Starts a labeling task for image. The type of image labeling task is configured by feature in the request.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `basic_config` | String |  | Required. Basic human annotation config. |
| `segmentation_config` | String |  | Configuration for segmentation task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required. |
| `polyline_config` | String |  | Configuration for polyline task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required. |
| `image_classification_config` | String |  | Configuration for image classification task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required. |
| `feature` | String |  | Required. The type of image labeling task. |
| `bounding_poly_config` | String |  | Configuration for bounding box and bounding poly task. One of image_classification_config, bounding_poly_config, polyline_config and segmentation_config are required. |
| `parent` | String | ✅ | Required. Name of the dataset to request labeling task, format: projects/{project_id}/datasets/{dataset_id} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create image
image = provider.datalabeling_api.Image {
    parent = "value"  # Required. Name of the dataset to request labeling task, format: projects/{project_id}/datasets/{dataset_id}
}

```

---


### Text

Starts a labeling task for text. The type of text labeling task is configured by feature in the request.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `basic_config` | String |  | Required. Basic human annotation config. |
| `text_classification_config` | String |  | Configuration for text classification task. One of text_classification_config and text_entity_extraction_config is required. |
| `text_entity_extraction_config` | String |  | Configuration for entity extraction task. One of text_classification_config and text_entity_extraction_config is required. |
| `feature` | String |  | Required. The type of text labeling task. |
| `parent` | String | ✅ | Required. Name of the data set to request labeling task, format: projects/{project_id}/datasets/{dataset_id} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create text
text = provider.datalabeling_api.Text {
    parent = "value"  # Required. Name of the data set to request labeling task, format: projects/{project_id}/datasets/{dataset_id}
}

```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
```

---


### Evaluation

 Gets an evaluation by resource name (to search, use projects.evaluations.search).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `evaluation_job_run_time` | String | Output only. Timestamp for when the evaluation job that created this evaluation ran. |
| `config` | String | Output only. Options used in the evaluation job that created this evaluation. |
| `evaluation_metrics` | String | Output only. Metrics comparing predictions to ground truth labels. |
| `evaluated_item_count` | String | Output only. The number of items in the ground truth dataset that were used for this evaluation. Only populated when the evaulation is for certain AnnotationTypes. |
| `name` | String | Output only. Resource name of an evaluation. The name has the following format: "projects/{project_id}/datasets/{dataset_id}/evaluations/ {evaluation_id}' |
| `annotation_type` | String | Output only. Type of task that the model version being evaluated performs, as defined in the evaluationJobConfig.inputConfig.annotationType field of the evaluation job that created this evaluation. |
| `create_time` | String | Output only. Timestamp for when this evaluation was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access evaluation outputs
evaluation_id = evaluation.id
evaluation_evaluation_job_run_time = evaluation.evaluation_job_run_time
evaluation_config = evaluation.config
evaluation_evaluation_metrics = evaluation.evaluation_metrics
evaluation_evaluated_item_count = evaluation.evaluated_item_count
evaluation_name = evaluation.name
evaluation_annotation_type = evaluation.annotation_type
evaluation_create_time = evaluation.create_time
```

---


### Example_comparison

Searches example comparisons from an evaluation. The return format is a list of example comparisons that show ground truth and prediction(s) for a single input. Search by providing an evaluation ID.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_size` | i64 |  | Optional. Requested page size. Server may return fewer results than requested. Default value is 100. |
| `page_token` | String |  | Optional. A token identifying a page of results for the server to return. Typically obtained by the nextPageToken of the response to a previous search rquest. If you don't specify this field, the API call requests the first page of the search. |
| `parent` | String | ✅ | Required. Name of the Evaluation resource to search for example comparisons from. Format: "projects/{project_id}/datasets/{dataset_id}/evaluations/ {evaluation_id}" |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create example_comparison
example_comparison = provider.datalabeling_api.Example_comparison {
    parent = "value"  # Required. Name of the Evaluation resource to search for example comparisons from. Format: "projects/{project_id}/datasets/{dataset_id}/evaluations/ {evaluation_id}"
}

```

---


### Example

Gets an example by resource name, including both data and annotation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Name of the example, in format of: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/ {annotated_dataset_id}/examples/{example_id} |
| `image_payload` | String | The image payload, a container of the image bytes/uri. |
| `text_payload` | String | The text payload, a container of the text content. |
| `annotations` | Vec<String> | Output only. Annotations for the piece of data in Example. One piece of data can have multiple annotations. |
| `video_payload` | String | The video payload, a container of the video uri. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access example outputs
example_id = example.id
example_name = example.name
example_image_payload = example.image_payload
example_text_payload = example.text_payload
example_annotations = example.annotations
example_video_payload = example.video_payload
```

---


### Data_item

Gets a data item in a dataset by resource name. This API can be called after data are imported into dataset.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `image_payload` | String | The image payload, a container of the image bytes/uri. |
| `name` | String | Output only. Name of the data item, in format of: projects/{project_id}/datasets/{dataset_id}/dataItems/{data_item_id} |
| `text_payload` | String | The text payload, a container of text content. |
| `video_payload` | String | The video payload, a container of the video uri. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access data_item outputs
data_item_id = data_item.id
data_item_image_payload = data_item.image_payload
data_item_name = data_item.name
data_item_text_payload = data_item.text_payload
data_item_video_payload = data_item.video_payload
```

---


### Feedback_message

Create a FeedbackMessage object.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `body` | String |  | String content of the feedback. Maximum of 10000 characters. |
| `image` | String |  | The image storing this feedback if the feedback is an image representing operator's comments. |
| `operator_feedback_metadata` | String |  |  |
| `requester_feedback_metadata` | String |  |  |
| `name` | String |  | Name of the feedback message in a feedback thread. Format: 'project/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}/feedbackMessage/{feedback_message_id}' |
| `create_time` | String |  | Create time. |
| `parent` | String | ✅ | Required. FeedbackMessage resource parent, format: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `body` | String | String content of the feedback. Maximum of 10000 characters. |
| `image` | String | The image storing this feedback if the feedback is an image representing operator's comments. |
| `operator_feedback_metadata` | String |  |
| `requester_feedback_metadata` | String |  |
| `name` | String | Name of the feedback message in a feedback thread. Format: 'project/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}/feedbackMessage/{feedback_message_id}' |
| `create_time` | String | Create time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create feedback_message
feedback_message = provider.datalabeling_api.Feedback_message {
    parent = "value"  # Required. FeedbackMessage resource parent, format: projects/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}.
}

# Access feedback_message outputs
feedback_message_id = feedback_message.id
feedback_message_body = feedback_message.body
feedback_message_image = feedback_message.image
feedback_message_operator_feedback_metadata = feedback_message.operator_feedback_metadata
feedback_message_requester_feedback_metadata = feedback_message.requester_feedback_metadata
feedback_message_name = feedback_message.name
feedback_message_create_time = feedback_message.create_time
```

---


### Instruction

Creates an instruction for how data should be labeled.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `instruction` | String |  | Required. Instruction of how to perform the labeling task. |
| `parent` | String | ✅ | Required. Instruction resource parent, format: projects/{project_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Creation time of instruction. |
| `update_time` | String | Output only. Last update time of instruction. |
| `blocking_resources` | Vec<String> | Output only. The names of any related resources that are blocking changes to the instruction. |
| `data_type` | String | Required. The data type of this instruction. |
| `name` | String | Output only. Instruction resource name, format: projects/{project_id}/instructions/{instruction_id} |
| `description` | String | Optional. User-provided description of the instruction. The description can be up to 10000 characters long. |
| `display_name` | String | Required. The display name of the instruction. Maximum of 64 characters. |
| `pdf_instruction` | String | Instruction from a PDF document. The PDF should be in a Cloud Storage bucket. |
| `csv_instruction` | String | Deprecated: this instruction format is not supported any more. Instruction from a CSV file, such as for classification task. The CSV file should have exact two columns, in the following format: * The first column is labeled data, such as an image reference, text. * The second column is comma separated labels associated with data. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create instruction
instruction = provider.datalabeling_api.Instruction {
    parent = "value"  # Required. Instruction resource parent, format: projects/{project_id}
}

# Access instruction outputs
instruction_id = instruction.id
instruction_create_time = instruction.create_time
instruction_update_time = instruction.update_time
instruction_blocking_resources = instruction.blocking_resources
instruction_data_type = instruction.data_type
instruction_name = instruction.name
instruction_description = instruction.description
instruction_display_name = instruction.display_name
instruction_pdf_instruction = instruction.pdf_instruction
instruction_csv_instruction = instruction.csv_instruction
```

---


### Feedback_thread

 Get a FeedbackThread object.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Name of the feedback thread. Format: 'project/{project_id}/datasets/{dataset_id}/annotatedDatasets/{annotated_dataset_id}/feedbackThreads/{feedback_thread_id}' |
| `feedback_thread_metadata` | String | Metadata regarding the feedback thread. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access feedback_thread outputs
feedback_thread_id = feedback_thread.id
feedback_thread_name = feedback_thread.name
feedback_thread_feedback_thread_metadata = feedback_thread.feedback_thread_metadata
```

---


### Annotation_spec_set

Creates an annotation spec set by providing a set of labels.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotation_spec_set` | String |  | Required. Annotation spec set to create. Annotation specs must be included. Only one annotation spec will be accepted for annotation specs with same display_name. |
| `parent` | String | ✅ | Required. AnnotationSpecSet resource parent, format: projects/{project_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotation_specs` | Vec<String> | Required. The array of AnnotationSpecs that you define when you create the AnnotationSpecSet. These are the possible labels for the labeling task. |
| `display_name` | String | Required. The display name for AnnotationSpecSet that you define when you create it. Maximum of 64 characters. |
| `description` | String | Optional. User-provided description of the annotation specification set. The description can be up to 10,000 characters long. |
| `blocking_resources` | Vec<String> | Output only. The names of any related resources that are blocking changes to the annotation spec set. |
| `name` | String | Output only. The AnnotationSpecSet resource name in the following format: "projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create annotation_spec_set
annotation_spec_set = provider.datalabeling_api.Annotation_spec_set {
    parent = "value"  # Required. AnnotationSpecSet resource parent, format: projects/{project_id}
}

# Access annotation_spec_set outputs
annotation_spec_set_id = annotation_spec_set.id
annotation_spec_set_annotation_specs = annotation_spec_set.annotation_specs
annotation_spec_set_display_name = annotation_spec_set.display_name
annotation_spec_set_description = annotation_spec_set.description
annotation_spec_set_blocking_resources = annotation_spec_set.blocking_resources
annotation_spec_set_name = annotation_spec_set.name
```

---


### Video

Starts a labeling task for video. The type of video labeling task is configured by feature in the request.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `object_tracking_config` | String |  | Configuration for video object tracking task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required. |
| `video_classification_config` | String |  | Configuration for video classification task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required. |
| `object_detection_config` | String |  | Configuration for video object detection task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required. |
| `basic_config` | String |  | Required. Basic human annotation config. |
| `feature` | String |  | Required. The type of video labeling task. |
| `event_config` | String |  | Configuration for video event task. One of video_classification_config, object_detection_config, object_tracking_config and event_config is required. |
| `parent` | String | ✅ | Required. Name of the dataset to request labeling task, format: projects/{project_id}/datasets/{dataset_id} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create video
video = provider.datalabeling_api.Video {
    parent = "value"  # Required. Name of the dataset to request labeling task, format: projects/{project_id}/datasets/{dataset_id}
}

```

---


### Evaluation_job

 Creates an evaluation job.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `job` | String |  | Required. The evaluation job to create. |
| `parent` | String | ✅ | Required. Evaluation job resource parent. Format: "projects/{project_id}" |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. Describes the current state of the job. |
| `name` | String | Output only. After you create a job, Data Labeling Service assigns a name to the job with the following format: "projects/{project_id}/evaluationJobs/ {evaluation_job_id}" |
| `description` | String | Required. Description of the job. The description can be up to 25,000 characters long. |
| `create_time` | String | Output only. Timestamp of when this evaluation job was created. |
| `evaluation_job_config` | String | Required. Configuration details for the evaluation job. |
| `annotation_spec_set` | String | Required. Name of the AnnotationSpecSet describing all the labels that your machine learning model outputs. You must create this resource before you create an evaluation job and provide its name in the following format: "projects/{project_id}/annotationSpecSets/{annotation_spec_set_id}" |
| `label_missing_ground_truth` | bool | Required. Whether you want Data Labeling Service to provide ground truth labels for prediction input. If you want the service to assign human labelers to annotate your data, set this to `true`. If you want to provide your own ground truth labels in the evaluation job's BigQuery table, set this to `false`. |
| `attempts` | Vec<String> | Output only. Every time the evaluation job runs and an error occurs, the failed attempt is appended to this array. |
| `model_version` | String | Required. The [AI Platform Prediction model version](/ml-engine/docs/prediction-overview) to be evaluated. Prediction input and output is sampled from this model version. When creating an evaluation job, specify the model version in the following format: "projects/{project_id}/models/{model_name}/versions/{version_name}" There can only be one evaluation job per model version. |
| `schedule` | String | Required. Describes the interval at which the job runs. This interval must be at least 1 day, and it is rounded to the nearest day. For example, if you specify a 50-hour interval, the job runs every 2 days. You can provide the schedule in [crontab format](/scheduler/docs/configuring/cron-job-schedules) or in an [English-like format](/appengine/docs/standard/python/config/cronref#schedule_format). Regardless of what you specify, the job will run at 10:00 AM UTC. Only the interval from this schedule is used, not the specific time of day. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create evaluation_job
evaluation_job = provider.datalabeling_api.Evaluation_job {
    parent = "value"  # Required. Evaluation job resource parent. Format: "projects/{project_id}"
}

# Access evaluation_job outputs
evaluation_job_id = evaluation_job.id
evaluation_job_state = evaluation_job.state
evaluation_job_name = evaluation_job.name
evaluation_job_description = evaluation_job.description
evaluation_job_create_time = evaluation_job.create_time
evaluation_job_evaluation_job_config = evaluation_job.evaluation_job_config
evaluation_job_annotation_spec_set = evaluation_job.annotation_spec_set
evaluation_job_label_missing_ground_truth = evaluation_job.label_missing_ground_truth
evaluation_job_attempts = evaluation_job.attempts
evaluation_job_model_version = evaluation_job.model_version
evaluation_job_schedule = evaluation_job.schedule
```

---


### Dataset

 Creates dataset. If success return a Dataset resource.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dataset` | String |  | Required. The dataset to be created. |
| `parent` | String | ✅ | Required. Dataset resource parent, format: projects/{project_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `input_configs` | Vec<String> | Output only. This is populated with the original input configs where ImportData is called. It is available only after the clients import data to this dataset. |
| `data_item_count` | String | Output only. The number of data items in the dataset. |
| `last_migrate_time` | String | Last time that the Dataset is migrated to AI Platform V2. If any of the AnnotatedDataset is migrated, the last_migration_time in Dataset is also updated. |
| `display_name` | String | Required. The display name of the dataset. Maximum of 64 characters. |
| `name` | String | Output only. Dataset resource name, format is: projects/{project_id}/datasets/{dataset_id} |
| `create_time` | String | Output only. Time the dataset is created. |
| `description` | String | Optional. User-provided description of the annotation specification set. The description can be up to 10000 characters long. |
| `blocking_resources` | Vec<String> | Output only. The names of any related resources that are blocking changes to the dataset. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create dataset
dataset = provider.datalabeling_api.Dataset {
    parent = "value"  # Required. Dataset resource parent, format: projects/{project_id}
}

# Access dataset outputs
dataset_id = dataset.id
dataset_input_configs = dataset.input_configs
dataset_data_item_count = dataset.data_item_count
dataset_last_migrate_time = dataset.last_migrate_time
dataset_display_name = dataset.display_name
dataset_name = dataset.name
dataset_create_time = dataset.create_time
dataset_description = dataset.description
dataset_blocking_resources = dataset.blocking_resources
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple annotated_dataset resources
annotated_dataset_0 = provider.datalabeling_api.Annotated_dataset {
}
annotated_dataset_1 = provider.datalabeling_api.Annotated_dataset {
}
annotated_dataset_2 = provider.datalabeling_api.Annotated_dataset {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    annotated_dataset = provider.datalabeling_api.Annotated_dataset {
    }
```

---

## Related Documentation

- [GCP Datalabeling_api Documentation](https://cloud.google.com/datalabeling_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
