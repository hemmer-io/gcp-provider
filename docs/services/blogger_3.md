# Blogger_3 Service



**Resources**: 8

---

## Overview

The blogger_3 service provides access to 8 resource types:

- [Page_view](#page_view) [R]
- [Blog](#blog) [R]
- [User](#user) [R]
- [Comment](#comment) [CRD]
- [Post](#post) [CRUD]
- [Page](#page) [CRUD]
- [Blog_user_info](#blog_user_info) [R]
- [Post_user_info](#post_user_info) [R]

---

## Resources


### Page_view

Retrieve pageview stats for a Blog.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `blog_id` | String | Blog Id |
| `counts` | Vec<String> | The container of posts in this blog. |
| `kind` | String | The kind of this entry. Always blogger#page_views |


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
| `pages` | String | The container of pages in this blog. |
| `url` | String | The URL where this blog is published. |
| `custom_meta_data` | String | The JSON custom meta-data for the Blog |
| `description` | String | The description of this blog. This is displayed underneath the title. |
| `id` | String | The identifier for this resource. |
| `posts` | String | The container of posts in this blog. |
| `kind` | String | The kind of this entry. Always blogger#blog |
| `name` | String | The name of this blog. This is displayed as the title. |
| `published` | String | RFC 3339 date-time when this blog was published. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `updated` | String | RFC 3339 date-time when this blog was last updated. |


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
blog_pages = blog.pages
blog_url = blog.url
blog_custom_meta_data = blog.custom_meta_data
blog_description = blog.description
blog_id = blog.id
blog_posts = blog.posts
blog_kind = blog.kind
blog_name = blog.name
blog_published = blog.published
blog_self_link = blog.self_link
blog_updated = blog.updated
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
| `url` | String | The user's profile page. |
| `kind` | String | The kind of this entity. Always blogger#user |
| `created` | String | The timestamp of when this profile was created, in seconds since epoch. |
| `id` | String | The identifier for this User. |
| `blogs` | String | The container of blogs for this user. |
| `display_name` | String | The display name. |
| `about` | String | Profile summary information. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `locale` | String | This user's locale |


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
user_created = user.created
user_id = user.id
user_blogs = user.blogs
user_display_name = user.display_name
user_about = user.about
user_self_link = user.self_link
user_locale = user.locale
```

---


### Comment

Removes the content of a comment.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `blog_id` | String | ✅ | The Id of the Blog. |
| `post_id` | String | ✅ | The ID of the Post. |
| `comment_id` | String | ✅ | The ID of the comment to delete content from. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | The API REST URL to fetch this resource from. |
| `author` | String | The author of this Comment. |
| `in_reply_to` | String | Data about the comment this is in reply to. |
| `updated` | String | RFC 3339 date-time when this comment was last updated. |
| `content` | String | The actual content of the comment. May include HTML markup. |
| `blog` | String | Data about the blog containing this comment. |
| `status` | String | The status of the comment (only populated for admin users) |
| `published` | String | RFC 3339 date-time when this comment was published. |
| `id` | String | The identifier for this resource. |
| `kind` | String | The kind of this entry. Always blogger#comment |
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
comment = provider.blogger_3.Comment {
    blog_id = "value"  # The Id of the Blog.
    post_id = "value"  # The ID of the Post.
    comment_id = "value"  # The ID of the comment to delete content from.
}

# Access comment outputs
comment_id = comment.id
comment_self_link = comment.self_link
comment_author = comment.author
comment_in_reply_to = comment.in_reply_to
comment_updated = comment.updated
comment_content = comment.content
comment_blog = comment.blog
comment_status = comment.status
comment_published = comment.published
comment_id = comment.id
comment_kind = comment.kind
comment_post = comment.post
```

---


### Post

Add a post.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `replies` | String |  | The container of comments on this Post. |
| `id` | String |  | The identifier of this Post. |
| `author` | String |  | The author of this Post. |
| `images` | Vec<String> |  | Display image for the Post. |
| `content` | String |  | The content of the Post. May contain HTML markup. |
| `url` | String |  | The URL where this Post is displayed. |
| `updated` | String |  | RFC 3339 date-time when this Post was last updated. |
| `title` | String |  | The title of the Post. |
| `self_link` | String |  | The API REST URL to fetch this resource from. |
| `labels` | Vec<String> |  | The list of labels this Post was tagged with. |
| `status` | String |  | Status of the post. Only set for admin-level requests |
| `title_link` | String |  | The title link URL, similar to atom's related link. |
| `kind` | String |  | The kind of this entity. Always blogger#post |
| `location` | String |  | The location for geotagged posts. |
| `custom_meta_data` | String |  | The JSON meta-data for the Post. |
| `blog` | String |  | Data about the blog containing this Post. |
| `published` | String |  | RFC 3339 date-time when this Post was published. |
| `blog_id` | String | ✅ | ID of the blog to add the post to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replies` | String | The container of comments on this Post. |
| `id` | String | The identifier of this Post. |
| `author` | String | The author of this Post. |
| `images` | Vec<String> | Display image for the Post. |
| `content` | String | The content of the Post. May contain HTML markup. |
| `url` | String | The URL where this Post is displayed. |
| `updated` | String | RFC 3339 date-time when this Post was last updated. |
| `title` | String | The title of the Post. |
| `self_link` | String | The API REST URL to fetch this resource from. |
| `labels` | Vec<String> | The list of labels this Post was tagged with. |
| `status` | String | Status of the post. Only set for admin-level requests |
| `title_link` | String | The title link URL, similar to atom's related link. |
| `kind` | String | The kind of this entity. Always blogger#post |
| `location` | String | The location for geotagged posts. |
| `custom_meta_data` | String | The JSON meta-data for the Post. |
| `blog` | String | Data about the blog containing this Post. |
| `published` | String | RFC 3339 date-time when this Post was published. |


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
post_replies = post.replies
post_id = post.id
post_author = post.author
post_images = post.images
post_content = post.content
post_url = post.url
post_updated = post.updated
post_title = post.title
post_self_link = post.self_link
post_labels = post.labels
post_status = post.status
post_title_link = post.title_link
post_kind = post.kind
post_location = post.location
post_custom_meta_data = post.custom_meta_data
post_blog = post.blog
post_published = post.published
```

---


### Page

Add a page.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `self_link` | String |  | The API REST URL to fetch this resource from. |
| `status` | String |  | The status of the page for admin resources (either LIVE or DRAFT). |
| `id` | String |  | The identifier for this resource. |
| `published` | String |  | RFC 3339 date-time when this Page was published. |
| `updated` | String |  | RFC 3339 date-time when this Page was last updated. |
| `url` | String |  | The URL that this Page is displayed at. |
| `kind` | String |  | The kind of this entity. Always blogger#page |
| `content` | String |  | The body content of this Page, in HTML. |
| `title` | String |  | The title of this entity. This is the name displayed in the Admin user interface. |
| `author` | String |  | The author of this Page. |
| `blog` | String |  | Data about the blog containing this Page. |
| `blog_id` | String | ✅ | ID of the blog to add the page to. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | The API REST URL to fetch this resource from. |
| `status` | String | The status of the page for admin resources (either LIVE or DRAFT). |
| `id` | String | The identifier for this resource. |
| `published` | String | RFC 3339 date-time when this Page was published. |
| `updated` | String | RFC 3339 date-time when this Page was last updated. |
| `url` | String | The URL that this Page is displayed at. |
| `kind` | String | The kind of this entity. Always blogger#page |
| `content` | String | The body content of this Page, in HTML. |
| `title` | String | The title of this entity. This is the name displayed in the Admin user interface. |
| `author` | String | The author of this Page. |
| `blog` | String | Data about the blog containing this Page. |


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
page_self_link = page.self_link
page_status = page.status
page_id = page.id
page_published = page.published
page_updated = page.updated
page_url = page.url
page_kind = page.kind
page_content = page.content
page_title = page.title
page_author = page.author
page_blog = page.blog
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
| `blog_user_info` | String | Information about a User for the Blog. |
| `blog` | String | The Blog resource. |
| `kind` | String | The kind of this entity. Always blogger#blogUserInfo |


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
blog_user_info_blog_user_info = blog_user_info.blog_user_info
blog_user_info_blog = blog_user_info.blog
blog_user_info_kind = blog_user_info.kind
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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple page_view resources
page_view_0 = provider.blogger_3.Page_view {
}
page_view_1 = provider.blogger_3.Page_view {
}
page_view_2 = provider.blogger_3.Page_view {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    page_view = provider.blogger_3.Page_view {
    }
```

---

## Related Documentation

- [GCP Blogger_3 Documentation](https://cloud.google.com/blogger_3/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
