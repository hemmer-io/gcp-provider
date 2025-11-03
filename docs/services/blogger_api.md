# Blogger_api Service



**Resources**: 13

---

## Overview

The blogger_api service provides access to 13 resource types:

- [Comment](#comment) [R]
- [Page](#page) [R]
- [Post](#post) [R]
- [Blog](#blog) [R]
- [User](#user) [R]
- [Comment](#comment) [CRD]
- [User](#user) [R]
- [Post](#post) [CRUD]
- [Page](#page) [CRUD]
- [Blog](#blog) [R]
- [Page_view](#page_view) [R]
- [Post_user_info](#post_user_info) [R]
- [Blog_user_info](#blog_user_info) [R]

---

## Resources


### Comment

Gets a comment by blog id, post id and comment id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `published` | String | RFC 3339 date-time when this comment was published. |
| `id` | String | The identifier for this resource. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `author` | String | The author of this Comment. |
| `content` | String | The actual content of the comment. May include HTML markup. |
| `kind` | String | The kind of this entry. Always blogger#comment. |
| `updated` | String | RFC 3339 date-time when this comment was last updated. |
| `in_reply_to` | String | Data about the comment this is in reply to. |
| `post` | String | Data about the post containing this comment. |
| `blog` | String | Data about the blog containing this comment. |
| `status` | String | The status of the comment (only populated for admin users). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access comment outputs
comment_id = comment.id
comment_published = comment.published
comment_id = comment.id
comment_self_link = comment.self_link
comment_author = comment.author
comment_content = comment.content
comment_kind = comment.kind
comment_updated = comment.updated
comment_in_reply_to = comment.in_reply_to
comment_post = comment.post
comment_blog = comment.blog
comment_status = comment.status
```

---


### Page

Gets a page by blog id and page id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `updated` | String | RFC 3339 date-time when this Page was last updated. |
| `kind` | String | The kind of this entity. Always blogger#page. |
| `blog` | String | Data about the blog containing this Page. |
| `author` | String | The author of this Page. |
| `id` | String | The identifier for this resource. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `etag` | String | Etag of the resource. |
| `status` | String | The status of the page for admin resources (either LIVE or DRAFT). |
| `published` | String | RFC 3339 date-time when this Page was published. |
| `title` | String | The title of this entity. This is the name displayed in the Admin user interface. |
| `content` | String | The body content of this Page, in HTML. |
| `trashed` | String | RFC 3339 date-time when this Page was trashed. |
| `url` | String | The URL that this Page is displayed at. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access page outputs
page_id = page.id
page_updated = page.updated
page_kind = page.kind
page_blog = page.blog
page_author = page.author
page_id = page.id
page_self_link = page.self_link
page_etag = page.etag
page_status = page.status
page_published = page.published
page_title = page.title
page_content = page.content
page_trashed = page.trashed
page_url = page.url
```

---


### Post

Gets a post by blog id and post id

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `published` | String | RFC 3339 date-time when this Post was published. |
| `title` | String | The title of the Post. |
| `trashed` | String | RFC 3339 date-time when this Post was last trashed. |
| `title_link` | String | The title link URL, similar to atom's related link. |
| `url` | String | The URL where this Post is displayed. |
| `blog` | String | Data about the blog containing this Post. |
| `content` | String | The content of the Post. May contain HTML markup. |
| `kind` | String | The kind of this entity. Always blogger#post. |
| `custom_meta_data` | String | The JSON meta-data for the Post. |
| `etag` | String | Etag of the resource. |
| `updated` | String | RFC 3339 date-time when this Post was last updated. |
| `replies` | String | The container of comments on this Post. |
| `id` | String | The identifier of this Post. |
| `status` | String | Status of the post. Only set for admin-level requests. |
| `reader_comments` | String | Comment control and display setting for readers of this post. |
| `location` | String | The location for geotagged posts. |
| `labels` | Vec<String> | The list of labels this Post was tagged with. |
| `author` | String | The author of this Post. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `images` | Vec<String> | Display image for the Post. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access post outputs
post_id = post.id
post_published = post.published
post_title = post.title
post_trashed = post.trashed
post_title_link = post.title_link
post_url = post.url
post_blog = post.blog
post_content = post.content
post_kind = post.kind
post_custom_meta_data = post.custom_meta_data
post_etag = post.etag
post_updated = post.updated
post_replies = post.replies
post_id = post.id
post_status = post.status
post_reader_comments = post.reader_comments
post_location = post.location
post_labels = post.labels
post_author = post.author
post_self_link = post.self_link
post_images = post.images
```

---


### Blog

Gets a blog by id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `locale` | String | The locale this Blog is set to. |
| `id` | String | The identifier for this resource. |
| `name` | String | The name of this blog. This is displayed as the title. |
| `pages` | String | The container of pages in this blog. |
| `published` | String | RFC 3339 date-time when this blog was published. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `description` | String | The description of this blog. This is displayed underneath the title. |
| `status` | String | The status of the blog. |
| `updated` | String | RFC 3339 date-time when this blog was last updated. |
| `kind` | String | The kind of this entry. Always blogger#blog. |
| `url` | String | The URL where this blog is published. |
| `posts` | String | The container of posts in this blog. |
| `custom_meta_data` | String | The JSON custom meta-data for the Blog. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access blog outputs
blog_id = blog.id
blog_locale = blog.locale
blog_id = blog.id
blog_name = blog.name
blog_pages = blog.pages
blog_published = blog.published
blog_self_link = blog.self_link
blog_description = blog.description
blog_status = blog.status
blog_updated = blog.updated
blog_kind = blog.kind
blog_url = blog.url
blog_posts = blog.posts
blog_custom_meta_data = blog.custom_meta_data
```

---


### User

Gets a user by user id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `about` | String | Profile summary information. |
| `blogs` | String | The container of blogs for this user. |
| `display_name` | String | The display name. |
| `id` | String | The identifier for this User. |
| `locale` | String | This user's locale |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `created` | String | The timestamp of when this profile was created, in seconds since epoch. |
| `url` | String | The user's profile page. |
| `kind` | String | The kind of this entity. Always blogger#user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user outputs
user_id = user.id
user_about = user.about
user_blogs = user.blogs
user_display_name = user.display_name
user_id = user.id
user_locale = user.locale
user_self_link = user.self_link
user_created = user.created
user_url = user.url
user_kind = user.kind
```

---


### Comment

Marks a comment as spam by blog id, post id and comment id.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `comment_id` | String | ✅ |  |
| `post_id` | String | ✅ |  |
| `blog_id` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | The API REST URL to fetch this resource from. |
| `updated` | String | RFC 3339 date-time when this comment was last updated. |
| `blog` | String | Data about the blog containing this comment. |
| `id` | String | The identifier for this resource. |
| `status` | String | The status of the comment (only populated for admin users). |
| `in_reply_to` | String | Data about the comment this is in reply to. |
| `content` | String | The actual content of the comment. May include HTML markup. |
| `author` | String | The author of this Comment. |
| `kind` | String | The kind of this entry. Always blogger#comment. |
| `published` | String | RFC 3339 date-time when this comment was published. |
| `post` | String | Data about the post containing this comment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create comment
comment = provider.blogger_api.Comment {
    comment_id = "value"  # Required field
    post_id = "value"  # Required field
    blog_id = "value"  # Required field
}

# Access comment outputs
comment_id = comment.id
comment_self_link = comment.self_link
comment_updated = comment.updated
comment_blog = comment.blog
comment_id = comment.id
comment_status = comment.status
comment_in_reply_to = comment.in_reply_to
comment_content = comment.content
comment_author = comment.author
comment_kind = comment.kind
comment_published = comment.published
comment_post = comment.post
```

---


### User

Gets one user by user_id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `about` | String | Profile summary information. |
| `blogs` | String | The container of blogs for this user. |
| `created` | String | The timestamp of when this profile was created, in seconds since epoch. |
| `kind` | String | The kind of this entity. Always blogger#user. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `url` | String | The user's profile page. |
| `display_name` | String | The display name. |
| `locale` | String | This user's locale |
| `id` | String | The identifier for this User. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user outputs
user_id = user.id
user_about = user.about
user_blogs = user.blogs
user_created = user.created
user_kind = user.kind
user_self_link = user.self_link
user_url = user.url
user_display_name = user.display_name
user_locale = user.locale
user_id = user.id
```

---


### Post

Inserts a post.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `published` | String |  | RFC 3339 date-time when this Post was published. |
| `status` | String |  | Status of the post. Only set for admin-level requests. |
| `title_link` | String |  | The title link URL, similar to atom's related link. |
| `images` | Vec<String> |  | Display image for the Post. |
| `trashed` | String |  | RFC 3339 date-time when this Post was last trashed. |
| `reader_comments` | String |  | Comment control and display setting for readers of this post. |
| `url` | String |  | The URL where this Post is displayed. |
| `updated` | String |  | RFC 3339 date-time when this Post was last updated. |
| `etag` | String |  | Etag of the resource. |
| `self_link` | String |  | The API REST URL to fetch this resource from. |
| `custom_meta_data` | String |  | The JSON meta-data for the Post. |
| `id` | String |  | The identifier of this Post. |
| `location` | String |  | The location for geotagged posts. |
| `blog` | String |  | Data about the blog containing this Post. |
| `author` | String |  | The author of this Post. |
| `labels` | Vec<String> |  | The list of labels this Post was tagged with. |
| `kind` | String |  | The kind of this entity. Always blogger#post. |
| `content` | String |  | The content of the Post. May contain HTML markup. |
| `replies` | String |  | The container of comments on this Post. |
| `title` | String |  | The title of the Post. |
| `blog_id` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `published` | String | RFC 3339 date-time when this Post was published. |
| `status` | String | Status of the post. Only set for admin-level requests. |
| `title_link` | String | The title link URL, similar to atom's related link. |
| `images` | Vec<String> | Display image for the Post. |
| `trashed` | String | RFC 3339 date-time when this Post was last trashed. |
| `reader_comments` | String | Comment control and display setting for readers of this post. |
| `url` | String | The URL where this Post is displayed. |
| `updated` | String | RFC 3339 date-time when this Post was last updated. |
| `etag` | String | Etag of the resource. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `custom_meta_data` | String | The JSON meta-data for the Post. |
| `id` | String | The identifier of this Post. |
| `location` | String | The location for geotagged posts. |
| `blog` | String | Data about the blog containing this Post. |
| `author` | String | The author of this Post. |
| `labels` | Vec<String> | The list of labels this Post was tagged with. |
| `kind` | String | The kind of this entity. Always blogger#post. |
| `content` | String | The content of the Post. May contain HTML markup. |
| `replies` | String | The container of comments on this Post. |
| `title` | String | The title of the Post. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create post
post = provider.blogger_api.Post {
    blog_id = "value"  # Required field
}

# Access post outputs
post_id = post.id
post_published = post.published
post_status = post.status
post_title_link = post.title_link
post_images = post.images
post_trashed = post.trashed
post_reader_comments = post.reader_comments
post_url = post.url
post_updated = post.updated
post_etag = post.etag
post_self_link = post.self_link
post_custom_meta_data = post.custom_meta_data
post_id = post.id
post_location = post.location
post_blog = post.blog
post_author = post.author
post_labels = post.labels
post_kind = post.kind
post_content = post.content
post_replies = post.replies
post_title = post.title
```

---


### Page

Inserts a page.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The identifier for this resource. |
| `blog` | String |  | Data about the blog containing this Page. |
| `kind` | String |  | The kind of this entity. Always blogger#page. |
| `published` | String |  | RFC 3339 date-time when this Page was published. |
| `content` | String |  | The body content of this Page, in HTML. |
| `etag` | String |  | Etag of the resource. |
| `status` | String |  | The status of the page for admin resources (either LIVE or DRAFT). |
| `title` | String |  | The title of this entity. This is the name displayed in the Admin user interface. |
| `updated` | String |  | RFC 3339 date-time when this Page was last updated. |
| `url` | String |  | The URL that this Page is displayed at. |
| `trashed` | String |  | RFC 3339 date-time when this Page was trashed. |
| `self_link` | String |  | The API REST URL to fetch this resource from. |
| `author` | String |  | The author of this Page. |
| `blog_id` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The identifier for this resource. |
| `blog` | String | Data about the blog containing this Page. |
| `kind` | String | The kind of this entity. Always blogger#page. |
| `published` | String | RFC 3339 date-time when this Page was published. |
| `content` | String | The body content of this Page, in HTML. |
| `etag` | String | Etag of the resource. |
| `status` | String | The status of the page for admin resources (either LIVE or DRAFT). |
| `title` | String | The title of this entity. This is the name displayed in the Admin user interface. |
| `updated` | String | RFC 3339 date-time when this Page was last updated. |
| `url` | String | The URL that this Page is displayed at. |
| `trashed` | String | RFC 3339 date-time when this Page was trashed. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `author` | String | The author of this Page. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create page
page = provider.blogger_api.Page {
    blog_id = "value"  # Required field
}

# Access page outputs
page_id = page.id
page_id = page.id
page_blog = page.blog
page_kind = page.kind
page_published = page.published
page_content = page.content
page_etag = page.etag
page_status = page.status
page_title = page.title
page_updated = page.updated
page_url = page.url
page_trashed = page.trashed
page_self_link = page.self_link
page_author = page.author
```

---


### Blog

Gets a blog by id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_meta_data` | String | The JSON custom meta-data for the Blog. |
| `published` | String | RFC 3339 date-time when this blog was published. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `pages` | String | The container of pages in this blog. |
| `updated` | String | RFC 3339 date-time when this blog was last updated. |
| `url` | String | The URL where this blog is published. |
| `status` | String | The status of the blog. |
| `kind` | String | The kind of this entry. Always blogger#blog. |
| `id` | String | The identifier for this resource. |
| `description` | String | The description of this blog. This is displayed underneath the title. |
| `locale` | String | The locale this Blog is set to. |
| `name` | String | The name of this blog. This is displayed as the title. |
| `posts` | String | The container of posts in this blog. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access blog outputs
blog_id = blog.id
blog_custom_meta_data = blog.custom_meta_data
blog_published = blog.published
blog_self_link = blog.self_link
blog_pages = blog.pages
blog_updated = blog.updated
blog_url = blog.url
blog_status = blog.status
blog_kind = blog.kind
blog_id = blog.id
blog_description = blog.description
blog_locale = blog.locale
blog_name = blog.name
blog_posts = blog.posts
```

---


### Page_view

Gets page views by blog id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | The kind of this entry. Always blogger#page_views. |
| `blog_id` | String | Blog Id. |
| `counts` | Vec<String> | The container of posts in this blog. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access page_view outputs
page_view_id = page_view.id
page_view_kind = page_view.kind
page_view_blog_id = page_view.blog_id
page_view_counts = page_view.counts
```

---


### Post_user_info

Gets one post and user info pair, by post_id and user_id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `post_user_info` | String | Information about a User for the Post. |
| `post` | String | The Post resource. |
| `kind` | String | The kind of this entity. Always blogger#postUserInfo. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access post_user_info outputs
post_user_info_id = post_user_info.id
post_user_info_post_user_info = post_user_info.post_user_info
post_user_info_post = post_user_info.post
post_user_info_kind = post_user_info.kind
```

---


### Blog_user_info

Gets one blog and user info pair by blog id and user id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `blog` | String | The Blog resource. |
| `blog_user_info` | String | Information about a User for the Blog. |
| `kind` | String | The kind of this entity. Always blogger#blogUserInfo. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access blog_user_info outputs
blog_user_info_id = blog_user_info.id
blog_user_info_blog = blog_user_info.blog
blog_user_info_blog_user_info = blog_user_info.blog_user_info
blog_user_info_kind = blog_user_info.kind
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
comment_0 = provider.blogger_api.Comment {
}
comment_1 = provider.blogger_api.Comment {
}
comment_2 = provider.blogger_api.Comment {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    comment = provider.blogger_api.Comment {
    }
```

---

## Related Documentation

- [GCP Blogger_api Documentation](https://cloud.google.com/blogger_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
