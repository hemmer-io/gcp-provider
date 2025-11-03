# Blogger_api Service



**Resources**: 13

---

## Overview

The blogger_api service provides access to 13 resource types:

- [Blog](#blog) [R]
- [Post](#post) [R]
- [Comment](#comment) [R]
- [Page](#page) [R]
- [User](#user) [R]
- [Blog](#blog) [R]
- [User](#user) [R]
- [Blog_user_info](#blog_user_info) [R]
- [Comment](#comment) [CRD]
- [Post](#post) [CRUD]
- [Page_view](#page_view) [R]
- [Page](#page) [CRUD]
- [Post_user_info](#post_user_info) [R]

---

## Resources


### Blog

Gets a blog by id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The identifier for this resource. |
| `status` | String | The status of the blog. |
| `locale` | String | The locale this Blog is set to. |
| `name` | String | The name of this blog. This is displayed as the title. |
| `published` | String | RFC 3339 date-time when this blog was published. |
| `updated` | String | RFC 3339 date-time when this blog was last updated. |
| `url` | String | The URL where this blog is published. |
| `custom_meta_data` | String | The JSON custom meta-data for the Blog. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `kind` | String | The kind of this entry. Always blogger#blog. |
| `posts` | String | The container of posts in this blog. |
| `description` | String | The description of this blog. This is displayed underneath the title. |
| `pages` | String | The container of pages in this blog. |


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
blog_id = blog.id
blog_status = blog.status
blog_locale = blog.locale
blog_name = blog.name
blog_published = blog.published
blog_updated = blog.updated
blog_url = blog.url
blog_custom_meta_data = blog.custom_meta_data
blog_self_link = blog.self_link
blog_kind = blog.kind
blog_posts = blog.posts
blog_description = blog.description
blog_pages = blog.pages
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
| `custom_meta_data` | String | The JSON meta-data for the Post. |
| `published` | String | RFC 3339 date-time when this Post was published. |
| `labels` | Vec<String> | The list of labels this Post was tagged with. |
| `reader_comments` | String | Comment control and display setting for readers of this post. |
| `content` | String | The content of the Post. May contain HTML markup. |
| `etag` | String | Etag of the resource. |
| `replies` | String | The container of comments on this Post. |
| `kind` | String | The kind of this entity. Always blogger#post. |
| `images` | Vec<String> | Display image for the Post. |
| `title_link` | String | The title link URL, similar to atom's related link. |
| `url` | String | The URL where this Post is displayed. |
| `updated` | String | RFC 3339 date-time when this Post was last updated. |
| `blog` | String | Data about the blog containing this Post. |
| `id` | String | The identifier of this Post. |
| `location` | String | The location for geotagged posts. |
| `status` | String | Status of the post. Only set for admin-level requests. |
| `trashed` | String | RFC 3339 date-time when this Post was last trashed. |
| `author` | String | The author of this Post. |
| `title` | String | The title of the Post. |
| `self_link` | String | The API REST URL to fetch this resource from. |


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
post_custom_meta_data = post.custom_meta_data
post_published = post.published
post_labels = post.labels
post_reader_comments = post.reader_comments
post_content = post.content
post_etag = post.etag
post_replies = post.replies
post_kind = post.kind
post_images = post.images
post_title_link = post.title_link
post_url = post.url
post_updated = post.updated
post_blog = post.blog
post_id = post.id
post_location = post.location
post_status = post.status
post_trashed = post.trashed
post_author = post.author
post_title = post.title
post_self_link = post.self_link
```

---


### Comment

Gets a comment by blog id, post id and comment id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `blog` | String | Data about the blog containing this comment. |
| `in_reply_to` | String | Data about the comment this is in reply to. |
| `status` | String | The status of the comment (only populated for admin users). |
| `content` | String | The actual content of the comment. May include HTML markup. |
| `post` | String | Data about the post containing this comment. |
| `id` | String | The identifier for this resource. |
| `author` | String | The author of this Comment. |
| `updated` | String | RFC 3339 date-time when this comment was last updated. |
| `published` | String | RFC 3339 date-time when this comment was published. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `kind` | String | The kind of this entry. Always blogger#comment. |


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
comment_blog = comment.blog
comment_in_reply_to = comment.in_reply_to
comment_status = comment.status
comment_content = comment.content
comment_post = comment.post
comment_id = comment.id
comment_author = comment.author
comment_updated = comment.updated
comment_published = comment.published
comment_self_link = comment.self_link
comment_kind = comment.kind
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
| `id` | String | The identifier for this resource. |
| `kind` | String | The kind of this entity. Always blogger#page. |
| `title` | String | The title of this entity. This is the name displayed in the Admin user interface. |
| `trashed` | String | RFC 3339 date-time when this Page was trashed. |
| `status` | String | The status of the page for admin resources (either LIVE or DRAFT). |
| `url` | String | The URL that this Page is displayed at. |
| `author` | String | The author of this Page. |
| `blog` | String | Data about the blog containing this Page. |
| `updated` | String | RFC 3339 date-time when this Page was last updated. |
| `etag` | String | Etag of the resource. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `content` | String | The body content of this Page, in HTML. |
| `published` | String | RFC 3339 date-time when this Page was published. |


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
page_id = page.id
page_kind = page.kind
page_title = page.title
page_trashed = page.trashed
page_status = page.status
page_url = page.url
page_author = page.author
page_blog = page.blog
page_updated = page.updated
page_etag = page.etag
page_self_link = page.self_link
page_content = page.content
page_published = page.published
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
| `url` | String | The user's profile page. |
| `kind` | String | The kind of this entity. Always blogger#user. |
| `display_name` | String | The display name. |
| `locale` | String | This user's locale |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `blogs` | String | The container of blogs for this user. |
| `id` | String | The identifier for this User. |
| `about` | String | Profile summary information. |
| `created` | String | The timestamp of when this profile was created, in seconds since epoch. |


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
user_url = user.url
user_kind = user.kind
user_display_name = user.display_name
user_locale = user.locale
user_self_link = user.self_link
user_blogs = user.blogs
user_id = user.id
user_about = user.about
user_created = user.created
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
| `id` | String | The identifier for this resource. |
| `locale` | String | The locale this Blog is set to. |
| `pages` | String | The container of pages in this blog. |
| `name` | String | The name of this blog. This is displayed as the title. |
| `kind` | String | The kind of this entry. Always blogger#blog. |
| `description` | String | The description of this blog. This is displayed underneath the title. |
| `posts` | String | The container of posts in this blog. |
| `custom_meta_data` | String | The JSON custom meta-data for the Blog. |
| `published` | String | RFC 3339 date-time when this blog was published. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `status` | String | The status of the blog. |
| `updated` | String | RFC 3339 date-time when this blog was last updated. |
| `url` | String | The URL where this blog is published. |


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
blog_id = blog.id
blog_locale = blog.locale
blog_pages = blog.pages
blog_name = blog.name
blog_kind = blog.kind
blog_description = blog.description
blog_posts = blog.posts
blog_custom_meta_data = blog.custom_meta_data
blog_published = blog.published
blog_self_link = blog.self_link
blog_status = blog.status
blog_updated = blog.updated
blog_url = blog.url
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
| `id` | String | The identifier for this User. |
| `url` | String | The user's profile page. |
| `kind` | String | The kind of this entity. Always blogger#user. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `about` | String | Profile summary information. |
| `locale` | String | This user's locale |
| `display_name` | String | The display name. |
| `created` | String | The timestamp of when this profile was created, in seconds since epoch. |
| `blogs` | String | The container of blogs for this user. |


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
user_id = user.id
user_url = user.url
user_kind = user.kind
user_self_link = user.self_link
user_about = user.about
user_locale = user.locale
user_display_name = user.display_name
user_created = user.created
user_blogs = user.blogs
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
| `id` | String | The identifier for this resource. |
| `post` | String | Data about the post containing this comment. |
| `blog` | String | Data about the blog containing this comment. |
| `published` | String | RFC 3339 date-time when this comment was published. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `status` | String | The status of the comment (only populated for admin users). |
| `content` | String | The actual content of the comment. May include HTML markup. |
| `updated` | String | RFC 3339 date-time when this comment was last updated. |
| `in_reply_to` | String | Data about the comment this is in reply to. |
| `author` | String | The author of this Comment. |
| `kind` | String | The kind of this entry. Always blogger#comment. |


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
comment_id = comment.id
comment_post = comment.post
comment_blog = comment.blog
comment_published = comment.published
comment_self_link = comment.self_link
comment_status = comment.status
comment_content = comment.content
comment_updated = comment.updated
comment_in_reply_to = comment.in_reply_to
comment_author = comment.author
comment_kind = comment.kind
```

---


### Post

Inserts a post.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | Vec<String> |  | The list of labels this Post was tagged with. |
| `content` | String |  | The content of the Post. May contain HTML markup. |
| `url` | String |  | The URL where this Post is displayed. |
| `kind` | String |  | The kind of this entity. Always blogger#post. |
| `reader_comments` | String |  | Comment control and display setting for readers of this post. |
| `status` | String |  | Status of the post. Only set for admin-level requests. |
| `location` | String |  | The location for geotagged posts. |
| `id` | String |  | The identifier of this Post. |
| `replies` | String |  | The container of comments on this Post. |
| `images` | Vec<String> |  | Display image for the Post. |
| `blog` | String |  | Data about the blog containing this Post. |
| `etag` | String |  | Etag of the resource. |
| `updated` | String |  | RFC 3339 date-time when this Post was last updated. |
| `custom_meta_data` | String |  | The JSON meta-data for the Post. |
| `published` | String |  | RFC 3339 date-time when this Post was published. |
| `title` | String |  | The title of the Post. |
| `self_link` | String |  | The API REST URL to fetch this resource from. |
| `trashed` | String |  | RFC 3339 date-time when this Post was last trashed. |
| `author` | String |  | The author of this Post. |
| `title_link` | String |  | The title link URL, similar to atom's related link. |
| `blog_id` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | Vec<String> | The list of labels this Post was tagged with. |
| `content` | String | The content of the Post. May contain HTML markup. |
| `url` | String | The URL where this Post is displayed. |
| `kind` | String | The kind of this entity. Always blogger#post. |
| `reader_comments` | String | Comment control and display setting for readers of this post. |
| `status` | String | Status of the post. Only set for admin-level requests. |
| `location` | String | The location for geotagged posts. |
| `id` | String | The identifier of this Post. |
| `replies` | String | The container of comments on this Post. |
| `images` | Vec<String> | Display image for the Post. |
| `blog` | String | Data about the blog containing this Post. |
| `etag` | String | Etag of the resource. |
| `updated` | String | RFC 3339 date-time when this Post was last updated. |
| `custom_meta_data` | String | The JSON meta-data for the Post. |
| `published` | String | RFC 3339 date-time when this Post was published. |
| `title` | String | The title of the Post. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `trashed` | String | RFC 3339 date-time when this Post was last trashed. |
| `author` | String | The author of this Post. |
| `title_link` | String | The title link URL, similar to atom's related link. |


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
post_labels = post.labels
post_content = post.content
post_url = post.url
post_kind = post.kind
post_reader_comments = post.reader_comments
post_status = post.status
post_location = post.location
post_id = post.id
post_replies = post.replies
post_images = post.images
post_blog = post.blog
post_etag = post.etag
post_updated = post.updated
post_custom_meta_data = post.custom_meta_data
post_published = post.published
post_title = post.title
post_self_link = post.self_link
post_trashed = post.trashed
post_author = post.author
post_title_link = post.title_link
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
| `blog_id` | String | Blog Id. |
| `counts` | Vec<String> | The container of posts in this blog. |
| `kind` | String | The kind of this entry. Always blogger#page_views. |


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
page_view_blog_id = page_view.blog_id
page_view_counts = page_view.counts
page_view_kind = page_view.kind
```

---


### Page

Inserts a page.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `published` | String |  | RFC 3339 date-time when this Page was published. |
| `kind` | String |  | The kind of this entity. Always blogger#page. |
| `trashed` | String |  | RFC 3339 date-time when this Page was trashed. |
| `status` | String |  | The status of the page for admin resources (either LIVE or DRAFT). |
| `author` | String |  | The author of this Page. |
| `id` | String |  | The identifier for this resource. |
| `title` | String |  | The title of this entity. This is the name displayed in the Admin user interface. |
| `etag` | String |  | Etag of the resource. |
| `self_link` | String |  | The API REST URL to fetch this resource from. |
| `blog` | String |  | Data about the blog containing this Page. |
| `url` | String |  | The URL that this Page is displayed at. |
| `content` | String |  | The body content of this Page, in HTML. |
| `updated` | String |  | RFC 3339 date-time when this Page was last updated. |
| `blog_id` | String | ✅ |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `published` | String | RFC 3339 date-time when this Page was published. |
| `kind` | String | The kind of this entity. Always blogger#page. |
| `trashed` | String | RFC 3339 date-time when this Page was trashed. |
| `status` | String | The status of the page for admin resources (either LIVE or DRAFT). |
| `author` | String | The author of this Page. |
| `id` | String | The identifier for this resource. |
| `title` | String | The title of this entity. This is the name displayed in the Admin user interface. |
| `etag` | String | Etag of the resource. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `blog` | String | Data about the blog containing this Page. |
| `url` | String | The URL that this Page is displayed at. |
| `content` | String | The body content of this Page, in HTML. |
| `updated` | String | RFC 3339 date-time when this Page was last updated. |


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
page_published = page.published
page_kind = page.kind
page_trashed = page.trashed
page_status = page.status
page_author = page.author
page_id = page.id
page_title = page.title
page_etag = page.etag
page_self_link = page.self_link
page_blog = page.blog
page_url = page.url
page_content = page.content
page_updated = page.updated
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
| `post` | String | The Post resource. |
| `kind` | String | The kind of this entity. Always blogger#postUserInfo. |
| `post_user_info` | String | Information about a User for the Post. |


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
post_user_info_post = post_user_info.post
post_user_info_kind = post_user_info.kind
post_user_info_post_user_info = post_user_info.post_user_info
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple blog resources
blog_0 = provider.blogger_api.Blog {
}
blog_1 = provider.blogger_api.Blog {
}
blog_2 = provider.blogger_api.Blog {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    blog = provider.blogger_api.Blog {
    }
```

---

## Related Documentation

- [GCP Blogger_api Documentation](https://cloud.google.com/blogger_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
