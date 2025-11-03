# Factchecktools_api Service



**Resources**: 2

---

## Overview

The factchecktools_api service provides access to 2 resource types:

- [Claim](#claim) [R]
- [Page](#page) [CRUD]

---

## Resources


### Claim

Search through fact-checked claims.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `claims` | Vec<String> | The list of claims and all of their associated information. |
| `next_page_token` | String | The next pagination token in the Search response. It should be used as the `page_token` for the following request. An empty value means no more results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access claim outputs
claim_id = claim.id
claim_claims = claim.claims
claim_next_page_token = claim.next_page_token
```

---


### Page

Create `ClaimReview` markup on a page.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `page_url` | String |  | The URL of the page associated with this `ClaimReview` markup. While every individual `ClaimReview` has its own URL field, semantically this is a page-level field, and each `ClaimReview` on this page will use this value unless individually overridden. Corresponds to `ClaimReview.url` |
| `claim_review_author` | String |  | Info about the author of this claim review. Similar to the above, semantically these are page-level fields, and each `ClaimReview` on this page will contain the same values. |
| `version_id` | String |  | The version ID for this markup. Except for update requests, this field is output-only and should not be set by the user. |
| `name` | String |  | The name of this `ClaimReview` markup page resource, in the form of `pages/{page_id}`. Except for update requests, this field is output-only and should not be set by the user. |
| `publish_date` | String |  | The date when the fact check was published. Similar to the URL, semantically this is a page-level field, and each `ClaimReview` on this page will contain the same value. Corresponds to `ClaimReview.datePublished` |
| `claim_review_markups` | Vec<String> |  | A list of individual claim reviews for this page. Each item in the list corresponds to one `ClaimReview` element. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `page_url` | String | The URL of the page associated with this `ClaimReview` markup. While every individual `ClaimReview` has its own URL field, semantically this is a page-level field, and each `ClaimReview` on this page will use this value unless individually overridden. Corresponds to `ClaimReview.url` |
| `claim_review_author` | String | Info about the author of this claim review. Similar to the above, semantically these are page-level fields, and each `ClaimReview` on this page will contain the same values. |
| `version_id` | String | The version ID for this markup. Except for update requests, this field is output-only and should not be set by the user. |
| `name` | String | The name of this `ClaimReview` markup page resource, in the form of `pages/{page_id}`. Except for update requests, this field is output-only and should not be set by the user. |
| `publish_date` | String | The date when the fact check was published. Similar to the URL, semantically this is a page-level field, and each `ClaimReview` on this page will contain the same value. Corresponds to `ClaimReview.datePublished` |
| `claim_review_markups` | Vec<String> | A list of individual claim reviews for this page. Each item in the list corresponds to one `ClaimReview` element. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create page
page = provider.factchecktools_api.Page {
}

# Access page outputs
page_id = page.id
page_page_url = page.page_url
page_claim_review_author = page.claim_review_author
page_version_id = page.version_id
page_name = page.name
page_publish_date = page.publish_date
page_claim_review_markups = page.claim_review_markups
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple claim resources
claim_0 = provider.factchecktools_api.Claim {
}
claim_1 = provider.factchecktools_api.Claim {
}
claim_2 = provider.factchecktools_api.Claim {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    claim = provider.factchecktools_api.Claim {
    }
```

---

## Related Documentation

- [GCP Factchecktools_api Documentation](https://cloud.google.com/factchecktools_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
