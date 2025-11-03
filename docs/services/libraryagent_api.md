# Libraryagent_api Service



**Resources**: 2

---

## Overview

The libraryagent_api service provides access to 2 resource types:

- [Book](#book) [CR]
- [Shelve](#shelve) [R]

---

## Resources


### Book

Borrow a book from the library. Returns the book if it is borrowed successfully. Returns NOT_FOUND if the book does not exist in the library. Returns quota exceeded error if the amount of books borrowed exceeds allocation quota in any dimensions.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the book to borrow. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `title` | String | The title of the book. |
| `read` | bool | Value indicating whether the book has been read. |
| `author` | String | The name of the book author. |
| `name` | String | The resource name of the book. Book names have the form `shelves/{shelf_id}/books/{book_id}`. The name is ignored when creating a book. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create book
book = provider.libraryagent_api.Book {
    name = "value"  # Required. The name of the book to borrow.
}

# Access book outputs
book_id = book.id
book_title = book.title
book_read = book.read
book_author = book.author
book_name = book.name
```

---


### Shelve

Gets a shelf. Returns NOT_FOUND if the shelf does not exist.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `theme` | String | The theme of the shelf |
| `name` | String | Output only. The resource name of the shelf. Shelf names have the form `shelves/{shelf_id}`. The name is ignored when creating a shelf. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access shelve outputs
shelve_id = shelve.id
shelve_theme = shelve.theme
shelve_name = shelve.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple book resources
book_0 = provider.libraryagent_api.Book {
    name = "value-0"
}
book_1 = provider.libraryagent_api.Book {
    name = "value-1"
}
book_2 = provider.libraryagent_api.Book {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    book = provider.libraryagent_api.Book {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Libraryagent_api Documentation](https://cloud.google.com/libraryagent_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
