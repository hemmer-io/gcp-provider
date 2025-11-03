# Mybusinessqanda_api Service



**Resources**: 2

---

## Overview

The mybusinessqanda_api service provides access to 2 resource types:

- [Answer](#answer) [CRD]
- [Question](#question) [CRUD]

---

## Resources


### Answer

Creates an answer or updates the existing answer written by the user for the specified question. A user can only create one answer per question.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `answer` | String |  | Required. The new answer. |
| `parent` | String | ✅ | Required. The name of the question to write an answer for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_size` | i64 | The total number of answers posted for this question across all pages. |
| `next_page_token` | String | If the number of answers exceeds the requested max page size, this field is populated with a token to fetch the next page of answers on a subsequent call. If there are no more answers, this field is not present in the response. |
| `answers` | Vec<String> | The requested answers. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create answer
answer = provider.mybusinessqanda_api.Answer {
    parent = "value"  # Required. The name of the question to write an answer for.
}

# Access answer outputs
answer_id = answer.id
answer_total_size = answer.total_size
answer_next_page_token = answer.next_page_token
answer_answers = answer.answers
```

---


### Question

Adds a question for the specified location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `author` | String |  | Output only. The author of the question. |
| `total_answer_count` | i64 |  | Output only. The total number of answers posted for this question. |
| `top_answers` | Vec<String> |  | Output only. A list of answers to the question, sorted by upvotes. This may not be a complete list of answers depending on the request parameters (answers_per_question) |
| `name` | String |  | Immutable. The unique name for the question. locations/*/questions/* This field will be ignored if set during question creation. |
| `create_time` | String |  | Output only. The timestamp for when the question was written. |
| `update_time` | String |  | Output only. The timestamp for when the question was last modified. |
| `upvote_count` | i64 |  | Output only. The number of upvotes for the question. |
| `text` | String |  | Required. The text of the question. It should contain at least three words and the total length should be greater than or equal to 10 characters. The maximum length is 4096 characters. |
| `parent` | String | ✅ | Required. The name of the location to write a question for. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_size` | i64 | The total number of questions posted for this location across all pages. |
| `next_page_token` | String | If the number of questions exceeds the requested max page size, this field is populated with a token to fetch the next page of questions on a subsequent call. If there are no more questions, this field is not present in the response. |
| `questions` | Vec<String> | The requested questions, |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create question
question = provider.mybusinessqanda_api.Question {
    parent = "value"  # Required. The name of the location to write a question for.
}

# Access question outputs
question_id = question.id
question_total_size = question.total_size
question_next_page_token = question.next_page_token
question_questions = question.questions
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple answer resources
answer_0 = provider.mybusinessqanda_api.Answer {
    parent = "value-0"
}
answer_1 = provider.mybusinessqanda_api.Answer {
    parent = "value-1"
}
answer_2 = provider.mybusinessqanda_api.Answer {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    answer = provider.mybusinessqanda_api.Answer {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Mybusinessqanda_api Documentation](https://cloud.google.com/mybusinessqanda_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
