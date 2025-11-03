# Manufacturers_api Service



**Resources**: 2

---

## Overview

The manufacturers_api service provides access to 2 resource types:

- [Product_certification](#product_certification) [RUD]
- [Product](#product) [RUD]

---

## Resources


### Product_certification

Gets a product certification by its name. This method can only be called by certification bodies.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `certification` | Vec<String> |  | Required. A list of certifications to link to the described product. |
| `product_type` | Vec<String> |  | Optional. These are your own product categorization system in your product data. |
| `mpn` | Vec<String> |  | Optional. These are the Manufacturer Part Numbers (MPN). MPNs are used to uniquely identify a specific product among all products from the same manufacturer |
| `destination_statuses` | Vec<String> |  | Output only. The statuses of the destinations. |
| `country_code` | Vec<String> |  | Optional. A 2-letter country code (ISO 3166-1 Alpha 2). |
| `brand` | String |  | Required. This is the product's brand name. The brand is used to help identify your product. |
| `issues` | Vec<String> |  | Output only. A server-generated list of issues associated with the product. |
| `name` | String |  | Required. The unique name identifier of a product certification Format: accounts/{account}/languages/{language_code}/productCertifications/{id} Where `id` is a some unique identifier and `language_code` is a 2-letter ISO 639-1 code of a Shopping supported language according to https://support.google.com/merchants/answer/160637. |
| `product_code` | Vec<String> |  | Optional. Another name for GTIN. |
| `title` | String |  | Required. This is to clearly identify the product you are certifying. |
| `name` | String | ✅ | Required. The unique name identifier of a product certification Format: accounts/{account}/languages/{language_code}/productCertifications/{id} Where `id` is a some unique identifier and `language_code` is a 2-letter ISO 639-1 code of a Shopping supported language according to https://support.google.com/merchants/answer/160637. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certification` | Vec<String> | Required. A list of certifications to link to the described product. |
| `product_type` | Vec<String> | Optional. These are your own product categorization system in your product data. |
| `mpn` | Vec<String> | Optional. These are the Manufacturer Part Numbers (MPN). MPNs are used to uniquely identify a specific product among all products from the same manufacturer |
| `destination_statuses` | Vec<String> | Output only. The statuses of the destinations. |
| `country_code` | Vec<String> | Optional. A 2-letter country code (ISO 3166-1 Alpha 2). |
| `brand` | String | Required. This is the product's brand name. The brand is used to help identify your product. |
| `issues` | Vec<String> | Output only. A server-generated list of issues associated with the product. |
| `name` | String | Required. The unique name identifier of a product certification Format: accounts/{account}/languages/{language_code}/productCertifications/{id} Where `id` is a some unique identifier and `language_code` is a 2-letter ISO 639-1 code of a Shopping supported language according to https://support.google.com/merchants/answer/160637. |
| `product_code` | Vec<String> | Optional. Another name for GTIN. |
| `title` | String | Required. This is to clearly identify the product you are certifying. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access product_certification outputs
product_certification_id = product_certification.id
product_certification_certification = product_certification.certification
product_certification_product_type = product_certification.product_type
product_certification_mpn = product_certification.mpn
product_certification_destination_statuses = product_certification.destination_statuses
product_certification_country_code = product_certification.country_code
product_certification_brand = product_certification.brand
product_certification_issues = product_certification.issues
product_certification_name = product_certification.name
product_certification_product_code = product_certification.product_code
product_certification_title = product_certification.title
```

---


### Product

Gets the product from a Manufacturer Center account, including product issues. A recently updated product takes around 15 minutes to process. Changes are only visible after it has been processed. While some issues may be available once the product has been processed, other issues may take days to appear.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rich_product_content` | Vec<String> |  | Rich product content. For more information, see https://support.google.com/manufacturers/answer/9389865 |
| `capacity` | String |  | The capacity of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#capacity. |
| `pattern` | String |  | The pattern of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#pattern. |
| `included_destination` | Vec<String> |  | A list of included destinations such as "ClientExport", "ClientShoppingCatalog" or "PartnerShoppingCatalog". For more information, see https://support.google.com/manufacturers/answer/7443550 |
| `additional_image_link` | Vec<String> |  | The additional images of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#addlimage. |
| `product_highlight` | Vec<String> |  | The product highlights. For more information, see https://support.google.com/manufacturers/answer/10066942 |
| `certification` | Vec<String> |  | Optional. List of certifications claimed by this product. |
| `feature_description` | Vec<String> |  | The rich format description of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#featuredesc. |
| `gender` | String |  | The target gender of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#gender. |
| `excluded_destination` | Vec<String> |  | A list of excluded destinations such as "ClientExport", "ClientShoppingCatalog" or "PartnerShoppingCatalog". For more information, see https://support.google.com/manufacturers/answer/7443550 |
| `product_detail` | Vec<String> |  | The details of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productdetail. |
| `product_type` | Vec<String> |  | The type or category of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#producttype. |
| `size_system` | String |  | The size system of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#sizesystem. |
| `scent` | String |  | The scent of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#scent. |
| `nutrition` | String |  | Nutrition Attributes. See more at https://support.google.com/manufacturers/answer/12098458#food-servings. |
| `suggested_retail_price` | String |  | The suggested retail price (MSRP) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#price. |
| `mpn` | String |  | The Manufacturer Part Number (MPN) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#mpn. |
| `grocery` | String |  | Grocery Attributes. See more at https://support.google.com/manufacturers/answer/12098458#grocery. |
| `product_page_url` | String |  | The URL of the detail page of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productpage. |
| `size` | String |  | The size of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#size. |
| `size_type` | Vec<String> |  | The size type of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#sizetype. |
| `count` | String |  | The count of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#count. |
| `image_link` | String |  | The image of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#image. |
| `intended_country` | Vec<String> |  | Optional. List of countries to show this product in. Countries provided in this attribute will override any of the countries configured at feed level. The values should be: the [CLDR territory code](http://www.unicode.org/repos/cldr/tags/latest/common/main/en.xml) of the countries in which this item will be shown. |
| `brand` | String |  | The brand name of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#brand. |
| `video_link` | Vec<String> |  | The videos of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#video. |
| `product_name` | String |  | The canonical name of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productname. |
| `gtin` | Vec<String> |  | The Global Trade Item Number (GTIN) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#gtin. |
| `age_group` | String |  | The target age group of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#agegroup. |
| `item_group_id` | String |  | The item group id of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#itemgroupid. |
| `theme` | String |  | The theme of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#theme. |
| `disclosure_date` | String |  | The disclosure date of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#disclosure. |
| `description` | String |  | The description of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#description. |
| `title` | String |  | The title of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#title. |
| `release_date` | String |  | The release date of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#release. |
| `product_line` | String |  | The name of the group of products related to the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productline. |
| `color` | String |  | The color of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#color. |
| `flavor` | String |  | The flavor of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#flavor. |
| `virtual_model_link` | String |  | Virtual Model (3d) asset link. |
| `format` | String |  | The format of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#format. |
| `target_client_id` | String |  | The target client id. Should only be used in the accounts of the data partners. For more information, see https://support.google.com/manufacturers/answer/10857344 |
| `material` | String |  | The material of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#material. |
| `name` | String | ✅ | Name in the format `{target_country}:{content_language}:{product_id}`. `target_country` - The target country of the product as a CLDR territory code (for example, US). `content_language` - The content language of the product as a two-letter ISO 639-1 language code (for example, en). `product_id` - The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id. |
| `parent` | String | ✅ | Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `issues` | Vec<String> | A server-generated list of issues associated with the product. |
| `product_id` | String | The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id. |
| `attributes` | String | Attributes of the product uploaded to the Manufacturer Center. Manually edited attributes are taken into account. |
| `target_country` | String | The target country of the product as a CLDR territory code (for example, US). |
| `name` | String | Name in the format `{target_country}:{content_language}:{product_id}`. `target_country` - The target country of the product as a CLDR territory code (for example, US). `content_language` - The content language of the product as a two-letter ISO 639-1 language code (for example, en). `product_id` - The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id. |
| `destination_statuses` | Vec<String> | The status of the destinations. |
| `feed_label` | String | Optional. The feed label for the product. |
| `content_language` | String | The content language of the product as a two-letter ISO 639-1 language code (for example, en). |
| `parent` | String | Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access product outputs
product_id = product.id
product_issues = product.issues
product_product_id = product.product_id
product_attributes = product.attributes
product_target_country = product.target_country
product_name = product.name
product_destination_statuses = product.destination_statuses
product_feed_label = product.feed_label
product_content_language = product.content_language
product_parent = product.parent
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple product_certification resources
product_certification_0 = provider.manufacturers_api.Product_certification {
    name = "value-0"
}
product_certification_1 = provider.manufacturers_api.Product_certification {
    name = "value-1"
}
product_certification_2 = provider.manufacturers_api.Product_certification {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    product_certification = provider.manufacturers_api.Product_certification {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Manufacturers_api Documentation](https://cloud.google.com/manufacturers_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
