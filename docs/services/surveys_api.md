# Surveys_api Service



**Resources**: 2

---

## Overview

The surveys_api service provides access to 2 resource types:

- [Survey](#survey) [CRUD]
- [Result](#result) [R]

---

## Resources


### Survey

Creates a survey.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rejection_reason` | String |  | Reason for the survey being rejected. Only present if the survey state is rejected. |
| `title` | String |  | Optional name that will be given to the survey. |
| `questions` | Vec<String> |  | List of questions defining the survey. |
| `description` | String |  | Text description of the survey. |
| `wanted_response_count` | i64 |  | Number of responses desired for the survey. |
| `audience` | String |  | Targeting-criteria message containing demographic information |
| `survey_url_id` | String |  | Unique survey ID, that is viewable in the URL of the Survey Creator UI |
| `owners` | Vec<String> |  | List of email addresses for survey owners. Must contain at least the address of the user making the API call. |
| `cost` | String |  | Cost to run the survey and collect the necessary number of responses. |
| `customer_data` | String |  | Additional information to store on behalf of the API consumer and associate with this question. This binary blob is treated as opaque. This field is limited to 64K bytes. |
| `state` | String |  | State that the survey is in. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rejection_reason` | String | Reason for the survey being rejected. Only present if the survey state is rejected. |
| `title` | String | Optional name that will be given to the survey. |
| `questions` | Vec<String> | List of questions defining the survey. |
| `description` | String | Text description of the survey. |
| `wanted_response_count` | i64 | Number of responses desired for the survey. |
| `audience` | String | Targeting-criteria message containing demographic information |
| `survey_url_id` | String | Unique survey ID, that is viewable in the URL of the Survey Creator UI |
| `owners` | Vec<String> | List of email addresses for survey owners. Must contain at least the address of the user making the API call. |
| `cost` | String | Cost to run the survey and collect the necessary number of responses. |
| `customer_data` | String | Additional information to store on behalf of the API consumer and associate with this question. This binary blob is treated as opaque. This field is limited to 64K bytes. |
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
survey_rejection_reason = survey.rejection_reason
survey_title = survey.title
survey_questions = survey.questions
survey_description = survey.description
survey_wanted_response_count = survey.wanted_response_count
survey_audience = survey.audience
survey_survey_url_id = survey.survey_url_id
survey_owners = survey.owners
survey_cost = survey.cost
survey_customer_data = survey.customer_data
survey_state = survey.state
```

---


### Result

Retrieves any survey results that have been produced so far. Results are formatted as an Excel file. You must add "?alt=media" to the URL as an argument to get results.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | Human readable string describing the status of the request. |
| `survey_url_id` | String | External survey ID as viewable by survey owners in the editor view. |


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
result_status = result.status
result_survey_url_id = result.survey_url_id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple survey resources
survey_0 = provider.surveys_api.Survey {
}
survey_1 = provider.surveys_api.Survey {
}
survey_2 = provider.surveys_api.Survey {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    survey = provider.surveys_api.Survey {
    }
```

---

## Related Documentation

- [GCP Surveys_api Documentation](https://cloud.google.com/surveys_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
