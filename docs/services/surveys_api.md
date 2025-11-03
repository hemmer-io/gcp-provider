# Surveys_api Service



**Resources**: 2

---

## Overview

The surveys_api service provides access to 2 resource types:

- [Result](#result) [R]
- [Survey](#survey) [CRUD]

---

## Resources


### Result

Retrieves any survey results that have been produced so far. Results are formatted as an Excel file. You must add "?alt=media" to the URL as an argument to get results.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `survey_url_id` | String | External survey ID as viewable by survey owners in the editor view. |
| `status` | String | Human readable string describing the status of the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access result outputs
result_id = result.id
result_survey_url_id = result.survey_url_id
result_status = result.status
```

---


### Survey

Creates a survey.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String |  | Optional name that will be given to the survey. |
| `audience` | String |  | Targeting-criteria message containing demographic information |
| `survey_url_id` | String |  | Unique survey ID, that is viewable in the URL of the Survey Creator UI |
| `questions` | Vec<String> |  | List of questions defining the survey. |
| `wanted_response_count` | i64 |  | Number of responses desired for the survey. |
| `owners` | Vec<String> |  | List of email addresses for survey owners. Must contain at least the address of the user making the API call. |
| `rejection_reason` | String |  | Reason for the survey being rejected. Only present if the survey state is rejected. |
| `description` | String |  | Text description of the survey. |
| `customer_data` | String |  | Additional information to store on behalf of the API consumer and associate with this question. This binary blob is treated as opaque. This field is limited to 64K bytes. |
| `cost` | String |  | Cost to run the survey and collect the necessary number of responses. |
| `state` | String |  | State that the survey is in. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `title` | String | Optional name that will be given to the survey. |
| `audience` | String | Targeting-criteria message containing demographic information |
| `survey_url_id` | String | Unique survey ID, that is viewable in the URL of the Survey Creator UI |
| `questions` | Vec<String> | List of questions defining the survey. |
| `wanted_response_count` | i64 | Number of responses desired for the survey. |
| `owners` | Vec<String> | List of email addresses for survey owners. Must contain at least the address of the user making the API call. |
| `rejection_reason` | String | Reason for the survey being rejected. Only present if the survey state is rejected. |
| `description` | String | Text description of the survey. |
| `customer_data` | String | Additional information to store on behalf of the API consumer and associate with this question. This binary blob is treated as opaque. This field is limited to 64K bytes. |
| `cost` | String | Cost to run the survey and collect the necessary number of responses. |
| `state` | String | State that the survey is in. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create survey
survey = provider.surveys_api.Survey {
}

# Access survey outputs
survey_id = survey.id
survey_title = survey.title
survey_audience = survey.audience
survey_survey_url_id = survey.survey_url_id
survey_questions = survey.questions
survey_wanted_response_count = survey.wanted_response_count
survey_owners = survey.owners
survey_rejection_reason = survey.rejection_reason
survey_description = survey.description
survey_customer_data = survey.customer_data
survey_cost = survey.cost
survey_state = survey.state
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple result resources
result_0 = provider.surveys_api.Result {
}
result_1 = provider.surveys_api.Result {
}
result_2 = provider.surveys_api.Result {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    result = provider.surveys_api.Result {
    }
```

---

## Related Documentation

- [GCP Surveys_api Documentation](https://cloud.google.com/surveys_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
