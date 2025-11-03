# Groupssettings_api Service



**Resources**: 1

---

## Overview

The groupssettings_api service provides access to 1 resource type:

- [Group](#group) [RU]

---

## Resources


### Group

Gets one resource by id.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `who_can_discover_group` | String |  | Specifies the set of users for whom this group is discoverable. Possible values are:  
- ANYONE_CAN_DISCOVER 
- ALL_IN_DOMAIN_CAN_DISCOVER 
- ALL_MEMBERS_CAN_DISCOVER |
| `who_can_contact_owner` | String |  | Permission to contact owner of the group via web UI. Possible values are:  
- ALL_IN_DOMAIN_CAN_CONTACT 
- ALL_MANAGERS_CAN_CONTACT 
- ALL_MEMBERS_CAN_CONTACT 
- ANYONE_CAN_CONTACT 
- ALL_OWNERS_CAN_CONTACT |
| `who_can_make_topics_sticky` | String |  | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can make topics appear at the top of the topic list. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `allow_external_members` | String |  | Identifies whether members external to your organization can join the group. Possible values are:  
- true: G Suite users external to your organization can become members of this group. 
- false: Users not belonging to the organization are not allowed to become members of this group. |
| `who_can_enter_free_form_tags` | String |  | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to enter free form tags for topics in a forum. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `archive_only` | String |  | Allows the group to be archived only. Possible values are:  
- true: Group is archived and the group is inactive. New messages to this group are rejected. The older archived messages are browseable and searchable.  
- If true, the whoCanPostMessage property is set to NONE_CAN_POST.  
- If reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.  
- false: The group is active and can receive messages.  
- When false, updating whoCanPostMessage to NONE_CAN_POST, results in an error. |
| `members_can_post_as_the_group` | String |  | Enables members to post messages as the group. Possible values are:  
- true: Group member can post messages using the group's email address instead of their own email address. Message appear to originate from the group itself. Note: When true, any message moderation settings on individual users or new members do not apply to posts made on behalf of the group. 
- false: Members can not post in behalf of the group's email address. |
| `who_can_view_membership` | String |  | Permissions to view membership. Possible values are:  
- ALL_IN_DOMAIN_CAN_VIEW: Anyone in the account can view the group members list.
If a group already has external members, those members can still send email to this group.
 
- ALL_MEMBERS_CAN_VIEW: The group members can view the group members list. 
- ALL_MANAGERS_CAN_VIEW: The group managers can view group members list. |
| `who_can_moderate_content` | String |  | Specifies who can moderate content. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `who_can_add` | String |  | Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to add members. Possible values are:  
- ALL_MEMBERS_CAN_ADD: Managers and members can directly add new members. 
- ALL_MANAGERS_CAN_ADD: Only managers can directly add new members. this includes the group's owner. 
- ALL_OWNERS_CAN_ADD: Only owners can directly add new members. 
- NONE_CAN_ADD: No one can directly add new members. |
| `who_can_mark_duplicate` | String |  | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as a duplicate of another topic. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_approve_members` | String |  | Specifies who can approve members who ask to join groups. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:  
- ALL_MEMBERS_CAN_APPROVE 
- ALL_MANAGERS_CAN_APPROVE 
- ALL_OWNERS_CAN_APPROVE 
- NONE_CAN_APPROVE |
| `allow_web_posting` | String |  | Allows posting from web. Possible values are:  
- true: Allows any member to post to the group forum. 
- false: Members only use Gmail to communicate with the group. |
| `who_can_take_topics` | String |  | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to take topics in a forum. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_join` | String |  | Permission to join group. Possible values are:  
- ANYONE_CAN_JOIN: Any Internet user who is outside your domain can access your Google Groups service and view the list of groups in your Groups directory. Warning: Group owners can add external addresses, outside of the domain to their groups. They can also allow people outside your domain to join their groups. If you later disable this option, any external addresses already added to users' groups remain in those groups. 
- ALL_IN_DOMAIN_CAN_JOIN: Anyone in the account domain can join. This includes accounts with multiple domains. 
- INVITED_CAN_JOIN: Candidates for membership can be invited to join.  
- CAN_REQUEST_TO_JOIN: Non members can request an invitation to join. |
| `who_can_unassign_topic` | String |  | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unassign any topic in a forum. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_view_group` | String |  | Permissions to view group messages. Possible values are:  
- ANYONE_CAN_VIEW: Any Internet user can view the group's messages.  
- ALL_IN_DOMAIN_CAN_VIEW: Anyone in your account can view this group's messages. 
- ALL_MEMBERS_CAN_VIEW: All group members can view the group's messages. 
- ALL_MANAGERS_CAN_VIEW: Any group manager can view this group's messages. |
| `who_can_lock_topics` | String |  | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can prevent users from posting replies to topics. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `who_can_ban_users` | String |  | Specifies who can deny membership to users. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `kind` | String |  | The type of the resource. It is always groupsSettings#groups. |
| `who_can_mark_favorite_reply_on_any_topic` | String |  | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark any other user's post as a favorite reply. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `custom_reply_to` | String |  | An email address used when replying to a message if the replyTo property is set to REPLY_TO_CUSTOM. This address is defined by an account administrator.  
- When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds a custom email address used when replying to a message. 
- If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a text value or an error is returned. |
| `who_can_assist_content` | String |  | Specifies who can moderate metadata. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_modify_tags_and_categories` | String |  | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to change tags and categories. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_assign_topics` | String |  | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to assign topics in a forum to another user. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_moderate_members` | String |  | Specifies who can manage members. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `email` | String |  | The group's email address. This property can be updated using the Directory API. Note: Only a group owner can change a group's email address. A group manager can't do this.
When you change your group's address using the Directory API or the control panel, you are changing the address your subscribers use to send email and the web address people use to access your group. People can't reach your group by visiting the old address. |
| `who_can_leave_group` | String |  | Permission to leave the group. Possible values are:  
- ALL_MANAGERS_CAN_LEAVE 
- ALL_MEMBERS_CAN_LEAVE 
- NONE_CAN_LEAVE |
| `who_can_delete_any_post` | String |  | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete replies to topics. (Authors can always delete their own posts). Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `who_can_move_topics_in` | String |  | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics into the group or forum. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `name` | String |  | Name of the group, which has a maximum size of 75 characters. |
| `who_can_approve_messages` | String |  | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can approve pending messages in the moderation queue. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `who_can_mark_no_response_needed` | String |  | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as not needing a response. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `allow_google_communication` | String |  | Deprecated. Allows Google to contact administrator of the group.  
- true: Allow Google to contact managers of this group. Occasionally Google may send updates on the latest features, ask for input on new features, or ask for permission to highlight your group. 
- false: Google can not contact managers of this group. |
| `include_custom_footer` | String |  | Whether to include custom footer. Possible values are:  
- true 
- false |
| `who_can_invite` | String |  | Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to invite new members. Possible values are:  
- ALL_MEMBERS_CAN_INVITE: Managers and members can invite a new member candidate. 
- ALL_MANAGERS_CAN_INVITE: Only managers can invite a new member. This includes the group's owner. 
- ALL_OWNERS_CAN_INVITE: Only owners can invite a new member. 
- NONE_CAN_INVITE: No one can invite a new member candidate. |
| `include_in_global_address_list` | String |  | Enables the group to be included in the Global Address List. For more information, see the help center. Possible values are:  
- true: Group is included in the Global Address List. 
- false: Group is not included in the Global Address List. |
| `default_sender` | String |  | Default sender for members who can post messages as the group. Possible values are: - `DEFAULT_SELF`: By default messages will be sent from the user - `GROUP`: By default messages will be sent from the group |
| `who_can_hide_abuse` | String |  | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can hide posts by reporting them as abuse. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `show_in_group_directory` | String |  | Deprecated. This is merged into the new whoCanDiscoverGroup setting. Allows the group to be visible in the Groups Directory. Possible values are:  
- true: All groups in the account are listed in the Groups directory. 
- false: All groups in the account are not listed in the directory. |
| `primary_language` | String |  | The primary language for group. For a group's primary language use the language tags from the G Suite languages found at G Suite Email Settings API Email Language Tags. |
| `who_can_unmark_favorite_reply_on_any_topic` | String |  | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unmark any post from a favorite reply. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `reply_to` | String |  | Specifies who receives the default reply. Possible values are:  
- REPLY_TO_CUSTOM: For replies to messages, use the group's custom email address.
When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds the custom email address used when replying to a message. If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a value. Otherwise an error is returned.
 
- REPLY_TO_SENDER: The reply sent to author of message. 
- REPLY_TO_LIST: This reply message is sent to the group. 
- REPLY_TO_OWNER: The reply is sent to the owner(s) of the group. This does not include the group's managers. 
- REPLY_TO_IGNORE: Group users individually decide where the message reply is sent. 
- REPLY_TO_MANAGERS: This reply message is sent to the group's managers, which includes all managers and the group owner. |
| `max_message_bytes` | i64 |  | Deprecated. The maximum size of a message is 25Mb. |
| `who_can_post_announcements` | String |  | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can post announcements, a special topic type. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `who_can_post_message` | String |  | Permissions to post messages. Possible values are:  
- NONE_CAN_POST: The group is disabled and archived. No one can post a message to this group.  
- When archiveOnly is false, updating whoCanPostMessage to NONE_CAN_POST, results in an error. 
- If archiveOnly is reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.  
- ALL_MANAGERS_CAN_POST: Managers, including group owners, can post messages. 
- ALL_MEMBERS_CAN_POST: Any group member can post a message. 
- ALL_OWNERS_CAN_POST: Only group owners can post a message. 
- ALL_IN_DOMAIN_CAN_POST: Anyone in the account can post a message.  
- ANYONE_CAN_POST: Any Internet user who outside your account can access your Google Groups service and post a message. Note: When whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam. |
| `message_display_font` | String |  | Deprecated. The default message display font always has a value of "DEFAULT_FONT". |
| `spam_moderation_level` | String |  | Specifies moderation levels for messages detected as spam. Possible values are:  
- ALLOW: Post the message to the group. 
- MODERATE: Send the message to the moderation queue. This is the default. 
- SILENTLY_MODERATE: Send the message to the moderation queue, but do not send notification to moderators. 
- REJECT: Immediately reject the message. |
| `favorite_replies_on_top` | String |  | Indicates if favorite replies should be displayed above other replies.  
- true: Favorite replies will be displayed above other replies. 
- false: Favorite replies will not be displayed above other replies. |
| `description` | String |  | Description of the group. This property value may be an empty string if no group description has been entered. If entered, the maximum group description is no more than 300 characters. |
| `enable_collaborative_inbox` | String |  | Specifies whether a collaborative inbox will remain turned on for the group. Possible values are:  
- true 
- false |
| `custom_footer_text` | String |  | Set the content of custom footer text. The maximum number of characters is 1,000. |
| `who_can_move_topics_out` | String |  | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics out of the group or forum. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `message_moderation_level` | String |  | Moderation level of incoming messages. Possible values are:  
- MODERATE_ALL_MESSAGES: All messages are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
- MODERATE_NON_MEMBERS: All messages from non group members are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
- MODERATE_NEW_MEMBERS: All messages from new members are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
- MODERATE_NONE: No moderator approval is required. Messages are delivered directly to the group. Note: When the whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam.
When memberCanPostAsTheGroup is true, any message moderation settings on individual users or new members will not apply to posts made on behalf of the group. |
| `who_can_delete_topics` | String |  | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete topics. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `is_archived` | String |  | Allows the Group contents to be archived. Possible values are:  
- true: Archive messages sent to the group. 
- false: Do not keep an archive of messages sent to this group. If false, previously archived messages remain in the archive. |
| `who_can_add_references` | String |  | Deprecated. This functionality is no longer supported in the Google Groups UI. The value is always "NONE". |
| `who_can_modify_members` | String |  | Deprecated. This is merged into the new whoCanModerateMembers setting. Specifies who can change group members' roles. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `custom_roles_enabled_for_settings_to_be_merged` | String |  | Specifies whether the group has a custom role that's included in one of the settings being merged. This field is read-only and update/patch requests to it are ignored. Possible values are:  
- true 
- false |
| `default_message_deny_notification_text` | String |  | When a message is rejected, this is text for the rejection notification sent to the message's author. By default, this property is empty and has no value in the API's response body. The maximum notification text size is 10,000 characters. Note: Requires sendMessageDenyNotification property to be true. |
| `who_can_mark_favorite_reply_on_own_topic` | String |  | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a post for a topic they started as a favorite reply. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `send_message_deny_notification` | String |  | Allows a member to be notified if the member's message to the group is denied by the group owner. Possible values are:  
- true: When a message is rejected, send the deny message notification to the message author.
The defaultMessageDenyNotificationText property is dependent on the sendMessageDenyNotification property being true.
 
- false: When a message is rejected, no notification is sent. |
| `group_unique_id` | String | ✅ | The group's email address. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `who_can_discover_group` | String | Specifies the set of users for whom this group is discoverable. Possible values are:  
- ANYONE_CAN_DISCOVER 
- ALL_IN_DOMAIN_CAN_DISCOVER 
- ALL_MEMBERS_CAN_DISCOVER |
| `who_can_contact_owner` | String | Permission to contact owner of the group via web UI. Possible values are:  
- ALL_IN_DOMAIN_CAN_CONTACT 
- ALL_MANAGERS_CAN_CONTACT 
- ALL_MEMBERS_CAN_CONTACT 
- ANYONE_CAN_CONTACT 
- ALL_OWNERS_CAN_CONTACT |
| `who_can_make_topics_sticky` | String | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can make topics appear at the top of the topic list. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `allow_external_members` | String | Identifies whether members external to your organization can join the group. Possible values are:  
- true: G Suite users external to your organization can become members of this group. 
- false: Users not belonging to the organization are not allowed to become members of this group. |
| `who_can_enter_free_form_tags` | String | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to enter free form tags for topics in a forum. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `archive_only` | String | Allows the group to be archived only. Possible values are:  
- true: Group is archived and the group is inactive. New messages to this group are rejected. The older archived messages are browseable and searchable.  
- If true, the whoCanPostMessage property is set to NONE_CAN_POST.  
- If reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.  
- false: The group is active and can receive messages.  
- When false, updating whoCanPostMessage to NONE_CAN_POST, results in an error. |
| `members_can_post_as_the_group` | String | Enables members to post messages as the group. Possible values are:  
- true: Group member can post messages using the group's email address instead of their own email address. Message appear to originate from the group itself. Note: When true, any message moderation settings on individual users or new members do not apply to posts made on behalf of the group. 
- false: Members can not post in behalf of the group's email address. |
| `who_can_view_membership` | String | Permissions to view membership. Possible values are:  
- ALL_IN_DOMAIN_CAN_VIEW: Anyone in the account can view the group members list.
If a group already has external members, those members can still send email to this group.
 
- ALL_MEMBERS_CAN_VIEW: The group members can view the group members list. 
- ALL_MANAGERS_CAN_VIEW: The group managers can view group members list. |
| `who_can_moderate_content` | String | Specifies who can moderate content. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `who_can_add` | String | Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to add members. Possible values are:  
- ALL_MEMBERS_CAN_ADD: Managers and members can directly add new members. 
- ALL_MANAGERS_CAN_ADD: Only managers can directly add new members. this includes the group's owner. 
- ALL_OWNERS_CAN_ADD: Only owners can directly add new members. 
- NONE_CAN_ADD: No one can directly add new members. |
| `who_can_mark_duplicate` | String | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as a duplicate of another topic. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_approve_members` | String | Specifies who can approve members who ask to join groups. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:  
- ALL_MEMBERS_CAN_APPROVE 
- ALL_MANAGERS_CAN_APPROVE 
- ALL_OWNERS_CAN_APPROVE 
- NONE_CAN_APPROVE |
| `allow_web_posting` | String | Allows posting from web. Possible values are:  
- true: Allows any member to post to the group forum. 
- false: Members only use Gmail to communicate with the group. |
| `who_can_take_topics` | String | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to take topics in a forum. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_join` | String | Permission to join group. Possible values are:  
- ANYONE_CAN_JOIN: Any Internet user who is outside your domain can access your Google Groups service and view the list of groups in your Groups directory. Warning: Group owners can add external addresses, outside of the domain to their groups. They can also allow people outside your domain to join their groups. If you later disable this option, any external addresses already added to users' groups remain in those groups. 
- ALL_IN_DOMAIN_CAN_JOIN: Anyone in the account domain can join. This includes accounts with multiple domains. 
- INVITED_CAN_JOIN: Candidates for membership can be invited to join.  
- CAN_REQUEST_TO_JOIN: Non members can request an invitation to join. |
| `who_can_unassign_topic` | String | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unassign any topic in a forum. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_view_group` | String | Permissions to view group messages. Possible values are:  
- ANYONE_CAN_VIEW: Any Internet user can view the group's messages.  
- ALL_IN_DOMAIN_CAN_VIEW: Anyone in your account can view this group's messages. 
- ALL_MEMBERS_CAN_VIEW: All group members can view the group's messages. 
- ALL_MANAGERS_CAN_VIEW: Any group manager can view this group's messages. |
| `who_can_lock_topics` | String | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can prevent users from posting replies to topics. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `who_can_ban_users` | String | Specifies who can deny membership to users. This permission will be deprecated once it is merged into the new whoCanModerateMembers setting. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `kind` | String | The type of the resource. It is always groupsSettings#groups. |
| `who_can_mark_favorite_reply_on_any_topic` | String | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark any other user's post as a favorite reply. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `custom_reply_to` | String | An email address used when replying to a message if the replyTo property is set to REPLY_TO_CUSTOM. This address is defined by an account administrator.  
- When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds a custom email address used when replying to a message. 
- If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a text value or an error is returned. |
| `who_can_assist_content` | String | Specifies who can moderate metadata. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_modify_tags_and_categories` | String | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to change tags and categories. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_assign_topics` | String | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to assign topics in a forum to another user. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `who_can_moderate_members` | String | Specifies who can manage members. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `email` | String | The group's email address. This property can be updated using the Directory API. Note: Only a group owner can change a group's email address. A group manager can't do this.
When you change your group's address using the Directory API or the control panel, you are changing the address your subscribers use to send email and the web address people use to access your group. People can't reach your group by visiting the old address. |
| `who_can_leave_group` | String | Permission to leave the group. Possible values are:  
- ALL_MANAGERS_CAN_LEAVE 
- ALL_MEMBERS_CAN_LEAVE 
- NONE_CAN_LEAVE |
| `who_can_delete_any_post` | String | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete replies to topics. (Authors can always delete their own posts). Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `who_can_move_topics_in` | String | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics into the group or forum. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `name` | String | Name of the group, which has a maximum size of 75 characters. |
| `who_can_approve_messages` | String | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can approve pending messages in the moderation queue. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `who_can_mark_no_response_needed` | String | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a topic as not needing a response. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `allow_google_communication` | String | Deprecated. Allows Google to contact administrator of the group.  
- true: Allow Google to contact managers of this group. Occasionally Google may send updates on the latest features, ask for input on new features, or ask for permission to highlight your group. 
- false: Google can not contact managers of this group. |
| `include_custom_footer` | String | Whether to include custom footer. Possible values are:  
- true 
- false |
| `who_can_invite` | String | Deprecated. This is merged into the new whoCanModerateMembers setting. Permissions to invite new members. Possible values are:  
- ALL_MEMBERS_CAN_INVITE: Managers and members can invite a new member candidate. 
- ALL_MANAGERS_CAN_INVITE: Only managers can invite a new member. This includes the group's owner. 
- ALL_OWNERS_CAN_INVITE: Only owners can invite a new member. 
- NONE_CAN_INVITE: No one can invite a new member candidate. |
| `include_in_global_address_list` | String | Enables the group to be included in the Global Address List. For more information, see the help center. Possible values are:  
- true: Group is included in the Global Address List. 
- false: Group is not included in the Global Address List. |
| `default_sender` | String | Default sender for members who can post messages as the group. Possible values are: - `DEFAULT_SELF`: By default messages will be sent from the user - `GROUP`: By default messages will be sent from the group |
| `who_can_hide_abuse` | String | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can hide posts by reporting them as abuse. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `show_in_group_directory` | String | Deprecated. This is merged into the new whoCanDiscoverGroup setting. Allows the group to be visible in the Groups Directory. Possible values are:  
- true: All groups in the account are listed in the Groups directory. 
- false: All groups in the account are not listed in the directory. |
| `primary_language` | String | The primary language for group. For a group's primary language use the language tags from the G Suite languages found at G Suite Email Settings API Email Language Tags. |
| `who_can_unmark_favorite_reply_on_any_topic` | String | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to unmark any post from a favorite reply. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `reply_to` | String | Specifies who receives the default reply. Possible values are:  
- REPLY_TO_CUSTOM: For replies to messages, use the group's custom email address.
When the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property holds the custom email address used when replying to a message. If the group's ReplyTo property is set to REPLY_TO_CUSTOM, the customReplyTo property must have a value. Otherwise an error is returned.
 
- REPLY_TO_SENDER: The reply sent to author of message. 
- REPLY_TO_LIST: This reply message is sent to the group. 
- REPLY_TO_OWNER: The reply is sent to the owner(s) of the group. This does not include the group's managers. 
- REPLY_TO_IGNORE: Group users individually decide where the message reply is sent. 
- REPLY_TO_MANAGERS: This reply message is sent to the group's managers, which includes all managers and the group owner. |
| `max_message_bytes` | i64 | Deprecated. The maximum size of a message is 25Mb. |
| `who_can_post_announcements` | String | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can post announcements, a special topic type. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `who_can_post_message` | String | Permissions to post messages. Possible values are:  
- NONE_CAN_POST: The group is disabled and archived. No one can post a message to this group.  
- When archiveOnly is false, updating whoCanPostMessage to NONE_CAN_POST, results in an error. 
- If archiveOnly is reverted from true to false, whoCanPostMessages is set to ALL_MANAGERS_CAN_POST.  
- ALL_MANAGERS_CAN_POST: Managers, including group owners, can post messages. 
- ALL_MEMBERS_CAN_POST: Any group member can post a message. 
- ALL_OWNERS_CAN_POST: Only group owners can post a message. 
- ALL_IN_DOMAIN_CAN_POST: Anyone in the account can post a message.  
- ANYONE_CAN_POST: Any Internet user who outside your account can access your Google Groups service and post a message. Note: When whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam. |
| `message_display_font` | String | Deprecated. The default message display font always has a value of "DEFAULT_FONT". |
| `spam_moderation_level` | String | Specifies moderation levels for messages detected as spam. Possible values are:  
- ALLOW: Post the message to the group. 
- MODERATE: Send the message to the moderation queue. This is the default. 
- SILENTLY_MODERATE: Send the message to the moderation queue, but do not send notification to moderators. 
- REJECT: Immediately reject the message. |
| `favorite_replies_on_top` | String | Indicates if favorite replies should be displayed above other replies.  
- true: Favorite replies will be displayed above other replies. 
- false: Favorite replies will not be displayed above other replies. |
| `description` | String | Description of the group. This property value may be an empty string if no group description has been entered. If entered, the maximum group description is no more than 300 characters. |
| `enable_collaborative_inbox` | String | Specifies whether a collaborative inbox will remain turned on for the group. Possible values are:  
- true 
- false |
| `custom_footer_text` | String | Set the content of custom footer text. The maximum number of characters is 1,000. |
| `who_can_move_topics_out` | String | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can move topics out of the group or forum. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `message_moderation_level` | String | Moderation level of incoming messages. Possible values are:  
- MODERATE_ALL_MESSAGES: All messages are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
- MODERATE_NON_MEMBERS: All messages from non group members are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
- MODERATE_NEW_MEMBERS: All messages from new members are sent to the group owner's email address for approval. If approved, the message is sent to the group. 
- MODERATE_NONE: No moderator approval is required. Messages are delivered directly to the group. Note: When the whoCanPostMessage is set to ANYONE_CAN_POST, we recommend the messageModerationLevel be set to MODERATE_NON_MEMBERS to protect the group from possible spam.
When memberCanPostAsTheGroup is true, any message moderation settings on individual users or new members will not apply to posts made on behalf of the group. |
| `who_can_delete_topics` | String | Deprecated. This is merged into the new whoCanModerateContent setting. Specifies who can delete topics. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `is_archived` | String | Allows the Group contents to be archived. Possible values are:  
- true: Archive messages sent to the group. 
- false: Do not keep an archive of messages sent to this group. If false, previously archived messages remain in the archive. |
| `who_can_add_references` | String | Deprecated. This functionality is no longer supported in the Google Groups UI. The value is always "NONE". |
| `who_can_modify_members` | String | Deprecated. This is merged into the new whoCanModerateMembers setting. Specifies who can change group members' roles. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- OWNERS_ONLY 
- NONE |
| `custom_roles_enabled_for_settings_to_be_merged` | String | Specifies whether the group has a custom role that's included in one of the settings being merged. This field is read-only and update/patch requests to it are ignored. Possible values are:  
- true 
- false |
| `default_message_deny_notification_text` | String | When a message is rejected, this is text for the rejection notification sent to the message's author. By default, this property is empty and has no value in the API's response body. The maximum notification text size is 10,000 characters. Note: Requires sendMessageDenyNotification property to be true. |
| `who_can_mark_favorite_reply_on_own_topic` | String | Deprecated. This is merged into the new whoCanAssistContent setting. Permission to mark a post for a topic they started as a favorite reply. Possible values are:  
- ALL_MEMBERS 
- OWNERS_AND_MANAGERS 
- MANAGERS_ONLY 
- OWNERS_ONLY 
- NONE |
| `send_message_deny_notification` | String | Allows a member to be notified if the member's message to the group is denied by the group owner. Possible values are:  
- true: When a message is rejected, send the deny message notification to the message author.
The defaultMessageDenyNotificationText property is dependent on the sendMessageDenyNotification property being true.
 
- false: When a message is rejected, no notification is sent. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access group outputs
group_id = group.id
group_who_can_discover_group = group.who_can_discover_group
group_who_can_contact_owner = group.who_can_contact_owner
group_who_can_make_topics_sticky = group.who_can_make_topics_sticky
group_allow_external_members = group.allow_external_members
group_who_can_enter_free_form_tags = group.who_can_enter_free_form_tags
group_archive_only = group.archive_only
group_members_can_post_as_the_group = group.members_can_post_as_the_group
group_who_can_view_membership = group.who_can_view_membership
group_who_can_moderate_content = group.who_can_moderate_content
group_who_can_add = group.who_can_add
group_who_can_mark_duplicate = group.who_can_mark_duplicate
group_who_can_approve_members = group.who_can_approve_members
group_allow_web_posting = group.allow_web_posting
group_who_can_take_topics = group.who_can_take_topics
group_who_can_join = group.who_can_join
group_who_can_unassign_topic = group.who_can_unassign_topic
group_who_can_view_group = group.who_can_view_group
group_who_can_lock_topics = group.who_can_lock_topics
group_who_can_ban_users = group.who_can_ban_users
group_kind = group.kind
group_who_can_mark_favorite_reply_on_any_topic = group.who_can_mark_favorite_reply_on_any_topic
group_custom_reply_to = group.custom_reply_to
group_who_can_assist_content = group.who_can_assist_content
group_who_can_modify_tags_and_categories = group.who_can_modify_tags_and_categories
group_who_can_assign_topics = group.who_can_assign_topics
group_who_can_moderate_members = group.who_can_moderate_members
group_email = group.email
group_who_can_leave_group = group.who_can_leave_group
group_who_can_delete_any_post = group.who_can_delete_any_post
group_who_can_move_topics_in = group.who_can_move_topics_in
group_name = group.name
group_who_can_approve_messages = group.who_can_approve_messages
group_who_can_mark_no_response_needed = group.who_can_mark_no_response_needed
group_allow_google_communication = group.allow_google_communication
group_include_custom_footer = group.include_custom_footer
group_who_can_invite = group.who_can_invite
group_include_in_global_address_list = group.include_in_global_address_list
group_default_sender = group.default_sender
group_who_can_hide_abuse = group.who_can_hide_abuse
group_show_in_group_directory = group.show_in_group_directory
group_primary_language = group.primary_language
group_who_can_unmark_favorite_reply_on_any_topic = group.who_can_unmark_favorite_reply_on_any_topic
group_reply_to = group.reply_to
group_max_message_bytes = group.max_message_bytes
group_who_can_post_announcements = group.who_can_post_announcements
group_who_can_post_message = group.who_can_post_message
group_message_display_font = group.message_display_font
group_spam_moderation_level = group.spam_moderation_level
group_favorite_replies_on_top = group.favorite_replies_on_top
group_description = group.description
group_enable_collaborative_inbox = group.enable_collaborative_inbox
group_custom_footer_text = group.custom_footer_text
group_who_can_move_topics_out = group.who_can_move_topics_out
group_message_moderation_level = group.message_moderation_level
group_who_can_delete_topics = group.who_can_delete_topics
group_is_archived = group.is_archived
group_who_can_add_references = group.who_can_add_references
group_who_can_modify_members = group.who_can_modify_members
group_custom_roles_enabled_for_settings_to_be_merged = group.custom_roles_enabled_for_settings_to_be_merged
group_default_message_deny_notification_text = group.default_message_deny_notification_text
group_who_can_mark_favorite_reply_on_own_topic = group.who_can_mark_favorite_reply_on_own_topic
group_send_message_deny_notification = group.send_message_deny_notification
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple group resources
group_0 = provider.groupssettings_api.Group {
    group_unique_id = "value-0"
}
group_1 = provider.groupssettings_api.Group {
    group_unique_id = "value-1"
}
group_2 = provider.groupssettings_api.Group {
    group_unique_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    group = provider.groupssettings_api.Group {
        group_unique_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Groupssettings_api Documentation](https://cloud.google.com/groupssettings_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
