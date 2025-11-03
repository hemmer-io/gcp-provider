# Commentanalyzer_api Service



**Resources**: 1

---

## Overview

The commentanalyzer_api service provides access to 1 resource type:

- [Comment](#comment) [C]

---

## Resources


### Comment

Suggest comment scores as training data.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attribute_scores` | HashMap<String, String> |  | Attribute scores for the comment. The map keys are attribute names, same as
the requested_attribute field in AnalyzeCommentRequest (for example
"ATTACK_ON_AUTHOR", "INFLAMMATORY", etc.). This field has the same type as
the `attribute_scores` field in AnalyzeCommentResponse.

To specify an overall attribute score for the entire comment as a whole,
use the `summary_score` field of the mapped AttributeScores object. To
specify scores on specific subparts of the comment, use the `span_scores`
field. All SpanScore objects must have begin and end fields set.

All Score objects must be explicitly set (for binary classification, use
the score values 0 and 1). If Score objects don't include a ScoreType,
`PROBABILITY` is assumed.

`attribute_scores` must not be empty. The mapped AttributeScores objects
also must not be empty. An `INVALID_ARGUMENT` error is returned for all
malformed requests. |
| `comment` | String |  | The comment being scored. |
| `languages` | Vec<String> |  | The language(s) of the comment and context. If none are specified, we
attempt to automatically detect the language. Both ISO and BCP-47 language
codes are accepted. |
| `context` | String |  | The context of the comment. |
| `client_token` | String |  | Opaque token that is echoed from the request to the response. |
| `session_id` | String |  | Session ID. Used to join related RPCs into a single session. For example,
an interactive tool that calls both the AnalyzeComment and
SuggestCommentScore RPCs should set all invocations of both RPCs to the
same Session ID, typically a random 64-bit integer. |
| `community_id` | String |  | Optional identifier associating this comment score suggestion with a
particular sub-community. Different communities may have different norms
and rules. Specifying this value enables training community-specific
models. |



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
