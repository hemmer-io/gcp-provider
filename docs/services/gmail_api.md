# Gmail_api Service



**Resources**: 15

---

## Overview

The gmail_api service provides access to 15 resource types:

- [Smime_info](#smime_info) [CRD]
- [Filter](#filter) [CRD]
- [Delegate](#delegate) [CRD]
- [Send_a](#send_a) [CRUD]
- [Label](#label) [CRUD]
- [History](#history) [R]
- [Keypair](#keypair) [CR]
- [Identitie](#identitie) [CRUD]
- [Draft](#draft) [CRUD]
- [Attachment](#attachment) [R]
- [Message](#message) [CRD]
- [Forwarding_addresse](#forwarding_addresse) [CRD]
- [Setting](#setting) [RU]
- [Thread](#thread) [CRD]
- [User](#user) [CR]

---

## Resources


### Smime_info

Insert (upload) the given S/MIME config for the specified send-as alias. Note that pkcs12 format is required for the key.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encrypted_key_password` | String |  | Encrypted key password, when key is encrypted. |
| `id` | String |  | The immutable ID for the SmimeInfo. |
| `is_default` | bool |  | Whether this SmimeInfo is the default one for this user's send-as address. |
| `issuer_cn` | String |  | The S/MIME certificate issuer's common name. |
| `pem` | String |  | PEM formatted X509 concatenated certificate string (standard base64 encoding). Format used for returning key, which includes public key as well as certificate chain (not private key). |
| `expiration` | String |  | When the certificate expires (in milliseconds since epoch). |
| `pkcs12` | String |  | PKCS#12 format containing a single private/public key pair and certificate chain. This format is only accepted from client for creating a new SmimeInfo and is never returned, because the private key is not intended to be exported. PKCS#12 may be encrypted, in which case encryptedKeyPassword should be set appropriately. |
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |
| `send_as_email` | String | ✅ | The email address that appears in the "From:" header for mail sent using this alias. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `encrypted_key_password` | String | Encrypted key password, when key is encrypted. |
| `id` | String | The immutable ID for the SmimeInfo. |
| `is_default` | bool | Whether this SmimeInfo is the default one for this user's send-as address. |
| `issuer_cn` | String | The S/MIME certificate issuer's common name. |
| `pem` | String | PEM formatted X509 concatenated certificate string (standard base64 encoding). Format used for returning key, which includes public key as well as certificate chain (not private key). |
| `expiration` | String | When the certificate expires (in milliseconds since epoch). |
| `pkcs12` | String | PKCS#12 format containing a single private/public key pair and certificate chain. This format is only accepted from client for creating a new SmimeInfo and is never returned, because the private key is not intended to be exported. PKCS#12 may be encrypted, in which case encryptedKeyPassword should be set appropriately. |


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
smime_info_encrypted_key_password = smime_info.encrypted_key_password
smime_info_id = smime_info.id
smime_info_is_default = smime_info.is_default
smime_info_issuer_cn = smime_info.issuer_cn
smime_info_pem = smime_info.pem
smime_info_expiration = smime_info.expiration
smime_info_pkcs12 = smime_info.pkcs12
```

---


### Filter

Creates a filter. Note: you can only create a maximum of 1,000 filters.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `criteria` | String |  | Matching criteria for the filter. |
| `action` | String |  | Action that the filter performs. |
| `id` | String |  | The server assigned ID of the filter. |
| `user_id` | String | ✅ | User's email address. The special value "me" can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `criteria` | String | Matching criteria for the filter. |
| `action` | String | Action that the filter performs. |
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
filter_criteria = filter.criteria
filter_action = filter.action
filter_id = filter.id
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


### Send_a

Creates a custom "from" send-as alias. If an SMTP MSA is specified, Gmail will attempt to connect to the SMTP service to validate the configuration before creating the alias. If ownership verification is required for the alias, a message will be sent to the email address and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. If a signature is provided, Gmail will sanitize the HTML before saving it with the alias. This method is only available to service account clients that have been delegated domain-wide authority.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | A name that appears in the "From:" header for mail sent using this alias. For custom "from" addresses, when this is empty, Gmail will populate the "From:" header with the name that is used for the primary address associated with the account. If the admin has disabled the ability for users to update their name format, requests to update this field for the primary login will silently fail. |
| `send_as_email` | String |  | The email address that appears in the "From:" header for mail sent using this alias. This is read-only for all operations except create. |
| `treat_as_alias` | bool |  | Whether Gmail should treat this address as an alias for the user's primary email address. This setting only applies to custom "from" aliases. |
| `verification_status` | String |  | Indicates whether this address has been verified for use as a send-as alias. Read-only. This setting only applies to custom "from" aliases. |
| `signature` | String |  | An optional HTML signature that is included in messages composed with this alias in the Gmail web UI. This signature is added to new emails only. |
| `is_default` | bool |  | Whether this address is selected as the default "From:" address in situations such as composing a new message or sending a vacation auto-reply. Every Gmail account has exactly one default send-as address, so the only legal value that clients may write to this field is `true`. Changing this from `false` to `true` for an address will result in this field becoming `false` for the other previous default address. |
| `reply_to_address` | String |  | An optional email address that is included in a "Reply-To:" header for mail sent using this alias. If this is empty, Gmail will not generate a "Reply-To:" header. |
| `smtp_msa` | String |  | An optional SMTP service that will be used as an outbound relay for mail sent using this alias. If this is empty, outbound mail will be sent directly from Gmail's servers to the destination SMTP service. This setting only applies to custom "from" aliases. |
| `is_primary` | bool |  | Whether this address is the primary address used to login to the account. Every Gmail account has exactly one primary address, and it cannot be deleted from the collection of send-as aliases. This field is read-only. |
| `user_id` | String | ✅ | User's email address. The special value "me" can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | A name that appears in the "From:" header for mail sent using this alias. For custom "from" addresses, when this is empty, Gmail will populate the "From:" header with the name that is used for the primary address associated with the account. If the admin has disabled the ability for users to update their name format, requests to update this field for the primary login will silently fail. |
| `send_as_email` | String | The email address that appears in the "From:" header for mail sent using this alias. This is read-only for all operations except create. |
| `treat_as_alias` | bool | Whether Gmail should treat this address as an alias for the user's primary email address. This setting only applies to custom "from" aliases. |
| `verification_status` | String | Indicates whether this address has been verified for use as a send-as alias. Read-only. This setting only applies to custom "from" aliases. |
| `signature` | String | An optional HTML signature that is included in messages composed with this alias in the Gmail web UI. This signature is added to new emails only. |
| `is_default` | bool | Whether this address is selected as the default "From:" address in situations such as composing a new message or sending a vacation auto-reply. Every Gmail account has exactly one default send-as address, so the only legal value that clients may write to this field is `true`. Changing this from `false` to `true` for an address will result in this field becoming `false` for the other previous default address. |
| `reply_to_address` | String | An optional email address that is included in a "Reply-To:" header for mail sent using this alias. If this is empty, Gmail will not generate a "Reply-To:" header. |
| `smtp_msa` | String | An optional SMTP service that will be used as an outbound relay for mail sent using this alias. If this is empty, outbound mail will be sent directly from Gmail's servers to the destination SMTP service. This setting only applies to custom "from" aliases. |
| `is_primary` | bool | Whether this address is the primary address used to login to the account. Every Gmail account has exactly one primary address, and it cannot be deleted from the collection of send-as aliases. This field is read-only. |


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
send_a_display_name = send_a.display_name
send_a_send_as_email = send_a.send_as_email
send_a_treat_as_alias = send_a.treat_as_alias
send_a_verification_status = send_a.verification_status
send_a_signature = send_a.signature
send_a_is_default = send_a.is_default
send_a_reply_to_address = send_a.reply_to_address
send_a_smtp_msa = send_a.smtp_msa
send_a_is_primary = send_a.is_primary
```

---


### Label

Creates a new label.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads. |
| `name` | String |  | The display name of the label. |
| `message_list_visibility` | String |  | The visibility of messages with this label in the message list in the Gmail web interface. |
| `color` | String |  | The color to assign to the label. Color is only available for labels that have their `type` set to `user`. |
| `messages_unread` | i64 |  | The number of unread messages with the label. |
| `threads_unread` | i64 |  | The number of unread threads with the label. |
| `label_list_visibility` | String |  | The visibility of the label in the label list in the Gmail web interface. |
| `messages_total` | i64 |  | The total number of messages with the label. |
| `id` | String |  | The immutable ID of the label. |
| `threads_total` | i64 |  | The total number of threads with the label. |
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads. |
| `name` | String | The display name of the label. |
| `message_list_visibility` | String | The visibility of messages with this label in the message list in the Gmail web interface. |
| `color` | String | The color to assign to the label. Color is only available for labels that have their `type` set to `user`. |
| `messages_unread` | i64 | The number of unread messages with the label. |
| `threads_unread` | i64 | The number of unread threads with the label. |
| `label_list_visibility` | String | The visibility of the label in the label list in the Gmail web interface. |
| `messages_total` | i64 | The total number of messages with the label. |
| `id` | String | The immutable ID of the label. |
| `threads_total` | i64 | The total number of threads with the label. |


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
label_name = label.name
label_message_list_visibility = label.message_list_visibility
label_color = label.color
label_messages_unread = label.messages_unread
label_threads_unread = label.threads_unread
label_label_list_visibility = label.label_list_visibility
label_messages_total = label.messages_total
label_id = label.id
label_threads_total = label.threads_total
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
| `history` | Vec<String> | List of history records. Any `messages` contained in the response will typically only have `id` and `threadId` fields populated. |
| `history_id` | String | The ID of the mailbox's current history record. |
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
history_history = history.history
history_history_id = history.history_id
history_next_page_token = history.next_page_token
```

---


### Keypair

Creates and uploads a client-side encryption S/MIME public key certificate chain and private key metadata for the authenticated user. For administrators managing identities and keypairs for users in their organization, requests require authorization with a [service account](https://developers.google.com/identity/protocols/OAuth2ServiceAccount) that has [domain-wide delegation authority](https://developers.google.com/identity/protocols/OAuth2ServiceAccount#delegatingauthority) to impersonate users with the `https://www.googleapis.com/auth/gmail.settings.basic` scope. For users managing their own identities and keypairs, requests require [hardware key encryption](https://support.google.com/a/answer/14153163) turned on and configured.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `disable_time` | String |  | Output only. If a key pair is set to `DISABLED`, the time that the key pair's state changed from `ENABLED` to `DISABLED`. This field is present only when the key pair is in state `DISABLED`. |
| `enablement_state` | String |  | Output only. The current state of the key pair. |
| `key_pair_id` | String |  | Output only. The immutable ID for the client-side encryption S/MIME key pair. |
| `pem` | String |  | Output only. The public key and its certificate chain, in [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail) format. |
| `pkcs7` | String |  | Input only. The public key and its certificate chain. The chain must be in [PKCS#7](https://en.wikipedia.org/wiki/PKCS_7) format and use PEM encoding and ASCII armor. |
| `private_key_metadata` | Vec<String> |  | Metadata for instances of this key pair's private key. |
| `subject_email_addresses` | Vec<String> |  | Output only. The email address identities that are specified on the leaf certificate. |
| `user_id` | String | ✅ | The requester's primary email address. To indicate the authenticated user, you can use the special value `me`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `disable_time` | String | Output only. If a key pair is set to `DISABLED`, the time that the key pair's state changed from `ENABLED` to `DISABLED`. This field is present only when the key pair is in state `DISABLED`. |
| `enablement_state` | String | Output only. The current state of the key pair. |
| `key_pair_id` | String | Output only. The immutable ID for the client-side encryption S/MIME key pair. |
| `pem` | String | Output only. The public key and its certificate chain, in [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail) format. |
| `pkcs7` | String | Input only. The public key and its certificate chain. The chain must be in [PKCS#7](https://en.wikipedia.org/wiki/PKCS_7) format and use PEM encoding and ASCII armor. |
| `private_key_metadata` | Vec<String> | Metadata for instances of this key pair's private key. |
| `subject_email_addresses` | Vec<String> | Output only. The email address identities that are specified on the leaf certificate. |


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
keypair_disable_time = keypair.disable_time
keypair_enablement_state = keypair.enablement_state
keypair_key_pair_id = keypair.key_pair_id
keypair_pem = keypair.pem
keypair_pkcs7 = keypair.pkcs7
keypair_private_key_metadata = keypair.private_key_metadata
keypair_subject_email_addresses = keypair.subject_email_addresses
```

---


### Identitie

Creates and configures a client-side encryption identity that's authorized to send mail from the user account. Google publishes the S/MIME certificate to a shared domain-wide directory so that people within a Google Workspace organization can encrypt and send mail to the identity. For administrators managing identities and keypairs for users in their organization, requests require authorization with a [service account](https://developers.google.com/identity/protocols/OAuth2ServiceAccount) that has [domain-wide delegation authority](https://developers.google.com/identity/protocols/OAuth2ServiceAccount#delegatingauthority) to impersonate users with the `https://www.googleapis.com/auth/gmail.settings.basic` scope. For users managing their own identities and keypairs, requests require [hardware key encryption](https://support.google.com/a/answer/14153163) turned on and configured.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email_address` | String |  | The email address for the sending identity. The email address must be the primary email address of the authenticated user. |
| `sign_and_encrypt_key_pairs` | String |  | The configuration of a CSE identity that uses different key pairs for signing and encryption. |
| `primary_key_pair_id` | String |  | If a key pair is associated, the ID of the key pair, CseKeyPair. |
| `user_id` | String | ✅ | The requester's primary email address. To indicate the authenticated user, you can use the special value `me`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `email_address` | String | The email address for the sending identity. The email address must be the primary email address of the authenticated user. |
| `sign_and_encrypt_key_pairs` | String | The configuration of a CSE identity that uses different key pairs for signing and encryption. |
| `primary_key_pair_id` | String | If a key pair is associated, the ID of the key pair, CseKeyPair. |


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
identitie_sign_and_encrypt_key_pairs = identitie.sign_and_encrypt_key_pairs
identitie_primary_key_pair_id = identitie.primary_key_pair_id
```

---


### Draft

Creates a new draft with the `DRAFT` label.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `message` | String |  | The message content of the draft. |
| `id` | String |  | The immutable ID of the draft. |
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `message` | String | The message content of the draft. |
| `id` | String | The immutable ID of the draft. |


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
draft_message = draft.message
draft_id = draft.id
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
| `size` | i64 | Number of bytes for the message part data (encoding notwithstanding). |
| `attachment_id` | String | When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field. |


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
attachment_size = attachment.size
attachment_attachment_id = attachment.attachment_id
```

---


### Message

Directly inserts a message into only this user's mailbox similar to `IMAP APPEND`, bypassing most scanning and classification. Does not send a message.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `history_id` | String |  | The ID of the last history record that modified this message. |
| `internal_date` | String |  | The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header. |
| `size_estimate` | i64 |  | Estimated size in bytes of the message. |
| `label_ids` | Vec<String> |  | List of IDs of labels applied to this message. |
| `snippet` | String |  | A short part of the message text. |
| `raw` | String |  | The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied. |
| `payload` | String |  | The parsed email structure in the message parts. |
| `id` | String |  | The immutable ID of the message. |
| `thread_id` | String |  | The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.  |
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `history_id` | String | The ID of the last history record that modified this message. |
| `internal_date` | String | The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header. |
| `size_estimate` | i64 | Estimated size in bytes of the message. |
| `label_ids` | Vec<String> | List of IDs of labels applied to this message. |
| `snippet` | String | A short part of the message text. |
| `raw` | String | The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied. |
| `payload` | String | The parsed email structure in the message parts. |
| `id` | String | The immutable ID of the message. |
| `thread_id` | String | The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.  |


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
message_history_id = message.history_id
message_internal_date = message.internal_date
message_size_estimate = message.size_estimate
message_label_ids = message.label_ids
message_snippet = message.snippet
message_raw = message.raw
message_payload = message.payload
message_id = message.id
message_thread_id = message.thread_id
```

---


### Forwarding_addresse

Creates a forwarding address. If ownership verification is required, a message will be sent to the recipient and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. This method is only available to service account clients that have been delegated domain-wide authority.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `forwarding_email` | String |  | An email address to which messages can be forwarded. |
| `verification_status` | String |  | Indicates whether this address has been verified and is usable for forwarding. Read-only. |
| `user_id` | String | ✅ | User's email address. The special value "me" can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `forwarding_email` | String | An email address to which messages can be forwarded. |
| `verification_status` | String | Indicates whether this address has been verified and is usable for forwarding. Read-only. |


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
forwarding_addresse_forwarding_email = forwarding_addresse.forwarding_email
forwarding_addresse_verification_status = forwarding_addresse.verification_status
```

---


### Setting

Gets vacation responder settings.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `response_body_html` | String |  | Response body in HTML format. Gmail will sanitize the HTML before storing it. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used. |
| `response_body_plain_text` | String |  | Response body in plain text format. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used. |
| `restrict_to_domain` | bool |  | Flag that determines whether responses are sent to recipients who are outside of the user's domain. This feature is only available for Google Workspace users. |
| `response_subject` | String |  | Optional text to prepend to the subject line in vacation responses. In order to enable auto-replies, either the response subject or the response body must be nonempty. |
| `start_time` | String |  | An optional start time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives after the start time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`. |
| `enable_auto_reply` | bool |  | Flag that controls whether Gmail automatically replies to messages. |
| `restrict_to_contacts` | bool |  | Flag that determines whether responses are sent to recipients who are not in the user's list of contacts. |
| `end_time` | String |  | An optional end time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives before the end time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`. |
| `user_id` | String | ✅ | User's email address. The special value "me" can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response_body_html` | String | Response body in HTML format. Gmail will sanitize the HTML before storing it. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used. |
| `response_body_plain_text` | String | Response body in plain text format. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used. |
| `restrict_to_domain` | bool | Flag that determines whether responses are sent to recipients who are outside of the user's domain. This feature is only available for Google Workspace users. |
| `response_subject` | String | Optional text to prepend to the subject line in vacation responses. In order to enable auto-replies, either the response subject or the response body must be nonempty. |
| `start_time` | String | An optional start time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives after the start time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`. |
| `enable_auto_reply` | bool | Flag that controls whether Gmail automatically replies to messages. |
| `restrict_to_contacts` | bool | Flag that determines whether responses are sent to recipients who are not in the user's list of contacts. |
| `end_time` | String | An optional end time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives before the end time. If both `startTime` and `endTime` are specified, `startTime` must precede `endTime`. |


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
setting_response_body_html = setting.response_body_html
setting_response_body_plain_text = setting.response_body_plain_text
setting_restrict_to_domain = setting.restrict_to_domain
setting_response_subject = setting.response_subject
setting_start_time = setting.start_time
setting_enable_auto_reply = setting.enable_auto_reply
setting_restrict_to_contacts = setting.restrict_to_contacts
setting_end_time = setting.end_time
```

---


### Thread

Moves the specified thread to the trash. Any messages that belong to the thread are also moved to the trash.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |
| `id` | String | ✅ | The ID of the thread to Trash. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `messages` | Vec<String> | The list of messages in the thread. |
| `history_id` | String | The ID of the last history record that modified this thread. |
| `id` | String | The unique ID of the thread. |
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
    user_id = "value"  # The user's email address. The special value `me` can be used to indicate the authenticated user.
    id = "value"  # The ID of the thread to Trash.
}

# Access thread outputs
thread_id = thread.id
thread_messages = thread.messages
thread_history_id = thread.history_id
thread_id = thread.id
thread_snippet = thread.snippet
```

---


### User

Stop receiving push notifications for the given user mailbox.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String | ✅ | The user's email address. The special value `me` can be used to indicate the authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `messages_total` | i64 | The total number of messages in the mailbox. |
| `history_id` | String | The ID of the mailbox's current history record. |
| `email_address` | String | The user's email address. |
| `threads_total` | i64 | The total number of threads in the mailbox. |


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
user_messages_total = user.messages_total
user_history_id = user.history_id
user_email_address = user.email_address
user_threads_total = user.threads_total
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple smime_info resources
smime_info_0 = provider.gmail_api.Smime_info {
    user_id = "value-0"
    send_as_email = "value-0"
}
smime_info_1 = provider.gmail_api.Smime_info {
    user_id = "value-1"
    send_as_email = "value-1"
}
smime_info_2 = provider.gmail_api.Smime_info {
    user_id = "value-2"
    send_as_email = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    smime_info = provider.gmail_api.Smime_info {
        user_id = "production-value"
        send_as_email = "production-value"
    }
```

---

## Related Documentation

- [GCP Gmail_api Documentation](https://cloud.google.com/gmail_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
