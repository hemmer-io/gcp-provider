# Licensing_api Service



**Resources**: 1

---

## Overview

The licensing_api service provides access to 1 resource type:

- [License_assignment](#license_assignment) [CRUD]

---

## Resources


### License_assignment

Assign a license.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String |  | Email id of the user |
| `product_id` | String | ✅ | A product's unique identifier. For more information about products in this version of the API, see Products and SKUs. |
| `sku_id` | String | ✅ | A product SKU's unique identifier. For more information about available SKUs in this version of the API, see Products and SKUs. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | Link to this page. |
| `product_name` | String | Display Name of the product. |
| `product_id` | String | A product's unique identifier. For more information about products in this version of the API, see Product and SKU IDs. |
| `etags` | String | ETag of the resource. |
| `sku_name` | String | Display Name of the sku of the product. |
| `user_id` | String | The user's current primary email address. If the user's email address changes, use the new email address in your API requests. Since a `userId` is subject to change, do not use a `userId` value as a key for persistent data. This key could break if the current user's email address changes. If the `userId` is suspended, the license status changes. |
| `kind` | String | Identifies the resource as a LicenseAssignment, which is `licensing#licenseAssignment`. |
| `sku_id` | String | A product SKU's unique identifier. For more information about available SKUs in this version of the API, see Products and SKUs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create license_assignment
license_assignment = provider.licensing_api.License_assignment {
    product_id = "value"  # A product's unique identifier. For more information about products in this version of the API, see Products and SKUs.
    sku_id = "value"  # A product SKU's unique identifier. For more information about available SKUs in this version of the API, see Products and SKUs.
}

# Access license_assignment outputs
license_assignment_id = license_assignment.id
license_assignment_self_link = license_assignment.self_link
license_assignment_product_name = license_assignment.product_name
license_assignment_product_id = license_assignment.product_id
license_assignment_etags = license_assignment.etags
license_assignment_sku_name = license_assignment.sku_name
license_assignment_user_id = license_assignment.user_id
license_assignment_kind = license_assignment.kind
license_assignment_sku_id = license_assignment.sku_id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple license_assignment resources
license_assignment_0 = provider.licensing_api.License_assignment {
    product_id = "value-0"
    sku_id = "value-0"
}
license_assignment_1 = provider.licensing_api.License_assignment {
    product_id = "value-1"
    sku_id = "value-1"
}
license_assignment_2 = provider.licensing_api.License_assignment {
    product_id = "value-2"
    sku_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    license_assignment = provider.licensing_api.License_assignment {
        product_id = "production-value"
        sku_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Licensing_api Documentation](https://cloud.google.com/licensing_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
