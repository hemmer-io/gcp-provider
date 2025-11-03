# Css_api Service



**Resources**: 5

---

## Overview

The css_api service provides access to 5 resource types:

- [Quota](#quota) [R]
- [Css_product](#css_product) [R]
- [Css_product_input](#css_product_input) [CUD]
- [Label](#label) [CRUD]
- [Account](#account) [CR]

---

## Resources


### Quota

Lists the daily call quota and usage per group for your CSS Center account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `quota_groups` | Vec<String> | The methods, current quota usage and limits per each group. The quota is shared between all methods in the group. The groups are sorted in descending order based on quota_usage. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access quota outputs
quota_id = quota.id
quota_next_page_token = quota.next_page_token
quota_quota_groups = quota.quota_groups
```

---


### Css_product

Retrieves the processed CSS Product from your CSS Center account. After inserting, updating, or deleting a product input, it may take several minutes before the updated final product can be retrieved.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_attributes` | Vec<String> | Output only. A list of custom (CSS-provided) attributes. It can also be used to submit any attribute of the feed specification in its generic form (for example, `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google. |
| `raw_provided_id` | String | Output only. Your unique raw identifier for the product. |
| `css_product_status` | String | Output only. The status of a product, data validation issues, that is, information about a product computed asynchronously. |
| `attributes` | String | Output only. A list of product attributes. |
| `content_language` | String | Output only. The two-letter [ISO 639-1](http://en.wikipedia.org/wiki/ISO_639-1) language code for the product. |
| `feed_label` | String | Output only. The feed label for the product. |
| `name` | String | The name of the CSS Product. Format: `"accounts/{account}/cssProducts/{css_product}"` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access css_product outputs
css_product_id = css_product.id
css_product_custom_attributes = css_product.custom_attributes
css_product_raw_provided_id = css_product.raw_provided_id
css_product_css_product_status = css_product.css_product_status
css_product_attributes = css_product.attributes
css_product_content_language = css_product.content_language
css_product_feed_label = css_product.feed_label
css_product_name = css_product.name
```

---


### Css_product_input

Uploads a CssProductInput to your CSS Center account. If an input with the same contentLanguage, identity, feedLabel and feedId already exists, this method replaces that entry. After inserting, updating, or deleting a CSS Product input, it may take several minutes before the processed CSS Product can be retrieved.

**Operations**: ✅ Create ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content_language` | String |  | Required. The two-letter [ISO 639-1](http://en.wikipedia.org/wiki/ISO_639-1) language code for the CSS Product. |
| `feed_label` | String |  | Required. The [feed label](https://developers.google.com/shopping-content/guides/products/feed-labels) for the CSS Product. Feed Label is synonymous to "target country" and hence should always be a valid region code. For example: 'DE' for Germany, 'FR' for France. |
| `name` | String |  | Identifier. The name of the CSS Product input. Format: `accounts/{account}/cssProductInputs/{css_product_input}`, where the last section `css_product_input` consists of 3 parts: contentLanguage~feedLabel~offerId. Example: accounts/123/cssProductInputs/de~DE~rawProvidedId123 |
| `custom_attributes` | Vec<String> |  | A list of custom (CSS-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (for example: `{ "name": "size type", "value": "regular" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google. |
| `raw_provided_id` | String |  | Required. Your unique identifier for the CSS Product. This is the same for the CSS Product input and processed CSS Product. We only allow ids with alphanumerics, underscores and dashes. See the [products feed specification](https://support.google.com/merchants/answer/188494#id) for details. |
| `freshness_time` | String |  | DEPRECATED. Use expiration_date instead. Represents the existing version (freshness) of the CSS Product, which can be used to preserve the right order when multiple updates are done at the same time. This field must not be set to the future time. If set, the update is prevented if a newer version of the item already exists in our system (that is the last update time of the existing CSS products is later than the freshness time set in the update). If the update happens, the last update time is then set to this freshness time. If not set, the update will not be prevented and the last update time will default to when this request was received by the CSS API. If the operation is prevented, the aborted exception will be thrown. |
| `final_name` | String |  | Output only. The name of the processed CSS Product. Format: `accounts/{account}/cssProducts/{css_product}` " |
| `attributes` | String |  | A list of CSS Product attributes. |
| `parent` | String | ✅ | Required. The account where this CSS Product will be inserted. Format: accounts/{account} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create css_product_input
css_product_input = provider.css_api.Css_product_input {
    parent = "value"  # Required. The account where this CSS Product will be inserted. Format: accounts/{account}
}

```

---


### Label

Creates a new label, not assigned to any account.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `label_type` | String |  | Output only. The type of this label. |
| `description` | String |  | The description of this label. |
| `label_id` | String |  | Output only. The ID of the label. |
| `name` | String |  | Identifier. The resource name of the label. Format: accounts/{account}/labels/{label} |
| `account_id` | String |  | Output only. The ID of account this label belongs to. |
| `display_name` | String |  | The display name of this label. |
| `parent` | String | ✅ | Required. The parent account. Format: accounts/{account} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_labels` | Vec<String> | The labels from the specified account. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create label
label = provider.css_api.Label {
    parent = "value"  # Required. The parent account. Format: accounts/{account}
}

# Access label outputs
label_id = label.id
label_account_labels = label.account_labels
label_next_page_token = label.next_page_token
```

---


### Account

Updates labels assigned to CSS/MC accounts by a CSS domain.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `parent` | String |  | Optional. Only required when updating MC account labels. The CSS domain that is the parent resource of the MC account. Format: accounts/{account} |
| `label_ids` | Vec<String> |  | The list of label IDs to overwrite the existing account label IDs. If the list is empty, all currently assigned label IDs will be deleted. |
| `name` | String | ✅ | Required. The label resource name. Format: accounts/{account} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `automatic_label_ids` | Vec<String> | Automatically created label IDs assigned to the MC account by CSS Center. |
| `account_type` | String | Output only. The type of this account. |
| `parent` | String | The CSS/MC account's parent resource. CSS group for CSS domains; CSS domain for MC accounts. Returned only if the user has access to the parent account. Note: For MC sub-accounts, this is also the CSS domain that is the parent resource of the MCA account, since we are effectively flattening the hierarchy." |
| `display_name` | String | The CSS/MC account's short display name. |
| `homepage_uri` | String | Output only. Immutable. The CSS/MC account's homepage. |
| `label_ids` | Vec<String> | Manually created label IDs assigned to the CSS/MC account by a CSS parent account. |
| `name` | String | The label resource name. Format: accounts/{account} |
| `full_name` | String | Output only. Immutable. The CSS/MC account's full name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.css_api.Account {
    name = "value"  # Required. The label resource name. Format: accounts/{account}
}

# Access account outputs
account_id = account.id
account_automatic_label_ids = account.automatic_label_ids
account_account_type = account.account_type
account_parent = account.parent
account_display_name = account.display_name
account_homepage_uri = account.homepage_uri
account_label_ids = account.label_ids
account_name = account.name
account_full_name = account.full_name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple quota resources
quota_0 = provider.css_api.Quota {
}
quota_1 = provider.css_api.Quota {
}
quota_2 = provider.css_api.Quota {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    quota = provider.css_api.Quota {
    }
```

---

## Related Documentation

- [GCP Css_api Documentation](https://cloud.google.com/css_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
