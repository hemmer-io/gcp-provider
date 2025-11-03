# Gmail_api Service



**Resources**: 15

---

## Overview

The gmail_api service provides access to 15 resource types:

- [Draft](#draft) [CRUD]
- [Setting](#setting) [RU]
- [Identitie](#identitie) [CRUD]
- [User](#user) [CR]
- [Delegate](#delegate) [CRD]
- [History](#history) [R]
- [Send_a](#send_a) [CRUD]
- [Label](#label) [CRUD]
- [Thread](#thread) [CRD]
- [Message](#message) [CRD]
- [Smime_info](#smime_info) [CRD]
- [Attachment](#attachment) [R]
- [Forwarding_addresse](#forwarding_addresse) [CRD]
- [Filter](#filter) [CRD]
- [Keypair](#keypair) [CR]

---

## Resources


### Draft

Creates a new draft with the `DRAFT` label.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The immutable ID of the draft. |
| `message` | String |  | The message content of the draft. |
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The immutable ID of the draft. |
| `message` | String | The message content of the draft. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create draft
draft = provider.gmail_api.Draft {
    user_id = "value"  # The user's email address. The special value `me` can be used to indicate the authenticated user.
}

# Access draft outputs
draft_id = draft.id
draft_id = draft.id
draft_message = draft.message
```

---


### Setting

Gets POP settings.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `access_window` | String |  | The range of messages which are accessible via POP. |
| `disposition` | String |  | The action that will be executed on a message after it has been fetched via POP. |
| `user_id` | String | ✅ | User's email address. The special value "me" can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_window` | String | The range of messages which are accessible via POP. |
| `disposition` | String | The action that will be executed on a message after it has been fetched via POP. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access setting outputs
setting_id = setting.id
setting_access_window = setting.access_window
setting_disposition = setting.disposition
```

---


### Identitie

Creates and configures a client-side encryption identity that's authorized to send mail from the user account. Google publishes the S/MIME certificate to a shared domain-wide directory so that people within a Google Workspace organization can encrypt and send mail to the identity. For administrators managing identities and keypairs for users in their organization, requests require authorization with a [service account](https://developers.google.com/identity/protocols/OAuth2ServiceAccount) that has [domain-wide delegation authority](https://developers.google.com/identity/protocols/OAuth2ServiceAccount#delegatingauthority) to impersonate users with the `https://www.googleapis.com/auth/gmail.settings.basic` scope. For users managing their own identities and keypairs, requests require [hardware key encryption](https://support.google.com/a/answer/14153163) turned on and configured.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email_address` | String |  | The email address for the sending identity. The email address must be the primary email address of the authenticated user. |
| `primary_key_pair_id` | String |  | If a key pair is associated, the ID of the key pair, CseKeyPair. |
| `sign_and_encrypt_key_pairs` | String |  | The configuration of a CSE identity that uses different key pairs for signing and encryption. |
| `user_id` | String | ✅ | The requester's primary email address. To indicate the authenticated user, you can use the special value `me`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `email_address` | String | The email address for the sending identity. The email address must be the primary email address of the authenticated user. |
| `primary_key_pair_id` | String | If a key pair is associated, the ID of the key pair, CseKeyPair. |
| `sign_and_encrypt_key_pairs` | String | The configuration of a CSE identity that uses different key pairs for signing and encryption. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create identitie
identitie = provider.gmail_api.Identitie {
    user_id = "value"  # The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
}

# Access identitie outputs
identitie_id = identitie.id
identitie_email_address = identitie.email_address
identitie_primary_key_pair_id = identitie.primary_key_pair_id
identitie_sign_and_encrypt_key_pairs = identitie.sign_and_encrypt_key_pairs
```

---


### User

Set up or update a push notification watch on the given user mailbox.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `label_ids` | Vec<String> |  | List of label_ids to restrict notifications about. By default, if unspecified, all changes are pushed out. If specified then dictates which labels are required for a push notification to be generated. |
| `topic_name` | String |  | A fully qualified Google Cloud Pub/Sub API topic name to publish the events to. This topic name **must** already exist in Cloud Pub/Sub and you **must** have already granted gmail "publish" permission on it. For example, "projects/my-project-identifier/topics/my-topic-name" (using the Cloud Pub/Sub "v1" topic naming format). Note that the "my-project-identifier" portion must exactly match your Google developer project id (the one executing this watch request). |
| `label_filter_behavior` | String |  | Filtering behavior of `labelIds list` specified. This field replaces `label_filter_action`; if set, `label_filter_action` is ignored. |
| `label_filter_action` | String |  | Filtering behavior of `labelIds list` specified. This field is deprecated because it caused incorrect behavior in some cases; use `label_filter_behavior` instead. |
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `history_id` | String | The ID of the mailbox's current history record. |
| `threads_total` | i64 | The total number of threads in the mailbox. |
| `email_address` | String | The user's email address. |
| `messages_total` | i64 | The total number of messages in the mailbox. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.gmail_api.User {
    user_id = "value"  # The user's email address. The special value `me` can be used to indicate the authenticated user.
}

# Access user outputs
user_id = user.id
user_history_id = user.history_id
user_threads_total = user.threads_total
user_email_address = user.email_address
user_messages_total = user.messages_total
```

---


### Delegate

Adds a delegate with its verification status set directly to `accepted`, without sending any verification email. The delegate user must be a member of the same Google Workspace organization as the delegator user. Gmail imposes limitations on the number of delegates and delegators each user in a Google Workspace organization can have. These limits depend on your organization, but in general each user can have up to 25 delegates and up to 10 delegators. Note that a delegate user must be referred to by their primary email address, and not an email alias. Also note that when a new delegate is created, there may be up to a one minute delay before the new delegate is available for use. This method is only available to service account clients that have been delegated domain-wide authority.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `verification_status` | String |  | Indicates whether this address has been verified and can act as a delegate for the account. Read-only. |
| `delegate_email` | String |  | The email address of the delegate. |
| `user_id` | String | ✅ | User's email address. The special value "me" can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `verification_status` | String | Indicates whether this address has been verified and can act as a delegate for the account. Read-only. |
| `delegate_email` | String | The email address of the delegate. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create delegate
delegate = provider.gmail_api.Delegate {
    user_id = "value"  # User's email address. The special value "me" can be used to indicate the authenticated user.
}

# Access delegate outputs
delegate_id = delegate.id
delegate_verification_status = delegate.verification_status
delegate_delegate_email = delegate.delegate_email
```

---


### History

Lists the history of all changes to the given mailbox. History results are returned in chronological order (increasing `historyId`).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `history_id` | String | The ID of the mailbox's current history record. |
| `history` | Vec<String> | List of history records. Any `messages` contained in the response will typically only have `id` and `threadId` fields populated. |
| `next_page_token` | String | Page token to retrieve the next page of results in the list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access history outputs
history_id = history.id
history_history_id = history.history_id
history_history = history.history
history_next_page_token = history.next_page_token
```

---


### Send_a

Creates a custom "from" send-as alias. If an SMTP MSA is specified, Gmail will attempt to connect to the SMTP service to validate the configuration before creating the alias. If ownership verification is required for the alias, a message will be sent to the email address and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. If a signature is provided, Gmail will sanitize the HTML before saving it with the alias. This method is only available to service account clients that have been delegated domain-wide authority.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `signature` | String |  | An optional HTML signature that is included in messages composed with this alias in the Gmail web UI. This signature is added to new emails only. |
| `verification_status` | String |  | Indicates whether this address has been verified for use as a send-as alias. Read-only. This setting only applies to custom "from" aliases. |
| `is_primary` | bool |  | Whether this address is the primary address used to login to the account. Every Gmail account has exactly one primary address, and it cannot be deleted from the collection of send-as aliases. This field is read-only. |
| `is_default` | bool |  | Whether this address is selected as the default "From:" address in situations such as composing a new message or sending a vacation auto-reply. Every Gmail account has exactly one default send-as address, so the only legal value that clients may write to this field is `true`. Changing this from `false` to `true` for an address will result in this field becoming `false` for the other previous default address. |
| `display_name` | String |  | A name that appears in the "From:" header for mail sent using this alias. For custom "from" addresses, when this is empty, Gmail will populate the "From:" header with the name that is used for the primary address associated with the account. If the admin has disabled the ability for users to update their name format, requests to update this field for the primary login will silently fail. |
| `send_as_email` | String |  | The email address that appears in the "From:" header for mail sent using this alias. This is read-only for all operations except create. |
| `reply_to_address` | String |  | An optional email address that is included in a "Reply-To:" header for mail sent using this alias. If this is empty, Gmail will not generate a "Reply-To:" header. |
| `treat_as_alias` | bool |  | Whether Gmail should treat this address as an alias for the user's primary email address. This setting only applies to custom "from" aliases. |
| `smtp_msa` | String |  | An optional SMTP service that will be used as an outbound relay for mail sent using this alias. If this is empty, outbound mail will be sent directly from Gmail's servers to the destination SMTP service. This setting only applies to custom "from" aliases. |
| `user_id` | String | ✅ | User's email address. The special value "me" can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `signature` | String | An optional HTML signature that is included in messages composed with this alias in the Gmail web UI. This signature is added to new emails only. |
| `verification_status` | String | Indicates whether this address has been verified for use as a send-as alias. Read-only. This setting only applies to custom "from" aliases. |
| `is_primary` | bool | Whether this address is the primary address used to login to the account. Every Gmail account has exactly one primary address, and it cannot be deleted from the collection of send-as aliases. This field is read-only. |
| `is_default` | bool | Whether this address is selected as the default "From:" address in situations such as composing a new message or sending a vacation auto-reply. Every Gmail account has exactly one default send-as address, so the only legal value that clients may write to this field is `true`. Changing this from `false` to `true` for an address will result in this field becoming `false` for the other previous default address. |
| `display_name` | String | A name that appears in the "From:" header for mail sent using this alias. For custom "from" addresses, when this is empty, Gmail will populate the "From:" header with the name that is used for the primary address associated with the account. If the admin has disabled the ability for users to update their name format, requests to update this field for the primary login will silently fail. |
| `send_as_email` | String | The email address that appears in the "From:" header for mail sent using this alias. This is read-only for all operations except create. |
| `reply_to_address` | String | An optional email address that is included in a "Reply-To:" header for mail sent using this alias. If this is empty, Gmail will not generate a "Reply-To:" header. |
| `treat_as_alias` | bool | Whether Gmail should treat this address as an alias for the user's primary email address. This setting only applies to custom "from" aliases. |
| `smtp_msa` | String | An optional SMTP service that will be used as an outbound relay for mail sent using this alias. If this is empty, outbound mail will be sent directly from Gmail's servers to the destination SMTP service. This setting only applies to custom "from" aliases. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create send_a
send_a = provider.gmail_api.Send_a {
    user_id = "value"  # User's email address. The special value "me" can be used to indicate the authenticated user.
}

# Access send_a outputs
send_a_id = send_a.id
send_a_signature = send_a.signature
send_a_verification_status = send_a.verification_status
send_a_is_primary = send_a.is_primary
send_a_is_default = send_a.is_default
send_a_display_name = send_a.display_name
send_a_send_as_email = send_a.send_as_email
send_a_reply_to_address = send_a.reply_to_address
send_a_treat_as_alias = send_a.treat_as_alias
send_a_smtp_msa = send_a.smtp_msa
```

---


### Label

Creates a new label.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads. |
| `id` | String |  | The immutable ID of the label. |
| `threads_unread` | i64 |  | The number of unread threads with the label. |
| `messages_unread` | i64 |  | The number of unread messages with the label. |
| `threads_total` | i64 |  | The total number of threads with the label. |
| `message_list_visibility` | String |  | The visibility of messages with this label in the message list in the Gmail web interface. |
| `color` | String |  | The color to assign to the label. Color is only available for labels that have their `type` set to `user`. |
| `label_list_visibility` | String |  | The visibility of the label in the label list in the Gmail web interface. |
| `name` | String |  | The display name of the label. |
| `messages_total` | i64 |  | The total number of messages with the label. |
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads. |
| `id` | String | The immutable ID of the label. |
| `threads_unread` | i64 | The number of unread threads with the label. |
| `messages_unread` | i64 | The number of unread messages with the label. |
| `threads_total` | i64 | The total number of threads with the label. |
| `message_list_visibility` | String | The visibility of messages with this label in the message list in the Gmail web interface. |
| `color` | String | The color to assign to the label. Color is only available for labels that have their `type` set to `user`. |
| `label_list_visibility` | String | The visibility of the label in the label list in the Gmail web interface. |
| `name` | String | The display name of the label. |
| `messages_total` | i64 | The total number of messages with the label. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create label
label = provider.gmail_api.Label {
    user_id = "value"  # The user's email address. The special value `me` can be used to indicate the authenticated user.
}

# Access label outputs
label_id = label.id
label_type = label.type
label_id = label.id
label_threads_unread = label.threads_unread
label_messages_unread = label.messages_unread
label_threads_total = label.threads_total
label_message_list_visibility = label.message_list_visibility
label_color = label.color
label_label_list_visibility = label.label_list_visibility
label_name = label.name
label_messages_total = label.messages_total
```

---


### Thread

Moves the specified thread to the trash. Any messages that belong to the thread are also moved to the trash.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | ✅ | The ID of the thread to Trash. |
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The unique ID of the thread. |
| `messages` | Vec<String> | The list of messages in the thread. |
| `history_id` | String | The ID of the last history record that modified this thread. |
| `snippet` | String | A short part of the message text. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create thread
thread = provider.gmail_api.Thread {
    id = "value"  # The ID of the thread to Trash.
    user_id = "value"  # The user's email address. The special value `me` can be used to indicate the authenticated user.
}

# Access thread outputs
thread_id = thread.id
thread_id = thread.id
thread_messages = thread.messages
thread_history_id = thread.history_id
thread_snippet = thread.snippet
```

---


### Message

Directly inserts a message into only this user's mailbox similar to `IMAP APPEND`, bypassing most scanning and classification. Does not send a message.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The immutable ID of the message. |
| `payload` | String |  | The parsed email structure in the message parts. |
| `label_ids` | Vec<String> |  | List of IDs of labels applied to this message. |
| `internal_date` | String |  | The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header. |
| `thread_id` | String |  | The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.  |
| `history_id` | String |  | The ID of the last history record that modified this message. |
| `size_estimate` | i64 |  | Estimated size in bytes of the message. |
| `raw` | String |  | The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied. |
| `snippet` | String |  | A short part of the message text. |
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The immutable ID of the message. |
| `payload` | String | The parsed email structure in the message parts. |
| `label_ids` | Vec<String> | List of IDs of labels applied to this message. |
| `internal_date` | String | The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header. |
| `thread_id` | String | The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.  |
| `history_id` | String | The ID of the last history record that modified this message. |
| `size_estimate` | i64 | Estimated size in bytes of the message. |
| `raw` | String | The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied. |
| `snippet` | String | A short part of the message text. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create message
message = provider.gmail_api.Message {
    user_id = "value"  # The user's email address. The special value `me` can be used to indicate the authenticated user.
}

# Access message outputs
message_id = message.id
message_id = message.id
message_payload = message.payload
message_label_ids = message.label_ids
message_internal_date = message.internal_date
message_thread_id = message.thread_id
message_history_id = message.history_id
message_size_estimate = message.size_estimate
message_raw = message.raw
message_snippet = message.snippet
```

---


### Smime_info

Insert (upload) the given S/MIME config for the specified send-as alias. Note that pkcs12 format is required for the key.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pkcs12` | String |  | PKCS#12 format containing a single private/public key pair and certificate chain. This format is only accepted from client for creating a new SmimeInfo and is never returned, because the private key is not intended to be exported. PKCS#12 may be encrypted, in which case encryptedKeyPassword should be set appropriately. |
| `issuer_cn` | String |  | The S/MIME certificate issuer's common name. |
| `pem` | String |  | PEM formatted X509 concatenated certificate string (standard base64 encoding). Format used for returning key, which includes public key as well as certificate chain (not private key). |
| `id` | String |  | The immutable ID for the SmimeInfo. |
| `encrypted_key_password` | String |  | Encrypted key password, when key is encrypted. |
| `expiration` | String |  | When the certificate expires (in milliseconds since epoch). |
| `is_default` | bool |  | Whether this SmimeInfo is the default one for this user's send-as address. |
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |
| `send_as_email` | String | ✅ | The email address that appears in the "From:" header for mail sent using this alias. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pkcs12` | String | PKCS#12 format containing a single private/public key pair and certificate chain. This format is only accepted from client for creating a new SmimeInfo and is never returned, because the private key is not intended to be exported. PKCS#12 may be encrypted, in which case encryptedKeyPassword should be set appropriately. |
| `issuer_cn` | String | The S/MIME certificate issuer's common name. |
| `pem` | String | PEM formatted X509 concatenated certificate string (standard base64 encoding). Format used for returning key, which includes public key as well as certificate chain (not private key). |
| `id` | String | The immutable ID for the SmimeInfo. |
| `encrypted_key_password` | String | Encrypted key password, when key is encrypted. |
| `expiration` | String | When the certificate expires (in milliseconds since epoch). |
| `is_default` | bool | Whether this SmimeInfo is the default one for this user's send-as address. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create smime_info
smime_info = provider.gmail_api.Smime_info {
    user_id = "value"  # The user's email address. The special value `me` can be used to indicate the authenticated user.
    send_as_email = "value"  # The email address that appears in the "From:" header for mail sent using this alias.
}

# Access smime_info outputs
smime_info_id = smime_info.id
smime_info_pkcs12 = smime_info.pkcs12
smime_info_issuer_cn = smime_info.issuer_cn
smime_info_pem = smime_info.pem
smime_info_id = smime_info.id
smime_info_encrypted_key_password = smime_info.encrypted_key_password
smime_info_expiration = smime_info.expiration
smime_info_is_default = smime_info.is_default
```

---


### Attachment

Gets the specified message attachment.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `data` | String | The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. An attachment ID is present if the body data is contained in a separate attachment. |
| `attachment_id` | String | When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field. |
| `size` | i64 | Number of bytes for the message part data (encoding notwithstanding). |


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
attachment_data = attachment.data
attachment_attachment_id = attachment.attachment_id
attachment_size = attachment.size
```

---


### Forwarding_addresse

Creates a forwarding address. If ownership verification is required, a message will be sent to the recipient and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. This method is only available to service account clients that have been delegated domain-wide authority.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `verification_status` | String |  | Indicates whether this address has been verified and is usable for forwarding. Read-only. |
| `forwarding_email` | String |  | An email address to which messages can be forwarded. |
| `user_id` | String | ✅ | User's email address. The special value "me" can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `verification_status` | String | Indicates whether this address has been verified and is usable for forwarding. Read-only. |
| `forwarding_email` | String | An email address to which messages can be forwarded. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create forwarding_addresse
forwarding_addresse = provider.gmail_api.Forwarding_addresse {
    user_id = "value"  # User's email address. The special value "me" can be used to indicate the authenticated user.
}

# Access forwarding_addresse outputs
forwarding_addresse_id = forwarding_addresse.id
forwarding_addresse_verification_status = forwarding_addresse.verification_status
forwarding_addresse_forwarding_email = forwarding_addresse.forwarding_email
```

---


### Filter

Creates a filter. Note: you can only create a maximum of 1,000 filters.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | String |  | Action that the filter performs. |
| `criteria` | String |  | Matching criteria for the filter. |
| `id` | String |  | The server assigned ID of the filter. |
| `user_id` | String | ✅ | User's email address. The special value "me" can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `action` | String | Action that the filter performs. |
| `criteria` | String | Matching criteria for the filter. |
| `id` | String | The server assigned ID of the filter. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create filter
filter = provider.gmail_api.Filter {
    user_id = "value"  # User's email address. The special value "me" can be used to indicate the authenticated user.
}

# Access filter outputs
filter_id = filter.id
filter_action = filter.action
filter_criteria = filter.criteria
filter_id = filter.id
```

---


### Keypair

Creates and uploads a client-side encryption S/MIME public key certificate chain and private key metadata for the authenticated user. For administrators managing identities and keypairs for users in their organization, requests require authorization with a [service account](https://developers.google.com/identity/protocols/OAuth2ServiceAccount) that has [domain-wide delegation authority](https://developers.google.com/identity/protocols/OAuth2ServiceAccount#delegatingauthority) to impersonate users with the `https://www.googleapis.com/auth/gmail.settings.basic` scope. For users managing their own identities and keypairs, requests require [hardware key encryption](https://support.google.com/a/answer/14153163) turned on and configured.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subject_email_addresses` | Vec<String> |  | Output only. The email address identities that are specified on the leaf certificate. |
| `disable_time` | String |  | Output only. If a key pair is set to `DISABLED`, the time that the key pair's state changed from `ENABLED` to `DISABLED`. This field is present only when the key pair is in state `DISABLED`. |
| `pem` | String |  | Output only. The public key and its certificate chain, in [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail) format. |
| `key_pair_id` | String |  | Output only. The immutable ID for the client-side encryption S/MIME key pair. |
| `enablement_state` | String |  | Output only. The current state of the key pair. |
| `pkcs7` | String |  | Input only. The public key and its certificate chain. The chain must be in [PKCS#7](https://en.wikipedia.org/wiki/PKCS_7) format and use PEM encoding and ASCII armor. |
| `private_key_metadata` | Vec<String> |  | Metadata for instances of this key pair's private key. |
| `user_id` | String | ✅ | The requester's primary email address. To indicate the authenticated user, you can use the special value `me`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subject_email_addresses` | Vec<String> | Output only. The email address identities that are specified on the leaf certificate. |
| `disable_time` | String | Output only. If a key pair is set to `DISABLED`, the time that the key pair's state changed from `ENABLED` to `DISABLED`. This field is present only when the key pair is in state `DISABLED`. |
| `pem` | String | Output only. The public key and its certificate chain, in [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail) format. |
| `key_pair_id` | String | Output only. The immutable ID for the client-side encryption S/MIME key pair. |
| `enablement_state` | String | Output only. The current state of the key pair. |
| `pkcs7` | String | Input only. The public key and its certificate chain. The chain must be in [PKCS#7](https://en.wikipedia.org/wiki/PKCS_7) format and use PEM encoding and ASCII armor. |
| `private_key_metadata` | Vec<String> | Metadata for instances of this key pair's private key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create keypair
keypair = provider.gmail_api.Keypair {
    user_id = "value"  # The requester's primary email address. To indicate the authenticated user, you can use the special value `me`.
}

# Access keypair outputs
keypair_id = keypair.id
keypair_subject_email_addresses = keypair.subject_email_addresses
keypair_disable_time = keypair.disable_time
keypair_pem = keypair.pem
keypair_key_pair_id = keypair.key_pair_id
keypair_enablement_state = keypair.enablement_state
keypair_pkcs7 = keypair.pkcs7
keypair_private_key_metadata = keypair.private_key_metadata
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple draft resources
draft_0 = provider.gmail_api.Draft {
    user_id = "value-0"
}
draft_1 = provider.gmail_api.Draft {
    user_id = "value-1"
}
draft_2 = provider.gmail_api.Draft {
    user_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    draft = provider.gmail_api.Draft {
        user_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Gmail_api Documentation](https://cloud.google.com/gmail_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
