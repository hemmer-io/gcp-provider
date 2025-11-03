# Blogger_3 Service



**Resources**: 8

---

## Overview

The blogger_3 service provides access to 8 resource types:

- [Comment](#comment) [CRD]
- [Post_user_info](#post_user_info) [R]
- [Blog_user_info](#blog_user_info) [R]
- [Blog](#blog) [R]
- [Post](#post) [CRUD]
- [User](#user) [R]
- [Page](#page) [CRUD]
- [Page_view](#page_view) [R]

---

## Resources


### Comment

Removes the content of a comment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `blog_id` | String | ✅ | The Id of the Blog. |
| `comment_id` | String | ✅ | The ID of the comment to delete content from. |
| `post_id` | String | ✅ | The ID of the Post. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | String | The actual content of the comment. May include HTML markup. |
| `author` | String | The author of this Comment. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `blog` | String | Data about the blog containing this comment. |
| `in_reply_to` | String | Data about the comment this is in reply to. |
| `kind` | String | The kind of this entry. Always blogger#comment |
| `status` | String | The status of the comment (only populated for admin users) |
| `post` | String | Data about the post containing this comment. |
| `updated` | String | RFC 3339 date-time when this comment was last updated. |
| `published` | String | RFC 3339 date-time when this comment was published. |
| `id` | String | The identifier for this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create comment
comment = provider.blogger_3.Comment {
    blog_id = "value"  # The Id of the Blog.
    comment_id = "value"  # The ID of the comment to delete content from.
    post_id = "value"  # The ID of the Post.
}

# Access comment outputs
comment_id = comment.id
comment_content = comment.content
comment_author = comment.author
comment_self_link = comment.self_link
comment_blog = comment.blog
comment_in_reply_to = comment.in_reply_to
comment_kind = comment.kind
comment_status = comment.status
comment_post = comment.post
comment_updated = comment.updated
comment_published = comment.published
comment_id = comment.id
```

---


### Post_user_info

Gets one post and user info pair by postId and userId.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | The kind of this entity. Always blogger#postUserInfo |
| `post_user_info` | String | Information about a User for the Post. |
| `post` | String | The Post resource. |


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
post_user_info_kind = post_user_info.kind
post_user_info_post_user_info = post_user_info.post_user_info
post_user_info_post = post_user_info.post
```

---


### Blog_user_info

Gets one blog and user info pair by blogId and userId.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | The kind of this entity. Always blogger#blogUserInfo |
| `blog` | String | The Blog resource. |
| `blog_user_info` | String | Information about a User for the Blog. |


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
blog_user_info_kind = blog_user_info.kind
blog_user_info_blog = blog_user_info.blog
blog_user_info_blog_user_info = blog_user_info.blog_user_info
```

---


### Blog

Gets one blog by id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `locale` | String | The locale this Blog is set to. |
| `id` | String | The identifier for this resource. |
| `kind` | String | The kind of this entry. Always blogger#blog |
| `pages` | String | The container of pages in this blog. |
| `updated` | String | RFC 3339 date-time when this blog was last updated. |
| `name` | String | The name of this blog. This is displayed as the title. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `url` | String | The URL where this blog is published. |
| `custom_meta_data` | String | The JSON custom meta-data for the Blog |
| `description` | String | The description of this blog. This is displayed underneath the title. |
| `posts` | String | The container of posts in this blog. |
| `published` | String | RFC 3339 date-time when this blog was published. |


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
blog_kind = blog.kind
blog_pages = blog.pages
blog_updated = blog.updated
blog_name = blog.name
blog_self_link = blog.self_link
blog_url = blog.url
blog_custom_meta_data = blog.custom_meta_data
blog_description = blog.description
blog_posts = blog.posts
blog_published = blog.published
```

---


### Post

Add a post.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `status` | String |  | Status of the post. Only set for admin-level requests |
| `content` | String |  | The content of the Post. May contain HTML markup. |
| `location` | String |  | The location for geotagged posts. |
| `title_link` | String |  | The title link URL, similar to atom's related link. |
| `custom_meta_data` | String |  | The JSON meta-data for the Post. |
| `published` | String |  | RFC 3339 date-time when this Post was published. |
| `title` | String |  | The title of the Post. |
| `blog` | String |  | Data about the blog containing this Post. |
| `labels` | Vec<String> |  | The list of labels this Post was tagged with. |
| `kind` | String |  | The kind of this entity. Always blogger#post |
| `images` | Vec<String> |  | Display image for the Post. |
| `author` | String |  | The author of this Post. |
| `url` | String |  | The URL where this Post is displayed. |
| `updated` | String |  | RFC 3339 date-time when this Post was last updated. |
| `replies` | String |  | The container of comments on this Post. |
| `self_link` | String |  | The API REST URL to fetch this resource from. |
| `id` | String |  | The identifier of this Post. |
| `blog_id` | String | ✅ | ID of the blog to add the post to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String | Status of the post. Only set for admin-level requests |
| `content` | String | The content of the Post. May contain HTML markup. |
| `location` | String | The location for geotagged posts. |
| `title_link` | String | The title link URL, similar to atom's related link. |
| `custom_meta_data` | String | The JSON meta-data for the Post. |
| `published` | String | RFC 3339 date-time when this Post was published. |
| `title` | String | The title of the Post. |
| `blog` | String | Data about the blog containing this Post. |
| `labels` | Vec<String> | The list of labels this Post was tagged with. |
| `kind` | String | The kind of this entity. Always blogger#post |
| `images` | Vec<String> | Display image for the Post. |
| `author` | String | The author of this Post. |
| `url` | String | The URL where this Post is displayed. |
| `updated` | String | RFC 3339 date-time when this Post was last updated. |
| `replies` | String | The container of comments on this Post. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `id` | String | The identifier of this Post. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create post
post = provider.blogger_3.Post {
    blog_id = "value"  # ID of the blog to add the post to.
}

# Access post outputs
post_id = post.id
post_status = post.status
post_content = post.content
post_location = post.location
post_title_link = post.title_link
post_custom_meta_data = post.custom_meta_data
post_published = post.published
post_title = post.title
post_blog = post.blog
post_labels = post.labels
post_kind = post.kind
post_images = post.images
post_author = post.author
post_url = post.url
post_updated = post.updated
post_replies = post.replies
post_self_link = post.self_link
post_id = post.id
```

---


### User

Gets one user by id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | The API REST URL to fetch this resource from. |
| `created` | String | The timestamp of when this profile was created, in seconds since epoch. |
| `blogs` | String | The container of blogs for this user. |
| `display_name` | String | The display name. |
| `id` | String | The identifier for this User. |
| `url` | String | The user's profile page. |
| `locale` | String | This user's locale |
| `about` | String | Profile summary information. |
| `kind` | String | The kind of this entity. Always blogger#user |


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
user_self_link = user.self_link
user_created = user.created
user_blogs = user.blogs
user_display_name = user.display_name
user_id = user.id
user_url = user.url
user_locale = user.locale
user_about = user.about
user_kind = user.kind
```

---


### Page

Add a page.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | String |  | The URL that this Page is displayed at. |
| `status` | String |  | The status of the page for admin resources (either LIVE or DRAFT). |
| `blog` | String |  | Data about the blog containing this Page. |
| `author` | String |  | The author of this Page. |
| `id` | String |  | The identifier for this resource. |
| `published` | String |  | RFC 3339 date-time when this Page was published. |
| `self_link` | String |  | The API REST URL to fetch this resource from. |
| `content` | String |  | The body content of this Page, in HTML. |
| `kind` | String |  | The kind of this entity. Always blogger#page |
| `updated` | String |  | RFC 3339 date-time when this Page was last updated. |
| `title` | String |  | The title of this entity. This is the name displayed in the Admin user interface. |
| `blog_id` | String | ✅ | ID of the blog to add the page to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `url` | String | The URL that this Page is displayed at. |
| `status` | String | The status of the page for admin resources (either LIVE or DRAFT). |
| `blog` | String | Data about the blog containing this Page. |
| `author` | String | The author of this Page. |
| `id` | String | The identifier for this resource. |
| `published` | String | RFC 3339 date-time when this Page was published. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `content` | String | The body content of this Page, in HTML. |
| `kind` | String | The kind of this entity. Always blogger#page |
| `updated` | String | RFC 3339 date-time when this Page was last updated. |
| `title` | String | The title of this entity. This is the name displayed in the Admin user interface. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create page
page = provider.blogger_3.Page {
    blog_id = "value"  # ID of the blog to add the page to.
}

# Access page outputs
page_id = page.id
page_url = page.url
page_status = page.status
page_blog = page.blog
page_author = page.author
page_id = page.id
page_published = page.published
page_self_link = page.self_link
page_content = page.content
page_kind = page.kind
page_updated = page.updated
page_title = page.title
```

---


### Page_view

Retrieve pageview stats for a Blog.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | The kind of this entry. Always blogger#page_views |
| `blog_id` | String | Blog Id |
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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple comment resources
comment_0 = provider.blogger_3.Comment {
    blog_id = "value-0"
    comment_id = "value-0"
    post_id = "value-0"
}
comment_1 = provider.blogger_3.Comment {
    blog_id = "value-1"
    comment_id = "value-1"
    post_id = "value-1"
}
comment_2 = provider.blogger_3.Comment {
    blog_id = "value-2"
    comment_id = "value-2"
    post_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    comment = provider.blogger_3.Comment {
        blog_id = "production-value"
        comment_id = "production-value"
        post_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Blogger_3 Documentation](https://cloud.google.com/blogger_3/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
