# Commentanalyzer_api Service



**Resources**: 1

---

## Overview

The commentanalyzer_api service provides access to 1 resource type:

- [Comment](#comment) [C]

---

## Resources


### Comment

Analyzes the provided text and returns scores for requested attributes.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session_id` | String |  | Session ID. Used to join related RPCs into a single session. For example,
an interactive tool that calls both the AnalyzeComment and
SuggestCommentScore RPCs should set all invocations of both RPCs to the
same Session ID, typically a random 64-bit integer. |
| `span_annotations` | bool |  | An advisory parameter that will return span annotations if the model
is capable of providing scores with sub-comment resolution. This will
likely increase the size of the returned message. |
| `languages` | Vec<String> |  | The language(s) of the comment and context. If none are specified, we
attempt to automatically detect the language. Specifying multiple languages
means the text contains multiple lanugages. Both ISO and BCP-47 language
codes are accepted.

The server returns an error if no language was specified and language
detection fails. The server also returns an error if the languages (either
specified by the caller, or auto-detected) are not *all* supported by the
service. |
| `requested_attributes` | HashMap<String, String> |  | Specification of requested attributes. The AttributeParameters serve as
configuration for each associated attribute. The map keys are attribute
names. The available attributes may be different on each RFE installation,
and can be seen by calling ListAttributes (see above).
For the prod installation, known as Perspective API, at
blade:commentanalyzer-esf and commentanalyzer.googleapis.com, see
go/checker-models (internal) and
https://github.com/conversationai/perspectiveapi/blob/master/2-api/models.md#all-attribute-types. |
| `client_token` | String |  | Opaque token that is echoed from the request to the response. |
| `comment` | String |  | The comment to analyze. |
| `community_id` | String |  | Optional identifier associating this AnalyzeCommentRequest with a
particular client's community. Different communities may have different
norms and rules. Specifying this value enables us to explore building
community-specific models for clients. |
| `context` | String |  | The context of the comment. |
| `do_not_store` | bool |  | Do not store the comment or context sent in this request. By default, the
service may store comments/context for debugging purposes. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create comment
comment = provider.commentanalyzer_api.Comment {
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

# Create multiple comment resources
comment_0 = provider.commentanalyzer_api.Comment {
}
comment_1 = provider.commentanalyzer_api.Comment {
}
comment_2 = provider.commentanalyzer_api.Comment {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    comment = provider.commentanalyzer_api.Comment {
    }
```

---

## Related Documentation

- [GCP Commentanalyzer_api Documentation](https://cloud.google.com/commentanalyzer_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
