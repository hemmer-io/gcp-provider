# Chat_api Service



**Resources**: 10

---

## Overview

The chat_api service provides access to 10 resource types:

- [Space](#space) [CRUD]
- [Reaction](#reaction) [CRD]
- [Member](#member) [CRUD]
- [Message](#message) [CRUD]
- [Space_notification_setting](#space_notification_setting) [RU]
- [Thread](#thread) [R]
- [Attachment](#attachment) [R]
- [Custom_emoji](#custom_emoji) [CRD]
- [Space_event](#space_event) [R]
- [Media](#media) [CR]

---

## Resources


### Space

Creates a space. Can be used to create a named space, or a group chat in `Import mode`. For an example, see [Create a space](https://developers.google.com/workspace/chat/create-spaces). Supports the following types of [authentication](https://developers.google.com/workspace/chat/authenticate-authorize): - [App authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) with [administrator approval](https://support.google.com/a?p=chat-app-auth) and one of the following authorization scopes: - `https://www.googleapis.com/auth/chat.app.spaces.create` - `https://www.googleapis.com/auth/chat.app.spaces` - [User authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with one of the following authorization scopes: - `https://www.googleapis.com/auth/chat.spaces.create` - `https://www.googleapis.com/auth/chat.spaces` - `https://www.googleapis.com/auth/chat.import` (import mode spaces only) When authenticating as an app, the `space.customer` field must be set in the request. When authenticating as an app, the Chat app is added as a member of the space. However, unlike human authentication, the Chat app is not added as a space manager. By default, the Chat app can be removed from the space by all space members. To allow only space managers to remove the app from a space, set `space.permission_settings.manage_apps` to `managers_allowed`. Space membership upon creation depends on whether the space is created in `Import mode`: * **Import mode:** No members are created. * **All other modes:** The calling user is added as a member. This is: * The app itself when using app authentication. * The human user when using user authentication. If you receive the error message `ALREADY_EXISTS` when creating a space, try a different `displayName`. An existing space within the Google Workspace organization might already use this display name.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Optional. Immutable. The customer id of the domain of the space. Required only when creating a space with [app authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) and `SpaceType` is `SPACE`, otherwise should not be set. In the format `customers/{customer}`, where `customer` is the `id` from the [Admin SDK customer resource](https://developers.google.com/admin-sdk/directory/reference/rest/v1/customers). Private apps can also use the `customers/my_customer` alias to create the space in the same Google Workspace organization as the app. This field isn't populated for direct messages (DMs) or when the space is created by non-Google Workspace users. |
| `predefined_permission_settings` | String |  | Optional. Input only. Predefined space permission settings, input only when creating a space. If the field is not set, a collaboration space is created. After you create the space, settings are populated in the `PermissionSettings` field. Setting predefined permission settings supports: - [App authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) with [administrator approval](https://support.google.com/a?p=chat-app-auth) with the `chat.app.spaces` or `chat.app.spaces.create` scopes. - [User authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) |
| `import_mode` | bool |  | Optional. Whether this space is created in `Import Mode` as part of a data migration into Google Workspace. While spaces are being imported, they aren't visible to users until the import is complete. Creating a space in `Import Mode`requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user). |
| `create_time` | String |  | Optional. Immutable. For spaces created in Chat, the time the space was created. This field is output only, except when used in import mode spaces. For import mode spaces, set this field to the historical timestamp at which the space was created in the source in order to preserve the original creation time. Only populated in the output when `spaceType` is `GROUP_CHAT` or `SPACE`. |
| `access_settings` | String |  | Optional. Specifies the [access setting](https://support.google.com/chat/answer/11971020) of the space. Only populated when the `space_type` is `SPACE`. |
| `import_mode_expire_time` | String |  | Output only. The time when the space will be automatically deleted by the system if it remains in import mode. Each space created in import mode must exit this mode before this expire time using `spaces.completeImport`. This field is only populated for spaces that were created with import mode. |
| `space_history_state` | String |  | Optional. The message history state for messages and threads in this space. |
| `space_threading_state` | String |  | Output only. The threading state in the Chat space. |
| `name` | String |  | Identifier. Resource name of the space. Format: `spaces/{space}` Where `{space}` represents the system-assigned ID for the space. You can obtain the space ID by calling the [`spaces.list()`](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces/list) method or from the space URL. For example, if the space URL is `https://mail.google.com/mail/u/0/#chat/space/AAAAAAAAA`, the space ID is `AAAAAAAAA`. |
| `permission_settings` | String |  | Optional. Space permission settings for existing spaces. Input for updating exact space permission settings, where existing permission settings are replaced. Output lists current permission settings. Reading and updating permission settings supports: - [App authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) with [administrator approval](https://support.google.com/a?p=chat-app-auth) with the `chat.app.spaces` scope. Only populated and settable when the Chat app created the space. - [User authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) |
| `display_name` | String |  | Optional. The space's display name. Required when [creating a space](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces/create) with a `spaceType` of `SPACE`. If you receive the error message `ALREADY_EXISTS` when creating a space or updating the `displayName`, try a different `displayName`. An existing space within the Google Workspace organization might already use this display name. For direct messages, this field might be empty. Supports up to 128 characters. |
| `last_active_time` | String |  | Output only. Timestamp of the last message in the space. |
| `space_uri` | String |  | Output only. The URI for a user to access the space. |
| `threaded` | bool |  | Output only. Deprecated: Use `spaceThreadingState` instead. Whether messages are threaded in this space. |
| `type` | String |  | Output only. Deprecated: Use `space_type` instead. The type of a space. |
| `space_type` | String |  | Optional. The type of space. Required when creating a space or updating the space type of a space. Output only for other usage. |
| `single_user_bot_dm` | bool |  | Optional. Whether the space is a DM between a Chat app and a single human. |
| `admin_installed` | bool |  | Output only. For direct message (DM) spaces with a Chat app, whether the space was created by a Google Workspace administrator. Administrators can install and set up a direct message with a Chat app on behalf of users in their organization. To support admin install, your Chat app must feature direct messaging. |
| `membership_count` | String |  | Output only. The count of joined memberships grouped by member type. Populated when the `space_type` is `SPACE`, `DIRECT_MESSAGE` or `GROUP_CHAT`. |
| `space_details` | String |  | Optional. Details about the space including description and rules. |
| `external_user_allowed` | bool |  | Optional. Immutable. Whether this space permits any Google Chat user as a member. Input when creating a space in a Google Workspace organization. Omit this field when creating spaces in the following conditions: * The authenticated user uses a consumer account (unmanaged user account). By default, a space created by a consumer account permits any Google Chat user. For existing spaces, this field is output only. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer` | String | Optional. Immutable. The customer id of the domain of the space. Required only when creating a space with [app authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) and `SpaceType` is `SPACE`, otherwise should not be set. In the format `customers/{customer}`, where `customer` is the `id` from the [Admin SDK customer resource](https://developers.google.com/admin-sdk/directory/reference/rest/v1/customers). Private apps can also use the `customers/my_customer` alias to create the space in the same Google Workspace organization as the app. This field isn't populated for direct messages (DMs) or when the space is created by non-Google Workspace users. |
| `predefined_permission_settings` | String | Optional. Input only. Predefined space permission settings, input only when creating a space. If the field is not set, a collaboration space is created. After you create the space, settings are populated in the `PermissionSettings` field. Setting predefined permission settings supports: - [App authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) with [administrator approval](https://support.google.com/a?p=chat-app-auth) with the `chat.app.spaces` or `chat.app.spaces.create` scopes. - [User authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) |
| `import_mode` | bool | Optional. Whether this space is created in `Import Mode` as part of a data migration into Google Workspace. While spaces are being imported, they aren't visible to users until the import is complete. Creating a space in `Import Mode`requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user). |
| `create_time` | String | Optional. Immutable. For spaces created in Chat, the time the space was created. This field is output only, except when used in import mode spaces. For import mode spaces, set this field to the historical timestamp at which the space was created in the source in order to preserve the original creation time. Only populated in the output when `spaceType` is `GROUP_CHAT` or `SPACE`. |
| `access_settings` | String | Optional. Specifies the [access setting](https://support.google.com/chat/answer/11971020) of the space. Only populated when the `space_type` is `SPACE`. |
| `import_mode_expire_time` | String | Output only. The time when the space will be automatically deleted by the system if it remains in import mode. Each space created in import mode must exit this mode before this expire time using `spaces.completeImport`. This field is only populated for spaces that were created with import mode. |
| `space_history_state` | String | Optional. The message history state for messages and threads in this space. |
| `space_threading_state` | String | Output only. The threading state in the Chat space. |
| `name` | String | Identifier. Resource name of the space. Format: `spaces/{space}` Where `{space}` represents the system-assigned ID for the space. You can obtain the space ID by calling the [`spaces.list()`](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces/list) method or from the space URL. For example, if the space URL is `https://mail.google.com/mail/u/0/#chat/space/AAAAAAAAA`, the space ID is `AAAAAAAAA`. |
| `permission_settings` | String | Optional. Space permission settings for existing spaces. Input for updating exact space permission settings, where existing permission settings are replaced. Output lists current permission settings. Reading and updating permission settings supports: - [App authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) with [administrator approval](https://support.google.com/a?p=chat-app-auth) with the `chat.app.spaces` scope. Only populated and settable when the Chat app created the space. - [User authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) |
| `display_name` | String | Optional. The space's display name. Required when [creating a space](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces/create) with a `spaceType` of `SPACE`. If you receive the error message `ALREADY_EXISTS` when creating a space or updating the `displayName`, try a different `displayName`. An existing space within the Google Workspace organization might already use this display name. For direct messages, this field might be empty. Supports up to 128 characters. |
| `last_active_time` | String | Output only. Timestamp of the last message in the space. |
| `space_uri` | String | Output only. The URI for a user to access the space. |
| `threaded` | bool | Output only. Deprecated: Use `spaceThreadingState` instead. Whether messages are threaded in this space. |
| `type` | String | Output only. Deprecated: Use `space_type` instead. The type of a space. |
| `space_type` | String | Optional. The type of space. Required when creating a space or updating the space type of a space. Output only for other usage. |
| `single_user_bot_dm` | bool | Optional. Whether the space is a DM between a Chat app and a single human. |
| `admin_installed` | bool | Output only. For direct message (DM) spaces with a Chat app, whether the space was created by a Google Workspace administrator. Administrators can install and set up a direct message with a Chat app on behalf of users in their organization. To support admin install, your Chat app must feature direct messaging. |
| `membership_count` | String | Output only. The count of joined memberships grouped by member type. Populated when the `space_type` is `SPACE`, `DIRECT_MESSAGE` or `GROUP_CHAT`. |
| `space_details` | String | Optional. Details about the space including description and rules. |
| `external_user_allowed` | bool | Optional. Immutable. Whether this space permits any Google Chat user as a member. Input when creating a space in a Google Workspace organization. Omit this field when creating spaces in the following conditions: * The authenticated user uses a consumer account (unmanaged user account). By default, a space created by a consumer account permits any Google Chat user. For existing spaces, this field is output only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create space
space = provider.chat_api.Space {
}

# Access space outputs
space_id = space.id
space_customer = space.customer
space_predefined_permission_settings = space.predefined_permission_settings
space_import_mode = space.import_mode
space_create_time = space.create_time
space_access_settings = space.access_settings
space_import_mode_expire_time = space.import_mode_expire_time
space_space_history_state = space.space_history_state
space_space_threading_state = space.space_threading_state
space_name = space.name
space_permission_settings = space.permission_settings
space_display_name = space.display_name
space_last_active_time = space.last_active_time
space_space_uri = space.space_uri
space_threaded = space.threaded
space_type = space.type
space_space_type = space.space_type
space_single_user_bot_dm = space.single_user_bot_dm
space_admin_installed = space.admin_installed
space_membership_count = space.membership_count
space_space_details = space.space_details
space_external_user_allowed = space.external_user_allowed
```

---


### Reaction

Creates a reaction and adds it to a message. For an example, see [Add a reaction to a message](https://developers.google.com/workspace/chat/create-reactions). Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with one of the following [authorization scopes](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.messages.reactions.create` - `https://www.googleapis.com/auth/chat.messages.reactions` - `https://www.googleapis.com/auth/chat.messages` - `https://www.googleapis.com/auth/chat.import` (import mode spaces only)

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user` | String |  | Output only. The user who created the reaction. |
| `emoji` | String |  | Required. The emoji used in the reaction. |
| `name` | String |  | Identifier. The resource name of the reaction. Format: `spaces/{space}/messages/{message}/reactions/{reaction}` |
| `parent` | String | ✅ | Required. The message where the reaction is created. Format: `spaces/{space}/messages/{message}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Continuation token to retrieve the next page of results. It's empty for the last page of results. |
| `reactions` | Vec<String> | List of reactions in the requested (or first) page. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create reaction
reaction = provider.chat_api.Reaction {
    parent = "value"  # Required. The message where the reaction is created. Format: `spaces/{space}/messages/{message}`
}

# Access reaction outputs
reaction_id = reaction.id
reaction_next_page_token = reaction.next_page_token
reaction_reactions = reaction.reactions
```

---


### Member

Creates a membership for the calling Chat app, a user, or a Google Group. Creating memberships for other Chat apps isn't supported. When creating a membership, if the specified member has their auto-accept policy turned off, then they're invited, and must accept the space invitation before joining. Otherwise, creating a membership adds the member directly to the specified space. Supports the following types of [authentication](https://developers.google.com/workspace/chat/authenticate-authorize): - [App authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) with [administrator approval](https://support.google.com/a?p=chat-app-auth) and the authorization scope: - `https://www.googleapis.com/auth/chat.app.memberships` - [User authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with one of the following authorization scopes: - `https://www.googleapis.com/auth/chat.memberships` - `https://www.googleapis.com/auth/chat.memberships.app` (to add the calling app to the space) - `https://www.googleapis.com/auth/chat.import` (import mode spaces only) - User authentication grants administrator privileges when an administrator account authenticates, `use_admin_access` is `true`, and the following authorization scope is used: - `https://www.googleapis.com/auth/chat.admin.memberships` App authentication is not supported for the following use cases: - Inviting users external to the Workspace organization that owns the space. - Adding a Google Group to a space. - Adding a Chat app to a space. For example usage, see: - [Invite or add a user to a space](https://developers.google.com/workspace/chat/create-members#create-user-membership). - [Invite or add a Google Group to a space](https://developers.google.com/workspace/chat/create-members#create-group-membership). - [Add the Chat app to a space](https://developers.google.com/workspace/chat/create-members#create-membership-calling-api).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role` | String |  | Optional. User's role within a Chat space, which determines their permitted actions in the space. This field can only be used as input in `UpdateMembership`. |
| `create_time` | String |  | Optional. Immutable. The creation time of the membership, such as when a member joined or was invited to join a space. This field is output only, except when used to import historical memberships in import mode spaces. |
| `group_member` | String |  | Optional. The Google Group the membership corresponds to. Reading or mutating memberships for Google Groups requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user). |
| `delete_time` | String |  | Optional. Immutable. The deletion time of the membership, such as when a member left or was removed from a space. This field is output only, except when used to import historical memberships in import mode spaces. |
| `state` | String |  | Output only. State of the membership. |
| `member` | String |  | Optional. The Google Chat user or app the membership corresponds to. If your Chat app [authenticates as a user](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user), the output populates the [user](https://developers.google.com/workspace/chat/api/reference/rest/v1/User) `name` and `type`. |
| `name` | String |  | Identifier. Resource name of the membership, assigned by the server. Format: `spaces/{space}/members/{member}` |
| `parent` | String | ✅ | Required. The resource name of the space for which to create the membership. Format: spaces/{space} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role` | String | Optional. User's role within a Chat space, which determines their permitted actions in the space. This field can only be used as input in `UpdateMembership`. |
| `create_time` | String | Optional. Immutable. The creation time of the membership, such as when a member joined or was invited to join a space. This field is output only, except when used to import historical memberships in import mode spaces. |
| `group_member` | String | Optional. The Google Group the membership corresponds to. Reading or mutating memberships for Google Groups requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user). |
| `delete_time` | String | Optional. Immutable. The deletion time of the membership, such as when a member left or was removed from a space. This field is output only, except when used to import historical memberships in import mode spaces. |
| `state` | String | Output only. State of the membership. |
| `member` | String | Optional. The Google Chat user or app the membership corresponds to. If your Chat app [authenticates as a user](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user), the output populates the [user](https://developers.google.com/workspace/chat/api/reference/rest/v1/User) `name` and `type`. |
| `name` | String | Identifier. Resource name of the membership, assigned by the server. Format: `spaces/{space}/members/{member}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create member
member = provider.chat_api.Member {
    parent = "value"  # Required. The resource name of the space for which to create the membership. Format: spaces/{space}
}

# Access member outputs
member_id = member.id
member_role = member.role
member_create_time = member.create_time
member_group_member = member.group_member
member_delete_time = member.delete_time
member_state = member.state
member_member = member.member
member_name = member.name
```

---


### Message

Creates a message in a Google Chat space. For an example, see [Send a message](https://developers.google.com/workspace/chat/create-messages). Supports the following types of [authentication](https://developers.google.com/workspace/chat/authenticate-authorize): - [App authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) with the authorization scope: - `https://www.googleapis.com/auth/chat.bot` - [User authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with one of the following authorization scopes: - `https://www.googleapis.com/auth/chat.messages.create` - `https://www.googleapis.com/auth/chat.messages` - `https://www.googleapis.com/auth/chat.import` (import mode spaces only) Chat attributes the message sender differently depending on the type of authentication that you use in your request. The following image shows how Chat attributes a message when you use app authentication. Chat displays the Chat app as the message sender. The content of the message can contain text (`text`), cards (`cardsV2`), and accessory widgets (`accessoryWidgets`). ![Message sent with app authentication](https://developers.google.com/workspace/chat/images/message-app-auth.svg) The following image shows how Chat attributes a message when you use user authentication. Chat displays the user as the message sender and attributes the Chat app to the message by displaying its name. The content of message can only contain text (`text`). ![Message sent with user authentication](https://developers.google.com/workspace/chat/images/message-user-auth.svg) The maximum message size, including the message contents, is 32,000 bytes. For [webhook](https://developers.google.com/workspace/chat/quickstart/webhooks) requests, the response doesn't contain the full message. The response only populates the `name` and `thread.name` fields in addition to the information that was in the request.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `last_update_time` | String |  | Output only. The time at which the message was last edited by a user. If the message has never been edited, this field is empty. |
| `quoted_message_metadata` | String |  | Optional. Information about a message that another message quotes. When you create a message, you can quote messages within the same thread, or quote a root message to create a new root message. However, you can't quote a message reply from a different thread. When you update a message, you can't add or replace the `quotedMessageMetadata` field, but you can remove it. For example usage, see [Quote another message](https://developers.google.com/workspace/chat/create-messages#quote-a-message). |
| `space` | String |  | Output only. If your Chat app [authenticates as a user](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user), the output only populates the [space](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces) `name`. |
| `formatted_text` | String |  | Output only. Contains the message `text` with markups added to communicate formatting. This field might not capture all formatting visible in the UI, but includes the following: * [Markup syntax](https://developers.google.com/workspace/chat/format-messages) for bold, italic, strikethrough, monospace, monospace block, and bulleted list. * [User mentions](https://developers.google.com/workspace/chat/format-messages#messages-@mention) using the format ``. * Custom hyperlinks using the format `<{url}|{rendered_text}>` where the first string is the URL and the second is the rendered text—for example, ``. * Custom emoji using the format `:{emoji_name}:`—for example, `:smile:`. This doesn't apply to Unicode emoji, such as `U+1F600` for a grinning face emoji. * Bullet list items using asterisks (`*`)—for example, `* item`. For more information, see [View text formatting sent in a message](https://developers.google.com/workspace/chat/format-messages#view_text_formatting_sent_in_a_message) |
| `annotations` | Vec<String> |  | Output only. Annotations can be associated with the plain-text body of the message or with chips that link to Google Workspace resources like Google Docs or Sheets with `start_index` and `length` of 0. |
| `matched_url` | String |  | Output only. A URL in `spaces.messages.text` that matches a link preview pattern. For more information, see [Preview links](https://developers.google.com/workspace/chat/preview-links). |
| `name` | String |  | Identifier. Resource name of the message. Format: `spaces/{space}/messages/{message}` Where `{space}` is the ID of the space where the message is posted and `{message}` is a system-assigned ID for the message. For example, `spaces/AAAAAAAAAAA/messages/BBBBBBBBBBB.BBBBBBBBBBB`. If you set a custom ID when you create a message, you can use this ID to specify the message in a request by replacing `{message}` with the value from the `clientAssignedMessageId` field. For example, `spaces/AAAAAAAAAAA/messages/client-custom-name`. For details, see [Name a message](https://developers.google.com/workspace/chat/create-messages#name_a_created_message). |
| `sender` | String |  | Output only. The user who created the message. If your Chat app [authenticates as a user](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user), the output populates the [user](https://developers.google.com/workspace/chat/api/reference/rest/v1/User) `name` and `type`. |
| `text` | String |  | Optional. Plain-text body of the message. The first link to an image, video, or web page generates a [preview chip](https://developers.google.com/workspace/chat/preview-links). You can also [@mention a Google Chat user](https://developers.google.com/workspace/chat/format-messages#messages-@mention), or everyone in the space. To learn about creating text messages, see [Send a message](https://developers.google.com/workspace/chat/create-messages). |
| `client_assigned_message_id` | String |  | Optional. A custom ID for the message. You can use field to identify a message, or to get, delete, or update a message. To set a custom ID, specify the [`messageId`](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.messages/create#body.QUERY_PARAMETERS.message_id) field when you create the message. For details, see [Name a message](https://developers.google.com/workspace/chat/create-messages#name_a_created_message). |
| `attachment` | Vec<String> |  | Optional. User-uploaded attachment. |
| `delete_time` | String |  | Output only. The time at which the message was deleted in Google Chat. If the message is never deleted, this field is empty. |
| `private_message_viewer` | String |  | Optional. Immutable. Input for creating a message, otherwise output only. The user that can view the message. When set, the message is private and only visible to the specified user and the Chat app. To include this field in your request, you must call the Chat API using [app authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) and omit the following: * [Attachments](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.messages.attachments) * [Accessory widgets](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.messages#Message.AccessoryWidget) For details, see [Send a message privately](https://developers.google.com/workspace/chat/create-messages#private). |
| `thread` | String |  | The thread the message belongs to. For example usage, see [Start or reply to a message thread](https://developers.google.com/workspace/chat/create-messages#create-message-thread). |
| `thread_reply` | bool |  | Output only. When `true`, the message is a response in a reply thread. When `false`, the message is visible in the space's top-level conversation as either the first message of a thread or a message with no threaded replies. If the space doesn't support reply in threads, this field is always `false`. |
| `argument_text` | String |  | Output only. Plain-text body of the message with all Chat app mentions stripped out. |
| `action_response` | String |  | Input only. Parameters that a Chat app can use to configure how its response is posted. |
| `deletion_metadata` | String |  | Output only. Information about a deleted message. A message is deleted when `delete_time` is set. |
| `fallback_text` | String |  | Optional. A plain-text description of the message's cards, used when the actual cards can't be displayed—for example, mobile notifications. |
| `emoji_reaction_summaries` | Vec<String> |  | Output only. The list of emoji reaction summaries on the message. |
| `slash_command` | String |  | Output only. Slash command information, if applicable. |
| `accessory_widgets` | Vec<String> |  | Optional. One or more interactive widgets that appear at the bottom of a message. You can add accessory widgets to messages that contain text, cards, or both text and cards. Not supported for messages that contain dialogs. For details, see [Add interactive widgets at the bottom of a message](https://developers.google.com/workspace/chat/create-messages#add-accessory-widgets). Creating a message with accessory widgets requires [app authentication] (https://developers.google.com/workspace/chat/authenticate-authorize-chat-app). |
| `cards` | Vec<String> |  | Deprecated: Use `cards_v2` instead. Rich, formatted, and interactive cards that you can use to display UI elements such as: formatted texts, buttons, and clickable images. Cards are normally displayed below the plain-text body of the message. `cards` and `cards_v2` can have a maximum size of 32 KB. |
| `attached_gifs` | Vec<String> |  | Output only. GIF images that are attached to the message. |
| `create_time` | String |  | Optional. Immutable. For spaces created in Chat, the time at which the message was created. This field is output only, except when used in import mode spaces. For import mode spaces, set this field to the historical timestamp at which the message was created in the source in order to preserve the original creation time. |
| `cards_v2` | Vec<String> |  | Optional. An array of [cards](https://developers.google.com/workspace/chat/api/reference/rest/v1/cards). Only Chat apps can create cards. If your Chat app [authenticates as a user](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user), the messages can't contain cards. To learn how to create a message that contains cards, see [Send a message](https://developers.google.com/workspace/chat/create-messages). [Card builder](https://addons.gsuite.google.com/uikit/builder) |
| `parent` | String | ✅ | Required. The resource name of the space in which to create a message. Format: `spaces/{space}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_update_time` | String | Output only. The time at which the message was last edited by a user. If the message has never been edited, this field is empty. |
| `quoted_message_metadata` | String | Optional. Information about a message that another message quotes. When you create a message, you can quote messages within the same thread, or quote a root message to create a new root message. However, you can't quote a message reply from a different thread. When you update a message, you can't add or replace the `quotedMessageMetadata` field, but you can remove it. For example usage, see [Quote another message](https://developers.google.com/workspace/chat/create-messages#quote-a-message). |
| `space` | String | Output only. If your Chat app [authenticates as a user](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user), the output only populates the [space](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces) `name`. |
| `formatted_text` | String | Output only. Contains the message `text` with markups added to communicate formatting. This field might not capture all formatting visible in the UI, but includes the following: * [Markup syntax](https://developers.google.com/workspace/chat/format-messages) for bold, italic, strikethrough, monospace, monospace block, and bulleted list. * [User mentions](https://developers.google.com/workspace/chat/format-messages#messages-@mention) using the format ``. * Custom hyperlinks using the format `<{url}|{rendered_text}>` where the first string is the URL and the second is the rendered text—for example, ``. * Custom emoji using the format `:{emoji_name}:`—for example, `:smile:`. This doesn't apply to Unicode emoji, such as `U+1F600` for a grinning face emoji. * Bullet list items using asterisks (`*`)—for example, `* item`. For more information, see [View text formatting sent in a message](https://developers.google.com/workspace/chat/format-messages#view_text_formatting_sent_in_a_message) |
| `annotations` | Vec<String> | Output only. Annotations can be associated with the plain-text body of the message or with chips that link to Google Workspace resources like Google Docs or Sheets with `start_index` and `length` of 0. |
| `matched_url` | String | Output only. A URL in `spaces.messages.text` that matches a link preview pattern. For more information, see [Preview links](https://developers.google.com/workspace/chat/preview-links). |
| `name` | String | Identifier. Resource name of the message. Format: `spaces/{space}/messages/{message}` Where `{space}` is the ID of the space where the message is posted and `{message}` is a system-assigned ID for the message. For example, `spaces/AAAAAAAAAAA/messages/BBBBBBBBBBB.BBBBBBBBBBB`. If you set a custom ID when you create a message, you can use this ID to specify the message in a request by replacing `{message}` with the value from the `clientAssignedMessageId` field. For example, `spaces/AAAAAAAAAAA/messages/client-custom-name`. For details, see [Name a message](https://developers.google.com/workspace/chat/create-messages#name_a_created_message). |
| `sender` | String | Output only. The user who created the message. If your Chat app [authenticates as a user](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user), the output populates the [user](https://developers.google.com/workspace/chat/api/reference/rest/v1/User) `name` and `type`. |
| `text` | String | Optional. Plain-text body of the message. The first link to an image, video, or web page generates a [preview chip](https://developers.google.com/workspace/chat/preview-links). You can also [@mention a Google Chat user](https://developers.google.com/workspace/chat/format-messages#messages-@mention), or everyone in the space. To learn about creating text messages, see [Send a message](https://developers.google.com/workspace/chat/create-messages). |
| `client_assigned_message_id` | String | Optional. A custom ID for the message. You can use field to identify a message, or to get, delete, or update a message. To set a custom ID, specify the [`messageId`](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.messages/create#body.QUERY_PARAMETERS.message_id) field when you create the message. For details, see [Name a message](https://developers.google.com/workspace/chat/create-messages#name_a_created_message). |
| `attachment` | Vec<String> | Optional. User-uploaded attachment. |
| `delete_time` | String | Output only. The time at which the message was deleted in Google Chat. If the message is never deleted, this field is empty. |
| `private_message_viewer` | String | Optional. Immutable. Input for creating a message, otherwise output only. The user that can view the message. When set, the message is private and only visible to the specified user and the Chat app. To include this field in your request, you must call the Chat API using [app authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) and omit the following: * [Attachments](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.messages.attachments) * [Accessory widgets](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.messages#Message.AccessoryWidget) For details, see [Send a message privately](https://developers.google.com/workspace/chat/create-messages#private). |
| `thread` | String | The thread the message belongs to. For example usage, see [Start or reply to a message thread](https://developers.google.com/workspace/chat/create-messages#create-message-thread). |
| `thread_reply` | bool | Output only. When `true`, the message is a response in a reply thread. When `false`, the message is visible in the space's top-level conversation as either the first message of a thread or a message with no threaded replies. If the space doesn't support reply in threads, this field is always `false`. |
| `argument_text` | String | Output only. Plain-text body of the message with all Chat app mentions stripped out. |
| `action_response` | String | Input only. Parameters that a Chat app can use to configure how its response is posted. |
| `deletion_metadata` | String | Output only. Information about a deleted message. A message is deleted when `delete_time` is set. |
| `fallback_text` | String | Optional. A plain-text description of the message's cards, used when the actual cards can't be displayed—for example, mobile notifications. |
| `emoji_reaction_summaries` | Vec<String> | Output only. The list of emoji reaction summaries on the message. |
| `slash_command` | String | Output only. Slash command information, if applicable. |
| `accessory_widgets` | Vec<String> | Optional. One or more interactive widgets that appear at the bottom of a message. You can add accessory widgets to messages that contain text, cards, or both text and cards. Not supported for messages that contain dialogs. For details, see [Add interactive widgets at the bottom of a message](https://developers.google.com/workspace/chat/create-messages#add-accessory-widgets). Creating a message with accessory widgets requires [app authentication] (https://developers.google.com/workspace/chat/authenticate-authorize-chat-app). |
| `cards` | Vec<String> | Deprecated: Use `cards_v2` instead. Rich, formatted, and interactive cards that you can use to display UI elements such as: formatted texts, buttons, and clickable images. Cards are normally displayed below the plain-text body of the message. `cards` and `cards_v2` can have a maximum size of 32 KB. |
| `attached_gifs` | Vec<String> | Output only. GIF images that are attached to the message. |
| `create_time` | String | Optional. Immutable. For spaces created in Chat, the time at which the message was created. This field is output only, except when used in import mode spaces. For import mode spaces, set this field to the historical timestamp at which the message was created in the source in order to preserve the original creation time. |
| `cards_v2` | Vec<String> | Optional. An array of [cards](https://developers.google.com/workspace/chat/api/reference/rest/v1/cards). Only Chat apps can create cards. If your Chat app [authenticates as a user](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user), the messages can't contain cards. To learn how to create a message that contains cards, see [Send a message](https://developers.google.com/workspace/chat/create-messages). [Card builder](https://addons.gsuite.google.com/uikit/builder) |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create message
message = provider.chat_api.Message {
    parent = "value"  # Required. The resource name of the space in which to create a message. Format: `spaces/{space}`
}

# Access message outputs
message_id = message.id
message_last_update_time = message.last_update_time
message_quoted_message_metadata = message.quoted_message_metadata
message_space = message.space
message_formatted_text = message.formatted_text
message_annotations = message.annotations
message_matched_url = message.matched_url
message_name = message.name
message_sender = message.sender
message_text = message.text
message_client_assigned_message_id = message.client_assigned_message_id
message_attachment = message.attachment
message_delete_time = message.delete_time
message_private_message_viewer = message.private_message_viewer
message_thread = message.thread
message_thread_reply = message.thread_reply
message_argument_text = message.argument_text
message_action_response = message.action_response
message_deletion_metadata = message.deletion_metadata
message_fallback_text = message.fallback_text
message_emoji_reaction_summaries = message.emoji_reaction_summaries
message_slash_command = message.slash_command
message_accessory_widgets = message.accessory_widgets
message_cards = message.cards
message_attached_gifs = message.attached_gifs
message_create_time = message.create_time
message_cards_v2 = message.cards_v2
```

---


### Space_notification_setting

Gets the space notification setting. For an example, see [Get the caller's space notification setting](https://developers.google.com/workspace/chat/get-space-notification-setting). Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with the [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.users.spacesettings`

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `mute_setting` | String |  | The space notification mute setting. |
| `notification_setting` | String |  | The notification setting. |
| `name` | String |  | Identifier. The resource name of the space notification setting. Format: `users/{user}/spaces/{space}/spaceNotificationSetting`. |
| `name` | String | ✅ | Identifier. The resource name of the space notification setting. Format: `users/{user}/spaces/{space}/spaceNotificationSetting`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mute_setting` | String | The space notification mute setting. |
| `notification_setting` | String | The notification setting. |
| `name` | String | Identifier. The resource name of the space notification setting. Format: `users/{user}/spaces/{space}/spaceNotificationSetting`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access space_notification_setting outputs
space_notification_setting_id = space_notification_setting.id
space_notification_setting_mute_setting = space_notification_setting.mute_setting
space_notification_setting_notification_setting = space_notification_setting.notification_setting
space_notification_setting_name = space_notification_setting.name
```

---


### Thread

Returns details about a user's read state within a thread, used to identify read and unread messages. For an example, see [Get details about a user's thread read state](https://developers.google.com/workspace/chat/get-thread-read-state). Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with one of the following [authorization scopes](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.users.readstate.readonly` - `https://www.googleapis.com/auth/chat.users.readstate`

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `last_read_time` | String | The time when the user's thread read state was updated. Usually this corresponds with the timestamp of the last read message in a thread. |
| `name` | String | Resource name of the thread read state. Format: `users/{user}/spaces/{space}/threads/{thread}/threadReadState` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access thread outputs
thread_id = thread.id
thread_last_read_time = thread.last_read_time
thread_name = thread.name
```

---


### Attachment

Gets the metadata of a message attachment. The attachment data is fetched using the [media API](https://developers.google.com/workspace/chat/api/reference/rest/v1/media/download). For an example, see [Get metadata about a message attachment](https://developers.google.com/workspace/chat/get-media-attachments). Requires [app authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) with the [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.bot`

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `drive_data_ref` | String | Output only. A reference to the Google Drive attachment. This field is used with the Google Drive API. |
| `content_type` | String | Output only. The content type (MIME type) of the file. |
| `source` | String | Output only. The source of the attachment. |
| `download_uri` | String | Output only. The download URL which should be used to allow a human user to download the attachment. Chat apps shouldn't use this URL to download attachment content. |
| `name` | String | Optional. Resource name of the attachment, in the form `spaces/{space}/messages/{message}/attachments/{attachment}`. |
| `thumbnail_uri` | String | Output only. The thumbnail URL which should be used to preview the attachment to a human user. Chat apps shouldn't use this URL to download attachment content. |
| `attachment_data_ref` | String | Optional. A reference to the attachment data. This field is used to create or update messages with attachments, or with the media API to download the attachment data. |
| `content_name` | String | Output only. The original file name for the content, not the full path. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access attachment outputs
attachment_id = attachment.id
attachment_drive_data_ref = attachment.drive_data_ref
attachment_content_type = attachment.content_type
attachment_source = attachment.source
attachment_download_uri = attachment.download_uri
attachment_name = attachment.name
attachment_thumbnail_uri = attachment.thumbnail_uri
attachment_attachment_data_ref = attachment.attachment_data_ref
attachment_content_name = attachment.content_name
```

---


### Custom_emoji

Creates a custom emoji. Custom emojis are only available for Google Workspace accounts, and the administrator must turn custom emojis on for the organization. For more information, see [Learn about custom emojis in Google Chat](https://support.google.com/chat/answer/12800149) and [Manage custom emoji permissions](https://support.google.com/a/answer/12850085). Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with the [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.customemojis`

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `temporary_image_uri` | String |  | Output only. A temporary image URL for the custom emoji, valid for at least 10 minutes. Note that this is not populated in the response when the custom emoji is created. |
| `name` | String |  | Identifier. The resource name of the custom emoji, assigned by the server. Format: `customEmojis/{customEmoji}` |
| `payload` | String |  | Optional. Input only. Payload data. Required when the custom emoji is created. |
| `emoji_name` | String |  | Optional. Immutable. User-provided name for the custom emoji, which is unique within the organization. Required when the custom emoji is created, output only otherwise. Emoji names must start and end with colons, must be lowercase and can only contain alphanumeric characters, hyphens, and underscores. Hyphens and underscores should be used to separate words and cannot be used consecutively. Example: `:valid-emoji-name:` |
| `uid` | String |  | Output only. Unique key for the custom emoji resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `temporary_image_uri` | String | Output only. A temporary image URL for the custom emoji, valid for at least 10 minutes. Note that this is not populated in the response when the custom emoji is created. |
| `name` | String | Identifier. The resource name of the custom emoji, assigned by the server. Format: `customEmojis/{customEmoji}` |
| `payload` | String | Optional. Input only. Payload data. Required when the custom emoji is created. |
| `emoji_name` | String | Optional. Immutable. User-provided name for the custom emoji, which is unique within the organization. Required when the custom emoji is created, output only otherwise. Emoji names must start and end with colons, must be lowercase and can only contain alphanumeric characters, hyphens, and underscores. Hyphens and underscores should be used to separate words and cannot be used consecutively. Example: `:valid-emoji-name:` |
| `uid` | String | Output only. Unique key for the custom emoji resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_emoji
custom_emoji = provider.chat_api.Custom_emoji {
}

# Access custom_emoji outputs
custom_emoji_id = custom_emoji.id
custom_emoji_temporary_image_uri = custom_emoji.temporary_image_uri
custom_emoji_name = custom_emoji.name
custom_emoji_payload = custom_emoji.payload
custom_emoji_emoji_name = custom_emoji.emoji_name
custom_emoji_uid = custom_emoji.uid
```

---


### Space_event

Returns an event from a Google Chat space. The [event payload](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.spaceEvents#SpaceEvent.FIELDS.oneof_payload) contains the most recent version of the resource that changed. For example, if you request an event about a new message but the message was later updated, the server returns the updated `Message` resource in the event payload. Note: The `permissionSettings` field is not returned in the Space object of the Space event data for this request. Supports the following types of [authentication](https://developers.google.com/workspace/chat/authenticate-authorize) with an [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes) appropriate for reading the requested data: - [App authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) with [administrator approval](https://support.google.com/a?p=chat-app-auth) in [Developer Preview](https://developers.google.com/workspace/preview) with one of the following authorization scopes: - `https://www.googleapis.com/auth/chat.app.spaces` - `https://www.googleapis.com/auth/chat.app.messages.readonly` - `https://www.googleapis.com/auth/chat.app.memberships` - [User authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with one of the following authorization scopes: - `https://www.googleapis.com/auth/chat.spaces.readonly` - `https://www.googleapis.com/auth/chat.spaces` - `https://www.googleapis.com/auth/chat.messages.readonly` - `https://www.googleapis.com/auth/chat.messages` - `https://www.googleapis.com/auth/chat.messages.reactions.readonly` - `https://www.googleapis.com/auth/chat.messages.reactions` - `https://www.googleapis.com/auth/chat.memberships.readonly` - `https://www.googleapis.com/auth/chat.memberships` To get an event, the authenticated caller must be a member of the space. For an example, see [Get details about an event from a Google Chat space](https://developers.google.com/workspace/chat/get-space-event).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_time` | String | Time when the event occurred. |
| `membership_batch_updated_event_data` | String | Event payload for multiple updated memberships. Event type: `google.workspace.chat.membership.v1.batchUpdated` |
| `reaction_batch_created_event_data` | String | Event payload for multiple new reactions. Event type: `google.workspace.chat.reaction.v1.batchCreated` |
| `membership_batch_deleted_event_data` | String | Event payload for multiple deleted memberships. Event type: `google.workspace.chat.membership.v1.batchDeleted` |
| `reaction_deleted_event_data` | String | Event payload for a deleted reaction. Event type: `google.workspace.chat.reaction.v1.deleted` |
| `event_type` | String | Type of space event. Each event type has a batch version, which represents multiple instances of the event type that occur in a short period of time. For `spaceEvents.list()` requests, omit batch event types in your query filter. By default, the server returns both event type and its batch version. Supported event types for [messages](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.messages): * New message: `google.workspace.chat.message.v1.created` * Updated message: `google.workspace.chat.message.v1.updated` * Deleted message: `google.workspace.chat.message.v1.deleted` * Multiple new messages: `google.workspace.chat.message.v1.batchCreated` * Multiple updated messages: `google.workspace.chat.message.v1.batchUpdated` * Multiple deleted messages: `google.workspace.chat.message.v1.batchDeleted` Supported event types for [memberships](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.members): * New membership: `google.workspace.chat.membership.v1.created` * Updated membership: `google.workspace.chat.membership.v1.updated` * Deleted membership: `google.workspace.chat.membership.v1.deleted` * Multiple new memberships: `google.workspace.chat.membership.v1.batchCreated` * Multiple updated memberships: `google.workspace.chat.membership.v1.batchUpdated` * Multiple deleted memberships: `google.workspace.chat.membership.v1.batchDeleted` Supported event types for [reactions](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.messages.reactions): * New reaction: `google.workspace.chat.reaction.v1.created` * Deleted reaction: `google.workspace.chat.reaction.v1.deleted` * Multiple new reactions: `google.workspace.chat.reaction.v1.batchCreated` * Multiple deleted reactions: `google.workspace.chat.reaction.v1.batchDeleted` Supported event types about the [space](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces): * Updated space: `google.workspace.chat.space.v1.updated` * Multiple space updates: `google.workspace.chat.space.v1.batchUpdated` |
| `membership_batch_created_event_data` | String | Event payload for multiple new memberships. Event type: `google.workspace.chat.membership.v1.batchCreated` |
| `membership_updated_event_data` | String | Event payload for an updated membership. Event type: `google.workspace.chat.membership.v1.updated` |
| `message_batch_deleted_event_data` | String | Event payload for multiple deleted messages. Event type: `google.workspace.chat.message.v1.batchDeleted` |
| `message_batch_created_event_data` | String | Event payload for multiple new messages. Event type: `google.workspace.chat.message.v1.batchCreated` |
| `reaction_batch_deleted_event_data` | String | Event payload for multiple deleted reactions. Event type: `google.workspace.chat.reaction.v1.batchDeleted` |
| `space_batch_updated_event_data` | String | Event payload for multiple updates to a space. Event type: `google.workspace.chat.space.v1.batchUpdated` |
| `membership_deleted_event_data` | String | Event payload for a deleted membership. Event type: `google.workspace.chat.membership.v1.deleted` |
| `message_created_event_data` | String | Event payload for a new message. Event type: `google.workspace.chat.message.v1.created` |
| `message_deleted_event_data` | String | Event payload for a deleted message. Event type: `google.workspace.chat.message.v1.deleted` |
| `message_updated_event_data` | String | Event payload for an updated message. Event type: `google.workspace.chat.message.v1.updated` |
| `membership_created_event_data` | String | Event payload for a new membership. Event type: `google.workspace.chat.membership.v1.created` |
| `name` | String | Resource name of the space event. Format: `spaces/{space}/spaceEvents/{spaceEvent}` |
| `reaction_created_event_data` | String | Event payload for a new reaction. Event type: `google.workspace.chat.reaction.v1.created` |
| `space_updated_event_data` | String | Event payload for a space update. Event type: `google.workspace.chat.space.v1.updated` |
| `message_batch_updated_event_data` | String | Event payload for multiple updated messages. Event type: `google.workspace.chat.message.v1.batchUpdated` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access space_event outputs
space_event_id = space_event.id
space_event_event_time = space_event.event_time
space_event_membership_batch_updated_event_data = space_event.membership_batch_updated_event_data
space_event_reaction_batch_created_event_data = space_event.reaction_batch_created_event_data
space_event_membership_batch_deleted_event_data = space_event.membership_batch_deleted_event_data
space_event_reaction_deleted_event_data = space_event.reaction_deleted_event_data
space_event_event_type = space_event.event_type
space_event_membership_batch_created_event_data = space_event.membership_batch_created_event_data
space_event_membership_updated_event_data = space_event.membership_updated_event_data
space_event_message_batch_deleted_event_data = space_event.message_batch_deleted_event_data
space_event_message_batch_created_event_data = space_event.message_batch_created_event_data
space_event_reaction_batch_deleted_event_data = space_event.reaction_batch_deleted_event_data
space_event_space_batch_updated_event_data = space_event.space_batch_updated_event_data
space_event_membership_deleted_event_data = space_event.membership_deleted_event_data
space_event_message_created_event_data = space_event.message_created_event_data
space_event_message_deleted_event_data = space_event.message_deleted_event_data
space_event_message_updated_event_data = space_event.message_updated_event_data
space_event_membership_created_event_data = space_event.membership_created_event_data
space_event_name = space_event.name
space_event_reaction_created_event_data = space_event.reaction_created_event_data
space_event_space_updated_event_data = space_event.space_updated_event_data
space_event_message_batch_updated_event_data = space_event.message_batch_updated_event_data
```

---


### Media

Uploads an attachment. For an example, see [Upload media as a file attachment](https://developers.google.com/workspace/chat/upload-media-attachments). Requires user [authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with one of the following [authorization scopes](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.messages.create` - `https://www.googleapis.com/auth/chat.messages` - `https://www.googleapis.com/auth/chat.import` (import mode spaces only) You can upload attachments up to 200 MB. Certain file types aren't supported. For details, see [File types blocked by Google Chat](https://support.google.com/chat/answer/7651457?&co=GENIE.Platform%3DDesktop#File%20types%20blocked%20in%20Google%20Chat).

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filename` | String |  | Required. The filename of the attachment, including the file extension. |
| `parent` | String | ✅ | Required. Resource name of the Chat space in which the attachment is uploaded. Format "spaces/{space}". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `resource_name` | String | Name of the media resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.chat_api.Media {
    parent = "value"  # Required. Resource name of the Chat space in which the attachment is uploaded. Format "spaces/{space}".
}

# Access media outputs
media_id = media.id
media_resource_name = media.resource_name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple space resources
space_0 = provider.chat_api.Space {
}
space_1 = provider.chat_api.Space {
}
space_2 = provider.chat_api.Space {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    space = provider.chat_api.Space {
    }
```

---

## Related Documentation

- [GCP Chat_api Documentation](https://cloud.google.com/chat_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
