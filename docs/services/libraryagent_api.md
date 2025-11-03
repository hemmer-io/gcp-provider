# Libraryagent_api Service



**Resources**: 2

---

## Overview

The libraryagent_api service provides access to 2 resource types:

- [Shelve](#shelve) [R]
- [Book](#book) [CR]

---

## Resources


### Shelve

Gets a shelf. Returns NOT_FOUND if the shelf does not exist.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name of the shelf. Shelf names have the form `shelves/{shelf_id}`. The name is ignored when creating a shelf. |
| `theme` | String | The theme of the shelf |


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
shelve_name = shelve.name
shelve_theme = shelve.theme
```

---


### Book

Return a book to the library. Returns the book if it is returned to the library successfully. Returns error if the book does not belong to the library or the users didn't borrow before.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The name of the book to return. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `read` | bool | Value indicating whether the book has been read. |
| `title` | String | The title of the book. |
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
    name = "value"  # Required. The name of the book to return.
}

# Access book outputs
book_id = book.id
book_read = book.read
book_title = book.title
book_author = book.author
book_name = book.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple shelve resources
shelve_0 = provider.libraryagent_api.Shelve {
}
shelve_1 = provider.libraryagent_api.Shelve {
}
shelve_2 = provider.libraryagent_api.Shelve {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    shelve = provider.libraryagent_api.Shelve {
    }
```

---

## Related Documentation

- [GCP Libraryagent_api Documentation](https://cloud.google.com/libraryagent_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
